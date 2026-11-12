import { Author } from "../../veg-lib.ts";
const a = new Author("ivy-on-trellis");
for (const s of ["L", "R"]) {
  const p = a.box2("elevation", `post-${s}`), sh = a.box2("elevation", `shoe-${s}`);
  a.box(`post-${s.toLowerCase()}`, `post-${s}`, [(p.minX + p.maxX) / 2, (p.minY + p.maxY) / 2, 0], [p.maxX - p.minX, p.maxY - p.minY, 0.1], a.meta(`post-${s}`));
  a.box(`shoe-${s.toLowerCase()}`, `post-${s}`, [(sh.minX + sh.maxX) / 2, (sh.minY + sh.maxY) / 2, 0], [sh.maxX - sh.minX, sh.maxY - sh.minY, 0.13], a.meta(`shoe-${s}`), { cornerRadius: 0.01 });
}
for (const r of ["rail-top", "rail-bottom", "batten-h-0", "batten-h-1"]) {
  const e = a.box2("elevation", r);
  a.box(r, "panel", [(e.minX + e.maxX) / 2, (e.minY + e.maxY) / 2, 0], [e.maxX - e.minX, e.maxY - e.minY, r.startsWith("rail") ? 0.07 : 0.05], a.meta(r));
}
{
  const e = a.box2("elevation", "batten-v-0");
  a.box("batten-v", "panel", [(e.minX + e.maxX) / 2, (e.minY + e.maxY) / 2, 0], [e.maxX - e.minX, e.maxY - e.minY, 0.05], a.meta("batten-v-0"), { extra: { repeat: { count: 8, translation: [0.2, 0, 0] } } });
}
for (const n of [1, 2, 3, 4]) {
  const o = a.origin(`stem-${n}`), top = a.tip(`stem-${n}`);
  a.cap(`stem-${n}`, `stem-${n}`, [o[0], o[1] + 0.05, 0.02], [o[0] + 0.02, top[1] - 0.1, 0.02], 0.014, 0.01, a.meta(`stem-${n}`));
  for (const h of ["lo", "hi"]) {
    const id = `mat-${n}-${h}-0`, m = a.box2("elevation", id), p = a.box2("plan", id);
    a.box(`mat-${n}-${h}`, `fol-${n}-${h}`, [(m.minX + m.maxX + p.minX + p.maxX) / 4, (m.minY + m.maxY) / 2, (p.minY + p.maxY) / 2], [(m.maxX - m.minX + p.maxX - p.minX) / 2, m.maxY - m.minY, p.maxY - p.minY], a.meta(id), { cornerRadius: (p.maxY - p.minY) / 2 * 0.98 });
  }
}
a.note("Posts, shoes, rails and battens are boxes of the authored sizes; the eight vertical battens are one box with a repeat of 0.2 m, the horizontal ones two boxes. The panel is 0.05 to 0.07 m deep as the plan view draws it.", "Pfosten, Schuhe, Riegel und Latten sind Boxen in den gezeichneten Größen; die acht senkrechten Latten sind eine Box mit Wiederholung im Abstand 0,2 m, die waagerechten zwei Boxen. Das Gitter ist 0,05 bis 0,07 m tief wie in der Draufsicht gezeichnet.");
a.note("The ivy is four climbing stems (thin capsules) and per stem a lower and an upper leaf mat, each a rounded box as wide and tall as its elevation outline and as thick as its plan outline (0.19 to 0.24 m, centred on the trellis); individual leaves are not modelled.", "Der Efeu besteht aus vier kletternden Trieben (dünne Kapseln) und je Trieb einer unteren und oberen Blattmatte, jeweils eine gerundete Box so breit und hoch wie ihr Umriss in der Ansicht und so dick wie ihr Umriss in der Draufsicht (0,19 bis 0,24 m, mittig am Gitter); einzelne Blätter sind nicht modelliert.");
a.save();
