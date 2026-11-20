/** 🔭️ Round 3 English oak: the cross-scale boundary distance of the product measured twice per pair of scales and view, once with the product's own windows (2D pair without a limit, 3D pair with the limit derived from it) and once with one common wide window for both, to show where the 95th percentile saturates. Usage: bun r3-english-oak-cross.ts [windowM] [candidate] [pairs, e.g. 100,200] (candidate: the document `🗑️generated/r3-english-oak/🔣️.json` with its own 2D and volumes) */
import { compareGroups, silhouetteAgreement } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { VIEWPOINTS, drawnFor, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { readFileSync } from "node:fs";
import { entryOf } from "./volumes-lib.ts";
const wide = Number(process.argv[2] ?? 1.5);
const asset: any = process.argv[3] === "candidate" ? JSON.parse(readFileSync(`${import.meta.dir}/🗑️generated/r3-english-oak/🔣️.json`, "utf8")).asset : (entryOf("vegetation/english-oak") as any).asset;
const only = (process.argv[4] ?? "100,200,500").split(",").map(Number);
const FEATURE: Record<number, number> = { 100: 0.015, 200: 0.03, 500: 0.075 };
const state = (d: number): any => evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } } as never);
const drawn = (s: any, view: string): any[] => s.shapes[view].filter((i: any) => i.closed && i.shape.filled !== false && ["body", "detail", "glazing"].includes(i.shape.role)).map((i: any) => i.rings);
for (const [fine, coarse] of [[50, 100], [100, 200], [200, 500]] as const) {
  if (!only.includes(coarse)) continue;
  const f = FEATURE[coarse]!;
  const [sc, sf] = [state(coarse), state(fine)];
  const [wc, wf] = [worldSolids(asset, sc.bones, coarse, "body").filter((w: any) => w.texture !== true), worldSolids(asset, sf.bones, fine, "body").filter((w: any) => w.texture !== true)];
  for (const view of VIEWPOINTS) {
    const pair = compareGroups(drawn(sc, view), drawn(sf, view), f);
    const limit = Math.max(2 * f, pair.p95M + f);
    const options = { drawnFirst: drawnFor(asset, sc, view, coarse), drawnSecond: drawnFor(asset, sf, view, fine) };
    const own = silhouetteAgreement(wc, wf, view, f, f, { ...options, limitM: limit });
    const pairWide = compareGroups(drawn(sc, view), drawn(sf, view), f, 0, undefined, wide);
    const ownWide = silhouetteAgreement(wc, wf, view, f, f, { ...options, limitM: wide });
    console.log(`1:${coarse} vs 1:${fine} ${view.padEnd(9)} product: 2D p95 ${pair.p95M.toFixed(4)} limit ${limit.toFixed(4)} 3D p95 ${own.p95M.toFixed(4)} | common window ${wide} m: 2D p95 ${pairWide.p95M.toFixed(4)} 3D p95 ${ownWide.p95M.toFixed(4)}`);
  }
}
