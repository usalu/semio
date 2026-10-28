import {existsSync,readdirSync,readFileSync} from "node:fs";
import {join,posix} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {parseJsoncV1} from "../🔌️managers/📖️formats/🟦️.ts";
import {selectPackageManagerV1} from "../🔌️managers/📚️catalog/🟦️.ts";
import{refuseBootstrapV1}from"../../📖️texts/🟦️.ts";
import type {LockDriftV1} from "../🔌️managers/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";

const protocol=schema.$defs.Protocol.const,texts=schema.$defs.Texts.const as Readonly<Record<string,Readonly<Record<string,string>>>>;
const fill=(template:string,values:Readonly<Record<string,string|number|undefined>>):string=>Object.entries(values).reduce((text,[key,value])=>text.replaceAll("{"+key+"}",String(value??"")),template);

/** 🔒️ One dependency scope as the lock check reads it: the directory that owns the lock, the lock text and the member manifests keyed by manifest path relative to that directory. */
export type JavascriptLockScopeV1=Readonly<{scope:string;lock:string;manifests:ReadonlyMap<string,unknown>}>;
/** 📊️ What the lock check found in one scope. */
export type JavascriptLockReportV1=Readonly<{scope:string;drift:readonly LockDriftV1[]}>;

const memberKeyV1=(member:string):string=>posix.normalize(member.replaceAll("\\","/")).replace(/\/$/u,"");
const readManifestV1=(path:string):unknown=>{const text=readFileSync(path,"utf8");try{return JSON.parse(text);}catch{return parseJsoncV1(text);}};

/** 🧭️ Reads every dependency scope of the checkout without walking it: the root scope and each directory its `semio.workspace.owners` pattern (one level below the root) names that carries a lock file; the members are the explicit list the scope's last sync published. */
export function readJavascriptLockScopesV1(root:string):readonly JavascriptLockScopeV1[]{
 const manager=selectPackageManagerV1("javascript"),lockName=manager.primaryLockName(),manifestName=manager.manifestFileNames()[0]!,rootManifest=readManifestV1(join(root,manifestName)) as {semio?:{workspace?:{owners?:unknown}}},directories=new Set<string>(["."]);
 for(const pattern of Array.isArray(rootManifest.semio?.workspace?.owners)?rootManifest.semio!.workspace!.owners as unknown[]:[]){
  const match=typeof pattern==="string"?new RegExp(protocol.ownerPattern,"u").exec(pattern):null;
  if(!match)throw refuseBootstrapV1("invalid","dependencies.owner-pattern-required",{pattern:String(pattern)});
  for(const entry of readdirSync(root,{withFileTypes:true}))if(entry.isDirectory()&&existsSync(join(root,entry.name,match[1]!))&&existsSync(join(root,entry.name,lockName)))directories.add(entry.name);
 }
 return Object.freeze([...directories].sort().filter(directory=>existsSync(join(root,directory,lockName))).map(directory=>{
  const base=directory==="."?root:join(root,directory),manifest=readManifestV1(join(base,manifestName)) as {workspaces?:unknown},manifests=new Map<string,unknown>([[manifestName,manifest]]);
  for(const member of Array.isArray(manifest.workspaces)?manifest.workspaces as unknown[]:[]){
   if(typeof member!=="string"||member.includes("*"))continue;
   const key=memberKeyV1(member),file=join(base,key,manifestName);
   if(existsSync(file))manifests.set(key+"/"+manifestName,readManifestV1(file));
  }
  return Object.freeze({scope:directory,lock:readFileSync(join(base,lockName),"utf8"),manifests});
 }));
}

/** 🩺️ Compares every scope's lock with its manifests through the selected package manager. */
export function checkJavascriptLockScopesV1(scopes:readonly JavascriptLockScopeV1[]):readonly JavascriptLockReportV1[]{
 const manager=selectPackageManagerV1("javascript");
 return Object.freeze(scopes.map(scope=>Object.freeze({scope:scope.scope,drift:manager.checkLock(scope.lock,scope.manifests)})));
}

const languagesV1=(environment:Readonly<Record<string,string|undefined>>):readonly string[]=>{try{return [resolveLanguageV1(protocol.languages,[],environment).language];}catch{return protocol.languages;}};
const describeV1=(language:string,row:LockDriftV1):string=>{
 const table=texts[language]!,member=row.manifest.replace(/\/?package\.json$/u,"")||table.root!;
 return fill(table[row.kind]!,{member,section:row.section,dependency:row.dependency,manifestVersion:row.manifestVersion??table.absent,lockVersion:row.lockVersion??table.absent});
};
const changesV1=(language:string,drift:readonly LockDriftV1[]):string=>{
 const table=texts[language]!,listed=drift.slice(0,protocol.maximumListed).map(row=>describeV1(language,row));
 if(drift.length>listed.length)listed.push(fill(table.more!,{count:drift.length-listed.length}));
 return listed.length?listed.join("; "):table.unexplained!;
};

