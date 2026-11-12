import { resolveLod } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🧊️lod/🟦️.ts";
for (const [lod, fps] of [["preview", 15], ["preview", 30], ["full", 15], ["poster", 15]] as const) console.log(lod, fps, resolveLod(lod, fps).revision);
