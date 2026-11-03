import { load, save, rep } from "./lib.ts";
{ const d = load("delivery-van");
  const src = rep(d, 50).geometry.plan.find((s: any) => s.id === "exhaust-tip");
  for (const k of [100, 200]) { const r = rep(d, k); if (r.pens.detail === null || r.geometry.plan.some((s: any) => s.id === "exhaust-tip")) continue; const c = structuredClone(src); r.geometry.plan.push(c); console.log("van plan exhaust-tip added at", k); }
  const a = d.asset.anchors.find((x: any) => x.id === "tailpipe"); a.z = -0.31; save("delivery-van", d); }
{ const d = load("motor-scooter"); const a = d.asset.anchors.find((x: any) => x.id === "tailpipe"); a.z = -0.15; save("motor-scooter", d); }
