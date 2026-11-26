/** 📏️ Reads the symbol table of snapshot packs: symbol count, symbol payload bytes and the manifest field count, for the offline discovery census. usage: bun symbol-census.ts <pack..> */
import { readFileSync } from "node:fs";
import { decodeSegment, openPack, Reader, unwrapBinary } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts";
for (const file of process.argv.slice(2)) {
  let data = new Uint8Array(readFileSync(file));
  try { data = unwrapBinary(data).inner; } catch {}
  const opened = openPack(data);
  const reader = new Reader(data, 32);
  let symbols = new Uint8Array(0);
  for (;;) { const { kind, payload } = decodeSegment(reader); if (kind === 3) symbols = payload; if (kind === 0) break; }
  console.log(file.split("/").pop(), JSON.stringify({ manifest: opened.manifest, symbolPayload: symbols.length }, (_, v) => (typeof v === "bigint" ? Number(v) : v instanceof Uint8Array ? undefined : v)));
}
