#!/usr/bin/env bun
/**
 * 📇️ Slice L-S2 probe: census of the registry catalog input set (the paths `repo:generator-inputs` digests) and the
 * generator-contract problems `plugin-registry:check` would raise, printed as one JSON line for before/after comparison.
 */
import { createHash } from "node:crypto";
import { resolve } from "node:path";
import { loadCatalogTaxonomy, registryCatalogInputPaths, registryCatalogInputView, validateGeneratorContractsAgainstWorkspace } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../.."), taxonomy = loadCatalogTaxonomy(), started = performance.now();
const paths = registryCatalogInputPaths(root, taxonomy, registryCatalogInputView(root, taxonomy));
const manifests = paths.filter((path) => path === "📋️project.json" || path.endsWith("/📋️project.json"));
console.log(JSON.stringify({
  inputs: paths.length,
  projectManifests: manifests.length,
  projectManifestSample: manifests.slice(0, 6),
  pathSetSha256: createHash("sha256").update(paths.join("\n")).digest("hex"),
  scanMs: Math.round(performance.now() - started),
  generatorContractProblems: validateGeneratorContractsAgainstWorkspace(root, taxonomy),
  registryContract: { inputPatterns: taxonomy.generatorContracts["plugin-registry"]!.inputPatterns.length, outputRoots: taxonomy.generatorContracts["plugin-registry"]!.outputRoots },
}));
