export default (a) => {
  const g = a.representations["50"].geometry.elevation;
  g.find((s) => s.id === "grip").d = process.env.GD;
};
