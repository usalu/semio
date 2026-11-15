import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const dir = process.env.VALIDATOR_MODULE ?? "🧊️volumes-next-b";
const S = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/${dir}/🌓️silhouette/🟦️.ts`);
const [slug, scale, view] = [process.argv[2]!, Number(process.argv[3]), process.argv[4] ?? "elevation"];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry) as any;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = S.worldSolids(asset, state.bones, scale, "body");
const drawn = S.drawnFor(asset, state, view, scale);
for (const w of solids.filter((w: any) => w.slender)) {
  const withD = S.strokesOf(view, w, drawn), without = S.strokesOf(view, w, undefined);
  const ext = (rs: any[]) => { const xs = rs.flat().map((p: any) => p.x), ys = rs.flat().map((p: any) => p.y); return [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)].map((x) => +x.toFixed(3)).join(","); };
  console.log(w.id, w.bone, "pen", w.penM, "quads", withD.length, ext(withD), "| old", ext(without), "| drawn bone", drawn.filter((d: any) => d.bone === w.bone).map((d: any) => `${d.closed ? "c" : "o"}${d.filled ? "f" : "u"}${(d.penM * 1000).toFixed(0)}`).join(" "));
}
for (const d of drawn.filter((d: any) => /brace/.test(d.bone))) console.log("2D", d.bone, d.closed, d.filled, d.penM, JSON.stringify(d.rings).slice(0, 300));
const w = solids.find((w: any) => w.id === "brace-l");
console.log(JSON.stringify(S.strokesOf(view, w, drawn)).slice(0, 700));
