import { T, I, O, P, K, pick } from "./🐍️lib.mjs";

const C = "brep.interchange";
const grab = { pick: [pick("shape", "shape")] };
const fileIn = (en, de) => I("data", { description: T(en, de) });
const textOut = (name, label, en, de) => O(name, "text", label, T(en, de));
const imported = (en, de) => O("shape", "shape", T("Shape", "Form"), T(en, de));

export default {
  category: {
    id: C, emoji: "💾", order: 110,
    label: T("Import and export", "Import und Export"),
    description: T("Exchange B-Rep geometry with STEP, STL, OBJ and DWG files.", "B-Rep-Geometrie über STEP-, STL-, OBJ- und DWG-Dateien austauschen."),
  },
  kinds: [
    K(C, {
      was: "brep.io.exportStep", id: "brep.interchange.exportStep", emoji: "💾", quality: "exact-analytic",
      label: T("Export STEP", "STEP exportieren"),
      description: T("Writes a shape as STEP text with exact curved geometry.", "Schreibt eine Form als STEP-Text mit exakter gekrümmter Geometrie."),
      in: [I("shape")],
      out: [textOut("step", T("STEP", "STEP"), "The STEP file content.", "Der Inhalt der STEP-Datei.")],
      interaction: grab,
      ren: { in: { geometry: "shape" } },
    }),
    K(C, {
      was: "brep.io.importStep", id: "brep.interchange.importStep", emoji: "📂", quality: "exact-analytic", preview: true,
      label: T("Import STEP", "STEP importieren"),
      description: T("Reads the first solid of STEP text; only planes, quadrics, B-splines and closed shells are supported.", "Liest den ersten Körper aus STEP-Text; unterstützt werden nur Ebenen, Quadriken, B-Splines und geschlossene Schalen."),
      in: [fileIn("The STEP file content.", "Der Inhalt der STEP-Datei.")],
      out: [imported("The imported shape.", "Die importierte Form.")],
      ren: { out: { geometry: "shape" } },
    }),
    K(C, {
      was: "brep.io.exportStl", id: "brep.interchange.exportStl", emoji: "🧊", quality: "mesh-derived-brep",
      label: T("Export STL", "STL exportieren"),
      description: T("Triangulates a shape and writes it as binary STL, encoded as base64 text.", "Trianguliert eine Form und schreibt sie als binäres STL, als Base64-Text kodiert."),
      in: [I("shape"), I("deflection")],
      out: [textOut("stl", T("STL", "STL"), "The STL file content as base64 text.", "Der STL-Dateiinhalt als Base64-Text.")],
      interaction: grab,
      ren: { in: { geometry: "shape" } },
    }),
    K(C, {
      was: "brep.io.importStl", id: "brep.interchange.importStl", emoji: "📥", quality: "mesh-derived-brep", preview: true,
      label: T("Import STL", "STL importieren"),
      description: T("Reads base64 STL text into a faceted solid with one planar face per triangle.", "Liest Base64-STL-Text in einen facettierten Körper mit einer ebenen Fläche je Dreieck."),
      in: [fileIn("The STL file content as base64 text.", "Der STL-Dateiinhalt als Base64-Text."), I("tolerance", { default: 0.1, description: T("Distance within which vertices are merged.", "Abstand, innerhalb dessen Eckpunkte verschmolzen werden.") })],
      out: [imported("The faceted solid.", "Der facettierte Körper.")],
      ren: { out: { geometry: "shape" } },
    }),
    K(C, {
      was: "brep.io.exportObj", id: "brep.interchange.exportObj", emoji: "🗒", quality: "mesh-derived-brep",
      label: T("Export OBJ", "OBJ exportieren"),
      description: T("Triangulates a shape and writes it as OBJ text.", "Trianguliert eine Form und schreibt sie als OBJ-Text."),
      in: [I("shape"), I("deflection")],
      out: [textOut("obj", T("OBJ", "OBJ"), "The OBJ file content.", "Der Inhalt der OBJ-Datei.")],
      interaction: grab,
      ren: { in: { geometry: "shape" } },
    }),
    K(C, {
      was: "brep.io.importObj", id: "brep.interchange.importObj", emoji: "🗂", quality: "mesh-derived-brep", preview: true,
      label: T("Import OBJ", "OBJ importieren"),
      description: T("Reads OBJ text into a faceted solid with one planar face per triangle.", "Liest OBJ-Text in einen facettierten Körper mit einer ebenen Fläche je Dreieck."),
      in: [fileIn("The OBJ file content.", "Der Inhalt der OBJ-Datei."), I("tolerance", { default: 0.1, description: T("Distance within which vertices are merged.", "Abstand, innerhalb dessen Eckpunkte verschmolzen werden.") })],
      out: [imported("The faceted solid.", "Der facettierte Körper.")],
      ren: { out: { geometry: "shape" } },
    }),
    K(C, {
      was: "brep.io.exportDwg", id: "brep.interchange.exportDwg", emoji: "📐", quality: "mesh-derived-brep",
      label: T("Export DWG", "DWG exportieren"),
      description: T("Triangulates a shape and writes it as a DWG triangle bridge, encoded as base64 text.", "Trianguliert eine Form und schreibt sie als DWG-Dreiecksbrücke, als Base64-Text kodiert."),
      in: [I("shape"), I("deflection")],
      out: [textOut("dwg", T("DWG", "DWG"), "The DWG file content as base64 text.", "Der DWG-Dateiinhalt als Base64-Text.")],
      interaction: grab,
      ren: { in: { geometry: "shape" } },
    }),
    K(C, {
      was: "brep.io.importDwg", id: "brep.interchange.importDwg", emoji: "📏", quality: "mesh-derived-brep", preview: true,
      label: T("Import DWG", "DWG importieren"),
      description: T("Reads a base64 DWG triangle bridge into a faceted solid.", "Liest eine Base64-DWG-Dreiecksbrücke in einen facettierten Körper."),
      in: [fileIn("The DWG file content as base64 text.", "Der DWG-Dateiinhalt als Base64-Text."), I("tolerance", { default: 0.1, description: T("Distance within which vertices are merged.", "Abstand, innerhalb dessen Eckpunkte verschmolzen werden.") })],
      out: [imported("The faceted solid.", "Der facettierte Körper.")],
      ren: { out: { geometry: "shape" } },
    }),
  ],
};
