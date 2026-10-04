/** 🔬️ Ticket tool of work package D1 (round 2): prints one case of `🗑️generated/d1/gear-bits.json` with its arguments and answers decoded from their bit patterns, to find out why a twin disagrees on it.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d1_probe_case.ts <case index>
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const view = new DataView(new ArrayBuffer(8));

/** 🔢️ The number a bit pattern stands for. */
function decode(text: string): number {
  if (text === "nan") return Number.NaN;
  view.setBigUint64(0, BigInt(`0x${text}`));
  return view.getFloat64(0);
}

const cases = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "d1", "gear-bits.json"), "utf8")) as { fn: string; args: string[]; out: string[] | "throws" }[];
const index = Number(process.argv[2]);
const found = cases[index]!;
console.log(`[DEBUG] case ${index} ${found.fn}`);
console.log(`[DEBUG] args ${JSON.stringify(found.args.map(decode))}`);
console.log(`[DEBUG] out ${found.out === "throws" ? "throws" : JSON.stringify(found.out.map(decode))}`);
