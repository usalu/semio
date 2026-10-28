import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import * as routing from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withPreparedCargoDependencyPairV1} from "../../🟦️.ts";
import {prepareCargoWorkspaceInvocation} from "../../../🟦️.ts";
import {cargoPreparationRecordPathsV1,cargoPreparationWitnessHoldsV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

const lock='version = 4\n\n[[package]]\nname = "synchronized_kernel"\nversion = "0.1.0"\n';
/** 🧪️ Builds one synchronized fixture workspace whose receipt (and, with tool inputs, witness) the real preparation pair wrote. */
async function synchronized(base:string,id:string,control:ReturnType<typeof routing.createIssuedResourceControlV1>,tools:boolean){
 const root=join(base,id),storage={version:1 as const,directory:join(root,".prep"),leaseDirectory:join(base,"l"),program:join(base,"child","📜️script.ts")},tool=join(base,"tool-"+id);mkdirSync(join(root,"kernel"),{recursive:true});mkdirSync(storage.directory);
 writeFileSync(join(root,"Cargo.toml"),'[workspace]\nresolver="2"\nmembers=["kernel"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=[]\nmember-manifests=["kernel/Cargo.toml"]\nexclude-patterns=[]\n');
 writeFileSync(join(root,"kernel/Cargo.toml"),'[package]\nname="synchronized_kernel"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="lib.rs"\n');writeFileSync(join(root,"kernel/lib.rs"),"pub fn ready() {}\n");writeFileSync(tool,"#!/bin/sh\n");
 const commands:string[][]=[],preparation={leaseControl:control,signal:control.signal,advance:async()=>{await control.yieldContinuation();},childBudget:(requested:number)=>requested,childOptions:(env:Readonly<Record<string,string|undefined>>)=>({signal:control.signal,env,output:{maximumBytes:1048576,maximumLines:65536},onProgress:async()=>{}})};
 await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",control,async argv=>{commands.push(argv);if(argv[0]==="update")writeFileSync(join(root,"Cargo.lock"),lock);},async()=>fixture.tool,preparation as never,[],[fixture.package],tools?[tool]:[]);
 expect(commands.map(row=>row[0])).toEqual(["update","fetch"]);return{root,storage,tool};
}

test("a launcher skips the Cargo preparation child exactly while the witnessed receipt still holds, and the independent oracle agrees on every edit",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"cfw-"))),marker=join(base,"marker"),validate=new Ajv({strict:false}).compile(schema),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url));
 mkdirSync(join(base,"child"));mkdirSync(join(base,"l"));writeFileSync(join(base,"child","📜️script.ts"),'import {appendFileSync} from "node:fs";appendFileSync(process.env.WITNESS_LAW_MARKER!,process.argv.slice(2).join(" ")+"\\n");\n');
 try{for(const row of fixture.cases)await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"cargo:witness-law",maximumElapsedMilliseconds:60000},{native:{fixture:"cargo-witness"}},Date.now()),async original=>{
  const control=routing.createIssuedResourceControlV1(original,"cargo:witness-law",0),{root,storage,tool}=await synchronized(base,row.id,control,row.edit!=="no-tool-inputs"),paths=cargoPreparationRecordPathsV1(storage,root,"Cargo.toml",[fixture.package]);let environment:Record<string,string|undefined>={...process.env,WITNESS_LAW_MARKER:marker};
  if(existsSync(paths.witness))expect(validate(JSON.parse(readFileSync(paths.witness,"utf8"))),JSON.stringify(validate.errors)).toBe(true);
  if(row.edit==="unrelated-file")writeFileSync(join(root,"notes.txt"),"unrelated\n");
  if(row.edit==="member-manifest")writeFileSync(join(root,"kernel/Cargo.toml"),readFileSync(join(root,"kernel/Cargo.toml"),"utf8").replace('version="0.1.0"','version="0.2.0"'));
  if(row.edit==="owner-manifest")writeFileSync(join(root,"Cargo.toml"),readFileSync(join(root,"Cargo.toml"),"utf8")+"# owner note\n");
  if(row.edit==="lock")writeFileSync(join(root,"Cargo.lock"),lock+"# changed lock\n");
  if(row.edit==="new-directory")mkdirSync(join(root,"docs"));
  if(row.edit==="configuration"){mkdirSync(join(root,".cargo"));writeFileSync(join(root,".cargo/config.toml"),"[build]\n");}
  if(row.edit==="environment")environment={...environment,CARGO_PROFILE_WITNESS:"1"};
  if(row.edit==="tool")writeFileSync(tool,"#!/bin/sh\n# rebuilt\n");
  if(row.edit==="receipt")rmSync(paths.receipt);
  if(row.edit==="witness")rmSync(paths.witness);
  const holds=cargoPreparationWitnessHoldsV1(storage,root,"Cargo.toml",[fixture.package],environment),environmentPath=join(base,row.id+".env.json"),storagePath=join(base,row.id+".storage.json");writeFileSync(environmentPath,JSON.stringify(environment));writeFileSync(storagePath,JSON.stringify(storage));
  const python=spawnSync("python3",["-I",oracle,schemaPath,storagePath,root,"Cargo.toml",environmentPath,fixture.package],{encoding:"utf8"});expect(python.status,python.stderr).toBe(0);
  const before=existsSync(marker)?readFileSync(marker,"utf8").split("\n").filter(Boolean).length:0;
  await prepareCargoWorkspaceInvocation(storage,root,["test","--manifest-path",join(root,"kernel/Cargo.toml"),"-p",fixture.package],root,environment,control);
  const spawned=(existsSync(marker)?readFileSync(marker,"utf8").split("\n").filter(Boolean).length:0)-before;
  expect({id:row.id,child:spawned,holds,oracle:python.stdout.trim()}).toEqual({id:row.id,child:row.child,holds:row.child===0,oracle:String(row.child===0)});
 });}finally{rmSync(base,{recursive:true,force:true});}
},120_000);
