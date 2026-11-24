/** 🔁️ Whether the catalog file of a plant is what `JSON.stringify` writes in its own indentation (then a repair changes only what it means to change).
 *  bun roundtrip.ts <slug>… */
import { fileOf } from "./measure-lib.ts";
for (const slug of process.argv.slice(2)) {
  const text = await Bun.file(fileOf(slug)).text();
  const second = text.split("\n", 2)[1] ?? "";
  const again = JSON.stringify(JSON.parse(text), null, second.length - second.trimStart().length) + "\n";
  console.log(`${slug}: ${again === text ? "identical" : `differs (${text.length} against ${again.length} characters)`}`);
}
