import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";
import { meshIn, meshOut, pickMany, pickOne } from "./🐍️mesh-common.mjs";

const grab = (component, port) => ({ pick: [pick("mesh", "mesh"), pick(component, port)] });
const whole = { pick: [pick("mesh", "mesh")] };
const fileIn = (en, de) => P("data", "text", T("Data", "Daten"), T(en, de), { default: "" });
const textOut = (name, label, en, de) => O(name, "text", label, T(en, de));

export const inspect = {
  category: {
    id: "mesh.inspect", emoji: "🔎", order: 260,
    label: T("Mesh inspection", "Netzinspektion"),
    description: T("Read positions, endpoints, normals and centers of single mesh components.", "Positionen, Endpunkte, Normalen und Mittelpunkte einzelner Netzelemente auslesen."),
  },
  kinds: [
    K("mesh.inspect", {
      was: "brep.mesh.inspectVertex", id: "mesh.inspect.vertex", emoji: "📍", quality: "polygon-mesh",
      label: T("Inspect vertex", "Eckpunkt untersuchen"),
      description: T("Reads the position of one vertex without changing the mesh.", "Liest die Position eines Eckpunkts, ohne das Netz zu ändern."),
      in: [meshIn(), pickOne("vertex", "vertex", "The vertex to read.", "Der auszulesende Eckpunkt.")],
      out: [O("position", "point", T("Position", "Position"), T("The position of the vertex.", "Die Position des Eckpunkts."))],
      interaction: grab("vertex", "vertex"),
      ren: { in: { index: "vertex (single selection)" }, out: { VertexPosition: "position" } },
    }),
    K("mesh.inspect", {
      was: "brep.mesh.inspectEdge", id: "mesh.inspect.edge", emoji: "📏", quality: "polygon-mesh",
      label: T("Inspect edge", "Kante untersuchen"),
      description: T("Reads the endpoints and length of one edge without changing the mesh.", "Liest Endpunkte und Länge einer Kante, ohne das Netz zu ändern."),
      in: [meshIn(), pickOne("edge", "edge", "The edge to read.", "Die auszulesende Kante.")],
      out: [
        O("start", "point", T("Start", "Start"), T("The first end of the edge.", "Das erste Ende der Kante.")),
        O("end", "point", T("End", "Ende"), T("The second end of the edge.", "Das zweite Ende der Kante.")),
        O("length", "length", T("Length", "Länge"), T("The length of the edge.", "Die Länge der Kante.")),
      ],
      interaction: grab("edge", "edge"),
      ren: { in: { index: "edge (single selection)" } },
    }),
    K("mesh.inspect", {
      was: "brep.mesh.inspectFace", id: "mesh.inspect.face", emoji: "⬜", quality: "polygon-mesh",
      label: T("Inspect face", "Fläche untersuchen"),
      description: T("Reads the corner indices, unit normal and center of one face without changing the mesh.", "Liest Eckindizes, Einheitsnormale und Mittelpunkt einer Fläche, ohne das Netz zu ändern."),
      in: [meshIn(), pickOne("face", "face", "The face to read.", "Die auszulesende Fläche.")],
      out: [
        O("vertices", "integer", T("Corner vertices", "Eckpunkte"), T("The vertex indices around the face.", "Die Eckpunktindizes rund um die Fläche."), { list: true }),
        O("normal", "vector", T("Normal", "Normale"), T("The unit normal of the face.", "Die Einheitsnormale der Fläche.")),
        O("center", "point", T("Center", "Mittelpunkt"), T("The mean of the corner positions.", "Der Mittelwert der Eckpositionen.")),
      ],
      interaction: grab("face", "face"),
      ren: { in: { index: "face (single selection)" }, out: { V: "vertices (text becomes an integer list)", FaceNormal: "normal", center: "center" } },
    }),
  ],
};

