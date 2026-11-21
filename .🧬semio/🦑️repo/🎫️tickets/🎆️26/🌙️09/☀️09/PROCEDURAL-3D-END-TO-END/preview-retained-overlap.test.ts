import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/preview-retained-overlap.json").json();
const database = new Database(":memory:", { strict: true });

test("exactly coincident retained surfaces share a hover group and keep their own identities", () => {
  const source = fixture.evaluation[fixture.widget].out;
  const groups = database.query(`
    WITH geometries AS (SELECT key, json(value) AS surface FROM json_each(?)),
         channels AS (SELECT key, json_extract(value, '$.handle') AS handle FROM json_each(?))
    SELECT json_group_array(? || channels.key ORDER BY channels.key) AS group_json
    FROM channels JOIN geometries ON channels.handle = geometries.key
    GROUP BY geometries.surface HAVING COUNT(*) > 1
  `).all(JSON.stringify(fixture.meshes), JSON.stringify(source), "h:" + fixture.widget + "@");
  expect(groups.map(({ group_json }) => JSON.parse(group_json))).toEqual([fixture.expected.group]);
  expect(Object.keys(fixture.meshes).length).toBe(fixture.expected.meshes);
  expect(Object.values(source).length).toBe(fixture.expected.instances);
});
