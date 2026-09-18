/** 🧪️ Cross-language fixture oracle — every committed `s.wfc.grid2d` vector, replayed by the
 * TypeScript twin and compared against the bytes Rust committed.
 *
 * Vectors are DISCOVERED on disk under `🧫️fixtures/🧬️mutations/<slug>/<case>/`, never transcribed:
 * a vector added to the ticket's own case table must reach this half without anyone editing it.
 * What the TS half actually re-computes is the DIFF APPLY — the one step where a language can
 * disagree about row position — plus the wire vocabularies (mutation tags, direction tokens, media
 * tags) the two schemas are supposed to share.
 */

import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { applyGrid2dDiff, cellId, type Grid2dDiff } from "../../🔺️diff/🟦️.ts";
import { GRID2D_MUTATION_KINDS, type Grid2dMutation } from "../../🧬️mutations/🟦️.ts";
import { WFC_GRID2D_DOCUMENT_SCHEMA, directionOffset, oppositeDirection, type Grid2dSnapshot, type WfcDirection2d } from "../../📸️snapshot/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const subset = join(here, "../../..");
const vectorsRoot = join(subset, "🧫️fixtures/🧬️mutations");

interface Outcome {
  status: "applied" | "rejected";
  messages?: { level: string; code: string }[];
}

interface Vector {
  slug: string;
  caseName: string;
  before: Grid2dSnapshot;
  after: Grid2dSnapshot;
  mutation: Grid2dMutation;
  diff: Grid2dDiff;
  outcome: Outcome;
}

const readJson = <T>(path: string): T => JSON.parse(readFileSync(path, "utf8")) as T;

/** 🔎 Globs the committed vectors exactly as they are on disk right now. */
function discoverVectors(): Vector[] {
  const vectors: Vector[] = [];
  for (const slug of readdirSync(vectorsRoot).sort()) {
    for (const caseName of readdirSync(join(vectorsRoot, slug)).sort()) {
      const root = join(vectorsRoot, slug, caseName);
      if (!existsSync(join(root, "🦠️mutation/🔣️.json"))) continue;
      vectors.push({
        slug,
        caseName,
        before: readJson<Grid2dSnapshot>(join(root, "📸️snapshot/⬅️before/🔣️.json")),
        after: readJson<Grid2dSnapshot>(join(root, "📸️snapshot/➡️after/🔣️.json")),
        mutation: readJson<Grid2dMutation>(join(root, "🦠️mutation/🔣️.json")),
        diff: readJson<Grid2dDiff>(join(root, "🔺️diff/🔣️.json")),
        outcome: readJson<Outcome>(join(root, "🎯️outcome/🔣️.json")),
      });
    }
  }
  return vectors;
}

/** 🏷️ The PascalCase dispatch tag each kebab-case kind serializes as. */
const tagOf = (kind: string): string => kind.split("-").map((word) => word[0]!.toUpperCase() + word.slice(1)).join("");

const vectors = discoverVectors();

describe("s.wfc.grid2d cross-language fixture oracle", () => {
  it("discovers one committed vector per declared mutation kind", () => {
    expect(vectors.length).toBeGreaterThanOrEqual(GRID2D_MUTATION_KINDS.length);
    const slugs = new Set(vectors.map((vector) => vector.slug));
    expect(slugs.size).toBe(GRID2D_MUTATION_KINDS.length);
  });

  it("every committed before-snapshot states this artifact's own schema id", () => {
    for (const vector of vectors) expect(vector.before.schema, `${vector.slug}/${vector.caseName}`).toBe(WFC_GRID2D_DOCUMENT_SCHEMA);
  });

  it("every committed mutation carries exactly one known dispatch tag", () => {
    const known = new Set(GRID2D_MUTATION_KINDS.map(tagOf));
    for (const vector of vectors) {
      const tags = Object.keys(vector.mutation as Record<string, unknown>);
      expect(tags.length, `${vector.slug}/${vector.caseName}`).toBe(1);
      expect(known.has(tags[0]!), `${vector.slug}/${vector.caseName}: unknown tag ${tags[0]}`).toBe(true);
    }
  });

  it.each(vectors.map((vector) => [`${vector.slug}/${vector.caseName}`, vector] as const))("%s: the committed diff carries before to after", (_label, vector) => {
    expect(applyGrid2dDiff(vector.before, vector.diff)).toEqual(vector.after);
  });

  it("an applied vector moves the document and a rejected one leaves it alone", () => {
    for (const vector of vectors) {
      const label = `${vector.slug}/${vector.caseName}`;
      if (vector.outcome.status === "applied") expect(vector.after, label).not.toEqual(vector.before);
      else expect(vector.after, label).toEqual(vector.before);
    }
  });

  it("every rule in every committed vector names a tile the catalogue holds", () => {
    for (const vector of vectors) {
      for (const snapshot of [vector.before, vector.after]) {
        const tiles = new Set(snapshot.tiles.map((tile) => tile.id));
        for (const rule of snapshot.rules) {
          expect(tiles.has(rule.tileAId), `${vector.slug}/${vector.caseName}: ${rule.id}`).toBe(true);
          expect(tiles.has(rule.tileBId), `${vector.slug}/${vector.caseName}: ${rule.id}`).toBe(true);
        }
        for (const pin of snapshot.pinned) expect(tiles.has(pin.tileId), `${vector.slug}/${vector.caseName}: pin ${cellId(pin.x, pin.y)}`).toBe(true);
      }
    }
  });

  it("every committed collection is in its canonical order", () => {
    for (const vector of vectors) {
      for (const snapshot of [vector.before, vector.after]) {
        const label = `${vector.slug}/${vector.caseName}`;
        expect(snapshot.tiles.map((tile) => tile.id), label).toEqual([...snapshot.tiles.map((tile) => tile.id)].sort());
        expect(snapshot.rules.map((rule) => rule.id), label).toEqual([...snapshot.rules.map((rule) => rule.id)].sort());
        for (const cells of [snapshot.pinned, snapshot.masked]) {
          const keys = cells.map((cell) => [cell.y, cell.x] as const);
          expect(keys, label).toEqual([...keys].sort((left, right) => left[0] - right[0] || left[1] - right[1]));
        }
      }
    }
  });

  it("the direction vocabulary is closed and every direction cancels its own opposite", () => {
    const directions: WfcDirection2d[] = ["LEFT", "RIGHT", "TOP", "BOTTOM"];
    for (const direction of directions) {
      expect(oppositeDirection(oppositeDirection(direction))).toBe(direction);
      const [dx, dy] = directionOffset(direction);
      const [ix, iy] = directionOffset(oppositeDirection(direction));
      expect([dx + ix, dy + iy]).toEqual([0, 0]);
    }
    for (const vector of vectors) {
      for (const rule of vector.before.rules) expect(directions, `${vector.slug}: ${rule.id}`).toContain(rule.direction);
    }
  });

  it("every committed tile media carries a known tag and its own payload", () => {
    for (const vector of vectors) {
      for (const tile of vector.after.tiles) {
        const label = `${vector.slug}/${vector.caseName}: ${tile.id}`;
        expect(tile.weight, label).toBeGreaterThan(0);
        if (tile.media.kind === "bitmap") {
          expect(tile.media.palette.length, label).toBeGreaterThan(0);
          expect(typeof tile.media.pixels, label).toBe("string");
        } else if (tile.media.kind === "vector") {
          expect(Array.isArray(tile.media.paths), label).toBe(true);
        } else {
          expect(tile.media.kind, label).toBe("image");
        }
      }
    }
  });
});
