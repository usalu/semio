/** 🧪️ Cross-language fixture oracle — every committed `s.wfc.bitmap` vector, replayed by the
 * TypeScript twin and compared against the bytes Rust committed.
 *
 * Vectors are DISCOVERED on disk under `🧫️fixtures/🧬️mutations/<slug>/<case>/`, never transcribed,
 * so a vector added on the Rust side reaches this half without anyone editing it. What the TS half
 * re-computes is the DIFF APPLY — the lane order, the resize pad rule and the index a pin is
 * restored at — plus the base64 pixel carrier, which is the one field a language could disagree
 * about invisibly.
 */

import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { applyBitmapDiff, type BitmapDiff } from "../../🔺️diff/🟦️.ts";
import { BITMAP_MUTATION_KINDS, type BitmapMutation } from "../../🧬️mutations/🟦️.ts";
import { BITMAP_MAX_PALETTE, WFC_BITMAP_DOCUMENT_SCHEMA, bitmapIndices, decodeBase64, encodeBase64, pinKey, type BitmapSnapshot } from "../../📸️snapshot/🟦️.ts";

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
  before: BitmapSnapshot;
  after: BitmapSnapshot;
  mutation: BitmapMutation;
  diff: BitmapDiff;
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
        before: readJson<BitmapSnapshot>(join(root, "📸️snapshot/⬅️before/🔣️.json")),
        after: readJson<BitmapSnapshot>(join(root, "📸️snapshot/➡️after/🔣️.json")),
        mutation: readJson<BitmapMutation>(join(root, "🦠️mutation/🔣️.json")),
        diff: readJson<BitmapDiff>(join(root, "🔺️diff/🔣️.json")),
        outcome: readJson<Outcome>(join(root, "🎯️outcome/🔣️.json")),
      });
    }
  }
  return vectors;
}

/** 🏷️ The PascalCase dispatch tag each kebab-case kind serializes as. */
const tagOf = (kind: string): string => kind.split("-").map((word) => word[0]!.toUpperCase() + word.slice(1)).join("");

const vectors = discoverVectors();

describe("s.wfc.bitmap cross-language fixture oracle", () => {
  it("discovers one committed vector per declared mutation kind", () => {
    expect(vectors.length).toBeGreaterThanOrEqual(BITMAP_MUTATION_KINDS.length);
    expect(new Set(vectors.map((vector) => vector.slug)).size).toBe(BITMAP_MUTATION_KINDS.length);
  });

  it("every committed before-snapshot states this artifact's own schema id", () => {
    for (const vector of vectors) expect(vector.before.schema, `${vector.slug}/${vector.caseName}`).toBe(WFC_BITMAP_DOCUMENT_SCHEMA);
  });

  it("every committed mutation carries exactly one known dispatch tag", () => {
    const known = new Set(BITMAP_MUTATION_KINDS.map(tagOf));
    for (const vector of vectors) {
      const tags = Object.keys(vector.mutation as Record<string, unknown>);
      expect(tags.length, `${vector.slug}/${vector.caseName}`).toBe(1);
      expect(known.has(tags[0]!), `${vector.slug}/${vector.caseName}: unknown tag ${tags[0]}`).toBe(true);
    }
  });

  it.each(vectors.map((vector) => [`${vector.slug}/${vector.caseName}`, vector] as const))("%s: the committed diff carries before to after", (_label, vector) => {
    expect(applyBitmapDiff(vector.before, vector.diff)).toEqual(vector.after);
  });

  it("an applied vector moves the document and a rejected one leaves it alone", () => {
    for (const vector of vectors) {
      const label = `${vector.slug}/${vector.caseName}`;
      if (vector.outcome.status === "applied") expect(vector.after, label).not.toEqual(vector.before);
      else expect(vector.after, label).toEqual(vector.before);
    }
  });

  it("every committed pixel buffer decodes to exactly its own extent and names real palette entries", () => {
    for (const vector of vectors) {
      for (const snapshot of [vector.before, vector.after]) {
        const label = `${vector.slug}/${vector.caseName}`;
        const indices = bitmapIndices(snapshot.input);
        expect(indices, label).not.toBeNull();
        expect(indices!.length, label).toBe(snapshot.input.width * snapshot.input.height);
        expect(snapshot.input.palette.length, label).toBeGreaterThan(0);
        expect(snapshot.input.palette.length, label).toBeLessThanOrEqual(BITMAP_MAX_PALETTE);
        for (const index of indices!) expect(index, label).toBeLessThan(snapshot.input.palette.length);
        for (const pin of snapshot.pinned) {
          expect(pin.color, `${label}: pin ${pinKey(pin.x, pin.y)}`).toBeLessThan(snapshot.input.palette.length);
          expect(pin.x, `${label}: pin ${pinKey(pin.x, pin.y)}`).toBeLessThan(snapshot.output.width);
          expect(pin.y, `${label}: pin ${pinKey(pin.x, pin.y)}`).toBeLessThan(snapshot.output.height);
        }
      }
    }
  });

  it("the base64 carrier round-trips every committed buffer byte for byte", () => {
    for (const vector of vectors) {
      const label = `${vector.slug}/${vector.caseName}`;
      const decoded = decodeBase64(vector.after.input.pixels);
      expect(encodeBase64(decoded), label).toBe(vector.after.input.pixels);
    }
  });

  it("every committed pin list is in canonical row-major order", () => {
    for (const vector of vectors) {
      for (const snapshot of [vector.before, vector.after]) {
        const keys = snapshot.pinned.map((pin) => [pin.y, pin.x] as const);
        expect(keys, `${vector.slug}/${vector.caseName}`).toEqual([...keys].sort((left, right) => left[0] - right[0] || left[1] - right[1]));
      }
    }
  });

  it("every committed model parameter is inside the range the extractor admits", () => {
    for (const vector of vectors) {
      for (const snapshot of [vector.before, vector.after]) {
        const label = `${vector.slug}/${vector.caseName}`;
        expect(snapshot.model.patternSize, label).toBeGreaterThanOrEqual(2);
        expect(snapshot.model.patternSize, label).toBeLessThanOrEqual(5);
        expect(snapshot.model.symmetry, label).toBeGreaterThanOrEqual(1);
        expect(snapshot.model.symmetry, label).toBeLessThanOrEqual(8);
        if (snapshot.model.ground != null) expect(snapshot.model.ground, label).toBeLessThan(snapshot.input.palette.length);
      }
    }
  });
});
