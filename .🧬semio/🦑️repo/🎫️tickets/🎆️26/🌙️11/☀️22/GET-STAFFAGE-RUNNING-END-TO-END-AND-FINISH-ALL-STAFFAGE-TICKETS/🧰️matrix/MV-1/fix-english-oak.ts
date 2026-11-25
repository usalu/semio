/** 🌰️ The multiview repair of `english-oak` at 1:200 (idempotent, a step of its chain after `rebind.ts --stem` and before `carry.ts`): the plan leaf patches `sv1-200-p-202` and `sv1-200-p-203` of the silhouette round lie 37.5 mm apart (0.1875 mm on paper between the centrelines of two 0.18 mm pens: an ink gap of 0.0075 mm, below the 0.15 mm of the print profile `office-laser`). On the askew limb bone `br-e-low` the default pose foreshortened the pair until the pens met; on the stem bone `rebind.ts --stem` binds them to they keep their distance and the scale stops printing. The two patches are one leaf mass: the vertex of `sv1-200-p-202` nearest to `sv1-200-p-203` moves a quarter of the distance beyond the nearest point of that patch, so the outlines cross and the patches are joined in every pose.
 *  bun fix-english-oak.ts [<file.json>] [--out file] [--write]
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts — the gap rule (same bone, same `z`, boxes apart) */
import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

type P = [number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]?.endsWith(".json") ? args[0] : "english-oak");
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const representation = next.asset.representations["200"];
const find = (id: string): any => representation.geometry.plan.find((shape: any) => shape.id === id && shape.kind === "path");
const moved = find("sv1-200-p-202");
const fixed = find("sv1-200-p-203");
if (moved === undefined || fixed === undefined) throw new Error("english-oak: plan patches sv1-200-p-202 and sv1-200-p-203 of 1:200 not found");
const pointsOf = (d: string): P[] => {
  if (!/^M [-\d. ]+(?: L [-\d. ]+)* Z$/u.test(d)) throw new Error("english-oak: a patch is not one closed ring of straight lines");
  return d.slice(2, -2).split(" L ").map((pair) => pair.split(" ").map(Number) as P);
};
const nearestOn = (p: P, a: P, b: P): P => {
  const dx = b[0] - a[0], dy = b[1] - a[1];
  const t = dx * dx + dy * dy === 0 ? 0 : Math.max(0, Math.min(1, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / (dx * dx + dy * dy)));
  return [a[0] + t * dx, a[1] + t * dy];
};
const ring = pointsOf(moved.d);
const other = pointsOf(fixed.d);
let best = { d: Infinity, i: -1, to: [0, 0] as P };
ring.forEach((p, i) => other.forEach((a, j) => {
  const q = nearestOn(p, a, other[(j + 1) % other.length]!);
  const d = Math.hypot(p[0] - q[0], p[1] - q[1]);
  if (d < best.d) best = { d, i, to: q };
}));
const pen = moved.penMm ?? representation.pens[moved.role];
const inkMm = (best.d * 1000) / 200 - pen;
if (inkMm > 0 && inkMm < 0.15) {
  const from = ring[best.i]!;
  ring[best.i] = [Number((from[0] + 1.25 * (best.to[0] - from[0])).toFixed(4)), Number((from[1] + 1.25 * (best.to[1] - from[1])).toFixed(4))];
  moved.d = `M ${moved.d.slice(2, -2).split(" L ").map((pair: string, i: number) => (i === best.i ? ring[i]!.join(" ") : pair)).join(" L ")} Z`;
}
const written = rewrite(text, previous, next);
console.log(`english-oak: plan patches sv1-200-p-202 and sv1-200-p-203 of 1:200 were ${(best.d * 1000).toFixed(1)} mm apart (ink gap ${inkMm.toFixed(4)} mm); ${written === text ? "unchanged" : `vertex ${best.i} moved to ${ring[best.i]!.join(" ")}`}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || out !== undefined) writeFileSync(out ?? file, written);
}
