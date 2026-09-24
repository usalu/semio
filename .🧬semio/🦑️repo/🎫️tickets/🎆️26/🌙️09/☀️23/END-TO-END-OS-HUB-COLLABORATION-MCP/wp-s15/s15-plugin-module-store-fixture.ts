/** 🗄️ S15: generates the language-agnostic plugin module STORE fixture (`🌎️hub-source/🧫️fixtures/🗄️store/🔣️.json`): two catalog
 * generations of note (the bridge changed) and one of draw, sharing their vendored files, with their canonical manifests, the
 * store's module URLs, hostile paths, serve cases and collection cases. Built through the TS twin's own encoder. */
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { blake3Hex } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔏️hash/🟦️.ts";
import { encodeTrustedPluginModuleBundleV1, utf8OrderV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🟦️.ts";

const out = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧫️fixtures/🗄️store/🔣️.json";
const enc = new TextEncoder();
const sha = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const hex = (bytes: Uint8Array) => Buffer.from(bytes).toString("hex");
const vendor = { "🪞️vendor/🔤️guestslim-typst-fonts.bin": enc.encode("fonts"), "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js": enc.encode("export const streams = {};\n") };
const module = (pluginId: string, directory: string, bridge: string) => {
  const descriptor = enc.encode(`descriptor ${pluginId}`);
  const contents: Record<string, Uint8Array> = {
    ...vendor,
    [`${directory}/semio_s_plugin_${pluginId}_component.core.wasm`]: Uint8Array.from([0, 97, 115, 109, 1, 0, 0, 0]),
    [`${directory}/🌉️bridge.js`]: enc.encode(bridge),
    [`${directory}/🔣️.json`]: enc.encode(JSON.stringify({ manifest: { pluginId } })),
    [`${directory}/🛂️.descriptor.semio`]: descriptor,
  };
  const entry = { pluginId, packageId: `semio:${pluginId}`, version: "0.1.0", componentSha256: sha(enc.encode(`component ${pluginId} ${bridge}`)), descriptorByteSha256: sha(descriptor), dependencies: [], dialectArtifactKinds: [`s.${pluginId}.${pluginId === "note" ? "note" : "drawing"}`], extendsPluginId: null, bundleSha256: "", bundleByteLength: 0, entry: `${directory}/🌉️bridge.js` };
  const files = Object.keys(contents).sort(utf8OrderV1).map((path) => ({ path, byteLength: contents[path]!.byteLength, sha256: sha(contents[path]!), blake3: blake3Hex(contents[path]!) }));
  const manifest = encodeTrustedPluginModuleBundleV1({ schema: "semio.hub.trusted-plugin-module/v1", pluginId, packageId: entry.packageId, version: "0.1.0", sourceComponentSha256: entry.componentSha256, sourceDescriptorByteSha256: entry.descriptorByteSha256, moduleDirectory: directory, entry: entry.entry, files });
  entry.bundleSha256 = sha(manifest);
  entry.bundleByteLength = manifest.byteLength;
  return { entry, manifestUtf8: new TextDecoder().decode(manifest), contents: Object.fromEntries(Object.entries(contents).map(([path, bytes]) => [path, hex(bytes)])) };
};
const generationA = "a1".repeat(32), generationB = "b2".repeat(32), generationC = "c3".repeat(32);
const noteA = module("note", "🗒️note", "export async function createActorApi() { return 1; }\n");
const noteB = module("note", "🗒️note", "export async function createActorApi() { return 2; }\n");
const drawB = module("draw", "🖍️draw", "export async function createActorApi() { return 3; }\n");
const bundles = { noteA, noteB, drawB };
const record = (generationId: string, bundle: keyof typeof bundles) => ({ schema: "semio.os.plugin-module-store-record/v1", generationId, entry: bundles[bundle].entry, installedAtMs: 1_790_000_000_000 });
const url = (generationId: string, bundle: keyof typeof bundles, path: string) => `/_semio/plugin-modules/${generationId}/${bundles[bundle].entry.bundleSha256}/${path.split("/").map(encodeURIComponent).join("/")}`;
const stored = { noteA: { generationId: generationA, bundle: "noteA" }, noteB: { generationId: generationB, bundle: "noteB" }, drawB: { generationId: generationB, bundle: "drawB" } };
const fixture = {
  schema: "semio.os.plugin-module-store-fixture/v1",
  origin: "http://semio.test",
  generations: { a: generationA, b: generationB, c: generationC },
  bundles,
  records: { noteA: record(generationA, "noteA"), noteB: record(generationB, "noteB"), drawB: record(generationB, "drawB") },
  urls: [
    { bundle: "noteB", generationId: generationB, path: "🗒️note/🌉️bridge.js", url: url(generationB, "noteB", "🗒️note/🌉️bridge.js") },
    { bundle: "drawB", generationId: generationB, path: "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js", url: url(generationB, "drawB", "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js") },
  ],
  hostilePaths: [
    "/_semio/plugin-modules/", `/_semio/plugin-modules/${generationB}`, `/_semio/plugin-modules/${generationB}/${noteB.entry.bundleSha256}`,
    `/_semio/plugin-modules/${generationB}/${noteB.entry.bundleSha256}/%F0%9F%97%92%EF%B8%8Fnote/..%2F..%2Fx.js`, `/_semio/plugin-modules/${generationB}/${noteB.entry.bundleSha256}/a//b.js`,
    `/_semio/plugin-modules/${"B2".repeat(32)}/${noteB.entry.bundleSha256}/x.js`, `/_semio/plugin-modules/${generationB}/nothex/x.js`, `/_semio/plugin-modules/${generationB}/${noteB.entry.bundleSha256}/%E0%A4%A.js`,
    `/_semio/other/${generationB}/${noteB.entry.bundleSha256}/x.js`,
  ],
  serveCases: [
    { id: "every-file", store: ["noteB"], change: "none", paths: Object.keys(noteB.contents), status: 200, remove: [] },
    { id: "media-types", store: ["noteB"], change: "none", paths: ["🗒️note/🌉️bridge.js", "🗒️note/semio_s_plugin_note_component.core.wasm", "🗒️note/🔣️.json", "🗒️note/🛂️.descriptor.semio"], contentTypes: ["text/javascript; charset=utf-8", "application/wasm", "application/json", "application/octet-stream"], status: 200, remove: [] },
    { id: "tampered-file", store: ["noteB"], change: "tamper:🗒️note/🌉️bridge.js", paths: ["🗒️note/🌉️bridge.js"], status: 404, problem: "plugin-module-store.corrupt", remove: ["record:noteB", "blob:noteB:🗒️note/🌉️bridge.js"] },
    { id: "missing-file", store: ["noteB"], change: "drop:🗒️note/semio_s_plugin_note_component.core.wasm", paths: ["🗒️note/🔣️.json"], status: 200, remove: [] },
    { id: "missing-served-file", store: ["noteB"], change: "drop:🗒️note/🔣️.json", paths: ["🗒️note/🔣️.json"], status: 404, problem: "plugin-module-store.corrupt", remove: ["record:noteB"] },
    { id: "tampered-manifest", store: ["noteB"], change: "tamper-manifest:noteB", paths: ["🗒️note/🌉️bridge.js"], status: 404, problem: "plugin-module-store.corrupt", remove: ["record:noteB", "manifest:noteB"] },
    { id: "other-generation", store: ["noteB"], change: "none", generationId: generationA, paths: ["🗒️note/🌉️bridge.js"], status: 404, problem: "plugin-module-store.missing", remove: [] },
    { id: "unlisted-path", store: ["noteB"], change: "none", paths: ["🗒️note/unlisted.js"], status: 404, problem: "plugin-module-store.missing", remove: [] },
  ],
  collectCases: [
    { id: "superseded-generation-released", store: ["noteA", "noteB", "drawB"], current: generationB, held: [], removedRecords: ["noteA"], keptRecords: ["noteB", "drawB"] },
    { id: "superseded-generation-still-running", store: ["noteA", "noteB", "drawB"], current: generationB, held: ["noteA"], removedRecords: [], keptRecords: ["noteA", "noteB", "drawB"] },
    { id: "everything-superseded", store: ["noteA", "noteB", "drawB"], current: generationC, held: [], removedRecords: ["noteA", "noteB", "drawB"], keptRecords: [] },
    { id: "nothing-superseded", store: ["noteB", "drawB"], current: generationB, held: [], removedRecords: [], keptRecords: ["noteB", "drawB"] },
  ],
  stored,
};
mkdirSync(out.replace(/\/[^/]+$/u, ""), { recursive: true });
writeFileSync(out, `${JSON.stringify(fixture, null, 2)}\n`);
console.log(`store fixture: bundles=${Object.keys(bundles).length} serve=${fixture.serveCases.length} collect=${fixture.collectCases.length}`);
