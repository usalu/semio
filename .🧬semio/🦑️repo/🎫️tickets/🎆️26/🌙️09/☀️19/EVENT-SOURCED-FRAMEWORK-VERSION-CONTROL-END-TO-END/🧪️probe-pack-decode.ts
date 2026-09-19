import { decodePackValue } from "../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
const samples: Record<string, number[]> = {
  opBinaryLike: [1, 1, 0, 1, 0, 8, 1, 0xdd],
  revertTransition: [0, 1, 4, 0x6f, 0x70, 0x2d, 0x61],
};
for (const [name, bytes] of Object.entries(samples)) {
  try { console.log(name, JSON.stringify(decodePackValue(Uint8Array.from(bytes)))); } catch (error) { console.log(name, "THROWS", (error as Error).message); }
}
