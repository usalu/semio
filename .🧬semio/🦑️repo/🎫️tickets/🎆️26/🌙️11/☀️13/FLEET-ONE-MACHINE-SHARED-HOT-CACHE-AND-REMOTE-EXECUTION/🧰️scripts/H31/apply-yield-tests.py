import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from patchlib import rep, atomic

root = sys.argv[1]
H = os.path.join(root, "🧪️tests", "🪧️holder", "🟦️.ts")
T = os.path.join(root, "🧪️tests", "🟦️.ts")

rep(H, 'import {graphQueueYieldPolicyV1,waitGraphQueueYieldV1,yieldGraphQueueV1} from "../../🟨️.mjs";',
 'import {graphQueueYieldPolicyV1,waitGraphQueueYieldV1,yieldGraphQueueV1} from "../../🟨️.mjs";\nimport {readProcessStartV1} from "../../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟨️.mjs";')
rep(H, 'const door=()=>{', 'const ownStart=()=>Math.round(Date.now()-process.uptime()*1000);\nconst door=(holder:{pid:number;startedMs:number}={pid:process.pid,startedMs:ownStart()})=>{')
rep(H, 'holder:{pid:process.pid,command:"script.ts repo show projects",startedMs:Date.UTC(2026,10,17,16,0,0),digest:"c".repeat(64)}}));',
 'holder:{pid:holder.pid,command:"script.ts repo show projects",startedMs:holder.startedMs,digest:"c".repeat(64)}}));')

tests = '''
const gone=async()=>{const child=Bun.spawn([process.execPath,"-e","0"]);await child.exited;return child.pid;};
const resume=(directory:string,root:string,name:string,read:()=>number=()=>7)=>{
 const ticket=join(root,name+".json"),lines:string[]=[];writeFileSync(ticket,JSON.stringify({version:1,generation:7,pointer:"/p/current.json",requestedAt:1_000_000,round:1,at:1,door:directory}));
 return{lines,run:()=>waitGraphQueueYieldV1({environment:{[names.ticket]:ticket,SEMIO_LANGUAGE:""},write:text=>lines.push(text),read})};
};

test("a door whose only holder already ended is not waited for: the front door starts again at once, in under a second, and says so in both languages",async()=>{
 const{root,directory}=door({pid:await gone(),startedMs:Date.now()-60_000}),{lines,run}=resume(directory,root,"ended");
 const began=performance.now(),next=await run(),took=performance.now()-began;
 expect(took).toBeLessThan(1000);expect(next).toEqual({[names.requestedAt]:"1000000",[names.round]:"2"});
 expect(lines.at(-1)!).toContain("no live holder is ahead any more");expect(lines.at(-1)!).toContain("kein lebender Halter ist mehr vor diesem Lauf");
});

test("a pid reused by another process counts as a dead holder: the recorded start time must match, not only the pid",async()=>{
 const{root,directory}=door({pid:process.pid,startedMs:ownStart()-86_400_000}),{lines,run}=resume(directory,root,"reused");
 const began=performance.now();await run();expect(performance.now()-began).toBeLessThan(1000);expect(lines.at(-1)!).toContain("no live holder is ahead any more");
});

test("a live holder is still waited for until the generation advances; the poll is the door poll, not the generation poll",async()=>{
 const{root,directory}=door(),ticket=join(root,"live.json"),lines:string[]=[],sleeps:number[]=[];
 writeFileSync(ticket,JSON.stringify({version:1,generation:7,pointer:"/p/current.json",requestedAt:1_000_000,round:1,at:1,door:directory}));
 let clock=1_000_000;const start=clock;
 await waitGraphQueueYieldV1({environment:{[names.ticket]:ticket,SEMIO_LANGUAGE:""},now:()=>clock,sleep:async milliseconds=>{sleeps.push(milliseconds);clock+=milliseconds;},write:text=>lines.push(text),read:()=>clock-start>=5000?8:7});
 expect(lines.at(-1)!).toContain("generation 8");expect(lines.join("\\n")).not.toContain("no live holder");
 expect(new Set(sleeps)).toEqual(new Set([graphQueueYieldPolicyV1().doorPollMilliseconds]));expect(clock-start).toBe(5000);
});

test("a holder that ends while the front door waits is skipped at the next door poll",async()=>{
 const child=Bun.spawn(["sleep","60"]),started=readProcessStartV1(child.pid);expect(typeof started).toBe("number");
 const{root,directory}=door({pid:child.pid,startedMs:started as number}),{lines,run}=resume(directory,root,"dies");
 const pending=run();await Bun.sleep(1200);expect(lines.join("\\n")).not.toContain("no live holder");
 child.kill(9);await child.exited;const killed=performance.now();await pending;
 expect(performance.now()-killed).toBeLessThan(1000);expect(lines.at(-1)!).toContain("no live holder is ahead any more");
});
'''
text = open(H, encoding="utf-8").read()
atomic(H, text.rstrip("\n") + "\n" + tests)

rep(T, 'for(const key of ["waiting","resumed"] as const){', 'for(const key of ["waiting","resumed","holderGone"] as const){')
print("yield tests patched")
