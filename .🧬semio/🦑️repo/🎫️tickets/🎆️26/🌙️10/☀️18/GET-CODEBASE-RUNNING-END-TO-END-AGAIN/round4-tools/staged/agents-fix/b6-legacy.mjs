import { load, set, edit } from "./lib.mjs";

const M = "🧰️framework/🔨️modules/🛂️manifest/";
const K = "🧰️framework/🔨️modules/🎠️kernel/";
const REG = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/";
export const L = {
  examples: M + "🗣️contributions-and-examples/🟦️.ts",
  index: M + "🟦️.ts",
  admission: K + "🛂️descriptor-admission/🟦️.ts",
  scopeTest: K + "🧪️tests/🔬️scope-contributions/🟦️.ts",
  assetsTest: K + "🧪️tests/📦️package-assets/🟦️.ts",
  audits: REG + "✅️catalog-verification/📚️navbar-example-audits/🟦️.ts",
  completeTest: REG + "🧪️tests/✅️catalog-complete/🟦️.ts",
  integrationTest: "✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧩️integration/🟦️.ts",
  mediaHost: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎬️MediaTransportHost/🟦️.tsx",
};

export function build() {
  const text = load(L.examples);
  const from = text.indexOf("/** 🧹️ Stamps `dialect` onto a descriptor example row");
  const toMarker = "/** 📚️ THE example-picker predicate";
  const to = text.indexOf(toMarker);
  if (from < 0 || to < from) throw Error("legacy block anchors missing");
  set(L.examples, text.slice(0, from) + text.slice(to));
  edit(L.examples, `import { dialectCoordinate, parseDialectCoordinate } from`, `import { dialectCoordinate } from`);
  edit(L.index, `export { dialectFromLegacyExampleAppId, exampleBodyAssetPrefix, examplesForApp, normalizeManifestExampleRow, normalizeManifestExamples } from`, `export { exampleBodyAssetPrefix, examplesForApp } from`);

  edit(L.admission, `import { normalizeManifestExamples } from "../../🛂️manifest/🟦️.ts";
`, ``);
  edit(L.admission, `    return { descriptor: descriptor as object, manifest: normalizeManifestExamples(manifest as PluginManifest) as PluginManifest };`, `    if ("examples" in manifest && Array.isArray(manifest.examples)) for (const row of manifest.examples) if (!row?.dialect) throw fault("plugin.descriptor-invalid", \`example \${String(row?.id ?? "?")} misses its dialect\`);
    return { descriptor: descriptor as object, manifest: manifest as PluginManifest };`);
  edit(L.admission, `async function fetchDescriptorDocument(`, `/** 📥️ Fetches and admits one descriptor document: a JSON object whose manifest names the requested owner and an app roster, and whose example rows all name the dialect they were authored for. */
async function fetchDescriptorDocument(`);

  edit(L.assetsTest, `      await expect(fetchDescriptorManifest("demo", "http://page.test/modules/demo/bridge.js")).rejects.toThrow("malformed package asset declaration");
    } finally`, `      await expect(fetchDescriptorManifest("demo", "http://page.test/modules/demo/bridge.js")).rejects.toThrow("malformed package asset declaration");
      const dialect = { artifactKind: "s.demo.demo", standard: "1", subset: "*" };
      serve({ manifest: { pluginId: "demo", apps: [], examples: [{ id: "authored", dialect, artifactJson: "semio …" }] } });
      expect((await fetchDescriptorManifest("demo", "http://page.test/modules/demo/bridge.js")).examples.map((row) => row.id)).toEqual(["authored"]);
      serve({ manifest: { pluginId: "demo", apps: [], examples: [{ id: "undialected", artifactJson: "semio …" }] } });
      await expect(fetchDescriptorManifest("demo", "http://page.test/modules/demo/bridge.js")).rejects.toThrow("example undialected misses its dialect");
    } finally`);

  const scope = load(L.scopeTest);
  const a = scope.indexOf('describe("normalizeManifestExamples"'), b = scope.indexOf('describe("scopeContributionsJson"');
  if (a < 0 || b < a) throw Error("scope test anchors missing");
  set(L.scopeTest, scope.slice(0, a) + `describe("examplesForApp", () => {
  it("offers a surface exactly the rows authored for its dialect and none to a surface without one", () => {
    const dialect = { artifactKind: "s.sourcing.curation", standard: "1", subset: "*" };
    const rows = [{ id: "demo", dialect, artifactJson: "semio …" }, { id: "draw", dialect: { ...dialect, artifactKind: "s.draw.drawing" }, artifactJson: "…" }, { id: "undialected", artifactJson: "…" }];
    expect(examplesForApp(rows, { dialect }).map((row) => row.id)).toEqual(["demo"]);
    expect(examplesForApp(rows, {})).toEqual([]);
  });
});

` + scope.slice(b));
  edit(L.scopeTest, `import { dialectFromLegacyExampleAppId, examplesForApp, normalizeManifestExampleRow, normalizeManifestExamples, type PluginManifest } from`, `import { examplesForApp, type PluginManifest } from`);

  edit(L.audits, `import { exampleBodyAssetPrefix, examplesForApp, normalizeManifestExamples, type PluginManifest } from`, `import { exampleBodyAssetPrefix, examplesForApp, type PluginManifest } from`);
  edit(L.audits, `  const admitted = normalizeManifestExamples(manifest);
  const problems: string[] = [];
  for (const app of admitted.apps ?? []) {`, `  const problems: string[] = [];
  for (const app of manifest.apps ?? []) {`);
  edit(L.audits, `    if (examplesForApp(admitted.examples ?? [], { dialect: row.dialect }).length === 0) {`, `    if (examplesForApp(manifest.examples ?? [], { dialect: row.dialect }).length === 0) {`);
  edit(L.audits, `  const admitted = normalizeManifestExamples(manifest);
  const declared =`, `  const declared =`);
  edit(L.audits, `  for (const example of admitted.examples ?? []) {
    const row = example as { readonly id?: string; readonly artifactJson?: string; readonly dialect?: { readonly artifactKind: string; readonly standard: string; readonly subset: string } };
    const payload = row.artifactJson;`, `  for (const example of manifest.examples ?? []) {
    const row = example as { readonly id?: string; readonly artifactJson?: string; readonly dialect?: { readonly artifactKind: string; readonly standard: string; readonly subset: string } };
    if (!row.dialect) {
      problems.push(\`\${manifest.pluginId}: example \${String(row.id ?? "?")} names no dialect, so no navbar picker offers it\`);
      continue;
    }
    const payload = row.artifactJson;`);

  edit(L.completeTest, `import { examplesForApp, normalizeManifestExamples } from "@semio-tech/framework";`, `import { examplesForApp } from "@semio-tech/framework";`);
  edit(L.integrationTest, `import { examplesForApp, normalizeManifestExamples, type PluginManifest } from`, `import { examplesForApp, type PluginManifest } from`);
  edit(L.integrationTest, `    const admitted = normalizeManifestExamples(manifest!);
    const editor = admitted.apps?.find(`, `    const editor = manifest!.apps?.find(`);
  edit(L.integrationTest, `examplesForApp(admitted.examples ?? [], editor as`, `examplesForApp(manifest!.examples ?? [], editor as`);

  edit(L.mediaHost, `/// <reference types="vitest/importMeta" />
`, ``);
}
