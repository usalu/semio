import fs from "node:fs";

const idx = fs.readFileSync("compose/js/index.ts", "utf8");
const idxHead = idx.split("//#region 🧩WasmGraphNamespace")[0] ?? idx;
const cqrsNames = new Set();
for (const re of [/^export (async )?function (\w+)/gm, /^export const (\w+)/gm, /^export class (\w+)/gm]) {
  let m;
  while ((m = re.exec(idxHead))) cqrsNames.add(m[2] || m[1]);
}

const wasm = fs.readFileSync("compose/js/kit-wasm-store.ts", "utf8");
const names = new Set();
for (const re of [/^export (async )?function (\w+)/gm, /^export const (\w+)/gm, /^export class (\w+)/gm]) {
  let m;
  while ((m = re.exec(wasm))) names.add(m[2] || m[1]);
}
const omitReexport = new Set(["KIT_EVENT_STREAM_SUBSCRIPTION", "kitReadPointKey", "theKitReadPoint"]);
const alias = new Map([
  ["Kit", "KitGraphDto"],
  ["Design", "DesignGraphDto"],
  ["Type", "TypeGraphDto"],
  ["Piece", "PieceGraphDto"],
  ["Connection", "ConnectionGraphDto"],
  ["Author", "AuthorGraphDto"],
  ["Concept", "ConceptGraphDto"],
  ["Quality", "QualityGraphDto"],
  ["Tag", "TagGraphDto"],
  ["Representation", "RepresentationGraphDto"],
  ["Connector", "ConnectorGraphDto"],
  ["Port", "PortGraphDto"],
  ["Family", "FamilyGraphDto"],
  ["openKit", "openKitStore"],
]);
const lines = [];
for (const n of [...names].sort()) {
  if (omitReexport.has(n)) continue;
  if (cqrsNames.has(n)) {
    const out = alias.get(n);
    if (out) lines.push(`export import ${out} = WasmGraph.${n};`);
    continue;
  }
  lines.push(`export import ${n} = WasmGraph.${n};`);
}

const body = wasm.replace(/^\/\/ #region[^\n]*\n\/\/[^\n]*\n\/\/ #endregion[^\n]*\n\n?/m, "");

const block = `//#region 🧷KitWasmHost
/** @emoji 🧷 Wasm kit graph + KitStore (nested under {@linkcode WasmGraph}). */
export namespace WasmGraph {
${body}
}

//#endregion 🧷KitWasmHost

//#region 🧷WasmGraphFlatReexports
${lines.join("\n")}
//#endregion 🧷WasmGraphFlatReexports
`;

fs.writeFileSync(".repo/🎫/26/05/12/FIELD-ONLY-KIT-READS-CQRS-CLASSES/merged-wasm-chunk.txt", block, "utf8");
console.log("export import count", lines.length, "chunk bytes", block.length);