export const interchange = {
  category: {
    id: "mesh.interchange", emoji: "💾", order: 270,
    label: T("Mesh import and export", "Netz-Import und -Export"),
    description: T("Read and write meshes as OBJ, STL, GLB or editable JSON.", "Netze als OBJ, STL, GLB oder bearbeitbares JSON lesen und schreiben."),
  },
  kinds: [
    K("mesh.interchange", {
      was: "brep.mesh.exportObj", id: "mesh.interchange.exportObj", emoji: "🗒", quality: "polygon-mesh",
      label: T("Export OBJ", "OBJ exportieren"),
      description: T("Writes the mesh vertices and polygon faces as OBJ text.", "Schreibt Eckpunkte und Polygonflächen des Netzes als OBJ-Text."),
      in: [meshIn()],
      out: [textOut("text", T("OBJ", "OBJ"), "The OBJ file content.", "Der Inhalt der OBJ-Datei.")],
      interaction: whole,
    }),
    K("mesh.interchange", {
      was: "brep.mesh.exportJson", id: "mesh.interchange.exportJson", emoji: "🧾", quality: "polygon-mesh",
      label: T("Export JSON", "JSON exportieren"),
      description: T("Writes the editable indexed vertices and polygon faces as JSON text.", "Schreibt die bearbeitbaren indizierten Eckpunkte und Polygonflächen als JSON-Text."),
      in: [meshIn()],
      out: [textOut("text", T("JSON", "JSON"), "The mesh as JSON text.", "Das Netz als JSON-Text.")],
      interaction: whole,
    }),
    K("mesh.interchange", {
      id: "mesh.interchange.exportStl", emoji: "🧊", quality: "polygon-mesh",
      label: T("Export STL", "STL exportieren"),
      description: T("Writes the mesh as binary STL, encoded as base64 text.", "Schreibt das Netz als binäres STL, als Base64-Text kodiert."),
      in: [meshIn()],
      out: [textOut("stl", T("STL", "STL"), "The STL file content as base64 text.", "Der STL-Dateiinhalt als Base64-Text.")],
      interaction: whole,
    }),
    K("mesh.interchange", {
      id: "mesh.interchange.exportGlb", emoji: "📦", quality: "polygon-mesh",
      label: T("Export GLB", "GLB exportieren"),
      description: T("Writes the mesh as a binary glTF file, encoded as base64 text.", "Schreibt das Netz als binäre glTF-Datei, als Base64-Text kodiert."),
      in: [meshIn()],
      out: [textOut("glb", T("GLB", "GLB"), "The GLB file content as base64 text.", "Der GLB-Dateiinhalt als Base64-Text.")],
      interaction: whole,
    }),
    K("mesh.interchange", {
      id: "mesh.interchange.importObj", emoji: "🗂", quality: "polygon-mesh", preview: true,
      label: T("Import OBJ", "OBJ importieren"),
      description: T("Reads OBJ text into an editable polygon mesh.", "Liest OBJ-Text in ein bearbeitbares Polygonnetz."),
      in: [fileIn("The OBJ file content.", "Der Inhalt der OBJ-Datei.")],
      out: [meshOut("The imported mesh.", "Das importierte Netz.")],
    }),
    K("mesh.interchange", {
      id: "mesh.interchange.importStl", emoji: "📥", quality: "polygon-mesh", preview: true,
      label: T("Import STL", "STL importieren"),
      description: T("Reads base64 STL text into a polygon mesh.", "Liest Base64-STL-Text in ein Polygonnetz."),
      in: [fileIn("The STL file content as base64 text.", "Der STL-Dateiinhalt als Base64-Text.")],
      out: [meshOut("The imported mesh.", "Das importierte Netz.")],
    }),
    K("mesh.interchange", {
      id: "mesh.interchange.importGlb", emoji: "📂", quality: "polygon-mesh", preview: true,
      label: T("Import GLB", "GLB importieren"),
      description: T("Reads a base64 GLB file into a polygon mesh.", "Liest eine Base64-GLB-Datei in ein Polygonnetz."),
      in: [fileIn("The GLB file content as base64 text.", "Der GLB-Dateiinhalt als Base64-Text.")],
      out: [meshOut("The imported mesh.", "Das importierte Netz.")],
    }),
  ],
};

