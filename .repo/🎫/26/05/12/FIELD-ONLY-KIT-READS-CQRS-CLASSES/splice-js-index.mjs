import fs from "node:fs";

const idx = fs.readFileSync("semio/js/index.ts", "utf8");
const chunk = fs.readFileSync(
  ".repo/🎫/26/05/12/FIELD-ONLY-KIT-READS-CQRS-CLASSES/merged-wasm-chunk.txt",
  "utf8",
);
const startMark = "//#region 🧩WasmGraphNamespace";
const endMark = "//#endregion 🧷WasmOnlyTypesReexport";
const i0 = idx.indexOf(startMark);
const i1 = idx.indexOf(endMark);
if (i0 < 0 || i1 < 0) throw new Error("markers not found");
const i1e = i1 + endMark.length;
const head = idx.slice(0, i0).trimEnd();
const tail = idx.slice(i1e);
const out = `${head}\n\n${chunk}\n${tail}`;
fs.writeFileSync("semio/js/index.ts", out, "utf8");
console.log("written semio/js/index.ts bytes", out.length);
