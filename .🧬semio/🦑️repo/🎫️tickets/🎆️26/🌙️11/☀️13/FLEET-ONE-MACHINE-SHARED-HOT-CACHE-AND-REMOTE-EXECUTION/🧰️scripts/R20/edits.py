"""R20: the serving claim stays alive, the serving read never waits silently. Anchored edits (rel path from the worktree root, old, new, count)."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
RX=B+"🌐️remote-execution/"
SC=RX+"🛰️sidecar/"
WG=B+"🪪️graph-state/🙋️wanted/"
TS="🟦️.ts"
E=[]
def edit(rel,old,new,count=1):E.append((rel,old,new,count))

STALL=SC+"🕸️graph/🔬️stall/"+TS
edit(STALL,'''  if(blocker===undefined)return waited>0?(input.say(wantedTextV1("leaseWaitFree",input.language,{resource:last?.resource??"",seconds:Math.round(waited/1000)})),waited):0;''','''  if(blocker===undefined){
   if(last===undefined)return 0;
   input.say(wantedTextV1("leaseWaitFree",input.language,{resource:last.resource,seconds:Math.round(waited/1000)}));return waited;
  }''')
edit(SC+"🧪️tests/🕸️graph/🔬️stall/"+TS,'''const fakeRepo=(script:string)=>''','''test("a start that finds no blocker says nothing, however long looking took",async()=>{
 const lines:string[]=[];let time=0;
 expect(await awaitGraphReadLeasesV1({blockers:()=>{time+=2000;return[];},language:"en",say:line=>lines.push(line),signal:undefined,now:()=>time,sleep:async()=>{throw new Error("must not sleep");}})).toBe(0);
 expect(lines).toEqual([]);
});

const fakeRepo=(script:string)=>''')
