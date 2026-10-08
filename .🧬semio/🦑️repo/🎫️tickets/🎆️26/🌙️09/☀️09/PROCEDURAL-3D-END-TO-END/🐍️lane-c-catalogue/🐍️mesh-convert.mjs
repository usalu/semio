import { T, I, O, P, K, pick } from "./🐍️lib.mjs";
import { meshIn, meshOut } from "./🐍️mesh-common.mjs";

const C = "mesh.convert";

export default {
  category: {
    id: C, emoji: "🔀", order: 210,
    label: T("Mesh conversion", "Netz-Umwandlung"),
    description: T("Convert between exact B-Rep shapes and polygon meshes.", "Zwischen exakten B-Rep-Formen und Polygonnetzen umwandeln."),
  },
  kinds: [
    K(C, {
      was: "brep.mesh.fromBrep", id: "mesh.convert.fromBrep", emoji: "🔺", quality: "tessellated-mesh", preview: true,
      label: T("B-Rep to mesh", "B-Rep in Netz"),
      description: T("Triangulates a B-Rep shape into a mesh; a smaller deflection gives more detail.", "Trianguliert eine B-Rep-Form zu einem Netz; eine kleinere Abweichung ergibt mehr Details."),
      in: [I("shape"), I("deflection")],
      out: [meshOut("The triangulated mesh.", "Das triangulierte Netz.")],
      interaction: { pick: [pick("shape", "shape")] },
      ren: { in: { geometry: "shape" }, out: { meshOut: "mesh" } },
    }),
    K(C, {
      was: "brep.mesh.toBrep", id: "mesh.convert.toBrep", emoji: "🧊", quality: "mesh-derived-brep", preview: true,
      label: T("Mesh to B-Rep", "Netz in B-Rep"),
      description: T("Turns every polygon into a planar B-Rep face; curved surfaces are not reconstructed.", "Macht aus jedem Polygon eine ebene B-Rep-Fläche; gekrümmte Flächen werden nicht rekonstruiert."),
      in: [meshIn(), I("tolerance", { default: 0.001, description: T("Distance within which vertices are merged.", "Abstand, innerhalb dessen Eckpunkte verschmolzen werden.") })],
      out: [O("shape", "shape", T("Shape", "Form"), T("The faceted B-Rep shape.", "Die facettierte B-Rep-Form."))],
      ren: { out: { geometry: "shape" } },
    }),
  ],
};
