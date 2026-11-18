export default (a) => {
  const pts = [];
  for (const s of a.volumes[200].body) if (s.id.startsWith("hose-")) { if (pts.length === 0) pts.push(s.p0); pts.push(s.p1); }
  const sh = a.representations[200].geometry.elevation.find((s) => s.id === "hose");
  sh.d = "M " + pts.map((p) => `${p[0]} ${p[1]}`).join(" L ");
  if (process.env.PEN) sh.penMm = Number(process.env.PEN);
};
