import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { admittedOptimizeSchema, matchesTranspileKey } from "./verify-browser-core-lineage.ts";
import fixture from "./🔣️browser-core-lineage-keys.json";

const database = new Database(":memory:", { strict: true });
const compare = database.query<{ accepted: number }, { key: string; component: string; base: string }>(`
  SELECT (
    json_extract(:key, '$.stage') = 'transpile'
    AND (SELECT COUNT(*) FROM json_each(:key, '$.inputs')) = 1
    AND json_extract(:key, '$.inputs.component') = :component
    AND json_extract(:key, '$.tools.base') = :base
    AND length(json_extract(:key, '$.tools.materialization')) = 64
    AND json_extract(:key, '$.tools.materialization') NOT GLOB '*[^0-9a-f]*'
    AND CASE json_extract(:key, '$.schemaVersion')
      WHEN 1 THEN length(json_extract(:key, '$.tools.jco')) > 0 AND
        ((SELECT COUNT(*) FROM json_each(:key, '$.tools')) = 3
          AND json_extract(:key, '$.tools.native-transpile') IS NULL
        OR (SELECT COUNT(*) FROM json_each(:key, '$.tools')) = 4
          AND length(json_extract(:key, '$.tools.native-transpile')) = 64
          AND json_extract(:key, '$.tools.native-transpile') NOT GLOB '*[^0-9a-f]*')
      WHEN 2 THEN (SELECT COUNT(*) FROM json_each(:key, '$.tools')) = 3
        AND length(json_extract(:key, '$.tools.native-transpile')) = 64
        AND json_extract(:key, '$.tools.native-transpile') NOT GLOB '*[^0-9a-f]*'
      WHEN 3 THEN (SELECT COUNT(*) FROM json_each(:key, '$.tools')) = 4
        AND length(json_extract(:key, '$.tools.native-transpile')) = 64
        AND json_extract(:key, '$.tools.native-transpile') NOT GLOB '*[^0-9a-f]*'
        AND length(json_extract(:key, '$.tools.maps')) = 64
        AND json_extract(:key, '$.tools.maps') NOT GLOB '*[^0-9a-f]*'
      ELSE FALSE END
  ) AS accepted
`);

for (const row of fixture.cases) {
  test(row.id, () => {
    const tools: Record<string, string> = { ...fixture.tools, ...row.tools };
    if (row.removeTool) delete tools[row.removeTool];
    const key = {
      stage: row.stage ?? "transpile",
      schemaVersion: row.version,
      inputs: { component: fixture.component, ...row.inputs },
      tools,
    };
    const oracle = compare.get({ key: JSON.stringify(key), component: fixture.component, base: fixture.base });
    expect(Boolean(oracle?.accepted)).toBe(row.accepted);
    expect(matchesTranspileKey({ version: 1, key, digest: "", files: [] }, fixture.component, fixture.base)).toBe(row.accepted);
  });
}

const optimize = database.query<{ accepted: number }, { version: number; tools: string }>(`
  SELECT (
    (:version = 1 OR :version = 2)
    AND length(json_extract(:tools, '$."wasm-opt"')) > 0
  ) AS accepted
`);

for (const row of [
  { id: "optimize-v1", version: 1, tools: { "wasm-opt": "version 123" }, accepted: true },
  { id: "optimize-v2", version: 2, tools: { "wasm-opt": "wasm-opt version 130 (version_130)", binary: "aa", args: "-Oz" }, accepted: true },
  { id: "optimize-v3", version: 3, tools: { "wasm-opt": "version 130" }, accepted: false },
  { id: "optimize-empty", version: 2, tools: { "wasm-opt": "" }, accepted: false },
] as const) {
  test(row.id, () => {
    const oracle = optimize.get({ version: row.version, tools: JSON.stringify(row.tools) });
    expect(Boolean(oracle?.accepted)).toBe(row.accepted);
    expect(admittedOptimizeSchema(row.version, row.tools)).toBe(row.accepted);
  });
}
