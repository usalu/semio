import { loadAsset, resolveAsset, restState, bboxOf } from "./volumes-base.ts";
const a = loadAsset(resolveAsset(process.argv[2]!).path);
const scales = process.argv[3] ? [Number(process.argv[3])] : [50, 100, 200, 500];
const views = process.argv[4] ? [process.argv[4]] : ["elevation", "front", "plan"];
for (const s of scales) {
  if (a.representations[String(s)].status === "unavailable") continue;
  const st: any = restState(a, s);
  for (const v of views) {
    const seen = new Map<string, string>();
    for (const e of st[v]) {
      const pts = e.rings.flat();
      if (!pts.length) continue;
      const b = bboxOf(pts);
      const k = `${e.shape.id}(${e.shape.bone},${e.shape.role}${e.closed ? "" : ",open"}${e.shape.region ? "," + e.shape.region : ""})`;
      seen.set(k, `[${b.minX.toFixed(2)}..${b.maxX.toFixed(2)}|${b.minY.toFixed(2)}..${b.maxY.toFixed(2)}]`);
    }
    console.log(`1:${s} ${v}: ` + [...seen].map(([k, b]) => k + b).join("  "));
  }
}
