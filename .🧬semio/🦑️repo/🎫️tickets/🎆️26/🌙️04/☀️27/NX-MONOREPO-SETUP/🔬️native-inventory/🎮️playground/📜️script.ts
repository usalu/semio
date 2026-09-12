import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!;
const { testPlaygroundInputView } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎮️playground-input-view/🟦️.ts"));
await testPlaygroundInputView(root);
if (process.argv.includes("sessions")) {
  const { default: assert } = await import("node:assert/strict");
  const registry = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
  const { buildPlaygroundSession } = await import(join(registry, "🎮️playground/🧭️session/🟦️.ts"));
  const { readGeneratedCatalogProjection } = await import(join(registry, "📖️catalog-view/🟦️.ts"));
  const projection = readGeneratedCatalogProjection(), start = performance.now();
  for (const row of projection.playgrounds) {
    const session = buildPlaygroundSession(row.variant);
    assert.deepEqual(session, buildPlaygroundSession(row.variant, projection));
    assert.equal(session.registryPluginId, row.pluginId);
  }
  console.log(`[DEBUG] Default and explicit projection sessions matched all ${projection.playgrounds.length} playgrounds in ${Math.round(performance.now() - start)}ms`);
}
if (process.argv.includes("runtime")) {
  const { default: assert } = await import("node:assert/strict");
  const registry = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
  const { generatePlaygroundRegistry } = await import(join(registry, "🎮️playground/🔎️discovery/🟦️.ts"));
  const { readGeneratedCatalogProjection } = await import(join(registry, "📖️catalog-view/🟦️.ts"));
  const start = performance.now(), rows = generatePlaygroundRegistry(root);
  assert.deepEqual(JSON.parse(JSON.stringify(rows)), readGeneratedCatalogProjection().playgrounds);
  console.log(`[DEBUG] Default discovery matched all ${rows.length} projected playgrounds in ${Math.round(performance.now() - start)}ms`);
}
