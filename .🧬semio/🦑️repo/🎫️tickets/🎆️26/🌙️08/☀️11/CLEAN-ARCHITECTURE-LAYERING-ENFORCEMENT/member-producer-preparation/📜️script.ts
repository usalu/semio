import {createRequire} from "node:module";
import {readFileSync,mkdirSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";
const require=createRequire(import.meta.url),Ajv=require(resolve("node_modules/ajv/dist/2020.js")).default;
const ticket=resolve(process.argv[2]),schema=JSON.parse(readFileSync(resolve(ticket,"member-producer-guard-descriptor.schema.json"),"utf8")),input=JSON.parse(readFileSync(resolve(ticket,"member-producer-guard-preparation.json"),"utf8"));
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
const results=[];
for(const row of input.cases.filter(row=>row.change===null||row.change.removeRole)){
 const candidate=structuredClone(input.descriptor);
 if(row.change)delete candidate.roles[row.change.removeRole];
 const accepted=validate(candidate);
 results.push({id:row.id,accepted,expected:row.accepted});
 if(accepted!==row.accepted)throw Error("descriptor parity: "+row.id);
}
for(const row of input.descriptorCases){
 const candidate=structuredClone(input.descriptor),parent=row.path.slice(0,-1).reduce((owner,key)=>owner[key],candidate),key=row.path.at(-1);
 if(row.remove)delete parent[key];else parent[key]=row.value;
 const accepted=validate(candidate);results.push({id:row.id,accepted,expected:row.accepted});
 if(accepted!==row.accepted)throw Error("descriptor parity: "+row.id);
}
const tokens=[];
for(const row of input.cases.filter(row=>row.change?.from)){
 const source=readFileSync(resolve(input.descriptor.roles[row.change.role].path),"utf8"),count=source.split(row.change.from).length-1;
 tokens.push({id:row.id,role:row.change.role,matches:count});
 if(count===0)throw Error("current defining mutation absent: "+row.id);
}
const sourceSymbols=[];
for(const [role,ref] of Object.entries({...input.descriptor.roles,...Object.fromEntries(Object.entries(input.descriptor.extension.roles).map(([key,value])=>["extension."+key,value]))})){
 const source=readFileSync(resolve(ref.path),"utf8");
 for(const symbol of ref.symbols){const present=new RegExp("\\b"+symbol+"\\b","u").test(source);sourceSymbols.push({role,symbol,present});if(!present)throw Error("source symbol absent: "+role+":"+symbol);}
}
const out=resolve(ticket,"🗑️generated/member-producer-preparation");mkdirSync(out,{recursive:true});
writeFileSync(resolve(out,"probe.json"),JSON.stringify({kind:"pure-schema-and-current-byte-preparation-probe-not-registered-validation",node:process.version,oracle:require.resolve(resolve("node_modules/ajv/dist/2020.js")),schemaCases:results,mutationTargets:tokens,sourceSymbols,productionMutations:0},null,2)+"\n");
process.stdout.write("[DEBUG] preparation: "+results.length+" strict Ajv descriptor cases; "+tokens.length+" exact current mutation targets; "+sourceSymbols.length+" declared symbol occurrences; probe source writes=0\n");

const ts=require("typescript"),domain="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/",entries=process.argv[3]?.startsWith("root")?[resolve("📜️script.ts"),resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts")]:[resolve(domain+"🟦️.ts"),resolve(domain+"🧪️tests/🟦️.ts"),resolve("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📥️source/🧪️tests/🟦️.ts")],options={strict:true,noUncheckedIndexedAccess:process.argv[3]!=="root-strict",noEmit:true,skipLibCheck:true,allowImportingTsExtensions:true,esModuleInterop:true,resolveJsonModule:true,target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,types:["bun","node"]};
const program=ts.createProgram(entries,options),diagnostics=ts.getPreEmitDiagnostics(program).map(row=>({code:row.code,path:row.file?.fileName,line:row.file&&row.start!==undefined?row.file.getLineAndCharacterOfPosition(row.start).line+1:null,message:ts.flattenDiagnosticMessageText(row.messageText,"\n")}));
writeFileSync(resolve(out,process.argv[3]?.startsWith("root")?process.argv[3]+"-semantic.json":"defining-semantic.json"),JSON.stringify({kind:"pure-typescript-semantic-probe-not-registered-test",entries,sourceFiles:program.getSourceFiles().length,diagnostics},null,2)+"\n");
process.stdout.write("[DEBUG] defining semantic diagnostics "+diagnostics.length+"\n");
if(diagnostics.length)throw Error("Semantic diagnostics retained: "+diagnostics.length);
