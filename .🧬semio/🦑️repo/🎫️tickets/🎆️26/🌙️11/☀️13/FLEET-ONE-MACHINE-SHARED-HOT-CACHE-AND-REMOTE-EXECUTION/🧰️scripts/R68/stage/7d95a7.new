import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The numbers of the report and the checkpoint of the ownership walk.
 * @see ./🧬️schema/🔣️.json */
export const OWNERSHIP_WALK_POLICY_V1=schema.definitions.Policy.const;
/** 🗣️ The report line in both languages. */
export const OWNERSHIP_WALK_TEXTS_V1=schema.definitions.Texts.const;

/** 🕓️ What the walk remembers between two steps. */
export type OwnershipWalkStateV1=Readonly<{startMs:number;reportMs:number;reportCount:number;checkpointMs:number;checkpointCount:number}>;
/** 📈️ The numbers of one report: steps so far, seconds since the start, steps per second since the start and since the last report. */
export type OwnershipWalkReportV1=Readonly<{operation:string;completedOperations:number;elapsedSeconds:number;perSecond:number;recentPerSecond:number}>;
/** 🕓️ What one step asks for: the new state, a report line, a checkpoint of the settled listings. */
export type OwnershipWalkTickV1=Readonly<{state:OwnershipWalkStateV1;report?:OwnershipWalkReportV1;checkpoint?:true}>;

/** 🕓️ The state at the start of a walk. */
export const ownershipWalkStartV1=(nowMs:number):OwnershipWalkStateV1=>({startMs:nowMs,reportMs:nowMs,reportCount:0,checkpointMs:nowMs,checkpointCount:0});

/** 🕓️ Decides at one step (`count` steps completed at `nowMs`): a report when the report interval has passed, a checkpoint when the checkpoint interval has passed and enough steps are new. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function ownershipWalkTickV1(state:OwnershipWalkStateV1,input:Readonly<{nowMs:number;operation:string;count:number}>,policy:typeof OWNERSHIP_WALK_POLICY_V1=OWNERSHIP_WALK_POLICY_V1):OwnershipWalkTickV1{
 const {nowMs,operation,count}=input;
 let next=state,report:OwnershipWalkReportV1|undefined,checkpoint=false;
 if(nowMs-state.reportMs>=policy.reportMilliseconds){
  const seconds=Math.max(0.001,(nowMs-state.startMs)/1000),recent=Math.max(0.001,(nowMs-state.reportMs)/1000);
  report={operation,completedOperations:count,elapsedSeconds:Math.round(seconds),perSecond:Math.round(count/seconds),recentPerSecond:Math.round((count-state.reportCount)/recent)};
  next={...next,reportMs:nowMs,reportCount:count};
 }
 if(nowMs-state.checkpointMs>=policy.checkpointMilliseconds&&count-state.checkpointCount>=policy.checkpointMinimumOperations){checkpoint=true;next={...next,checkpointMs:nowMs,checkpointCount:count};}
 return{state:next,...report===undefined?{}:{report},...checkpoint?{checkpoint:true as const}:{}};
}

/** 🗣️ The report as a human line in the language of the user. */
export function ownershipWalkLineV1(report:OwnershipWalkReportV1,language:"en"|"de"):string{
 return OWNERSHIP_WALK_TEXTS_V1[language].report.replace("{count}",String(report.completedOperations)).replace("{seconds}",String(report.elapsedSeconds)).replace("{rate}",String(report.perSecond)).replace("{recent}",String(report.recentPerSecond)).replace("{operation}",report.operation);
}
