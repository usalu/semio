export default (a) => {
  const b = a.volumes["50"].body;
  b.find((s) => s.id === "neck").sections[9].rx = 0.0476;
};
