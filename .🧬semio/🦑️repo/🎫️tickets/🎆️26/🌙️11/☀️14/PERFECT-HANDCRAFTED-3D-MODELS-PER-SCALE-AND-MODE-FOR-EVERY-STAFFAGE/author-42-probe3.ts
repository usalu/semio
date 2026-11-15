/** 🔬️ Probe (author 42): boundary cells of the drawing that lie far from the 3D boundary (product measureView with edge): bun author-42-probe3.ts <asset> <scale> <view> [minMm] */
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const sil: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
const ev: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts");
const [spec, sc, view, minMm] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
const state = ev.evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const solids = sil.worldSolids(m, state.bones, Number(sc)).filter((w: any) => w.texture !== true);
const r = sil.measureView(m, state, solids, view, Number(sc), 420, undefined, undefined, true);
const e = r.edge;
console.log("p95", r.p95M, "haus", r.hausdorffM, "grid", e.width, e.height, "cell", e.cell);
const section = m.representations[sc!].sections?.[view!];
const ref = sil.referenceOf(view, solids, section);
const box = { minX: e.minX, minY: e.minY, maxX: e.minX + e.width * e.cell, maxY: e.minY + e.height * e.cell };
const mk = (gs: any[]) => { const k = new sil.Mask(box, e.cell); for (const g of gs) k.fill(g); return k; };
const a = mk(ref.cut), b = mk(ref.visible ?? []);
const u = new Uint8Array(e.width * e.height); for (let i = 0; i < u.length; i++) u[i] = a.cells[i] | b.cells[i];
const bnd = sil.boundaryOf(u, e.width, e.height);
const dist = sil.distanceField(bnd, e.width, e.height);
const minCells = (Number(minMm ?? 20) / 1000) / e.cell;
const far: number[][] = [];
for (let j = 0; j < e.height; j++) for (let i = 0; i < e.width; i++) { const k = j * e.width + i; if (e.mask[k] && !bnd[k] && Math.sqrt(dist[k]) > minCells) far.push([i, j]); }
console.log("edge cells far from the 3D boundary:", far.length);
const cl: Record<string, number[]> = {};
for (const [i, j] of far) { const key = `${Math.floor(i / 25)},${Math.floor(j / 25)}`; (cl[key] ??= []).push(i, j); }
for (const [k, v] of Object.entries(cl)) { const xs = v.filter((_, q) => q % 2 === 0), ys = v.filter((_, q) => q % 2 === 1); console.log(k, v.length / 2, "x", (e.minX + Math.min(...xs) * e.cell).toFixed(3), (e.minX + Math.max(...xs) * e.cell).toFixed(3), "y", (e.minY + Math.min(...ys) * e.cell).toFixed(3), (e.minY + Math.max(...ys) * e.cell).toFixed(3)); }
