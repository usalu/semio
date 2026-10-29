import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import jsonpatch from "fast-json-patch";
const path="🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🧪️tests/🔣️.json";
const fixture=await Bun.file(path).json(),schema=await Bun.file(path.replace("🔣️.json","📐️.json")).json();
if(!new Ajv({strict:false}).compile(schema)(fixture))throw Error("typing edit schema refusal");
const expected=new Map<string,object>();
for(const row of fixture.cases){
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
const log=process.argv[2];let compared=0;
if(log)for(const line of(await Bun.file(log).text()).split(/\r?\n/)){const prefix="[DEBUG] [original-typing-edit] ",at=line.indexOf(prefix);if(at<0)continue;const actual=JSON.parse(line.slice(at+prefix.length)),reference=expected.get(actual.name);if(!reference||jsonpatch.compare(actual.result,reference).length)throw Error("native edit differs "+actual.name);compared++;}
if(log&&compared!==expected.size)throw Error("missing native edit records");
console.log(JSON.stringify({referenceCases:expected.size,nativeCompared:!!log,nativeRecords:compared}));
