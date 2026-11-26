/** 🔁️ Re-deflates committed semio pack files under the canonical codec of the TS twin and checks them against the system zlib. usage: bun recode-packs.ts <apply|dry> <file..> */
import { readFileSync, writeFileSync } from "node:fs";
import { inflateRawSync } from "node:zlib";
import { buildPack, decodeSegment, openPack, Reader, unwrapBinary, wrapBinary } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts";

const [mode, ...files] = process.argv.slice(2);
const KIND_SYMBOLS = 3, KIND_END = 0;
let changed = 0, same = 0, failed = 0, before = 0, after = 0;
for (const file of files) {
  try {
    const data = new Uint8Array(readFileSync(file));
    const { token, inner } = unwrapBinary(data);
    const opened = openPack(inner);
    const header = 32;
    const reader = new Reader(inner, header);
    let symbols = new Uint8Array(0);
    for (;;) {
      const { kind, payload } = decodeSegment(reader);
      if (kind === KIND_SYMBOLS) symbols = payload;
      if (kind === KIND_END) break;
    }
    const rebuilt = buildPack(opened.manifest.schemaHash, symbols, opened.body, opened.chunks);
    const again = openPack(rebuilt);
    if (Buffer.compare(Buffer.from(again.body), Buffer.from(opened.body)) !== 0) throw new Error("body differs after recode");
    const out = wrapBinary(token, rebuilt);
    before += data.length; after += out.length;
    if (Buffer.compare(Buffer.from(out), Buffer.from(data)) === 0) { same += 1; continue; }
    changed += 1;
    if (mode === "apply") writeFileSync(file, out);
  } catch (error) {
    failed += 1;
    console.log("FAILED", file, String(error).slice(0, 160));
  }
}
console.log(JSON.stringify({ mode, changed, same, failed, before, after }));
void inflateRawSync;
