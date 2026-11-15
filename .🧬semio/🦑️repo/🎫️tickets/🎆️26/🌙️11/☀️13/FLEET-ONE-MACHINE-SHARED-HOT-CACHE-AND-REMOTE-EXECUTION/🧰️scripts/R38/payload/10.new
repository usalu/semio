/** 🏔️ Contract: on Linux the kernel limit of one slice load (`memory.max` of its leaf cgroup) never lies below the point where the plan guard settles and judges the load, and the cgroup of the service takes its limit from the machine, never from a constant. On g1-m04 (15848 MB, MemAvailable 8 to 10 GB) every try ended in 6339 MB (40 % of the machine, the plan limit) with `The plan cannot continue: the service holds N MB of its N MB memory limit`: the kernel killed the load at that limit without a settle, while the macOS guard (settle first, refuse only below 2048 MB available, at critical pressure or above 60 % of the machine) lets such a plan through. The cgroup v2 semantics used (`memory.max` is a hard limit that ends in an OOM kill of the group, `memory.high` only throttles, the effective limit of a group is the smallest along its ancestors, a write to `memory.max` of an existing group takes effect at once) are those of the kernel documentation, Documentation/admin-guide/cgroup-v2.rst.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {planGuardVerdictV1,sliceCeilingMegabytesV1,sliceLimitPolicyV1,slicePlanCeilingMegabytesV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs";
import {HOST_POLICY_V1} from "../../🟨️.mjs";
import {GRAPH_MEMORY_POLICY_V1,graphServiceLimitMegabytesV1} from "../../../🧠️memory/🟨️.mjs";
import {prepareServiceGroupV1,type CgroupPortsV1} from "../../../🧱️job-limits/🐧️cgroup/🟦️.ts";

const MiB=1048576,python=Bun.which("python3")??Bun.which("python");
const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostShareBasisPoints:number;hostFloorMegabytes:number;hostCapBasisPoints:number},guard=HOST_POLICY_V1.slicePlanGuard as {reserveMegabytes:number;ceilingBasisPoints:number},margin=Number(HOST_POLICY_V1.sliceSafetyMarginMegabytes);
type Case={host:number;current:number;available:number;learned:number};
const planLimitOf=(host:number)=>host>0&&host<GRAPH_MEMORY_POLICY_V1.serveHostMegabytes?sliceCeilingMegabytesV1({hostMegabytes:host,hostCapBasisPoints:elasticity.hostCapBasisPoints,maximumMegabytes:elasticity.maximumMegabytes}):Infinity;
const policyOf=(item:Case,guarded:boolean)=>{
 const service=graphServiceLimitMegabytesV1(item.host),planCeilingMegabytes=guarded?slicePlanCeilingMegabytesV1({hostMegabytes:item.host,ceilingBasisPoints:guard.ceilingBasisPoints,planLimitMegabytes:planLimitOf(item.host)}):0;
 return sliceLimitPolicyV1({limitBytes:(guarded?service:4096)*MiB,currentBytes:item.current*MiB,availableBytes:item.available*MiB},margin,{...elasticity,hostMegabytes:item.host,planCeilingMegabytes},item.learned)!.memoryMegabytes;
};
const oracle=(cases:Case[])=>{
 const code=`import json,sys
d=json.load(sys.stdin);E=d['e'];G=d['g'];M=d['m'];S=d['s']
INF=float('inf')
def svc(h,share,minimum):return max(minimum,int(h*share)) if h>0 else minimum
def sliceceil(h):return min(E['maximumMegabytes'],h*E['hostCapBasisPoints']//10000) if h>0 else E['maximumMegabytes']
def planlimit(h):return sliceceil(h) if 0<h<S['serveHostMegabytes'] else INF
def planceil(h):
  if h<=0:return 0
  c=max(h*G['ceilingBasisPoints']//10000,planlimit(h));return 0 if c==INF else c
def policy(limit,cur,avail,h,learned,guarded):
  declared=limit-cur-M;lent=min(E['maximumMegabytes'],avail*E['hostShareBasisPoints']//10000-E['hostFloorMegabytes'])
  g=planceil(h)+M if guarded and planceil(h)>0 else 0
  return max(64,declared,lent,g,min(sliceceil(h),learned))
out=[]
for c in d['c']:
  out.append({'service':svc(c['host'],S['serviceHostShare'],S['serviceMinimumMegabytes']),'new':policy(svc(c['host'],S['serviceHostShare'],S['serviceMinimumMegabytes']),c['current'],c['available'],c['host'],c['learned'],True),'old':policy(4096,c['current'],c['available'],c['host'],c['learned'],False),'ceiling':planceil(c['host'])})
print(json.dumps(out))`;
 const run=spawnSync(python!,["-I","-c",code],{input:JSON.stringify({c:cases,e:elasticity,g:guard,m:margin,s:{serveHostMegabytes:GRAPH_MEMORY_POLICY_V1.serveHostMegabytes,serviceHostShare:GRAPH_MEMORY_POLICY_V1.serviceHostShare,serviceMinimumMegabytes:GRAPH_MEMORY_POLICY_V1.serviceMinimumMegabytes}}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as {service:number;new:number;old:number;ceiling:number}[];
};
const cases:Case[]=[
 {host:15848,current:1200,available:8123,learned:0},{host:15848,current:1400,available:9200,learned:6339},{host:15848,current:1400,available:10473,learned:7924},
 {host:16384,current:1500,available:8000,learned:6553},{host:8192,current:1000,available:4000,learned:3276},{host:4096,current:900,available:2000,learned:1638},
 {host:32768,current:1500,available:20000,learned:8192},{host:0,current:1000,available:5000,learned:0},{host:15848,current:200,available:3000,learned:0},
];

test("the kernel limit of a load equals an independent Python computation, lies at the plan ceiling plus the margin on every machine below the serve size, and does not rise where the guard is off",()=>{
 const expected=oracle(cases);
 cases.forEach((item,index)=>{
  expect(policyOf(item,true),JSON.stringify(item)).toBe(expected[index]!.new);
  expect(policyOf(item,false),JSON.stringify(item)).toBe(expected[index]!.old);
  if(expected[index]!.ceiling>0)expect(policyOf(item,true)).toBeGreaterThanOrEqual(expected[index]!.ceiling+margin);
 });
 expect(expected[0]!.ceiling).toBe(9508);expect(expected[0]!.new).toBe(9508+margin);
 expect(expected[6]!.ceiling).toBe(0);expect(slicePlanCeilingMegabytesV1({hostMegabytes:32768,ceilingBasisPoints:guard.ceilingBasisPoints,planLimitMegabytes:planLimitOf(32768)})).toBe(0);
});

test("g1-m04: the old limit of 6339 MB killed a load the guard would settle and let run, the new limit leaves every verdict of the guard to the guard",()=>{
 const item=cases[1]!,old=policyOf(item,false),next=policyOf(item,true),planLimit=planLimitOf(item.host),ceiling=slicePlanCeilingMegabytesV1({hostMegabytes:item.host,ceilingBasisPoints:guard.ceilingBasisPoints,planLimitMegabytes:planLimit});
 expect(old).toBe(6339);expect(planLimit).toBe(6339);expect(ceiling).toBe(9508);expect(next).toBe(9508+margin);
 const verdict=(tree:number,over:{available?:number;pressure?:string;settled?:boolean})=>planGuardVerdictV1({treeMegabytes:tree,limitMegabytes:planLimit,ceilingMegabytes:ceiling,availableMegabytes:over.available??8123,reserveMegabytes:guard.reserveMegabytes,pressure:over.pressure as never,settled:over.settled??false});
 expect(verdict(7000,{})).toEqual({action:"settle"});
 expect(verdict(7000,{settled:true,pressure:"warn"})).toEqual({action:"continue"});
 expect(verdict(7000,{settled:true,available:1500})).toEqual({action:"refuse",reason:"memory"});
 expect(verdict(7000,{settled:true,pressure:"critical"})).toEqual({action:"refuse",reason:"pressure"});
 expect(verdict(9600,{})).toEqual({action:"refuse",reason:"ceiling"});
 expect(7000>old).toBe(true);expect(7000<next&&ceiling<next).toBe(true);
});

test("the service cgroup takes its limit from the machine: 60 percent of the memory, never below the minimum, and no constant is left in the policy or the spawn",()=>{
 const expected=oracle(cases);
 cases.forEach((item,index)=>expect(graphServiceLimitMegabytesV1(item.host)).toBe(expected[index]!.service));
 expect(graphServiceLimitMegabytesV1(15848)).toBe(9508);expect(graphServiceLimitMegabytesV1(16384)).toBe(9830);expect(graphServiceLimitMegabytesV1(1024)).toBe(GRAPH_MEMORY_POLICY_V1.serviceMinimumMegabytes);expect(graphServiceLimitMegabytesV1(Number.NaN)).toBe(GRAPH_MEMORY_POLICY_V1.serviceMinimumMegabytes);
 expect(GRAPH_MEMORY_POLICY_V1).not.toHaveProperty("serviceLimitMegabytes");
 const spawn=readFileSync(fileURLToPath(new URL("../../🏗️composition/🌐️host/🟨️.mjs",import.meta.url)),"utf8");
 expect(spawn).not.toContain("serviceLimitMegabytes");expect(spawn).toContain("graphServiceLimitMegabytesV1(");
});

test("a service group that an older revision created with 4096 MB is corrected by the next start through plain file writes, without root",()=>{
 const files=new Map<string,string>(),dirs=new Set<string>(),root="/sys/fs/cgroup",directory=join(root,"user.slice/user@1000.service/app.slice/semio-service-graph-x");
 files.set(join(root,"cgroup.controllers"),"cpuset cpu io memory pids\n");files.set(join(root,"user.slice/user@1000.service/app.slice/cgroup.subtree_control"),"cpu memory pids\n");
 const ports:CgroupPortsV1={root,read:path=>{const text=files.get(path);if(text===undefined)throw Object.assign(Error("ENOENT"),{code:"ENOENT"});return text;},write:(path,text)=>{files.set(path,text);},mkdir:path=>{if(dirs.has(path))throw Object.assign(Error("EEXIST"),{code:"EEXIST"});dirs.add(path);},rmdir:path=>{dirs.delete(path);},list:path=>[...dirs].filter(row=>row.startsWith(path+"/")).map(row=>row.slice(path.length+1)),writable:path=>path===join(root,"user.slice/user@1000.service/app.slice"),own:()=>"/user.slice/user@1000.service/app.slice/session.scope",sleep:async()=>undefined,pid:4242};
 dirs.add(directory);files.set(join(directory,"memory.max"),String(4096*MiB));files.set(join(directory,"memory.high"),String(Math.floor(4096*MiB*0.9)));
 const made=prepareServiceGroupV1({name:"graph-x",limitMegabytes:graphServiceLimitMegabytesV1(15848),softRatioBasisPoints:9000},ports);
 expect(made).toBe(directory);
 expect(files.get(join(directory,"memory.max"))).toBe(String(9508*MiB));expect(files.get(join(directory,"memory.high"))).toBe(String(Math.floor(9508*MiB*0.9)));
 expect(files.get(join(directory,"memory.swap.max"))).toBe("0");expect(files.get(join(directory,"memory.oom.group"))).toBe("1");
});
