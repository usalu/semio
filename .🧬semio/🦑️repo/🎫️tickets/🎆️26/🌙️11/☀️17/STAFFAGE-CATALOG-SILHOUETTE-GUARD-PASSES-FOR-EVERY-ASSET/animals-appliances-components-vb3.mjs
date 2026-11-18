export default (a) => {
  for (const sc of [100, 200]) for (const s of a.volumes[sc].body) if (s.id.startsWith("guide")) delete s.slender;
  for (const s of a.volumes[100].body) if (s.id.startsWith("slat-")) for (const q of s.sections) q.ry = Number(process.env.RY);
};
