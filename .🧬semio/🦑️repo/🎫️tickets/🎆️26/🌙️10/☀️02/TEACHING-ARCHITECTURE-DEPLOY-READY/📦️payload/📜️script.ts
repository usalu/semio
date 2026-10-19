import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {execFileSync} from "node:child_process";
import {resolve,relative} from "node:path";
import * as path from "node:path";
import {Script,createContext} from "node:vm";
import assert from "node:assert/strict";

const root=resolve(process.argv[2]??process.cwd()),helper="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📦️payload/🟨️.cjs",expected="548974b74047bc609b3432b70360210e655c0fb3cd3c58f4f55f83755b6067e2";
const source=readFileSync(resolve(root,helper)),hash=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
assert.equal(hash(source),expected);
const module={exports:{} as {ownsPayload:(owner:unknown,payload:unknown)=>boolean}},context=createContext({module,require:(name:string)=>{assert.equal(name,"node:path");return path;}},{codeGeneration:{strings:false,wasm:false}});
new Script(source.toString("utf8"),{filename:helper}).runInContext(context,{timeout:1000});
const read=String.raw`import {readFileSync,lstatSync,existsSync} from 'node:fs';import {createHash} from 'node:crypto';import {resolve,join} from 'node:path';
const roots=['🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust','🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust','🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust','🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust'];
const read=(directory)=>{const file=join('/repo',directory,'package.json'),bytes=readFileSync(file),facts=lstatSync(file);if(!facts.isFile()||facts.isSymbolicLink())throw Error('Ordinary manifest required');return{directory,document:JSON.parse(bytes),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};};
const targets=value=>typeof value==='string'?[value]:value&&typeof value==='object'?Object.values(value).flatMap(targets):[];
const pairs=roots.map((parent,i)=>{const child=parent+'/'+(i===2?'🕸️bindings':'pkg'),owner=read(parent),payload=read(child),entries=[];for(const target of targets(owner.document.exports)){const file=resolve('/repo',parent,target);if(file.startsWith(resolve('/repo',child)+'/'))entries.push({target,exists:existsSync(file)});}return{owner,payload,entries};});
const os='🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust',osPayload=read(os+'/pkg'),ancestors=[];for(let parts=os.split('/');parts.length;parts.pop()){const directory=parts.join('/');if(existsSync(join('/repo',directory,'package.json')))ancestors.push(read(directory));}const published=JSON.parse(readFileSync('/repo/package.json')).workspaces;console.log(JSON.stringify({pairs,os:{payload:osPayload,parentManifestExists:existsSync(join('/repo',os,'package.json')),ancestors,published:published.includes(os+'/pkg')}}));`;
const frozen=JSON.parse(execFileSync("docker",["exec","-i","semio-course-builder-sol-8579fa82486d","node","--input-type=module"],{input:read,maxBuffer:1048576,timeout:30000}).toString("utf8"));
const cases:Array<{id:string;owner:Record<string,unknown>;payload:Record<string,unknown>;expected:boolean}>=[];
for(const pair of frozen.pairs){assert.equal(pair.owner.document.name,pair.payload.document.name);assert.ok(pair.entries.length>0);assert.ok(pair.entries.every((row:{exists:boolean})=>!row.exists));cases.push({id:"actual-cold:"+pair.owner.document.name,owner:{...pair.owner.document,absDir:"/repo/"+pair.owner.directory},payload:{...pair.payload.document,absDir:"/repo/"+pair.payload.directory},expected:true});console.log("[DEBUG] frozen-cold-pair",JSON.stringify({name:pair.owner.document.name,parent:{path:pair.owner.directory+"/package.json",bytes:pair.owner.bytes,sha256:pair.owner.sha256},child:{path:pair.payload.directory+"/package.json",bytes:pair.payload.bytes,sha256:pair.payload.sha256},exports:pair.entries}));}
const sample=cases[0]!;
cases.push({id:"unrelated-name",owner:sample.owner,payload:{...sample.payload,name:"@control/independent"},expected:false},{id:"true-duplicate-unbound",owner:{...sample.owner,exports:{".":"./source.js"}},payload:sample.payload,expected:false},{id:"prefix-sibling",owner:{...sample.owner,exports:{".":"./pkg-other/output.js"}},payload:sample.payload,expected:false},{id:"malformed-dot-prefix",owner:{...sample.owner,exports:{".":".pkg/output.js"}},payload:sample.payload,expected:false},{id:"traversal",owner:{...sample.owner,exports:{".":"./pkg/../other/output.js"}},payload:sample.payload,expected:false});
const oracle=`import json,sqlite3,sys
rows=json.load(sys.stdin);db=sqlite3.connect(':memory:');db.execute('create table cases(id text,owner text,payload text,prefix text)')
db.executemany('insert into cases values(?,?,?,?)',[(r['id'],json.dumps(r['owner']),json.dumps(r['payload']),r['prefix']) for r in rows])
sql="""select id,json_extract(owner,'$.name')=json_extract(payload,'$.name') and exists(select 1 from json_tree(json_extract(owner,'$.exports')) e where e.type='text' and substr(e.atom,1,2)='./' and substr(e.atom,3,length(c.prefix))=c.prefix and instr(e.atom,'..')=0 and instr(e.atom,'*')=0 and instr(e.atom,'?')=0) from cases c"""
print(json.dumps(dict(db.execute(sql))))`;
const neutral=cases.map(row=>({...row,prefix:relative(row.owner.absDir as string,row.payload.absDir as string).replaceAll("\\","/")+"/"})),external=JSON.parse(execFileSync("python3",["-c",oracle],{input:JSON.stringify(neutral),maxBuffer:1048576,timeout:30000}).toString("utf8"));
for(const row of cases){const actual=module.exports.ownsPayload(row.owner,row.payload);assert.equal(actual,row.expected,row.id);assert.equal(Boolean(external[row.id]),row.expected,row.id+" SQLite oracle");console.log("[DEBUG] candidate-case",JSON.stringify({id:row.id,owned:actual,sourceChildRetained:!actual,sqliteOwned:Boolean(external[row.id])}));}
assert.equal(frozen.os.parentManifestExists,false);assert.ok(frozen.os.payload.document.name);
assert.equal(frozen.os.published,true);for(const ancestor of frozen.os.ancestors)assert.equal(module.exports.ownsPayload({...ancestor.document,absDir:"/repo/"+ancestor.directory},{...frozen.os.payload.document,absDir:"/repo/"+frozen.os.payload.directory}),false);
console.log("[DEBUG] actual-pkg-only-source-retained",JSON.stringify({path:frozen.os.payload.directory+"/package.json",name:frozen.os.payload.document.name,bytes:frozen.os.payload.bytes,sha256:frozen.os.payload.sha256,parentManifestExists:false,published:true,ancestors:frozen.os.ancestors.map((row:{directory:string;sha256:string;document:{name:string}})=>({path:row.directory+"/package.json",name:row.document.name,sha256:row.sha256})),ownerPredicateApplicable:false}));
assert.equal(hash(readFileSync(resolve(root,helper))),expected);
console.log("[DEBUG] ordinary Node pure candidate proof PASS",JSON.stringify({node:process.version,candidate:expected,pairs:4,controls:5,pkgOnlySources:1,sqliteCases:cases.length,privateSourceMutated:false,managedOrInstallerExecuted:false}));
