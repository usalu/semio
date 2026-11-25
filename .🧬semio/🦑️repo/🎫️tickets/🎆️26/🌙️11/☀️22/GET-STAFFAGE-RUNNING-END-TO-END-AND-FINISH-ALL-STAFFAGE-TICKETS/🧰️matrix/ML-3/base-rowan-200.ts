/** 🌳️ Sets the 1:200 front and plan of `rowan` back to one flat outline per part, the step of its chain that replaces `../MS/make-rowan.sh`, idempotent: `bun base-rowan-200.ts [--in file] [--out file] [--write]`.
 *
 * Batch MS had redrawn the 1:200 front and plan as stacks of depth slabs 0.6 m apart (`🧰️silhouette/R4-P/redraw.ts`, 93 and 120 shapes): they draw broken contour lines in the default pose and ticks, notches and nested rings in the wind poses (sheets of 2026-11-25, `multiview-ML-3.md`). The run sets the 27 front and 23 plan shapes of `ops/rowan-200-base.json` in their place (absolute values: the outlines as authored before that redraw; the 1:200 elevation and the 1:200 body did not change since, so they are still the true views of that body at rest). The foliage masses then get their depth from `../MV-L1/loft.ts` (the body of each mass itself), the next step of the chain (`make-rowan.sh`).
 * @see ../../🧰️silhouette/R4-P/preserve.ts — `spans`, by which only the two lists are rewritten in the text */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileOf } from "../MR/lib.ts";
import { spans } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = flag("in") ?? fileOf("rowan");
const text = readFileSync(file, "utf8");
const next = JSON.parse(text);
const base = JSON.parse(readFileSync(join(dirname(Bun.main), "ops", "rowan-200-base.json"), "utf8"));
const geometry = next.asset.representations[base.scale].geometry;
const inline = (value: unknown): string => (Array.isArray(value) ? `[${value.map(inline).join(", ")}]` : value !== null && typeof value === "object" ? `{${Object.entries(value).map(([key, member]) => `${JSON.stringify(key)}: ${inline(member)}`).join(", ")}}` : JSON.stringify(value));
const node = (path: readonly string[]): { start: number; end: number } => path.reduce((at: any, key) => at.members.find((member: any) => member.key === key).value, spans(text));
const edits: { start: number; end: number; text: string }[] = [];
let replaced = 0;
for (const view of ["front", "plan"] as const) {
  const slabs = (geometry[view] as any[]).filter((shape) => shape.solid !== undefined && shape.kind !== "loft").length;
  if (slabs === 0) continue;
  const at = node(["asset", "representations", base.scale, "geometry", view]);
  const indent = /^[ \t]*/u.exec(text.slice(text.lastIndexOf("\n", at.start - 1) + 1, at.start))![0];
  edits.push({ start: at.start, end: at.end, text: `[\n${(base[view] as unknown[]).map((shape) => `${indent}  ${inline(shape)}`).join(",\n")}\n${indent}]` });
  geometry[view] = structuredClone(base[view]);
  replaced += slabs;
}
let written = text;
for (const edit of edits.sort((p, q) => q.start - p.start)) written = written.slice(0, edit.start) + edit.text + written.slice(edit.end);
if (JSON.stringify(JSON.parse(written)) !== JSON.stringify(next)) throw new Error("rowan: the rewritten text does not hold the new document");
console.log(`rowan: ${replaced} depth slabs of the 1:200 front and plan replaced by ${base.front.length} and ${base.plan.length} flat outlines; ${written === text ? "unchanged" : "changed"}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || (out !== undefined && !existsSync(out))) writeFileSync(out ?? file, written);
}
