/** 🔍️ A declared nasal emitter a metre outside its own skin still needs to fail feature validation. */
import {featureRule} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import {entryOf} from "./volumes-lib.ts";
const {asset}=entryOf("animals/pigeon");
const source=asset.volumes["50"].thermal.find(s=>s.id==="th-breath-nostril");
source.transform.translation[2]+=1;
const findings=featureRule(asset as never,[50]).findings.filter(f=>f.message.en.includes('"th-breath-nostril"'));
console.log("Remote aperture findings",findings.length);
if(!findings.length)process.exitCode=1;
