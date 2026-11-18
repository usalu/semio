export default (a) => {
  for (const sc of (process.env.BODY_SCALES ?? "50,100").split(",")) {
    const b = a.volumes[sc].body;
    const f = (id) => b.find((s) => s.id === id);
    for (const [id, z] of [["toe-l-boots-hi", 0.01], ["toe-r-boots-hi", -0.01], ["foot-l-boots-hi", 0.01], ["foot-r-boots-hi", -0.01]]) f(id)?.sections.forEach((s) => (s.at[2] = z));
    for (const id of ["toe-l-upper", "toe-l-sole", "toe-r-upper", "toe-r-sole"]) { const s = f(id); if (s && s.sections.length >= 8 && f("toe-l-boots-hi")) [0.0448, 0.0446, 0.0444, 0.0443, 0.0466].forEach((v, k) => (s.sections[s.sections.length - 5 + k].rx = v)); }
    for (const id of ["toe-l-boots-hi", "toe-r-boots-hi"]) { const s = f(id); if (s) s.sections[0].at[0] = 0.0646; }
    for (const id of ["pelvis-trousers", "pelvis-skin"]) { const s = f(id); if (s) for (const k of s.sections) { k.ry -= 0.012; k.at[1] += 0.012; } }
    for (const id of ["hand-l-glove-thumb-l", "hand-r-glove-thumb-r"]) { const s = f(id); if (s) { s.p0[1] -= Number(process.env.THUMB ?? 0.008); s.p1[1] -= Number(process.env.THUMB ?? 0.008); } }
    for (const s of b) if (/^(foot|toe)-[lr]-(sole|boots-hi)$/.test(s.id)) for (const q of s.sections) q.at[1] += 0.007;
  }
};
