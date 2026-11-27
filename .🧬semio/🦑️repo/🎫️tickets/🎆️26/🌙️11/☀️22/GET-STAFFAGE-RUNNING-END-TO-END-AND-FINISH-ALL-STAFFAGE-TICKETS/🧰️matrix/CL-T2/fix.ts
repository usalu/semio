/** 🩹️ The repair of the closing lane CL-T2, idempotent: `bun fix.ts <id> [--out <file.json>] [--write]` cures what `🧪️tests/🧊️model-catalog` finds in the geometry of an asset (a revolve profile that crosses itself, repeated boxes whose copies share faces, a list whose triangles grow towards a coarser scale) and prints one line per change. Without `--write` and `--out` it only reports; `--out` writes the repaired document to a trial copy, `--write` to the catalog file (refused when the file changed on disk since it was read).
 * @see ./repairs.ts — the repair of every asset */
import { parseArgs } from "node:util";
import { readDoc, writeDoc } from "../MR/lib.ts";
import { log, repairs } from "./repairs.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { out: { type: "string" }, write: { type: "boolean", default: false } } });
const read = readDoc(positionals[0]!), repair = repairs[read.doc.asset.id];
if (repair === undefined) throw new Error(`${read.doc.asset.id} is no asset of lane CL-T2`);
repair(read.doc.asset);
for (const line of log.slice(0, Number(process.env.SHOW ?? 400))) console.log(`   ${line}`);
const wrote = values.out !== undefined ? writeDoc(read, values.out) : values.write ? writeDoc(read) : false;
console.log(`${read.doc.asset.id}: ${log.length} changes${values.out !== undefined ? `, trial copy ${values.out}` : values.write ? (wrote ? ", catalog file written" : ", catalog file unchanged") : ", nothing written"}`);
