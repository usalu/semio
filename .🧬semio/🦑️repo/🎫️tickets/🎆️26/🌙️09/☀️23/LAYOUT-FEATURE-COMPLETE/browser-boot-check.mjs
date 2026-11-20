import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd();
const base = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
const { renderWgpuPackageArtifacts } = await import(pathToFileURL(join(root, base, "📽️projection", "🟦️.ts")).href);
const { checkWgpuBrowserClosure, generateWgpuBrowserClosure } = await import(pathToFileURL(join(root, base, "📽️projection", "🔭️browser-closure", "🟦️.ts")).href);
if (process.argv.includes("--update-closure")) console.log("closure projection", await generateWgpuBrowserClosure(root));
await checkWgpuBrowserClosure(root);
if (process.argv.includes("--named-input-gaps")) {
  const { readFileSync } = await import("node:fs");
  const { resolveWorkspaceTaxonomyAuthority } = await import("@semio-tech/repo-lib/inventory");
  const { renderWgpuBrowserBundles } = await import(pathToFileURL(join(root, base, "📽️projection", "🟦️.ts")).href);
  const taxonomy = JSON.parse(readFileSync(resolveWorkspaceTaxonomyAuthority(root).taxonomyPath, "utf8"));
  const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
  const project = JSON.parse(readFileSync(join(root, base, "📦️packages", "🟦️typescript", "📋️project.json"), "utf8"));
  const declared = project.namedInputs.browserBootSources;
  const paths = new Set(declared.filter(value => typeof value === "string").map(value => value.replace("{workspaceRoot}/", "")));
  const { inputs } = await renderWgpuBrowserBundles(root, profile, { taxonomy, entryIds: ["browser-boot", "renderer-boot"] });
  const missing = inputs.filter(path => !paths.has(path));
  const roots = [...new Set(Object.values(profile.workspaceImports).flatMap(binding => inputs.includes(binding.entryPath) ? [binding.manifestPath, binding.entryPath] : []))].filter(path => !paths.has(path)).sort();
  console.log(JSON.stringify({ missing, roots }));
  process.exit(0);
}
if (process.argv.includes("--package-authority")) {
  const { nodes } = await renderWgpuPackageArtifacts(root);
  assert.equal(nodes.filter(node => node.path.endsWith("/🟨️.js")).length, 3);
  console.log("WGPU package authority rendered", nodes.length, "owned paths");
  process.exit(0);
}
const failures = [];
for (const [entry, folder] of [["browser-boot", "🚀️browser-boot"], ["renderer-boot", "🎬️renderer-boot"], ["frame-worker", "🎞️frame-worker"]]) {
  try {
    const { nodes } = await renderWgpuPackageArtifacts(root, { producerTarget: "@semio-tech/framework-renderer-wgpu:generate-" + entry });
    const expectedPath = join(base, folder, "🤖️generated", "🟨️.js").replaceAll("\\", "/");
    const node = nodes.find(row => row.path === expectedPath);
    assert.ok(node?.content.length > 1024, "the scoped producer returns its complete JS bundle");
    console.log(entry, Buffer.byteLength(node.content), createHash("sha256").update(node.content).digest("hex"));
  } catch (error) {
    failures.push(error);
    console.error(entry, error instanceof Error ? error.message : error);
    if (error instanceof AggregateError) for (const nested of error.errors.slice(0, 12)) console.error(nested);
  }
}
if (failures.length !== 0) process.exitCode = 1;
if (process.argv.includes("--independent")) {
  const engine = join(root, base, "..", "..");
  const tests = join(engine, "🧪️tests", "🧊️wgpu-browser-boot-cache-inputs", "🟦️.ts");
  const { testWgpuBootInputs } = await import(pathToFileURL(tests).href);
  await testWgpuBootInputs(root, join(import.meta.dir, "🗑️generated"));
  console.log("independent esbuild browser boot law passed");
}
