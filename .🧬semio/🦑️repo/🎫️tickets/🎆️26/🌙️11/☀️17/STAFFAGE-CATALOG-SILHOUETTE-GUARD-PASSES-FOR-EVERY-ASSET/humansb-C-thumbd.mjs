export default (a) => {
  const dd = Number(process.env.TDD ?? 0.012);
  for (const sc of (process.env.SCALES ?? "100").split(",")) for (const side of ["l", "r"]) {
    const s = a.representations[sc].geometry.elevation.find((x) => x.id === "glove-thumb-" + side);
    s.d = s.d.replace("L0.06727 0.03080", `L0.06727 ${(0.0308 - dd).toFixed(5)}`).replace("L0.11581 0.06470", `L0.11581 ${(0.0647 - dd).toFixed(5)}`);
  }
};
