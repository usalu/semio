/** 🌸️ The multiview repair of `common-lilac` at 1:500 (idempotent, a step of its chain before `carry.ts`): the stem cluster `root-collar` is one cone from a radius of 0.24 m at the ground to 0.60 m at 1.45 m (`volumes.500.body`); side and front elevation draw that cone, the plan drew the circle of its foot (0.24 m), so the three views of the bone differed by 0.36 m in the default pose (cause G of `multiview-diagnosis.md`). Seen from above the cone is the circle of its widest reach: the plan circle takes the radius 0.60 m of the top of the loft, read from the body.
 *  bun fix-common-lilac.ts [<file.json>] [--out file] [--write] */
import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]?.endsWith(".json") ? args[0] : "common-lilac");
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const loft = next.asset.volumes["500"].body.find((solid: any) => solid.id === "root-collar" && solid.kind === "loft");
const shape = next.asset.representations["500"].geometry.plan.find((candidate: any) => candidate.id === "root-collar" && candidate.bone === "root-plate" && candidate.kind === "poly");
if (loft === undefined || shape === undefined) throw new Error("common-lilac: loft or plan shape root-collar of 1:500 not found");
const radius = Math.max(...loft.sections.map((section: any) => Math.max(section.rx, section.ry)));
const n = shape.points.length;
shape.points = shape.points.map((_: unknown, i: number) => [Number((radius * Math.cos((2 * Math.PI * i) / n)).toFixed(4)), Number((radius * Math.sin((2 * Math.PI * i) / n)).toFixed(4))]);
const written = rewrite(text, previous, next);
console.log(`common-lilac: plan root-collar of 1:500 is a circle of ${radius} m (${n} points); ${written === text ? "unchanged" : "changed"}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || out !== undefined) writeFileSync(out ?? file, written);
}
