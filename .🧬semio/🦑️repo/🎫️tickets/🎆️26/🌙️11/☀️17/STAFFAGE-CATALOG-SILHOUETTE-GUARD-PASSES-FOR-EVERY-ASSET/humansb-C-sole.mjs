export default (a) => {
  const d = Number(process.env.SD ?? 0.007), c = 1;
  for (const s of a.volumes["50"].body) if (/^(foot|toe)-[lr]-(sole|boots-hi)$/.test(s.id)) for (const q of s.sections) q.at[c] += d;
};
