/** 🧷️ Keeps the other guards green while the views are nested: `bun nsettle.ts <id>` (run from `S/📦️packages/🟦️typescript`, env of `../MR/env.sh` with BATCH=MA) rebuilds the trial copy `$GEN/nest/<id>.json` with `nstage.sh build` (from the backup or, with NEST_BASE, from the copy of the single-solid chain), compares the silhouette guard (IoU, largest boundary distance, bounding box per scale and view), `validateAsset` and the three-views sweep of the copy with those of the backup and, while the copy fails a check the backup passes, puts the nested shapes of that view back one twin family at a time, takes the family whose return helps most (a family named in `protect/<id>.json` as `"<scale> <view> <family>"` only when no other family helps: the families that carry the last hard findings) and removes its ops from `nest/<id>.json` (the shape stays as authored and its sync finding stays, to be reported). When the copy is as good as the backup, `which.ts` names the solids the evaluator cannot draw at a subject of the matrix; their ops are removed too and the loop starts again. Ends when the copy is as good as the backup and every subject evaluates, or no family helps.
 * @see ./settle.ts — the same loop for the single-solid ops of `fit.ts`
 * @see ./nest.ts — the repair */
import { join } from "node:path";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { S, readDoc } from "../MR/lib.ts";

const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { validateAsset } = await import(join(S, "📦️packages/🟦️typescript/🟦️.ts"));
const { authoredScales, consistency, sweep } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));
const id = process.argv[2]!;
const { GEN, W, B, T } = process.env as Record<string, string>;
const opsFile = join(W, "nest", `${id}.json`);
const copyFile = join(GEN, "nest", `${id}.json`);
const base = (name: string): string => name.replace(/-nest-\d+$/u, "");
const family = (name: string): string => base(name).split("-").filter((token) => !["n", "f", "near", "far", "l", "r"].includes(token)).map((token) => token.replace(/^(h|f)(n|f)$/u, "$1")).join("-");
const viewOf = (op: any): string | undefined => op.view ?? op.to ?? (op.op === "side" ? "elevation" : undefined);

type Check = { key: string; scale?: string; view?: string; value: number };
function checks(asset: any, only?: { scale: string; view: string }, raw = false): Check[] {
  const out: Check[] = [];
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const scale of [50, 100, 200, 500].filter((d) => asset.volumes?.[String(d)]?.status === "authored" && asset.representations[String(d)]?.status === "authored" && (only === undefined || only.scale === String(d)))) {
    const feature = featureOf(scale);
    let m: any;
    try {
      m = measureAsset(asset, scale, undefined, undefined, feature);
    } catch (error) {
      out.push({ key: `${scale} measure-error`, scale: String(scale), value: 1 });
      continue;
    }
    m.bbox.forEach((b: number, k: number) => (raw || Math.abs(b - dims[k]!) > feature) && out.push({ key: `${scale} bbox ${k}`, scale: String(scale), value: Math.abs(b - dims[k]!) }));
    for (const view of VIEWPOINTS as string[]) {
      if (raw || m.views[view].iou < 0.92) out.push({ key: `${scale} ${view} iou`, scale: String(scale), view, value: m.views[view].iou });
      if (raw || m.views[view].hausdorffM > 2 * feature) out.push({ key: `${scale} ${view} distance`, scale: String(scale), view, value: m.views[view].hausdorffM });
    }
  }
  if (only !== undefined) return out;
  for (const issue of validateAsset(asset) as { code: string; path?: string }[]) {
    const where = /representations\.(\d+)\.geometry\.(\w+)/u.exec(issue.path ?? "") ?? /^volumes\.(\d+)()/u.exec(issue.path ?? "");
    out.push({ key: `validate ${issue.code} ${where ? `${where[1]} ${where[2]}` : (issue.path ?? "")}`, scale: where?.[1], view: where?.[2] || undefined, value: 1 });
  }
  for (const scale of authoredScales(asset) as number[])
    for (const finding of [...sweep(asset, scale), ...consistency(asset, scale)] as { check: string }[]) {
      const view = /front/u.test(finding.check) ? "front" : /plan/u.test(finding.check) ? "plan" : undefined;
      out.push({ key: `sweep ${finding.check} ${scale}`, scale: String(scale), view, value: 1 });
    }
  return out;
}

