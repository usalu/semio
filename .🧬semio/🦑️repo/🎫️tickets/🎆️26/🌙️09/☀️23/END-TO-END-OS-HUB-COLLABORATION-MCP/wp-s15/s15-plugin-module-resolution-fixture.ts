/** 🔍️ S15: generates the language-agnostic hub program RESOLUTION fixture (`🌎️hub-source/🧫️fixtures/🔍️resolution/🔣️.json`):
 * program ids, staged-module roots, the source decision (store / local / hub) for staged modules that match, differ or are
 * missing, the owner of a dialect and the same-generation closure of a hub program. Bundles are the store fixture's own. */
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";

const root = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧫️fixtures";
const store = JSON.parse(readFileSync(`${root}/🗄️store/🔣️.json`, "utf8"));
const out = `${root}/🔍️resolution/🔣️.json`;
const noteB = store.bundles.noteB;
const manifest = JSON.parse(noteB.manifestUtf8);
const core = "🗒️note/semio_s_plugin_note_component.core.wasm";
const bridge = "🗒️note/🌉️bridge.js";
const vendorJs = "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js";
const staged = (edit: (files: Record<string, string | null>) => void = () => {}) => {
  const files: Record<string, string | null> = { ...noteB.contents };
  edit(files);
  return files;
};
const flip = (hex: string) => (Number.parseInt(hex.slice(0, 2), 16) ^ 1).toString(16).padStart(2, "0") + hex.slice(2);
const localEntryUrl = `/plugin-modules/${encodeURIComponent("🗒️note")}/${encodeURIComponent("🌉️bridge.js")}?v=1790000000000`;
const sourceCases = [
  { id: "matching", stored: "none", local: { moduleUrl: localEntryUrl, files: staged() }, source: "local" },
  { id: "matching-with-extra-files", stored: "none", local: { moduleUrl: localEntryUrl, files: staged((files) => { files["🗒️note/extra.js"] = "00"; }) }, source: "local" },
  { id: "differing-core", stored: "none", local: { moduleUrl: localEntryUrl, files: staged((files) => { files[core] = flip(files[core]!); }) }, source: "hub" },
  { id: "differing-vendor", stored: "none", local: { moduleUrl: localEntryUrl, files: staged((files) => { files[vendorJs] = flip(files[vendorJs]!); }) }, source: "hub" },
  { id: "longer-bridge", stored: "none", local: { moduleUrl: localEntryUrl, files: staged((files) => { files[bridge] = `${files[bridge]}0a`; }) }, source: "hub" },
  { id: "missing-file", stored: "none", local: { moduleUrl: localEntryUrl, files: staged((files) => { files[bridge] = null; }) }, source: "hub" },
  { id: "missing-staging", stored: "none", local: null, source: "hub" },
  { id: "other-entry", stored: "none", local: { moduleUrl: `/plugin-modules/${encodeURIComponent("🗒️note")}/index.js`, files: staged() }, source: "hub" },
  { id: "stored-complete-local-differing", stored: "complete", local: { moduleUrl: localEntryUrl, files: staged((files) => { files[core] = flip(files[core]!); }) }, source: "store" },
  { id: "stored-complete-no-staging", stored: "complete", local: null, source: "store" },
  { id: "stored-incomplete-local-matching", stored: "incomplete", local: { moduleUrl: localEntryUrl, files: staged() }, source: "local" },
  { id: "stored-incomplete-local-differing", stored: "incomplete", local: { moduleUrl: localEntryUrl, files: staged((files) => { files[bridge] = flip(files[bridge]!); }) }, source: "hub" },
];
const bundleSha256 = noteB.entry.bundleSha256;
const programIds = [
  { pluginId: "note", bundleSha256, programId: `note@${bundleSha256}` },
  { pluginId: "cad-extension-aec-building", bundleSha256, programId: `cad-extension-aec-building@${bundleSha256}` },
  { pluginId: "semio@scoped", bundleSha256, programId: `semio@scoped@${bundleSha256}` },
];
const hostileProgramIds = ["note", `note@${bundleSha256.slice(1)}`, `note@${bundleSha256.toUpperCase()}`, `@${bundleSha256}`, ` note@${bundleSha256}`, `note @${bundleSha256}x`, `note#${bundleSha256}`];
const roots = [
  { moduleUrl: localEntryUrl, entry: bridge, root: "/plugin-modules/" },
  { moduleUrl: `http://127.0.0.1:6541/plugin-modules/🗒️note/🌉️bridge.js#top`, entry: bridge, root: "http://127.0.0.1:6541/plugin-modules/" },
  { moduleUrl: `/plugin-modules/${encodeURIComponent("🗒️note")}/other.js`, entry: bridge, root: null },
  { moduleUrl: `/plugin-modules/x${encodeURIComponent("🗒️note")}/${encodeURIComponent("🌉️bridge.js")}`, entry: bridge, root: null },
  { moduleUrl: "/plugin-modules/%E0%A4%A/🌉️bridge.js", entry: bridge, root: null },
];
const entry = (pluginId: string, dependencies: string[], dialectArtifactKinds: string[], extendsPluginId: string | null) => ({
  ...noteB.entry,
  pluginId,
  packageId: `semio:${pluginId}`,
  dependencies,
  dialectArtifactKinds,
  extendsPluginId,
  entry: `${pluginId}/🌉️bridge.js`,
});
const index = {
  schema: "semio.hub.trusted-plugin-module-index/v1",
  generationId: store.generations.b,
  modules: [
    entry("draw", [], ["s.draw.drawing"], null),
    entry("flow", ["stdio"], ["s.flow.graph"], null),
    entry("flow-extension-brep", ["flow", "stdio"], [], "flow"),
    entry("flow-extension-brep-mesh", ["flow-extension-brep"], [], "flow-extension-brep"),
    entry("gis", ["stdio"], ["s.gis.gismap", "s.gis.gisterrain"], null),
    entry("loop-a", ["loop-b"], ["s.loop.a"], null),
    entry("loop-b", ["loop-a"], [], null),
    entry("orphan", ["missing"], ["s.orphan.orphan"], null),
    entry("shared-a", [], ["s.shared.kind"], null),
    entry("shared-b", [], ["s.shared.kind"], null),
    entry("stdio", [], ["s.stdio.csv", "s.stdio.txt"], null),
  ],
};
const closureCases = [
  { id: "plugin-without-dependencies", pluginId: "draw", closure: ["draw"] },
  { id: "plugin-with-dependency", pluginId: "gis", closure: ["stdio", "gis"] },
  { id: "plugin-with-extensions", pluginId: "flow", closure: ["stdio", "flow", "flow-extension-brep", "flow-extension-brep-mesh"] },
  { id: "dependency-does-not-pull-its-dependents", pluginId: "stdio", closure: ["stdio"] },
  { id: "extension-alone", pluginId: "flow-extension-brep", closure: ["stdio", "flow", "flow-extension-brep", "flow-extension-brep-mesh"] },
  { id: "dependency-cycle", pluginId: "loop-a", closure: null },
  { id: "missing-dependency", pluginId: "orphan", closure: null },
  { id: "unknown-plugin", pluginId: "nothing", closure: null },
];
const ownerCases = [
  { artifactKind: "s.gis.gisterrain", owner: "gis" },
  { artifactKind: "s.stdio.csv", owner: "stdio" },
  { artifactKind: "s.shared.kind", owner: null },
  { artifactKind: "s.none.none", owner: null },
];
const fixture = {
  schema: "semio.os.plugin-module-resolution-fixture/v1",
  generationId: store.generations.b,
  bundle: { entry: noteB.entry, manifestUtf8: noteB.manifestUtf8, contents: noteB.contents },
  sourceCases,
  programIds,
  hostileProgramIds,
  roots,
  index,
  closureCases,
  ownerCases,
};
if (manifest.files.length !== Object.keys(noteB.contents).length) throw new Error("store fixture bundle and contents disagree");
mkdirSync(out.replace(/\/[^/]+$/u, ""), { recursive: true });
writeFileSync(out, `${JSON.stringify(fixture, null, 2)}\n`);
console.log(`resolution fixture: sources=${sourceCases.length} programIds=${programIds.length}+${hostileProgramIds.length} roots=${roots.length} closures=${closureCases.length} owners=${ownerCases.length}`);
