/** 📏️ Reads snapshot packs and writes each raw body for the offline compressor cost census. usage: bun pack-census.ts <out dir> <pack..> */
import { readFileSync, writeFileSync } from "node:fs";
import { openPack, unwrapBinary } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts";
const [out, ...files] = process.argv.slice(2);
for (const file of files) {
  let data = new Uint8Array(readFileSync(file));
  try { data = unwrapBinary(data).inner; } catch {}
  const opened = openPack(data);
  const name = file.split("/").pop()!.replace(".snapshot.pack", "");
  writeFileSync(`${out}/${name}.body.bin`, opened.body);
  console.log(name, "pack", data.length, "raw body", opened.body.length);
}
