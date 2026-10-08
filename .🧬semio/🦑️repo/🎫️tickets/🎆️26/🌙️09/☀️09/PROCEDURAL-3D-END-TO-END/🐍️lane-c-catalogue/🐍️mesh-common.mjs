import { T, I, O, P, sel } from "./🐍️lib.mjs";

export const meshIn = (description = T("The polygon mesh to edit.", "Das zu bearbeitende Polygonnetz.")) => P("mesh", "mesh", T("Mesh", "Netz"), description);
export const meshOut = (en, de) => O("mesh", "mesh", T("Mesh", "Netz"), T(en, de));

const COMPONENT_LABELS = {
  vertex: [T("Vertices", "Eckpunkte"), T("Vertex", "Eckpunkt")],
  edge: [T("Edges", "Kanten"), T("Edge", "Kante")],
  face: [T("Faces", "Flächen"), T("Face", "Fläche")],
};

export const pickMany = (name, component, en, de, def = [0], extra = {}) =>
  P(name, "selection", COMPONENT_LABELS[component][0], T(en, de), { ...sel(component, "mesh"), default: def, minItems: 1, ...extra });

export const pickOne = (name, component, en, de, def = [0]) =>
  P(name, "selection", COMPONENT_LABELS[component][1], T(en, de), { ...sel(component, "mesh", { multiple: false }), default: def, minItems: 1, maxItems: 1 });

export const modeOption = () => [
  { value: "vertex", label: T("Vertices", "Eckpunkte") },
  { value: "edge", label: T("Edges", "Kanten") },
  { value: "face", label: T("Faces", "Flächen") },
];

export const modePort = () => P("mode", "enum", T("Component type", "Elementtyp"), T("Which kind of mesh component the selection refers to.", "Auf welche Art von Netzelement sich die Auswahl bezieht."), { default: "vertex", options: modeOption() });

export const modeSelection = (en, de) => P("selection", "selection", T("Selection", "Auswahl"), T(en, de), { ...sel("mode", "mesh", { modeFrom: "mode" }), default: [0], minItems: 1 });

export const pivotPort = () =>
  P("pivot", "enum", T("Pivot", "Drehpunkt"), T("Whether the transformation is centered on the selection or on a chosen point.", "Ob die Transformation um die Auswahl oder um einen gewählten Punkt zentriert ist."), {
    default: "selection",
    options: [
      { value: "selection", label: T("Center of selection", "Mitte der Auswahl") },
      { value: "point", label: T("Chosen point", "Gewählter Punkt") },
    ],
  });

export const pivotPoint = () => I("center", { label: T("Pivot point", "Drehpunkt"), description: T("The fixed point, used when the pivot is set to a chosen point.", "Der feste Punkt, verwendet wenn als Drehpunkt ein gewählter Punkt eingestellt ist.") });

export const amount = (en, de, den, dde, def = 0.1) => P("amount", "length", T(en, de), T(den, dde), { default: def, min: 0, exclusiveMin: true });
