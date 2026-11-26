/** 🗣️ The stated decisions of a band follow a repair of its drawing: `bun say.ts <id|file.json> --ops <ops.json> [--write] [--out <file>]` sets `representations.<scale>.decisions.<key>` to the `{ en, de }` text of every op `{ "scale", "key", "en", "de" }`; a second run changes nothing; `--write` writes the catalog document only when it did not change on disk since it was read. The last step of the chain of an asset, after `../MH-4/fit.ts` and `../MH-4/lofta.ts`.
 * @see ../MR/lib.ts — reading and writing a document in its own indentation */
import { readDoc, writeDoc } from "../MR/lib.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const read = readDoc(args[0]!);
const ops = JSON.parse(await Bun.file(flag("ops")!).text()) as { scale: string; key: string; en: string; de: string }[];
let changed = 0;
for (const op of ops) {
  const decisions = read.doc.asset.representations[op.scale]?.decisions;
  if (decisions?.[op.key] === undefined) throw new Error(`1:${op.scale} has no decision "${op.key}"`);
  if (decisions[op.key].en === op.en && decisions[op.key].de === op.de) continue;
  decisions[op.key] = { en: op.en, de: op.de };
  changed++;
}
console.log(`${read.doc.asset.id}: ${ops.length} texts, ${changed} changed`);
if (args.includes("--write") || flag("out") !== undefined) console.log(writeDoc(read, flag("out")) ? `written: ${flag("out") ?? read.file}` : "file unchanged");