const readOps = (): any[] => JSON.parse(readFileSync(opsFile, "utf8"));
const writeOps = (ops: any[]): void => writeFileSync(opsFile, JSON.stringify(ops, null, 1));
const before = readDoc(join(GEN, "backup", `${id}.json`)).doc.asset;
const origin = process.env.NEST_BASE === undefined ? before : readDoc(join(process.env.NEST_BASE, `${id}.json`)).doc.asset;
const allowed = new Set(checks(before).map((check) => check.key));
const dead = new Set<string>();
const guarded = new Set<string>(existsSync(join(W, "protect", `${id}.json`)) ? JSON.parse(readFileSync(join(W, "protect", `${id}.json`), "utf8")) : []);
let cured = 0;
for (let round = 1; round <= 60; round++) {
  const run = Bun.spawnSync([join(W, "nstage.sh"), "build", id], { env: process.env });
  const after = readDoc(copyFile).doc.asset;
  const failing = checks(after).filter((check) => !allowed.has(check.key));
  const open = failing.filter((check) => !dead.has(check.key));
  console.log(`[round ${round}] ${run.stdout.toString().trim().split("\n").at(-1)?.slice(0, 160)} | failing: ${failing.map((check) => `${check.key}${check.value === 1 ? "" : ` ${check.value.toFixed(4)}`}`).join(", ") || "none"}`);
  if (open.length === 0) {
    if (cured >= 8) break;
    const which = Bun.spawnSync([B!, join(W, "which.ts"), copyFile], { env: process.env, cwd: T });
    const named = which.stdout.toString().split("\n").filter((line) => line.startsWith("{")).map((line) => JSON.parse(line) as { scale: string; view: string; id: string; bone?: string; subject: string });
    if (named.length === 0) {
      console.log("  every subject evaluates");
      break;
    }
    cured++;
    const ops = readOps();
    const next = ops.filter((op) => !named.some((hit) => hit.scale === op.scale && hit.view === viewOf(op) && base(hit.id) === op.id && (op.bone === undefined || hit.bone === undefined || op.bone === hit.bone)));
    console.log(`  the evaluator cannot draw ${named.map((hit) => `1:${hit.scale} ${hit.view} ${hit.id} @ ${hit.subject}`).join("; ")}: ${ops.length - next.length} ops removed`);
    if (next.length === ops.length) {
      console.log("  no op is left for them, left open");
      break;
    }
    writeOps(next);
    continue;
  }
  const check = open[0]!;
  if (check.scale === undefined) {
    dead.add(check.key);
    console.log(`  ${check.key}: not bound to a scale, left open`);
    continue;
  }
  const views = check.view !== undefined ? [check.view] : ["front", "plan"];
  const ops = readOps();
  const counted = check.key.startsWith("validate") || check.key.startsWith("sweep") || check.key.endsWith("measure-error");
  const score = (): number => {
    if (counted) return checks(after).filter((other) => other.key === check.key).length;
    return checks(after, { scale: check.scale!, view: views[0]! }, true).find((other) => other.key === check.key)?.value ?? Number.POSITIVE_INFINITY;
  };
  const higher = check.key.endsWith("iou");
  const start = score();
  let best: { view: string; name: string; gain: number; kept: boolean } | undefined;
  for (const view of views) {
    const geometry = after.representations[check.scale!].geometry;
    const list = geometry[view] as any[];
    const old = origin.representations[check.scale!].geometry[view] as any[];
    const touched = new Set(ops.filter((op) => op.scale === check.scale && viewOf(op) === view).map((op) => family(op.id)));
    for (const name of touched) {
      geometry[view] = list.filter((shape) => !(family(shape.id) === name && base(shape.id) !== shape.id)).map((shape) => (family(shape.id) === name ? (old.find((other) => other.id === shape.id && other.bone === shape.bone) ?? shape) : shape));
      const value = score();
      const gain = higher ? value - start : start - value;
      geometry[view] = list;
      const kept = guarded.has(`${check.scale} ${view} ${name}`);
      if (gain > 1e-6 && (best === undefined || (best.kept && !kept) || (best.kept === kept && gain > best.gain))) best = { view, name, gain, kept };
    }
  }
  if (best === undefined) {
    const rest = ops.filter((op) => op.scale !== check.scale || !views.includes(viewOf(op) ?? ""));
    if (rest.length === ops.length) {
      dead.add(check.key);
      console.log(`  ${check.key}: no nested family helps and no op is left for it, left open`);
    } else {
      writeOps(rest);
      console.log(`  ${check.key}: no single family helps: all ${ops.length - rest.length} ops of 1:${check.scale} ${views.join(" and ")} removed`);
    }
    continue;
  }
  const next = ops.filter((op) => op.scale !== check.scale || viewOf(op) !== best!.view || family(op.id) !== best!.name);
  writeOps(next);
  console.log(`  ${check.key}: family "${best.name}" in the ${best.view} helps by ${best.gain.toFixed(4)}: ${ops.length - next.length} ops removed`);
  if (next.length === ops.length) dead.add(check.key);
}
