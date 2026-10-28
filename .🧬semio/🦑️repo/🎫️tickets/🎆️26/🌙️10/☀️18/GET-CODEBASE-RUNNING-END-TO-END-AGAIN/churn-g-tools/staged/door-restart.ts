import {runWithStaleReexecV1} from "../../🧷️format-stamps/🔄️reexec/🟨️.mjs";
import {useProcessWideImplementationPinV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";

const invocationEnvironmentV1="SEMIO_SCRIPT_PROCESS_INVOCATION";
/** 🔖️ The environment variable that marks a command started inside another one, spelled as the routing module spells it (a law compares the two) so the front door need not load the routing module to know whether it is the outermost command. */
export const BUILD_DOOR_INVOCATION_ENVIRONMENT_V1:string=invocationEnvironmentV1;

/** ♻️ Runs the build implementation of one command line as the unit a fresh process can run again: the loading of the implementation is part of `run`, so a module a peer is moving or half editing while it loads, a raw filesystem race and a moved format stamp all end in the declared restart (the outermost command starts again with the current implementation, bounded, paused and announced in both languages by the format stamp protocol) instead of an uncaught failure; a nested command only yields its exit code. This process is one invocation unless it is the resident service, which runs invocations side by side and never opts into the process-wide pin.
 * @see ../../🧷️format-stamps/🔄️reexec/🟨️.mjs */
export async function runBuildWithRestartV1(run:()=>Promise<void>,argv:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void>{
 if(!argv.slice(0,3).includes("service"))useProcessWideImplementationPinV1();
 const outcome=await runWithStaleReexecV1(run,{environment,outermost:environment[invocationEnvironmentV1]===undefined});
 if(outcome.status!=="completed")process.exit(outcome.code);
}
