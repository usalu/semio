import type {DoctorAreaV1,DoctorFactsV1,DoctorPolicyV1} from "../🟦️.ts";
import type {DoctorPortsV1} from "../🔌️ports/🟦️.ts";
import {toolsProbeV1} from "./🔧️tools/🟦️.ts";
import {storageProbeV1} from "./💾️storage/🟦️.ts";
import {leasesProbeV1} from "./🔒️leases/🟦️.ts";
import {capacityProbeV1} from "./🎫️capacity/🟦️.ts";
import {sessionsProbeV1} from "./🪪️sessions/🟦️.ts";
import {graphProbeV1} from "./🕸️graph/🟦️.ts";
import {dependenciesProbeV1} from "./📦️dependencies/🟦️.ts";
import {cachesProbeV1} from "./⚡️caches/🟦️.ts";
import {processesProbeV1} from "./🏃️processes/🟦️.ts";

/** 🧩️ The sections of the facts a probe may fill. */
export type DoctorSectionsV1=Partial<Pick<DoctorFactsV1,"host"|"storage"|"tools"|"leases"|"capacity"|"sessions"|"graph"|"dependencies"|"caches"|"processes">>;
/** 🎛️ What a probe knows besides the ports: the policy, the processes earlier probes found holding or waiting, where the library sources live and when it must stop. */
export type DoctorProbeContextV1=Readonly<{policy:DoctorPolicyV1;known:ReadonlySet<number>;libraryRoot:string;deadlineMs:number}>;
/** 📥️ What a probe returns: facts, or the reason there is nothing to read here. */
export type DoctorProbeResultV1=Readonly<{sections:DoctorSectionsV1}>|Readonly<{unavailable:string}>;
/** 🛠️ What a safe repair did, or would do in a dry run. */
export type DoctorRepairRunV1=Readonly<{removed:number;bytes:number;status?:"nothing"|"unavailable";reason?:string}>;
/** 🔧️ One declared safe repair; it re-derives its targets from the live state and never trusts an earlier report. */
export type DoctorRepairV1=Readonly<{id:string;area:DoctorAreaV1;run(ports:DoctorPortsV1,context:DoctorProbeContextV1,dryRun:boolean):Promise<DoctorRepairRunV1>}>;
/** 🔎️ One probe: it reads the live state of one area through the ports and never changes it. */
export type DoctorProbeV1=Readonly<{area:DoctorAreaV1;phase:1|2;read(ports:DoctorPortsV1,context:DoctorProbeContextV1):Promise<DoctorProbeResultV1>;repairs?:readonly DoctorRepairV1[]}>;

/** 🗂️ Every probe in the order the report lists them; phase 2 needs the pids phase 1 found. */
export const DOCTOR_PROBES_V1:readonly DoctorProbeV1[]=Object.freeze([toolsProbeV1,storageProbeV1,leasesProbeV1,capacityProbeV1,sessionsProbeV1,graphProbeV1,dependenciesProbeV1,cachesProbeV1,processesProbeV1]);
/** 🧰️ Every declared repair by id. */
export const DOCTOR_REPAIRS_V1:ReadonlyMap<string,DoctorRepairV1>=new Map(DOCTOR_PROBES_V1.flatMap(probe=>(probe.repairs??[]).map(repair=>[repair.id,repair] as const)));
