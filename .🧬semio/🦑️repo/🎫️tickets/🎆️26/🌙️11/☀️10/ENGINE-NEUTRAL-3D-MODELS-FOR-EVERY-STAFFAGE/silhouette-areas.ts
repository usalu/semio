/** 🔬️ Areas (m2) of the masks of one view: `bun silhouette-areas.ts <slug> <view>`. */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, measureView, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const a = process.argv.slice(2);
const asset = loadAsset(catalogEntries().find((x) => x.slug === a[0])!.path);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: 50 } } as never) as any;
const m = measureView(asset, state, worldSolids(asset, state.bones, 50), a[1] as never, 50, 800);
const area = (gs: readonly any[]) => { const k = new Mask(m.box, m.cell); for (const g of gs) k.fill(g); return k.cells.reduce((s, v) => s + v, 0) * m.cell * m.cell; };
console.log({ filled: area(m.filled), outlines: area(m.outlines), cut: area(m.reference), visible: area(m.visible), full: area(m.full), iou: m.iou, section: m.section });
const U = new Mask(m.box, m.cell), F = new Mask(m.box, m.cell), A = new Mask(m.box, m.cell), I = new Mask(m.box, m.cell);
for (const g of m.visible) U.fill(g);
for (const g of m.full) F.fill(g);
for (const g of m.filled) A.fill(g);
for (const g of m.outlines) I.fill(g);
let f_out = 0, i_out = 0, f_tot = 0, i_tot = 0;
for (let i = 0; i < A.cells.length; i++) { if (A.cells[i]) { f_tot++; if (!U.cells[i]) f_out++; } if (I.cells[i]) { i_tot++; if (!F.cells[i]) i_out++; } }
console.log({ filledOutsideVisible: f_out * m.cell * m.cell, filledTotal: f_tot * m.cell * m.cell, inkOutsideFull: i_out * m.cell * m.cell, inkTotal: i_tot * m.cell * m.cell });
