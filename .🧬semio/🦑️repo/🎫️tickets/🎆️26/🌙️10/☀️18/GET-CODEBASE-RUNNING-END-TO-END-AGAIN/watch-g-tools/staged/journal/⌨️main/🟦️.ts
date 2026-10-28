import {runChangeJournalMainV1} from "../🟦️.ts";
import {createWatchSupervisorV1} from "../../🔭️watch/🟨️.mjs";
import {POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {RESIDENT_EXCLUDES_V1,RESIDENT_FENCE_DIRECTORY_V1} from "../../⌨️command/🟨️.mjs";

/** 🏁️ The detached process of the change journal service: the existing native watcher and supervisor, the resident exclusions and fence directory, and the watcher provisioned by the tool registry only when no binary is named. */
export function runChangeJournalProcessV1(argv:readonly string[],environment:Readonly<Record<string,string|undefined>>=process.env):Promise<number>{
 return runChangeJournalMainV1(argv,{
  createSupervisor:createWatchSupervisorV1 as never,watcherPolicy:POLICY_V1 as unknown as Record<string,unknown>,exclude:RESIDENT_EXCLUDES_V1,fence:RESIDENT_FENCE_DIRECTORY_V1,
  locate:async()=>{
   if(environment.SEMIO_REPO_WATCH_BINARY)return environment.SEMIO_REPO_WATCH_BINARY;
   const{ensureResidentWatcherV1}=await import("../../🔭️watch/🏗️provision/🔌️tools/🟨️.mjs");
   return ensureResidentWatcherV1();
  },
 });
}

if(import.meta.main)process.exit(await runChangeJournalProcessV1(process.argv.slice(2)));
