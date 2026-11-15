import { entryOf, poseState, restState, frames } from "./volumes-lib.ts";
const [spec, scaleArg, view, re, mode] = process.argv.slice(2);
const { asset } = entryOf(spec!);
const scale = Number(scaleArg);
const rx = new RegExp(re ?? ".");
if (view === "frames") {
  const st = poseState(asset, scale);
  for (const [id, f] of Object.entries(st.frames)) if (rx.test(id)) console.log(id, JSON.stringify(f, (_k, v) => (typeof v === "number" ? Math.round(v * 1e4) / 1e4 : v)));
} else {
  const st = mode === "rest" ? restState(asset, scale) : poseState(asset, scale);
  for (const e of (st as any)[view!]) {
    if (!rx.test(e.shape.id) && !rx.test(e.shape.bone)) continue;
    console.log(e.shape.id, e.shape.bone, e.shape.role, e.shape.region, "closed", e.closed, "filled", e.shape.filled, JSON.stringify(e.rings.map((r: any[]) => r.map((p) => [Math.round(p.x * 1e4) / 1e4, Math.round(p.y * 1e4) / 1e4]))));
  }
}
