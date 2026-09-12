import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const wgpu = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const taxonomy = JSON.parse(readFileSync(join(root, library, "🔣️taxonomy.json"), "utf8"));
if (process.argv.includes("boot-test")) {
  const { testWgpuBootInputs } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts"));
  await testWgpuBootInputs(root, process.env.SEMIO_TEST_ARTIFACT_DIR!);
  process.exit(0);
}
const projection = await import(join(root, wgpu, "📽️projection/🟦️.ts"));
const contract = taxonomy.generatorContracts["wgpu-frame-worker"];
if (process.argv.includes("sources")) {
  const paths = new Set<string>();
  for (const entry of contract.packageGeneration.browserProfile.entries) {
    const result = await Bun.build({ entrypoints: [join(root, wgpu, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "native-input-census", setup(build) { build.onLoad({ filter: /.*/ }, args => { paths.add(args.path.slice(root.length + 1).replaceAll("\\", "/")); return undefined; }); } }] });
    assert.equal(result.success, true, result.logs.map(String).join("\n"));
  }
  const current = contract.packageGeneration.browserProfile.sourceModulePaths;
  const sources = [...paths].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  const delta = { added: sources.filter(path => !current.includes(path)), removed: current.filter((path: string) => !paths.has(path)) };
  writeFileSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "wgpu-native-browser-sources.json"), JSON.stringify(sources, null, 2));
  console.log("[DEBUG] Native Bun observed " + sources.length + " source modules; delta " + JSON.stringify(delta));
  process.exit(0);
}
const results: any[] = [];
for (const target of ["generate-browser-boot", "generate-frame-worker"]) {
  const producerTarget = "@semio-tech/framework-renderer-wgpu:" + target;
  const rendered = await projection.renderWgpuPackageArtifacts(root, { producerTarget });
  assert.equal(rendered.nodes.length, 1);
  assert.equal(rendered.nodes[0].path, contract.outputRoots.find((output: any) => output.producer?.target === producerTarget).path);
  results.push({ producerTarget, paths: rendered.nodes.map((node: any) => node.path), inputs: rendered.inputs, bytes: rendered.nodes.map((node: any) => Buffer.byteLength(node.content)) });
  console.log("[DEBUG] Selected WGPU producer " + target + " renders one artifact from " + rendered.inputs.length + " inputs PASS");
}
writeFileSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "wgpu-projection-inputs.json"), JSON.stringify(results, null, 2));
const full = await projection.renderWgpuPackageArtifacts(root);
const parent = await projection.renderWgpuPackageArtifacts(root, { producerTarget: contract.target });
assert.equal(parent.nodes.length, 4);
assert.equal(full.nodes.length, 6);
const browser = await projection.renderWgpuBrowserBundles(root, contract.packageGeneration.browserProfile, { taxonomy });
assert.deepEqual([...parent.nodes, ...browser.nodes].sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path))), full.nodes);
console.log("[DEBUG] Four package outputs plus two browser outputs exactly match the six-node canonical preview PASS");
