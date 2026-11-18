export default (a) => {
  const sc = process.env.SC ?? "100", view = process.env.VW ?? "front", id = process.env.SID ?? "thigh-l", map = process.env.MAP ?? "none";
  const s = a.representations[sc].geometry[view].find((x) => x.id === id);
  const f = { none: (p) => p, ux: (p) => [-p[1], p[0]], xu: (p) => [p[1], p[0]], mx: (p) => [p[1], -p[0]], nxy: (p) => [-p[0], -p[1]] }[map];
  if (map !== "none") s.points = s.points.map(f);
  if (process.env.SW) s.solid = { kind: "extruded", sweep: process.env.SW, z0: -Number(process.env.D ?? 0.07), z1: Number(process.env.D ?? 0.07) };
};
