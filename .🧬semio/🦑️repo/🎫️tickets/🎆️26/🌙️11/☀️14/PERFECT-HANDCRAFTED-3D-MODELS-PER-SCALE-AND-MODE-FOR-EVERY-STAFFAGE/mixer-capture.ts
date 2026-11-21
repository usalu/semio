/** 📦️ Capture a mixer GLB for an offline, byte-for-byte attribute analysis. */
import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {entryOf,GENERATED} from "./volumes-lib.ts";
import {writeGlb} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
const {asset}=entryOf("machinery/concrete-mixer-truck");
const bytes=await writeGlb(asset,{pose:asset.presentation.defaultPose});
writeFileSync(join(GENERATED,"equipment-check",process.argv[2]??"mixer-base.glb"),bytes);
console.log(bytes.length);
