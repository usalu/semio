/** ⏰ Verifies the archive maintenance wake contract against SQLite and the live Rust owners. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./note-archive-maintenance-wake.json";

const base = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
const read = (path: string) => Bun.file(base + path).text();

test("the caller wakes an eligible archive while terminal, host and resource waits remain idle", async () => {
  const database = new Database(":memory:");
  try {
    const select = database.query(`
      SELECT CAST(
        $live AND $state IN ('pending', 'running') AND
        ($phase <> 'MergeParent' OR $terminalTarget) AND
        $items >= 16 AND $fits
      AS INTEGER) AS awake
    `);
    for (const row of fixture) {
      const paid = select.get({
        $live: Number(row.live), $state: row.state, $phase: row.phase,
        $terminalTarget: Number(row.terminalTarget), $items: row.items, $fits: Number(row.fits),
      }) as { awake: number };
      expect(Boolean(paid.awake)).toBe(row.wake);
    }
  } finally {
    database.close();
  }
  const [contract, impl, ladder, driver, turn] = await Promise.all([
    read("⚛️reactor/🔌️app/🦀️.rs"),
    read("🧵️instance/🔌️plugin-app-impl/🦀️.rs"),
    read("🪜️maintenance-ladder/🦀️.rs"),
    read("⚛️reactor/🏃️runtime/🧰️ops/📄️document/🦀️.rs"),
    read("⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs"),
  ]);
  expect(contract).toMatch(/fn document_archive_maintenance_is_runnable\([^\n]*\{\s*Ok\(false\)/);
  expect(impl).toMatch(/fn document_archive_maintenance_is_runnable\([^\n]*\{\s*self\.ladder_ref\(\)\.document_archive_maintenance_is_runnable\(stock\)/);
  expect(ladder).toMatch(/stock\.maximum_items < DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS/);
  expect(ladder).toMatch(/document_archive_target\(false\)[\s\S]{0,240}plugin_grant_funds\(demand, stock\)/);
  expect(ladder).toContain("closing || (!entry.1.terminal() && !entry.1.awaits_merge())");
  expect(driver).toMatch(/guest_lifetimes\.get\(id\).*?!slot\.cell\.is_live\(\)[\s\S]{0,420}instance\.app\.document_archive_maintenance_is_runnable\(stock\)/);
  expect(turn).toMatch(/document_load_work \|\| [^\n]*plugin_document_archive_maintenance_is_runnable\(runtime, cx\.original_source_total_grant\(\)\)/);
});
