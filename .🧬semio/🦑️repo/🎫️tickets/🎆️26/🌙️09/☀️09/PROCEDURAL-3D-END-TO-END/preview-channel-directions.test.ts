import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/preview-channel-directions.json").json();
const database = new Database(":memory:", { strict: true });
const leaves = database.query(`
  WITH tree AS (SELECT * FROM json_tree(?))
  SELECT leaf.atom FROM tree AS leaf JOIN tree AS owner ON leaf.parent = owner.id
  WHERE leaf.key = 'handle' AND leaf.type = 'text'
    AND json_extract(CASE WHEN owner.type = 'object' THEN owner.value END, '$."$schema"') IN ('geometry', 'vertex', 'edge', 'face', 'shell')
  ORDER BY leaf.id
`);
const marks = database.query("SELECT value FROM json_each(?) WHERE value IN (?, ?)");
function side(widget, channels, input) {
  return Object.keys(channels).sort().flatMap(channel => {
    const port = widget + "@" + (input ? "!i" : (!channel || channel.startsWith("!")) ? "!o" : "") + channel;
    return leaves.all(JSON.stringify(channels[channel])).map(({ atom: handle }, index) => ({
      port, instance: port + "#" + (input ? "in-" : "") + index, handle, input,
    }));
  });
}
for (const row of fixture.rows) {
  test(row.widget + " distinguishes viewport directions and export authority", () => {
    const value = fixture.evaluation[row.widget];
    const input = side(row.widget, value.in ?? {}, true);
    const output = side(row.widget, value.out ?? {}, false);
    expect([...output, ...input]).toEqual(row.viewport);
    expect((Object.keys(value.out ?? {}).length ? output : input).map(item => item.handle)).toEqual(row.export);
    expect(new Set(row.viewport.map(item => item.instance)).size).toBe(row.viewport.length);
    for (const mark of row.marks) {
      const seen = row.viewport.filter(item => marks.all(JSON.stringify(mark.ids), "n:" + row.widget, "h:" + item.port).length);
      expect(seen.map(item => item.instance)).toEqual(mark.instances);
    }
  });
}

test("a hidden export source retains its named channel scope", () => {
  const authority = fixture.exportOnly;
  const output = side(authority.widget, fixture.evaluation[authority.widget].out, false);
  expect(output.filter(item => item.port === authority.widget + "@" + authority.channel).map(item => item.handle)).toEqual([authority.pending]);
  expect(fixture.rows.some(row => row.widget === authority.widget)).toBe(false);
});
