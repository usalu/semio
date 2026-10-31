import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import jsonpatch from "fast-json-patch";
const path="🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🧪️tests/🔣️.json";
const fixture=await Bun.file(path).json(),schema=await Bun.file(path.replace("🔣️.json","📐️.json")).json();
if(!new Ajv({strict:false}).compile(schema)(fixture))throw Error("typing edit schema refusal");
const expected=new Map<string,object>();
for(const row of fixture.cases){
 if(expected.has(row.name))throw Error("duplicate typing edit fixture case "+row.name);
 const db=new Database(":memory:");db.exec("CREATE TABLE typing(tool TEXT,buffer TEXT,leaves TEXT,deadline INTEGER)");
 if(row.before)db.query("INSERT INTO typing VALUES (?,?,?,?)").run(row.before.tool,row.before.buffer,JSON.stringify(row.before.leaves),row.before.deadline);
 const previous=db.query("SELECT * FROM typing").get() as {tool:string,buffer:string,leaves:string,deadline:number}|null;
 const commit=!!previous&&(previous.tool!==row.input.tool||previous.deadline<=row.input.now||previous.buffer!==row.input.buffer||row.fold==="split");
 const reuse=!!previous&&!commit;
 const leaves=row.fold==="net"?row.net:row.input.leaves,open=reuse||leaves.length>0;
 const result={steps:[...(commit?[JSON.parse(previous!.leaves).length?"committed":"empty"]:[]),open?"open":"idle"],committed:commit?JSON.parse(previous!.leaves):[],open:open?{tool:row.input.tool,buffer:row.input.buffer,leaves,deadline:Math.min(Number.MAX_SAFE_INTEGER,row.input.now+750),reuseTransaction:reuse}:null};
 if(jsonpatch.compare(result,row.expected).length)throw Error("independent SQLite edit output differs "+row.name);
 expected.set(row.name,result);db.close();
}
export function compareNativeRecords(log:string):number{
 const seen=new Set<string>();
 for(const line of log.split(/\r?\n/)){
  const prefix="[DEBUG] [original-typing-edit] ",at=line.indexOf(prefix);if(at<0)continue;
  const actual=JSON.parse(line.slice(at+prefix.length));
  if(typeof actual.name!=="string"||!expected.has(actual.name))throw Error("unknown native edit record "+actual.name);
  if(seen.has(actual.name))throw Error("duplicate native edit record "+actual.name);
  if(jsonpatch.compare(actual.result,expected.get(actual.name)!).length)throw Error("native edit differs "+actual.name);
  seen.add(actual.name);
 }
 const missing=[...expected.keys()].filter(name=>!seen.has(name));
 if(missing.length)throw Error("missing native edit records "+missing.join(", "));
 return seen.size;
}
if(import.meta.main){
 const log=process.argv[2],compared=log?compareNativeRecords(await Bun.file(log).text()):0;
 console.log(JSON.stringify({referenceCases:expected.size,nativeCompared:!!log,nativeRecords:compared}));
}
