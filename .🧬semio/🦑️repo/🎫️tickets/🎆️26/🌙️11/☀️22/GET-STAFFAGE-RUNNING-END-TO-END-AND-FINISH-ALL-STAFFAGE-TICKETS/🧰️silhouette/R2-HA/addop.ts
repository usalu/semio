/** ➕️ Merges hand-written ops (a JSON file with a list of ops) into an ops file, an op of the same target replacing the earlier one: bun addop.ts <ops.json> <hand.json>… */
import { mergeOps, readOps, writeOps } from "./ops.ts";
const [out, ...hands] = process.argv.slice(2) as string[];
let ops = await readOps(out!);
for (const hand of hands) ops = mergeOps(ops, JSON.parse(await Bun.file(hand).text()));
await writeOps(out!, ops);
console.log(`${out!.split("/").slice(-1)[0]}: ${ops.length} ops`);
