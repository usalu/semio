/** 🧩️ S15: generates the language-agnostic trusted plugin module fixture (hub `🧫️fixtures/🧩️plugin-module/🔣️.json`). */
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
import { blake3Hex } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔏️hash/🟦️.ts";

const out = "/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧩️plugin-module/🔣️.json";
const enc = new TextEncoder();
const sha = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const utf8Order = (a: string, b: string) => Buffer.compare(Buffer.from(a, "utf8"), Buffer.from(b, "utf8"));
const contents: Record<string, Uint8Array> = {
  "🗒️note/semio_s_plugin_note_component.core.wasm": Uint8Array.from([0, 97, 115, 109, 1, 0, 0, 0]),
  "🗒️note/semio_s_plugin_note_component.js": enc.encode("export const reactor = {};\n"),
  "🗒️note/🌉️bridge.js": enc.encode("export async function createActorApi() {}\n"),
  "🗒️note/🔣️.json": enc.encode('{"manifest":{"pluginId":"note"}}\n'),
  "🗒️note/🛂️.descriptor.semio": enc.encode("semio-descriptor-fixture"),
  "🗒️note/🟨️.js": enc.encode("export function log() {}\n"),
  "🪞️vendor/🔤️guestslim-typst-fonts.bin": enc.encode("fonts"),
  "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js": enc.encode("export const streams = {};\n"),
};
const file = (path: string) => ({ path, byteLength: contents[path]!.byteLength, sha256: sha(contents[path]!), blake3: blake3Hex(contents[path]!) });
const paths = Object.keys(contents).sort(utf8Order);
const record = { pluginId: "note", packageId: "semio:note", version: "0.1.0", componentSha256: "11".repeat(32), descriptorByteSha256: sha(contents["🗒️note/🛂️.descriptor.semio"]!) };
const valid = {
  schema: "semio.hub.trusted-plugin-module/v1",
  pluginId: record.pluginId,
  packageId: record.packageId,
  version: record.version,
  sourceComponentSha256: record.componentSha256,
  sourceDescriptorByteSha256: record.descriptorByteSha256,
  moduleDirectory: "🗒️note",
  entry: "🗒️note/🌉️bridge.js",
  files: paths.map(file),
};
const canonical = JSON.stringify(valid) + "\n";
const bundleSha256 = sha(enc.encode(canonical));
const clone = () => JSON.parse(JSON.stringify(valid));
const edit = (id: string, schemaValid: boolean, accepted: boolean, mutate: (m: any) => void) => {
  const manifest = clone();
  mutate(manifest);
  return { id, schemaValid, accepted, manifest };
};
const at = (m: any, path: string) => m.files.find((row: any) => row.path === path);
const cases = [
  edit("valid", true, true, () => {}),
  edit("unknown-field", false, false, (m) => { m.note = 1; }),
  edit("wrong-schema", false, false, (m) => { m.schema = "semio.hub.trusted-plugin-module/v2"; }),
  edit("traversal-path", false, false, (m) => { at(m, "🗒️note/🟨️.js").path = "🗒️note/../🟨️.js"; }),
  edit("absolute-path", false, false, (m) => { at(m, "🗒️note/🟨️.js").path = "/🗒️note/🟨️.js"; }),
  edit("query-path", false, false, (m) => { at(m, "🗒️note/🟨️.js").path = "🗒️note/🟨️.js?v=1"; }),
  edit("module-directory-slash", false, false, (m) => { m.moduleDirectory = "🗒️note/sub"; }),
  edit("zero-length", false, false, (m) => { at(m, "🗒️note/🟨️.js").byteLength = 0; }),
  edit("uppercase-digest", false, false, (m) => { const row = at(m, "🗒️note/🟨️.js"); row.sha256 = row.sha256.toUpperCase(); }),
  edit("too-few-files", false, false, (m) => { m.files = m.files.slice(0, 2); }),
  edit("unsorted", true, false, (m) => { m.files.reverse(); }),
  edit("duplicate-path", true, false, (m) => { m.files.splice(1, 0, JSON.parse(JSON.stringify(m.files[1]))); }),
  edit("outside-roots", true, false, (m) => { m.files.push({ ...file("🪞️vendor/🔤️guestslim-typst-fonts.bin"), path: "🧵️shard/x.js" }); m.files.sort((a: any, b: any) => utf8Order(a.path, b.path)); }),
  edit("entry-not-bridge", true, false, (m) => { m.entry = "🗒️note/🟨️.js"; }),
  edit("entry-outside-directory", true, false, (m) => { m.entry = "🪞️vendor/🌉️bridge.js"; }),
  edit("entry-missing", true, false, (m) => { m.files = m.files.filter((row: any) => row.path !== "🗒️note/🌉️bridge.js"); }),
  edit("descriptor-json-missing", true, false, (m) => { m.files = m.files.filter((row: any) => row.path !== "🗒️note/🔣️.json"); }),
  edit("descriptor-pack-missing", true, false, (m) => { m.files = m.files.filter((row: any) => row.path !== "🗒️note/🛂️.descriptor.semio"); }),
  edit("descriptor-pack-differs", true, false, (m) => { at(m, "🗒️note/🛂️.descriptor.semio").sha256 = "44".repeat(32); }),
  edit("source-component-differs", true, false, (m) => { m.sourceComponentSha256 = "12".repeat(32); }),
  edit("source-descriptor-differs", true, false, (m) => { m.sourceDescriptorByteSha256 = "13".repeat(32); }),
  edit("identity-differs", true, false, (m) => { m.pluginId = "draw"; }),
  edit("version-differs", true, false, (m) => { m.version = "0.2.0"; }),
];
const entry = { pluginId: "note", packageId: "semio:note", version: "0.1.0", componentSha256: record.componentSha256, descriptorByteSha256: record.descriptorByteSha256, dependencies: ["stdio"], dialectArtifactKinds: ["s.note.note"], extendsPluginId: null as string | null, bundleSha256, bundleByteLength: enc.encode(canonical).byteLength, entry: "🗒️note/🌉️bridge.js" };
const index = { schema: "semio.hub.trusted-plugin-module-index/v1", generationId: "22".repeat(32), modules: [{ ...entry, pluginId: "draw", packageId: "semio:draw", entry: "🖍️draw/🌉️bridge.js", dependencies: [], dialectArtifactKinds: ["s.draw.drawing", "s.draw.symbol"] }, entry] };
const indexEdit = (id: string, schemaValid: boolean, accepted: boolean, mutate: (i: any) => void) => {
  const value = JSON.parse(JSON.stringify(index));
  mutate(value);
  return { id, schemaValid, accepted, index: value };
};
const indexCases = [
  indexEdit("valid", true, true, () => {}),
  indexEdit("unknown-field", false, false, (i) => { i.modules[0].extra = true; }),
  indexEdit("zero-bundle-length", false, false, (i) => { i.modules[0].bundleByteLength = 0; }),
  indexEdit("duplicate-dependency", false, false, (i) => { i.modules[1].dependencies = ["stdio", "stdio"]; }),
  indexEdit("missing-dialects", false, false, (i) => { delete i.modules[1].dialectArtifactKinds; }),
  indexEdit("duplicate-dialect", false, false, (i) => { i.modules[0].dialectArtifactKinds = ["s.draw.drawing", "s.draw.drawing"]; }),
  indexEdit("unsorted-dialects", true, false, (i) => { i.modules[0].dialectArtifactKinds = ["s.draw.symbol", "s.draw.drawing"]; }),
  indexEdit("unsorted", true, false, (i) => { i.modules.reverse(); }),
  indexEdit("duplicate-plugin", true, false, (i) => { i.modules[0].pluginId = "note"; }),
  indexEdit("extension", true, true, (i) => { i.modules.push({ ...i.modules[1], pluginId: "note-extension-ink", packageId: "semio:note-extension-ink", entry: "🖋️note-extension-ink/🌉️bridge.js", dependencies: ["note"], dialectArtifactKinds: [], extendsPluginId: "note" }); }),
  indexEdit("extends-outside-dependencies", true, false, (i) => { i.modules[1].extendsPluginId = "draw"; }),
  indexEdit("missing-extends", false, false, (i) => { delete i.modules[1].extendsPluginId; }),
  indexEdit("blank-extends", false, false, (i) => { i.modules[1].extendsPluginId = " "; }),
];
const hex = (bytes: Uint8Array) => Buffer.from(bytes).toString("hex");
const fixture = {
  schema: "semio.hub.trusted-plugin-module-fixture/v1",
  record,
  contents: Object.fromEntries(paths.map((path) => [path, hex(contents[path]!)])),
  canonical: { bytesUtf8: canonical, sha256: bundleSha256, blake3: blake3Hex(enc.encode(canonical)), pretty: JSON.stringify(valid, null, 2) + "\n" },
  cases,
  fileCases: [
    { id: "exact", path: "🗒️note/🟨️.js", contentHex: hex(contents["🗒️note/🟨️.js"]!), accepted: true },
    { id: "tampered", path: "🗒️note/🟨️.js", contentHex: hex(enc.encode("export function log() {}\n").map((byte, i) => (i === 0 ? byte ^ 1 : byte))), accepted: false },
    { id: "truncated", path: "🗒️note/🟨️.js", contentHex: hex(contents["🗒️note/🟨️.js"]!.slice(1)), accepted: false },
    { id: "appended", path: "🪞️vendor/🔤️guestslim-typst-fonts.bin", contentHex: hex(enc.encode("fonts!")), accepted: false },
  ],
  index: indexCases,
  contentTypes: [
    { path: "🗒️note/🌉️bridge.js", contentType: "text/javascript; charset=utf-8" },
    { path: "🗒️note/x.mjs", contentType: "text/javascript; charset=utf-8" },
    { path: "🗒️note/semio_s_plugin_note_component.core.wasm", contentType: "application/wasm" },
    { path: "🗒️note/🔣️.json", contentType: "application/json" },
    { path: "🗒️note/🛂️.descriptor.semio", contentType: "application/octet-stream" },
    { path: "🪞️vendor/🔤️guestslim-typst-fonts.bin", contentType: "application/octet-stream" },
  ],
};
const { mkdirSync } = await import("node:fs");
mkdirSync(out.replace(/\/[^/]+$/, ""), { recursive: true });
writeFileSync(out, JSON.stringify(fixture, null, 2) + "\n");
console.log(`plugin-module fixture: cases=${cases.length} indexCases=${indexCases.length} bundleSha256=${bundleSha256}`);
