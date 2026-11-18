export default (a) => {
  const g = a.representations["50"].geometry.plan;
  const [x0, x1] = [Number(process.env.WX0 ?? -0.04), Number(process.env.WX1 ?? 0.2)];
  const i = g.findIndex((s) => s.id === "chain");
  g.splice(i + 1, 0, { kind: "capsule", x0, y0: -0.0395, x1, y1: -0.0395, r0: 0.0065, r1: 0.0065, id: "chain-web", bone: "frame", role: "detail", z: 1.69, filled: true, tone: 0.75, region: "bike-metal", material: "steel" });
};
