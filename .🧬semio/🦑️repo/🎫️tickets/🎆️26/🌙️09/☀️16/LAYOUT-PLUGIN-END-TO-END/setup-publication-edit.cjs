const fs=require("fs"),path=require("path");
const root=process.cwd(),native="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/",library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const put=(p,s)=>{fs.mkdirSync(path.dirname(p),{recursive:true});fs.writeFileSync(p,s)},edit=(p,a,b)=>{const s=fs.readFileSync(p,"utf8");if(s.split(a).length!==2)throw Error("Exact unique edit required: "+p+" "+a.slice(0,80));fs.writeFileSync(p,s.replace(a,b))};

const fixtureBase=native+"📤️publication/📸️observation/🧪️tests/";
put(fixtureBase+"🧬️schema.json",JSON.stringify({$schema:"http://json-schema.org/draft-07/schema#",type:"array",minItems:1,items:{type:"object",additionalProperties:false,required:["id","authored","requested","backing","allocated","physical","identity","custody","accept"],properties:{id:{type:"string"},authored:{type:"integer",minimum:1},requested:{type:"integer",minimum:1},backing:{type:"integer",minimum:1},allocated:{type:"integer",minimum:1},physical:{type:"integer",minimum:1},identity:{type:"boolean"},custody:{type:"boolean"},accept:{type:"boolean"}}}},null,2)+"\n");
put(fixtureBase+"🔣️.json",JSON.stringify([
{id:"unchanged-eight-gib-backing-new-envelope",authored:8588886016,requested:38653657088,backing:8588886016,allocated:8588886016,physical:38654705664,identity:true,custody:true,accept:true},
{id:"current-envelope",authored:38653657088,requested:38653657088,backing:8588886016,allocated:8588886016,physical:38654705664,identity:true,custody:true,accept:true},
{id:"backing-over-quota",authored:8,requested:36,backing:40,allocated:40,physical:36,identity:true,custody:true,accept:false},
{id:"allocated-over-quota",authored:8,requested:36,backing:8,allocated:40,physical:36,identity:true,custody:true,accept:false},
{id:"foreign-receipt",authored:8,requested:36,backing:8,allocated:8,physical:36,identity:false,custody:true,accept:false},
{id:"lost-custody",authored:8,requested:36,backing:8,allocated:8,physical:36,identity:true,custody:false,accept:false}
],null,2)+"\n");
put(fixtureBase+"🟦️.ts",`import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema.json";
import corpus from"./🔣️.json";
const source=()=>readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8");
function capture(ports:Record<string,unknown>){return new Function(...Object.keys(ports),ts.transpileModule(source(),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS}}).outputText.replace(/^.*require\\(.*$/gmu,"")+";return captureObservedNativeImagePublicationV1;")(...Object.values(ports));}
test("observed publication neutral unchanged backing and quota laws match Ajv and SQLite",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");try{for(const row of corpus){let checks=0;const directory={isDirectory:()=>true,isSymbolicLink:()=>false,dev:1n,ino:2n},backing={isFile:()=>true,isSymbolicLink:()=>false,nlink:1n,dev:1n,ino:3n,size:BigInt(row.backing)},root={...directory,dev:4n,ino:5n},record={bytes:Buffer.from(JSON.stringify({version:1,dev:1,ino:row.identity?3:30,size:row.backing,filesystemBytes:row.authored,maximumPhysicalBytes:row.physical}))},ports={exports:{},join,process:{platform:"darwin"},lstatSync:(p:string)=>p.endsWith("fixed.dmg")?backing:p.endsWith("/m")?root:directory,nativeStorageBackendOfDirectoryV1:()=>({backingFilename:"fixed.dmg"}),nativeStorageMountedAtV1:()=>true,nativeImageRootV1:()=>"/image/m",nativeImageAttachedV1:()=>true,nativeImageBackingAllocatedBytesV1:()=>BigInt(row.allocated),readNativeImageRetirementFileV1:()=>record,validateJsonSchemaSubset:()=>[],imageSchema:{definitions:{ImageReceipt:{}}},metadataSchema:{definitions:{ImageReceipt:{}}},mountSchema:{},createHash:()=>({update:()=>({digest:()=>"a".repeat(64)})}),readNativeImagePublicationRequestV1:(x:unknown)=>x};let accepted=false;try{const result=capture(ports)({directory:"/image",filesystemBytes:row.requested,maximumPhysicalBytes:row.physical},()=>{checks++;if(!row.custody)throw Error("Custody lost");});accepted=true;expect(result.backing.size).toBe(row.backing);expect(result.purpose).toBe("current-exact-publication");expect(result.initialState).toBe("mounted");}catch{}const oracle=db.query("select ? and ? and ?<=? and ?<=? accepted").get(row.identity,row.custody,row.backing,row.physical,row.allocated,row.physical)as{accepted:number};expect(accepted,row.id).toBe(row.accept);expect(accepted,row.id).toBe(Boolean(oracle.accepted));expect(checks).toBeGreaterThan(0);}}finally{db.close();}});
`);
