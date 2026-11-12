import { Author, type V3, type Section } from "../../veg-lib.ts";
const a = new Author("hosta-clump");
const bones: string[] = [];
for (const side of ["R", "L"]) for (const n of [1, 2, 3, 4]) bones.push(`${side}${n}`);
bones.push("C1");
for (const n of bones) {
  const pb = `petiole-${n}`, bb = `blade-${n}`;
  const po = a.origin(pb), pt = a.tip(pb);
  a.cap(pb, pb, po, pt, 0.016, 0.012, a.meta(pb));
  const bo = a.origin(bb), bt = a.tip(bb);
  const pl = a.box2("plan", bb), el = a.box2("elevation", bb);
  const lat = n === "C1" ? a.box2("plan", bb) : pl;
  const half = n === "C1" ? (pl.maxY - pl.minY) / 2 : (pl.maxY - pl.minY) / 2;
  const cz = (pl.minY + pl.maxY) / 2;
  const prof: [number, number][] = [[0, 0.25], [0.15, 0.8], [0.4, 1], [0.7, 0.8], [0.92, 0.4], [1, 0.08]];
  const secs: Section[] = prof.map(([q, w]) => ({ c: [bo[0] + (bt[0] - bo[0]) * q, bo[1] + (bt[1] - bo[1]) * q, cz] as V3, rx: Math.max(half * w, 0.004), ry: 0.004, exp: 2.2 }));
  a.loft(bb, bb, secs, a.meta(bb), { axis: [bt[0] - bo[0], bt[1] - bo[1], 0].map((v, i) => (i === 1 ? v : v)) as V3 });
  a.cap(`midrib-${n.toLowerCase()}`, bb, [bo[0], bo[1], 0], [bt[0], bt[1], 0], 0.005, 0.003, a.meta(bb, { material: "hosta-petiole", role: "detail", tone: 0.3, region: "petioles" }), { maxDenominator: 50 });
}
a.note("Hosta: nine petioles (tapered capsules) each carry a lamina loft: a flat leaf of 0.23 m width and 0.28 to 0.31 m length with an elliptical outline (widest at 40 percent), 8 mm thick, centred on the lateral offset of the plan outline; the midrib is a thin capsule on top. The rest pose is a flat fan in the sagittal plane; the spread to depth 0.9 m appears only in the default pose through the abduction and twist of the petiole bones, so the bounding box is judged in that pose.", "Funkie: neun Blattstiele (verjüngte Kapseln) tragen je einen Spreitenloft: ein flaches Blatt mit 0,23 m Breite und 0,28 bis 0,31 m Länge, elliptischem Umriss (am breitesten bei 40 Prozent), 8 mm dick, mittig auf dem Seitenversatz des Umrisses der Draufsicht; die Mittelrippe ist eine dünne Kapsel obenauf. Die Ruhelage ist ein flacher Fächer in der Längsebene; die Auffächerung auf 0,9 m Tiefe entsteht nur in der Standardpose durch Abduktion und Verdrehung der Stielknochen, daher wird die Bounding Box in dieser Pose beurteilt.");
a.save();
