import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020.js";
import {readFileSync,createReadStream} from "node:fs";
import {resolve} from "node:path";
import {createInterface} from "node:readline";
import {createHash} from "node:crypto";
import wireSchema from "./owned-validation-oracle.schema.json";

type Demand=[number,number,number,number];
type Grant=[number,number,number,number,number];
type Progress=[number,number,number,number];
export type OwnedTurnV1={name:string;ordinal:number;op:"birth"|"occupied"|"advance"|"close";demand:Demand;grant:Grant;progress:Progress;heap:[number,number];denied:boolean;done:boolean};
export type OwnedSummaryV1={name:string;completed:boolean;valid:boolean|null;closed:boolean;order:number[];relocated:boolean;birthDenied:number;closeDenied:number;identity:boolean;capacity:number;released:number;sourceReleased:number;turnCount:number;programBirths:number;relocations:number;identityChecks:number;readsAfterClose:number};
export type OwnedRecordsV1={turns:OwnedTurnV1[];summaries:OwnedSummaryV1[]};
type CaseV1={name:string;stop:"done"|"birth"|"index"|"program-birth"|"program-pending";duplicateStory:boolean;completed:boolean;valid:boolean|null};
export type OwnedInputsV1={cases:{version:1;maxObservedTurns:number;birthDenials:string[];closeOrder:string[];cases:CaseV1[]};base:{stories:{id:string}[]}};
type Atom=string|number|null;
interface SqlStateV1{exec(sql:string):void;put(sql:string,...values:Atom[]):void;rows(sql:string):Record<string,unknown>[]}
interface SchemaToolV1{require(schema:unknown,value:unknown):void}
const schemaTool:SchemaToolV1={require(schema,value){const tool=new Ajv2020({strict:true,allErrors:true});if(!tool.validate(schema as object,value))throw Error("Owned lifecycle schema refused: "+tool.errorsText());}};
const withSqlState=<T>(body:(sql:SqlStateV1)=>T):T=>{const db=new Database(":memory:");try{return body({exec:sql=>{db.exec(sql);},put:(sql,...values)=>{db.query(sql).run(...values);},rows:sql=>db.query(sql).all()as Record<string,unknown>[]});}finally{db.close();}};
const basePath="✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any";
export function loadOwnedValidationInputsV1():OwnedInputsV1{
 const root=process.cwd(),directory=resolve(root,basePath,"🧬️schema/✅️validation/🧳️retained/📦️owned/🧪️tests"),cases=JSON.parse(readFileSync(resolve(directory,"🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(resolve(directory,"📐️.json"),"utf8")),corpus=JSON.parse(readFileSync(resolve(root,basePath,"🧪️tests/🧫️fixtures/✅️validation/🔣️.json"),"utf8"));
 schemaTool.require(schema,cases);
 if(cases.maxObservedTurns!==4096||cases.cases.length!==6||!Array.isArray(corpus.base?.stories)||corpus.base.stories.length<2||corpus.base.stories.some((story:unknown)=>!story||typeof(story as {id?:unknown}).id!=="string"))throw Error("Exact owned lifecycle matrix and actual base stories required");
 return{cases,base:corpus.base};
}
function installExpected(sql:SqlStateV1,inputs:OwnedInputsV1):void{
 sql.exec("pragma foreign_keys=on;create table cases(name text primary key, stop text not null, declared_completed integer not null, declared_valid integer);create table stories(name text references cases(name),ordinal integer,id text not null);");
 for(const row of inputs.cases.cases){sql.put("insert into cases values(?,?,?,?)",row.name,row.stop,Number(row.completed),row.valid===null?null:Number(row.valid));for(let index=0;index<inputs.base.stories.length;index++){const id=row.duplicateStory&&index===1?inputs.base.stories[0]!.id:inputs.base.stories[index]!.id;sql.put("insert into stories values(?,?,?)",row.name,index,id);}}
 sql.exec("create view expected as select c.name,c.stop,c.stop='done' completed,case when c.stop='done' then (select count(*)=count(distinct id) and min(length(trim(id)))>0 from stories where name=c.name) else null end valid from cases c;");
 if(sql.rows("select c.name from cases c join expected e using(name) where c.declared_completed<>e.completed or c.declared_valid is not e.valid").length)throw Error("Canonical outcome differs from independent story-identity model");
}
export function compareOwnedValidationRecordsV1(inputs:OwnedInputsV1,records:OwnedRecordsV1):{cases:number;turns:number;nativeCompared:false}{
 schemaTool.require(wireSchema,records);
 return withSqlState(sql=>{
  installExpected(sql,inputs);
  sql.exec("create table turns(name text references cases(name),ordinal integer,op text,dc integer,da integer,dr integer,dd integer,gi integer,gc integer,ga integer,gr integer,gd integer,pi integer,pc integer,pa integer,pr integer,ha integer,hr integer,denied integer,done integer,primary key(name,ordinal));create table summaries(name text primary key references cases(name),completed integer,valid integer,closed integer,o0 integer,o1 integer,o2 integer,relocated integer,birth_denied integer,close_denied integer,identity integer,capacity integer,released integer,source_released integer,turn_count integer,program_births integer,relocations integer,identity_checks integer,reads_after_close integer);");
  for(const row of records.turns)sql.put("insert into turns values("+Array(20).fill("?").join(",")+")",row.name,row.ordinal,row.op,...row.demand,...row.grant,...row.progress,...row.heap,Number(row.denied),Number(row.done));
  for(const row of records.summaries)sql.put("insert into summaries values("+Array(19).fill("?").join(",")+")",row.name,Number(row.completed),row.valid===null?null:Number(row.valid),Number(row.closed),row.order[0]!,row.order[1]!,row.order[2]!,Number(row.relocated),row.birthDenied,row.closeDenied,Number(row.identity),row.capacity,row.released,row.sourceReleased,row.turnCount,row.programBirths,row.relocations,row.identityChecks,row.readsAfterClose);
  const refuse=(law:string,query:string)=>{const bad=sql.rows(query);if(bad.length)throw Error("Owned lifecycle "+law+" refused: "+JSON.stringify(bad.slice(0,2)));};
  refuse("complete identities","select name from cases where name not in(select name from summaries) or name not in(select name from turns)");
  refuse("bounded contiguous observations","select name from turns group by name having min(ordinal)<>1 or max(ordinal)<>count(*) or count(*)>"+inputs.cases.maxObservedTurns);
  sql.exec("create view receipts as select *,gi>=1 and gc>=dc and ga>=da and gr>=dr and gd>=dd admitted from turns;");
  refuse("original startup order","select name,ordinal from turns where (ordinal<=4 and(op<>'birth' or denied<>1)) or(ordinal=5 and(op<>'birth' or denied<>0)) or(ordinal=6 and(op<>'occupied' or denied<>1)) or(ordinal>6 and op in('birth','occupied'))");
  refuse("four original denial axes","select name,ordinal from turns where ordinal<=4 and not(gi=case when ordinal=1 then 0 else 1 end and gc=dc-case when ordinal=2 then 1 else 0 end and ga=da-case when ordinal=3 then 1 else 0 end and gr=dr and gd=dd-case when ordinal=4 then 1 else 0 end)");
  refuse("paired zero-credit turns","select t.name,t.ordinal from turns t left join turns next on next.name=t.name and next.ordinal=t.ordinal+1 where t.ordinal>6 and((t.ordinal%2=1 and(t.denied<>1 or t.gi<>0 or next.denied<>0 or next.op<>t.op or next.dc<>t.dc or next.da<>t.da or next.dr<>t.dr or next.dd<>t.dd or next.gc<>t.gc or next.ga<>t.ga or next.gr<>t.gr or next.gd<>t.gd))or(t.ordinal%2=0 and t.denied<>0))");
  refuse("actual grant admission","select name,ordinal from receipts where(op='occupied' and(denied<>1 or admitted<>1)) or(op<>'occupied' and((denied=1 and admitted=1)or(denied=0 and admitted=0)))");
  refuse("denied zero effects","select name,ordinal from turns where denied=1 and(pi<>0 or pc<>0 or pa<>0 or pr<>0 or ha<>0 or hr<>0 or done<>0)");
  refuse("grant progress and actual heap","select name,ordinal from turns where pi>gi or pc>gc or pa>ga or pr>gr or pa<>ha or pr<>hr");
  sql.exec("create view lifecycle as with recursive states(name,n,state) as(select name,0,'new' from cases union all select t.name,t.ordinal,case when t.op='birth' and s.state='new' then case when t.denied=1 then 'new' else 'active' end when t.op='occupied' and s.state='active' and t.denied=1 then 'active' when t.op='advance' and s.state='active' then case when t.done=1 and t.denied=0 then 'validated' else 'active' end when t.op='close' and s.state in('active','validated','closing') then case when t.done=1 and t.denied=0 then 'closed' else 'closing' end else 'invalid' end from states s join turns t on t.name=s.name and t.ordinal=s.n+1)select * from states;");
  refuse("state transition","select name,n from lifecycle where state='invalid'");
  refuse("closed final state","select c.name from cases c where not exists(select 1 from lifecycle l where l.name=c.name and l.n=(select max(ordinal)from turns where name=c.name)and state='closed')");
  refuse("completion cannot precede close","select name,ordinal from turns where done=1 and(denied=1 or op not in('advance','close'))");
  sql.exec("create view totals as select name,count(*) turns,sum(pa) capacity,sum(pr) released,sum(op='birth' and denied=1) birth_denied,sum(op='close' and denied=1) close_denied,sum(op='advance' and denied=0) relocations,sum(op='advance' and denied=0 and dd=1 and da>0) program_births,sum(op='advance' and denied=0 and done=1) completions from turns group by name;");
  refuse("semantic outcome","select s.name from summaries s join expected e using(name) join totals t using(name) where s.completed<>e.completed or s.valid is not e.valid or t.completions<>e.completed");
  refuse("canonical cancellation frontier","select c.name from cases c join totals t using(name) where(c.stop='birth' and(t.relocations<>0 or t.program_births<>0))or(c.stop='index' and(t.relocations<>1 or t.program_births<>0))or(c.stop='program-birth' and(t.program_births<>1 or(select max(ordinal)from turns where name=c.name and op='advance' and denied=0)<>(select min(ordinal)from turns where name=c.name and op='advance' and denied=0 and dd=1 and da>0)))or(c.stop='program-pending' and(t.program_births<>1 or(select count(*)from turns a where a.name=c.name and a.op='advance' and a.denied=0 and a.ordinal>(select min(ordinal)from turns b where b.name=c.name and b.op='advance' and b.denied=0 and b.dd=1 and b.da>0))<>1))");
  refuse("lifecycle observations","select s.name from summaries s join totals t using(name) join cases c using(name) where s.closed<>1 or s.o0<>1 or s.o1<>2 or s.o2<>3 or s.relocated<>1 or s.identity<>1 or s.identity_checks<1 or s.reads_after_close<>0 or s.turn_count<>t.turns or s.birth_denied<>4 or t.birth_denied<>4 or s.close_denied<>t.close_denied or s.close_denied<1 or s.relocations<>t.relocations+1 or s.program_births<>t.program_births or s.program_births>1 or((c.stop in('program-birth','program-pending')or(c.stop='done' and s.valid=1))and s.program_births<>1)");
  refuse("allocation conservation","select s.name from summaries s join totals t using(name) where s.capacity<>t.capacity or s.released<>t.released or s.released-s.source_released<>s.capacity or s.source_released<0");
  for(const row of sql.rows("select capacity,released from totals"))if(!Number.isSafeInteger(row.capacity)||!Number.isSafeInteger(row.released))throw Error("Exact safe lifecycle accounting required");
  return{cases:records.summaries.length,turns:records.turns.length,nativeCompared:false};
 });
}
export function referenceOwnedValidationRecordsV1(inputs:OwnedInputsV1):OwnedRecordsV1{
 const truth=withSqlState(sql=>{installExpected(sql,inputs);return sql.rows("select name,completed,valid from expected order by name");}),records:OwnedRecordsV1={turns:[],summaries:[]};
 for(const row of inputs.cases.cases){
  const turns:OwnedTurnV1[]=[],add=(op:OwnedTurnV1["op"],demand:Demand,progress:Progress,denied=false,done=false,axis?:number)=>{
   const grant:Grant=[1,demand[0],demand[1],demand[2],demand[3]];if(axis!==undefined)grant[axis]=grant[axis]!-1;
   turns.push({name:row.name,ordinal:turns.length+1,op,demand,grant,progress,heap:[progress[2],progress[3]],denied,done});
  },birth:Demand=[20,10,0,1],zero:Progress=[0,0,0,0],work=(d:Demand,p:Progress,done=false)=>{add("advance",d,zero,true,false,0);add("advance",d,p,false,done);},close=(d:Demand,done=false)=>{add("close",d,zero,true,false,0);add("close",d,[1,d[0],0,d[2]],false,done);};
  for(const axis of[0,1,2,4])add("birth",birth,zero,true,false,axis);
  add("birth",birth,[1,20,10,0]);add("occupied",birth,zero,true);
  let capacity=10,programBirths=0,relocations=1;
  if(row.stop!=="birth"){const early=row.stop==="done"&&row.duplicateStory;work([12,4,0,2],[1,12,4,0],early);capacity+=4;relocations++;if(!early&&row.stop!=="index"){work([14,6,0,1],[1,14,6,0]);capacity+=6;programBirths++;relocations++;if(row.stop!=="program-birth"){work([16,0,0,1],[1,16,0,0],row.stop==="done");relocations++;}}}
  if(programBirths)close([14,0,6,1]);if(row.stop!=="birth")close([12,0,4,2]);close([2,0,0,1]);close([9,0,7,2]);close([20,0,10,1],true);
  const expected=truth.find(item=>item.name===row.name)!;
  records.turns.push(...turns);records.summaries.push({name:row.name,completed:Boolean(expected.completed),valid:expected.valid===null?null:Boolean(expected.valid),closed:true,order:[1,2,3],relocated:true,birthDenied:4,closeDenied:turns.filter(t=>t.op==="close"&&t.denied).length,identity:true,capacity,released:capacity+7,sourceReleased:7,turnCount:turns.length,programBirths,relocations,identityChecks:relocations+1,readsAfterClose:0});
 }
 return records;
}
export function mutateOwnedValidationRecordsV1(records:OwnedRecordsV1,mutation:string):OwnedRecordsV1{
 const out=structuredClone(records),first=out.summaries[0]!,start=out.turns.find(row=>row.op==="birth"&&!row.denied)!,work=out.turns.find(row=>row.op==="advance"&&!row.denied)!,closed=out.turns.findLast(row=>row.op==="close"&&!row.denied)!;
 switch(mutation){
  case"missing-summary":out.summaries.pop();break;
  case"duplicate-summary":out.summaries.push(structuredClone(first));break;
  case"foreign-summary":first.name="foreign";break;
  case"missing-turn":{const index=out.turns.findIndex(row=>row.op==="advance"&&row.denied),name=out.turns[index]!.name;out.turns.splice(index,1);let ordinal=0;for(const row of out.turns)if(row.name===name)row.ordinal=++ordinal;out.summaries.find(row=>row.name===name)!.turnCount=ordinal;break;}
  case"duplicate-turn":out.turns.push(structuredClone(out.turns[0]!));break;
  case"foreign-turn":out.turns[0]!.name="foreign";break;
  case"ordinal-gap":out.turns[0]!.ordinal=99;break;
  case"close-before-work":work.op="close";break;
  case"undergrant-admitted":start.grant[4]=0;break;
  case"dirty-denial":out.turns[0]!.progress[2]=1;out.turns[0]!.heap[0]=1;break;
  case"heap-mismatch":start.heap[0]++;break;
  case"capacity-summary":first.capacity++;break;
  case"release-conservation":first.sourceReleased++;break;
  case"close-order":first.order=[2,1,3];break;
  case"cancel-completed":out.summaries.find(row=>!row.completed)!.completed=true;break;
  case"semantic-validity":first.valid=false;break;
  case"identity-false":first.identity=false;break;
  case"reads-after-close":first.readsAfterClose=1;break;
  case"turn-count":first.turnCount++;break;
  case"relocation-count":first.relocations++;break;
  case"unfinished-close":closed.done=false;break;
  case"program-birth":first.programBirths=0;break;
  case"unknown-field":Object.assign(first,{unrecognized:true});break;
  default:throw Error("Unknown authored hostile lifecycle mutation");
 }
 return out;
}
const turnPrefix="[DEBUG] [owned-layout-turn] ",summaryPrefix="[DEBUG] [owned-layout-validation] ";
function recordLine(records:OwnedRecordsV1,line:string):void{
 const markers=[turnPrefix,summaryPrefix].filter(prefix=>line.includes(prefix));if(!markers.length)return;if(markers.length!==1)throw Error("Ambiguous actual lifecycle record");
 const prefix=markers[0]!,index=line.indexOf(prefix),head=line.slice(0,index);
 if(head!==""&&!/^test [A-Za-z0-9_:]+ \.\.\. $/.test(head))throw Error("Actual lifecycle prefix requires atomic stdout context");
 const value=JSON.parse(line.slice(index+prefix.length));if(prefix===turnPrefix){if(records.turns.length>=6*4096)throw Error("Bounded actual lifecycle observations required");records.turns.push(value);}else{if(records.summaries.length>=6)throw Error("Exact six actual lifecycle summaries required");records.summaries.push(value);}
}
export function parseOwnedValidationLinesV1(lines:Iterable<string>):OwnedRecordsV1{const records:OwnedRecordsV1={turns:[],summaries:[]};for(const line of lines)recordLine(records,line);return records;}
async function main():Promise<void>{
 const args=process.argv.slice(2),inputs=loadOwnedValidationInputsV1();
 if(args.length===1&&args[0]==="--reference"){const result=compareOwnedValidationRecordsV1(inputs,referenceOwnedValidationRecordsV1(inputs));console.log(JSON.stringify({mode:"reference-harness-only",...result}));return;}
 if(args.length!==2||args[0]!=="--native"||!args[1])throw Error("Use --reference or --native ACTUAL_FINAL_NATIVE_LOG");
 const path=resolve(args[1]),stream=createReadStream(path),hash=createHash("sha256"),records:OwnedRecordsV1={turns:[],summaries:[]};let bytes=0;
 stream.on("data",chunk=>{bytes+=chunk.length;hash.update(chunk);});
 for await(const line of createInterface({input:stream,crlfDelay:Infinity}))recordLine(records,line);
 const result=compareOwnedValidationRecordsV1(inputs,records);console.log(JSON.stringify({mode:"actual-log",...result,nativeCompared:true,input:path,inputBytes:bytes,inputSha256:hash.digest("hex")}));
}
if(import.meta.main)await main();
