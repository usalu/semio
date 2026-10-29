import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import Ajv from "ajv";
import patch from "fast-json-patch";
const base=resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/⌨️typing/✍️edit");
const fixture=JSON.parse(await readFile(resolve(base,"🧪️fixtures/🔣️.json"),"utf8"));
const schema=JSON.parse(await readFile(resolve(base,"🧬️schema/🔣️.json"),"utf8"));delete schema.$schema;
if(!new Ajv({strict:false}).compile(schema)(fixture))throw Error("neutral typing inlet fixture violates its defining contract");
const source={operation:"original",decoder:"same"};const model={source,committed:null,open:null,events:0,allowed:true,closed:false};
const [first,second]=fixture.deliveries.map(row=>row.name);
patch.applyPatch(model,[{op:"test",path:"/allowed",value:true},{op:"replace",path:"/open",value:first}],true,true);
const firstPreview=model.open,firstRows=model.events;
patch.applyPatch(model,[{op:"move",from:"/open",path:"/committed"},{op:"add",path:"/open",value:second},{op:"replace",path:"/events",value:1}],true,true);
const priorCommitted=model.committed,lastPreview=model.open,priorRows=model.events;
patch.applyPatch(model,[{op:"move",from:"/open",path:"/committed"},{op:"replace",path:"/closed",value:true}],true,true);
const expected={firstPreview,priorCommitted,lastPreview,finalCommitted:model.committed,firstRows,priorRows,finalRows:model.events,sameSource:model.source===source,closed:model.closed};
const pending={resident:{name:first},cursor:null,allowed:false};const saved=pending.resident;let denied=false;
try{patch.applyPatch(pending,[{op:"test",path:"/allowed",value:true},{op:"move",from:"/resident",path:"/cursor"}],true,true);}catch(error){if(!(error instanceof Error)||error.name!=="TEST_OPERATION_FAILED")throw error;denied=true;}
if(!denied||pending.resident!==saved||pending.cursor!==null)throw Error("underfunded edit moved original resident");
patch.applyPatch(pending,[{op:"replace",path:"/cursor",value:{incoming:second}},{op:"remove",path:"/cursor"}],true,true);
if(pending.resident!==saved)throw Error("cancelled staged edit changed the actual retained resident");
if(process.argv[2]){const log=await readFile(process.argv[2],"utf8");const line=log.split("\n").find(line=>line.startsWith("mounted-typing-edit="));if(!line)throw Error("actual native mounted typing edit record missing");const native=JSON.parse(line.slice("mounted-typing-edit=".length));for(const key of Object.keys(expected))if(native[key]!==expected[key])throw Error("actual native typing edit differs: "+key);}
console.log(JSON.stringify({neutralSchema:true,mountedTypingEdit:expected,underfundedUnchanged:denied,cancelKeepsPrevious:pending.resident===saved,nativeExecuted:false}));
