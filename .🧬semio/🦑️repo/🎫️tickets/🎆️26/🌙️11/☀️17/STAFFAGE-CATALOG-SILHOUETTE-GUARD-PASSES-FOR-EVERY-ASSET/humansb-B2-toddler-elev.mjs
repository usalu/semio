export default (a) => {
  const e = a.representations["200"].geometry.elevation.find((s) => s.id === "toddler-boot-n");
  e.points = e.points.map(([x, y]) => (x > 0.2 ? [+(x + 0.012).toFixed(5), y] : [x, y]));
};
