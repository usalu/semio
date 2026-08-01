import zlib from "node:zlib";

const hexNull = "8953504b0d0a1a0a010000000100000001000000ef65ef880000000000000000030303016300001d75fe6e0403060463641414020072ba15e70103332e012e00d1ff0087b135201cabc2b7708ae6d5410d1b10578cbdfbd359c82d21bcea70477f7bd92b0e01200b00000000040100007bc75acd0000007aa3646053504b464f4f5431010000000100000039000000000000003b00000000000000cf000000000000006e3211ee00d53fca02ae29b450f9dc240b01f758753c346d3232e90eaab117ef0000000000000000de539a67";

function hexToBytes(hex) {
  const out = new Uint8Array(hex.length / 2);
  for (let i = 0; i < out.length; i++) out[i] = parseInt(hex.substr(i * 2, 2), 16);
  return out;
}

const bytes = hexToBytes(hexNull);
console.log("total len", bytes.length);

function readVarint(bytes, pos) {
  let result = 0n, shift = 1n;
  for (let i = 0; i < 10; i++) {
    const b = bytes[pos[0]]; pos[0]++;
    result += BigInt(b & 0x7f) * shift;
    if ((b & 0x80) === 0) return result;
    shift *= 128n;
  }
  throw new Error("overlong");
}

// Header: 32 bytes
console.log("magic", Buffer.from(bytes.slice(0,8)).toString('hex'));
console.log("version_major", bytes[8] | (bytes[9]<<8));
console.log("version_minor", bytes[10] | (bytes[11]<<8));
const requiredFlags = bytes[12] | (bytes[13]<<8) | (bytes[14]<<16) | (bytes[15]<<24);
const optionalFlags = bytes[16] | (bytes[17]<<8) | (bytes[18]<<16) | (bytes[19]<<24);
console.log("required_flags", requiredFlags, "optional_flags", optionalFlags);
console.log("header_crc", Buffer.from(bytes.slice(20,24)).toString('hex'));
console.log("reserved", Buffer.from(bytes.slice(24,32)).toString('hex'));

let pos = 32;
const totalLen = bytes.length;
const footerSize = 84;
console.log("footer starts at", totalLen - footerSize);

// walk segments sequentially until we hit footer offset
function readSegmentAt(bytes, offset) {
  const kind = bytes[offset];
  const flags = bytes[offset+1];
  const compressed = (flags & 1) !== 0;
  const codec = (flags >> 1) & 0x07;
  const p = [offset+2];
  const storedLen = readVarint(bytes, p);
  let rawLen = storedLen;
  if (compressed) {
    rawLen = readVarint(bytes, p);
  }
  const payloadStart = p[0];
  const stored = bytes.slice(payloadStart, payloadStart + Number(storedLen));
  const crcStart = payloadStart + Number(storedLen);
  const crc = bytes.slice(crcStart, crcStart+4);
  const consumed = crcStart + 4 - offset;
  let raw = stored;
  if (compressed) {
    try {
      raw = zlib.inflateRawSync(Buffer.from(stored));
    } catch (e) {
      raw = null;
    }
  }
  return { kind, flags, compressed, codec, storedLen, rawLen, payloadStart, stored, crc, consumed, raw };
}

let offset = 32;
while (offset < totalLen - footerSize) {
  const seg = readSegmentAt(bytes, offset);
  console.log(`--- segment at ${offset}: kind=0x${seg.kind.toString(16)} flags=0x${seg.flags.toString(16)} compressed=${seg.compressed} codec=${seg.codec} storedLen=${seg.storedLen} rawLen=${seg.rawLen} stored=${Buffer.from(seg.stored).toString('hex')} crc=${Buffer.from(seg.crc).toString('hex')} consumed=${seg.consumed}`);
  console.log(`    raw (inflated)=${seg.raw ? Buffer.from(seg.raw).toString('hex') : 'INFLATE FAILED'}`);
  offset += seg.consumed;
  if (seg.kind === 0) break; // KIND_END
}

console.log("offset after segments", offset, "expected footer at", totalLen-footerSize);

// footer
const footerOffset = totalLen - footerSize;
console.log("footer magic", Buffer.from(bytes.slice(footerOffset, footerOffset+8)).toString());
const fb = bytes.slice(footerOffset);
console.log("footer version_major", fb[8]|(fb[9]<<8));
console.log("footer required_flags", fb[12]|(fb[13]<<8)|(fb[14]<<16)|(fb[15]<<24));
function readU64LE(b, off) {
  let v = 0n;
  for (let i=7;i>=0;i--) v = (v<<8n) | BigInt(b[off+i]);
  return v;
}
console.log("manifest_offset", readU64LE(fb, 16));
console.log("manifest_len", readU64LE(fb, 24));
console.log("file_len", readU64LE(fb, 32));
console.log("content_hash", Buffer.from(fb.slice(40,72)).toString('hex'));
console.log("prev_footer_offset", readU64LE(fb, 72));
console.log("footer_crc", Buffer.from(fb.slice(80,84)).toString('hex'));
