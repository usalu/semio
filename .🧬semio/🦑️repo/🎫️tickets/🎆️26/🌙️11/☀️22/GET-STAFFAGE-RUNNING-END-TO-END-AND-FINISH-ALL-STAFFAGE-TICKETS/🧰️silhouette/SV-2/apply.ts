/** 🧷️ The last step of the generator of a plant of SV-2: applies the repair ops `repair/<slug>.json` (drawings, volumes, declared dimensions; every op is idempotent) to the catalog file and writes it back in its own indentation. The file is read right before it is written.
 *  bun apply.ts <slug>… */
import { load, save } from "./lib.ts";

for (const slug of process.argv.slice(2)) {
  const { text, doc, ops } = await load(slug);
  await save(slug, text, doc);
  console.log(`${slug}: ${ops.length} ops applied`);
}
