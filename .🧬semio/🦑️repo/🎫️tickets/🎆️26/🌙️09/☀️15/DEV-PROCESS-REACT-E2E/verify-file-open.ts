import assert from "node:assert/strict";
import fs from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const renderer = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/";
const pickerPath = renderer + "🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
const fixture = JSON.parse(fs.readFileSync(renderer + "🧪️tests/💾️media-export-and-natural-files/file-open.json", "utf8")).bound;
const { readSelectedFilesBoundedV1 } = await import(pathToFileURL(resolve(pickerPath)).href);
const within = new File([Uint8Array.from(fixture.octets)], fixture.name);
const above = new File([new Uint8Array(fixture.maximumBytes + 1)], fixture.oversized);
let slices = 0;
class MeasuredFile extends File {
  override slice(start?: number, end?: number, type?: string) {
    slices++;
    return super.slice(start, end, type);
  }
}
const measured = new MeasuredFile([within], within.name);
const progress: number[][] = [];
const cancel = new AbortController();
const control = { signal: cancel.signal, progress: (completed: number, total: number) => progress.push([completed, total]) };
await assert.rejects(readSelectedFilesBoundedV1([measured, above], "dataUrl", fixture.maximumBytes, control), { fileName: above.name, size: above.size, maximumBytes: fixture.maximumBytes });
assert.equal(slices, 0);
const opened = await readSelectedFilesBoundedV1([measured], "dataUrl", fixture.maximumBytes, control);
assert.deepEqual(opened, [{ name: measured.name, contents: "data:application/octet-stream;base64," + Buffer.from(fixture.octets).toString("base64") }]);
assert.equal(slices, 1);
assert.deepEqual(progress.at(-1), [fixture.maximumBytes, fixture.maximumBytes]);
cancel.abort();
await assert.rejects(readSelectedFilesBoundedV1([measured], "dataUrl", fixture.maximumBytes, control), {name: "AbortError"});
assert.equal(slices, 1);
console.log("[DEBUG] Bounded picker refuses before reads, matches independent Buffer encoding, reports completion, and honors cancellation.");
const wireFixturePath = "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/📤️file-open-import/🔣️.json";
const wireFixture = JSON.parse(fs.readFileSync(wireFixturePath, "utf8"));
const { wireEffectToFriendly } = await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts")).href);
const bounded = wireFixture.wireCases.find((row: { id: string }) => row.id === "bounded-file-open");
assert.deepEqual(wireEffectToFriendly(bounded.effect, () => assert.fail("unexpected packed arguments")), bounded.friendly);
for (const invalid of [-1, 1.5, 0x100000000, "24576"]) {
  const effect = structuredClone(bounded.effect);
  effect.val.params["max-file-bytes"].val = invalid;
  assert.equal(wireEffectToFriendly(effect, () => null), null);
}
console.log("[DEBUG] Shared bounded picker wire fixture normalizes correctly and refuses non-u32 limits.");
const { wireEffectToFriendly: retainedEffectToFriendly } = await import(pathToFileURL(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🪟️retained-ui-patch/🟦️.ts")).href);
assert.deepEqual(retainedEffectToFriendly(bounded.effect), bounded.friendly);
console.log("[DEBUG] React retained effects normalize the same bounded picker fixture.");
const processLimit = bounded.friendly.requestFileOpen.maxFileBytes;
const limitFile = new File([new Uint8Array(processLimit + 1)], "oversized.step");
await assert.rejects(readSelectedFilesBoundedV1([limitFile], "dataUrl", processLimit, { signal: new AbortController().signal, progress: () => {} }), { fileName: limitFile.name, size: processLimit + 1, maximumBytes: processLimit });
console.log("[DEBUG] The picker refuses a STEP file one byte over the Process3d cap.");
const { encodePackValue } = await import("@semio-tech/framework/packs");
const scoped = structuredClone(bounded.effect);
scoped.val.params.args = { tag: "some", val: encodePackValue({ target: "workpiece" }) };
assert.deepEqual(retainedEffectToFriendly(scoped), {
  requestFileOpen: { ...bounded.friendly.requestFileOpen, args: { target: "workpiece" } },
});
console.log("[DEBUG] Retained file-open effects preserve the typed limit and packed target arguments.");
const { hostEffectInvocationV1, importChunkArguments } = await import("@semio-tech/framework");
const commandScope = { pluginId: "process", appId: "s.process.process3d@1/*#editor", modeId: "edit", windowKindId: "main", windowInstanceId: "main" };
const commandId = "importModelFile";
const longName = "製".repeat(250) + ".step";
assert.equal(longName.length, 255);
assert.equal(Buffer.byteLength(longName, "utf8"), 755);
const ceilingFile = new File([new Uint8Array(processLimit)], longName);
const [sizedFile] = await readSelectedFilesBoundedV1([ceilingFile], "dataUrl", processLimit, { signal: new AbortController().signal, progress: () => {} });
assert.ok(sizedFile);
assert.equal(sizedFile.name, longName);
assert.equal(sizedFile.contents.length, 32805);
const commandArgs = importChunkArguments(sizedFile.name, { payload: sizedFile.contents, chunk: 0, chunkCount: 1 });
const command = hostEffectInvocationV1(commandScope, [commandId], commandId, commandArgs);
assert.equal(command.kind, "command");
const encodedCommand = encodePackValue(JSON.parse(JSON.stringify(command.invocation)));
assert.equal(encodedCommand.length, 33754);
assert.ok(encodedCommand.length <= 33856);
console.log("[DEBUG] The 24,576-byte import with a 255-unit Unicode filename packs to 33,754 bytes under the 33,856-byte import wire cap.");
