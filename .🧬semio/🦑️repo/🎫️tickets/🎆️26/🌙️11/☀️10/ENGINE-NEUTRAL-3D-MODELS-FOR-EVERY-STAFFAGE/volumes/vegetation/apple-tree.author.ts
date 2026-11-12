import { Author, type V3 } from "../../veg-lib.ts";
const a = new Author("apple-tree");
const bark = a.meta("stem"), limb = a.meta("wood-sc-w1a"), twig = a.meta("stem", { material: "apple-twig", region: "spur-twig", tone: 0.3 });

a.loftFromViews("stem", "trunk", { elev: "stem", front: "stem", range: [0, 1.6], n: 6, exp: 2.1, pinch: 1 });
a.loftFromViews("leader", "ld", { elev: "stem", front: "stem-fork", range: [1.6, 3.8], n: 5, exp: 2.1, pinch: 1, m: a.meta("stem-fork") });
a.loftFromViews("leader-top", "ld-b", { elev: "wood-ld-b", front: "wood-ld-b", range: [3.8, 5.33], n: 4, exp: 2.1, pinch: 0.3, m: a.meta("wood-ld-b") });

const sc: Record<string, { a: string; b: string; s: string; fol: string; o: V3; t: V3; e: V3; ra: [number, number]; rb: [number, number] }> = {
  w1: { a: "sc-w1a", b: "sc-w1b", s: "spur-w1", fol: "fol-w1", o: [0, 1.55, 0], t: [-1.379, 2.707, 1.58], e: [-2.94, 3.04, 2.3], ra: [0.11, 0.08], rb: [0.075, 0.04] },
  e1: { a: "sc-e1a", b: "sc-e1b", s: "spur-e1", fol: "fol-e1", o: [0, 1.6, 0], t: [1.497, 2.77, -1.65], e: [3.06, 3.1, -2.4], ra: [0.11, 0.08], rb: [0.075, 0.04] },
  w2: { a: "sc-w2a", b: "sc-w2b", s: "spur-w2", fol: "fol-w2", o: [-0.004, 2.1, 0], t: [-1.037, 4.042, 1.15], e: [-2.1, 5.1, 1.5], ra: [0.085, 0.06], rb: [0.06, 0.035] },
  e2: { a: "sc-e2a", b: "sc-e2b", s: "spur-e2", fol: "fol-e2", o: [-0.006, 2.3, 0], t: [1.002, 4.367, -1.15], e: [2.0, 5.45, -1.45], ra: [0.085, 0.06], rb: [0.06, 0.035] },
};
for (const [k, c] of Object.entries(sc)) {
  a.cap(`wood-${c.a}`, c.a, c.o, c.t, c.ra[0], c.ra[1], a.meta(`wood-${c.a}`));
  a.cap(`wood-${c.b}`, c.b, c.t, c.e, c.rb[0], c.rb[1], a.meta(`wood-${c.b}`));
  const f = a.box2("plan", { w1: "back", e1: "front", w2: "left", e2: "right" }[k]!);
  const s0: V3 = [c.e[0] + (k.startsWith("w") ? 0.35 : -0.35), c.e[1] - 0.05, c.e[2]];
  a.cap(`spur-${k}`, c.s, c.e, [c.e[0] + (c.e[0] < 0 ? -0.1 : 0.1), c.e[1] + 0.55, c.e[2] * 0.95], 0.02, 0.01, twig);
}

