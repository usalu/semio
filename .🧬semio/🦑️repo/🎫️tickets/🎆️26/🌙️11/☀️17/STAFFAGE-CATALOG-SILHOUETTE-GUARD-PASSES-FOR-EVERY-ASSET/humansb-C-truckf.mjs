export default (a) => {
  const g = a.representations["50"].geometry.front;
  const f = (id) => g.find((s) => s.id === id);
  Object.assign(f("plate"), { y: 0.176, h: 0.137 });
  Object.assign(f("axle-stay"), { x: -0.225, w: 0.45, y: 0.1, h: 0.03 });
  g.splice(g.indexOf(f("axle-stay")) + 1, 0, { kind: "rect", x: -0.297, y: 0.118, w: 0.594, h: 0.024, id: "axle", bone: "hand-truck", role: "detail", z: 6.56, filled: true, tone: 0.5, region: "hand-truck" });
  g.splice(g.indexOf(f("plate")) + 1, 0, ...[["l", 0.182], ["r", -0.198]].map(([n, x]) => ({ kind: "rect", x, y: 0.141, w: 0.016, h: 0.161, id: "stay-" + n, bone: "hand-truck", role: "detail", z: 6.91, filled: true, tone: 0.5, region: "hand-truck" })));
  f("brace-0").points = [[-0.21, 0.547], [0.21, 0.547]];
  f("brace-1").points = [[-0.21, 0.872], [0.21, 0.872]];
};
