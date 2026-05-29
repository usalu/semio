import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const dir = join(root, "semio/client/lib/js");

function stripHeader(src: string): string {
  const lines = src.split(/\r?\n/);
  let i = 0;
  if (lines[i]?.includes("#region 🧲Header")) {
    i++;
    while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
    i++;
  }
  while (i < lines.length && lines[i]?.trim() === "") i++;
  return lines.slice(i).join("\n");
}

const indexPath = join(dir, "index.ts");
let index = readFileSync(indexPath, "utf8");

index = index
  .replace(/^import \{ assertRsJsSessionOpenUri, RS_WASM_EMPTY_STORE_URI \} from "\.\/graphql-contract";\r?\n/m, "")
  .replace(/^import \{ GQL_RESPONSE_SELECTION, withResponseSelection \} from "\.\/graphql-kit-selection";\r?\n/m, "")
  .replace(/^import \{ createRsWasmGraphqlHandle \} from "\.\/rs-wasm-transport";\r?\n/m, "")
  .replace(/^export \{ GQL_RESPONSE_SELECTION, withResponseSelection \} from "\.\/graphql-kit-selection";\r?\n/m, "");

const contract = stripHeader(readFileSync(join(dir, "graphql-contract.ts"), "utf8"));
const kitSel = stripHeader(readFileSync(join(dir, "graphql-kit-selection.ts"), "utf8"));
let wasm = stripHeader(readFileSync(join(dir, "rs-wasm-transport.ts"), "utf8"));
wasm = wasm.replace(
  /\/\/#region 🔌Adapters\r?\nimport \{ RS_WASM_EMPTY_STORE_URI \} from "\.\/graphql-contract";\r?\n\/\/#endregion 🔌Adapters\r?\n\r?\n/m,
  "",
);

const block = `//#region 🌐GraphqlContract\n${contract}//#endregion 🌐GraphqlContract\n\n//#region 🌐GraphqlKitSelection\n${kitSel}//#endregion 🌐GraphqlKitSelection\n\n//#region 🌐RsWasmTransport\n${wasm}//#endregion 🌐RsWasmTransport\n\n`;

index = index.replace(/^(#\/\/endregion 🧲Header\r?\n)/m, `$1\n${block}`);

index = index.replace(
  /export \{\r?\n  assertRsJsSessionOpenUri,\r?\n  RS_WASM_EMPTY_STORE_URI,\r?\n  SEMIO_GRAPHQL_GOLDEN_SCHEMA_PATH,\r?\n  type GraphqlWirePostBody,\r?\n\} from "\.\/graphql-contract";\r?\n\r?\n/m,
  "",
);

writeFileSync(indexPath, index);
unlinkSync(join(dir, "graphql-kit-selection.ts"));
unlinkSync(join(dir, "rs-wasm-transport.ts"));
unlinkSync(join(dir, "graphql-contract.ts"));

console.log("merged semio/js satellites into index.ts");
