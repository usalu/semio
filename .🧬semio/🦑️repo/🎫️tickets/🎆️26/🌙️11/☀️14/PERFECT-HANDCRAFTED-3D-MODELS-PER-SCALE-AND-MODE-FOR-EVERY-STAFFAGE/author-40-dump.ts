/** 📐️ World outlines of the drawn shapes and bone frames: bun author-40-dump.ts <asset> <scale> [bone-prefix] */
import { entryOf, loadSource, merged, poseState, VIEWS } from "./volumes-lib.ts";

const [spec, sc, pre] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const st = poseState(m, Number(sc));
const f = (x: number) => (Math.round(x * 1000) / 1000).toString();
for (const [id, fr] of Object.entries(st.frames as Record<string, any>)) {
  if (pre && !id.startsWith(pre)) continue;
  console.log("FRAME", id, "o", fr.o.map(f).join(","), "u", fr.u.map(f).join(","), "v", fr.v.map(f).join(","), "w", fr.w.map(f).join(","));
}
for (const v of VIEWS) {
  for (const s of st[v] as any[]) {
    if (pre && !String(s.shape.bone ?? "").startsWith(pre)) continue;
    let b = { x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity };
    for (const ring of s.rings) for (const p of ring) { b.x0 = Math.min(b.x0, p.x); b.x1 = Math.max(b.x1, p.x); b.y0 = Math.min(b.y0, p.y); b.y1 = Math.max(b.y1, p.y); }
    const pts = s.rings[0]?.length <= 14 ? JSON.stringify(s.rings[0].map((p: any) => [Number(f(p.x)), Number(f(p.y))])) : "";
    console.log(v, s.shape.id, s.shape.bone, s.shape.kind, s.shape.role, s.shape.filled === false ? "open" : "", `x ${f(b.x0)}..${f(b.x1)} y ${f(b.y0)}..${f(b.y1)}`, pts);
  }
}
