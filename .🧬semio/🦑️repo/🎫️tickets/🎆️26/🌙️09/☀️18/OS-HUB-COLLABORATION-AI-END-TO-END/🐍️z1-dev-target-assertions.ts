#!/usr/bin/env bun
/** 🔎️ Z1: runs the renderer/variant-selection assertions added to `⚡️cache-contracts` standalone, so
 * they are proven without waiting for that suite's native Trunk/Cargo fixtures. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const { resolveNxInvocation } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts`);

for (const [renderer, plugin, expected] of [
  ["react", "s", "@semio-tech/framework-os-dev:dev-s-react-dev"],
  ["wgpu", "s", "@semio-tech/framework-os-dev:dev-s-wgpu-dev"],
  ["react", "draw", "@semio-tech/framework-os-dev:dev-draw-react-dev"],
  ["wgpu", "draw", "@semio-tech/framework-os-dev:dev-draw-wgpu-dev"],
] as const) {
  process.env.SEMIO_RENDERER = renderer;
  process.env.SEMIO_PLUGIN = plugin;
  delete process.env.S_OS_PORT;
  assert.equal(resolveNxInvocation(["run", "workspace:dev", "--", plugin]).args[1], expected, `${plugin}/${renderer} must select its own renderer target`);
  assert.equal(resolveNxInvocation(["run", "@semio-tech/framework-os-dev:dev"]).args[1], expected, `the bare dev alias must honour SEMIO_PLUGIN=${plugin} instead of defaulting to s`);
  console.log(`ok  ${plugin}/${renderer} -> ${expected}`);
}

const rootTargets = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json"), "utf8")).targets as Record<string, { dependsOn?: readonly string[]; options?: { command?: string } }>;
assert.ok(!(rootTargets.dev?.options?.command ?? "").includes("🧊️wgpu"), "the framework-os-dev dev alias may not hard-code the wgpu browser server");
assert.deepEqual(rootTargets.dev?.dependsOn, undefined, "the framework-os-dev dev alias may not pin one renderer's activation");
console.log("ok  @semio-tech/framework-os-dev:dev is renderer-neutral");

const workspaceTargets = JSON.parse(readFileSync(join(ROOT, "📋️project.json"), "utf8")).targets as Record<string, { dependsOn?: readonly string[] }>;
for (const dependency of ["setup-git", "prepare", "repo-mcp:build", "@semio-tech/framework-os-mcp-rs:build"]) assert.ok(workspaceTargets.setup!.dependsOn!.includes(dependency), `workspace:setup must reach ${dependency}`);
assert.deepEqual(Bun.JSONC.parse(readFileSync(join(ROOT, ".devcontainer/devcontainer.json"), "utf8")).postCreateCommand, ["bun", "nx", "run", "workspace:setup"]);
console.log("ok  devcontainer postCreateCommand runs the full zero-touch setup graph");
