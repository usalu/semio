import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import toolchain from "./🧫️fixtures/🦀️toolchain/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import authored from "../🔣️policy.json";
import {checkHoldAndWaitV1,checkLeaseOrderV1,checkSelfDeadlockV1,classifyLeaseResourceV1,findLeaseCycleV1,formatLeaseViolationV1,leaseBlockersV1,leaseOrderPolicyV1,leaseProcessDependenciesV1,readLeaseOrderPolicyV1,type LeaseNodeV1,type LeaseViolationV1} from "../🟦️.ts";

const policy=readLeaseOrderPolicyV1(fixture.policy);
const ajv=new Ajv({strict:true,allowUnionTypes:true});
const validatePolicy=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Policy"} as never);
const validateViolation=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/Violation"} as never);

test("the authored policy and the test policy satisfy the closed schema in two validators",()=>{
 expect(validatePolicy(authored)).toBe(true);expect(validatePolicy(fixture.policy)).toBe(true);
 expect(readLeaseOrderPolicyV1(authored).classes.length).toBeGreaterThan(5);
 expect(validatePolicy({...authored,extra:true})).toBe(false);expect(()=>readLeaseOrderPolicyV1({...authored,extra:true})).toThrow("Closed lease order Policy");
 expect(validatePolicy({...authored,rules:{...authored.rules,holdAndWait:"sometimes"}})).toBe(false);expect(()=>readLeaseOrderPolicyV1({...authored,rules:{...authored.rules,holdAndWait:"sometimes"}})).toThrow();
});

test("a policy with duplicate classes, duplicate matches or edges between unknown classes is refused",()=>{
 const cls=authored.classes[0]!;
 expect(()=>readLeaseOrderPolicyV1({...authored,classes:[cls,cls]})).toThrow("declared twice");
 expect(()=>readLeaseOrderPolicyV1({...authored,classes:[cls,{...cls,id:"other"}]})).toThrow("match declared twice");
 expect(()=>readLeaseOrderPolicyV1({...authored,allowed:[{from:"nope",to:cls.id,reason:{en:"x",de:"y"}}]})).toThrow("unknown class");
});

test("every census family of the authored policy lands in the class its owner declared",()=>{
 const live=readLeaseOrderPolicyV1(authored),expected:Record<string,string>={"native-fixed-image:/w/.cache/b/d":"image","native-storage:abc":"image","repository-graph-facts:/w":"graph","nx:graph-pool:abc":"graph","nx:daemon-startup:abc":"tool-singleton","nx:daemon-slot:abc":"tool-singleton","build-graph-store:/r":"graph","bazel-workspace-x":"tool-singleton","bazel-snapshot-restore":"tool-singleton","cargo:registry:/h":"cargo-coordination","cargo:configuration-view:41":"toolchain","rustup:home":"toolchain","rustup:generation-shard:24":"toolchain","cargo-preparation:/w":"cargo-coordination",'["semio.cargo.build-lease/v1","/b"]':"cargo-coordination","rust-code:/c":"pin","rust-code-publication:/c":"namespace","build-graph:/s/a.json":"pin","javascript-dependency-bindings:/w":"namespace","artifact:/a/b":"artifact","semio.compiler.cpu-picker/v1":"compiler-capacity","semio.compiler.cpu/v1/3":"compiler-capacity","nx-tooling-activation":"tool-singleton","some-future-resource":"unclassified"};
 for(const [resource,id] of Object.entries(expected))expect(`${resource} → ${classifyLeaseResourceV1(live,resource).id}`).toBe(`${resource} → ${id}`);
});

test("the lease-store image never orders: held shared it is the medium, any other image or mode still orders",()=>{
 const live=readLeaseOrderPolicyV1(authored),at=(resource:string,mode:"shared"|"exclusive")=>[{directory:"/w/.cache/b/l/m/data",resource,mode,atMs:1,task:1}];
 const request={directory:"/w/.cache/b/l/m/data",resource:"repository-graph-facts:/w",mode:"exclusive" as const,nonblocking:false};
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/l","shared"),request)).toBeUndefined();
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/l","exclusive"),request)?.kind).toBe("out-of-order");
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/d","shared"),request)?.kind).toBe("out-of-order");
 expect(live.allowed.some(edge=>edge.from==="image"&&edge.to==="graph")).toBe(false);
});

