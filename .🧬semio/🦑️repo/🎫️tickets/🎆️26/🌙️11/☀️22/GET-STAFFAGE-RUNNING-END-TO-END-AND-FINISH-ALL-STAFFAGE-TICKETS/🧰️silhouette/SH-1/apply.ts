/** 🧷️ The last repair step of a person of the batch SH-1: applies the idempotent ops `ops/<slug>.json` to the catalog file (re-read right before the write) and writes it back in its own indentation.
 *  bun apply.ts <slug>… [--in <file>] [--out <file>]
 * @see ./ops.ts — the ops */
import { parseArgs } from "node:util";
import { applyOps, type Op } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { in: { type: "string" }, out: { type: "string" } } });
for (const slug of positionals) {
  const ops = JSON.parse(await Bun.file(opsFile(slug)).text()) as Op[];
  const text = await Bun.file(values.in ?? catalogFile(slug)).text();
  const second = text.split("\n", 2)[1] ?? "";
  const indent = second.length - second.trimStart().length;
  const doc = JSON.parse(text);
  const skipped = applyOps(doc.asset, ops);
  await Bun.write(values.out ?? catalogFile(slug), JSON.stringify(doc, null, indent) + "\n");
  console.log(`${slug}: ${ops.length} ops applied${skipped.length > 0 ? `, ${skipped.length} already in place` : ""}`);
}
