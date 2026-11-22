/** 🧷️ The last step of the generator of a person of HW-10 … HW-18: applies the repair ops `K/🧰️scripts/HW-1x/repair/<slug>.json` (drawings, volumes, thermal areas, declared dimensions; every op is idempotent) to the catalog file the generator wrote and writes it back in its own indentation.
 *  bun apply.ts <slug>… [--ops <file>] [--in <file>] [--out <file>]
 * @see ./ops.ts — the ops */
import { parseArgs } from "node:util";
import { applyOp, type Op } from "./ops.ts";
import { catalogFile, opsFile, WORKER } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { ops: { type: "string" }, in: { type: "string" }, out: { type: "string" } } });
for (const slug of positionals) {
  if (WORKER[slug] === undefined) throw new Error(`${slug} is not a person of HW-10 … HW-18`);
  const source = values.in ?? catalogFile(slug);
  const text = await Bun.file(source).text();
  const second = text.split("\n", 2)[1] ?? "";
  const indent = second.length - second.trimStart().length;
  const doc = JSON.parse(text);
  const ops = JSON.parse(await Bun.file(values.ops ?? opsFile(slug)).text()) as Op[];
  for (const o of ops) applyOp(doc.asset, o);
  await Bun.write(values.out ?? catalogFile(slug), JSON.stringify(doc, null, indent) + "\n");
  console.log(`${slug}: ${ops.length} ops applied`);
}
