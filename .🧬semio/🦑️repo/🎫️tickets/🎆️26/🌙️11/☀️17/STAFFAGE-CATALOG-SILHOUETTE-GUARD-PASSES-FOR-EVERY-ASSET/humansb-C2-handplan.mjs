export default (a) => {
  const g = a.representations["50"].geometry.plan;
  const x0 = Number(process.env.HX0 ?? 0.43575), x1 = Number(process.env.HX1 ?? 0.39775), r0 = Number(process.env.HR0 ?? 0.03), r1 = Number(process.env.HR1 ?? 0.03);
  for (const id of ["hand-l", "hand-r"]) Object.assign(g.find((s) => s.id === id), { x0, x1, r0, r1 });
};
