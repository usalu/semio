import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const fixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/preview-value-paths.json").json();

const nativeSchemas = new Set(["geometry", "vertex", "edge", "face", "shell"]);

function normalizeGeometry(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(normalizeGeometry);
  if (value === null || typeof value !== "object") return value;
  const record = value as Record<string, unknown>;
  if (record.$schema === "mesh" || record.$schema === "plane") return null;
  if (nativeSchemas.has(record.$schema) && typeof record.handle === "string") return { handle: record.handle };
  if ((record.$schema === "point" || record.$schema === "vector") && [record.x, record.y, record.z].every(Number.isFinite)) return null;
  if (record.$schema === "list") {
    const values: unknown[] = [];
    for (const [key, child] of Object.entries(record)) {
      if (/^(0|[1-9][0-9]*)$/.test(key)) values[Number(key)] = normalizeGeometry(child);
    }
    return values;
  }
  return Object.fromEntries(Object.entries(record).filter(([key, child]) => key !== "handle" || typeof child !== "string").map(([key, child]) => [key, normalizeGeometry(child)]));
}

test("older geometry resolves at the identical channel side and structural address", () => {
  const cache = new Database(":memory:", { strict: true });
  const query = cache.query(
    "SELECT current.atom AS handle, previous.atom AS fallback FROM json_tree(?) AS current LEFT JOIN json_tree(?) AS previous ON current.fullkey = previous.fullkey AND previous.type = 'text' WHERE current.key = 'handle'",
  );
  for (const row of fixture.cases) {
    const current = JSON.stringify({ [row.currentSide ?? "out"]: normalizeGeometry(row.current) });
    const previousSide = row.previousSide ?? "out";
    const peerSide = previousSide === "in" ? "out" : "in";
    const previous = JSON.stringify({ [previousSide]: normalizeGeometry(row.previous),
      ...(row.previousPeer === undefined ? {} : { [peerSide]: normalizeGeometry(row.previousPeer) }) });
    const seen = Object.fromEntries(query.all(current, previous).map(({ handle, fallback }) => [handle, fallback ?? null]));
    expect(seen, row.name).toEqual(row.expected);
  }
  cache.close();
});

test("the mesh projection preserves both evaluated payload fields", () => {
  const cache = new Database(":memory:", { strict: true });
  const row = cache.query("SELECT json_extract(?, '$.preview') AS preview, json_extract(?, '$.data') AS data")
    .get(JSON.stringify(fixture.inlineMesh), JSON.stringify(fixture.inlineMesh));
  expect(row).toEqual({ preview: fixture.inlineMesh.preview, data: fixture.inlineMesh.data });
  cache.close();
});
