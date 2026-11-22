/** 🔎️ Locates compiled mounted receipt checks in the retained browser core. */
const [path] = Bun.argv.slice(2);
const wasm = Buffer.from(await Bun.file(path).arrayBuffer());
let p = 8, imports = 0;
const code = [], data = [];
function leb(signed = false) {
  let value = 0;
  for (let shift = 0; shift < 49; shift += 7) {
    const octet = wasm[p++];
    value += (octet & 127) * 2 ** shift;
    if (octet < 128) return signed && octet & 64 ? value - 2 ** (shift + 7) : value;
  }
  throw Error("Unexpected LEB extent");
}
function name() { const length = leb(); p += length; }
function limits() { const flag = leb(); leb(); if (flag & 1) leb(); }
while (p < wasm.length) {
  const type = wasm[p++], size = leb(), end = p + size;
  if (type === 2) {
    for (let n = leb(); n > 0; n--) {
      name(); name();
      switch (wasm[p++]) {
        case 0: leb(); imports++; break;
        case 1: p++; limits(); break;
        case 2: limits(); break;
        case 3: p += 2; break;
        case 4: p++; leb(); break;
        default: throw Error("Unexpected import kind");
      }
    }
  }
  if (type === 10) {
    for (let n = leb(), index = imports; n > 0; n--, index++) {
      const size = leb(), start = p;
      code.push({ start, end: p += size, index });
    }
  }
  if (type === 11) {
    for (let n = leb(); n > 0; n--) {
      const flag = leb();
      if (flag === 2) leb();
      let address;
      if (flag !== 1) {
        if (wasm[p++] !== 0x41) throw Error("Nonstatic data");
        address = leb(true);
        if (wasm[p++] !== 0x0b) throw Error("Nonstatic data expression");
      }
      const length = leb();
      if (address !== undefined) data.push({ address, start: p, end: p + length });
      p += length;
    }
  }
  p = end;
}
console.log("code spans", code.length, code[0], code.at(-1), data.length);
function signedBytes(value) {
  const result = [];
  value = BigInt(value);
  while (value >= 64n) { result.push(Number(value & 127n) | 128); value >>= 7n; }
  result.push(Number(value));
  return Buffer.from(result);
}
const needle = Bun.argv[3] ?? "original lifecycle receipt differs from its caller authority";
const literals = [];
for (const segment of data) {
  const position = wasm.indexOf(needle, segment.start);
  if (position < segment.start || position >= segment.end) continue;
  const address = segment.address + position - segment.start;
  console.log("string location", address, position, segment);
  literals.push(address);
  console.log("related text", wasm.subarray(Math.max(segment.start, position - 192), Math.min(position + 384, segment.end)).toString("utf8"));
  for (let delta = -16; delta <= 16; delta += 1) if (delta) literals.push(address + delta);
  const raw = Buffer.alloc(4);
  raw.writeUInt32LE(address);
  for (const related of data) {
    for (let at = wasm.indexOf(raw, related.start), count = 0; at >= related.start && at + 4 <= related.end && count < 32; at = wasm.indexOf(raw, at + 4), count++) {
      console.log("related data pointer", related.address + at - related.start, wasm.subarray(at - 8, at + 16).toString("hex"));
      literals.push(related.address + at - related.start);
    }
  }
  const pair = Buffer.alloc(8);
  pair.writeUInt32LE(address, 0);
  pair.writeUInt32LE(Buffer.byteLength(needle), 4);
  for (const related of data) {
    const table = wasm.indexOf(pair, related.start);
    if (table >= related.start && table < related.end) literals.push(related.address + table - related.start);
  }
}
if (!literals.length) throw Error("Missing exact lifecycle error literal");
for (const address of literals) {
  const immediates = [Buffer.concat([Buffer.from([0x41]), signedBytes(address)]), Buffer.concat([Buffer.from([0x42]), signedBytes(BigInt(Buffer.byteLength(needle)) * 2n ** 32n + BigInt(address))])];
  immediates.push(Buffer.from([0x41, (address & 127) | 128, ((address >>> 7) & 127) | 128, ((address >>> 14) & 127) | 128, ((address >>> 21) & 127) | 128, (address >>> 28) & 127]));
  for (const pattern of immediates) {

   for (let pc = wasm.indexOf(pattern, code[0].start); pc !== -1 && pc < code.at(-1).end; pc = wasm.indexOf(pattern, pc + pattern.length)) {
    let lo = 0, hi = code.length - 1;
    while (lo < hi) { const mid = (lo + hi) >>> 1; if (code[mid].end <= pc) lo = mid + 1; else hi = mid; }
    const func = code[lo];
    console.log("compiled reference", func.index, func.start, func.end, pc, pc - func.start);
  }
}
}