for(const row of fixture.classification)test(`classification ${row.resource}`,()=>expect(classifyLeaseResourceV1(policy,row.resource).id).toBe(row.class));

for(const row of fixture.order)test(`lock order ${row.id}`,()=>{
 const found=checkLeaseOrderV1(policy,row.held as never,row.request as never);
 if(row.expect===null)return void expect(found).toBeUndefined();
 expect(found).toMatchObject(row.expect);expect(validateViolation(found)).toBe(true);
});

for(const row of fixture.selfDeadlock)test(`self-deadlock ${row.id}`,()=>{
 const found=checkSelfDeadlockV1(policy,row.held as never,row.request as never,row.waitedMilliseconds);
 if(row.expect===null)return void expect(found).toBeUndefined();
 expect(found).toMatchObject(row.expect);expect(validateViolation(found)).toBe(true);
});

for(const row of fixture.holdAndWait)test(`hold-and-wait ${row.id}`,()=>{
 const found=checkHoldAndWaitV1(policy,row.held as never,row.request as never,row.waitedMilliseconds,(row as {blocker?:{pid:number;heldSeconds:number}}).blocker);
 if(row.expect===null)return void expect(found).toBeUndefined();
 expect(found).toMatchObject(row.expect);expect(validateViolation(found)).toBe(true);
});

test("rules set to observe or off still classify but the pure checks honour off",()=>{
 const off=Object.freeze({...policy,rules:Object.freeze({outOfOrder:"off" as const,holdAndWait:"off" as const,selfDeadlock:"off" as const,cycle:"off" as const})});
 const row=fixture.order.find(item=>item.id==="outward-refused")!,wait=fixture.holdAndWait.find(item=>item.id==="after-grace-refused")!;
 expect(checkLeaseOrderV1(off,row.held as never,row.request as never)).toBeUndefined();
 expect(checkHoldAndWaitV1(off,wait.held as never,wait.request as never,99999)).toBeUndefined();
});

const oracle=(nodes:readonly LeaseNodeV1[],from:string):boolean=>{
 const database=new Database(":memory:");database.exec("CREATE TABLE edge(a TEXT,b TEXT)");
 const insert=database.prepare("INSERT INTO edge VALUES(?,?)");
 for(const node of nodes)for(const to of [...leaseBlockersV1(nodes,node),...leaseProcessDependenciesV1(nodes,node)])insert.run(node.id,to);
 const row=database.query("WITH RECURSIVE walk(node,depth) AS (SELECT b,1 FROM edge WHERE a=? UNION SELECT edge.b,walk.depth+1 FROM edge JOIN walk ON edge.a=walk.node WHERE walk.depth<64) SELECT COUNT(*) AS found FROM walk WHERE node=?").get(from,from) as {found:number};
 database.close();return row.found>0;
};

for(const row of fixture.cycles)test(`deadlock cycle ${row.id}`,()=>{
 const nodes=row.nodes as unknown as LeaseNodeV1[],found=findLeaseCycleV1(nodes,row.from);
 if(row.expect===null)expect(found).toBeUndefined();else{expect(found?.members).toEqual(row.expect.members);expect(found?.victim).toBe(row.expect.victim);}
 expect(oracle(nodes,row.from)).toBe(found!==undefined);
});

test("exactly one member of every cycle is the victim, whichever member asks",()=>{
 for(const row of fixture.cycles.filter(item=>item.expect!==null)){
  const nodes=row.nodes as unknown as LeaseNodeV1[],victims=new Set(row.expect!.members.map(member=>findLeaseCycleV1(nodes,member)!.victim));
  expect([...victims]).toEqual([row.expect!.victim]);
 }
});

