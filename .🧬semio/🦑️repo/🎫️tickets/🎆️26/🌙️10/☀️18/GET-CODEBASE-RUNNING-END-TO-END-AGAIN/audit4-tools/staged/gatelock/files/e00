import type {HostCapacityPolicyV1,HostRequestV1} from "../../📏️decision/🟦️.ts";
import type {HostRunStatusV1} from "../../📐️estimates/🟦️.ts";
import type {HostSampleRecordV1} from "../../📡️sampling/🟦️.ts";
import type {HostPhaseResizeV1,HostPhaseSegmentV1,HostPhaseV1} from "../../🪜️phases/🟦️.ts";
import type {checkResourceLeaseControl,ResourceLeaseControl} from "../../../../🔒️leases/🎛️control/🟦️.ts";
import type {waitingDelayV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 🪪️ Who a process is for the handover: its id and the start time that tells a recycled id from the original. */
export type HostProcessIdentityV1=Readonly<{pid:number;started:string}>;
/** 🔌️ The ports of one gate: time, process identity, host samples, process-tree observation, how long the process has only been waiting in queues, which child processes it has, the identity of its parent and the start time of itself; tests replace them. */
export type HostCapacityPortsV1=Readonly<{now?:()=>number;pid?:number;parent?:()=>Promise<HostProcessIdentityV1|undefined>;self?:()=>Promise<string|undefined>;sample?:(previous?:Readonly<{atMs:number;swappedOutMegabytes:number|null}>)=>Promise<HostSampleRecordV1>;observe?:(pid:number)=>Promise<number|null>;waited?:(nowMs:number)=>number;children?:(pid:number)=>readonly number[]|undefined}>;
/** 🏁️ What the gate saw of one finished grant: the jobs it ran, the highest megabytes its process tree held at a heartbeat, how long it was actively working (not lent to a queue wait), after how much of that time the tree first came within 5 % of its peak, when it ended and whether its body completed, failed or was cancelled; `phases` are the segments of a run that marked its phases; `executed` says, for a forecast-aware run, whether work had to run (its grant was upgraded or a process of the run asked to start work). */
export type HostRunV1=Readonly<{jobs:number;peakMegabytes:number;activeMilliseconds:number;peakAtMilliseconds:number;endedAtMs:number;status:HostRunStatusV1;phases?:readonly HostPhaseSegmentV1[];executed?:boolean}>;
/** ⚙️ Everything one admission needs. */
export type HostCapacityOptionsV1=HostCapacityPortsV1&Readonly<{directory:string;policy:HostCapacityPolicyV1;request:HostRequestV1;control:ResourceLeaseControl;environment?:Readonly<Record<string,string|undefined>>;write?:(line:string)=>void;session?:Readonly<{address:string;weight:number}>;park?:(error:unknown)=>boolean|Promise<boolean>;learn?:(run:HostRunV1)=>void|Promise<void>;upgrade?:HostRequestV1;resize?:(phase:HostPhaseV1,context:Readonly<{jobs:number;reservedMegabytes:number;activeMilliseconds:number}>)=>HostPhaseResizeV1|undefined}>;
/** 🧰️ The private primitives of the ledger the handover module works with, handed over by value so that it imports nothing from here and loads only when a handover happens. */
export type HostLedgerPrimitivesV1=Readonly<{protocol:typeof schema.definitions.Protocol.const;takeLock:(directory:string,now:number,policy:HostCapacityPolicyV1,pid:number)=>boolean;releaseLock:(directory:string,pid:number)=>void;holdsLock:(directory:string,pid:number)=>boolean;grantPath:(directory:string,id:string)=>string;readJson:(path:string)=>unknown;writeJson:(path:string,value:unknown)=>void;remove:(path:string)=>void;alive:(pid:number)=>boolean;bootAt:(directory:string,now:number)=>number;modified:(path:string)=>number;delay:typeof waitingDelayV1;checkResourceLeaseControl:typeof checkResourceLeaseControl}>;
