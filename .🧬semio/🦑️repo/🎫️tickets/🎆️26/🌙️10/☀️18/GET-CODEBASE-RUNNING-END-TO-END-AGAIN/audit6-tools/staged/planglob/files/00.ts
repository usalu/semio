import {createHash,randomBytes} from "node:crypto";
import {lstat,mkdir,readdir,readFile,rename,rm,utimes,writeFile} from "node:fs/promises";
import {dirname,join,parse,sep} from "node:path";
import {gunzipSync,gzipSync} from "node:zlib";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {runtimeParseTomlV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const;
/** 🧾️ One physical question a Cargo planning answer depends on, with the answer it gave. */
export type CargoPlanInputV1=Readonly<{kind:"text"|"presence"|"list";path:string;answer:string}>;
/** 📦️ The two planning answers a memo serves. */
export type CargoPlanAnswersV1=Readonly<{metadata:unknown;units:unknown}>;
/** 🧠️ Recalls and remembers Cargo planning answers by key; a recall answers only while every recorded input still answers the same. */
export interface CargoPlanMemoV1 {recall(key:string,control:ResourceLeaseControl):Promise<CargoPlanAnswersV1|undefined>;remember(key:string,answers:CargoPlanAnswersV1,startedMs:number,control:ResourceLeaseControl):Promise<void>}
const sha256=(value:string|Buffer):string=>createHash("sha256").update(value).digest("hex");

/** 🔑️ Keys one planning request by everything Cargo reads that is not a file below a package: program, arguments, working and workspace directories, the effective configuration digest, compiler and Cargo identities, and the Cargo-read environment. */
export function cargoPlanKeyV1(material:Readonly<{program:string;prefix:readonly string[];args:readonly string[];cwd:string;workspaceRoot:string;configurationDigest:string;compiler:unknown;cargo:string;environment:Readonly<Record<string,string|undefined>>}>):string{
 const rule=protocol.environment,environment=Object.entries(material.environment).filter(([name,value])=>value!==undefined&&(rule.names.includes(name)||rule.prefixes.some(prefix=>name.startsWith(prefix)))&&!rule.excludedNames.includes(name)).sort(([a],[b])=>a<b?-1:a>b?1:0);
 return sha256(JSON.stringify([1,material.program,material.prefix,material.args,material.cwd,material.workspaceRoot,material.configurationDigest,material.compiler,material.cargo,environment]));
}
/** 🪪️ Identifies one tool executable by kind, device, inode, size and change times. */
export async function cargoToolIdentityV1(path:string):Promise<string>{const info=await lstat(path,{bigint:true});return[info.isFile()?"file":"other",info.dev,info.ino,info.size,info.mtimeNs,info.ctimeNs].join(":");}

async function answerV1(kind:CargoPlanInputV1["kind"],path:string):Promise<string>{
 try{
  if(kind==="presence"){await lstat(path);return "true";}
  if(kind==="text")return sha256(await readFile(path));
  return sha256(JSON.stringify((await readdir(path)).sort()));
 }catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||(error as NodeJS.ErrnoException).code==="ENOTDIR")return kind==="presence"?"false":"absent";throw error;}
}
/** 🗺️ Names every physical question Cargo answered the metadata from: the workspace manifest and lock, each local package's manifest, its target auto-discovery paths (files, directories and their nested `main.rs`), and the manifest of every ancestor directory Cargo's workspace search reads (up to the workspace root for its members, up to the filesystem root for a path package outside it). `undefined` when the workspace manifest cannot be read or Cargo expands its members from a glob: a new member then moves no recorded text, and the lock receipt only re-reads the manifests it recorded.
 * @see https://doc.rust-lang.org/cargo/reference/workspaces.html#the-members-and-exclude-fields */
