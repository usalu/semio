/** ⏱️ F3 — throughput of the first-party TS BLAKE3 (`🔏️hash/🟦️.ts` blake3Hex) on 20 MB. usage: bun bench-current.ts [module] */
import { blake3Hex as head } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔏️hash/🟦️.ts";
const bytes = new Uint8Array(20 * 2 ** 20);
for (let i = 0; i < bytes.length; i++) bytes[i] = i % 251;
const module = process.argv[2] ? ((await import(process.argv[2])) as { blake3Hex: typeof head }) : { blake3Hex: head };
for (let round = 0; round < 3; round++) {
  const t0 = performance.now();
  const hex = module.blake3Hex(bytes);
  const ms = performance.now() - t0;
  console.log(`${hex.slice(0, 16)} ${ms.toFixed(0)} ms ${(bytes.length / 2 ** 20 / (ms / 1000)).toFixed(1)} MB/s`);
}
