/** 🪢️ Takes back the filled shapes this repair added on bones where `validateAsset` then reports `lateral-ambiguous` (a flat shape on a bone that a pose or clip turns out of its plane): the outline of such a shape is joined with the filled shape of the same material on the nearest ancestor bone that draws one in that view (else the largest of that material), and the added shape is removed. Writes the ops to `repair/<slug>.json`; the catalog file is written by `apply.ts`.
 *  bun rebind.ts <slug>… */
import { Mask } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { areaOf, grown, load, opsFile, placed, simplify, trace, viewOf, type P } from "./lib.ts";
import { applyOp, mergeOps, writeOps, type Op } from "./ops.ts";

for (const slug of process.argv.slice(2)) {
  const { asset, ops } = await load(slug);
  const added = ops.filter((o) => o.op === "shape-add" && o.shape.filled !== false);
  const named = new Set<string>((validateAsset(structuredClone(asset) as never) as any[]).filter((i) => i.code === "lateral-ambiguous").flatMap((i) => [...(i.data?.shapes ?? []), ...[...String(i.message.en).matchAll(/\(([^)]+)\) on bone/gu)].flatMap((m) => m[1]!.split(", "))]));
  const parent = new Map<string, string | null>(asset.rig.bones.map((b: any) => [b.id, b.parent]));
  const made: Op[] = [];
  for (const o of added) {
    if (!named.has(o.shape.id)) continue;
    const scale = Number(o.scale), view = o.view!;
    const list = asset.representations[o.scale!].geometry[view] as any[];
    if (!list.some((s) => s.id === o.shape.id)) continue;
    const v = viewOf(asset, scale, view);
    const mine = v.entries.find((e) => e.shape.id === o.shape.id)!;
    const hosts = v.entries.filter((e) => e.fills && e.shape.id !== o.shape.id && e.shape.role === "body" && e.shape.filled !== false && e.shape.solid === undefined && e.shape.material === o.shape.material && !named.has(e.shape.id));
    const size = (e: any): number => Math.abs(e.rings.reduce((n: number, r: P[]) => n + areaOf(r), 0));
    let host: (typeof hosts)[number] | undefined;
    for (let b = parent.get(o.shape.bone); b != null && host === undefined; b = parent.get(b)) host = hosts.filter((e) => e.shape.bone === b).sort((p, q) => size(q) - size(p))[0];
    host ??= hosts.sort((p, q) => size(q) - size(p))[0];
    if (host === undefined) { console.log(`[DEBUG] ${slug} 1:${scale} ${view} ${o.shape.id}: no shape to join it with, left`); continue; }
    const box = v.snap(grown({ minX: Math.min(mine.box.minX, host.box.minX), minY: Math.min(mine.box.minY, host.box.minY), maxX: Math.max(mine.box.maxX, host.box.maxX), maxY: Math.max(mine.box.maxY, host.box.maxY) }, 3 * v.cell));
    const m = new Mask(box, v.cell), extra = new Mask(box, v.cell);
    m.fill(host.rings);
    extra.fill(mine.rings);
    for (let k = 0; k < m.cells.length; k++) if (extra.cells[k]) m.cells[k] = 1;
    const rings = trace(m).filter((r) => { const a = areaOf(r); return a > 0 ? a >= 6 * v.cell * v.cell : -a >= (1.5 * v.feature) ** 2 || -a >= 40 * v.cell * v.cell; }).map((r) => simplify(r, 0.05 * v.feature)).filter((r) => r.length >= 3);
    const fit = placed(asset, scale, view, list.find((s) => s.id === host!.shape.id), rings);
    if (fit === undefined) { console.log(`[DEBUG] ${slug} 1:${scale} ${view} ${o.shape.id}: ${host.shape.id} cannot be placed, left`); continue; }
    for (const op of [{ op: "shape", scale: o.scale, view, id: host.shape.id, shape: fit.shape }, { op: "shape-remove", scale: o.scale, view, id: o.shape.id }] as Op[]) { applyOp(asset, op); made.push(op); }
    console.log(`[DEBUG] ${slug} 1:${scale} ${view} ${o.shape.id} (bone ${o.shape.bone}): joined with ${host.shape.id} (bone ${host.shape.bone}) and removed`);
  }
  const left = (validateAsset(structuredClone(asset) as never) as any[]).length;
  if (made.length > 0) await writeOps(opsFile(slug), [...ops.filter((x) => !(x.op === "shape-add" && made.some((r) => r.op === "shape-remove" && r.scale === x.scale && r.view === x.view && r.id === x.shape.id))), ...mergeOps([], made)]);
  console.log(`[DEBUG] ${slug}: ${made.length / 2} added shapes joined with another, validateAsset now reports ${left} issues`);
}
