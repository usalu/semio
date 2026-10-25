import { resolve } from "node:path";
const plugin = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw";
const fixture = await Bun.file(resolve(plugin, "🧪️tests/🧫️fixtures/🧪️publication-authority/🔣️.json")).json() as any;
for (const app of fixture.apps) {
  const path = resolve(plugin, app.reservedProducerSource);
  const producer = await Bun.file(path).text();
  console.log(app.owner, app.reservedProducerSource.slice(-80), producer.length, ["source:request.snapshot_read","NativeEncodeControl::resume","NativeDecodeControl::resume","DrawingLayerCloneWorkAuthority","fn next_close_capacity_byte_demand","fn terminal_is_empty"].filter((a) => !producer.includes(a)));
}
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
const text = await Bun.file(resolve(plugin, "📦️packages/🟦️typescript/📜️script.ts")).text();
const block = text.slice(text.indexOf("const ANCHORS = ["), text.indexOf("];", text.indexOf("const ANCHORS = [")));
const anchors = [...block.matchAll(/^\s*"((?:[^"\\]|\\.)*)",?$/gm)].map((m) => JSON.parse('"' + m[1] + '"'));
for (const app of fixture.apps) {
  const source = readModuleTreeSource(resolve(plugin, app.source));
  console.log("app", app.owner, "missing", anchors.filter((a) => !source.includes(a)));
}
