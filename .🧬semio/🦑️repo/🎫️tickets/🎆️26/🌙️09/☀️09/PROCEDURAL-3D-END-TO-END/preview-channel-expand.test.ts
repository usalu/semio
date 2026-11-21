import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/🔢️value-readout.json").json();
const db = new Database(":memory:");
for (const row of fixture.expanded) {
  test(row.path, () => {
    const result = db.query("select json_extract(?, ?) value, json_array_length(?, ?) total")
      .get(JSON.stringify(fixture.eval), row.path, JSON.stringify(fixture.eval), row.containerPath);
    expect(result).toEqual({ value: row.value, total: row.total });
  });
}
