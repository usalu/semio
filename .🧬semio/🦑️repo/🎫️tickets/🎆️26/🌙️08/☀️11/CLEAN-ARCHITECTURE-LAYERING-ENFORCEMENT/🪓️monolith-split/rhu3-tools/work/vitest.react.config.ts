/** 🧪️ Test-only wrapper of the renderer vitest config: the four wasm engine package aliases match their whole subpath tree (prefix aliasing turned `pkg/file.js` into `<stub>/file.js`). Used for both the before and the after run. */
import base from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";
const WASM = new Set(["@semio-tech/framework-os-node-graph-rs", "@semio-tech/framework-surface-rs", "@semio-tech/framework-editor-rs", "@semio-tech/flow-core"]);
const alias = (base.resolve?.alias as { find: string | RegExp; replacement: string }[]).map((entry) => typeof entry.find === "string" && WASM.has(entry.find) ? { find: new RegExp(`^${entry.find.replace(/[.*+?^${}()|[\]\\/]/g, "\\$&")}(?:/.*)?$`, "u"), replacement: entry.replacement } : entry);
export default { ...base, resolve: { ...base.resolve, alias } };