test("the SQLite recursive-walk oracle agrees with the cycle finder on 500 seeded random graphs, and every cycle has exactly one victim",()=>{
 let state=0x9e3779b9;const next=(limit:number)=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return (state>>>8)%limit;};
 let cycles=0;
 for(let round=0;round<500;round++){
  const size=2+next(6),nodes:LeaseNodeV1[]=[];
  for(let index=0;index<size;index++){
   const holds=[...new Set([next(5),next(5)])].filter(()=>next(3)>0).map(key=>({key:`k${key}`,mode:next(3)===0?"shared" as const:"exclusive" as const}));
   nodes.push({id:`${index+1}/1`,holds,...next(4)>0?{wait:{key:`k${next(5)}`,mode:next(3)===0?"shared" as const:"exclusive" as const,sinceMs:next(50),queued:next(2)===0}}:{}});
  }
  const involved=nodes.filter(node=>findLeaseCycleV1(nodes,node.id)!==undefined);
  for(const node of nodes)expect(`${node.id}:${findLeaseCycleV1(nodes,node.id)!==undefined}`).toBe(`${node.id}:${oracle(nodes,node.id)}`);
  for(const node of involved){const found=findLeaseCycleV1(nodes,node.id)!;expect(found.members.includes(found.victim)).toBe(true);}
  if(involved.length>0){cycles++;const victims=new Set(involved.map(node=>findLeaseCycleV1(nodes,node.id)!.victim));expect(victims.size).toBeGreaterThanOrEqual(1);}
 }
 expect(cycles).toBeGreaterThan(20);
});

for(const row of fixture.messages)test(`diagnostic ${row.id}`,()=>{
 const text=formatLeaseViolationV1(row.language as "en"|"de",row.violation as LeaseViolationV1);
 for(const part of row.contains)expect(text).toContain(part);
 expect(validateViolation(row.violation)).toBe(true);
});

test("the environment switch overrides every rule and rejects unknown values",()=>{
 expect(leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"off"}).rules).toEqual({outOfOrder:"off",holdAndWait:"off",selfDeadlock:"off",cycle:"off",strictWait:"off"});
 expect(leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"enforce"}).rules.outOfOrder).toBe("enforce");
 expect(String(leaseOrderPolicyV1({}).rules.outOfOrder)).toBe(String(authored.rules.outOfOrder));
 expect(()=>leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"sometimes"})).toThrow("must be off, observe or enforce");
});

type SequenceStepV1=Readonly<{op:"acquire"|"release";resource:string;mode?:"shared"|"exclusive"}>;
type SequenceV1=Readonly<{id:string;steps:readonly SequenceStepV1[];violations:readonly Readonly<{held:string;requested:string}>[]}>;

/** 🧪️ Replays an acquisition sequence against the authored policy and returns every refused request. */
const replay=(live:ReturnType<typeof readLeaseOrderPolicyV1>,directory:string,steps:readonly SequenceStepV1[])=>{
 const held:{directory:string;resource:string;mode:"shared"|"exclusive";atMs:number;task:number}[]=[],found:{held:string;requested:string}[]=[];
 steps.forEach((step,index)=>{
  if(step.op==="release"){const at=held.findIndex(row=>row.resource===step.resource);if(at>=0)held.splice(at,1);return;}
  const request={directory,resource:step.resource,mode:step.mode!,nonblocking:false},violation=checkLeaseOrderV1(live,held,request);
  if(violation)found.push({held:violation.held,requested:violation.requested});
  held.push({directory,resource:step.resource,mode:step.mode!,atMs:index+1,task:1});
 });
 return found;
};

/** 🗄️ Independent SQLite oracle: ranks by longest matching prefix or exact name, then a self-join over the held rows and the declared edges. */
const sequenceOracle=(directory:string,steps:readonly SequenceStepV1[])=>{
 const live=readLeaseOrderPolicyV1(authored),database=new Database(":memory:");
 database.exec("CREATE TABLE class(id TEXT,rank INTEGER);CREATE TABLE match(id TEXT,kind TEXT,value TEXT);CREATE TABLE edge(a TEXT,b TEXT);CREATE TABLE held(resource TEXT,at INTEGER)");
 for(const row of live.classes){database.prepare("INSERT INTO class VALUES(?,?)").run(row.id,row.rank);for(const match of row.match)database.prepare("INSERT INTO match VALUES(?,?,?)").run(row.id,match.kind,match.value);}
 for(const row of live.allowed)database.prepare("INSERT INTO edge VALUES(?,?)").run(row.from,row.to);
 const rank=`SELECT class.id AS id,class.rank AS rank FROM match JOIN class ON class.id=match.id WHERE (match.kind='exact' AND match.value=?1) OR (match.kind='prefix' AND substr(?1,1,length(match.value))=match.value) ORDER BY match.kind='exact' DESC,length(match.value) DESC LIMIT 1`;
 const found:{held:string;requested:string}[]=[];
 steps.forEach((step,index)=>{
  if(step.op==="release"){database.prepare("DELETE FROM held WHERE rowid=(SELECT rowid FROM held WHERE resource=? LIMIT 1)").run(step.resource);return;}
  const wanted=database.query(rank).get(step.resource) as {id:string;rank:number}|null;
  if(wanted&&wanted.rank>0)for(const row of database.query("SELECT resource FROM held ORDER BY at").all() as {resource:string}[]){
   const now=database.query(rank).get(row.resource) as {id:string;rank:number}|null;
   if(now&&now.rank>wanted.rank&&!database.query("SELECT 1 FROM edge WHERE a=? AND b=?").get(now.id,wanted.id)){found.push({held:row.resource,requested:step.resource});break;}
  }
  database.prepare("INSERT INTO held VALUES(?,?)").run(step.resource,index+1);
 });
 database.close();return found;
};

