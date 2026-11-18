export default (a) => {
  const body = (sc) => a.volumes[sc].body;
  const g = (sc, v) => a.representations[sc].geometry[v];
  const lift = (sc, id, dy) => { for (const s of body(sc).find((x) => x.id === id).sections) s.at[1] += dy; };
  lift("500", "toe-l", 0.041); lift("500", "toe-r", 0.041); lift("500", "foot-r", 0.038); lift("500", "foot-l", 0.03);
  const fp = g("500", "elevation").find((s) => s.id === "fit-foot-r-500-1");
  fp.points = fp.points.map(([x, y]) => [x, +(y + 0.041).toFixed(5)]);
  const c = g("500", "plan").find((s) => s.id === "toddler-boot-r"); c.x1 = 0.41;
  const p = g("200", "plan").find((s) => s.id === "toddler-boot-r");
  p.points = p.points.map(([x, y]) => (x === 0.43405 ? [0.44605, y] : [x, y]));
  const v = body("200").find((x) => x.id === "toddler-shank-n-toddler-boots"); v.sections[3].at[0] = 0.2175;
};
