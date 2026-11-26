/** 🥚️ Rounds the two ends of every body that `../MV-3/body.ts --save` found: `bun caps.ts <raw ops.json> <ops.json> [--reach 1] [--share 0.4]`.
 *
 * A body of `body.ts` ends with the section of its last station, so an end that stands where the drawn outline has a level edge is a flat lid as wide as the mass; when the wind tilts the mass the lid shows as a straight cut. This step draws both ends in to a dome: over the distance D from an end (the larger half extent of the end section times `reach`, at most `share` of the length of the body) every section is scaled about its centre by sqrt(1 - (1 - d / D)²), with new sections at 15 degree steps of the dome and a last one of 6 % at the end itself. The rounded body lies inside the body it came from, so the default pose draws what it drew before; nothing else of the ops is touched. The output is a pure function of the input, so the step is idempotent.
 * @see ../MV-3/body.ts — the bodies and the ops format */
import { readFileSync, writeFileSync } from "node:fs";

type Section = { at: number[]; rx: number; ry: number; rotation: number[] };
const args = process.argv.slice(2);
const flag = (name: string, value: number): number => (args.indexOf(`--${name}`) < 0 ? value : Number(args[args.indexOf(`--${name}`) + 1]));
const [REACH, SHARE, TINY, LAST] = [flag("reach", 1), flag("share", 0.4), 5e-4, 0.06];
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const ops = JSON.parse(readFileSync(args[0]!, "utf8"));

/** 🪡️ The section at the distance `d` along the path of `sections` (`along` their cumulative distances), scaled by `scale`. */
function cut(sections: Section[], along: number[], d: number, scale: number): Section {
  let i = 0;
  while (i + 2 < along.length && along[i + 1]! < d) i++;
  const t = Math.max(0, Math.min(1, (d - along[i]!) / (along[i + 1]! - along[i]!)));
  const [a, b] = [sections[i]!, sections[i + 1]!];
  const mix = (m: number, n: number): number => m + (n - m) * t;
  return { at: a.at.map((c, k) => round(mix(c, b.at[k]!))), rx: round(Math.max(TINY, mix(a.rx, b.rx) * scale)), ry: round(Math.max(TINY, mix(a.ry, b.ry) * scale)), rotation: a.rotation };
}

/** 🥚️ The sections with both ends drawn in to a dome. */
function rounded(sections: Section[]): Section[] {
  const along = [0];
  for (let i = 1; i < sections.length; i++) along.push(along[i - 1]! + Math.hypot(...sections[i]!.at.map((c, k) => c - sections[i - 1]!.at[k]!)));
  const length = along[along.length - 1]!;
  const reach = (end: Section): number => (Math.max(end.rx, end.ry) <= 3 * TINY ? 0 : Math.min(Math.max(end.rx, end.ry) * REACH, SHARE * length));
  const [d0, d1] = [reach(sections[0]!), reach(sections[sections.length - 1]!)];
  const scaleAt = (d: number): number => { const [a, b] = [d0 > 0 && d < d0 ? Math.sqrt(Math.max(0, 1 - (1 - d / d0) ** 2)) : 1, d1 > 0 && length - d < d1 ? Math.sqrt(Math.max(0, 1 - (1 - (length - d) / d1) ** 2)) : 1]; return Math.max(LAST, Math.min(a, b)); };
  const stations = new Set<number>(along);
  for (const [from, reachOf, sign] of [[0, d0, 1], [length, d1, -1]] as const) if (reachOf > 0) for (const degrees of [15, 30, 45, 60, 75, 90]) stations.add(from + sign * reachOf * (1 - Math.cos((degrees * Math.PI) / 180)));
  const sorted = [...stations].sort((a, b) => a - b).filter((d, i, list) => i === 0 || d - list[i - 1]! > 1e-4);
  return sorted.map((d) => cut(sections, along, d, scaleAt(d)));
}

let count = 0;
for (const key of Object.keys(ops)) for (const view of Object.keys(ops[key])) for (const entry of ops[key][view].bodies ?? []) { entry.shape.sections = rounded(entry.shape.sections); count++; }
writeFileSync(args[1]!, `${JSON.stringify(ops)}\n`);
console.log(`${count} bodies rounded at both ends (reach ${REACH}, share ${SHARE}) -> ${args[1]}`);
