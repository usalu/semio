import {join} from "node:path";
import {describeHostReasonV1,hostCapacityDirectoryV1,readHostCapacityStatusV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "../../🌡️host-capacity/🟦️.ts";
import {runLeasesCommandV1,type LeaseAdmissionV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/📋️status/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";

/** 🗂️ The repository's lease directory, the one every `repo` lease uses. */
export const REPOSITORY_LEASE_DIRECTORY_V1=".cache/b/l/m/data";

/** 🚪️ Reads the machine-wide admission queue next to the leases: grants, waiters and why each waits. A failing sampler or an unreadable gate directory omits the section instead of failing the command. */
export async function readLeaseAdmissionV1(environment:NodeJS.ProcessEnv,language:"en"|"de",now:number=Date.now()):Promise<LeaseAdmissionV1|undefined>{
 try{
  const control=createCapacityControlV1();
  try{
   const snapshot=await readHostCapacityStatusV1({directory:hostCapacityDirectoryV1(environment),policy:BUILD_HOST_CAPACITY_POLICY_V1,control:control.control});
   const decisions=new Map<string,string>(snapshot.evaluation.decisions.map((row:{id:string;reason:string})=>[row.id,row.reason]));
   return Object.freeze({
    floorMegabytes:snapshot.evaluation.floorMegabytes,headroomMegabytes:snapshot.evaluation.headroomMegabytes,
    grants:snapshot.grants.map((grant:{id:string;operation:string;pid:number;jobs:number;reservedMegabytes:number;observedMegabytes:number|null;admittedAtMs:number})=>({id:grant.id,operation:grant.operation,pid:grant.pid,jobs:grant.jobs,reservedMegabytes:grant.reservedMegabytes,observedMegabytes:grant.observedMegabytes,heldSeconds:Math.max(0,Math.floor((now-grant.admittedAtMs)/1000))})),
    waiters:snapshot.waiters.map((waiter:{request:{id:string;operation:string;fixedMegabytes:number;minimumJobs:number;perJobMegabytes:number};pid:number;arrivedAtMs:number})=>{const reason=decisions.get(waiter.request.id)??"queue-order";return{id:waiter.request.id,operation:waiter.request.operation,pid:waiter.pid,needMegabytes:waiter.request.fixedMegabytes+waiter.request.minimumJobs*waiter.request.perJobMegabytes,waitedSeconds:Math.max(0,Math.floor((now-waiter.arrivedAtMs)/1000)),reason,reasonText:reason==="fits"||reason==="reduced"||reason==="override"?reason:describeHostReasonV1(language,reason as never)};}),
   });
  }finally{control.dispose();}
 }catch{return undefined;}
}

/** ▶️ Runs `repo leases status [--json] [--lease-directory=<path>] [--all] [--violations=<n>]` for one checkout. */
export function runLeasesCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv,write:(line:string)=>void=writeResultLineV1):Promise<number>{
 return runLeasesCommandV1(args,{environment,write,directory:join(root,REPOSITORY_LEASE_DIRECTORY_V1),admission:language=>readLeaseAdmissionV1(environment,language)});
}
