import {hostCapacityDirectoryV1,readHostCapacityStatusV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/📋️status/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../🌡️host-capacity/🎛️control/🟦️.ts";
import type {DoctorProbeV1} from "../📋️contract/🟦️.ts";

type GrantRow=Readonly<{operation:string;pid:number;jobs:number;reservedMegabytes:number;observedMegabytes:number|null;admittedAtMs:number}>;
type WaiterRow=Readonly<{request:Readonly<{id:string;operation:string;fixedMegabytes:number;minimumJobs:number;perJobMegabytes:number}>;pid:number;arrivedAtMs:number}>;

/** 🎫️ The machine-wide admission state and the host sample next to it: grants, waiters and why each waits, free memory against the floor, load, swap, open files and attached images. */
export const capacityProbeV1:DoctorProbeV1=Object.freeze({
 area:"capacity",phase:1,
 async read(ports){
  const control={signal:ports.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))};
  const now=ports.now(),status=await readHostCapacityStatusV1({directory:hostCapacityDirectoryV1(ports.environment),policy:BUILD_HOST_CAPACITY_POLICY_V1,control:control as never,now:ports.now});
  const reasons=new Map<string,string>(status.evaluation.decisions.map((row:{id:string;reason:string})=>[row.id,row.reason])),host=status.host;
  return {sections:{
   host:{cores:host.cores,totalMegabytes:host.totalMegabytes,availableMegabytes:host.availableMegabytes,floorMegabytes:status.evaluation.floorMegabytes,pressure:host.pressure,swapUsedMegabytes:host.swapUsedMegabytes,swapOutMegabytesPerSecond:host.swapOutMegabytesPerSecond,load:host.load,openFiles:host.openFiles,openFilesMaximum:host.openFilesMaximum,attachedImages:host.attachedImages,governor:ports.environment.SEMIO_HOST_CAPACITY==="off"?"off" as const:"on" as const},
   capacity:{
    headroomMegabytes:status.evaluation.headroomMegabytes,
    grants:(status.grants as readonly GrantRow[]).map(grant=>({operation:grant.operation,pid:grant.pid,jobs:grant.jobs,reservedMegabytes:grant.reservedMegabytes,observedMegabytes:grant.observedMegabytes,heldSeconds:Math.max(0,Math.floor((now-grant.admittedAtMs)/1000))})),
    waiters:(status.waiters as readonly WaiterRow[]).map(waiter=>({operation:waiter.request.operation,pid:waiter.pid,needMegabytes:waiter.request.fixedMegabytes+waiter.request.minimumJobs*waiter.request.perJobMegabytes,waitedSeconds:Math.max(0,Math.floor((now-waiter.arrivedAtMs)/1000)),reason:reasons.get(waiter.request.id)??"queue-order"})),
   },
  }};
 },
});
