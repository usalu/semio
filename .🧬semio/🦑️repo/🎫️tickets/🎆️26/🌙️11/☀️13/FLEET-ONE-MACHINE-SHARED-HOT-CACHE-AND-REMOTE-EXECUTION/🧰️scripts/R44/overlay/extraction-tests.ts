import {expect,setDefaultTimeout,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {groupV1,readTopologyV1,type FleetTopologyV1} from "../../🟦️.ts";
import fixture from "../../🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};

setDefaultTimeout(120_000);
const topologyDirectory=resolve(import.meta.dirname,"../.."),provisionDirectory=resolve(import.meta.dirname,".."),oracle=join(import.meta.dirname,"🐍️.py");
const named=(directory:string,suffix:string):string=>join(directory,readdirSync(directory).find(name=>name.endsWith(suffix))!);
const shellText=readFileSync(named(provisionDirectory,".sh"),"utf8"),powershellText=readFileSync(named(provisionDirectory,".ps1"),"utf8"),topologyFile=named(topologyDirectory,".json");
const python=testPythonV1(),bash=Bun.which("bash")!,jq=Bun.which("jq"),pwsh=Bun.which("pwsh");
const PLATFORMS=["linux","macos"] as const,PERMUTATIONS=60;
const scratch=mkdtempSync(join(tmpdir(),"extraction-"));
process.on("exit",()=>{rmSync(scratch,{recursive:true,force:true});});

type Random=()=>number;
const seeded=(seed:number):Random=>()=>{seed=(seed+0x6d2b79f5)|0;let t=Math.imul(seed^(seed>>>15),1|seed);t=(t+Math.imul(t^(t>>>7),61|t))^t;return((t^(t>>>14))>>>0)/4294967296;};
const pick=<T>(random:Random,items:readonly T[]):T=>items[Math.floor(random()*items.length)]!;
const shuffle=<T>(random:Random,items:readonly T[]):T[]=>{const copy=[...items];for(let at=copy.length-1;at>0;at--){const other=Math.floor(random()*(at+1));[copy[at],copy[other]]=[copy[other]!,copy[at]!];}return copy;};
const isObject=(value:unknown):value is Record<string,unknown>=>typeof value==="object"&&value!==null&&!Array.isArray(value);

const UNKNOWN_VALUES:readonly unknown[]=[
 7,true,null,"plain","}{\"id\":\"g1\"}","a \"quoted\" ] , [ { value \\ with \\\\ slashes","host",[1,[2,{"nfs":{"host":"decoy.invalid"}}],"]"],
 {"id":"g1","nfs":{"host":"decoy.invalid","export":"/decoy"},"server":{"platform":"windows","protocol":"smb"}},
 {"worktree":{"linux":"/decoy","macos":"/decoy"},"groups":[{"id":"g1","nfs":{"host":"decoy.invalid","export":"/decoy"},"server":{"platform":"windows","protocol":"smb"}}]},
 {"platform":"decoy","protocol":"decoy","host":"decoy","export":"decoy","originId":"decoy","groupId":"decoy","code":"d".repeat(64),"sha256":"d".repeat(64),"files":0,"archive":"decoy.tar","digest":"decoy","createdMs":1}
];
const UNKNOWN_KEYS=["ports","access","links","labels","x","host","id","nfs","server","platform","protocol","worktree","groups","version","émoji🦑","with space","with\"quote","with{brace","with.dot","with[0]"];

function reorder(random:Random,value:unknown,extras:boolean,name:string=""):unknown{
 if(Array.isArray(value))return (name==="groups"?shuffle(random,value):value).map(item=>reorder(random,item,extras));
 if(!isObject(value))return value;
 const entries=Object.entries(value).map(([key,item])=>[key,reorder(random,item,extras,key)] as const);
 if(extras&&random()<0.55)for(let more=1+Math.floor(random()*3);more>0;more--){
  const key=`${pick(random,UNKNOWN_KEYS)}${random()<0.5?"":`-${Math.floor(random()*1000)}`}`;
  if(entries.some(([name])=>name===key))continue;
  entries.push([key,structuredClone(pick(random,UNKNOWN_VALUES))]);
 }
 return Object.fromEntries(shuffle(random,entries));
}

const SPACES=["","  "," ","\t","\n","\r\n","\n  ","\n\t\t"," \r\n "];
function render(random:Random,value:unknown,style:"compact"|"random"|"pretty",depth:number=0):string{
 const gap=():string=>style==="compact"?"":style==="pretty"?"":pick(random,SPACES);
 const string=(text:string):string=>`"${[...text].map(character=>{
  if(character==='"')return '\\"';
  if(character==="\\")return "\\\\";
  if(character==="/"&&style==="random"&&random()<0.5)return "\\/";
  const code=character.codePointAt(0)!;
  if(code<32||(style==="random"&&code>=32&&code<127&&random()<0.08))return `\\u${code.toString(16).padStart(4,"0")}`;
  return character;
 }).join("")}"`;
 if(typeof value==="string")return string(value);
 if(!isObject(value)&&!Array.isArray(value))return JSON.stringify(value);
 if(style==="pretty")return JSON.stringify(value,null,pick(random,[1,2,4,"\t"]));
 if(Array.isArray(value))return `[${gap()}${value.map(item=>render(random,item,style,depth+1)).join(`${gap()},${gap()}`)}${gap()}]`;
 return `{${gap()}${Object.entries(value).map(([key,item])=>`${string(key)}${gap()}:${gap()}${render(random,item,style,depth+1)}`).join(`${gap()},${gap()}`)}${gap()}}`;
}
const document=(random:Random,value:unknown):string=>{
 const style=pick(random,["compact","random","random","pretty"] as const),text=render(random,value,style);
 return `${random()<0.15?"﻿":""}${text}${pick(random,["","\n","\r\n","  \n\n"])}`;
};

const RUN_ENV={...process.env} as Record<string,string|undefined>;
for(const name of ["SEMIO_FLEET_GROUP","SEMIO_FLEET_NFS_HOST","SEMIO_FLEET_NETWORK","SEMIO_FLEET_CACHE_ENDPOINT"])delete RUN_ENV[name];
const region=(name:string):string=>shellText.slice(shellText.indexOf(`# #region ${name}`),shellText.indexOf(`# #endregion ${name}`));
const sh=(script:string,env:Record<string,string>={}):{out:string;err:string;code:number}=>{
 const run=Bun.spawnSync([bash,"-c",script],{env:{...RUN_ENV,...env},stdout:"pipe",stderr:"pipe"});
 return {out:run.stdout.toString(),err:run.stderr.toString(),code:run.exitCode};
};
const chunkStart=shellText.indexOf("TOPOLOGY_FLAT=$(json_flat"),chunkEndLine=shellText.indexOf("SERVER_PROTOCOL=$(json_fact"),chunk=shellText.slice(chunkStart,shellText.indexOf("\n",chunkEndLine)+1);

const factLine=(group:string,platform:string,template:string,host:string,exported:string,serverPlatform:string,protocol:string):string=>[group,platform,template,host,exported,serverPlatform,protocol].join("\t");
function typescriptFacts(raw:unknown):string{
 const topology:FleetTopologyV1=readTopologyV1(raw),lines:string[]=[];
 for(const {id} of topology.groups){
  const group=groupV1(topology,id);
  for(const platform of PLATFORMS)lines.push(factLine(id,platform,topology.worktree[platform],group.nfs.host,group.nfs.export,group.server.platform,group.server.protocol));
 }
 return lines.sort().join("\n");
}
function shellFacts(file:string,groups:readonly string[]):{out:string;err:string;code:number}{
 const script=`set -euo pipefail
fail(){ printf 'FAIL %s\\n' "$*"; exit 3; }
WORKER_HOME=/home/probe
${region("🔎️Json")}
for GROUP_ID in ${groups.join(" ")}; do
 for PLATFORM in ${PLATFORMS.join(" ")}; do
${chunk}
  printf '%s\\t%s\\t%s\\t%s\\t%s\\t%s\\t%s\\n' "$GROUP_ID" "$PLATFORM" "$WORKTREE_TEMPLATE" "$NFS_HOST" "$NFS_EXPORT" "$SERVER_PLATFORM" "$SERVER_PROTOCOL"
 done
done`;
 const run=sh(script,{TOPOLOGY_JSON:file});
 return {...run,out:run.out.trimEnd().split("\n").sort().join("\n")};
}
const write=(name:string,text:string):string=>{const file=join(scratch,name);mkdirSync(join(file,".."),{recursive:true});writeFileSync(file,text);return file;};

const committed=JSON.parse(readFileSync(topologyFile,"utf8")) as unknown;
const sources:readonly (readonly [string,unknown])[]=[["committed topology",committed],["fixture topology",fixture.topology]];

for(const [label,raw] of sources){
 test(`${label}: the facts that the shell script extracts equal the facts of the TypeScript reader, the Python json module and jq, for ${PERMUTATIONS} permutations of key and group order, unknown keys, whitespace, escapes and byte order marks`,()=>{
  const expected=typescriptFacts(raw),groups=(raw as {groups:{id:string}[]}).groups.map(group=>group.id);
  expect(expected.split("\n").length).toBe(groups.length*PLATFORMS.length);
  const random=seeded(label.length*7919);
  for(let round=0;round<PERMUTATIONS;round++){
   const extras=round>=PERMUTATIONS/4,text=document(random,reorder(random,raw,extras)),file=write(`${label.replaceAll(" ","-")}/${round}.json`,text);
   const found=shellFacts(file,groups);
   expect(found.err,`round ${round}`).toBe("");
   expect(found.code,`round ${round}: ${found.out}`).toBe(0);
   expect(found.out,`round ${round}: ${text.slice(0,300)}`).toBe(expected);
   if(python){
    const run=Bun.spawnSync([python,"-I",oracle,"facts",file,...PLATFORMS],{stdout:"pipe",stderr:"pipe"});
    expect(run.stdout.toString().trimEnd(),`oracle round ${round}`).toBe(expected);
   }
   if(jq){
    const program=`.groups[] as $g | ${JSON.stringify(PLATFORMS)}[] as $p | [$g.id,$p,.worktree[$p],$g.nfs.host,$g.nfs.export,$g.server.platform,$g.server.protocol] | @tsv`;
    const run=Bun.spawnSync([jq,"-r",program,file],{stdout:"pipe",stderr:"pipe"});
    if(run.exitCode===0)expect(run.stdout.toString().trimEnd().split("\n").sort().join("\n"),`jq round ${round}`).toBe(expected);
   }
  }
 });
}

test("the incident: ports and access between id and nfs, the server before nfs, and an unknown key before id change no fact",()=>{
 const group=(committed as {groups:Record<string,unknown>[]}).groups[0]!,expected=typescriptFacts(committed);
 const orders=[["id","ports","access","nfs","server","cache","peers"],["id","nfs","server","ports","access","cache","peers"],["server","nfs","id"],["peers","cache","server","access","ports","nfs","id"],["future","id","nfs","server"]];
 for(const keys of orders){
  const groups=(committed as {groups:Record<string,unknown>[]}).groups.map(item=>Object.fromEntries(keys.map(key=>[key,key==="future"?{id:"g9"}:item[key]]).filter(([,value])=>value!==undefined)));
  const file=write(`incident/${keys.join("-")}.json`,JSON.stringify({...(committed as object),groups},null,1));
  expect(shellFacts(file,groups.map(item=>item["id"] as string)).out).toContain(factLine("g1","macos",(committed as {worktree:{macos:string}}).worktree.macos,(group["nfs"] as {host:string}).host,(group["nfs"] as {export:string}).export,(group["server"] as {platform:string}).platform,(group["server"] as {protocol:string}).protocol));
 }
 expect(expected).toContain("g1\tmacos\t~/Documents/semio");
});

test("a topology that is not valid JSON, repeats a key, ends early or has text after it is refused with the reason, and an unknown group is empty instead of another group",()=>{
 const refused:readonly (readonly [string,string])[]=[["duplicate key",'{"a":1,"a":2}'],["unterminated","{\"a\":\"x"],["incomplete","{\"a\":"],["text after","{\"a\":1} {\"b\":2}"],["unbalanced","{\"a\":[1}"],["empty",""],["key expected","{1:2}"],["comma","1,2"]];
 for(const [reason,text] of refused){
  const file=write(`refused/${reason.replaceAll(" ","-")}.json`,text),run=sh(`${region("🔎️Json")}\njson_flat < '${file}'`);
  expect(run.code,reason).toBe(1);expect(run.err,reason).toContain("json:");
 }
 const flat=sh(`${region("🔎️Json")}\nF=$(json_flat < '${topologyFile}'); printf '[%s][%s][%s]' "$(json_group "$F" nope)" "$(json_group "$F" g1)" "$(json_fact "$F" groups[0].nfs.nothing)"`);
 expect(flat.out).toBe("[][groups[0]][]");
 const twice=write("refused/twice.json",JSON.stringify({groups:[{id:"g1",nfs:{host:"a",export:"/a"},server:{platform:"macos",protocol:"nfs"}},{id:"g1",nfs:{host:"b",export:"/b"},server:{platform:"linux",protocol:"nfs"}}]}));
 expect(sh(`${region("🔎️Json")}\nF=$(json_flat < '${twice}'); printf '[%s]' "$(json_group "$F" g1)"`).out).toBe("[]");
});

const hex=(random:Random,length:number):string=>Array.from({length},()=>"0123456789abcdef"[Math.floor(random()*16)]).join("");
test("machine.json, manifest.json, attest.json and sealed.json are read by key, not by position: permuted, decorated with decoys of the same names and reformatted they give the same identity, archive, code digest and file list",()=>{
 const random=seeded(4711),flatOf=(file:string):string=>`F=$(json_flat < '${file}')`;
 for(let round=0;round<PERMUTATIONS;round++){
  const origin=`g1-m${String(round).padStart(2,"0")}`,digest=hex(random,64),archiveHash=hex(random,64),stem=hex(random,16);
  const machine={version:1,originId:origin,groupId:"g1",storeRoot:"/Users/Shared/semio-store",key:hex(random,32)},manifest={version:1,hlc:"1794732675192-000000-g1-m07",files:282214+round,bytes:9492531264,sha256:archiveHash,archive:`worktree-${stem}.tar`};
  const attest={version:1,originId:"g1-server",groupId:"g1",wallMs:1,code:digest};
  const entries=Array.from({length:3+Math.floor(random()*40)},(_,at)=>({path:`🧰️framework/🛍️products/m${at}/${hex(random,4)} x/🟦️.ts`,bytes:Math.floor(random()*100000),sha256:hex(random,64),executable:random()<0.3}));
  const seal={version:1,digest,createdMs:1794740227660+round,moduleRoot:"🧰️framework/m",files:entries};
  const files=[["machine",machine],["manifest",manifest],["attest",attest],["seal",seal]].map(([name,value])=>write(`identity/${round}-${name as string}.json`,document(random,reorder(random,value,true))));
  const run=sh(`${region("🔎️Json")}
${flatOf(files[0]!)}; printf '%s|%s\\n' "$(json_fact "$F" originId)" "$(json_fact "$F" groupId)"
${flatOf(files[1]!)}; printf '%s|%s|%s\\n' "$(json_fact "$F" archive)" "$(json_fact "$F" sha256)" "$(json_fact "$F" files)"
${flatOf(files[2]!)}; printf '%s\\n' "$(json_fact "$F" code)"
${flatOf(files[3]!)}; printf '%s|%s|%s\\n' "$(json_fact "$F" version)" "$(json_fact "$F" digest)" "$(json_fact "$F" createdMs)"
json_flat < '${files[3]!}' | json_seal_files '${join(scratch,`identity/${round}-summary`)}'
printf 'summary %s\\n' "$(cat '${join(scratch,`identity/${round}-summary`)}')"`);
  expect(run.err,`round ${round}`).toBe("");
  const lines=run.out.trimEnd().split("\n");
  expect(lines.slice(0,4),`round ${round}`).toEqual([`${origin}|g1`,`${manifest.archive}|${archiveHash}|${manifest.files}`,digest,`1|${digest}|${seal.createdMs}`]);
  expect(lines.at(-1)).toBe(`summary ${entries.length} ${entries.length}`);
  expect(lines.slice(4,-1)).toEqual(entries.map(entry=>`${entry.executable?1:0}\t${entry.bytes}\t${entry.sha256}\t${entry.path}`));
 }
});

test("a seal entry that lacks a field, has a short hash, a negative size or a non-boolean execute bit is counted as listed but not readable, so the overlay refuses it",()=>{
 const good={path:"a/b.ts",bytes:1,sha256:"a".repeat(64),executable:false};
 const bad:readonly Record<string,unknown>[]=[{bytes:1,sha256:"a".repeat(64),executable:false},{path:"x",sha256:"a".repeat(64),executable:false},{path:"x",bytes:1,executable:false},{path:"x",bytes:1,sha256:"a".repeat(63),executable:false},{path:"x",bytes:-1,sha256:"a".repeat(64),executable:false},{path:"x",bytes:1,sha256:"a".repeat(64),executable:"yes"},{path:"x",bytes:1,sha256:"A".repeat(64),executable:true},{path:"x",bytes:1,sha256:"a".repeat(64)}];
 for(const [at,entry] of bad.entries()){
  const file=write(`seal/bad-${at}.json`,JSON.stringify({version:1,files:[good,entry]})),summary=join(scratch,`seal/bad-${at}.summary`);
  sh(`${region("🔎️Json")}\njson_flat < '${file}' | json_seal_files '${summary}' >/dev/null`);
  expect(readFileSync(summary,"utf8").trim(),`entry ${at}`).toBe("2 1");
 }
});

test("the PowerShell script reads the topology with ConvertFrom-Json and UTF-8, and when PowerShell is installed its extraction gives the facts of the TypeScript reader for permuted topologies",()=>{
 expect(powershellText).not.toMatch(/Get-Content -Raw -LiteralPath [^|\n]*\| ConvertFrom-Json/u);
 expect(powershellText.match(/Get-Content -Raw -Encoding UTF8 -LiteralPath [^|\n]*\| ConvertFrom-Json/gu)!.length).toBeGreaterThanOrEqual(5);
 if(!pwsh)return;
 const start=powershellText.indexOf("$topologyFiles = @("),end=powershellText.indexOf("\n",powershellText.indexOf("$ServerProtocol = "))+1,segment=powershellText.slice(start,end);
 const random=seeded(31337),groups=(committed as {groups:{id:string}[]}).groups.map(group=>group.id);
 for(let round=0;round<12;round++){
  const folder=join(scratch,`pwsh-${round}`),script=join(folder,"provision","segment.ps1");
  write(`pwsh-${round}/topology.json`,document(random,reorder(random,committed,round>=3)));
  write(`pwsh-${round}/provision/segment.ps1`,`$ErrorActionPreference = 'Stop'\n$Server = $false\nforeach ($GroupId in $args) {\n${segment}\n"$GroupId|$Worktree|$NfsHost|$($group[0].nfs.export)|$ServerPlatform|$ServerProtocol"\n}\n`);
  const run=Bun.spawnSync([pwsh,"-NoProfile","-NonInteractive","-File",script,...groups],{env:RUN_ENV as Record<string,string>,stdout:"pipe",stderr:"pipe"});
  expect(run.stderr.toString(),`round ${round}`).toBe("");
  const topology=readTopologyV1(committed),expected=groups.map(id=>{const group=groupV1(topology,id);return `${id}|${topology.worktree.windows}|${group.nfs.host}|${group.nfs.export}|${group.server.platform}|${group.server.protocol}`;});
  expect(run.stdout.toString().trimEnd().split(/\r?\n/u)).toEqual(expected);
 }
});

test("no other place of the provisioning scripts reads JSON with a text pattern",()=>{
 const code=shellText.split("\n").filter(line=>!line.startsWith("#")).join("\n").replace(/JSON_FLAT_AWK='[\s\S]*?\n'\n/u,"");
 expect(code).not.toMatch(/json_string|json_object|TOPOLOGY_TEXT|MACHINE_TEXT|OLD_TEXT/u);
 expect(code).not.toMatch(/\bsed\b[^\n]*"(archive|sha256|files|version|digest|createdMs|code|originId|groupId|id|nfs|server)"/u);
 expect(code).not.toMatch(/\bgrep\b[^\n]*"(version|digest|createdMs|code|sha256)"/u);
 expect(code).not.toMatch(/\bgrep\b[^\n]*'\{"path"/u);
});
