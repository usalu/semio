export default (a) => {
  const el = a.representations["50"].geometry.elevation, g = a.representations["50"].geometry.front;
  const E = (id) => el.find((s) => s.id === id);
  const zr = Number(process.env.HZ ?? 0.037);
  for (const [side, sg, zc, tz0, tz1] of [["l", 1, 0.21, 0.18, 0.208], ["r", -1, -0.21, -0.208, -0.18]]) {
    const i = g.findIndex((s) => s.id === "hand-" + side);
    const glove = E("glove-" + side), thumb = E("glove-thumb-" + side);
    g[i] = { kind: "path", d: glove.d, id: "hand-" + side, bone: "hand-" + side, role: "body", z: 4.6, filled: true, tone: 0.5, region: "gloves", solid: { kind: "extruded", sweep: "w", z0: +(zc - zr).toFixed(4), z1: +(zc + zr).toFixed(4) } };
    g.splice(i + 1, 0, { kind: "path", d: thumb.d, id: "glove-thumb-" + side, bone: "hand-" + side, role: "body", z: 4.62, filled: true, tone: 0.5, region: "gloves", solid: { kind: "extruded", sweep: "w", z0: tz0, z1: tz1 } });
  }
};