export async function cargoPlanQuestionsV1(metadata:unknown):Promise<readonly Omit<CargoPlanInputV1,"answer">[]|undefined>{
 const facts=metadata as {workspace_root?:unknown;packages?:{manifest_path?:unknown;source?:unknown}[]};
 if(!facts||typeof facts.workspace_root!=="string"||!Array.isArray(facts.packages))throw Error("Resolved Cargo metadata with a workspace root and packages required");
 const rows=new Map<string,Omit<CargoPlanInputV1,"answer">>(),ask=(kind:CargoPlanInputV1["kind"],path:string)=>{rows.set(kind+"\0"+path,{kind,path});};
 ask("text",join(facts.workspace_root,protocol.manifest));ask("text",join(facts.workspace_root,protocol.lock));
 const workspace=facts.workspace_root;
 let declared:{members?:unknown;"default-members"?:unknown}|undefined;try{declared=(runtimeParseTomlV1(await readFile(join(workspace,protocol.manifest),"utf8")) as {workspace?:typeof declared}).workspace;}catch{return undefined;}
 if([declared?.members,declared?.["default-members"]].some(list=>Array.isArray(list)&&list.some(member=>typeof member!=="string"||protocol.memberGlob.some(character=>member.includes(character)))))return undefined;
 for(const row of facts.packages){
  if(row.source!==null||typeof row.manifest_path!=="string")continue;const root=dirname(row.manifest_path),member=root.startsWith(workspace+sep);ask("text",row.manifest_path);
  for(const file of protocol.discovery.files)ask("presence",join(root,file));
  for(const directory of protocol.discovery.directories){const path=join(root,directory);ask("list",path);try{for(const entry of await readdir(path,{withFileTypes:true}))if(entry.isDirectory())ask("presence",join(path,entry.name,protocol.discovery.nested));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"&&(error as NodeJS.ErrnoException).code!=="ENOTDIR")throw error;}}
  if(root!==workspace)for(let current=dirname(root);!(member&&current===workspace);current=dirname(current)){ask("text",join(current,protocol.manifest));if(current===parse(current).root)break;}
 }
 return [...rows.values()];
}
/** 📝️ Answers questions now; refuses (undefined) when an asked path that exists, or the nearest existing directory above an absent one, changed within the racy window before `startedMs`, so a recorded answer is never younger than the planning answer it vouches for. */
export async function cargoPlanAnswersV1(questions:readonly Omit<CargoPlanInputV1,"answer">[],startedMs:number|null,control:ResourceLeaseControl):Promise<readonly CargoPlanInputV1[]|undefined>{
 const horizon=startedMs===null?null:BigInt(Math.floor(startedMs-protocol.racyMilliseconds))*1000000n,young=async(path:string):Promise<boolean>=>{try{const info=await lstat(path,{bigint:true});return info.mtimeNs>=horizon!||info.ctimeNs>=horizon!;}catch(error){const code=(error as NodeJS.ErrnoException).code;if((code==="ENOENT"||code==="ENOTDIR")&&dirname(path)!==path)return young(dirname(path));return true;}},result:CargoPlanInputV1[]=[];
 for(const [index,row] of questions.entries()){
  if(index%256===0)checkResourceLeaseControl(control);
  if(horizon!==null&&await young(row.path))return undefined;
  result.push({...row,answer:await answerV1(row.kind,row.path)});
 }
 return result;
}
/** 🗄️ Keeps at most the protocol's number of planning records, gzip-compressed, in one directory; the least recently served record goes first, and a record that cannot be read or written only costs the next planning its Cargo processes.
 * @see ../🟦️.ts — `captureCargoCompilationGraphV1`, the consumer */
export function cargoPlanMemoV1(directory:string):CargoPlanMemoV1{
 const file=(key:string)=>join(directory,key+protocol.suffix);
 return Object.freeze({
  async recall(key:string,control:ResourceLeaseControl):Promise<CargoPlanAnswersV1|undefined>{
   let record:{version:number;key:string;inputs:CargoPlanInputV1[];metadata:unknown;units:unknown};
   try{record=JSON.parse(gunzipSync(await readFile(file(key))).toString("utf8"));}catch{return undefined;}
   if(validateJsonSchemaSubset(schema,record).length||record.key!==key)return undefined;
   const now=await cargoPlanAnswersV1(record.inputs,null,control);
   if(!now||now.some((row,index)=>row.answer!==record.inputs[index]!.answer))return undefined;
   const touched=new Date();await utimes(file(key),touched,touched).catch(()=>{});
   return Object.freeze({metadata:record.metadata,units:record.units});
  },
  async remember(key:string,answers:CargoPlanAnswersV1,startedMs:number,control:ResourceLeaseControl):Promise<void>{
   const questions=await cargoPlanQuestionsV1(answers.metadata),inputs=questions&&await cargoPlanAnswersV1(questions,startedMs,control);if(!inputs)return;
   const record={version:1,key,inputs,metadata:answers.metadata,units:answers.units};if(validateJsonSchemaSubset(schema,record).length)return;
   const target=file(key),temporary=target+"."+randomBytes(4).toString("hex");
   try{await mkdir(directory,{recursive:true});await writeFile(temporary,gzipSync(JSON.stringify(record),{level:1}));await rename(temporary,target);}catch{await rm(temporary,{force:true});return;}
   try{const kept=(await Promise.all((await readdir(directory)).filter(name=>name.endsWith(protocol.suffix)).map(async name=>({name,at:(await lstat(join(directory,name))).mtimeMs})))).sort((a,b)=>b.at-a.at);for(const row of kept.slice(protocol.maximumRecords))await rm(join(directory,row.name),{force:true});}catch{}
  },
 });
}
