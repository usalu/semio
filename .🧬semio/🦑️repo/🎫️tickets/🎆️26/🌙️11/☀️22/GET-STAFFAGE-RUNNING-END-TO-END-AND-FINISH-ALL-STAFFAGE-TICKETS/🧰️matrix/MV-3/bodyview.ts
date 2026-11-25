/** 🪵️ Draws single parts of a plant in one view from their body, a step of the chain of a plant before `carry.ts`: `bun bodyview.ts <id|file.json> --work <dir> [--out file] [--write]`, configured by `bodyview/<id>.json`:
 * - `redraw`: a list of `{ scale, view, bones }`. Where the front and the plan draw one part (same id, same bone) with different lateral extents in the rest pose, one of the two is not the part the body holds (cause G of `multiview-diagnosis.md`; `parts.ts` names the view that leaves the body). The named view of each named bone is drawn again as the true outline of the body solids of that bone: `🧰️silhouette/R4-P/redraw.ts --flat` restricted to the solids of these bones (every other solid is passed as `--skip`), one flat outline per bone in the look of the shape it replaces. The outline keeps the id of the single shape it replaces, so the suite goes on comparing it with its twin.
 * - `rename`: a list of `{ scale, view, bone, from, to }` for a shape that carries the id of another part of its bone (the footprint of a crown under the id of the stem piece): it takes the name of what it draws.
 * The run is idempotent: a second run draws the same outlines from the same body.
 * @see ./parts.ts — the measurement that names the view
 * @see ../../🧰️silhouette/R4-P/redraw.ts — the drawing of a part from its body
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const id: string = JSON.parse(text).asset.id;
const here = dirname(Bun.main);
const config = JSON.parse(readFileSync(join(here, "bodyview", `${id}.json`), "utf8"));
const work = flag("work");
if (work === undefined) throw new Error("bodyview: --work <dir> is required");
mkdirSync(work, { recursive: true });
const COMPARED = ["body", "detail", "glazing"];
const escape = (value: string): string => value.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
let current = text;
let step = 0;
for (const entry of config.redraw ?? []) {
  const doc = JSON.parse(current);
  const body = doc.asset.volumes[String(entry.scale)].body as any[];
  const wanted = body.filter((solid) => entry.bones.includes(solid.bone)).map((solid) => solid.id as string);
  const missing = entry.bones.filter((bone: string) => !body.some((solid) => solid.bone === bone));
  if (missing.length > 0) throw new Error(`${id} 1:${entry.scale}: no body solid on ${missing.join(", ")}`);
  const before = new Map<string, string>();
  for (const bone of entry.bones) {
    const replaced = (doc.asset.representations[String(entry.scale)].geometry[entry.view] as any[]).filter((shape) => shape.bone === bone && COMPARED.includes(shape.role) && !(shape.filled === false && (shape.kind === "path" || (shape.kind === "poly" && shape.closed !== true))));
    if (replaced.length === 1) before.set(bone, replaced[0].id);
  }
  const input = join(work, `${id}-bodyview-${step}.json`);
  const output = join(work, `${id}-bodyview-${step}-out.json`);
  writeFileSync(input, current);
  const run = Bun.spawnSync([process.execPath, join(here, "..", "..", "🧰️silhouette", "R4-P", "redraw.ts"), id, "--in", input, "--out", output, "--scales", String(entry.scale), "--views", entry.view, "--flat", "--skip", `^(?!(?:${wanted.map(escape).join("|")})$)`], { cwd: join(S, "📦️packages", "🟦️typescript"), stdout: "pipe", stderr: "pipe" });
  if (run.exitCode !== 0 || !existsSync(output)) throw new Error(`${id} 1:${entry.scale} ${entry.view}: redraw.ts failed: ${run.stderr.toString().slice(-600)}`);
  console.log(`[DEBUG] ${run.stdout.toString().trim().split("\n").slice(-2).join(" | ").slice(0, 300)}`);
  const drawnText = readFileSync(output, "utf8");
  const previous = JSON.parse(drawnText);
  const next = structuredClone(previous);
  const list = next.asset.representations[String(entry.scale)].geometry[entry.view] as any[];
  let kept = 0;
  for (const bone of entry.bones) {
    const outline = list.filter((shape) => shape.bone === bone && shape.id === `${bone}-outline`);
    if (outline.length !== 1) throw new Error(`${id} 1:${entry.scale} ${entry.view}: ${outline.length} outlines of ${bone}`);
    const old = before.get(bone);
    if (old !== undefined && !list.some((shape) => shape.id === old)) { outline[0].id = old; kept++; }
  }
  current = rewrite(drawnText, previous, next);
  console.log(`${id} 1:${entry.scale} ${entry.view}: ${entry.bones.length} parts drawn from their body (${wanted.length} solids), ${kept} keep the id of the shape they replace`);
  step++;
}
if ((config.rename ?? []).length > 0) {
  const previous = JSON.parse(current);
  const next = structuredClone(previous);
  for (const entry of config.rename) {
    const list = next.asset.representations[String(entry.scale)].geometry[entry.view] as any[];
    if (list.some((shape) => shape.id === entry.to && shape.bone === entry.bone)) continue;
    const shape = list.find((candidate) => candidate.id === entry.from && candidate.bone === entry.bone);
    if (shape === undefined) throw new Error(`${id} 1:${entry.scale} ${entry.view}: no shape "${entry.from}" on ${entry.bone}`);
    shape.id = entry.to;
    console.log(`${id} 1:${entry.scale} ${entry.view}: "${entry.from}" on ${entry.bone} is named "${entry.to}"`);
  }
  current = rewrite(current, previous, next);
}
console.log(`${id}: ${current === text ? "unchanged" : `changed (${text.length} -> ${current.length} characters)`}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (current !== text || (out !== undefined && !existsSync(out))) writeFileSync(out ?? file, current);
}
