/** 🔬️ Author 61: product findings of one rule with their data (which solids). Usage: `bun author-61-diag.ts <asset> <scale> <rule>`. */
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, scale, rule] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const source = loadSource(entry);
const m = merged(asset, source);
const mod: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
const out = mod.checkVolumes(m, { scale: Number(scale), rules: [rule] }, { signal: { aborted: false, throwIfAborted() {} }, progress() {} });
for (const f of out) console.log(f.code, f.message.en.slice(0, 300), JSON.stringify(f.data ?? {}).slice(0, 800), f.measurements.map((x: any) => `${x.name}=${x.value}`).join(" "));
console.log(out.length, "findings");
