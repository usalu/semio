#!/usr/bin/env bun
/**
 * 🔬️ Slice L-S2 probe: renders the registry catalog through a recording input view and reports every file the render
 * actually reads that the declared catalog input set (`registryCatalogInputPaths` plus the contract's `inputPatterns`)
 * does not cover — the proof that shrinking the input set loses no real input.
 */
import { resolve } from "node:path";
import { loadCatalogTaxonomy, registryCatalogInputPaths, registryCatalogInputView, type RegistryCatalogInputView } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { renderCatalogFiles } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../.."), taxonomy = loadCatalogTaxonomy(), base = registryCatalogInputView(root, taxonomy);
const reads = new Set<string>();
const recording: RegistryCatalogInputView = {
  kind: (path) => base.kind(path),
  entries: (path) => base.entries(path),
  readText: (path) => { reads.add(path); return base.readText(path); },
  readBytes: (path) => { reads.add(path); return base.readBytes(path); },
};
const rendered = renderCatalogFiles(root, recording, "exclude");
const declared = new Set([...registryCatalogInputPaths(root, taxonomy, base), ...taxonomy.generatorContracts["plugin-registry"]!.inputPatterns]);
const undeclared = [...reads].filter((path) => !declared.has(path)).sort();
const manifests = [...reads].filter((path) => path.endsWith("📋️project.json")).sort();
console.log(JSON.stringify({
  renderedFiles: Object.keys(rendered.files).length,
  entries: rendered.entries.length,
  withheld: rendered.diagnostics.length,
  playgrounds: rendered.playgrounds.length,
  catalogPlaygrounds: (JSON.parse(rendered.files["🚀️playgrounds.json"]!) as unknown[]).length,
  filesRead: reads.size,
  projectManifestsRead: manifests,
  declaredInputs: declared.size,
  readsOutsideDeclaredInputs: undeclared,
}, null, 1));
