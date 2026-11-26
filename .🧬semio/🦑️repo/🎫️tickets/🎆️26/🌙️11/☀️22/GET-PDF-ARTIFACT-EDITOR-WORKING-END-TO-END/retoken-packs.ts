/** 🏷️ Rewrites the envelope token of committed semio pack files to the schema's spelling, container bytes untouched. usage: bun retoken-packs.ts <apply|dry> <file..> */
import { readFileSync, writeFileSync } from "node:fs";
import { unwrapBinary, wrapBinary } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts";

const [mode, ...files] = process.argv.slice(2);
let changed = 0;
for (const file of files) {
  const { token, inner } = unwrapBinary(new Uint8Array(readFileSync(file)));
  const fixed = token.replace(/^s\.(stdio\.semio\.)/, "$1");
  if (fixed === token) continue;
  changed += 1;
  if (mode === "apply") writeFileSync(file, wrapBinary(fixed, inner));
}
console.log(JSON.stringify({ mode, changed }));
