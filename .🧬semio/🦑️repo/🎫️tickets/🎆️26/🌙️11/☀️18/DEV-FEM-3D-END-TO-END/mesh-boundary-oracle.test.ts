import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../../../../../../../✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🎬️scene/🧪️tests/🔬️unit/🧫️boundary.json";

test("indexed demo preserves the boundary represented by the old browser mesh", async () => {
  const capture = await Bun.file(new URL("🗑️generated/browser-node-edit-fields-nov21/boot-geometry.json", import.meta.url)).json();
  const [meshes] = JSON.parse(capture.geometry);
  const solid = JSON.parse(meshes).find(mesh => mesh.id === "solid-" + fixture.solid);
  const db = new Database(":memory:");
  try {
    const result = db.query("SELECT COUNT(*) AS expandedVertices, COUNT(DISTINCT vertex) AS boundaryVertices FROM (SELECT json_group_array(value) AS vertex FROM json_each(?) GROUP BY CAST(key AS INTEGER) / 3)").get(JSON.stringify(solid.data.positions));
    expect(result).toEqual({ expandedVertices: fixture.expandedVertices, boundaryVertices: fixture.boundaryVertices });
    expect(solid.data.indices.length / 3).toBe(fixture.faces);
  } finally {
    db.close();
  }
});
