import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import fastGlob from "fast-glob";
import picomatch from "picomatch";
import { fixedFilenameContractIdsForPath, loadTaxonomy, taxonomyPathPatternMatches, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️ExternalFixedFilenames
const ticketRoot = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = join(ticketRoot, "../../../../../../../");
const golden = JSON.parse(readFileSync(join(ticketRoot, "🧪️external-fixed-filenames", "🔣️.json"), "utf8")) as {
  schemaVersion: number;
  mappings: { sourcePath: string; destinationPath: string; contractId: string; authority: string; contentSha256: string }[];
  referenceFiles: { path: string; forbiddenToken: string; requiredToken: string }[];
};
const sha256 = (path: string): string => createHash("sha256").update(readFileSync(path)).digest("hex");

describe("external fixed filenames", () => {
  const taxonomy = loadTaxonomy();

  test("keeps the language-neutral owner ledger and strict taxonomy coherent", () => {
    expect(golden.schemaVersion).toBe(1);
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    expect(golden.mappings.map((mapping) => mapping.contractId)).toEqual(["github-pages-cname", "caddyfile", "dockerfile"]);
  });

  test("discovers only canonical owner filenames with third-party parity", () => {
    const destinations = golden.mappings.map((mapping) => mapping.destinationPath);
    expect(fastGlob.sync(destinations, { cwd: repoRoot, dot: true, onlyFiles: true }).sort()).toEqual([...destinations].sort());
    expect(fastGlob.sync(golden.mappings.map((mapping) => mapping.sourcePath), { cwd: repoRoot, dot: true, onlyFiles: true })).toEqual([]);
    for (const mapping of golden.mappings) {
      const contract = taxonomy.fixedFilenameContracts[mapping.contractId]!;
      expect(contract.authority).toBe(mapping.authority);
      expect(fixedFilenameContractIdsForPath(mapping.destinationPath, taxonomy)).toEqual([mapping.contractId]);
      expect(fixedFilenameContractIdsForPath(mapping.sourcePath, taxonomy)).toEqual([]);
      expect(taxonomyPathPatternMatches(mapping.destinationPath, contract.pathPattern)).toBe(picomatch(contract.pathPattern)(mapping.destinationPath));
      expect(sha256(join(repoRoot, mapping.destinationPath))).toBe(mapping.contentSha256);
    }
  });

  test("contains no live emoji-prefixed external filename reference", () => {
    for (const reference of golden.referenceFiles) {
      const content = readFileSync(join(repoRoot, reference.path), "utf8");
      expect(content).not.toContain(reference.forbiddenToken);
      expect(content).toContain(reference.requiredToken);
    }
  });
});
//#endregion 🧪️ExternalFixedFilenames
