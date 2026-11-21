import { Database } from "bun:sqlite";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const owner = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval";
const law = JSON.parse(readFileSync(join(process.cwd(), owner, "🧪️tests/🧫️fixtures/preview-large-hover-group.json"), "utf8"));
const evidence = JSON.parse(readFileSync(join(import.meta.dirname, "🗑️generated/preview-direction/large-hover.json"), "utf8"));

test("a compact world hover reference preserves all authored directional addresses", () => {
  const sqlite = new Database(":memory:");
  try {
    const rows = sqlite.query(`
      WITH RECURSIVE copies(i) AS (
        SELECT 0 UNION ALL SELECT i + 1 FROM copies WHERE i + 1 < $count
      )
      SELECT 'h:' || printf('copy-%03d-', i) || replace(printf('%*s', $width - 9, ''), ' ', 'q')
        || '@' || CASE WHEN i % 2 = 0 THEN '!i' ELSE '' END || $channel AS id
      FROM copies ORDER BY i
    `).all({ $count: law.count, $width: law.widgetBytes, $channel: law.channel }) as {id: string}[];
    expect(evidence.instances.map(instance => instance.interactionId)).toEqual(rows.map(row => row.id));
    expect(evidence.hoveredIds).toEqual(evidence.instances.map(instance => instance.id));
    expect(new Set(evidence.instances.map(instance => instance.meshId)).size).toBe(1);
    const raw = rows.map(row => ({granularity:"handle", id:row.id}));
    expect(Buffer.byteLength(JSON.stringify(raw))).toBeGreaterThan(law.maximumActionBytes);
    const wire = evidence.instances[0].hoverTargets;
    expect(wire).toEqual([{granularity:"framework.hover-group", id:expect.stringMatching(/^preview-hover:\d+:\d+$/)}]);
    expect(Buffer.byteLength(JSON.stringify(wire))).toBeLessThan(law.maximumActionBytes);
    expect(evidence.instances.every(instance => JSON.stringify(instance.hoverTargets) === JSON.stringify(wire))).toBe(true);
  } finally {
    sqlite.close();
  }
});
