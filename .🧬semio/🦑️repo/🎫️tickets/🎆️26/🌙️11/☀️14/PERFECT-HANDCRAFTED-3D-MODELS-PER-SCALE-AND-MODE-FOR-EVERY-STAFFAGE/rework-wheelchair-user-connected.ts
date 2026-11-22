/** 🔗️ Every loose group of the measured rule `volumes-connected` (or the rules named) of a candidate document of humans/wheelchair-user.
 *
 * Usage: bun rework-wheelchair-user-connected.ts <asset document json> [scale] [rules]
 * @see ./rework-10-connected.ts */
import { readFileSync } from "node:fs";
import { checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [file, scaleArg, rulesArg] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const rules = (rulesArg ?? "volumes-connected").split(",");
const scales = scaleArg === undefined || scaleArg === "all" ? [50, 100, 200, 500] : [Number(scaleArg)];
let n = 0;
for (const scale of scales) {
  for (const f of checkVolumes(asset as never, { scale, rules } as never) as readonly any[]) {
    n++;
    console.log(`1:${f.scale} ${f.rule} ${f.path}: ${String(f.message.en).slice(0, 160)}`);
    const loose = f.data?.loose ?? f.data?.groups ?? f.data;
    if (Array.isArray(loose)) for (const g of loose) console.log(`   ${g.reason} gap ${(g.gapM * 1000).toFixed(1)} mm nearest ${g.nearest ?? "-"}: ${(g.ids ?? []).slice(0, 10).join(" ")}${(g.ids ?? []).length > 10 ? " …" : ""}`);
    else if (loose !== undefined) console.log("   " + JSON.stringify(loose).slice(0, 2600));
  }
}
console.log(n === 0 ? "CONNECTED" : `${n} finding(s)`);
