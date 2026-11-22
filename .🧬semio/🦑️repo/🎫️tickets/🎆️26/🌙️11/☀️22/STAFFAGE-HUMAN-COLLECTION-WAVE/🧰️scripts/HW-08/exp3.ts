import { loadDoc, measureAsset, featureOf } from "./lib.ts";
const a = loadDoc("hiker-with-trekking-poles").asset;
const shapes = ["elevation", "front", "plan"].flatMap((v) => a.representations["50"].geometry[v].filter((s: any) => /^hand-[lr]$/.test(s.bone) && s.solid?.kind === "extruded"));
const orig = shapes.map((s: any) => structuredClone(s.solid));
for (const [w0, wt] of [[0, 0], [0.012, 0], [0.012, -0.004], [0.018, -0.006], [0.024, -0.008]]) {
  shapes.forEach((s: any, i: number) => { const o = orig[i]; const c = (o.z0 + o.z1) / 2, h = (o.z1 - o.z0) / 2; s.solid = { ...o, z0: c - h + w0, z1: c + h - w0, zTip: [c - h - wt, c + h + wt] }; if (w0 === 0 && wt === 0) s.solid = o; });
  const m: any = measureAsset(a, 50, undefined, undefined, featureOf(50 as never));
  console.log(w0, wt, ["elevation", "front", "plan"].map((v) => `${m.views[v].iou.toFixed(3)}/${(m.views[v].hausdorffM * 1000).toFixed(0)}`).join(" "));
}
