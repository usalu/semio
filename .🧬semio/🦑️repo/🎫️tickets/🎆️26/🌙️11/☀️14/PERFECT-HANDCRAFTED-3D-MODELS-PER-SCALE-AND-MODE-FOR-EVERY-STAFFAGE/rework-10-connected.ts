/** 🔗️ The full findings of the measured rule `volumes-connected` (and any other rule named) of one asset, with every loose group, so that each floating part can be joined to what carries it.
 *
 * Usage: bun rework-10-connected.ts <asset> [--scale N] [--rules a,b]
 * @see ./r3-lib.ts */
import { checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { cli, load } from "./r3-lib.ts";

const { spec, flag } = cli(["--scale", "--rules"]);
const { asset } = load(spec, "rework-10-connected.ts <asset> [--scale N] [--rules a,b]");
const rules = (flag("--rules") ?? "volumes-connected").split(",");
const scales = flag("--scale") === undefined ? [50, 100, 200, 500] : [Number(flag("--scale"))];
let n = 0;
for (const scale of scales) {
  for (const f of checkVolumes(asset as never, { scale, rules } as never) as readonly any[]) {
    n++;
    console.log(`1:${f.scale} ${f.path}: ${String(f.message.en).slice(0, 110)}`);
    const loose = f.data?.loose ?? f.data?.groups ?? f.data;
    if (Array.isArray(loose)) for (const g of loose) console.log(`   ${g.reason} gap ${(g.gapM * 1000).toFixed(1)} mm nearest ${g.nearest ?? "-"}: ${(g.ids ?? []).slice(0, 14).join(" ")}${(g.ids ?? []).length > 14 ? " …" : ""}`);
    else if (loose?.solids !== undefined) console.log(`   ${loose.reason}: ${[...new Set(loose.solids as string[])].join(" ").slice(0, 900)} | gaps mm ${(loose.gapsM as string[]).map((g) => Math.round(Number(g) * 1000)).join(" ")}`);
    else if (loose !== undefined) console.log("   " + JSON.stringify(loose).slice(0, 900));
  }
}
console.log(n === 0 ? "CONNECTED" : `${n} finding(s)`);
