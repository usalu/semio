/** 🎟️ Compares the receiving scope's fresh authority with SQLite's independent limit calculation. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./note-native-scope-current-turn.json";

test("fresh native receiving cannot spend the current Job grant already used by scope setup", async () => {
  const sqlite = new Database(":memory:");
  try {
    const limit = sqlite.query(`
      SELECT key, CASE WHEN $recovery THEN value ELSE MIN(value, json_extract($fresh, '$.' || key)) END AS amount
      FROM json_each($requested)
    `);
    for (const row of fixture) {
      const result = Object.fromEntries((limit.all({ $requested: JSON.stringify(row.requested), $fresh: JSON.stringify(row.fresh), $recovery: Number(row.recovery) }) as { key: string; amount: number }[]).map(({ key, amount }) => [key, amount]));
      const exact = Object.entries(row.requested).every(([axis, amount]) => result[axis] === amount);
      const admitted = row.recovery || (!row.quoted || exact) && result.items > 0 && result.depth > 0 && result.work > 0;
      expect(admitted ? result : null).toEqual(row.admitted);
    }
  } finally {
    sqlite.close();
  }
  const source = await Bun.file("🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs").text();
  const scope = source.slice(source.indexOf("pub(super) fn original_receiving_outcome"));
  expect(scope).toMatch(/self\.consume_retained\(progress\)[\s\S]*?if !plan\.recovering_pending \{[\s\S]*?plan\.quantum\.restricted_by\(self\.native_scope_quantum\(\)\)[\s\S]*?quote\.is_some\(\)[\s\S]*?plan\.quantum = grant;[\s\S]*?\.run_scoped_on_caller/);
});

test("supersession uses the required copy, release and Work quote for the nested fold", async () => {
  const source = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧬️supersession/🦀️.rs").text();
  const sqlite = new Database(":memory:");
  try {
    const observed = sqlite.query("SELECT $issuedCopy >= $copyDemand AS copyCovered, $issuedRelease >= $releaseDemand AS releaseCovered");
    const demand = { $copyDemand: 4385, $releaseDemand: 2648 };
    expect(observed.get({ ...demand, $issuedCopy: 2257, $issuedRelease: 524288 })).toEqual({ copyCovered: 0, releaseCovered: 1 });
    expect(observed.get({ ...demand, $issuedCopy: 4385, $issuedRelease: 2648 })).toEqual({ copyCovered: 1, releaseCovered: 1 });
  } finally {
    sqlite.close();
  }
  expect(source).toMatch(/with_native_receiving_quoted\(&mut scope,\s*\(grant,\s*work\)/);
  expect(source).toMatch(/original_step_demand\(body\)\?[\s\S]{0,160}receive\(cx,\s*d,\s*protocol::HistoryFoldJob::<Output>::ORIGINAL_STEP_WORK,/);
});

test("the history fold quote includes its original receiving reply handoff", async () => {
  const fold = "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold";
  const { originalStepWork: fixture } = await Bun.file(`${fold}/🧪️tests/🧫️fixtures/🔣️.json`).json();
  const source = await Bun.file(`${fold}/🏗️birth/🎟️original/🦀️.rs`).text();
  expect(source).toContain("ORIGINAL_STEP_WORK: u64 = FOLD_BODY_WORK + ORIGINAL_INVOCATION_WORK");
  expect(source).toContain("native.with_effect(step_grant, FOLD_BODY_WORK,");
  const sqlite = new Database(":memory:");
  try {
    const admission = sqlite.query("SELECT CASE WHEN ? >= ? + ? THEN 1 ELSE 0 END AS admitted");
    for (const row of fixture.cases) {
      expect(admission.get(row.available, fixture.bodyLimit, fixture.replyHandoff)).toEqual({ admitted: row.bodyPerformed });
      expect(row.nativePerformed).toEqual(row.bodyPerformed ? row.bodyPerformed + fixture.replyHandoff : 0);
    }
  } finally {
    sqlite.close();
  }
});

