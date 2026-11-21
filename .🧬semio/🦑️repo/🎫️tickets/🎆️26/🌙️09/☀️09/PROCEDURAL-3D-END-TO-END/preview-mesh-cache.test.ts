import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/preview-mesh-cache.json").json();
const db = new Database(":memory:", { strict: true });
const rows = db.query(`
WITH steps AS (
  SELECT CAST(key AS INTEGER) position, value data, json_extract(value, '$.session') session
  FROM json_each($steps)
),
authority AS (
  SELECT step.position, step.data, session.value recipe,
   (SELECT COUNT(*) FROM steps prior WHERE prior.position <= step.position AND prior.session = step.session
     AND json_type(prior.data, '$.replace') IS NOT NULL) changes,
   (SELECT json_extract(prior.data, '$.replace[0]') FROM steps prior
     WHERE prior.position <= step.position AND prior.session = step.session AND json_type(prior.data, '$.replace') IS NOT NULL
     ORDER BY prior.position DESC LIMIT 1) replacement
  FROM steps step JOIN json_each($sessions) session ON step.session = session.key
),
table_authority AS (
  SELECT position, data,
    CASE
      WHEN json_extract(data, '$.hidden') THEN NULL
      WHEN json_extract(data, '$.fallback') THEN
        CASE WHEN NOT COALESCE(json_extract(data, '$.current'), 0) THEN json_extract(recipe, '$.meshes."solid-fallback"[0]') END
      ELSE COALESCE(replacement, json_extract(recipe, '$.meshes."solid-work"[0]'))
    END as x,
    json_array(json_extract(recipe, '$.owner'), json_extract(recipe, '$.epoch') + changes,
      COALESCE(json_extract(data, '$.mode'), 'solid'), COALESCE(json_extract(data, '$.detail'), 0.05),
      COALESCE(json_extract(data, '$.fallback'), 0), COALESCE(json_extract(data, '$.current'), 0),
      COALESCE(json_extract(data, '$.hidden'), 0)) AS identity
  FROM authority
),
requests AS (SELECT *, LAG(identity) OVER (ORDER BY position) AS previous FROM table_authority)
SELECT position, x, SUM(CASE WHEN x IS NOT NULL AND identity IS NOT previous THEN 1 ELSE 0 END)
  OVER (ORDER BY position) AS conversions
FROM requests ORDER BY position
`);

test("preview mesh reuse changes only with its exact source, display and residency authority", () => {
  const checks = rows.all({steps: JSON.stringify(fixture.steps), sessions: JSON.stringify(fixture.sessions)});
  expect(checks).toHaveLength(fixture.steps.length);
  for (const { position, x, conversions } of checks) {
    const expected = fixture.steps[position];
    expect(x).toEqual(expected.position ?? null);
    expect(conversions).toEqual(expected.conversions);
  }
});
