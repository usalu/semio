export default (a) => {
  for (const sc of [100, 200]) for (const s of a.volumes[sc].body) if (s.id.startsWith("guide")) s.slender = false;
  const g = a.representations[100].geometry;
  for (const s of g.elevation) if (s.id.startsWith("slat-")) { s.role = "texture"; s.penMm = 0.1; }
};
