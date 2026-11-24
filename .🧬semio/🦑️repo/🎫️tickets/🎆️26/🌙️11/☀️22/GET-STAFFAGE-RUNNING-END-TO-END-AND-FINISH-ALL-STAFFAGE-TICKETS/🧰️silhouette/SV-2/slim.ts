/** 🪶️ Thins the outlines this repair traced: every filled flat shape that an op of `repair/<slug>.json` wrote and that carries more than `--min` points (default 120) is simplified in metres of the view with `--eps` smallest features (default 0.5, two cells of the guard raster, so the cell steps of the trace collapse into straight runs) and written back at the same place. A plant then draws with about the number of points it had before the repair. Writes `shape` ops; the catalog file is written by `apply.ts`, and `retrace.ts` afterwards refines the few places where the coarser outline moves a boundary beyond the limit.
 *  bun slim.ts <slug>… [--eps 0.5] [--min 120] */
import { parseArgs } from "node:util";
import { load, opsFile, placed, simplify, viewOf } from "./lib.ts";
import { applyOp, mergeOps, writeOps, type Op } from "./ops.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { eps: { type: "string" }, min: { type: "string" } } });
const eps = Number(values.eps ?? 0.5), min = Number(values.min ?? 120);
for (const slug of positionals) {
  const { asset, ops } = await load(slug);
  const made: Op[] = [];
  let [was, now] = [0, 0];
  for (const sc of ["500", "200", "100", "50"]) if (asset.representations[sc]?.status === "authored") for (const view of ["elevation", "front", "plan"]) {
    const mine = new Set(ops.filter((o) => (o.op === "shape" || o.op === "shape-add") && o.scale === sc && o.view === view && o.shape.filled !== false).map((o) => o.shape.id as string));
    const list = (asset.representations[sc].geometry[view] as any[]).filter((s) => mine.has(s.id) && s.kind === "path" && s.filled !== false && s.solid === undefined && (s.d.match(/[ML] /gu)?.length ?? 0) > min);
    if (list.length === 0) continue;
    const v = viewOf(asset, Number(sc), view);
    for (const shape of list) {
      const entry = v.entries.find((e) => e.shape.id === shape.id);
      if (entry === undefined) continue;
      const rings = entry.rings.map((r) => simplify(r, eps * v.feature)).filter((r) => r.length >= 3);
      const [a, b] = [entry.rings.reduce((n, r) => n + r.length, 0), rings.reduce((n, r) => n + r.length, 0)];
      if (rings.length === 0 || b > 0.8 * a) continue;
      const fit = placed(asset, Number(sc), view, shape, rings);
      if (fit === undefined) continue;
      const op: Op = { op: "shape", scale: sc, view, id: shape.id, shape: fit.shape };
      applyOp(asset, op);
      made.push(op);
      was += a;
      now += b;
    }
  }
  if (made.length > 0) await writeOps(opsFile(slug), mergeOps(ops, made));
  console.log(`[DEBUG] ${slug}: ${made.length} shapes thinned, ${was} -> ${now} points`);
}
