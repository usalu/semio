import os
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🛰️plan/🟦️.ts"
s=open(P,encoding="utf8").read()
anchor='test("only repo cache options override the outer task fast path",async()=>{'
assert s.count(anchor)==1
new='''test("a task state with a lease runs the whole keying section inside it and one without runs the section plain",async()=>{
 const graph=graphOf(["app"]),task="app:build",events:string[]=[],derived={key:()=>task,verify:async()=>{events.push("verify");return [];},getMany:async()=>{events.push("keys");return [{value:{key:"a".repeat(64)}}];}};
 const recorder={upToDate:async()=>{events.push("up-to-date");return new Set<string>();}};
 const leased=async<T>(body:()=>Promise<T>):Promise<T>=>{events.push("enter");try{return await body();}finally{events.push("leave");}};
 for(const[name,state]of[["leased",{graph:derived,recorder,leased}],["plain",{graph:derived,recorder}]] as const){
  events.length=0;
  const planner=createGraphPlannerPortV1({loadGraph:async()=>graph,taskState:async()=>state}),index=await planner.load({root,workspace:"lease-"+name});
  const answer=await planner.plan({index,targets:[task],workspace:"lease-"+name,root,agent:"test",arguments:["run",task]});
  expect(answer.tasks[0]!.stale).toBe(true);
  expect(events,name).toEqual(name==="leased"?["enter","verify","keys","up-to-date","leave"]:["verify","keys","up-to-date"]);
 }
});

'''
s=s.replace(anchor,new+anchor)
open(P+".f121tmp","w",encoding="utf8").write(s);os.replace(P+".f121tmp",P)
