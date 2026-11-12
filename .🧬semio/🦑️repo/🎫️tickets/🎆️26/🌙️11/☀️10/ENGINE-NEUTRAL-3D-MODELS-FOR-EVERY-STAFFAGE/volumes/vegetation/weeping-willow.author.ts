import { Author, type V3, type Section } from "../../veg-lib.ts";
const a = new Author("weeping-willow");
a.loftFromViews("root-flare", "root-plate", { elev: "root-flare", front: "root-flare", range: [0, 0.34], n: 4, exp: 2.2, pinch: 1 });
a.loftFromViews("trunk-1", "trunk-1", { elev: "trunk-1", front: "trunk-1", range: [0.3, 2.2], n: 5, exp: 2.1, pinch: 1 });
a.loftFromViews("trunk-2", "trunk-2", { elev: "trunk-2", front: "trunk-2", range: [2.2, 3.5], n: 4, exp: 2.1, pinch: 1 });
const limbIds = ["limb-w-a", "limb-w-b", "limb-w-c", "limb-e-a", "limb-e-b", "limb-e-c"];
a.limbs(limbIds.map((bone) => ({ bone, meta: a.meta(bone) })));
const leaf = a.meta("f-c4-0");
const bark = a.meta("limb-w-a");
for (const sh of a.st.plan) {
  const id = sh.shape.id as string;
  if (!sh.closed || sh.shape.role !== "body" || !/^p-c\d/.test(id)) continue;
  const name = id.slice(2), bone = sh.shape.bone as string;
  const fb = a.box2("front", `f-${name}`), eb = a.box2("elevation", bone);
  const { cx, cz, phi, a: ra, b: rb } = a.pca(id, 0.94);
  const yT = eb.maxY - 0.1, yB = Math.max(0.3, fb.minY + 0.3);
  const prof: [number, number][] = [[0, 0.4], [0.08, 0.9], [0.3, 1], [0.7, 1], [0.93, 0.92], [1, 0.55]];
  const secs: Section[] = prof.map(([t, f]) => ({ c: [cx, yT - (yT - yB) * t, cz] as V3, rx: ra * f, ry: rb * f, exp: 2.6 }));
  a.loft(`curtain-${name}`, bone, secs, a.meta(`f-${name}`), { axis: [0, 1, 0], xdir: [Math.cos(phi), 0, Math.sin(phi)] });
  const og = a.origin(bone);
  const parent = a.ends.get(a.bone(bone).parent);
  const zo = parent ? parent.o[2] + ((og[0] - parent.o[0]) / (parent.t[0] - parent.o[0] || 1)) * (parent.t[2] - parent.o[2]) : 0;
  a.cap(`strand-${name}`, bone, [og[0], og[1], zo], [cx, yT - 0.5, cz], 0.05, 0.03, bark);
}
for (const sh of a.st.plan) {
  const id = sh.shape.id as string;
  if (!sh.closed || sh.shape.role !== "body" || !/^p-cap-/.test(id)) continue;
  const name = id.slice(2), bone = sh.shape.bone as string;
  a.lobe(`dome-${name}`, bone, id, `f-${name}`, { m: a.meta(bone), k: 1.0 });
}
const fillOpts = { elevIds: a.idsIn("elevation", /^leaf/), frontIds: a.idsIn("front", /^leaf/) };
a.fill("crown-fill-1", "curt-w2", "crown-fill-1", { ...fillOpts, y: [0.6, 9.4], shrink: 0.9, exp: 3, extra: { weights: [{ bone: "curt-w2", weight: 0.5 }, { bone: "curt-w1", weight: 0.5 }] } });
a.note("Every drooping curtain of the weeping crown is one vertical loft hung on its curtain bone: the footprint is the principal-axis ellipse of its authored plan outline (superellipse exponent 2.6), the height runs from the limb down to the ground of its front outline, and the top and bottom are tucked in so it reads as a draped sheet. A thin strand capsule hangs it from the bone origin on the limb.", "Jeder hängende Vorhang der Trauerkrone ist ein senkrechter Loft an seinem Vorhangknochen: der Grundriss ist die Hauptachsenellipse seines Umrisses in der Draufsicht (Superellipsen-Exponent 2,6), die Höhe reicht vom Ast bis zum Boden seines Front-Umrisses, oben und unten ist er eingezogen, damit er wie ein drapiertes Tuch wirkt. Eine dünne Strangkapsel hängt ihn am Knochenursprung des Astes auf.");
a.note("The crown caps above the limbs are ellipsoid domes fitted to the plan outlines with heights read from the elevation cap shape; one weighted crown fill closes the plan quarter between curtains.", "Die Kronenkappen über den Ästen sind Ellipsoid-Kuppeln, an die Umrisse der Draufsicht angepasst, mit Höhen aus dem Elevationsumriss der Kappe; eine gewichtete Kronenfüllung schließt das Viertel der Draufsicht zwischen den Vorhängen.");
a.note("Limbs are tapered capsules whose lateral drift is read from the front outlines (west limbs spread toward +z, east toward -z); the curtains are placed around the whole crown from the plan view, so strands carry the foliage from the limb to curtains on the other side.", "Äste sind verjüngte Kapseln, deren Seitendrift den Front-Umrissen folgt (Westäste nach +z, Ostäste nach -z); die Vorhänge sind nach der Draufsicht rund um die Krone verteilt, Stränge tragen das Laub vom Ast zu den Vorhängen der Gegenseite.");
a.save();
