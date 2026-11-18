export default (asset) => {
  const e = Number(process.env.EXP ?? 4), d = Number(process.env.D ?? 0.003);
  for (const s of asset.volumes["50"].body) {
    if (["hindbody","chest"].includes(s.id)) for (const q of s.sections) q.exponent = e;
    if (["near-forearm","far-forearm"].includes(s.id)) for (const q of s.sections.slice(1,4)) q.rx += d;
  }
};
