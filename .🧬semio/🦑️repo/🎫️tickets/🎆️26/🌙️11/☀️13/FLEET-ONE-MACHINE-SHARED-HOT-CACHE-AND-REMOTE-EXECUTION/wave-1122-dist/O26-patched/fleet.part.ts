
/** 🌐️ What one decision to start a pass on a fleet slot reads: the fleet jobs held here (`running`) and the most that may run (`maximum`), the worker slots that fit one more final right now (`slots`, `null` when the fleet does not answer), the free megabytes now (`null` when unreadable), the megabytes the fleet jobs here already charge, the hard floor that stays free, the memory pressure and the footprint of the job that asks. */
export type FleetJobAdmitHostV1=Readonly<{running:number;maximum:number;slots:number|null;availableMegabytes:number|null;reservedMegabytes:number;floorMegabytes:number;pressure:LocalJobPressureV1;footprintMegabytes:number}>;
/** 🎟️ The verdict: `admit`, why, and the megabytes that are free for a new footprint (free now less the footprints held and the hard floor). */
export type FleetJobAdmitDecisionV1=Readonly<{admit:boolean;reason:FleetJobReasonV1;freeMegabytes:number}>;

/** 🌐️ Decides whether one more pass may start on a fleet slot: while the cap is not reached, a worker slot that fits a final is free and the memory pressure is not critical, the pass is charged only its small local footprint against the free memory above the hard floor, so raised local memory pressure and the reservations of local finals never keep a fleet-placed pass away. Unreadable memory does not stop it either: its footprint is a few hundred megabytes, and the host warden acts on a real overshoot.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🌐️fleet/🐍️oracle/🐍️.py */
export function admitFleetComponentJobV1(hostValue:FleetJobAdmitHostV1):FleetJobAdmitDecisionV1{
 const host=requireDefinition<FleetJobAdmitHostV1>("FleetJobAdmitHost",hostValue),free=host.availableMegabytes===null?0:Math.max(-4194304,host.availableMegabytes-host.floorMegabytes-host.reservedMegabytes);
 const decide=(admit:boolean,reason:FleetJobReasonV1)=>Object.freeze(requireDefinition<FleetJobAdmitDecisionV1>("FleetJobAdmitDecision",{admit,reason,freeMegabytes:free}));
 if(host.running>=host.maximum)return decide(false,"fleet-cap");
 if(host.slots===null)return decide(false,"fleet-none");
 if(host.slots<=0)return decide(false,"fleet-busy");
 if(host.pressure==="critical")return decide(false,"fleet-pressure");
 if(host.availableMegabytes!==null&&host.footprintMegabytes>free)return decide(false,"fleet-memory");
 return decide(true,"fleet-fits");
}

/** 🌐️ The authored cap of concurrent component jobs (local and fleet slots together).
 * @see ./🧬️schema/🔣️.json */
export const componentJobsLimitV1:number=policy.componentJobsMaximum;

/** 📏️ The local footprint (megabytes) one fleet job is charged and the hard floor that stays free beside the fleet jobs: authored, measured on the play build (one pass held 14 rustc wrappers of about 32 MB and a cargo of 9 MB, 460 MB).
 * @see ./🧬️schema/🔣️.json */
export const componentFleetChargeV1={footprintMegabytes:policy.fleetPassFootprintMegabytes,floorMegabytes:policy.fleetFloorMegabytes,pollMilliseconds:policy.fleetPollMilliseconds} as const;

/** 🖨️ Describes a fleet slot event or plan in both languages on one line (English first, German second): `waiting` is worded by the kind of slot the pass waits for (`values.waitsFor`).
 * @see ./🧬️schema/🔣️.json */
export function describeFleetComponentJobsV1(name:"plan"|"started"|"waiting",values:Readonly<Record<string,string|number>>):string{
 return(["en","de"] as const).map(language=>{
  const catalogue=schema.definitions.Messages.const[language].fleet,key=name==="waiting"?({local:"waitingLocal",fleet:"waitingFleet",either:"waitingEither"} as const)[values.waitsFor as "local"|"fleet"|"either"]:name;
  const reason=values.reason,localReason=values.localReason;
  return fill(catalogue[key],{...values,...typeof reason==="string"&&reason in catalogue.reasons?{reason:catalogue.reasons[reason as keyof typeof catalogue.reasons]}:{},...typeof localReason==="string"&&localReason in schema.definitions.Messages.const[language].local.reasons?{lreason:schema.definitions.Messages.const[language].local.reasons[localReason as keyof typeof schema.definitions.Messages.const.en.local.reasons]}:{}});
 }).join(" / ");
}
