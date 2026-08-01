// Standalone BLAKE3 reference-impl port for validation before porting into index.ts.
const IV = new Uint32Array([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);
const MSG_PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
const CHUNK_START = 1 << 0;
const CHUNK_END = 1 << 1;
const PARENT = 1 << 2;
const ROOT = 1 << 3;
const BLOCK_LEN = 64;
const CHUNK_LEN = 1024;
const OUT_LEN = 32;

function rotr(x, n) {
  return ((x >>> n) | (x << (32 - n))) >>> 0;
}

function g(state, a, b, c, d, mx, my) {
  state[a] = (state[a] + state[b] + mx) >>> 0;
  state[d] = rotr(state[d] ^ state[a], 16);
  state[c] = (state[c] + state[d]) >>> 0;
  state[b] = rotr(state[b] ^ state[c], 12);
  state[a] = (state[a] + state[b] + my) >>> 0;
  state[d] = rotr(state[d] ^ state[a], 8);
  state[c] = (state[c] + state[d]) >>> 0;
  state[b] = rotr(state[b] ^ state[c], 7);
}

function round(state, m) {
  g(state, 0, 4, 8, 12, m[0], m[1]);
  g(state, 1, 5, 9, 13, m[2], m[3]);
  g(state, 2, 6, 10, 14, m[4], m[5]);
  g(state, 3, 7, 11, 15, m[6], m[7]);
  g(state, 0, 5, 10, 15, m[8], m[9]);
  g(state, 1, 6, 11, 12, m[10], m[11]);
  g(state, 2, 7, 8, 13, m[12], m[13]);
  g(state, 3, 4, 9, 14, m[14], m[15]);
}

function permute(m) {
  const p = new Uint32Array(16);
  for (let i = 0; i < 16; i++) p[i] = m[MSG_PERMUTATION[i]];
  return p;
}

function compress(cv, blockWords, counter, blockLen, flags) {
  const state = new Uint32Array(16);
  state.set(cv, 0);
  state.set(IV.subarray(0, 4), 8);
  state[12] = counter >>> 0;
  state[13] = Math.floor(counter / 0x100000000) >>> 0;
  state[14] = blockLen >>> 0;
  state[15] = flags >>> 0;
  let block = blockWords;
  for (let i = 0; i < 7; i++) {
    round(state, block);
    if (i < 6) block = permute(block);
  }
  for (let i = 0; i < 8; i++) {
    state[i] = (state[i] ^ state[i + 8]) >>> 0;
    state[i + 8] = (state[i + 8] ^ cv[i]) >>> 0;
  }
  return state;
}

function wordsFromLeBytes16(bytes /* 64 bytes, zero-padded */) {
  const words = new Uint32Array(16);
  const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  for (let i = 0; i < 16; i++) words[i] = dv.getUint32(i * 4, true);
  return words;
}

class ChunkState {
  constructor(key, chunkCounter, flags) {
    this.cv = key.slice();
    this.chunkCounter = chunkCounter;
    this.block = new Uint8Array(BLOCK_LEN);
    this.blockLen = 0;
    this.blocksCompressed = 0;
    this.flags = flags;
  }
  len() {
    return BLOCK_LEN * this.blocksCompressed + this.blockLen;
  }
  startFlag() {
    return this.blocksCompressed === 0 ? CHUNK_START : 0;
  }
  update(input) {
    let offset = 0;
    while (offset < input.length) {
      if (this.blockLen === BLOCK_LEN) {
        const blockWords = wordsFromLeBytes16(this.block);
        this.cv = compress(this.cv, blockWords, this.chunkCounter, BLOCK_LEN, this.flags | this.startFlag()).subarray(0, 8);
        this.blocksCompressed += 1;
        this.block = new Uint8Array(BLOCK_LEN);
        this.blockLen = 0;
      }
      const want = BLOCK_LEN - this.blockLen;
      const take = Math.min(want, input.length - offset);
      this.block.set(input.subarray(offset, offset + take), this.blockLen);
      this.blockLen += take;
      offset += take;
    }
  }
  output() {
    const blockWords = wordsFromLeBytes16(this.block);
    return { inputCv: this.cv, blockWords, counter: this.chunkCounter, blockLen: this.blockLen, flags: this.flags | this.startFlag() | CHUNK_END };
  }
}

function outputChainingValue(output) {
  return compress(output.inputCv, output.blockWords, output.counter, output.blockLen, output.flags).subarray(0, 8);
}

function outputRootBytes(output, outLen) {
  const out = new Uint8Array(outLen);
  let outputBlockCounter = 0;
  let written = 0;
  while (written < outLen) {
    const words = compress(output.inputCv, output.blockWords, outputBlockCounter, output.blockLen, output.flags | ROOT);
    const buf = new Uint8Array(64);
    const dv = new DataView(buf.buffer);
    for (let i = 0; i < 16; i++) dv.setUint32(i * 4, words[i], true);
    const take = Math.min(64, outLen - written);
    out.set(buf.subarray(0, take), written);
    written += take;
    outputBlockCounter += 1;
  }
  return out;
}

function parentOutput(leftCv, rightCv, key, flags) {
  const blockWords = new Uint32Array(16);
  blockWords.set(leftCv, 0);
  blockWords.set(rightCv, 8);
  return { inputCv: key, blockWords, counter: 0, blockLen: BLOCK_LEN, flags: PARENT | flags };
}
function parentCv(leftCv, rightCv, key, flags) {
  return outputChainingValue(parentOutput(leftCv, rightCv, key, flags));
}

class Hasher {
  constructor(key = IV, flags = 0) {
    this.key = key;
    this.flags = flags;
    this.chunkState = new ChunkState(key, 0, flags);
    this.cvStack = [];
  }
  addChunkChainingValue(newCvIn, totalChunks) {
    let newCv = newCvIn;
    while ((totalChunks & 1) === 0) {
      const left = this.cvStack.pop();
      newCv = parentCv(left, newCv, this.key, this.flags);
      totalChunks = Math.floor(totalChunks / 2);
    }
    this.cvStack.push(newCv);
  }
  update(input) {
    let offset = 0;
    while (offset < input.length) {
      if (this.chunkState.len() === CHUNK_LEN) {
        const chunkCv = outputChainingValue(this.chunkState.output());
        const totalChunks = this.chunkState.chunkCounter + 1;
        this.addChunkChainingValue(chunkCv, totalChunks);
        this.chunkState = new ChunkState(this.key, totalChunks, this.flags);
      }
      const want = CHUNK_LEN - this.chunkState.len();
      const take = Math.min(want, input.length - offset);
      this.chunkState.update(input.subarray(offset, offset + take));
      offset += take;
    }
  }
  finalize(outLen = OUT_LEN) {
    let output = this.chunkState.output();
    let parentNodesRemaining = this.cvStack.length;
    while (parentNodesRemaining > 0) {
      parentNodesRemaining -= 1;
      output = parentOutput(this.cvStack[parentNodesRemaining], outputChainingValue(output), this.key, this.flags);
    }
    return outputRootBytes(output, outLen);
  }
}

function blake3(bytes) {
  const h = new Hasher();
  h.update(bytes);
  return h.finalize();
}

function hex(bytes) {
  return Buffer.from(bytes).toString("hex");
}

// Known BLAKE3 test vector: hash of empty input.
console.log("blake3('') =", hex(blake3(new Uint8Array(0))));
console.log("expected   = af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262");

// hash of "abc"
console.log("blake3('abc') =", hex(blake3(new TextEncoder().encode("abc"))));
// known: 6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85

// our fixture: doc payload [0x01,0x01,0x11,0x12]
console.log("blake3(null doc payload) =", hex(blake3(new Uint8Array([0x01, 0x01, 0x11, 0x12]))));
console.log("expected                 = 6e3211ee00d53fca02ae29b450f9dc240b01f758753c346d3232e90eaab117ef");

// a longer input spanning multiple 1024-byte chunks to exercise the tree path
const big = new Uint8Array(5000);
for (let i = 0; i < big.length; i++) big[i] = i % 251;
console.log("blake3(5000 bytes) =", hex(blake3(big)));

export { blake3, hex };
