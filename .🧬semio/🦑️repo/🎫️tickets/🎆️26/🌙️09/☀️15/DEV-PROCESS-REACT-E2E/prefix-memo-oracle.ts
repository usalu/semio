import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import { Box3, Vector3 } from "three";

const fixture = await Bun.file("✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/📦️prefix-memo/🔣️.json").json();
const database = new Database(":memory:");
try {
  const prefixWindow = database.query(`
    WITH RECURSIVE prefixes(position) AS (
      VALUES (0) UNION ALL SELECT position + 1 FROM prefixes WHERE position < ?
    )
    SELECT count(*) AS count, min(position) AS oldest
    FROM (SELECT position FROM prefixes ORDER BY position DESC LIMIT ?)
  `).get(fixture.emptySteps + 1, fixture.memoCapacity);
  assert.deepEqual(prefixWindow, { count: fixture.memoCapacity, oldest: fixture.oldestRetainedPrefix });
  const stock = new Box3().setFromCenterAndSize(new Vector3(), new Vector3(...fixture.stockSize));
  const cutter = new Box3().setFromCenterAndSize(new Vector3(), new Vector3(...fixture.cutSize));
  const attachment = new Box3().setFromCenterAndSize(new Vector3(), new Vector3(...fixture.attachmentSize));
  const volume = (box: Box3) => {
    const { x, y, z } = box.getSize(new Vector3());
    return x * y * z;
  };
  assert.ok(cutter.containsBox(stock));
  assert.equal(volume(stock), fixture.stockVolume);
  assert.equal(volume(attachment), fixture.restoredVolume);
  console.log("[DEBUG] SQLite prefix window and Three.js geometry match the memo fixture");
} finally {
  database.close();
}
