import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const law = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🕹️interaction-selection-laws/👆️hover-targets.json").json();
const db = new Database(":memory:", { strict: true });
const route = db.query(`
  WITH action AS (SELECT json_patch(?, ?) AS args)
  SELECT CASE
    WHEN json_type(args, '$.targets') IS NOT NULL THEN
      CASE WHEN json_valid(json_extract(args, '$.targets')) THEN json_extract(args, '$.targets') END
    WHEN length(json_extract(args, '$.targetId')) > 0 AND length(json_extract(args, '$.granularityId')) > 0 THEN
      json_array(json_object('id', json_extract(args, '$.targetId'), 'granularity', json_extract(args, '$.granularityId')))
  END AS targets FROM action
`);
test("the raw authored handle fits while JSON escaping exceeds the UI text budget", () => {
  const row = law.cases[0];
  expect(Buffer.byteLength(row.args.targetId)).toBeLessThanOrEqual(law.maximumInteractionIdBytes);
  expect(Buffer.byteLength(JSON.stringify(row.expected))).toBeGreaterThan(law.maximumUiTextBytes);
});
for (const row of law.cases) test(row.name, () => {
  const base = { ...row.args };
  const override = row.name.includes("leave") ? { targets: base.targets } : {};
  if (row.name.includes("leave")) delete base.targets;
  const { targets } = route.get(JSON.stringify(base), JSON.stringify(override));
  expect(targets === null ? null : JSON.parse(targets)).toEqual(row.fault ? null : row.expected);
});
