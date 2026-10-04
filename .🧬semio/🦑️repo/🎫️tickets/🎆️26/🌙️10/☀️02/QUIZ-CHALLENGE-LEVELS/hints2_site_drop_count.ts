/** ✂️ Drops the round-2 `COUNT` pattern block (docstring and const) and the unused `reads` from the challenge-levels
 * spec; its regexes carry U+00A0, which the Edit tool cannot match. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const spec = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "❓️quiz", "🧪️tests", "⛰️challenge-levels", "🟦️.ts");
let text = readFileSync(spec, "utf8");
const cut = (from: string, through: string): void => {
  const start = text.indexOf(from);
  if (start < 0) throw new Error(`missing ${from}`);
  const end = text.indexOf(through, start) + through.length;
  text = text.slice(0, start) + text.slice(end).replace(/^\n+/u, "");
};
cut("/** 🔢️ A count as a hint writes it", "\n};\n");
cut("/** 🟰️ The question reads exactly", "\n}\n");
writeFileSync(spec, text);
console.log("done");