const masses: [string, string, string, string][] = [
  ["fol-core-low-mass", "core-low", "fol-core-low", "core-low"],
  ["fol-core-up-mass", "core-up", "fol-core-up", "core-up"],
  ["fol-w1-mass", "left-low", "fol-w1", "back"],
  ["fol-e1-mass", "right-low", "fol-e1", "front"],
  ["fol-w2-mass", "left-up", "fol-w2", "left"],
  ["fol-e2-mass", "right-up", "fol-e2", "right"],
  ["fol-top-mass", "top", "fol-top", "top"],
];
for (const [e, f, bone] of masses.map((m) => [m[0], m[1], m[2]] as const)) a.loftFromViews(e, bone, { elev: e, front: f, n: 9, exp: 2.4, pinch: 0.25, m: a.meta(e) });
const quad = (id: string, bone: string, planId: string, y0: number, y1: number, m = a.meta(planId)) => {
  const b = a.box2("plan", planId);
  a.blob(id, bone, [(b.minX + b.maxX) / 2, (y0 + y1) / 2, (b.minY + b.maxY) / 2], [(b.maxX - b.minX) / 2, (y1 - y0) / 2, (b.maxY - b.minY) / 2], m, { n: 9, exp: 2.6 });
};
quad("crown-fill-1", "fol-top", "crown-fill-1", 3.2, 5.2);
quad("crown-fill-2", "fol-core-up", "crown-fill-2", 3.0, 4.9);
quad("crown-fill-3", "fol-w2", "crown-fill-3", 3.0, 4.4);
const zc: Record<string, number> = { "fol-w1": 1.8, "fol-e1": -1.9, "fol-w2": 1.2, "fol-e2": -1.16, "fol-top": 0 };
const fruitIds = ["w1-fruit1", "w1-fruit2", "w1-fruit3", "e1-fruit1", "e1-fruit2", "e1-fruit3", "w2-fruit1", "e2-fruit1", "e2-fruit2", "e2-fruit3", "e2-fruit4", "top-fruit1", "top-fruit2", "top-fruit3", "top-fruit4"];
fruitIds.forEach((f, i) => {
  const id = `fol-${f}`;
  const bone = id.replace(/-fruit\d+$/, "");
  const b = a.box2("elevation", id);
  const z = zc[bone]! * (0.55 + 0.25 * (i % 3)) + (bone === "fol-top" ? [0.5, -0.5, 0.2, -0.3][i % 4]! : 0);
  a.ell(id, bone, [(b.minX + b.maxX) / 2, (b.minY + b.maxY) / 2, z], [0.04, 0.04, 0.04], a.meta(id), { extra: { maxDenominator: 50 } });
});
a.note("The crown is seven foliage masses and three quadrant fills, each a vertical loft of superelliptic sections whose x and z half extents are read from the authored elevation and front outlines, bound to the foliage bone that carries it so sway clips move it.", "Die Krone besteht aus sieben Laubmassen und drei Quadrantenfüllungen, jeweils ein senkrechter Loft aus superelliptischen Querschnitten, deren Halbachsen in x und z aus den Umrissen in Ansicht und Front stammen, an den tragenden Laubknochen gebunden, damit Schwankungsclips sie mitnehmen.");
a.note("The three crown-fill outlines of the plan view lie in the quadrants the four scaffold masses leave free (+x/+z, -x/-z and the middle of the +z side); they are modelled as flattened blobs at crown height so the crown is round in plan as drawn.", "Die drei Kronenfüllungen der Draufsicht liegen in den Quadranten, die die vier Leitastmassen freilassen; sie sind als abgeflachte Blobs in Kronenhöhe modelliert, damit die Krone in der Draufsicht wie gezeichnet rund ist.");
a.note("The scaffolds leave the stem in the sagittal bone plane but are drawn spreading sideways in the front view; each is a two-bone capsule chain ending in the middle of its foliage mass, so the lateral position follows the front view (about 1.6 m after the first, 2.3 m after the second capsule).", "Die Leitäste verlassen den Stamm in der Knochenebene, sind aber in der Front seitlich ausladend gezeichnet; jeder ist eine Kapselkette über zwei Knochen, die in der Mitte ihrer Laubmasse endet, die Seitenlage folgt der Front.");
a.note("Fruits are 4 cm ellipsoids placed from the elevation; their lateral position is estimated inside the mass because the 2D views do not agree on fruit positions; shown at 1:50 only.", "Früchte sind Ellipsoide mit 4 cm Radius, nach der Ansicht platziert; die Seitenlage ist innerhalb der Masse geschätzt, da die 2D-Ansichten die Fruchtpositionen nicht übereinstimmend zeigen; nur bei 1:50 sichtbar.");
a.save();
