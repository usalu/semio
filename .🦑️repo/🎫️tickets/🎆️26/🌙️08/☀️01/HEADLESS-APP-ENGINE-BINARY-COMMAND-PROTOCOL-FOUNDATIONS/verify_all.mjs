import zlib from "node:zlib";
import { blake3, hex } from "./blake3_test.mjs";

const fixtures = {
null: "8953504b0d0a1a0a010000000100000001000000ef65ef880000000000000000030303016300001d75fe6e0403060463641414020072ba15e70103332e012e00d1ff0087b135201cabc2b7708ae6d5410d1b10578cbdfbd359c82d21bcea70477f7bd92b0e01200b00000000040100007bc75acd0000007aa3646053504b464f4f5431010000000100000039000000000000003b00000000000000cf000000000000006e3211ee00d53fca02ae29b450f9dc240b01f758753c346d3232e90eaab117ef0000000000000000de539a67",
nested_deep: "8953504b0d0a1a0a010000000100000001000000ef65ef880000000000000000030308066364c9494d4c03004ab8d56604031f2d636414146064674c041149202299879995010c3ed843190c0e208914360600bd5d13240103332e012e00d1ff0087b135201cabc2b7708ae6d5410d1b10578cbdfbd359c82d21bcea70477f7bd93027012010000000002d0100019ea79e4b0000007aa3646053504b464f4f5431010000000100000057000000000000003b00000000000000ed0000000000000074537d8e5057abd9333a231e999b29ee6273a06a13cf15cb72a3361cc766d47c0000000000000000c60ba8d0",
array_ints: "8953504b0d0a1a0a010000000100000001000000ef65ef880000000000000000030303016300001d75fe6e04031420636414e461666500830ff65006830394c1e100000913e8c10103332e012e00d1ff0087b135201cabc2b7708ae6d5410d1b10578cbdfbd359c82d21bcea70477f7bd92b1c01200b00000000200100007092971a0000007aa3646053504b464f4f5431010000000100000047000000000000003b00000000000000dd00000000000000d5241e994636cedd8458f5b3c98baf0bfe9733e5befd654ff7d755a1408a9b170000000000000000e49161d2",
};

function hexToBytes(h) {
  const out = new Uint8Array(h.length / 2);
  for (let i = 0; i < out.length; i++) out[i] = parseInt(h.substr(i * 2, 2), 16);
  return out;
}
function readVarint(bytes, pos) {
  let result = 0n, shift = 1n;
  for (let i = 0; i < 10; i++) {
    const b = bytes[pos[0]]; pos[0]++;
    result += BigInt(b & 0x7f) * shift;
    if ((b & 0x80) === 0) return result;
    shift *= 128n;
  }
}
function readSegmentAt(bytes, offset) {
  const kind = bytes[offset];
  const flags = bytes[offset + 1];
  const compressed = (flags & 1) !== 0;
  const p = [offset + 2];
  const storedLen = Number(readVarint(bytes, p));
  let rawLen = storedLen;
  if (compressed) rawLen = Number(readVarint(bytes, p));
  const payloadStart = p[0];
  const stored = bytes.slice(payloadStart, payloadStart + storedLen);
  const crcStart = payloadStart + storedLen;
  const consumed = crcStart + 4 - offset;
  const raw = compressed ? zlib.inflateRawSync(Buffer.from(stored)) : Buffer.from(stored);
  return { kind, raw, consumed };
}

for (const [name, hexStr] of Object.entries(fixtures)) {
  const bytes = hexToBytes(hexStr);
  const totalLen = bytes.length;
  const footerOffset = totalLen - 84;
  const footerContentHash = Buffer.from(bytes.slice(footerOffset + 40, footerOffset + 72)).toString("hex");
  // find KIND_DOCUMENT segment (kind=4)
  let offset = 32;
  let docRaw = null;
  while (offset < footerOffset) {
    const seg = readSegmentAt(bytes, offset);
    if (seg.kind === 4) { docRaw = seg.raw; }
    offset += seg.consumed;
  }
  const myHash = hex(blake3(new Uint8Array(docRaw)));
  console.log(name, "match:", myHash === footerContentHash, myHash, footerContentHash);
}