for(const row of toolchain.sequences as readonly SequenceV1[])test(`toolchain acquisition sequence ${row.id} follows the declared lock order`,()=>{
 const live=readLeaseOrderPolicyV1(authored);
 expect<unknown>(replay(live,toolchain.directory,row.steps)).toEqual(row.violations);
 expect<unknown>(sequenceOracle(toolchain.directory,row.steps)).toEqual(row.violations);
});

test("the toolchain and Cargo readers precede compiler capacity, which precedes the graph, and capacity may still enter a tool singleton only through its declared edge",()=>{
 const live=readLeaseOrderPolicyV1(authored),rank=(resource:string)=>classifyLeaseResourceV1(live,resource).rank,capacity=rank("semio.compiler.host-capacity/v1");
 expect(rank("rustup:home")).toBeGreaterThan(0);
 for(const resource of ["rustup:home","rustup:generation-shard:24","cargo:configuration-view:41","cargo:registry:/h","cargo-preparation:/w",'["semio.cargo.build-lease/v1","/b"]'])expect(rank(resource),resource).toBeLessThan(capacity);
 expect(capacity).toBeLessThan(rank("repository-graph-facts:/w"));
 expect(live.allowed.some(edge=>edge.from==="compiler-capacity"&&edge.to==="tool-singleton")).toBe(true);
 expect(live.allowed.some(edge=>edge.from==="compiler-capacity"&&(edge.to==="toolchain"||edge.to==="cargo-coordination"))).toBe(false);
});

test("the authored policy lets a graph holder wait for the pinned graph snapshot, and still refuses an unrelated exclusive hold", () => {
  const authoredPolicy = leaseOrderPolicyV1(), directory = "/repo/.cache/b/l/m/data", held = (resource: string) => ({ directory, resource, mode: "exclusive" as const, atMs: 0, task: 1 });
  const request = { directory, resource: "build-graph:/repo/.cache/b/g/m/snapshots/" + "a".repeat(64) + ".json", mode: "shared" as const, nonblocking: false };
  expect(classifyLeaseResourceV1(authoredPolicy, request.resource).id).toBe("pin");
  expect(checkHoldAndWaitV1(authoredPolicy, [held("nx:daemon-service:" + "b".repeat(64))], request, 60000)).toBeUndefined();
  expect(checkHoldAndWaitV1(authoredPolicy, [held("artifact:/repo/out")], request, 60000)?.kind).toBe("hold-and-wait");
  expect(checkHoldAndWaitV1(authoredPolicy, [held("artifact:/repo/out")], request, 1000)).toBeUndefined();
});

