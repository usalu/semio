import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { ROOT } from "./patch.ts";
const canon: Record<string, [number, number]> = { "50": [20, 75], "100": [75, 150], "200": [150, 350], "500": [350, 1000] };
let changed = 0;
for (const cat of ["🪑️objects", "🛋️furniture"]) for (const slug of readdirSync(`${ROOT}/${cat}`)) {
  const file = `${ROOT}/${cat}/${slug}/🔣️.json`;
  let text = readFileSync(file, "utf8");
  const reps = text.indexOf('"representations"');
  let touched = false;
  for (const [band, [lo, hi]] of Object.entries(canon)) {
    const m = new RegExp(`\\n( *)"${band}": \\{\\s*"status": "authored"`).exec(text.slice(reps));
    if (!m) continue;
    const from = reps + m.index;
    const vr = /"validRange": \{\s*"minDenominator": (\d+),\s*"maxDenominator": (\d+)\s*\}/.exec(text.slice(from));
    if (!vr) throw new Error(`${slug} ${band} no validRange`);
    if (+vr[1]! === lo && +vr[2]! === hi) continue;
    const start = from + vr.index;
    const fixed = vr[0].replace(/("minDenominator": )\d+/, `$1${lo}`).replace(/("maxDenominator": )\d+/, `$1${hi}`);
    text = text.slice(0, start) + fixed + text.slice(start + vr[0].length);
    touched = true;
    changed++;
  }
  if (touched) writeFileSync(file, text);
}
console.log("ranges changed", changed);
