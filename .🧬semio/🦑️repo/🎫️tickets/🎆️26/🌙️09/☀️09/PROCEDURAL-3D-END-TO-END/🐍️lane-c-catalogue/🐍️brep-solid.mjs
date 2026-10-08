import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const C = "brep.solid";
const PI = Math.PI;
const solid = (en, de) => O("shape", "shape", T("Solid", "Körper"), T(en, de), { shapeKinds: ["solid"] });

export default {
  category: {
    id: C, emoji: "🏗", order: 40,
    label: T("Solids from profiles", "Körper aus Profilen"),
    description: T("Turns faces, wires and paths into solids by extruding, revolving, sweeping, lofting and piping.", "Macht aus Flächen, Kantenzügen und Pfaden Körper durch Extrudieren, Rotieren, Austragen, Ausformen und Rohrbildung."),
  },
  kinds: [
    K(C, {
      was: "brep.solid.extrude", id: "brep.solid.extrudeWire", emoji: "🧱", quality: "exact-analytic", preview: true,
      label: T("Extrude wire", "Kantenzug extrudieren"),
      description: T("Extrudes a closed wire in a plane parallel to the XY plane into a solid along a vector.", "Extrudiert einen geschlossenen Kantenzug in einer zur XY-Ebene parallelen Ebene entlang eines Vektors zu einem Körper."),
      in: [I("wire", { description: T("The closed wire to extrude.", "Der zu extrudierende geschlossene Kantenzug.") }), I("offset", { as: "vector", default: [0, 0, 1], label: T("Extrusion vector", "Extrusionsvektor"), description: T("Direction and length of the extrusion.", "Richtung und Länge der Extrusion.") })],
      out: [solid("The extruded solid.", "Der extrudierte Körper.")],
      interaction: { pick: [pick("shape", "wire")], gumball: [gumball("translate", "vector")] },
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.sweep.extrude", id: "brep.solid.extrudeFace", emoji: "⬆", quality: "exact-analytic", preview: true,
      label: T("Extrude face", "Fläche extrudieren"),
      description: T("Extrudes a face along a vector into a solid; the length of the vector is the extrusion distance.", "Extrudiert eine Fläche entlang eines Vektors zu einem Körper; die Länge des Vektors ist die Extrusionslänge."),
      in: [I("face", { description: T("The face to extrude.", "Die zu extrudierende Fläche.") }), I("offset", { as: "vector", default: [0, 0, 1], label: T("Extrusion vector", "Extrusionsvektor"), description: T("Direction and length of the extrusion.", "Richtung und Länge der Extrusion.") })],
      out: [solid("The extruded solid.", "Der extrudierte Körper.")],
      interaction: { pick: [pick("face", "face")], gumball: [gumball("translate", "vector")] },
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.sweep.revolve", id: "brep.solid.revolve", emoji: "🔄", quality: "exact-analytic", preview: true,
      label: T("Revolve", "Rotieren"),
      description: T("Revolves a face around an axis into a solid of revolution.", "Rotiert eine Fläche um eine Achse zu einem Rotationskörper."),
      in: [
        I("face", { description: T("The profile face; it must not cross the axis.", "Die Profilfläche; sie darf die Achse nicht schneiden.") }),
        I("axisOrigin"), I("axisDirection"),
        I("angle", { default: 2 * PI, min: 0, exclusiveMin: true, max: 2 * PI, description: T("Revolution angle; a full turn is 360 degrees.", "Rotationswinkel; eine volle Drehung sind 360 Grad.") }),
      ],
      out: [solid("The solid of revolution.", "Der Rotationskörper.")],
      interaction: { pick: [pick("face", "face")], gumball: [gumball("rotate", "angle", { axisPort: "axisDirection", originPort: "axisOrigin" })] },
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.sweep.loft", id: "brep.solid.loft", emoji: "🌉", quality: "exact-analytic", preview: true,
      label: T("Loft", "Ausformen (Loft)"),
      description: T("Blends a solid through a sequence of profiles.", "Formt einen Körper durch eine Folge von Profilen aus."),
      in: [
        P("profiles", "shapes", T("Profiles", "Profile"), T("The profiles in order; at least two, with compatible curve structure.", "Die Profile in Reihenfolge; mindestens zwei, mit kompatibler Kurvenstruktur."), { shapeKinds: ["wire", "face"], minItems: 2 }),
        P("smooth", "boolean", T("Smooth", "Glatt"), T("Blend smoothly through the profiles instead of straight between them.", "Glatt durch die Profile überblenden statt geradlinig dazwischen."), { default: false }),
      ],
      out: [solid("The lofted solid.", "Der ausgeformte Körper.")],
      ren: { out: { solid: "shape" } },
      note: "smooth was a number (0 or 1), now a boolean",
    }),
    K(C, {
      was: "brep.sweep.sweep", id: "brep.solid.sweep", emoji: "🛤", quality: "exact-numerical", preview: true,
      label: T("Sweep", "Austragen (Sweep)"),
      description: T("Moves a profile along a path to create a solid.", "Bewegt ein Profil entlang eines Pfads und erzeugt so einen Körper."),
      in: [I("profile"), I("path")],
      out: [solid("The swept solid.", "Der ausgetragene Körper.")],
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.sweep.pipe", id: "brep.solid.pipe", emoji: "🚰", quality: "exact-numerical", preview: true,
      label: T("Pipe", "Rohr"),
      description: T("Sweeps a profile along a path, optionally steered by a guide curve.", "Trägt ein Profil entlang eines Pfads aus, optional geführt von einer Leitkurve."),
      in: [I("profile"), I("path"), I("path", { as: "guide", optional: true, label: T("Guide", "Leitkurve"), description: T("Optional curve that controls the twist of the profile.", "Optionale Kurve, die die Verdrehung des Profils steuert.") })],
      out: [solid("The piped solid.", "Der Rohrkörper.")],
      ren: { out: { solid: "shape" } },
    }),
    K(C, {
      was: "brep.sweep.helical", id: "brep.solid.helicalSweep", emoji: "🌀", quality: "exact-numerical", preview: true,
      label: T("Helical sweep", "Schraubenförmig austragen"),
      description: T("Sweeps a profile along a helix around an axis, for springs and threads.", "Trägt ein Profil entlang einer Schraubenlinie um eine Achse aus, etwa für Federn und Gewinde."),
      in: [
        I("profile"), I("axisOrigin"), I("axisDirection"),
        I("radius", { description: T("Radius of the helix.", "Radius der Schraubenlinie.") }),
        P("pitch", "length", T("Pitch", "Steigung"), T("Distance travelled along the axis per full turn.", "Weg entlang der Achse pro voller Windung."), { default: 1, min: 0, exclusiveMin: true }),
        P("turns", "number", T("Turns", "Windungen"), T("Number of full turns; fractions are allowed.", "Anzahl voller Windungen; Bruchteile sind erlaubt."), { default: 1, min: 0, exclusiveMin: true, step: 0.25 }),
      ],
      out: [solid("The helical solid.", "Der schraubenförmige Körper.")],
      ren: { out: { solid: "shape" } },
    }),
  ],
};