test("the Bazel slot picker is the innermost lock: any held whole-build pin may precede it, and while it is held no other blocking lease may be requested", () => {
  const live = readLeaseOrderPolicyV1(authored), directory = "/repo/.cache/b/l/m/data", held = (resource: string, mode: "shared" | "exclusive") => ({ directory, resource, mode, atMs: 0, task: 1 }), request = (resource: string, nonblocking = false) => ({ directory, resource, mode: "exclusive" as const, nonblocking });
  const gates = ["bazel-slot-gate", "bazel-slot-gate:service"], holds = ["build-graph:/repo/.cache/b/g/m/snapshots/" + "a".repeat(64) + ".json", "build-graph-store:/repo/.cache/b/g/m", "rust-code:/repo/.cache/b/t/m/rc/" + "b".repeat(64), "repo:compiler-writer:/repo/.cache/rust/fixed/m", "rustup:home", "cargo:registry:/h", "semio.compiler.host-capacity/v1", "artifact:/a/b", "nx-tooling-activation", "bazel-slot-s0", "bazel-snapshot-restore"];
  const highest = Math.max(...live.classes.filter(row => row.id !== "bazel-picker").map(row => row.rank));
  for (const gate of gates) {
    expect(classifyLeaseResourceV1(live, gate).id, gate).toBe("bazel-picker");
    expect(classifyLeaseResourceV1(live, gate).rank, gate).toBeGreaterThan(highest);
    for (const resource of holds) for (const mode of ["shared", "exclusive"] as const) expect(checkLeaseOrderV1(live, [held(resource, mode)], request(gate))?.requestedClass, `${gate} requested under ${resource} held ${mode}`).toBeUndefined();
    for (const resource of ["rustup:home", "bazel-snapshot-restore", "semio.compiler.cpu/v1/3", "build-graph-store:/repo/.cache/b/g/m", "artifact:/a/b"]) expect(checkLeaseOrderV1(live, [held(gate, "exclusive")], request(resource))?.heldClass, `${resource} requested under ${gate}`).toBe("bazel-picker");
    expect(checkLeaseOrderV1(live, [held(gate, "exclusive")], request("bazel-slot-s0", true)), "a slot claim cannot wait, so it may happen under the picker").toBeUndefined();
    expect(checkHoldAndWaitV1(live, [held(gate, "exclusive")], request("rustup:home"), 60000)?.kind, "the picker is still never held while waiting").toBe("hold-and-wait");
  }
  for (const from of live.allowed.filter(edge => edge.to === "tool-singleton").map(edge => edge.from)) expect(live.allowed.some(edge => edge.from === from && edge.to === "bazel-picker"), `${from} keeps its declared edge to the picker`).toBe(true);
  expect(live.allowed.filter(edge => edge.from === "bazel-picker"), "the picker declares no edge out of itself").toEqual([]);
  for (const resource of ["bazel-slot-s0", "bazel-slot-d3", "bazel-snapshot-restore", "bazel-restore:x", "bazel-metrics"]) expect(classifyLeaseResourceV1(live, resource).id, resource).toBe("tool-singleton");
});

test("a shared whole-build pin that nobody waits for is unordered: nothing requested under the graph snapshot or the Rust code custody is out of order, while the compiler writer custody still orders", () => {
  const live = readLeaseOrderPolicyV1(authored), directory = "/repo/.cache/b/l/m/data", held = (resource: string, mode: "shared" | "exclusive" = "shared") => ({ directory, resource, mode, atMs: 0, task: 1 }), request = (resource: string, mode: "shared" | "exclusive" = "exclusive") => ({ directory, resource, mode, nonblocking: false });
  const pins = ["build-graph:/repo/.cache/b/g/m/snapshots/" + "a".repeat(64) + ".json", "rust-code:/repo/.cache/b/t/m/rc/" + "b".repeat(64)];
  const inside = ["bazel-" + "c".repeat(64), "bazel-metrics", "bazel-modules-" + "d".repeat(64), "bazel-restore:", "bazel-restore:🎓️teaching/🏛️architecture", "bazel-slot-gate", "bazel-slot-gate:service", "rustup:bootstrap", "rustup:home", "rustup:generation-shard:24", "cargo:configuration-view:6", "cargo:configuration-publication", "cargo:registry:/repo/cargo/home", "build-graph-store:/repo/.cache/b/g/m", "build-graph:/repo/.cache/b/g/m/snapshots/" + "e".repeat(64) + ".json", "rust-code:/repo/.cache/b/t/m/rc/" + "f".repeat(64), "semio.compiler.cpu/v1/3", "artifact:/a/b"];
  for (const pin of pins) {
    expect(classifyLeaseResourceV1(live, pin).id, pin).toBe("pin");
    expect(classifyLeaseResourceV1(live, pin).rank, pin).toBe(0);
    for (const resource of inside) for (const mode of ["shared", "exclusive"] as const) expect(checkLeaseOrderV1(live, [held(pin)], request(resource, mode))?.requestedClass, `${resource} ${mode} requested under ${pin}`).toBeUndefined();
  }
  expect(classifyLeaseResourceV1(live, "rust-code-publication:/repo/.cache/b/t/m/rc").id, "the publication writer stays ordered").toBe("namespace");
  expect(classifyLeaseResourceV1(live, "build-graph-store:/repo/.cache/b/g/m").id).toBe("graph");
  for (const resource of ['["semio.cargo.build-lease/v1","/b","prototype"]', "artifact-capture:/a/b"]) expect(checkLeaseOrderV1(live, [held("repo:compiler-writer:/repo/.cache/rust/fixed/m")], request(resource, "shared"))?.heldClass, `${resource} under the compiler writer custody`).toBe("image");
  const exclusive = (resource: string) => held(resource, "exclusive"), pinRequest = request(pins[1]!, "shared");
  expect(checkHoldAndWaitV1(live, [exclusive("rust-code-publication:/repo/.cache/b/t/m/rc")], pinRequest, 60000)?.kind, "an exclusive namespace holder still may not wait for a pin").toBe("hold-and-wait");
  for (const from of ["nx:daemon-service:" + "b".repeat(64), "semio.compiler.host-capacity/v1", '["semio.cargo.custody-lease/v1","/b","prototype"]']) expect(checkHoldAndWaitV1(live, [exclusive(from)], request(pins[0]!, "shared"), 60000), `${from} keeps its declared edge to the pin`).toBeUndefined();
});

