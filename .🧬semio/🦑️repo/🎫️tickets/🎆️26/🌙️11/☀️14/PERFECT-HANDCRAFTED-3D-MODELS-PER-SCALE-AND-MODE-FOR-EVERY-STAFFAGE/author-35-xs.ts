import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, referenceOf, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, cS, fS, view] = process.argv.slice(2) as string[];
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!) as any;
const W = (s: number) => { const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: s } }); return worldSolids(m, st.bones as never, s, "body").filter((w: any) => (process.env.ALL ? true : w.slender !== true) && w.texture !== true); };
const [c, f] = [W(Number(cS)), W(Number(fS))];
const ga = referenceOf(view as any, c, undefined).cut, gb = referenceOf(view as any, f, undefined).cut;
let x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9; for (const g of [...ga, ...gb]) for (const r of g) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
const cell = Number(process.env.CELL ?? 0.02); const box = { minX: x0 - cell, minY: y0 - cell, maxX: x1 + cell, maxY: y1 + cell };
const A = new Mask(box, cell), B = new Mask(box, cell); for (const g of ga) A.fill(g); for (const g of gb) B.fill(g);
let both = 0, either = 0, onlyA = 0, onlyB = 0; const grid = new Map<string, [number, number]>();
for (let j = 0; j < A.height; j++) for (let i = 0; i < A.width; i++) { const k = j * A.width + i; const a = A.cells[k]!, b = B.cells[k]!; both += a & b; either += a | b; if (a && !b) onlyA++; if (b && !a) onlyB++; if (a !== b) { const key = `${(Math.floor((x0 + i * cell) * 2) / 2).toFixed(1)},${(Math.floor((y0 + j * cell) * 2) / 2).toFixed(1)}`; const e = grid.get(key) ?? [0, 0]; if (a) e[0]++; else e[1]++; grid.set(key, e); } }
console.log(`${view}: iou ${(both / either).toFixed(4)} cells both ${both} coarse-only ${onlyA} fine-only ${onlyB} (cell ${cell} m, area ${(both * cell * cell).toFixed(1)} m2)`);
console.log([...grid.entries()].sort((p, q) => (q[1][0] + q[1][1]) - (p[1][0] + p[1][1])).slice(0, 14).map(([k, v]) => `${k}: c${v[0]}/f${v[1]}`).join("  "));
import { silhouetteAgreement } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const feat = 0.15e-3 * Number(cS);
for (const tol of [0, 1, 2]) { const a = silhouetteAgreement(c, f, view as any, feat, tol * feat, {}); console.log("tol", tol, "iou", a.iou.toFixed(4), "raw", a.rawIou.toFixed(4), "p95", a.p95M, "haus", a.hausdorffM, "cell", a.cellM); }
