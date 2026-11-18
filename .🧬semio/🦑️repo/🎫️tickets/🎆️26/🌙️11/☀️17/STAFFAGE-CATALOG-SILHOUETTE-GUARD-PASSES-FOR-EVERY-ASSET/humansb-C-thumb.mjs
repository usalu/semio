export default (a) => {
  const d = Number(process.env.TD ?? 0.007);
  for (const s of a.volumes["50"].body) if (s.id === "hand-l-glove-thumb-l" || s.id === "hand-r-glove-thumb-r") { s.p0[1] -= d; s.p1[1] -= d; }
};
