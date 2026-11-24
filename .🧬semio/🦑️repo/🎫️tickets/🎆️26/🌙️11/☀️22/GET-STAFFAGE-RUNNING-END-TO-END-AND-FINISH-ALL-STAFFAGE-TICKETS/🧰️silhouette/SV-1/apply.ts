/** 🧷️ The last step of the chain of a plant of SV-1: replays the repair ops `ops/<id>.json` (drawn shapes, solids, declared dimensions; every op is idempotent) on the catalog file, re-read right before it is written, and writes the changes into the text of that file, so everything the ops do not touch keeps its bytes.
 *  bun apply.ts <id>… [--in <file>] [--out <file>]
 * @see ./ops.ts — the ops
 * @see ./preserve.ts — the layout-preserving writer */
import { parseArgs } from "node:util";
import { applyOps, readOps } from "./ops.ts";
import { fileOf, opsFile } from "./plants.ts";
import { rewrite } from "./preserve.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { in: { type: "string" }, out: { type: "string" } } });
for (const id of positionals) {
  const ops = await readOps(opsFile(id));
  const text = await Bun.file(values.in ?? fileOf(id)).text();
  const doc = JSON.parse(text);
  if (doc.asset.id !== id) throw new Error(`${values.in ?? fileOf(id)} holds ${doc.asset.id}, not ${id}`);
  const next = structuredClone(doc);
  const skipped = applyOps(next.asset, ops);
  const written = rewrite(text, doc, next);
  if (written === text) { console.log(`${id}: ${ops.length} ops, the file already has them`); continue; }
  await Bun.write(values.out ?? fileOf(id), written);
  console.log(`${id}: ${ops.length} ops applied${skipped.length > 0 ? `, ${skipped.length} skipped as already in place` : ""} -> ${values.out ?? "catalog"} (${text.length} -> ${written.length} characters)`);
}
