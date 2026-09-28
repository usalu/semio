/** 🧪️ F3 — the candidate BLAKE3 against the current first-party port: every length 0…3100 and the chunk/tree edges up
 * to 1 MB + 1 in one update, the same inputs split into random update runs (incl. unaligned sub-arrays), and XOF lengths.
 * usage: bun differential.ts */
import { Blake3Hasher as Reference, blake3Hex as referenceHex } from "./head-hash.ts";
import { Blake3Hasher as Candidate, blake3Hex as candidateHex } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔏️hash/🟦️.ts";
const input = (length: number): Uint8Array => Uint8Array.from({ length }, (_, index) => index % 251);
const lengths = [...Array.from({ length: 3101 }, (_, index) => index), 4095, 4096, 4097, 8191, 8192, 8193, 16383, 16384, 16385, 31744, 65535, 65536, 65537, 102400, 1048575, 1048576, 1048577];
let failures = 0;
let seed = 7;
const random = (limit: number): number => ((seed = (seed * 1103515245 + 12345) >>> 0) % limit);
for (const length of lengths) {
  const bytes = input(length);
  const expected = referenceHex(bytes);
  if (candidateHex(bytes) !== expected) { failures++; if (failures < 5) console.log(`one-shot mismatch at ${length}`); }
  if (length % 97 === 0 || length > 3100) {
    const padded = new Uint8Array(length + 3);
    padded.set(bytes, 1 + (length % 3));
    const view = padded.subarray(1 + (length % 3), 1 + (length % 3) + length);
    const hasher = new Candidate();
    for (let offset = 0; offset < length; ) {
      const take = Math.min(length - offset, 1 + random(2100));
      hasher.update(view.subarray(offset, offset + take));
      offset += take;
    }
    const hex = Array.from(hasher.digest(), (byte) => byte.toString(16).padStart(2, "0")).join("");
    if (hex !== expected) { failures++; if (failures < 5) console.log(`split mismatch at ${length}`); }
  }
}
for (const outLen of [1, 31, 32, 33, 64, 65, 131]) for (const length of [0, 1, 1024, 1025, 5000]) {
  const [reference, candidate] = [new Reference(), new Candidate()];
  reference.update(input(length));
  candidate.update(input(length));
  if (reference.digest(outLen).join() !== candidate.digest(outLen).join()) { failures++; console.log(`xof mismatch ${length}/${outLen}`); }
}
console.log(`${lengths.length} lengths, failures ${failures}`);
process.exit(failures === 0 ? 0 : 1);
