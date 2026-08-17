import { existsSync } from "node:fs";
import { relative, resolve } from "node:path";
const OLD_TS_DIR = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust/🟦️typescript"; // gone now, computed for reference
const NEW_TS_DIR = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript";
const oldRel = "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/🤖️generated/🟦️session.ts";
const abs = resolve(OLD_TS_DIR, oldRel);
console.log("target abs:", abs, "exists:", existsSync(abs));
const newRel = relative(NEW_TS_DIR, abs);
const newAbs = resolve(NEW_TS_DIR, newRel);
console.log("new rel:", newRel, "resolvesOk:", existsSync(newAbs) && newAbs === abs);
