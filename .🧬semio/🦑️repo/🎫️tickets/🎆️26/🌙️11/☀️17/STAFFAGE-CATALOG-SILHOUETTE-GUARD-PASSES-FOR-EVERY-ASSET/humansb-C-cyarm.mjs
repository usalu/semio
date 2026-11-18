export default (a) => {
  const el = a.representations["50"].geometry.elevation, g = a.representations["50"].geometry.front;
  const E = (id) => el.find((s) => s.id === id);
  for (const [side, sg] of [["l", 1], ["r", -1]]) {
    const mk = (id, bone, src, solid, z) => ({ kind: "path", d: E(src).d, id, bone, role: "body", z, filled: true, tone: g.find((s) => s.id === id).tone, region: g.find((s) => s.id === id).region, material: g.find((s) => s.id === id).material, solid });
    const fi = g.findIndex((s) => s.id === "forearm-" + side), hi = g.findIndex((s) => s.id === "hand-" + side);
    const sp = (a0, a1) => (sg > 0 ? [a0, a1] : [-a1, -a0]);
    const [f0, f1] = sp(0.188, 0.272), [t0, t1] = sp(0.248, 0.312), [h0, h1] = sp(0.25, 0.31);
    g[fi] = mk("forearm-" + side, "forearm-" + side, "forearm-" + side, { kind: "extruded", sweep: "w", z0: f0, z1: f1, zTip: [t0, t1] }, 4.55);
    g[hi] = mk("hand-" + side, "hand-" + side, "glove-" + side, { kind: "extruded", sweep: "w", z0: h0, z1: h1 }, 4.6);
  }
};