/** 📣️ Words one stale or refused scope in every language it is asked for, with the exact command that regenerates its lock. */
export function javascriptLockRefusalTextV1(kind:"stale"|"refused",scope:string,drift:readonly LockDriftV1[],languages:readonly string[]):string{
 const command=fill(protocol.command,{scope});
 return languages.map(language=>fill(texts[language]![kind]!,{scope,changes:changesV1(language,drift),command})).join("\n");
}

const refusalV1=(kind:"stale"|"refused",reports:readonly JavascriptLockReportV1[],environment:Readonly<Record<string,string|undefined>>):Error=>{
 const languages=languagesV1(environment);
 return Error(reports.map(report=>javascriptLockRefusalTextV1(kind,report.scope,report.drift,languages)).join("\n"),{cause:Object.freeze({version:1,purpose:"dependency-lock-refusal",kind,scopes:reports.map(report=>Object.freeze({scope:report.scope,command:fill(protocol.command,{scope:report.scope}),drift:report.drift,en:javascriptLockRefusalTextV1(kind,report.scope,report.drift,["en"]),de:javascriptLockRefusalTextV1(kind,report.scope,report.drift,["de"])}))})});
};
const frozenV1=(reports:readonly JavascriptLockReportV1[]):readonly JavascriptLockReportV1[]=>reports.map(report=>Object.freeze({scope:report.scope,drift:report.drift.filter(row=>row.frozen)})).filter(report=>report.drift.length>0);

/** 🚦️ Refuses at once, before any capacity wait, store custody or installer, every selected scope whose lock Bun would refuse to install frozen; `owners` selects scopes by directory, none selects all. */
export function requireJavascriptLocksInSyncV1(root:string,owners:readonly string[]|undefined,environment:Readonly<Record<string,string|undefined>>):void{
 const stale=frozenV1(checkJavascriptLockScopesV1(readJavascriptLockScopesV1(root).filter(scope=>owners===undefined||owners.includes(scope.scope))));
 if(stale.length)throw refusalV1("stale",stale,environment);
}

/** 🧪️ One scope staged for an installer: its generation directory with the projected manifests (`w/<index>` per member, in member order) and the projected lock. */
export type StagedJavascriptLockV1=Readonly<{scope:string;members:readonly string[];directory:string;files:readonly Readonly<{path:string;bytes:Uint8Array}>[]}>;

/** ✅️ Compares a staged scope's projected lock with its projected manifests (exactly the bytes Bun will read) and names the findings by the original members. */
export function checkStagedJavascriptLockV1(staged:StagedJavascriptLockV1):JavascriptLockReportV1|undefined{
 const manager=selectPackageManagerV1("javascript"),lockName=manager.primaryLockName(),manifestName=manager.manifestFileNames()[0]!,bytes=new Map(staged.files.map(file=>[file.path,file.bytes] as const)),lock=bytes.get(join(staged.directory,lockName));
 if(!lock)return undefined;
 const manifests=new Map<string,unknown>(),decode=(value:Uint8Array)=>parseJsoncV1(new TextDecoder().decode(value));
 const root=bytes.get(join(staged.directory,manifestName));if(!root)return undefined;manifests.set(manifestName,decode(root));
 staged.members.forEach((_,index)=>{const value=bytes.get(join(staged.directory,"w",String(index),manifestName));if(value)manifests.set("w/"+index+"/"+manifestName,decode(value));});
 const original=(path:string):string=>{const match=new RegExp(protocol.stagedMember,"u").exec(path.endsWith("/"+manifestName)?path.slice(0,-manifestName.length-1):path),index=match?Number(match[1]):-1;return index>=0&&index<staged.members.length?memberKeyV1(staged.members[index]!)+"/"+manifestName:path;};
 return Object.freeze({scope:staged.scope,drift:Object.freeze(manager.checkLock(new TextDecoder().decode(lock),manifests).map(row=>Object.freeze({...row,manifest:original(row.manifest)})))});
}

/** 🛑️ Refuses before any installer starts every staged scope whose projected lock Bun would refuse to install frozen. */
export function requireStagedJavascriptLocksInSyncV1(staged:readonly StagedJavascriptLockV1[],environment:Readonly<Record<string,string|undefined>>):void{
 const stale=frozenV1(staged.flatMap(row=>{const report=checkStagedJavascriptLockV1(row);return report?[report]:[];}));
 if(stale.length)throw refusalV1("stale",stale,environment);
}

/** 🔁️ Translates the installer's own refusal of a frozen install into the same words: the scope, what the static comparison of the staged inputs found and the command that fixes it; any other failure is not this refusal. */
export function javascriptFrozenInstallRefusalV1(scope:string,stderr:string,staged:StagedJavascriptLockV1|undefined,environment:Readonly<Record<string,string|undefined>>):Error|undefined{
 const pattern=(protocol.frozenRefusalPatterns as Readonly<Record<string,string>>)[selectPackageManagerV1("javascript").id];
 if(!pattern||!stderr.includes(pattern))return undefined;
 return refusalV1("refused",[Object.freeze({scope,drift:staged?checkStagedJavascriptLockV1(staged)?.drift??[]:[]})],environment);
}
