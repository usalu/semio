#!/usr/bin/env bun
/** 🧭️ R9: which launch-projected Nx manifests the plugin-registry discovery already declares (content input, directory witness, neither). */
const ROOT = "/Users/ueli/Documents/semio";
const { declaredProjectTargets } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
const { registryCatalogInputPaths } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`);
let started = performance.now();
const manifests = (declaredProjectTargets(ROOT) as { path: string }[]).map(({ path }) => (path ? `${path}/📋️project.json` : "📋️project.json"));
const launchMs = Math.round(performance.now() - started);
started = performance.now();
const inputs = new Set(registryCatalogInputPaths(ROOT) as string[]);
const discoveryMs = Math.round(performance.now() - started);
const content = manifests.filter((path) => inputs.has(path));
const witnessed = manifests.filter((path) => !inputs.has(path) && inputs.has(path.split("/").slice(0, -1).join("/")));
const neither = manifests.filter((path) => !inputs.has(path) && !inputs.has(path.split("/").slice(0, -1).join("/")));
console.log(JSON.stringify({ launchMs, discoveryMs, inputs: inputs.size, manifests: manifests.length, content: content.length, witnessed: witnessed.length, neither: neither.length, neitherSample: neither.slice(0, 40) }, null, 1));
