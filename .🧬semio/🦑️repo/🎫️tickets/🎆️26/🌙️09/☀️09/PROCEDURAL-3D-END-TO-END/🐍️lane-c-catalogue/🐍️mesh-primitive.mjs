import { T, I, O, P, K } from "./🐍️lib.mjs";
import { meshOut } from "./🐍️mesh-common.mjs";

const C = "mesh.primitive";
const size = (name, en, de, den, dde) => P(name, "length", T(en, de), T(den, dde), { default: 1, min: 0, exclusiveMin: true });
const seg = (name, en, de, den, dde, def, min) => P(name, "integer", T(en, de), T(den, dde), { default: def, min, max: 1024 });

export default {
  category: {
    id: C, emoji: "🥽", order: 200,
    label: T("Mesh primitives", "Netz-Grundformen"),
    description: T("Create polygon meshes from numbers: boxes, planes, spheres, cylinders, cones, tori or raw vertex and face data.", "Polygonnetze aus Zahlen erzeugen: Quader, Ebenen, Kugeln, Zylinder, Kegel, Tori oder rohe Eckpunkt- und Flächendaten."),
  },
  kinds: [
    K(C, {
      was: "brep.mesh.construct", id: "mesh.primitive.construct", emoji: "🧮", quality: "polygon-mesh", preview: true,
      label: T("Mesh from data", "Netz aus Daten"),
      description: T("Builds a mesh from JSON text with vertex positions and zero-based face indices.", "Baut ein Netz aus JSON-Text mit Eckpunktpositionen und nullbasierten Flächenindizes."),
      in: [P("data", "text", T("Mesh data", "Netzdaten"), T("JSON with a \"vertices\" list of positions and a \"faces\" list of index lists.", "JSON mit einer Liste \"vertices\" aus Positionen und einer Liste \"faces\" aus Indexlisten."), { default: "{\"vertices\":[[0,0,0],[1,0,0],[0,1,0]],\"faces\":[[0,1,2]]}" })],
      out: [meshOut("The constructed mesh.", "Das erzeugte Netz.")],
      ren: { out: { meshOut: "mesh" } },
    }),
    K(C, {
      was: "brep.mesh.box", id: "mesh.primitive.box", emoji: "📦", quality: "polygon-mesh", preview: true,
      label: T("Mesh box", "Netz-Quader"),
      description: T("Creates a closed box with six quad faces.", "Erzeugt einen geschlossenen Quader mit sechs Vierecksflächen."),
      in: [size("width", "Width", "Breite", "First box dimension.", "Erste Abmessung des Quaders."), size("height", "Height", "Höhe", "Second box dimension.", "Zweite Abmessung des Quaders."), size("depth", "Depth", "Tiefe", "Third box dimension.", "Dritte Abmessung des Quaders.")],
      out: [meshOut("The box mesh.", "Das Quadernetz.")],
      ren: { out: { meshOut: "mesh" } },
    }),
    K(C, {
      was: "brep.mesh.plane", id: "mesh.primitive.plane", emoji: "⬜", quality: "polygon-mesh", preview: true,
      label: T("Mesh plane", "Netz-Ebene"),
      description: T("Creates one open quad in the horizontal plane.", "Erzeugt ein offenes Viereck in der horizontalen Ebene."),
      in: [size("width", "Width", "Breite", "Size in the first direction.", "Ausdehnung in der ersten Richtung."), size("depth", "Depth", "Tiefe", "Size in the second direction.", "Ausdehnung in der zweiten Richtung.")],
      out: [meshOut("The plane mesh.", "Das Ebenennetz.")],
      ren: { out: { meshOut: "mesh" } },
    }),
    K(C, {
      was: "brep.mesh.sphere", id: "mesh.primitive.sphere", emoji: "⚪", quality: "polygon-mesh", preview: true,
      label: T("Mesh icosphere", "Netz-Ikosphäre"),
      description: T("Creates a closed triangle sphere by subdividing an icosahedron.", "Erzeugt eine geschlossene Dreieckskugel durch Unterteilen eines Ikosaeders."),
      in: [I("radius"), P("subdivisions", "integer", T("Subdivisions", "Unterteilungen"), T("Each step quadruples the number of triangles; 0 to 5.", "Jeder Schritt vervierfacht die Dreieckszahl; 0 bis 5."), { default: 2, min: 0, max: 5 })],
      out: [meshOut("The sphere mesh.", "Das Kugelnetz.")],
      ren: { out: { meshOut: "mesh" } },
    }),
    K(C, {
      was: "brep.mesh.cylinder", id: "mesh.primitive.cylinder", emoji: "🛢", quality: "polygon-mesh", preview: true,
      label: T("Mesh cylinder", "Netz-Zylinder"),
      description: T("Creates a closed cylinder with polygon caps and quad sides.", "Erzeugt einen geschlossenen Zylinder mit Vieleckdeckeln und Vierecksseiten."),
      in: [I("radius"), I("height"), seg("segments", "Segments", "Segmente", "Number of sides around the axis.", "Anzahl der Seiten um die Achse.", 32, 3)],
      out: [meshOut("The cylinder mesh.", "Das Zylindernetz.")],
      ren: { out: { meshOut: "mesh" } },
    }),
    K(C, {
      was: "brep.mesh.cone", id: "mesh.primitive.cone", emoji: "🔺", quality: "polygon-mesh", preview: true,
      label: T("Mesh cone", "Netz-Kegel"),
      description: T("Creates a closed cone with a polygon base and triangle sides.", "Erzeugt einen geschlossenen Kegel mit Vieleckgrundfläche und Dreiecksseiten."),
      in: [I("radius", { description: T("Radius of the base.", "Radius der Grundfläche.") }), I("height", { description: T("Distance from the base to the apex.", "Abstand von der Grundfläche zur Spitze.") }), seg("segments", "Segments", "Segmente", "Number of sides around the axis.", "Anzahl der Seiten um die Achse.", 32, 3)],
      out: [meshOut("The cone mesh.", "Das Kegelnetz.")],
      ren: { out: { meshOut: "mesh" } },
    }),
    K(C, {
      id: "mesh.primitive.torus", emoji: "🍩", quality: "polygon-mesh", preview: true,
      label: T("Mesh torus", "Netz-Torus"),
      description: T("Creates a closed torus ring from quads.", "Erzeugt einen geschlossenen Torusring aus Vierecken."),
      in: [
        P("major", "length", T("Ring radius", "Ringradius"), T("Distance from the center to the middle of the tube.", "Abstand vom Mittelpunkt zur Mitte der Röhre."), { default: 2, min: 0, exclusiveMin: true }),
        P("minor", "length", T("Tube radius", "Röhrenradius"), T("Radius of the tube; keep it below the ring radius.", "Radius der Röhre; kleiner als der Ringradius halten."), { default: 0.5, min: 0, exclusiveMin: true }),
        seg("segments", "Ring segments", "Ringsegmente", "Number of segments around the ring.", "Anzahl der Segmente um den Ring.", 32, 3),
        seg("rings", "Tube segments", "Röhrensegmente", "Number of segments around the tube.", "Anzahl der Segmente um die Röhre.", 16, 3),
      ],
      out: [meshOut("The torus mesh.", "Das Torusnetz.")],
    }),
    K(C, {
      id: "mesh.primitive.uvSphere", emoji: "🌐", quality: "polygon-mesh", preview: true,
      label: T("Mesh UV sphere", "Netz-UV-Kugel"),
      description: T("Creates a closed sphere from latitude rings and longitude segments.", "Erzeugt eine geschlossene Kugel aus Breitenringen und Längensegmenten."),
      in: [I("radius"), seg("segments", "Segments", "Segmente", "Number of meridians around the poles.", "Anzahl der Meridiane um die Pole.", 32, 3), seg("rings", "Rings", "Ringe", "Number of latitude rings from pole to pole.", "Anzahl der Breitenringe von Pol zu Pol.", 16, 2)],
      out: [meshOut("The sphere mesh.", "Das Kugelnetz.")],
    }),
  ],
};
