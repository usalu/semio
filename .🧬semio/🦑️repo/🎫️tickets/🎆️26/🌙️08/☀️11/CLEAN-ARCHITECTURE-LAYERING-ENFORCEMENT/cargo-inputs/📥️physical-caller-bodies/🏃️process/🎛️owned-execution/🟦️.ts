import systemPolicy from "../../🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️entrypoint/🧩️system/🎛️policy/🔣️.json";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import {prepareRepositoryCargoCommand,runCargoSystemOperation,cargoSystemMilliseconds,cargoCommandOwner} from "../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🦀️ Executes caller commands under the repository's authored Cargo and build-budget policy. */
export async function runRepositoryCommand(command: string, args: string[], cwd: string, label: string, timeoutMs = buildBudgetMs(), options: { stdout?: "inherit" | "ignore"; env?: Readonly<Record<string, string | undefined>>; signal?: AbortSignal; onLine?: (line: string) => void } = {}): Promise<void> {
  if (options.signal?.aborted) throw new Error(`${label} stopped: cancelled`);
  if(command==="cargo")return runCargoSystemOperation(getWorkspaceRoot(),timeoutMs>0?timeoutMs:cargoSystemMilliseconds(systemPolicy),async()=>{const owner=cargoCommandOwner(),abort=()=>owner.cancelDiscovery();options.signal?.addEventListener("abort",abort,{once:true});try{if(options.signal?.aborted)owner.cancelDiscovery();await prepareRepositoryCargoCommand(getWorkspaceRoot(),args,cwd,{...process.env,...options.env});return await runOwnedCommand(command,args,cwd,label,timeoutMs,options);}finally{options.signal?.removeEventListener("abort",abort);}});
  return runOwnedCommand(command, args, cwd, label, timeoutMs, options);
}
