import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {cgroupMemoryFilesV1,effectiveCgroupMemoryV1,observeTreeMegabytesV1,parseCpuQuotaCoresV1,parseLinuxCountersV1,parseProcStatusV1,readHostCoresV1,readHostCountersV1,readLinuxProcessTableV1,treeMegabytesV1,type HostProbeV1} from "../../../📡️sampling/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import {hostCapacityDirectoryV1} from "../../../🎫️gate/🟦️.ts";

const oracle=new Ajv({strict:true});
const python=(code:string,...args:string[])=>Bun.spawnSync([testPythonV1()??"python3","-c",code,...args],{stdout:"pipe",stderr:"pipe"});
const hasPython=testPythonV1()!==null;
const probe=(files:Record<string,string|null>):HostProbeV1=>({platform:"linux",run:async()=>{throw Error("no command expected on Linux");},read:async path=>files[path]??null});

test("the cgroup chain is named innermost first for v2, v1 and hybrid hosts, as an independent pathlib walk says",()=>{
 const check=oracle.compile({type:"object",additionalProperties:false,required:["name","text","path","controller","levels"],properties:{name:{type:"string"},text:{type:["string","null"]},path:{type:["string","null"]},controller:{enum:["v1","v2","none"]},levels:{type:"array",items:{type:"string"},minItems:1}}});
 for(const row of fixture.membership){
  expect(check(row),row.name+JSON.stringify(check.errors)).toBe(true);
  const files=cgroupMemoryFilesV1(row.text),maximum=row.controller==="v1"?"memory.limit_in_bytes":"memory.max",current=row.controller==="v1"?"memory.usage_in_bytes":"memory.current";
  expect(files.map(file=>file.maximum.slice(0,-maximum.length-1)),row.name).toEqual(row.levels);
  expect(files.every(file=>file.maximum.endsWith("/"+maximum)&&file.current.endsWith("/"+current)),row.name).toBe(true);
  if(hasPython&&row.path!==null){
   const walked=python("import sys,pathlib\nbase=sys.argv[2]\np=pathlib.PurePosixPath(sys.argv[1])\nprint('\\n'.join(str(base)+('' if str(q)=='/' else str(q)) for q in [p,*p.parents] if True))",row.path,row.controller==="v1"?"/sys/fs/cgroup/memory":"/sys/fs/cgroup").stdout.toString().trim().split("\n").filter((value,index,all)=>all.indexOf(value)===index);
   expect(files.map(file=>file.maximum.slice(0,-(maximum.length+1))),row.name).toEqual(walked);
  }
 }
});

test("the effective memory limit is the level with the least room, as a SQLite minimum says",()=>{
 const db=new Database(":memory:");try{
  for(const row of fixture.effective){
   expect(effectiveCgroupMemoryV1(row.levels),row.name).toEqual(row.expect);
   const rows=row.levels.filter(level=>/^[0-9]+$/u.test(level.maximum?.trim()??"")&&Number(level.maximum!.trim())<2**62).map(level=>({maximum:level.maximum!.trim(),room:Number(level.maximum!.trim())-(/^[0-9]+$/u.test(level.current?.trim()??"")?Number(level.current!.trim()):0)}));
   const minimum=rows.length?db.query("select min(json_extract(value,'$.room')) as room from json_each(?)").get(JSON.stringify(rows))as{room:number}:undefined;
   expect(row.expect.maximum===null?undefined:Number(row.expect.maximum)-(row.expect.current===null?0:Number(row.expect.current.trim())),row.name).toBe(minimum?.room);
  }
 }finally{db.close();}
});

test("a container limit lowers the available memory through the whole read path and the Linux image count stays unknown",async()=>{
 const counters=await readHostCountersV1(probe({
  "/proc/meminfo":"MemTotal: 32768000 kB\nMemAvailable: 20971520 kB\nSwapTotal: 0 kB\nSwapFree: 0 kB\n","/proc/vmstat":"pswpout 0\n","/proc/sys/fs/file-nr":"100\t0\t1000\n",
  "/proc/self/cgroup":"0::/user.slice/limited.scope\n","/sys/fs/cgroup/user.slice/limited.scope/memory.max":"max\n","/sys/fs/cgroup/user.slice/limited.scope/memory.current":"1\n",
  "/sys/fs/cgroup/user.slice/memory.max":"4294967296\n","/sys/fs/cgroup/user.slice/memory.current":"1073741824\n","/sys/fs/cgroup/memory.max":null,"/proc/mounts":"/dev/loop1 /snap/a/1 squashfs ro 0 0\n"
 }));
 expect(counters.constrainedBytes).toBe(4294967296);expect(counters.availableBytes).toBe(3221225472);expect(counters.attachedImages).toBeNull();
 expect(parseLinuxCountersV1("MemAvailable: 33 kB\n","","",null,null,null).attachedImages).toBeNull();
});

