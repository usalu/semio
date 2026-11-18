export default (a) => {
  const b = a.volumes["100"].body;
  for (const id of (process.env.RM ?? "rear-cog,chainring").split(",")) b.splice(b.findIndex((s) => s.id === id), 1);
};
