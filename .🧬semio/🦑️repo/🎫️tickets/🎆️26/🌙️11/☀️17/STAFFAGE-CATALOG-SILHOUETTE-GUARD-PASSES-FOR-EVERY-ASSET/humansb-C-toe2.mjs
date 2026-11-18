export default (a) => {
  const b = a.volumes["50"].body;
  for (const id of ["toe-l-boots-hi", "toe-r-boots-hi"]) b.find((x) => x.id === id).sections[0].at[0] = Number(process.env.CAPX ?? 0.0646);
};
