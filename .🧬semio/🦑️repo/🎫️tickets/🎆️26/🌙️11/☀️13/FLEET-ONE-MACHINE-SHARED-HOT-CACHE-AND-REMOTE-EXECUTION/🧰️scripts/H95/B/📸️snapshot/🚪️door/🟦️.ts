import {snapshotDueV1,snapshotRunV1} from "../🟦️.ts";
import {snapshotArgumentPathsV1} from "../⏩️advance/🟦️.ts";

/** 🚪️ The hook of the front door: a command of the snapshot policy runs on a consistent source snapshot of the checkout (see `snapshotRunV1`); the exit code of the command, or nothing when it runs on in this process (a command outside the policy, a capture that failed, `digest` mode). */
export async function snapshotDoorV1(root:string,typed:readonly string[],operation:readonly string[],environment:NodeJS.ProcessEnv):Promise<number|undefined>{
 const mode=snapshotDueV1(operation,environment);
 if(mode===undefined)return undefined;
 const outcome=await snapshotRunV1({root,cwd:process.cwd(),operation,mode,entry:process.argv[1]??"",arguments:typed[0]==="repo"?typed.slice(1):typed,environment,extraPaths:snapshotArgumentPathsV1(root,process.cwd(),typed)});
 return outcome.kind==="spawned"?outcome.code:undefined;
}
