/** 🚦️ Contract: a read that makes no owner progress for the declared time stops with a plain text in English and German that names what it waits on, and a large machine is recognised by the memory and cores it declares.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{TAKE_POLICY_V1,createProgressWatchV1,takeNoProgressTextV1,takeStrongMachineV1}from"../../🟨️.mjs";

test("the policy declares five minutes without owner progress",()=>{
 expect(TAKE_POLICY_V1.noProgressMilliseconds).toBe(300000);
 expect(TAKE_POLICY_V1.watchMilliseconds).toBeGreaterThan(0);
 expect(TAKE_POLICY_V1.watchMilliseconds).toBeLessThanOrEqual(TAKE_POLICY_V1.noProgressMilliseconds/10);
});

test("a sign that never changes stalls the watch after the declared time and not before",()=>{
 let at=0;
 const watch=createProgressWatchV1({now:()=>at,windowMilliseconds:300000});
 expect(watch.check(7)).toBeUndefined();
 at=299999;expect(watch.check(7)).toBeUndefined();
 at=300000;expect(watch.check(7)).toEqual({sinceMilliseconds:300000});
});

test("every change of the sign restarts the time",()=>{
 let at=0;
 const watch=createProgressWatchV1({now:()=>at,windowMilliseconds:300000});
 watch.check(1);
 for(let step=1;step<=20;step++){at=step*200000;expect(watch.check(step+1)).toBeUndefined();}
 at+=300000;expect(watch.check(21)).toEqual({sinceMilliseconds:300000});
});

test("the text names the owners that run, the machine that holds them and what the read waits on, in both languages",()=>{
 const fields={minutes:5,done:22,total:4775,running:"procedural-generation2d-rs (2325 s, 0 rows), mit-bestand-demonstrator (828 s, 0 rows)",waiting:"memory headroom (21842 of 14336 MB reserved)"};
 const en=takeNoProgressTextV1("en",fields),de=takeNoProgressTextV1("de",fields);
 for(const text of[en,de]){expect(text).toContain("procedural-generation2d-rs");expect(text).toContain("memory headroom");expect(text).toContain("22");expect(text).toContain("4775");expect(text).not.toContain("{");}
 expect(en).toContain("no owner");
 expect(de).toContain("kein Projekt");
});

test("a machine is large when it has the declared memory and cores, and only then",()=>{
 expect(takeStrongMachineV1({memoryMegabytes:32768,cores:10})).toBe(true);
 expect(takeStrongMachineV1({memoryMegabytes:16384,cores:10})).toBe(false);
 expect(takeStrongMachineV1({memoryMegabytes:32768,cores:4})).toBe(false);
});
