import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { Database } from "bun:sqlite";

const fixture = JSON.parse(readFileSync(new URL("./terrain-nodata-editing.json", import.meta.url), "utf8"));
const db = new Database(":memory:");
try {
  const columns = fixture.rowsMillimetres[0].length;
  const mask = Buffer.from(fixture.missingSamples, "hex");
  const absent = (column, row) => {
    const position = row * columns + column;
    return Boolean(mask[Math.floor(position / 8)] & 1 << (position % 8));
  };
  db.run("CREATE TABLE elevation (x INTEGER, y INTEGER, height INTEGER, PRIMARY KEY (x,y))");
  const insert = db.prepare("INSERT INTO elevation VALUES (?, ?, ?)");
  for (const [row, values] of fixture.rowsMillimetres.entries()) {
    for (const [column, height] of values.entries()) {
      if (!absent(column, row)) insert.run(column, row, height);
    }
  }
  const localMean = db.query("SELECT avg(height) AS mean FROM elevation WHERE abs(x - ?) <= 1 AND abs(y - ?) <= 1");
  const fill = db.query("SELECT height FROM elevation WHERE x = ? AND y = ?");
  const stamp = db.query("SELECT max(0, (? + 1) * (? + 1) - (x - ?) * (x - ?) - (y - ?) * (y - ?)) AS influence FROM elevation WHERE x = ? AND y = ?");
  for (const test of fixture.tests) {
    if (test.error) {
      assert.equal(fill.get(test.column, test.row), null);
      continue;
    }
    const samples = fixture.rowsMillimetres.map((row, y) => row.map((height, x) => {
      if (absent(x, y)) return 0;
      if (test.operation === "offset") return height + test.value;
      const { influence } = stamp.get(test.radius, test.radius, test.column, test.column, test.row, test.row, x, y);
      const fraction = influence / (test.radius + 1) ** 2;
      if (test.operation === "raise-lower") return height + Math.round(test.value * fraction);
      return height + Math.round((localMean.get(x, y).mean - height) * test.value / 100 * fraction);
    }));
    assert.deepEqual(samples, test.rowsMillimetres, test.operation);
  }
  console.log("SQLite independently matched NoData editing, bounded smoothing, and sample refusal");
} finally {
  db.close();
}
