export default (a) => {
  const b = a.volumes["50"].body;
  for (const [id, z] of [["toe-l-boots-hi", 0.01], ["toe-r-boots-hi", -0.01], ["foot-l-boots-hi", 0.01], ["foot-r-boots-hi", -0.01]])
    for (const s of b.find((x) => x.id === id).sections) s.at[2] = z;
  const rx = [0.0448, 0.0446, 0.0444, 0.0443, 0.0466];
  for (const id of ["toe-l-upper", "toe-l-sole", "toe-r-upper", "toe-r-sole"]) {
    const s = b.find((x) => x.id === id);
    rx.forEach((v, k) => (s.sections[3 + k].rx = v));
  }
};
