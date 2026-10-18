import type {Spec} from "../tssplit.ts";
const MAT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization";
const spec: Spec = {
  orig: "bm",
  file: MAT + "/🟦️.ts",
  header: [7, 15],
  drop: [[29, 29]],
  redirect: {PREVIEW2_VENDOR_RELATIVE: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts", rewritePreview2ShimImportSource: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts"},
  children: [
    {folder: "🧾️contract", doc: "🧾️ Contract of the plugin web materialization: the generated file names, the shard worker byte, heartbeat, diagnostics and drive declarations that the generated worker interpolates, and the materialization context.", ranges: [[31, 90]]},
    {folder: "🔤️guestslim-fonts", doc: "🔤️ Guest slim typst fonts of the plugin web materialization: the staged font seed and its placement in one plugin module root.", ranges: [[92, 109]]},
    {folder: "🩹️preview2-shim-patches", doc: "🩹️ Preview2 shim patches of the plugin web materialization: the vendored shim copy, the guest log classification patch and the whole line release patch.", ranges: [[111, 194]]},
    {folder: "🧵️shard-worker-source", doc: "🧵️ Shard worker source of the plugin web materialization: the one package agnostic generated worker that every actor of a shard pool activates through.", ranges: [[196, 902]]},
    {folder: "🌉️component-bridge-source", doc: "🌉️ Component bridge source of the plugin web materialization: the generated jco component bridge that wraps one actor activation behind the actor api.", ranges: [[904, 1128]]},
    {folder: "🪶️wasm-opt", doc: "🪶️ Wasm opt of the plugin web materialization: the shipped optimization arguments, the selected tool and the blocking and asynchronous optimization of the extracted core modules.", ranges: [[1139, 1169], [1318, 1344]]},
    {folder: "🧬️jco-module-rewrites", doc: "🧬️ Jco module rewrites of the plugin web materialization: the async result lifting and the versioned host shim and core asset urls of a freshly transpiled jco module.", ranges: [[1171, 1288]]},
    {folder: "🪢️host-shim-source", doc: "🪢️ Host shim source of the plugin web materialization: the generated host shim that implements the pure and host async import interfaces of the actor world.", ranges: [[1369, 1562]]},
  ],
};
export default spec;
