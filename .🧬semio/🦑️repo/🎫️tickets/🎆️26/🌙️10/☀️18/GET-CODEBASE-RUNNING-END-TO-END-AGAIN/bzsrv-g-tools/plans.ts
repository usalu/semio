import {planBazelServersV1,bazelGrowthSlotIdsV1,BAZEL_SESSION_POLICY_V1 as policy} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts";
const memory={floorBasisPoints:2000,floorMinimumMegabytes:4096,floorMaximumMegabytes:8192};
for(const [gb,cores] of [[8,4],[16,8],[16,10],[32,10],[64,16]] as const){
 const plan=planBazelServersV1({cores,memoryMegabytes:gb*1024},memory);
 console.log(JSON.stringify({gb,cores,...plan,growth:bazelGrowthSlotIdsV1(plan).length,maxServers:plan.slots+bazelGrowthSlotIdsV1(plan).length,worstHeapGb:(plan.slots+bazelGrowthSlotIdsV1(plan).length)*plan.heapMegabytes/1024}));
}
