import corpus from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions/🔣️.json";
const api=await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts");
const graph=corpus.graph as any,plan=api.planBazelTasksV1(graph,corpus.request),emitted=api.bazelActionsV1(graph,plan,false);
const groups=new Map([["i_0123456789abcdef",{groups:[],files:["lib/src/a.ts","lib/src/deep/c.ts"]}],["i_fedcba9876543210",{groups:[],files:["app/x.ts","bun.lock","package.json"]}]]),sources:Record<string,string>={"lib:generate":"i_0123456789abcdef","app:build":"i_fedcba9876543210","app:test":"i_fedcba9876543210"};
const text=api.renderBazelBuildV1({actions:[...emitted.values()].map(action=>({...action,...(sources[action.id]?{sources:sources[action.id]}:{})})),groups,bun:"/opt/bun",sourceIdentity:"a".repeat(64),extension:""});
console.log(JSON.stringify(text.split("\n")));
