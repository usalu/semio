import { runtimeComponentClosure } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
const all = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
console.log("plugins", PLUGIN_BUILD_TARGETS.map(r => r.pluginId).join(","));
console.log("extensions", EXTENSION_TARGETS.length);
for (const v of ["s", "demonstrator", "generator"]) { const c = runtimeComponentClosure(all, [PLAYGROUND_BUILD_TARGETS.find(r => r.variant === v)!.pluginId]); console.log(v, c.length, c.filter(id => PLUGIN_BUILD_TARGETS.some(p => p.pluginId === id)).join(",")); }
const everything = runtimeComponentClosure(all, [...new Set(PLAYGROUND_BUILD_TARGETS.map(r => r.pluginId))]);
console.log("union", everything.length);
console.log("variants", PLAYGROUND_BUILD_TARGETS.length, PLAYGROUND_BUILD_TARGETS.filter(r => !r.brand).map(r => `${r.variant}:${r.app ?? "-"}`).join(" "));