export const shading = {
  category: {
    id: "mesh.shading", emoji: "💡", order: 280,
    label: T("Mesh shading", "Netzschattierung"),
    description: T("Switch faces between flat and smooth shading and rebuild vertex normals.", "Flächen zwischen flacher und glatter Schattierung umschalten und Eckpunktnormalen neu berechnen."),
  },
  kinds: [
    K("mesh.shading", {
      id: "mesh.shading.setShading", emoji: "🌗", quality: "polygon-mesh", preview: true,
      label: T("Set shading", "Schattierung setzen"),
      description: T("Marks the selected faces as smooth or flat and rebuilds the normals.", "Markiert die ausgewählten Flächen als glatt oder flach und berechnet die Normalen neu."),
      in: [meshIn(), pickMany("faces", "face", "The faces to change.", "Die zu ändernden Flächen."), P("smooth", "boolean", T("Smooth", "Glatt"), T("Smooth shading on; off gives flat shading.", "Glatte Schattierung an; aus ergibt flache Schattierung."), { default: true })],
      out: [meshOut("The mesh with updated shading.", "Das Netz mit aktualisierter Schattierung.")],
      interaction: grab("face", "faces"),
    }),
    K("mesh.shading", {
      id: "mesh.shading.recomputeNormals", emoji: "🧭", quality: "polygon-mesh", preview: true,
      label: T("Recompute normals", "Normalen neu berechnen"),
      description: T("Rebuilds all vertex normals from the current faces and their smooth or flat marks.", "Berechnet alle Eckpunktnormalen aus den aktuellen Flächen und ihren Glatt-Flach-Markierungen neu."),
      in: [meshIn()],
      out: [meshOut("The mesh with fresh normals.", "Das Netz mit frischen Normalen.")],
      interaction: whole,
    }),
  ],
};

export const uv = {
  category: {
    id: "mesh.uv", emoji: "🗺", order: 290,
    label: T("Mesh UV mapping", "Netz-UV-Abwicklung"),
    description: T("Mark seams and unwrap the surface into texture coordinates.", "Nähte markieren und die Oberfläche in Texturkoordinaten abwickeln."),
  },
  kinds: [
    K("mesh.uv", {
      id: "mesh.uv.markSeams", emoji: "✂", quality: "polygon-mesh", preview: true,
      label: T("Mark UV seams", "UV-Nähte markieren"),
      description: T("Marks or clears the selected edges as seams along which the surface is cut open for unwrapping.", "Markiert die ausgewählten Kanten als Nähte, entlang derer die Oberfläche zum Abwickeln aufgeschnitten wird, oder hebt die Markierung auf."),
      in: [meshIn(), pickMany("edges", "edge", "The edges to mark.", "Die zu markierenden Kanten."), P("seam", "boolean", T("Seam", "Naht"), T("On marks the edges as seams; off clears them.", "An markiert die Kanten als Nähte; aus hebt sie auf."), { default: true })],
      out: [meshOut("The mesh with updated seams.", "Das Netz mit aktualisierten Nähten.")],
      interaction: grab("edge", "edges"),
    }),
    K("mesh.uv", {
      id: "mesh.uv.unwrap", emoji: "🧻", quality: "polygon-mesh", preview: true,
      label: T("Unwrap UVs", "UVs abwickeln"),
      description: T("Unfolds the surface along the seams into packed texture coordinates.", "Faltet die Oberfläche entlang der Nähte in gepackte Texturkoordinaten auf."),
      in: [meshIn()],
      out: [meshOut("The mesh with texture coordinates.", "Das Netz mit Texturkoordinaten.")],
      interaction: whole,
    }),
  ],
};
