export default (a) => {
  for (const sc of (process.env.PARCEL_SCALES ?? "100,200,500").split(",")) {
    const rep = a.representations[sc].geometry, vol = a.volumes[sc].body;
    const top = -0.012;
    const e = (id) => rep.elevation.find((s) => s.id === id);
    for (const id of ["parcel-a", "parcel-b"]) { const s = e(id); s.points[0][1] = top; s.points[1][1] = top; }
    const c = e("parcel-c");
    if (c.d) c.d = c.d.replace(/-0\.02000|-0\.02076|-0\.01874|-0\.01623/g, (m) => m).replace(/^M0\.76000 -0\.02000/, "M0.76000 -0.01200").replace(/L0\.97214 -0\.02000/, "L0.97214 -0.01200").replace(/L0\.98710 -0\.02076/, "L0.98710 -0.01276");
    else { c.points[0][1] = top; c.points[1][1] = top; }
    const v = (id) => vol.find((s) => s.id === id);
    for (const [id, y] of [["parcel-a", -0.216], ["parcel-b", -0.196], ["parcel-c", -0.1541]]) { const s = v(id); const d = y - s.transform.translation[1]; if (Math.abs(d) < 0.01) { s.size[1] = +(s.size[1] + 0.008).toFixed(4); s.transform.translation[1] = +(s.transform.translation[1] + 0.004).toFixed(4); } }
  }
};
