export default (a) => {
  const g = a.representations["100"].geometry.plan;
  const x0 = Number(process.env.HX0 ?? 0.43575), x1 = Number(process.env.HX1 ?? 0.36075), r = Number(process.env.HR ?? 0.032);
  for (const id of ["hand-l", "hand-r"]) Object.assign(g.find((s) => s.id === id), { x0, x1, r0: r, r1: r });
  const at = g.findIndex((s) => s.id === "chain");
  g.splice(at + 1, 0, { kind: "capsule", x0: -0.04, y0: -0.0395, x1: 0.2, y1: -0.0395, r0: 0.0065, r1: 0.0065, id: "chain-web", bone: "frame", role: "detail", z: 1.69, filled: true, tone: 0.5, region: "bike-metal" });
  g.splice(at + 3, 0, { kind: "rect", x: -0.025, y: -0.035, w: 0.05, h: 0.07, id: "hub-rear", bone: "frame", role: "body", z: 1.5, filled: true, tone: 0.5, region: "bike-metal" });
  g.splice(at + 2, 0, { kind: "rect", x: 1.025, y: -0.035, w: 0.05, h: 0.07, id: "hub-front", bone: "frame", role: "body", z: 1.5, filled: true, tone: 0.5, region: "bike-metal" });
  const i = g.findIndex((s) => s.id === "fender-front"), o = g[i];
  g[i] = { kind: "rect", x: 0.71, y: -0.033, w: 0.673, h: 0.066, id: o.id, bone: o.bone, role: o.role, z: o.z, filled: o.filled, tone: o.tone, region: o.region };
};
