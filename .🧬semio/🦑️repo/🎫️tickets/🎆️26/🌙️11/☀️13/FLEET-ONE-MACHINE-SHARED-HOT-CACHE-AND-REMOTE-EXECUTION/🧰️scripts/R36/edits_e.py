"""R36 part 2: learned owner peaks are clamped to a ceiling. The 23 GB of garbage a stretch of the inference allocated were recorded as the peak of the owner kind (owner-peaks.json: project 23101 MB, cargo 9617, typescript 7154) and the governor admitted every later owner of the kind only after waitBoundMilliseconds, alone: 3 owners per minute."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
M=B+"🧠️memory/🟨️.mjs"
add(M,'.map(([kind,samples])=>[kind,samples.slice(-policy.peakSamples)]));','.map(([kind,samples])=>[kind,samples.slice(-policy.peakSamples).map(bytes=>Math.min(bytes,policy.peakCeilingMegabytes*MEGABYTE))]));')
add(M,'learned.set(owner.kind,[...samples,owner.taken].slice(-policy.peakSamples));','learned.set(owner.kind,[...samples,Math.min(owner.taken,policy.peakCeilingMegabytes*MEGABYTE)].slice(-policy.peakSamples));')
S=B+"🧠️memory/🧬️schema/🔣️.json"
add(S,'"peakSamples": 8,','"peakSamples": 8,\n        "peakCeilingMegabytes": 1024,')
TF=B+"🧠️memory/🦶️footprint/🧪️tests/🟦️.ts"
add(TF,'import{releaseIfGrownV1,','import{createMemoryGovernorV1}from"../../🟨️.mjs";\nimport{releaseIfGrownV1,')
add(TF,'//R36b\n','''//R36b
test("a learned peak that garbage inflated is clamped to the ceiling, persisted clamped, and the owners of the kind are admitted together at once instead of one per wait bound",async()=>{
 const ceiling=1024*MB,written:Record<string,number[]>[]=[];let time=0;
 const governor=createMemoryGovernorV1({workers:4,headroom:()=>6000*MB,estimateBytes:256*MB,sleep:async(milliseconds:number)=>{time+=milliseconds;},now:()=>time,peaks:{read:()=>({project:[23101*MB,14*MB,9617*MB]}),write:value=>{written.push(value as Record<string,number[]>);}}});
 let running=0,peak=0;
 await Promise.all([1,2,3,4].map(index=>governor.run(async()=>{running++;peak=Math.max(peak,running);await new Promise(resolve=>setTimeout(resolve,20));running--;},{kind:"project",key:"p"+index})));
 expect(peak).toBeGreaterThanOrEqual(4);
 expect(time).toBeLessThan(1000);
 expect(governor.stats().kinds.project).toBeLessThanOrEqual(1024);
 for(const row of written)for(const samples of Object.values(row))for(const bytes of samples)expect(bytes).toBeLessThanOrEqual(ceiling);
});
''')
