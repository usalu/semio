/** 🕵️ Runs the product `checkVolumes` on the bamboo clump for one scale and one rule and prints every finding with its message (the culprits by id). Usage: `bun r3-bamboo-clump-find.ts <scale> <rule>` */
import { checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { load } from "./r3-lib.ts";

const { asset } = load("vegetation/bamboo-clump", "r3-bamboo-clump-find.ts <scale> <rule>");
const out = checkVolumes(asset as never, { scale: Number(process.argv[2]) as never, rules: [process.argv[3]] as never } as never, {} as never);
for (const f of out as never as { message?: { en: string }; en?: string }[]) console.log(JSON.stringify(f).slice(0, 900));
console.log(`${out.length} finding(s)`);
