/** 🧷️ Applies repair ops (every op is idempotent) to an asset document and writes it back in its own indentation; for the plainly serialised files of the generator chains (`person-taking-selfie`, `person-with-guide-dog`) and for work copies.
 *  bun apply.ts <slug> [--ops <file>] [--in <file>] [--out <file>]
 * @see ./ops.ts — the ops */
import { parseArgs } from "node:util";
import { applyOps, type Op } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { ops: { type: "string" }, in: { type: "string" }, out: { type: "string" } } });
for (const slug of positionals) {
  const source = values.in ?? catalogFile(slug);
  const text = await Bun.file(source).text();
  const second = text.split("\n", 2)[1] ?? "";
  const indent = second.length - second.trimStart().length;
  const doc = JSON.parse(text);
  const ops = JSON.parse(await Bun.file(values.ops ?? opsFile(slug)).text()) as Op[];
  const skipped = applyOps(doc.asset, ops);
  if ((await Bun.file(source).text()) !== text) throw new Error(`${source} changed on disk since it was read: nothing written, run again`);
  await Bun.write(values.out ?? catalogFile(slug), JSON.stringify(doc, null, indent) + "\n");
  console.log(`${slug}: ${ops.length} ops applied${skipped.length > 0 ? `, ${skipped.length} already in place` : ""}`);
}