test("the usable cores follow the tightest cgroup CPU quota, and other platforms report the logical cores",async()=>{
 for(const row of fixture.cpu)expect(parseCpuQuotaCoresV1(row.rows),JSON.stringify(row.rows)).toBe(row.cores);
 const files={"/proc/self/cgroup":"0::/a/b\n","/sys/fs/cgroup/a/b/cpu.max":"max 100000\n","/sys/fs/cgroup/a/cpu.max":"250000 100000\n","/sys/fs/cgroup/cpu.max":null};
 expect(await readHostCoresV1(probe(files),16)).toBe(3);expect(await readHostCoresV1(probe(files),2)).toBe(2);
 expect(await readHostCoresV1(probe({}),16)).toBe(16);expect(await readHostCoresV1({...probe(files),platform:"darwin"},16)).toBe(16);expect(await readHostCoresV1({...probe(files),platform:"win32"},16)).toBe(16);
});

test("the process tree reads /proc without any external tool: status rows, the listing noise skipped, a vanished process ignored",async()=>{
 for(const row of fixture.status)expect(parseProcStatusV1(row.pid,row.text),String(row.pid)).toEqual(row.row??undefined);
 const files=fixture.tree.files as Record<string,string>,read=async(path:string)=>files[path]??null,list=async()=>fixture.tree.listing;
 const rows=await readLinuxProcessTableV1(read,list);expect(rows.map(row=>row.pid).sort((a,b)=>a-b)).toEqual([1,10,11,12,13]);
 expect(treeMegabytesV1(rows,fixture.tree.root)).toBe(fixture.tree.megabytes);expect(treeMegabytesV1(rows,fixture.tree.absent)).toBeNull();
 const run=async()=>{throw Error("ps is not used on Linux");};
 expect(await observeTreeMegabytesV1(fixture.tree.root,run,"linux",{read,list})).toBe(fixture.tree.megabytes);
 expect(await observeTreeMegabytesV1(fixture.tree.root,run,"win32",{read,list})).toBeNull();
});

test.skipIf(process.platform!=="linux")("on a real Linux host the observed tree of a child equals what ps reports for it",async()=>{
 const child=Bun.spawn(["sleep","5"],{stdout:"ignore",stderr:"ignore"});try{
  await Bun.sleep(100);
  const observed=await observeTreeMegabytesV1(process.pid),ps=Bun.spawnSync(["ps","-A","-o","pid=,ppid=,rss="],{stdout:"pipe"}).stdout.toString();
  const rows=ps.trim().split("\n").map(line=>line.trim().split(/\s+/u).map(Number)),mine=new Set([process.pid]);let grew=true;while(grew){grew=false;for(const[pid,parent]of rows)if(!mine.has(pid!)&&mine.has(parent!)){mine.add(pid!);grew=true;}}
  const expected=Math.floor(rows.filter(([pid])=>mine.has(pid!)).reduce((sum,[,,rss])=>sum+rss!,0)/1024);
  expect(Math.abs(observed!-expected)).toBeLessThanOrEqual(8);
 }finally{child.kill();}
});

test("the machine-wide gate directory follows each platform's per-user cache and ignores a relative XDG cache home",()=>{
 const tail=["semio","host-capacity"];
 expect(hostCapacityDirectoryV1({},"linux","/home/u")).toBe(["/home/u/.cache",...tail].join("/"));
 expect(hostCapacityDirectoryV1({XDG_CACHE_HOME:"/var/cache/u"},"linux","/home/u")).toBe(["/var/cache/u",...tail].join("/"));
 expect(hostCapacityDirectoryV1({XDG_CACHE_HOME:"relative/cache"},"linux","/home/u")).toBe(["/home/u/.cache",...tail].join("/"));
 expect(hostCapacityDirectoryV1({XDG_CACHE_HOME:""},"linux","/home/u")).toBe(["/home/u/.cache",...tail].join("/"));
 expect(hostCapacityDirectoryV1({},"darwin","/Users/u")).toBe(["/Users/u/Library/Caches",...tail].join("/"));
});