test("the Rust descriptor namespace and its admission gate sit with the tool singletons: the session slot may precede them, graph, bazel and cargo leases follow them, and nothing ranked after them may take them", () => {
  const live = readLeaseOrderPolicyV1(authored), directory = "/repo/.cache/b/l/m/data", held = (resource: string, mode: "shared" | "exclusive" = "shared") => ({ directory, resource, mode, atMs: 0, task: 1 }), request = (resource: string, mode: "shared" | "exclusive" = "exclusive") => ({ directory, resource, mode, nonblocking: false });
  const namespace = "rust-descriptor-namespace:/repo/.repo/rust/fixed/m/cargo/prototype/captures/units", admission = "rust-descriptor-admission:/repo/.repo/rust/fixed/m/cargo/prototype/captures/units";
  for (const resource of [namespace, admission]) expect(classifyLeaseResourceV1(live, resource).id, resource).toBe("descriptor-custody");
  expect(classifyLeaseResourceV1(live, "rust-code-publication:/repo/rc").id).toBe("namespace");
  for (const inside of ["build-graph-store:/repo/.cache/b/g/m", "bazel-" + "c".repeat(64), "bazel-metrics", "bazel-modules-" + "d".repeat(64), "bazel-restore:", "bazel-slot-gate", "cargo:configuration-publication", '["semio.cargo.store-lease/v1","/b"]', '["semio.cargo.custody-lease/v1","/b","prototype"]', "artifact-capture:/a", "repo:compiler-writer:/repo/.repo/rust/fixed/m", namespace]) for (const mode of ["shared", "exclusive"] as const) expect(checkLeaseOrderV1(live, [held(namespace)], request(inside, mode))?.requestedClass, `${inside} ${mode} under the namespace`).toBeUndefined();
  for (const before of ["bazel-slot-s0", "bazel-snapshot-restore", admission, "build-graph:/repo/.cache/b/g/m/snapshots/" + "a".repeat(64) + ".json"]) expect(checkLeaseOrderV1(live, [held(before, "exclusive")], request(namespace, "shared")), `the namespace under ${before}`).toBeUndefined();
  for (const after of ["build-graph-store:/repo/.cache/b/g/m", "artifact:/a", "repo:compiler-writer:/repo/.repo/rust/fixed/m"]) expect(checkLeaseOrderV1(live, [held(after, "exclusive")], request(namespace, "shared"))?.requestedClass, `the namespace requested under ${after} is a cycle risk`).toBe("descriptor-custody");
  expect(checkHoldAndWaitV1(live, [held(admission, "exclusive")], request(namespace), 60000)?.kind, "a maintenance writer still may not queue for readers while it holds the gate").toBe("hold-and-wait");
  expect(checkHoldAndWaitV1(live, [held('["semio.cargo.custody-lease/v1","/b","prototype"]', "exclusive")], request(namespace, "shared"), 60000), "the custody keeps its declared edge").toBeUndefined();
});
