import { describe, expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { basename, join, relative, resolve } from "node:path";
import fastGlob from "fast-glob";
import { inventoryTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

//#region 🧪️ProfileUnprefixedInference
const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const ticketRoot = import.meta.dir;
const vectors = JSON.parse(readFileSync(join(ticketRoot, "🧪️profile-unprefixed-inference", "🔣️.json"), "utf8")) as {
  schemaVersion: number;
  vectors: { relativePath: string; expectedNormalizedName: string }[];
};

describe("profile unprefixed inference", () => {
  test("keeps catalog profiles emoji-explicit while generic unprefixed test names resolve as test cases", async () => {
    const fixture = mkdtempSync(join(ticketRoot, "🧪️profile-inference-"));
    try {
      for (const vector of vectors.vectors) mkdirSync(join(fixture, vector.relativePath), { recursive: true });
      const discovered = (await fastGlob(["🧪️tests/*", "📚️examples/*"], { cwd: fixture, onlyDirectories: true })).sort();
      expect(vectors.schemaVersion).toBe(1);
      expect(discovered).toEqual(vectors.vectors.map((vector) => vector.relativePath).sort());
      const scope = relative(repoRoot, fixture).replaceAll("\\", "/");
      const inventory = inventoryTaxonomy({ repoRoot, scope, ticketDir: fixture });
      for (const vector of vectors.vectors) {
        const entry = inventory.entries.find((candidate) => candidate.sourcePath === `${scope}/${vector.relativePath}`);
        expect(entry?.violations.map((violation) => violation.code)).not.toContain("directory-kind-ambiguous");
        expect(basename(entry?.normalizedPath ?? "")).toBe(vector.expectedNormalizedName);
      }
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️ProfileUnprefixedInference
