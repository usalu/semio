export default (a) => {
  const e = a.representations["500"].geometry.elevation;
  const pen = Number(process.env.PEN500 ?? 0.2);
  for (const id of ["rail", "handle", "plate"]) { const s = e.find((x) => x.id === id); if (s) s.penMm = Number(process.env["PEN_" + id] ?? pen); }
};
