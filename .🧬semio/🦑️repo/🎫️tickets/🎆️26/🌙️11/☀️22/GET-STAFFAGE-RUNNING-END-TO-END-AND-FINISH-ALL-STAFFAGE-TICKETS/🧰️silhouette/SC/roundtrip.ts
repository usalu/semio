/** 🔁️ Whether a component file is reproduced byte for byte by JSON.parse and JSON.stringify in its own indentation (so that a repair changes only what it means to change): bun roundtrip.ts <id>… */
import { readDoc } from "./lib.ts";
for (const id of process.argv.slice(2)) {
  const { doc, indent, text } = await readDoc(id);
  const out = JSON.stringify(doc, null, indent) + (text.endsWith("\n") ? "\n" : "");
  console.log(`${id}\tindent ${indent}\t${out === text ? "identical" : `DIFFERS (${text.length} -> ${out.length})`}`);
}
