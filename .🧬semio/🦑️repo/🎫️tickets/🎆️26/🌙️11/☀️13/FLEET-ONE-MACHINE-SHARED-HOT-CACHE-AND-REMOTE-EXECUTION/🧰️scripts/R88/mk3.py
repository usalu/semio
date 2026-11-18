S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R88/st1/"
def rd(n):return open(S+n,encoding="utf8").read()
def wr(n,t):open(S+n,"w",encoding="utf8").write(t)
def sub(t,old,new,count=1):
    assert t.count(old)==count,(old[:70],t.count(old));return t.replace(old,new)
t=rd("4.base")
t=sub(t,"import{SLICE_REPORT_POLICY_V1 as P,summarizeHeapSnapshotV1,buildSliceReportV1,","import{SLICE_REPORT_POLICY_V1 as P,measureSliceProcessesV1,parseProcessTreeV1,sharedBytesV1,buildSliceReportV1,")
t=sub(t,'import{spawnSync}from"node:child_process";','import{spawn,spawnSync}from"node:child_process";')
t=sub(t,"const r=startSliceReportV1({root:process.argv[2],intervalMilliseconds:50});","const r=startSliceReportV1({root:process.argv[2],footprint:{self:async()=>200*1048576,tree:async()=>300*1048576},intervalMilliseconds:50});")
t=sub(t,"expect(validateJsonSchemaSubset({$ref:\"#/definitions/Report\"},last,schema)).toEqual([]);\n});","""expect(validateJsonSchemaSubset({$ref:"#/definitions/Report"},last,schema)).toEqual([]);
 expect(last.footprintMegabytes).toBe(200);expect(last.guard.measure).toBe("heapUsed+external");
 expect(Math.abs(last.guard.megabytes-(last.memory.heapUsed+last.memory.external))).toBeLessThan(2);
 if(process.platform!=="win32"){expect(last.processes.self.rssMegabytes).toBeGreaterThan(0);expect(last.processes.count).toBe(0);expect(last.treeFootprintMegabytes).toBeUndefined();}
});""")
a=t.index('test("a heap snapshot is summarised');b=t.index('test("collections are estimated')
new='''test("a structure shared by many entries is charged once, not once per referrer",()=>{
 const members=Array.from({length:150},(_all,at)=>Array.from({length:500},(_item,item)=>"m"+at+"-"+item));
 const entries=Array.from({length:120},()=>({sidecar:{facts:members.map((_member,at)=>({at})),members:members.slice()}}));
 const seen=new Set();let total=0;for(const entry of entries)total+=8+sharedBytesV1(entry.sidecar,seen);
 const inner=150*(16+500*8),perEntry=16+(16+(16+150*(8+32)))+(16+(16+150*8));
 expect(total).toBe(120*(8+perEntry)+inner);
 expect(total).toBeLessThan(2*1048576);
 expect(estimateCollectionBytesV1(120,entries.slice(0,16).map(entry=>entry.sidecar))).toBeGreaterThan(total*20);
 expect(sharedBytesV1(new Set([1,2,3]))).toBe(16+3*8);expect(sharedBytesV1(new Map([[1,{a:1}]]))).toBe(16+16+(16+16));
 const cycle:{self?:unknown}={};cycle.self=cycle;expect(sharedBytesV1(cycle)).toBe(16+16);
});

test("a process table names the descendants of a process by role with their resident and virtual sizes and leaves everything else out",()=>{
 const table=[" 100     1   2048   4096 /usr/bin/bun /x/👷️worker.ts --flag","\\n 101   100  10240  20480 /usr/bin/bun /x/🏭️walk-farm.mjs 1","\\n 102   101   5120   8192 /usr/bin/node child.js","\\n 103     1  99999  99999 /usr/bin/other","\\n 104   100    900   1000 ps -A -o pid=,ppid=,rss=,vsz=,command=","\\n garbage line"].join("");
 const tree=parseProcessTreeV1(table,100);
 expect(tree).toEqual({self:{rssMegabytes:2,vszMegabytes:4},count:2,rssMegabytes:15,children:[{pid:101,role:"🏭️walk-farm.mjs",rssMegabytes:10,vszMegabytes:20},{pid:102,role:"child.js",rssMegabytes:5,vszMegabytes:8}]});
 expect(parseProcessTreeV1(table,100,1).children.length).toBe(1);expect(parseProcessTreeV1(table,555)).toEqual({count:0,rssMegabytes:0,children:[]});
});

test("real processes: the children of this process are found with their roles and the resident size that ps reports for each of them",async()=>{
 if(process.platform==="win32")return;
 const root=temp("census"),script=join(root,"probe-kid.mjs");writeFileSync(script,"const keep=new Uint8Array(24*1048576).fill(1);setTimeout(()=>keep.length,30000);");
 const kids=[spawn(process.execPath,[script],{stdio:"ignore"}),spawn(process.execPath,[script],{stdio:"ignore"})];
 try{
  await new Promise(resolve=>setTimeout(resolve,1500));
  const row=await measureSliceProcessesV1({footprint:{self:async()=>7*1048576}});
  expect(row.footprintMegabytes).toBe(7);expect(row.processes!.count).toBeGreaterThanOrEqual(2);
  const found=row.processes!.children.filter(child=>child.role==="probe-kid.mjs");
  expect(found.map(child=>child.pid).sort()).toEqual(kids.map(kid=>kid.pid!).sort());
  for(const child of found){
   const oracle=Number(spawnSync("ps",["-o","rss=","-p",String(child.pid)],{encoding:"utf8"}).stdout.trim())/1024;
   expect(child.rssMegabytes).toBeGreaterThan(20);expect(Math.abs(child.rssMegabytes-oracle)).toBeLessThan(Math.max(8,oracle*0.25));
  }
  expect(row.treeFootprintMegabytes).toBeUndefined();
  expect((await measureSliceProcessesV1({footprint:{tree:async()=>11*1048576}})).treeFootprintMegabytes).toBe(11);
 }finally{for(const kid of kids)kid.kill();}
},60000);

/** 🧱️ A heap snapshot of a 500 MB heap held 4 GB of resident pages for the rest of the process (measured: 613 MB to 4 780 MB, not returned by a collection) and its summary made the plan guard (heapUsed plus external) see 10 GB. The report must never walk the heap, even where the caller still passes a snapshot function. */
test("real process: a report that runs for 24 seconds beside a 560 MB heap keeps the guard number under 2 GB and the resident set under 3 GB",()=>{
 const root=temp("noheap"),script=join(root,"worker.mjs");
 writeFileSync(script,`import{startSliceReportV1}from${JSON.stringify(join(import.meta.dir,"..","🟨️.mjs"))};
const MB=1048576,keep=[];while(process.memoryUsage().heapUsed/MB<560){for(let at=0;at<200;at++)keep.push({id:at,name:"p"+Math.random(),rows:["a"+at,"b"+at,{x:at}]});}
const report=startSliceReportV1({root:process.argv[2],snapshot:()=>globalThis.Bun.generateHeapSnapshot(),intervalMilliseconds:250});
let guard=0,rss=0;const probe=setInterval(()=>{const usage=process.memoryUsage();guard=Math.max(guard,(usage.heapUsed+usage.external)/MB);rss=Math.max(rss,usage.rss/MB);},100);
await new Promise(done=>setTimeout(done,24000));clearInterval(probe);report.stop();
const usage=process.memoryUsage();console.log(JSON.stringify({guard:Math.max(guard,(usage.heapUsed+usage.external)/MB),rss:Math.max(rss,usage.rss/MB),kept:keep.length}));`);
 const run=spawnSync(process.execPath,[script,root],{encoding:"utf8",timeout:150000}),row=JSON.parse(run.stdout.trim().split("\\n").pop()!);
 expect(row.kept).toBeGreaterThan(1000);expect(row.guard).toBeLessThan(2000);expect(row.rss).toBeLessThan(3000);
},170000);

'''
t=t[:a]+new+t[b:]
wr("4.new",t)
