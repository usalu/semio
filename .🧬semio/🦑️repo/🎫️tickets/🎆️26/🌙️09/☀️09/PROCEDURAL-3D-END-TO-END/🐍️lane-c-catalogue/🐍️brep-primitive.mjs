import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const C = "brep.primitive";
const solid = (en, de) => O("shape", "shape", T("Solid", "Körper"), T(en, de), { shapeKinds: ["solid"] });
const seed = { pick: [] };

export default {
  category: {
    id: C, emoji: "🧊", order: 10,
    label: T("Primitives", "Grundkörper"),
    description: T("Ready-made solids: box, sphere, cylinder, cone, torus and convex hull.", "Fertige Körper: Quader, Kugel, Zylinder, Kegel, Torus und konvexe Hülle."),
  },
  kinds: [
    K(C, {
      was: "brep.prim3d.box", id: "brep.primitive.box", emoji: "📦", quality: "exact-analytic", preview: true,
      label: T("Box", "Quader"),
      description: T("Creates an axis-aligned box with its corner at the origin.", "Erzeugt einen achsenparallelen Quader mit einer Ecke im Ursprung."),
      in: [I("width"), I("depth"), I("height")],
      out: [solid("The box as a closed solid.", "Der Quader als geschlossener Körper.")],
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.prim3d.sphere", id: "brep.primitive.sphere", emoji: "⚪", quality: "exact-analytic", preview: true,
      label: T("Sphere", "Kugel"),
      description: T("Creates a sphere centered at the origin.", "Erzeugt eine Kugel mit dem Mittelpunkt im Ursprung."),
      in: [I("radius")],
      out: [solid("The sphere as a closed solid.", "Die Kugel als geschlossener Körper.")],
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.prim3d.cylinder", id: "brep.primitive.cylinder", emoji: "🛢", quality: "exact-analytic", preview: true,
      label: T("Cylinder", "Zylinder"),
      description: T("Creates a cylinder standing on the XY plane around the Z axis.", "Erzeugt einen Zylinder, der auf der XY-Ebene um die Z-Achse steht."),
      in: [I("radius"), I("height")],
      out: [solid("The cylinder as a closed solid.", "Der Zylinder als geschlossener Körper.")],
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.prim3d.cone", id: "brep.primitive.cone", emoji: "🔺", quality: "exact-analytic", preview: true,
      label: T("Cone", "Kegel"),
      description: T("Creates a cone with its base on the XY plane and its apex on the Z axis.", "Erzeugt einen Kegel mit der Grundfläche auf der XY-Ebene und der Spitze auf der Z-Achse."),
      in: [I("radius", { description: T("Radius of the base circle.", "Radius des Grundkreises.") }), I("height", { description: T("Distance from the base to the apex.", "Abstand von der Grundfläche zur Spitze.") })],
      out: [solid("The cone as a closed solid.", "Der Kegel als geschlossener Körper.")],
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.prim3d.torus", id: "brep.primitive.torus", emoji: "🍩", quality: "exact-analytic", preview: true,
      label: T("Torus", "Torus"),
      description: T("Creates a torus around the Z axis; the tube radius must stay below the ring radius.", "Erzeugt einen Torus um die Z-Achse; der Röhrenradius muss kleiner als der Ringradius bleiben."),
      in: [
        P("major", "length", T("Ring radius", "Ringradius"), T("Distance from the torus center to the center of the tube.", "Abstand vom Torusmittelpunkt zur Mitte der Röhre."), { default: 2, min: 0, exclusiveMin: true }),
        P("minor", "length", T("Tube radius", "Röhrenradius"), T("Radius of the tube itself.", "Radius der Röhre selbst."), { default: 0.5, min: 0, exclusiveMin: true }),
      ],
      out: [solid("The torus as a closed solid.", "Der Torus als geschlossener Körper.")],
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.prim3d.convexHull", id: "brep.primitive.convexHull", emoji: "🔷", quality: "exact-analytic", preview: true,
      label: T("Convex hull", "Konvexe Hülle"),
      description: T("Wraps the smallest convex solid around a set of points.", "Umhüllt eine Punktmenge mit dem kleinsten konvexen Körper."),
      in: [I("points", { minItems: 4, default: [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1]], description: T("The points to enclose; at least four, not all in one plane.", "Die einzuhüllenden Punkte; mindestens vier, nicht alle in einer Ebene.") })],
      out: [solid("The convex hull as a closed solid.", "Die konvexe Hülle als geschlossener Körper.")],
      ren: { out: { solid: "shape" } },
    }),
  ],
};
