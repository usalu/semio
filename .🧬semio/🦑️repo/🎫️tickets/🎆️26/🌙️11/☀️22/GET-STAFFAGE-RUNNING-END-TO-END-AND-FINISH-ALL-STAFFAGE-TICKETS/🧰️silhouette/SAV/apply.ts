/** 🧷️ The silhouette repair of an asset of batch SAV: applies the repair ops `repair/<id>.json` (drawings, volumes, declared dimensions; every op is idempotent, see `ops.ts`) to the catalog file and writes it back in its own indentation. The file is read right before it is written.
 *  bun apply.ts <id>… [--ops <file>] [--in <file>] [--out <file>] */
import { parseArgs } from "node:util";
import { applyOps, type Op } from "./ops.ts";
import { catalogFile } from "./files.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { ops: { type: "string" }, in: { type: "string" }, out: { type: "string" } } });
for (const id of positionals) {
  const text = await Bun.file(values.in ?? catalogFile(id)).text();
  const second = text.split("\n", 2)[1] ?? "";
  const indent = second.length - second.trimStart().length;
  const doc = JSON.parse(text);
  const ops = JSON.parse(await Bun.file(values.ops ?? `${import.meta.dir}/repair/${id}.json`).text()) as Op[];
  const skipped = applyOps(doc.asset, ops);
  const next = JSON.stringify(doc, null, indent) + "\n";
  if (next !== text) await Bun.write(values.out ?? catalogFile(id), next);
  console.log(`${id}: ${ops.length} ops applied${skipped.length > 0 ? `, ${skipped.length} already in place` : ""}${next === text ? ", file unchanged" : ""}`);
}
