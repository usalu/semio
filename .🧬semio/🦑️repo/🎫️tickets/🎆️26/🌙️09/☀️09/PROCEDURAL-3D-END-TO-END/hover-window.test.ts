import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

test("only the current interactive window and channel may clear hover", async () => {
  const fixture = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🧭️interaction/🧪️tests/🧫️fixtures/hover-window.tsv").text();
  const cache = new Database(":memory:", { strict: true });
  cache.exec("CREATE TABLE hover (channel TEXT, owner TEXT)");
  for (const line of fixture.trimEnd().split("\n").slice(1)) {
    const [name, channel, owner, requested, source, clears] = line.split("\t");
    cache.exec("DELETE FROM hover");
    if (channel) cache.run("INSERT INTO hover VALUES (?, ?)", [channel, owner || null]);
    const result = cache.run("DELETE FROM hover WHERE channel = ? AND (? IS NULL OR owner IS NULL OR owner = ?)", [requested, source || null, source || null]);
    expect(!channel || result.changes > 0, name).toBe(clears === "true");
  }
  cache.close();
});
