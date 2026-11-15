/** 🪤️ Author 61: kernel trap findings (`volumeTrapIssues`) of one asset's source, listed per scale and solid. Usage: `bun author-61-traps.ts <asset>`. */
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const m = merged(asset, loadSource(entry));
const mod: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts");
const out = mod.volumeTrapIssues(m);
for (const i of out.slice(0, 40)) console.log(i.code, i.path, i.message.en.slice(0, 160));
console.log(out.length, "trap findings");
