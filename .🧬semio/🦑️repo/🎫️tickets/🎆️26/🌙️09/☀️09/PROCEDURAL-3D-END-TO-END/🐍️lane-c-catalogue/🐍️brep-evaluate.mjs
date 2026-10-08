import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const C = "brep.evaluate";
const onCurve = { pick: [pick("edge", "curve"), pick("shape", "curve")] };
const onSurface = { pick: [pick("face", "surface"), pick("shape", "surface")] };
const pointIn = (en, de) => I("point", { description: T(en, de) });
const distanceOut = () => O("distance", "length", T("Distance", "Abstand"), T("The achieved distance between the input point and the closest point.", "Der erreichte Abstand zwischen dem Eingabepunkt und dem nächsten Punkt."));

export default {
  category: {
    id: C, emoji: "🎯", order: 90,
    label: T("Evaluate curves and surfaces", "Kurven und Flächen auswerten"),
    description: T("Read points, tangents, normals, curvature and closest positions from curves and surfaces.", "Punkte, Tangenten, Normalen, Krümmung und nächste Positionen von Kurven und Flächen auslesen."),
  },
  kinds: [
    K(C, {
      was: "brep.eval.curvePoint", id: "brep.evaluate.curvePoint", emoji: "📍", quality: "exact-analytic",
      label: T("Point on curve", "Punkt auf Kurve"),
      description: T("Evaluates the position on a curve at a parameter.", "Berechnet die Position auf einer Kurve an einem Parameter."),
      in: [I("curve"), I("parameter")],
      out: [O("point", "point", T("Point", "Punkt"), T("The position on the curve.", "Die Position auf der Kurve."))],
      interaction: onCurve,
    }),
    K(C, {
      was: "brep.eval.curveTangent", id: "brep.evaluate.curveTangent", emoji: "➡", quality: "exact-analytic",
      label: T("Curve tangent", "Kurventangente"),
      description: T("Evaluates the tangent direction of a curve at a parameter.", "Berechnet die Tangentenrichtung einer Kurve an einem Parameter."),
      in: [I("curve"), I("parameter")],
      out: [O("tangent", "vector", T("Tangent", "Tangente"), T("The direction of the curve at the parameter.", "Die Richtung der Kurve am Parameter."))],
      interaction: onCurve,
    }),
    K(C, {
      was: "brep.eval.curveDomain", id: "brep.evaluate.curveDomain", emoji: "↔", quality: "exact-analytic",
      label: T("Curve domain", "Kurvenbereich"),
      description: T("Reads the parameter range over which a curve is defined.", "Liest den Parameterbereich aus, über dem eine Kurve definiert ist."),
      in: [I("curve")],
      out: [
        O("start", "number", T("Start", "Anfang"), T("Parameter at the start of the curve.", "Parameter am Anfang der Kurve.")),
        O("end", "number", T("End", "Ende"), T("Parameter at the end of the curve.", "Parameter am Ende der Kurve.")),
        O("span", "number", T("Span", "Spannweite"), T("Length of the parameter range.", "Länge des Parameterbereichs.")),
      ],
      interaction: onCurve,
      ren: { out: { span: "span (start and end are new)" } },
    }),
    K(C, {
      was: "brep.eval.curveCurvature", id: "brep.evaluate.curveCurvature", emoji: "🌊", quality: "exact-analytic",
      label: T("Curve curvature", "Kurvenkrümmung"),
      description: T("Evaluates how sharply a curve bends at a parameter.", "Berechnet, wie stark sich eine Kurve an einem Parameter krümmt."),
      in: [I("curve"), I("parameter")],
      out: [O("curvature", "number", T("Curvature", "Krümmung"), T("Reciprocal of the bending radius; zero on straight pieces.", "Kehrwert des Krümmungsradius; null auf geraden Stücken."))],
      interaction: onCurve,
    }),
    K(C, {
      was: "brep.eval.surfPoint", id: "brep.evaluate.surfacePoint", emoji: "📌", quality: "exact-analytic",
      label: T("Point on surface", "Punkt auf Fläche"),
      description: T("Evaluates the position on a surface at a pair of parameters.", "Berechnet die Position auf einer Fläche an einem Parameterpaar."),
      in: [I("surface"), I("u"), I("v")],
      out: [O("point", "point", T("Point", "Punkt"), T("The position on the surface.", "Die Position auf der Fläche."))],
      interaction: onSurface,
    }),
    K(C, {
      was: "brep.eval.surfNormal", id: "brep.evaluate.surfaceNormal", emoji: "🧭", quality: "exact-analytic",
      label: T("Surface normal", "Flächennormale"),
      description: T("Evaluates the unit normal of a surface at a pair of parameters.", "Berechnet die Einheitsnormale einer Fläche an einem Parameterpaar."),
      in: [I("surface"), I("u"), I("v")],
      out: [O("normal", "vector", T("Normal", "Normale"), T("The unit vector perpendicular to the surface.", "Der Einheitsvektor senkrecht zur Fläche."))],
      interaction: onSurface,
    }),
    K(C, {
      was: "brep.eval.curveClosestParameter", id: "brep.evaluate.curveClosestParameter", emoji: "🎯", quality: "exact-numerical",
      label: T("Closest parameter on curve", "Nächster Kurvenparameter"),
      description: T("Finds the parameter on a curve that is closest to a point.", "Findet den Parameter auf einer Kurve, der einem Punkt am nächsten liegt."),
      in: [I("curve"), pointIn("The point to measure from.", "Der Punkt, von dem aus gemessen wird.")],
      out: [
        O("parameter", "number", T("Parameter", "Parameter"), T("The curve parameter of the closest position.", "Der Kurvenparameter der nächsten Position.")),
        O("point", "point", T("Closest point", "Nächster Punkt"), T("The closest position on the curve.", "Die nächste Position auf der Kurve.")),
        distanceOut(),
      ],
      interaction: onCurve,
      ren: { out: { pointOut: "point" } },
    }),
    K(C, {
      was: "brep.eval.surfaceClosestUv", id: "brep.evaluate.surfaceClosestUv", emoji: "🧲", quality: "exact-numerical",
      label: T("Closest parameters on surface", "Nächste Flächenparameter"),
      description: T("Finds the surface parameters (u, v) that are closest to a point.", "Findet die Flächenparameter (u, v), die einem Punkt am nächsten liegen."),
      in: [I("surface"), pointIn("The point to measure from.", "Der Punkt, von dem aus gemessen wird.")],
      out: [
        O("u", "number", T("U", "U"), T("First parameter of the closest position.", "Erster Parameter der nächsten Position.")),
        O("v", "number", T("V", "V"), T("Second parameter of the closest position.", "Zweiter Parameter der nächsten Position.")),
        O("point", "point", T("Closest point", "Nächster Punkt"), T("The closest position on the surface.", "Die nächste Position auf der Fläche.")),
        distanceOut(),
      ],
      interaction: onSurface,
      ren: { out: { pointOut: "point" } },
    }),
  ],
};
