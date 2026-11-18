import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ✂️ The declared ladder of the graph work cut: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const GRAPH_CUT_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));

/** 🪜 The cut of one level: level 0 is the usual work (no cap), every level halves the projects inferred at once and the share of the owner wave, never below one project and the narrowest wave. */
export function graphCutV1(level,policy=GRAPH_CUT_POLICY_V1){
 const step=Number.isFinite(level)?Math.min(policy.levels,Math.max(0,Math.floor(level))):0;
 return Object.freeze(step===0?{level:0,waveBasisPoints:10000}:{level:step,concurrency:Math.max(1,Math.floor(policy.maximumConcurrency/2**step)),waveBasisPoints:Math.max(policy.minimumWaveBasisPoints,Math.floor(10000/2**step))});
}

/** 📉 The level a load that grew to `treeMegabytes` against `limitMegabytes` is cut to: one level for each doubling it was over, at least one; `undefined` at the last level (nothing is left to cut). */
export function graphCutAfterV1({level,treeMegabytes,limitMegabytes},policy=GRAPH_CUT_POLICY_V1){
 const doublings=limitMegabytes>0&&treeMegabytes>limitMegabytes?Math.ceil(Math.log2(treeMegabytes/limitMegabytes)):1,next=Math.min(policy.levels,level+Math.max(1,doublings));
 return next>level?next:undefined;
}

/** 🏁 The level a load starts at on a machine whose plan may take `limitMegabytes`: the doublings between the footprint of uncut work and the limit; 0 where the plan has no limit. */
export function graphCutStartLevelV1({limitMegabytes},policy=GRAPH_CUT_POLICY_V1){
 if(!(limitMegabytes>0)||!Number.isFinite(limitMegabytes))return 0;
 return Math.min(policy.levels,Math.max(0,Math.ceil(Math.log2(policy.unboundedMegabytes/limitMegabytes))));
}

/** 🌱 The environment a cut load runs in: the concurrency cap of its level (none at level 0). */
export function graphCutEnvironmentV1(cut,policy=GRAPH_CUT_POLICY_V1){
 return cut.concurrency===undefined?{}:{[policy.concurrencyEnvironment]:String(cut.concurrency)};
}

/** 🔢 The projects the inference of this process may run at once under the cut of its environment; Infinity where it is not cut. */
export function graphCutConcurrencyV1(environment=process.env,policy=GRAPH_CUT_POLICY_V1){
 const value=Number(environment[policy.concurrencyEnvironment]);
 return Number.isSafeInteger(value)&&value>=1?value:Infinity;
}

/** 🧠 The level the next load starts at after a load finished at `level` with a tree of at most `peakMegabytes`: the same level while the peak was near the limit, one level less cut when it was under `relaxBasisPoints` of it (a transient load must not keep a machine cut for ever). */
export function graphCutLearnedV1({level,peakMegabytes,limitMegabytes},policy=GRAPH_CUT_POLICY_V1){
 return level>0&&peakMegabytes<limitMegabytes*policy.relaxBasisPoints/10000?level-1:level;
}
