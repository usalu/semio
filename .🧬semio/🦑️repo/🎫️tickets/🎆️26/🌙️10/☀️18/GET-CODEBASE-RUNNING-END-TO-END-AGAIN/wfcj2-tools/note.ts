import { toolJobMountedDispatchOneTurnExact, toolJobPluginInstanceSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts";
const w = toolJobPluginInstanceSource(process.cwd()); const p = "session.pump_one_for_interactive_turn(same)";
console.log(toolJobMountedDispatchOneTurnExact(w), toolJobMountedDispatchOneTurnExact(w.replace(p, "")), toolJobMountedDispatchOneTurnExact(w.replace(p, `${p}; ${p}`)));
