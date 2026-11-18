import re,shutil
B="base/";W="work/"
def rd(n):return open(B+n,encoding="utf8").read()
def sub(t,old,new,cnt=1):
    assert t.count(old)==cnt,(old[:60],t.count(old));return t.replace(old,new)
# 1 failure module
t=rd("1")
t=sub(t,'import {lstatSync} from "node:fs";','import {lstatSync,mkdirSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";')
t=sub(t,'import {join} from "node:path";','import {join} from "node:path";\nimport {renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";')
t=sub(t,'const IMAGE_RECLAIM_COMMAND_V1="repo cache gc";','''const IMAGE_RECLAIM_COMMAND_V1="repo cache gc";
const FAILURE_MESSAGE_CHARACTERS_V1=300;
const FAILURE_FILES_KEPT_V1=16;

/** 📏️ The most bytes of one failure block in the store (a whole stack, both texts and the message fit well within it). */
export const FAILURE_BLOCK_BYTES_V1=65536;

/** 💬️ The one-line message of a failure for the refusal text: its name, message and code on one line, at most 300 characters, so that a remote machine can read what failed without the service log. */
export function failureMessageV1(error){
 const text=String(error?.message??error).replace(/\\s+/gu," ").trim(),code=typeof error?.code==="string"&&!text.includes(error.code)?" ["+error.code+"]":"";
 return(String(error?.name??"Error")+code+": "+text).slice(0,FAILURE_MESSAGE_CHARACTERS_V1);
}''')
t=sub(t,'export function failureRefusalV1(error,{log,','function failureRefusalCoreV1(error,{log,')
t=sub(t,'return{code:"internal",key:"internal",fields:{reference,log:logPath},retry};\n}','''return{code:"internal",key:"internal",fields:{reference,log:logPath,message:failureMessageV1(error)},retry};
}

/** 🧾️ The block the store keeps for one unexpected failure: the reference, name, code, the full message, the refusal text in English and German and the stack (`detail`), cut at `limit` bytes. */
export function failureBlockV1({reference,detail,error,refusal,at=new Date(),limit=FAILURE_BLOCK_BYTES_V1}){
 let words;try{words=renderBothV1(refusal.key,refusal.fields);}catch{words={en:"",de:""};}
 const head=["request-failed "+reference+" "+at.toISOString(),"name: "+String(error?.name??typeof error),"code: "+(typeof error?.code==="string"?error.code:"-"),"en: "+(words.en??""),"de: "+(words.de??""),"message: "+String(error?.message??error).slice(0,4000),"detail:",""].join("\\n");
 return Buffer.from(head+detail).subarray(0,limit).toString("utf8");
}

/** 📮️ Writes a failure block into the origin folder of the shared store (`<store>/v1/origins/<origin>/service-failure.txt` and `service-failure-<reference>.txt`, the newest 16 references kept) the moment it happens, from the service process itself, so that the cause of a failure on a machine without a shell is readable on the server within seconds. Returns the path written, or `undefined` when the environment names no store or origin or the write fails. */
export function mirrorFailureV1(block,reference,{environment=process.env}={}){
 const store=environment.SEMIO_FLEET_STORE_ROOT,origin=environment.SEMIO_FLEET_ORIGIN;
 if(!store||!origin||/[\\\\/]|^\\.+$/u.test(origin))return undefined;
 try{
  const directory=join(store,"v1","origins",origin);mkdirSync(directory,{recursive:true});
  for(const name of["service-failure-"+reference+".txt","service-failure.txt"]){const file=join(directory,name),temporary=file+"."+process.pid+".tmp";writeFileSync(temporary,block);renameSync(temporary,file);}
  const kept=readdirSync(directory).filter(name=>/^service-failure-[0-9a-f]{8}\\.txt$/u.test(name)).map(name=>[name,statSync(join(directory,name)).mtimeMs]).sort((a,b)=>b[1]-a[1]);
  for(const[name]of kept.slice(FAILURE_FILES_KEPT_V1))rmSync(join(directory,name),{force:true});
  return join(directory,"service-failure.txt");
 }catch{return undefined;}
}

/** 🛟️ Turns an unexpected failure of a request into the refusal the client receives (see `failureRefusalCoreV1` in this file for the kinds), and writes the block of every failure that carries a reference into the origin folder of the store. */
export function failureRefusalV1(error,options){
 const refusal=failureRefusalCoreV1(error,options),reference=refusal.fields?.reference;
 if(typeof reference==="string"&&/^[0-9a-f]{8}$/u.test(reference))mirrorFailureV1(failureBlockV1({reference,detail:failureDetailV1(error),error,refusal}),reference,{environment:options.environment});
 return refusal;
}''')
open(W+"1","w",encoding="utf8").write(t)
t=rd("2")
t+='export const FAILURE_BLOCK_BYTES_V1:number;\nexport function failureMessageV1(error:unknown):string;\nexport function failureBlockV1(input:{reference:string;detail:string;error:unknown;refusal:{key:string;fields:Record<string,string>};at?:Date;limit?:number}):string;\nexport function mirrorFailureV1(block:string,reference:string,options?:{environment?:Record<string,string|undefined>}):string|undefined;\n'
open(W+"2","w",encoding="utf8").write(t)
t=rd("3")
t=sub(t,'unexpected failure (reference {reference}); try again','unexpected failure (reference {reference}): {message}; try again')
t=sub(t,'unerwarteten Fehler gestoßen (Referenz {reference}); versuche','unerwarteten Fehler gestoßen (Referenz {reference}): {message}; versuche')
open(W+"3","w",encoding="utf8").write(t)

