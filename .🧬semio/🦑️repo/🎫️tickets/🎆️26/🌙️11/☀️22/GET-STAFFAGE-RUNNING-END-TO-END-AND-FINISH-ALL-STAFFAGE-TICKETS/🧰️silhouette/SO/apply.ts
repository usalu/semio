/** 🧷️ The reproducible repair of an object of batch SO: applies the repair ops `ops/<id>.json` (drawings, volumes, declared dimensions; every op is idempotent) to the catalog file and writes it back in its own indentation. The file is read right before it is written.
 *  bun apply.ts <id>… [--ops <file>] [--in <file>] [--out <file>]
 * @see ./ops.ts — the ops */
import { parseArgs } from "node:util";
import { applyOps, type Op } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { ops: { type: "string" }, in: { type: "string" }, out: { type: "string" } } });
for (const slug of positionals) {
  const source = values.in ?? catalogFile(slug);
  const ops = JSON.parse(await Bun.file(values.ops ?? opsFile(slug)).text()) as Op[];
  const text = await Bun.file(source).text();
  const second = text.split("\n", 2)[1] ?? "";
  const indent = second.length - second.trimStart().length;
  const doc = JSON.parse(text);
  const skipped = applyOps(doc.asset, ops);
  const next = JSON.stringify(doc, null, indent) + "\n";
  if (next !== text || values.out !== undefined) await Bun.write(values.out ?? catalogFile(slug), next);
  console.log(`${slug}: ${ops.length} ops applied${skipped.length > 0 ? `, ${skipped.length} already in place` : ""}${next === text ? " (file unchanged)" : ""}`);
}
