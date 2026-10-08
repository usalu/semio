import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const C = "brep.intersect";
const cutter = { pick: [pick("shape", "solid")], gumball: [gumball("translate", "plane"), gumball("rotate", "plane")] };
const tol = (en, de) => I("tolerance", { description: T(en, de) });
const points = (en, de) => O("points", "point", T("Points", "Punkte"), T(en, de), { list: true });
const wireOut = (en, de) => O("wire", "shape", T("Wire", "Kantenzug"), T(en, de), { shapeKinds: ["wire"], optional: true });

export default {
  category: {
    id: C, emoji: "✂", order: 80,
    label: T("Sections and intersections", "Schnitte und Schnittmengen"),
    description: T("Cut solids with planes and intersect curves and surfaces.", "Körper mit Ebenen schneiden sowie Kurven und Flächen schneiden."),
  },
  kinds: [
    K(C, {
      was: "brep.intersect.section", id: "brep.intersect.section", emoji: "🍰", quality: "exact-analytic", preview: true,
      label: T("Section", "Schnitt"),
      description: T("Cuts a solid with a plane and returns the cross-section faces; sections of curved faces are not exact yet.", "Schneidet einen Körper mit einer Ebene und liefert die Schnittflächen; Schnitte gekrümmter Flächen sind noch nicht exakt."),
      in: [I("solid"), I("plane")],
      out: [O("faces", "shapes", T("Section faces", "Schnittflächen"), T("Every cross-section face the plane produced.", "Jede Schnittfläche, die die Ebene erzeugt hat."), { shapeKinds: ["face"] })],
      interaction: cutter,
      ren: { in: { solid: "solid", "planeOrigin, planeNormal": "plane" } },
      note: "planeOrigin and planeNormal merged into one plane port; faces becomes a shapes output",
    }),
    K(C, {
      was: "brep.intersect.split", id: "brep.intersect.split", emoji: "✂", quality: "mesh-derived-brep", preview: true,
      label: T("Split", "Teilen"),
      description: T("Splits a solid at a plane into the part in front of and the part behind it.", "Teilt einen Körper an einer Ebene in den Teil vor und den Teil hinter der Ebene."),
      in: [I("solid"), I("plane")],
      out: [
        O("positive", "shape", T("Front", "Vorderseite"), T("The part on the side the plane normal points to.", "Der Teil auf der Seite, zu der die Ebenennormale zeigt."), { shapeKinds: ["solid"] }),
        O("negative", "shape", T("Back", "Rückseite"), T("The part on the opposite side.", "Der Teil auf der gegenüberliegenden Seite."), { shapeKinds: ["solid"] }),
      ],
      interaction: cutter,
      ren: { in: { solid: "solid", "planeOrigin, planeNormal": "plane" } },
      note: "planeOrigin and planeNormal merged into one plane port",
    }),
    K(C, {
      was: "brep.intersect.curveCurve", id: "brep.intersect.curveCurve", emoji: "❌", quality: "exact-numerical", preview: true,
      label: T("Curve-curve intersection", "Kurve-Kurve-Schnitt"),
      description: T("Finds the points where two curves meet.", "Findet die Punkte, an denen sich zwei Kurven treffen."),
      in: [I("curve", { as: "a", label: T("Curve A", "Kurve A"), description: T("The first curve.", "Die erste Kurve.") }), I("curve", { as: "b", label: T("Curve B", "Kurve B"), description: T("The second curve.", "Die zweite Kurve.") }), tol("Largest gap that still counts as a meeting point.", "Größter Abstand, der noch als Treffpunkt gilt.")],
      out: [points("The intersection points.", "Die Schnittpunkte."), wireOut("A polyline through the points; absent when fewer than two points were found.", "Eine Polylinie durch die Punkte; entfällt, wenn weniger als zwei Punkte gefunden wurden.")],
      ren: { out: { wire: "wire (optional) and points" } },
    }),
    K(C, {
      was: "brep.intersect.curveSurface", id: "brep.intersect.curveSurface", emoji: "📌", quality: "exact-numerical", preview: true,
      label: T("Curve-surface intersection", "Kurve-Fläche-Schnitt"),
      description: T("Finds the points where a curve pierces a surface.", "Findet die Punkte, an denen eine Kurve eine Fläche durchstößt."),
      in: [I("curve"), I("surface"), tol("Largest gap that still counts as a meeting point.", "Größter Abstand, der noch als Treffpunkt gilt.")],
      out: [points("The intersection points.", "Die Schnittpunkte."), wireOut("A polyline through the points; absent when fewer than two points were found.", "Eine Polylinie durch die Punkte; entfällt, wenn weniger als zwei Punkte gefunden wurden.")],
      ren: { out: { wire: "wire (optional) and points" } },
    }),
    K(C, {
      was: "brep.intersect.surfaceSurface", id: "brep.intersect.surfaceSurface", emoji: "🌐", quality: "exact-numerical", preview: true,
      label: T("Surface-surface intersection", "Fläche-Fläche-Schnitt"),
      description: T("Finds the curves along which two surfaces meet.", "Findet die Kurven, entlang derer sich zwei Flächen treffen."),
      in: [I("surface", { as: "a", label: T("Surface A", "Fläche A"), description: T("The first surface.", "Die erste Fläche.") }), I("surface", { as: "b", label: T("Surface B", "Fläche B"), description: T("The second surface.", "Die zweite Fläche.") }), tol("Largest gap that still counts as a meeting curve.", "Größter Abstand, der noch als Schnittkurve gilt.")],
      out: [O("wires", "shapes", T("Intersection wires", "Schnittkantenzüge"), T("Every intersection wire; two surfaces can meet along several disjoint curves.", "Jeder Schnittkantenzug; zwei Flächen können sich entlang mehrerer getrennter Kurven treffen."), { shapeKinds: ["wire"] })],
      ren: { out: { wires: "wires" } },
    }),
  ],
};
