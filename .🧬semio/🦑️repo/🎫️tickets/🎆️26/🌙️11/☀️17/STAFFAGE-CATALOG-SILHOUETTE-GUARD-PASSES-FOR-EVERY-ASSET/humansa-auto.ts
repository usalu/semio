/** 🤖 Per mismatch cluster, tries to redraw nearby drawn shapes from their volume solid, to patch them locally against the volume (clip or grow) one at a time, all together, or to clip strokes; keeps the best change that improves the view. Writes the accepted ops.
 *  bun humansa-auto.ts <asset> <ops.json> <scale,scale|all> [view,view|-] [maxRounds] */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { VIEWPOINTS, applyOps, areaClustersOf, checkOf, clipStroke, clustersOf, featureOf, loadAsset, nearOf, patchShape, plainOf, redrawShape, applyShape, shapesOf, type Viewpoint } from "./humansa-lib.ts";
const [name, outFile, scalesArg, viewsArg, roundsArg] = process.argv.slice(2) as [string, string, string, string | undefined, string | undefined];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const ops: any[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
const scales = scalesArg === "all" ? [500, 200, 100, 50] : scalesArg.split(",").map(Number);
const views = (viewsArg === undefined || viewsArg === "-" ? [...VIEWPOINTS] : viewsArg.split(",")) as Viewpoint[];
const issueKeys = (): Set<string> => new Set(validateAsset(structuredClone(asset) as any).map((i: any) => `${i.code}|${i.message.en}`));
const baseline = issueKeys();
const score = (r: { iou: number; excess: number }): number => r.excess + 10 * Math.max(0, 0.93 - r.iou);
const twinOf = (id: string): string | undefined => (/-[lr]($|-)/.test(id) ? id.replace(/-([lr])($|-)/, (_m, a, b) => `-${a === "l" ? "r" : "l"}${b}`) : undefined);
type Trial = { label: string; shapes: any[] };
for (const scale of scales) {
  const feature = featureOf(scale);
  const limit = 2 * feature;
  for (const view of views) {
    let cur = clustersOf(asset, scale, view);
    const dead = new Set<string>();
    for (let round = 0; round < Number(roundsArg ?? 20) && (cur.clusters.length > 0 || cur.iou < 0.925); round++) {
      let accepted = false;
      const list = cur.clusters.length > 0 ? cur.clusters : areaClustersOf(asset, scale, view);
      for (const c of list) {
        const key = `${c.side}${Math.round(c.x / 0.04)},${Math.round(c.y / 0.04)}`;
        if (dead.has(key)) continue;
        const near = nearOf(asset, scale, view, { x: c.x, y: c.y }, 0.045);
        const filledNear = near.shapes.filter((s) => { const sh = plainOf(asset, scale, view, s.id); return sh !== undefined && sh.filled !== false && sh.followTip !== true && ["body", "detail"].includes(sh.role); });
        const trials: Trial[] = [];
        const some = (label: string, shapes: (any | undefined)[]): void => { const ok = shapes.filter((s) => s !== undefined); if (ok.length > 0 && ok.length === shapes.length) trials.push({ label, shapes: ok }); };
        for (const s of near.shapes) for (const er of [0, 0.5]) { const sh = plainOf(asset, scale, view, s.id); if (sh !== undefined && sh.filled === false) some(`stroke ${s.id} e${er}`, [clipStroke(asset, scale, view, s.id, er)]); }
        for (const cand of filledNear) {
          const ids = near.solids.filter((v) => v.id === cand.id || v.id.startsWith(cand.id + "-")).map((v) => v.id);
          const tw = twinOf(cand.id);
          const twPlain = tw !== undefined && plainOf(asset, scale, view, tw) !== undefined;
          const twIds = ids.map((i) => twinOf(i) ?? i);
          if (ids.length > 0) for (const tol of [0.12, 0.04]) some(`redraw ${cand.id} <- ${ids.join("+")} t${tol}`, [redrawShape(asset, scale, view, cand.id, ids, "contour", tol), ...(twPlain ? [redrawShape(asset, scale, view, tw!, twIds, "contour", tol)] : [])]);
          for (const dr of [c.d + 0.03, 2 * c.d + 0.05]) for (const side of ["clip", "grow"] as const) for (const mg of [0.4, 0]) for (const it of [false, true]) some(`${side}${it ? "-iter" : ""} ${cand.id} r${dr.toFixed(3)} m${mg}`, [patchShape(asset, scale, view, cand.id, c, dr, mg, 0.12, side, it), ...(twPlain ? [patchShape(asset, scale, view, tw!, c, dr, mg, 0.12, side, it)] : [])]);
        }
        if (filledNear.length > 1) for (const dr of [c.d + 0.03, 2 * c.d + 0.05]) for (const side of ["clip", "grow"] as const) for (const it of [false, true]) some(`${side}-all${it ? "-iter" : ""} r${dr.toFixed(3)}`, filledNear.map((s) => patchShape(asset, scale, view, s.id, c, dr, 0.4, 0.12, side, it)));
        let best: { label: string; shapes: any[]; next: ReturnType<typeof clustersOf> } | undefined;
        for (const t of trials) {
          const before = t.shapes.map((s) => shapesOf(asset, scale, view).find((x) => x.id === s.id));
          for (const s of t.shapes) applyShape(asset, scale, view, s);
          const next = clustersOf(asset, scale, view);
          before.forEach((b) => applyShape(asset, scale, view, b));
          if (score(next) < score(cur) - 1e-4 && (best === undefined || score(next) < score(best.next))) {
            for (const s of t.shapes) applyShape(asset, scale, view, s);
            const fresh = [...issueKeys()].filter((k) => !baseline.has(k));
            before.forEach((b) => applyShape(asset, scale, view, b));
            if (fresh.length === 0) best = { ...t, next };
            else if (process.env.DBG) console.log(`  [DEBUG] rejected ${t.label}: ${fresh[0]!.slice(0, 120)}`);
          }
        }
        if (best === undefined) { dead.add(key); continue; }
        for (const s of best.shapes) { applyShape(asset, scale, view, s); ops.push({ op: "shape", scale: String(scale), view, id: s.id, shape: s }); }
        console.log(`1:${scale} ${view} ${best.label} at (${c.x},${c.y}) ${c.side}: h ${(cur.h * 1000).toFixed(1)} -> ${(best.next.h * 1000).toFixed(1)} excess ${cur.excess.toFixed(2)} -> ${best.next.excess.toFixed(2)} iou ${cur.iou.toFixed(3)} -> ${best.next.iou.toFixed(3)}`);
        cur = best.next;
        accepted = true;
        break;
      }
      if (!accepted) break;
    }
    console.log(`== 1:${scale} ${view} iou ${cur.iou.toFixed(3)} h ${(cur.h * 1000).toFixed(1)}mm limit ${(limit * 1000).toFixed(0)} remaining ${cur.clusters.length}`);
  }
}
writeFileSync(outFile, JSON.stringify(ops, null, 1));
for (const scale of scales) console.log(scale, JSON.stringify(checkOf(asset, scale).fails));
