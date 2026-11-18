export default (a) => {
  const sc = process.env.SC ?? "50", v = process.env.VW ?? "front";
  const g = a.representations[sc].geometry[v];
  for (const spec of (process.env.ZT ?? "").split(";").filter(Boolean)) {
    const [id, z0, z1, t0, t1] = spec.split(",");
    const s = g.find((x) => x.id === id);
    if (z0) s.solid.z0 = Number(z0);
    if (z1) s.solid.z1 = Number(z1);
    if (t0 && t1) s.solid.zTip = [Number(t0), Number(t1)];
  }
};
