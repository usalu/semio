/** 🔎️ Round 3, vegetation/hosta-clump: prints the product validator's findings (with the named culprits) for chosen rules and one scale. Usage: bun r3-hosta-clump-findings.ts <scale> <rule,rule>
 * @see ./r3-vegetation-hosta-clump.md */
import { join } from "node:path";
import { STAFFAGE, entryOf, loadSource, merged } from "./volumes-lib.ts";
const { checkVolumes } = (await import(join(STAFFAGE, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"))) as { checkVolumes: (asset: unknown, options: unknown) => { rule: string; message: { en: string } }[] };
const { entry, asset } = entryOf("vegetation/hosta-clump");
for (const f of checkVolumes(merged(asset, loadSource(entry)!), { scale: Number(process.argv[2]), rules: process.argv[3]!.split(",") })) console.log(`[DEBUG] ${f.rule}: ${f.message.en}`);
