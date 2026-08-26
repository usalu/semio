import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import fastGlob from "fast-glob";
import picomatch from "picomatch";
import {
  fixedDirectoryContractIdsForPath,
  fixedFilenameContractIdsForPath,
  loadTaxonomy,
  taxonomyPathPatternMatches,
  validateTaxonomy,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️FixedOwnerScope
const ticketRoot = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = join(ticketRoot, "../../../../../../../");
const golden = JSON.parse(readFileSync(join(ticketRoot, "🧪️fixed-owner-scope", "🔣️.json"), "utf8")) as {
  schemaVersion: number;
  cargoCachePaths: string[];
  blockedCachePaths: string[];
  nxManifestPaths: string[];
  blockedNxManifestPaths: string[];
};

describe("fixed owner scopes", () => {
  const taxonomy = loadTaxonomy();

  test("keeps the strict schema and exact current evidence coherent", () => {
    expect(golden.schemaVersion).toBe(1);
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    expect(fastGlob.sync([...golden.cargoCachePaths, ...golden.blockedCachePaths, ...golden.nxManifestPaths, ...golden.blockedNxManifestPaths], { cwd: repoRoot, dot: true, onlyFiles: true }).sort()).toEqual([...golden.cargoCachePaths, ...golden.blockedCachePaths, ...golden.nxManifestPaths, ...golden.blockedNxManifestPaths].sort());
  });

  test("resolves every Cargo target triple through its exact parent identity", () => {
    expect(golden.cargoCachePaths).toHaveLength(20);
    for (const path of golden.cargoCachePaths) {
      const triple = basename(dirname(path));
      const directoryId = `cargo-target-triple-${triple}`;
      const targetPath = dirname(path);
      expect(fixedDirectoryContractIdsForPath(targetPath, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([directoryId]);
      expect(fixedFilenameContractIdsForPath(path, taxonomy, { parentFixedDirectoryContractIds: [directoryId] })).toEqual([`cargo-cache-tag-${triple}`]);
      for (const contract of [taxonomy.fixedDirectoryContracts[directoryId]!, taxonomy.fixedFilenameContracts[`cargo-cache-tag-${triple}`]!]) {
        expect(taxonomyPathPatternMatches(contract === taxonomy.fixedDirectoryContracts[directoryId] ? targetPath : path, contract.pathPattern)).toBe(picomatch(contract.pathPattern)(contract === taxonomy.fixedDirectoryContracts[directoryId] ? targetPath : path));
      }
    }
  });

  test("keeps unowned cache paths blocked", () => {
    expect(golden.blockedCachePaths).toHaveLength(6);
    for (const path of golden.blockedCachePaths) expect(fixedFilenameContractIdsForPath(path, taxonomy)).toEqual([]);
    const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/TICKET/🧪️target-probe";
    expect(fixedDirectoryContractIdsForPath(`${ticket}/wasm32-wasi`, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([]);
    expect(fixedDirectoryContractIdsForPath("tmp/🧪️target-probe/wasm32-wasip2", taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([]);
  });

  test("resolves only manifests with an adjacent exact Nx project owner", () => {
    expect(golden.nxManifestPaths).toHaveLength(15);
    for (const path of golden.nxManifestPaths) {
      const sibling = `${dirname(path)}/📋️project.json`;
      expect(fastGlob.sync([path, sibling], { cwd: repoRoot, dot: true, onlyFiles: true }).sort()).toEqual([path, sibling].sort());
      const siblingIds = fixedFilenameContractIdsForPath(sibling, taxonomy);
      expect(siblingIds).toContain("nx-project-manifest");
      expect(fixedFilenameContractIdsForPath(path, taxonomy, { siblingFixedFilenameContractIds: siblingIds })).toEqual([basename(path) === "package.json" ? "nx-owned-node-package-manifest" : "nx-owned-typescript-config"]);
      const contract = taxonomy.fixedFilenameContracts[basename(path) === "package.json" ? "nx-owned-node-package-manifest" : "nx-owned-typescript-config"]!;
      expect(taxonomyPathPatternMatches(path, contract.pathPattern)).toBe(picomatch(contract.pathPattern)(path));
    }
    expect(golden.blockedNxManifestPaths).toHaveLength(1);
    for (const path of golden.blockedNxManifestPaths) expect(fixedFilenameContractIdsForPath(path, taxonomy)).toEqual([]);
    expect(fixedFilenameContractIdsForPath("counterfeit/package.json", taxonomy, { siblingFixedFilenameContractIds: ["project-json-lookalike"] })).toEqual([]);
  });
});
//#endregion 🧪️FixedOwnerScope
