import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const graph = await Bun.file(new URL("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🧫️fixtures/async-pointer.json", import.meta.url)).json();
const hover = await Bun.file(new URL("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🧫️fixtures/🔣️hover.json", import.meta.url)).json();
const db = new Database(":memory:");

test("SQLite resolves both companion addresses against their authored graph side", () => {
  const targets = [...graph.inputPublished, ...graph.published].map(({id}) => id);
  const rows = db.query(`
    WITH sides AS (
      SELECT 'in' direction, '$.inputs' column_path
      UNION ALL SELECT 'out', '$.outputs'
    ), physical AS (
      SELECT node.value ->> '$.id' widgetId, p.value ->> '$.id' address,
             p.value ->> '$.code' port, direction
      FROM json_each($nodes) node, sides, json_each(node.value, column_path) p
    )
    SELECT widgetId, port, direction
    FROM json_each($targets) targets
    JOIN physical ON physical.address = targets.value
    ORDER BY targets.key
  `).all({$nodes: JSON.stringify(graph.sceneNodes), $targets: JSON.stringify(targets)});
  expect(rows).toEqual([graph.inputTarget.channel, graph.nativeHover.channel]);
});

test("SQLite sees a companion-only change under a stationary primary port", () => {
  const law = hover.find(({id}) => id === "same primary with a different projected port group");
  const edits = db.query(`
    WITH frames AS (
      SELECT key, value -> '$.scene' scene, value -> '$.scene.portIds' port_ids
      FROM json_each($steps)
    ), observed AS (
      SELECT *, lag(scene) OVER (ORDER BY key) previous FROM frames
    )
    SELECT key, port_ids FROM observed WHERE scene IS NOT previous ORDER BY key
  `).all({$steps: JSON.stringify(law.steps)});
  expect(edits.map(({key, port_ids}) => ({key, group: port_ids ? JSON.parse(port_ids) : []}))).toEqual(
    law.steps.flatMap(({write}, key) => write ? [{key, group: write.portIds ?? []}] : []),
  );
});
