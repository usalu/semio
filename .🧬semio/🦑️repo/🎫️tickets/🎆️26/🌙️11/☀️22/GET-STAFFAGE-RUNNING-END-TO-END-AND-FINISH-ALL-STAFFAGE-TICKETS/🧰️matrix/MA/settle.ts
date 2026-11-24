/** ⚖️ Keeps the other guards green while the views are refitted: `bun settle.ts <id>` (run from `S/📦️packages/🟦️typescript`, env of `../MR/env.sh` with BATCH=MA) rebuilds the trial copy with `refit.sh`, compares the silhouette guard (IoU, largest boundary distance, bounding box per scale and view), `validateAsset` and the three-views sweep of the copy with those of the backup and, while the copy fails a check the backup passes, puts the changed shapes of that view back one twin family at a time (as `blame.ts`), takes the family whose return helps most and tightens its ops in `ops/<id>.json`: a `solid` op becomes `"strict": true` (a construction is only taken when it keeps the footprint within the whole tolerance and the default-pose outline within one feature, else the shape stays flat), a strict or other op is removed (the shape stays as it was and its sync finding stays, to be reported). Ends when the copy is as good as the backup or no family helps. */
import { join } from "node:path";
import { readFileSync, writeFileSync } from "node:fs";
import { S, readDoc } from "../MR/lib.ts";

const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { validateAsset } = await import(join(S, "📦️packages/🟦️typescript/🟦️.ts"));
const { authoredScales, consistency, sweep } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));
const id = process.argv[2]!;
const { GEN, W } = process.env as Record<string, string>;
const opsFile = join(W, "ops", `${id}.json`);
const family = (name: string): string => name.split("-").filter((token) => !["n", "f", "near", "far", "l", "r"].includes(token)).map((token) => token.replace(/^(h|f)(n|f)$/u, "$1")).join("-");

type Check = { key: string; scale?: string; view?: string; value: number; better: (a: number, b: number) => boolean };
function checks(asset: any, only?: { scale: string; view: string }, raw = false): Check[] {
  const out: Check[] = [];
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const scale of [50, 100, 200, 500].filter((d) => asset.volumes?.[String(d)]?.status === "authored" && asset.representations[String(d)]?.status === "authored" && (only === undefined || only.scale === String(d)))) {
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    m.bbox.forEach((b: number, k: number) => (raw || Math.abs(b - dims[k]!) > feature) && out.push({ key: `${scale} bbox ${k}`, scale: String(scale), value: Math.abs(b - dims[k]!), better: (a, c) => a < c }));
    for (const view of VIEWPOINTS as string[]) {
      if (raw || m.views[view].iou < 0.92) out.push({ key: `${scale} ${view} iou`, scale: String(scale), view, value: m.views[view].iou, better: (a, c) => a > c });
      if (raw || m.views[view].hausdorffM > 2 * feature) out.push({ key: `${scale} ${view} distance`, scale: String(scale), view, value: m.views[view].hausdorffM, better: (a, c) => a < c });
    }
  }
  if (only !== undefined) return out;
  for (const issue of validateAsset(asset) as { code: string; path?: string }[]) {
    const where = /representations\.(\d+)\.geometry\.(\w+)/u.exec(issue.path ?? "");
    out.push({ key: `validate ${issue.code} ${where ? `${where[1]} ${where[2]}` : (issue.path ?? "")}`, scale: where?.[1], view: where?.[2], value: 1, better: () => false });
  }
  for (const scale of authoredScales(asset) as number[])
    for (const finding of [...sweep(asset, scale), ...consistency(asset, scale)] as { check: string }[]) {
      const view = /front/u.test(finding.check) ? "front" : /plan/u.test(finding.check) ? "plan" : undefined;
      out.push({ key: `sweep ${finding.check} ${scale}`, scale: String(scale), view, value: 1, better: () => false });
    }
  return out;
}

const before = readDoc(join(GEN, "backup", `${id}.json`)).doc.asset;
const allowed = new Set(checks(before).map((check) => check.key));
const dead = new Set<string>();
for (let round = 1; round <= 40; round++) {
  const run = Bun.spawnSync([join(W, "refit.sh"), id], { env: process.env });
  const after = readDoc(join(GEN, "try", `${id}.json`)).doc.asset;
  const failing = checks(after).filter((check) => !allowed.has(check.key));
  const open = failing.filter((check) => !dead.has(check.key));
  console.log(`[round ${round}] ${run.stdout.toString().trim().split("\n").at(-1)} | failing: ${failing.map((check) => `${check.key}${check.value === 1 ? "" : ` ${check.value.toFixed(4)}`}`).join(", ") || "none"}`);
  if (open.length === 0) break;
  const check = open[0]!;
  const views = check.view !== undefined ? [check.view] : ["front", "plan"];
  const ops = JSON.parse(readFileSync(opsFile, "utf8")) as any[];
  let best: { view: string; name: string; gain: number } | undefined;
  const score = (): number => {
    if (check.key.startsWith("validate") || check.key.startsWith("sweep")) return checks(after).filter((other) => other.key === check.key).length;
    return checks(after, { scale: check.scale!, view: views[0]! }, true).find((other) => other.key === check.key)!.value;
  };
  const higher = check.key.endsWith("iou");
  const base = score();
  for (const view of views) {
    const list = after.representations[check.scale!].geometry[view] as any[];
    const old = before.representations[check.scale!].geometry[view] as any[];
    const changed = list.map((shape, i) => ({ i, was: old.find((other) => other.id === shape.id && other.bone === shape.bone) })).filter((entry) => entry.was !== undefined && JSON.stringify(entry.was) !== JSON.stringify(list[entry.i]));
    for (const name of new Set(changed.map((entry) => family(list[entry.i].id)))) {
      const group = changed.filter((entry) => family(list[entry.i].id) === name);
      const kept = group.map((entry) => list[entry.i]);
      for (const entry of group) list[entry.i] = entry.was;
      const gain = higher ? score() - base : base - score();
      group.forEach((entry, k) => (list[entry.i] = kept[k]));
      if (gain > 1e-6 && (best === undefined || gain > best.gain)) best = { view, name, gain };
    }
  }
  if (best === undefined) {
    dead.add(check.key);
    console.log(`  ${check.key}: no changed family helps, left open`);
    continue;
  }
  let tightened = 0;
  let removed = 0;
  const next = ops.filter((op) => {
    if (op.scale !== check.scale || (op.view ?? (op.to ?? [])[0]) !== best!.view || family(op.id) !== best!.name) return true;
    if (op.op === "solid" && op.strict !== true) {
      Object.assign(op, { strict: true, limit: 1 });
      tightened++;
      return true;
    }
    removed++;
    return false;
  });
  writeFileSync(opsFile, JSON.stringify(next, null, 1));
  console.log(`  ${check.key}: family "${best.name}" in the ${best.view} helps by ${best.gain.toFixed(4)}: ${tightened} ops made strict, ${removed} removed`);
  if (tightened + removed === 0) dead.add(check.key);
}
