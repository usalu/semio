/** 🩹️ Closes in the drawing the hairlines the silhouette guard closes in the body. The guard seals a raster cell that no triangle covers when it lies in a gap narrower than a quarter of a cell between touching solids (two leaves that meet edge to edge); the drawing, the exact union of the same triangles, keeps that gap, and one open cell can turn a pocket of the foliage into a bay. Per scale and view, every cell the guard seals and the drawing leaves open gets a square of half a cell (0.02 mm on paper at 1:50), joined to the outline of the nearest flat filled shape as further rings of its path (the part of the square the shape does not cover yet). Idempotent: a sealed drawing has no such cell left.
 *  bun seal.ts <id> [--in file] [--out file] [--scales 50,100]
 * @see ./redraw.ts — the redraw this follows */
import { parseArgs } from "node:util";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { edgeDistance, scene, tileMasks, tiles } from "./lib.ts";
import { fileOf } from "./plants.ts";
import { spans } from "./preserve.ts";

const clipping: any = (await import(Bun.resolveSync("polygon-clipping", "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/"))).default;

const { values, positionals } = parseArgs({ allowPositionals: true, options: { in: { type: "string" }, out: { type: "string" }, scales: { type: "string" } } });
const id = positionals[0]!;
const source = values.in ?? fileOf(id);
const text = await Bun.file(source).text();
const doc = JSON.parse(text);
if (doc.asset.id !== id) throw new Error(`${source} holds ${doc.asset.id}, not ${id}`);
const next = structuredClone(doc);
const asset = next.asset;
const r8 = (n: number): number => Math.round(n * 1e8) / 1e8;
let total = 0;
for (const scaleArg of (values.scales ?? "50,100,200,500").split(",")) {
  const rep = asset.representations[scaleArg];
  if (rep?.status !== "authored" || asset.volumes[scaleArg]?.status !== "authored") continue;
  for (const view of ["elevation", "front", "plan"]) {
    const shapes = rep.geometry[view] as any[];
    const sc = scene(asset, Number(scaleArg), view);
    const owners = (sc.evaluated as any[]).filter((s) => s.closed && s.shape.filled !== false && s.shape.solid === undefined);
    const cells = new Map<string, { x: number; y: number }[]>();
    for (const t of tiles(sc)) {
      const { drawn, vol, width } = tileMasks(sc, t);
      const covered = new Mask(t.box, sc.cell);
      for (const g of t.reference) covered.fill(g as never);
      const [i0, i1, j0, j1] = [Math.round((t.core.minX - t.box.minX) / sc.cell), Math.round((t.core.maxX - t.box.minX) / sc.cell), Math.round((t.core.minY - t.box.minY) / sc.cell), Math.round((t.core.maxY - t.box.minY) / sc.cell)];
      for (let j = j0; j < j1; j++) for (let i = i0; i < i1; i++) {
        const k = j * width + i;
        if (!vol[k] || drawn[k] || covered.cells[k]) continue;
        const c = { x: t.box.minX + (i + 0.5) * sc.cell, y: t.box.minY + (j + 0.5) * sc.cell };
        let [best, owner] = [Infinity, undefined as any];
        for (const s of owners) { const d = edgeDistance(c, s.rings, true); if (d < best) [best, owner] = [d, s]; }
        if (owner === undefined || best > sc.cell) continue;
        (cells.get(owner.shape.id) ?? cells.set(owner.shape.id, []).get(owner.shape.id)!).push(c);
      }
    }
    if (cells.size === 0) { console.log(`${id} 1:${scaleArg} ${view}: nothing to seal`); continue; }
    const bones = [...new Set([...cells.keys()].map((sid) => shapes.find((s) => s.id === sid).bone as string))];
    rep.geometry[view] = bones.map((bone) => ({ id: `flat-${bone}`, bone, role: "body", z: 0, filled: true, tone: 0.5, kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true }));
    const probed: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } } as never);
    rep.geometry[view] = shapes;
    const frames = new Map<string, { x: number; y: number }[]>(probed.shapes[view].map((s: any) => [s.shape.bone, s.rings[0]]));
    const h = sc.cell / 4;
    let count = 0;
    for (const [sid, list] of cells) {
      const at = shapes.findIndex((s) => s.id === sid);
      const owner = shapes[at];
      const [o, x, y] = frames.get(owner.bone)!;
      const [a, b, c, d] = [x!.x - o!.x, y!.x - o!.x, x!.y - o!.y, y!.y - o!.y];
      const det = a * d - b * c;
      const local = (wx: number, wy: number): string => `${r8((d * (wx - o!.x) - b * (wy - o!.y)) / det)} ${r8((-c * (wx - o!.x) + a * (wy - o!.y)) / det)}`;
      const rings = (sc.evaluated as any[]).find((s) => s.shape.id === sid).rings as { x: number; y: number }[][];
      const boxes = rings.map((r) => [Math.min(...r.map((q) => q.x)), Math.max(...r.map((q) => q.x)), Math.min(...r.map((q) => q.y)), Math.max(...r.map((q) => q.y))] as const);
      const added: string[] = [];
      for (const q of list) {
        const square = [[[q.x - h, q.y - h], [q.x + h, q.y - h], [q.x + h, q.y + h], [q.x - h, q.y + h], [q.x - h, q.y - h]]];
        const near = rings.filter((_, k) => boxes[k]![0] <= q.x + h && boxes[k]![1] >= q.x - h && boxes[k]![2] <= q.y + h && boxes[k]![3] >= q.y - h).map((r) => [[...r.map((v) => [v.x, v.y]), [r[0]!.x, r[0]!.y]]]);
        let pieces: number[][][][];
        try { pieces = near.length === 0 ? [square] : clipping.difference(square, near.length === 1 ? near[0] : clipping.xor(...near)); } catch { pieces = [square]; }
        for (const poly of pieces) for (const ring of poly) added.push(`M ${ring.slice(0, -1).map(([wx, wy]) => local(wx!, wy!)).join(" L ")} Z`);
      }
      const base = owner.kind === "path" ? (owner.d as string) : `M ${(owner.points as number[][]).map((v) => `${v[0]} ${v[1]}`).join(" L ")} Z`;
      const { points: _points, closed: _closed, ...rest } = owner;
      shapes[at] = { ...rest, kind: "path", d: `${base} ${added.join(" ")}` };
      count += list.length;
    }
    total += count;
    console.log(`${id} 1:${scaleArg} ${view}: ${count} sealed cells of ${(sc.cell * 1000).toFixed(2)} mm joined to ${cells.size} shapes`);
  }
}
const inline = (v: unknown): string => (Array.isArray(v) ? `[${v.map(inline).join(", ")}]` : v !== null && typeof v === "object" ? `{${Object.entries(v).filter(([, x]) => x !== undefined).map(([k, x]) => `${JSON.stringify(k)}: ${inline(x)}`).join(", ")}}` : JSON.stringify(v));
const member = (node: any, key: string): any => node.members.find((m: any) => m.key === key).value;
const root = spans(text);
const edits: { start: number; end: number; text: string }[] = [];
for (const [scaleArg, rep] of Object.entries(next.asset.representations) as [string, any][]) {
  if (rep.status !== "authored") continue;
  for (const view of Object.keys(rep.geometry)) {
    if (JSON.stringify(rep.geometry[view]) === JSON.stringify(doc.asset.representations[scaleArg].geometry[view])) continue;
    const node = member(member(member(member(member(root, "asset"), "representations"), scaleArg), "geometry"), view);
    const line = text.lastIndexOf("\n", node.start) + 1;
    const indent = " ".repeat(text.slice(line).search(/\S/u));
    edits.push({ start: node.start, end: node.end, text: `[\n${(rep.geometry[view] as unknown[]).map((s) => `${indent}  ${inline(s)}`).join(",\n")}\n${indent}]` });
  }
}
let written = text;
for (const e of edits.sort((p, q) => q.start - p.start)) written = written.slice(0, e.start) + e.text + written.slice(e.end);
if (JSON.stringify(JSON.parse(written)) !== JSON.stringify(next)) throw new Error("the rewritten text does not hold the new document");
if (written === text) console.log(`${id}: the file already has these ${total} sealed cells`);
else { await Bun.write(values.out ?? source, written); console.log(`${id}: ${total} sealed cells -> ${values.out ?? source}`); }
