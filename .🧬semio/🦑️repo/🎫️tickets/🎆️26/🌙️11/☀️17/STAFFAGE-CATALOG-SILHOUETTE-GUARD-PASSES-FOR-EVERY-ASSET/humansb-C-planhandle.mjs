export default (a) => {
  for (const sc of (process.env.HANDLE_SCALES ?? "50,100,200,500").split(",")) {
    const g = a.representations[sc].geometry.plan;
    if (g.some((s) => s.id === "handle")) continue;
    const r = g.find((s) => s.id === "rail-r");
    const r0 = sc === "500" ? 0.05 : 0.02, hx = sc === "500" ? 0.2665 : 0.1805;
    g.splice(g.indexOf(r) + 1, 0, { kind: "capsule", x0: hx, y0: -0.21, x1: hx, y1: 0.21, r0, r1: r0, id: "handle", bone: "hand-truck", role: "body", z: 6.52, filled: true, tone: 0.5, region: "hand-truck" });
  }
};
