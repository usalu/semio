import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import authored from "../🔣️policy.json";
import {checkHoldAndWaitV1,checkLeaseOrderV1,classifyLeaseResourceV1,findLeaseCycleV1,formatLeaseViolationV1,leaseBlockersV1,leaseOrderPolicyV1,readLeaseOrderPolicyV1,type LeaseNodeV1,type LeaseViolationV1} from "../🟦️.ts";

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
 const live=readLeaseOrderPolicyV1(authored),expected:Record<string,string>={"native-fixed-image:/w/.cache/b/d":"image","native-storage:abc":"image","repository-graph-facts:/w":"graph","nx:graph-pool:abc":"graph","build-graph-store:/r":"graph","bazel-workspace-x":"tool-singleton","bazel-snapshot-restore":"tool-singleton","cargo:registry:/h":"cargo-coordination","cargo-preparation:/w":"cargo-coordination",'["semio.cargo.build-lease/v1","/b"]':"cargo-coordination","rust-code:/c":"namespace","javascript-dependency-bindings:/w":"namespace","artifact:/a/b":"artifact","semio.compiler.cpu-picker/v1":"compiler-capacity","semio.compiler.cpu/v1/3":"compiler-capacity","nx-tooling-activation":"tool-singleton","some-future-resource":"unclassified"};
 for(const [resource,id] of Object.entries(expected))expect(`${resource} → ${classifyLeaseResourceV1(live,resource).id}`).toBe(`${resource} → ${id}`);
});

for(const row of fixture.classification)test(`classification ${row.resource}`,()=>expect(classifyLeaseResourceV1(policy,row.resource).id).toBe(row.class));

for(const row of fixture.order)test(`lock order ${row.id}`,()=>{
 const found=checkLeaseOrderV1(policy,row.held as never,row.request as never);
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
 for(const node of nodes)for(const to of leaseBlockersV1(nodes,node))insert.run(node.id,to);
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
 expect(leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"off"}).rules).toEqual({outOfOrder:"off",holdAndWait:"off",selfDeadlock:"off",cycle:"off"});
 expect(leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"enforce"}).rules.outOfOrder).toBe("enforce");
 expect(String(leaseOrderPolicyV1({}).rules.outOfOrder)).toBe(String(authored.rules.outOfOrder));
 expect(()=>leaseOrderPolicyV1({SEMIO_LEASE_ORDER:"sometimes"})).toThrow("must be off, observe or enforce");
});
