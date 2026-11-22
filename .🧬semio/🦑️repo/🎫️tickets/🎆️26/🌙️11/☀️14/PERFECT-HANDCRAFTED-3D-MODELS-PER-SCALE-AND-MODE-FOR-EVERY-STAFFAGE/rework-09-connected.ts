/** 🔗️ Author 09 ruler: only the product rules `volumes-connected` and `volumes-feature` of one asset (source merged over the catalog), with the full messages.
 *
 * Usage: bun rework-09-connected.ts <asset> [--scale N]
 * @see ./rework-brief.md */
import { checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { cli, load } from "./r3-lib.ts";

const { spec, flag } = cli(["--scale"]);
const { asset } = load(spec, "rework-09-connected.ts <asset> [--scale N]");
const scale = flag("--scale");
const findings = checkVolumes(asset as never, { rules: ["volumes-connected", "volumes-feature"] as never, ...(scale === undefined ? {} : { scale: Number(scale) as never }) }) as readonly { rule: string; scale?: number; path: string; message: { en: string } }[];
for (const f of findings) console.log(`${f.rule} 1:${f.scale} ${f.path}: ${f.message.en}`);
console.log(`${findings.length} finding(s)`);
