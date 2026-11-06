import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import Ajv from "ajv";
import snapshotSchema from "../../📸️snapshot/🔣️.json";

const subset = join(dirname(fileURLToPath(import.meta.url)), "../../..");
const root = join(subset, "🧪️tests/🧫️fixtures/🧬️mutations");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const vectors = readdirSync(root).sort().flatMap(kind => readdirSync(join(root, kind)).sort().map(name => ({ kind, name, path: join(root, kind, name) })));
const references = join(process.cwd(), "🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference");
const validate = new Ajv({ strict: false }).addSchema(read(join(references, "🔣️.json"))).addSchema(read(join(references, "🪆️child/🧬️schema/🔣️.json"))).compile(snapshotSchema);

describe("grid3d portable snapshots", () => {
  it("covers every declared mutation with a retained vector", () => {
    const mutationsRoot = join(subset, "🧬️schema/🧬️mutations");
    const mutations = readdirSync(mutationsRoot, { withFileTypes: true }).filter(entry => entry.isDirectory() && existsSync(join(mutationsRoot, entry.name, "🧬️schema/🔣️.json")));
    expect(vectors.length).toBeGreaterThanOrEqual(mutations.length);
    expect(new Set(vectors.map(vector => vector.kind))).toEqual(new Set(mutations.map(entry => entry.name)));
  });

  it.each(vectors.map(vector => [`${vector.kind}/${vector.name}`, vector] as const))("%s snapshots conform and reference their own tiles", (_label, vector) => {
    for (const side of ["⬅️before", "➡️after"]) {
      const snapshot = read(join(vector.path, "📸️snapshot", side, "🔣️.json"));
      expect(validate(snapshot), JSON.stringify(validate.errors)).toBe(true);
      for (const [axis, size] of [["X", "width"], ["Y", "height"], ["Z", "depth"]]) expect(snapshot[`cellSizes${axis}`]).toHaveLength(snapshot[size]);
      const ids = new Set(snapshot.tiles.map((tile: { id: string }) => tile.id));
      for (const rule of snapshot.rules) expect(ids.has(rule.tileAId) && ids.has(rule.tileBId)).toBe(true);
      for (const cell of [...snapshot.pinned, ...snapshot.masked]) {
        expect(cell.x < snapshot.width && cell.y < snapshot.height && cell.z < snapshot.depth).toBe(true);
        if ("tileId" in cell) expect(ids.has(cell.tileId)).toBe(true);
      }
    }
  });
});
