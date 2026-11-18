export default (a) => {
  const g = a.representations["50"].geometry.plan;
  const i = g.findIndex((s) => s.id === "axle-front");
  g.splice(i + 1, 0, { kind: "rect", x: 1.025, y: -0.035, w: 0.05, h: 0.07, id: "hub-front", bone: "frame", role: "body", z: 1.5, filled: true, tone: 0.75, region: "bike-metal", material: "steel" });
};
