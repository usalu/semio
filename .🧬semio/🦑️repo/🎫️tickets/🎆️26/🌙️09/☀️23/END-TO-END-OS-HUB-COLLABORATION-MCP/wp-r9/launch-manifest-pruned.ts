#!/usr/bin/env bun
/** ✂️ R9: the launch coverage law's pruned registry-catalog walk, timed outside vitest. */
const ROOT = "/Users/ueli/Documents/semio";
const { declaredProjectTargets } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
const { loadCatalogTaxonomy, registryCatalogInputPaths, registryCatalogInputView } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`);
let started = performance.now();
const manifests = (declaredProjectTargets(ROOT) as { path: string }[]).map(({ path }) => (path ? `${path}/📋️project.json` : "📋️project.json"));
const launchMs = Math.round(performance.now() - started);
started = performance.now();
const taxonomy = loadCatalogTaxonomy(), base = registryCatalogInputView(ROOT, taxonomy);
const reachable = new Set(manifests.flatMap((manifest) => manifest.split("/").map((_, index, segments) => segments.slice(0, index + 1).join("/"))));
let entryCalls = 0;
const inputs = new Set(registryCatalogInputPaths(ROOT, taxonomy, {
  kind: (path: string) => base.kind(path),
  readText: (path: string) => base.readText(path),
  entries: (path: string) => { entryCalls += 1; return base.entries(path).filter((entry: { name: string }) => reachable.has(path ? `${path}/${entry.name}` : entry.name)); },
}) as string[]);
const discoveryMs = Math.round(performance.now() - started);
console.log(JSON.stringify({ launchMs, discoveryMs, entryCalls, inputs: inputs.size, manifests: manifests.length, missing: manifests.filter((manifest) => !inputs.has(manifest)) }));
