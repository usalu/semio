import { describe, expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { basename, join, relative, resolve } from "node:path";
import fastGlob from "fast-glob";
import { loadTaxonomy, semanticDirectoryKindId, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";
import { inventoryTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

//#region 🧪️StandardSubsetParentContext
const ticketRoot = import.meta.dir;
const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const vectors = JSON.parse(readFileSync(join(ticketRoot, "🧪️standard-subset-parent-context", "🔣️.json"), "utf8")) as {
  schemaVersion: number;
  vectors: { name: string; parentKindId: string | null; expectedKindId: string | null }[];
};

describe("standard and subset parent context", () => {
  test("resolves the language-agnostic vectors only beneath their collection owners", () => {
    const taxonomy = loadTaxonomy();
    expect(vectors.schemaVersion).toBe(1);
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    for (const vector of vectors.vectors) {
      const context = vector.parentKindId ? { parentKindId: vector.parentKindId } : {};
      expect(semanticDirectoryKindId(vector.name, taxonomy, context)).toBe(vector.expectedKindId);
    }
  });

  test("matches third-party discovery of canonical collection children", async () => {
    const fixture = mkdtempSync(join(ticketRoot, "🧪️standard-subset-context-"));
    try {
      mkdirSync(join(fixture, "🏅️standards", "🔖️1", "🪆️subsets", "✳️any"), { recursive: true });
      const discovered = await fastGlob(["🏅️standards/🔖️*", "🏅️standards/🔖️*/🪆️subsets/✳️*"], { cwd: fixture, onlyDirectories: true });
      const taxonomy = loadTaxonomy();
      expect(discovered.sort()).toEqual(["🏅️standards/🔖️1", "🏅️standards/🔖️1/🪆️subsets/✳️any"]);
      expect(semanticDirectoryKindId(basename(discovered[0]!), taxonomy, { parentKindId: "standards" })).toBe("standard");
      expect(semanticDirectoryKindId(basename(discovered[1]!), taxonomy, { parentKindId: "subsets" })).toBe("subset");
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });

  test("rejects an exact kind-id slug beneath the wrong physical parent", () => {
    const fixture = mkdtempSync(join(ticketRoot, "🧪️standard-subset-exact-"));
    try {
      const wrong = join(fixture, "🧪️tests", "🔖️standard");
      mkdirSync(wrong, { recursive: true });
      mkdirSync(join(fixture, "🗿️artifacts", "📄txt", "🔖️utf-8", "✳️any"), { recursive: true });
      const scope = relative(repoRoot, fixture).replaceAll("\\", "/");
      const inventory = inventoryTaxonomy({ repoRoot, scope, ticketDir: fixture });
      const entry = inventory.entries.find((candidate) => candidate.sourcePath === `${scope}/🧪️tests/🔖️standard`);
      expect(entry?.violations.map((violation) => violation.code)).toContain("directory-kind-unresolved");
      for (const suffix of ["🗿️artifacts/📄txt", "🗿️artifacts/📄txt/🔖️utf-8", "🗿️artifacts/📄txt/🔖️utf-8/✳️any"]) {
        const foreign = inventory.entries.find((candidate) => candidate.sourcePath === `${scope}/${suffix}`);
        expect(foreign?.violations.map((violation) => violation.code)).not.toContain("directory-kind-unresolved");
      }
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️StandardSubsetParentContext
