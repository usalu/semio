import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { navbarExampleIdFromHistoryUpserts } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import cases from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔒️shell-option-locks/📚️example-offers-and-switching/history.json";

test("archive-only history has no example edit to undo", () => {
  const sqlite = new Database(":memory:");
  try {
    const read = sqlite.query("SELECT CASE COUNT(*) WHEN 0 THEN NULL ELSE CASE MAX(applied) WHEN 0 THEN 'demo' ELSE 'concrete-forest' END END AS label FROM (SELECT json_extract(value, '$.applied') AS applied FROM json_each(?) WHERE json_extract(value, '$.actionId') = 'setActiveExample' AND (json_extract(value, '$.editId') IS NOT NULL OR json_extract(value, '$.opCount') > 0))");
    for (const row of cases) {
      expect(read.get(JSON.stringify(row.upserts))).toEqual({ label: row.expected });
      expect(navbarExampleIdFromHistoryUpserts(row.upserts, "concrete-forest", "demo") ?? null).toBe(row.expected);
    }
  } finally {
    sqlite.close();
  }
});
