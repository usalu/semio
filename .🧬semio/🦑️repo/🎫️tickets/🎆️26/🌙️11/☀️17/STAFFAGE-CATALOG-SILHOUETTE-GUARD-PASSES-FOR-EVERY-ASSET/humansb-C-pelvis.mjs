export default (a) => {
  const d = Number(process.env.PD ?? 0.012);
  for (const s of a.volumes["50"].body) if (["pelvis-trousers", "pelvis-skin"].includes(s.id)) for (const k of s.sections) { k.ry -= d; k.at[1] -= d * Number(process.env.PS ?? 1); }
};
