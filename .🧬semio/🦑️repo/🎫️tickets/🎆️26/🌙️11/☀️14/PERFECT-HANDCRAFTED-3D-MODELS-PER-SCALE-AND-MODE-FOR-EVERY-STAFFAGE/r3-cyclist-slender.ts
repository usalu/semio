/** 🪢 Round 3 cyclist: prints the product's slenderness of every solid flagged slender (or named) in the candidate document, per scale and list.
 * Usage: `bun r3-cyclist-slender.ts [<candidate json>]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { slendernessOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";

const doc = JSON.parse(readFileSync(process.argv[2] ?? join(import.meta.dir, "🗑️generated/r3-cyclist/🔣️.json"), "utf8"));
for (const [scale, v] of Object.entries(doc.asset.volumes) as [string, Record<string, { id: string; slender?: boolean }[]>][])
  for (const list of ["body", "thermal", "structure"])
    for (const s of v[list] ?? []) if (s.slender === true) process.stdout.write(`${scale} ${list} ${s.id} ${slendernessOf(doc.asset, s as never).toFixed(1)}\n`);
