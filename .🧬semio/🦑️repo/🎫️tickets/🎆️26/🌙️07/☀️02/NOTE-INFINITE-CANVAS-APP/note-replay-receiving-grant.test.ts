/** 🎟️ Verifies that the native replay child receives the stock left after scope setup. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./note-replay-receiving-grant.json";

test("archive replay fits one scope item and at most two child items in an issued three", async () => {
  const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️native/🦀️.rs";
  const source = await Bun.file(path).text();
  const next = source.match(/owner\.step\(generation, self\.revision, (\w+)\.maximum_copy_bytes, (\w+), control,/);
  expect(next).not.toBeNull();
  const database = new Database(":memory:");
  try {
    const total = database.query(`
      SELECT $scopeItems + MIN($parentItems - $scopeItems, $demandItems) AS items,
             $scopeBytes + CASE WHEN $parentItems > $scopeItems THEN MIN($parentBytes - $scopeBytes, $demandBytes) ELSE 0 END AS bytes
    `);
    for (const { parent, scope, demand, expected } of fixture) {
      expect(total.get({
        $scopeItems: scope.items, $parentItems: parent.items, $demandItems: demand.items,
        $scopeBytes: scope.bytes, $parentBytes: parent.bytes, $demandBytes: demand.bytes,
      })).toEqual(expected);
      const loans = { grant: parent, available: { items: parent.items - scope.items, bytes: parent.bytes - scope.bytes } };
      const items = loans[next![2] as keyof typeof loans].items;
      const bytes = loans[next![1] as keyof typeof loans].bytes;
      expect({ items: scope.items + Math.min(items, demand.items), bytes: scope.bytes + (items > 0 ? Math.min(bytes, demand.bytes) : 0) }).toEqual(expected);
    }
  } finally {
    database.close();
  }
});
