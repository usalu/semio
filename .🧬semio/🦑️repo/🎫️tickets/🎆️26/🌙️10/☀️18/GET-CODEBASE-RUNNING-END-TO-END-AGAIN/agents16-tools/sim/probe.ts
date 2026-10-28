const root=process.argv[2]!;
const t=performance.now();
const hc=await import(root+"/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts");
const bz=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts");
console.log("[DEBUG] loaded",Math.round(performance.now()-t),"ms",Object.keys(hc).length,typeof bz.planBazelServersV1);
const pol=(await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json",{with:{type:"json"}})).default;
const policy=hc.readHostCapacityPolicyV1(pol.hostCapacity);
console.log(bz.planBazelServersV1({cores:8,memoryMegabytes:16384},policy.memory));