# 6 fixture
H="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🌐️host/"
fx='''import {createHash} from "node:crypto";
import {runHostForegroundV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
const[,,implementation]=process.argv;
const code=await runHostForegroundV1({implementation,identify:async({root})=>createHash("sha256").update(root).digest("hex"),policy:{residentCheckMilliseconds:50},loadGraph:async()=>{const error=new TypeError("fixture graph is not iterable "+"x".repeat(Number(process.env.HOST_FIXTURE_MESSAGE_PAD??0)));error.code="E_FIXTURE";throw error;},log:(event,detail)=>process.stderr.write("[DEBUG] "+event+" "+detail+"\\n")});
process.exit(code);
'''
open(W+"6","w",encoding="utf8").write(fx)
# 4 unit tests
t=rd("4")
t=sub(t,'import {accessDeniedOfV1,deniedPlaceOfV1,failureDetailV1,failureRefusalV1,','import {accessDeniedOfV1,deniedPlaceOfV1,failureBlockV1,failureDetailV1,failureMessageV1,failureRefusalV1,mirrorFailureV1,')
t=sub(t,'import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";','import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";')
t+='''
test("an unexpected failure carries its message in the refusal fields and in both texts, and its block lands in the origin folder of the store",()=>{
  const store=mkdtempSync(join(tmpdir(),"fail-store-")),error=Object.assign(new TypeError("x is not iterable"),{code:"E_BOOM"});
  try{
    const refusal=failureRefusalV1(error,{log:()=>undefined,logPath:"/log",root:"/Users/Shared/semio",environment:{SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:"g1-test"}});
    expect(refusal).toMatchObject({code:"internal",key:"internal",fields:{log:"/log",message:"TypeError [E_BOOM]: x is not iterable"}});
    const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8")),find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["internal"]?node:Object.values(node).map(find).find(Boolean)):undefined,table=find(all);
    for(const language of ["en","de"])expect(table[language].internal).toContain("{message}");
    const folder=join(store,"v1","origins","g1-test"),text=readFileSync(join(folder,"service-failure.txt"),"utf8");
    expect(text).toContain("request-failed "+refusal.fields.reference);expect(text).toContain("message: x is not iterable");expect(text).toContain("code: E_BOOM");
    expect(text).toContain("en: The resident service met an unexpected failure (reference "+refusal.fields.reference+"): TypeError [E_BOOM]: x is not iterable");
    expect(text).toContain("de: Der residente Dienst ist auf einen unerwarteten Fehler gestoßen (Referenz "+refusal.fields.reference+"): TypeError [E_BOOM]: x is not iterable");
    expect(text).toContain("TypeError: x is not iterable");expect(text).toContain("\\n    at ");
    expect(readFileSync(join(folder,"service-failure-"+refusal.fields.reference+".txt"),"utf8")).toBe(text);
  }finally{rmSync(store,{recursive:true,force:true});}
});

test("the block is bounded to 64 KiB and the message of the refusal to 300 characters, whatever the failure says",()=>{
  const huge=new Error("m".repeat(300000)),detail=failureDetailV1(huge),block=failureBlockV1({reference:"abcd0123",detail,error:huge,refusal:{key:"internal",fields:{reference:"abcd0123",log:"/log",message:failureMessageV1(huge)}}});
  expect(Buffer.byteLength(block)).toBeLessThanOrEqual(65536);expect(block.startsWith("request-failed abcd0123 ")).toBe(true);
  expect(failureMessageV1(huge).length).toBe(300);expect(failureMessageV1("plain text")).toBe("Error: plain text");expect(failureMessageV1("a\\n  b")).toBe("Error: a b");
});

test("no store or origin in the environment writes nothing, an origin that is a path writes nothing, and only the newest 16 references are kept",()=>{
  const store=mkdtempSync(join(tmpdir(),"fail-keep-"));
  try{
    expect(mirrorFailureV1("x","aaaaaaaa",{environment:{}})).toBeUndefined();
    expect(mirrorFailureV1("x","aaaaaaaa",{environment:{SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:"../x"}})).toBeUndefined();
    expect(mirrorFailureV1("x","aaaaaaaa",{environment:{SEMIO_FLEET_STORE_ROOT:store}})).toBeUndefined();
    expect(existsSync(join(store,"v1"))).toBe(false);
    const environment={SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:"g1-keep"},folder=join(store,"v1","origins","g1-keep");
    for(let at=0;at<20;at++){const reference=at.toString(16).padStart(8,"0");mirrorFailureV1("block "+at,reference,{environment});const stamp=new Date(1700000000000+at*1000);utimesSync(join(folder,"service-failure-"+reference+".txt"),stamp,stamp);}
    mirrorFailureV1("block last","ffffffff",{environment});
    const names=readdirSync(folder).filter(name=>/^service-failure-/u.test(name));
    expect(names.length).toBe(16);expect(names).toContain("service-failure-ffffffff.txt");expect(names).not.toContain("service-failure-00000000.txt");
    expect(readFileSync(join(folder,"service-failure.txt"),"utf8")).toBe("block last");
  }finally{rmSync(store,{recursive:true,force:true});}
});
'''
open(W+"4","w",encoding="utf8").write(t)
# 5 host tests
t=rd("5")
t=sub(t,'expect(JSON.stringify(error.message_)).not.toContain("secret implementation detail");expect(error.message_.de)','expect(error.message_.en).toContain("Error: secret implementation detail");expect(error.message_.de).toContain("Error: secret implementation detail");expect(error.message_.de)')
t=sub(t,'const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json"),MAIN=join(HERE,"🌐️host/🖥️host-main.mjs");','const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json"),MAIN=join(HERE,"🌐️host/🖥️host-main.mjs"),THROWING=join(HERE,"🌐️host/💥️throwing-host-main.mjs");')
t=sub(t,' test("a client of another implementation digest starts a new daemon and the old one retires",',''' test("real process: a resident service that throws refuses with its message in both languages and writes the whole block into the origin folder of the store within seconds",async()=>{
  const store=mkdtempSync(join(base,"store-")),env={...environment("t"),SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:"g1-real",HOST_FIXTURE_MESSAGE_PAD:"90000"},implementation=digestOf("proc-throw"),root=workspace("tw");
  const spawnService=spawnHostServiceV1({command:[process.execPath,THROWING,implementation],environment:env});
  const started=Date.now(),error=await requestPlanV1({root,implementation,targets:["app:test"],agent:"a",language:"en",environment:env,spawnService}).then(()=>undefined,reason=>reason as any);
  try{
   expect(error?.name).toBe("PlanRefusedErrorV1");expect(error.code).toBe("internal");
   const reference=/reference ([0-9a-f]{8})/u.exec(error.message_.en)![1]!;
   expect(error.message_.en).toContain("): TypeError [E_FIXTURE]: fixture graph is not iterable");expect(error.message_.de).toContain("): TypeError [E_FIXTURE]: fixture graph is not iterable");
   expect(error.message_.en.length).toBeLessThan(700);
   const file=join(store,"v1","origins","g1-real","service-failure.txt"),text=readFileSync(file,"utf8");
   expect(Date.now()-started).toBeLessThan(10000);
   expect(text.startsWith("request-failed "+reference+" ")).toBe(true);expect(text).toContain("code: E_FIXTURE");expect(text).toContain("name: TypeError");
   expect(text).toContain("en: The resident service met an unexpected failure (reference "+reference+")");expect(text).toContain("de: Der residente Dienst ist auf einen unerwarteten Fehler");
   expect(text).toContain("\\n    at ");expect(Buffer.byteLength(text)).toBeLessThanOrEqual(65536);expect(Buffer.byteLength(text)).toBeGreaterThan(60000);
   expect(existsSync(join(store,"v1","origins","g1-real","service-failure-"+reference+".txt"))).toBe(true);
  }finally{const status=await requestHostStatusV1({implementation,environment:env});if(status)process.kill(status.pid,"SIGTERM");rmSync(store,{recursive:true,force:true});}
 },60000);
 test("a client of another implementation digest starts a new daemon and the old one retires",''')
t=sub(t,'import {copyFileSync,mkdirSync,','import {copyFileSync,existsSync,mkdirSync,')
open(W+"5","w",encoding="utf8").write(t)
