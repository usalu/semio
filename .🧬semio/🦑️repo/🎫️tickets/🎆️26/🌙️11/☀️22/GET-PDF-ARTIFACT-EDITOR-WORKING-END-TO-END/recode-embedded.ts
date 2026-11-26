/** 🔁️ Re-deflates pack files embedded as hex or base64 strings in fixtures and sources under the canonical codec of the TS twin. usage: bun recode-embedded.ts <apply|dry> <file..> */
import { readFileSync, writeFileSync } from "node:fs";
import { buildPack, decodeSegment, openPack, Reader, unwrapBinary, wrapBinary } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts";

const [mode, ...files] = process.argv.slice(2);
const PACK = [0x89, 0x53, 0x50, 0x4b], WRAP = [0x89, 0x53, 0x45, 0x4d];
const starts = (data: Uint8Array, magic: number[]): boolean => magic.every((byte, index) => data[index] === byte);
function recode(data: Uint8Array): Uint8Array | undefined {
  let token: string | undefined;
  let inner = data;
  if (starts(data, WRAP)) ({ token, inner } = unwrapBinary(data));
  else if (!starts(data, PACK)) return undefined;
  const opened = openPack(inner);
  const reader = new Reader(inner, 32);
  let symbols = new Uint8Array(0);
  for (;;) {
    const { kind, payload } = decodeSegment(reader);
    if (kind === 3) symbols = payload;
    if (kind === 0) break;
  }
  const rebuilt = buildPack(opened.manifest.schemaHash, symbols, opened.body, opened.chunks);
  return token === undefined ? rebuilt : wrapBinary(token, rebuilt);
}
let changed = 0, same = 0, failed = 0;
for (const file of files) {
  let text = readFileSync(file, "utf8");
  const original = text;
  const swap = (match: string, decode: (value: string) => Uint8Array, encode: (data: Uint8Array) => string): string => {
    try {
      const data = decode(match);
      const out = recode(data);
      if (out === undefined) return match;
      const encoded = encode(out);
      if (encoded === match) same += 1; else changed += 1;
      return encoded;
    } catch (error) { failed += 1; console.log("FAILED", file, match.slice(0, 24), String(error).slice(0, 100)); return match; }
  };
  text = text.replace(/\b(?:[0-9a-f]{2}){32,}\b/g, (m) => swap(m, (v) => Uint8Array.from(Buffer.from(v, "hex")), (d) => Buffer.from(d).toString("hex")));
  text = text.replace(/(?:iVNQSw0KGgo|iVNFTQ0KGgo)[A-Za-z0-9+/]*={0,2}/g, (m) => swap(m, (v) => Uint8Array.from(Buffer.from(v, "base64")), (d) => Buffer.from(d).toString("base64")));
  if (text !== original && mode === "apply") writeFileSync(file, text);
  if (text !== original) console.log(mode === "apply" ? "REWROTE" : "WOULD", file);
}
console.log(JSON.stringify({ mode, changed, same, failed }));
