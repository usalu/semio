import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const.eta;
/** ⏲️ One estimate: the seconds left, whether it came from the holder's own counter or from earlier holds, and whether the hold is overdue. */
export type LeaseEtaV1=Readonly<{seconds:number;source:"progress"|"history";overdue:boolean}>;
/** 🧾️ The evidence of an estimate: elapsed times, the holder's own counter and the durations of earlier holds. */
export type LeaseEtaInputV1=Readonly<{elapsedMs:number;phaseElapsedMs:number;done?:number;total?:number;historyMs:readonly number[]}>;

/** 🧮️ Median of the most recent hold durations, or undefined when there are too few samples. */
export function leaseMedianV1(durations:readonly number[]):number|undefined{
 const rows=durations.slice(-policy.maximumSamples).filter(Number.isFinite).sort((left,right)=>left-right);
 if(rows.length<policy.minimumSamples)return undefined;
 const middle=rows.length>>1;return rows.length%2?rows[middle]!:(rows[middle-1]!+rows[middle]!)/2;
}

/** ⏳️ Time left of a holder: the rate of its own counter when it reports one (linear extrapolation of the current phase), else the median of earlier holds of the same kind minus the time already held; a hold past `overdueFactor` times the median is overdue and reports no number. */
export function leaseEtaV1(input:LeaseEtaInputV1):LeaseEtaV1|undefined{
 const{done,total}=input;
 if(done!==undefined&&total!==undefined&&total>0&&done<=total&&done/total>=policy.minimumFraction&&input.phaseElapsedMs>0){
  return Object.freeze({seconds:Math.round(input.phaseElapsedMs/done*(total-done)/1000),source:"progress",overdue:false});
 }
 const median=leaseMedianV1(input.historyMs);
 if(median===undefined)return undefined;
 if(input.elapsedMs>median*policy.overdueFactor)return Object.freeze({seconds:0,source:"history",overdue:true});
 return Object.freeze({seconds:Math.round(Math.max(0,median-input.elapsedMs)/1000),source:"history",overdue:input.elapsedMs>median});
}
