export default (a) => {
  const b = a.volumes["100"].body, r = Number(process.env.THR ?? 0.0085), z0 = Number(process.env.THZ ?? 0.072);
  for (const [side, z] of [["l", z0], ["r", -z0]]) {
    const i = b.findIndex((s) => s.id === "glove-" + side);
    b.splice(i + 1, 0, { id: "glove-thumb-" + side, kind: "capsule", bone: "hand-" + side, p0: [0.0, Number(process.env.TY0 ?? 0.02), z], p1: [0.098, Number(process.env.TY1 ?? 0.07), z], r0: r, r1: r * 0.9, material: "leather-glove", role: "body", tone: 0.07 });
  }
};
