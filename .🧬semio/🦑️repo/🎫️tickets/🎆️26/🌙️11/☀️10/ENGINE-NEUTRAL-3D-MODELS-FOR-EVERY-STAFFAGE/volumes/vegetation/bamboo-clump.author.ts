import { Author } from "../../veg-lib.ts";
const a = new Author("bamboo-clump");
const zc = (id: string) => { const b = a.box2("front", id); return (b.minX + b.maxX) / 2; };
const specs: { bone: string; zStart?: number; zEnd: number; meta: any }[] = [];
for (const n of [1, 2, 3, 4, 5, 6]) for (const p of ["a", "b", "c"]) {
  const id = `culm-${n}-${p}`;
  specs.push({ bone: id, zStart: p === "a" ? zc(id) : undefined, zEnd: zc(id), meta: a.meta(id) });
}
a.limbs(specs);
for (const sh of a.st.plan) {
  const id = sh.shape.id as string;
  if (!sh.closed || !/^mass-\d-(shade|sun)$/.test(id)) continue;
  a.lobe(id, sh.shape.bone, id, id, { m: a.meta(id), k: 1.0 });
}
a.note("Six culms, each in three bones (lower, middle, upper), are tapered capsule chains standing at the lateral positions of their front outlines and leaning out as the elevation shows; a clump, not a row.", "Sechs Halme, je in drei Knochen (unten, mitte, oben), sind verjüngte Kapselketten an den Seitenlagen ihrer Front-Umrisse und neigen sich nach außen wie in der Ansicht; ein Horst, keine Reihe.");
a.note("Each foliage bone carries a shade mass and a smaller sun mass as oriented ellipsoids fitted to the plan outlines at the height of the front outlines; individual leaves are not modelled.", "Jeder Laubknochen trägt eine Schattenmasse und eine kleinere Sonnenmasse als ausgerichtete Ellipsoide, an die Umrisse der Draufsicht angepasst und in der Höhe der Front-Umrisse; einzelne Blätter sind nicht modelliert.");
a.save();
