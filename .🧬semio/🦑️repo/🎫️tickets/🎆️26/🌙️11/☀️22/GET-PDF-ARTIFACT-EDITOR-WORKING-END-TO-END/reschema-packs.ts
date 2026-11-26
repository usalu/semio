/** 🏷️ Re-encodes committed semio record packs whose document `schema` still carries the retired `s.` prefix, through the TS twin. usage: bun reschema-packs.ts <apply|dry> */
import { readFileSync, writeFileSync } from "node:fs";
import { decodePack, encodePack, loadRecordSpec, type RecordSpecDocument } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts";

const root = new URL("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/", import.meta.url);
const fixture = JSON.parse(readFileSync(new URL("🧪️tests/🎒️record-document/🧫️fixtures/🔣️.json", root), "utf8")) as { subsets: { spec: string; packs: string[] }[] };
let changed = 0, same = 0;
for (const subset of fixture.subsets) {
  const spec = loadRecordSpec(JSON.parse(readFileSync(new URL(subset.spec, root), "utf8")) as RecordSpecDocument);
  for (const pack of subset.packs) {
    const url = new URL(pack, root);
    const data = new Uint8Array(readFileSync(url));
    const document = decodePack(spec, data);
    if (typeof document.schema !== "string" || !document.schema.startsWith("s.stdio.")) { same += 1; continue; }
    document.schema = document.schema.slice(2);
    changed += 1;
    if (process.argv[2] === "apply") writeFileSync(url, encodePack(spec, document));
  }
}
console.log(JSON.stringify({ mode: process.argv[2], changed, same }));
