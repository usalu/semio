import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync(new URL("./note-window-config-batch.json", import.meta.url), "utf8"));
const unitPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs";
const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗄️store-owners/🦀️.rs";

test("a paid camera Store batch is constructed independently of native decoding", () => {
  using oracle = new Database(":memory:");
  const route = oracle.query("SELECT CASE WHEN NOT $funding THEN 'wait' WHEN NOT $batch THEN 'construct' WHEN $decoder THEN 'receive' ELSE 'wait' END AS next");
  for (const row of fixture) {
    expect(route.get({
      $batch: +row.batch,
      $funding: +row.funding,
      $decoder: +row.decoder,
    })).toEqual({ next: row.next });
  }
});

test("the window publication issues a constructor receipt before opening a receiving scope", () => {
  const unit = readFileSync(unitPath, "utf8");
  const owner = readFileSync(ownerPath, "utf8");
  const section = unit.slice(unit.indexOf("let (pending_lane, _) = pending.lane()"));
  const constructor = section.indexOf("self.stores.window_config_store.admit_store_batch");
  const receive = section.indexOf("cx.with_native_receiving");
  expect(constructor).toBeGreaterThan(-1);
  expect(constructor).toBeLessThan(receive);
  expect(section.slice(constructor, receive)).toContain("mounted.publication_ownership_progress = Some((retained, ownership));");
  expect(section.slice(constructor, receive)).toContain("return Ok(())");
  const advance = owner.slice(owner.lastIndexOf("fn advance("), owner.lastIndexOf("fn refresh_demands("));
  expect(advance).toContain("let Some(batch) = publication.publication.as_mut()");
  expect(advance).not.toContain("admit_member_apply_batch(");
});
