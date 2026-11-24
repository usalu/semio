import { runTwins } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts";
const r = runTwins("/Users/ueli/Documents/semio", [process.env.TWINS_MODE ?? "check", "--under", process.env.TWINS_UNDER ?? "✏️s/🔌️plugins/📕️norm", "--only", process.env.TWINS_LANG ?? "typescript", "--strict"]);
console.log(JSON.stringify(r, null, 1).slice(0, 3000));
