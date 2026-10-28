import{expect,test}from"bun:test";
import Ajv from"ajv";
import{spawn}from"node:child_process";
import{createHash}from"node:crypto";
import{createRequire}from"node:module";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{buildSourceClosureIdentityV1,buildSourceClosureStreamLoaderV1}from"../../🟦️.ts";
import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{closureRefusalCatalogPathV1,closureRefusalMessagesV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";
import sitesCatalog from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json";
import{buildSourceClosureNodeArgumentsV1}from"../../🟢️node/🟨️.mjs";
import policyContract from"../../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

type Outcome={status:"ok";value:unknown}|{status:"refused";message:string};
const fixtureSchema={type:"object",additionalProperties:false,required:["version","entry","roots","files","late","cases","drift"],properties:{version:{const:1},entry:{type:"string"},roots:{type:"object",additionalProperties:{type:"array",items:{type:"string"}}},files:{type:"object",additionalProperties:{type:"string"}},late:{type:"object",additionalProperties:{type:"string"}},drift:{type:"object",additionalProperties:false,required:["file","text","code"],properties:{file:{type:"string"},text:{type:"string"},code:{enum:Object.keys(closureRefusalMessagesV1)}}},cases:{type:"array",minItems:1,items:{type:"object",additionalProperties:false,required:["name","expression","outcome"],properties:{name:{type:"string",minLength:1},expression:{type:"string",minLength:1},outcome:{enum:["ok","refused"]},value:{},code:{enum:Object.keys(closureRefusalMessagesV1)}}}}}};
const prefix=(code:string,language:"en"|"de")=>(closureRefusalMessagesV1 as Record<string,Record<string,{refusal:string}>>)[code]![language]!.refusal.split("{")[0]!;

/** ▶️ Runs the captured entry of the fixture workspace through the real stream loader. */
async function runLoaderV1(workspace:string,text:string,sha:string,language:string):Promise<Record<string,Outcome>|string>{
 const child=spawn(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",sha],{cwd:workspace,stdio:["ignore","pipe","pipe","pipe"],env:{...process.env,SEMIO_LANGUAGE:language}});let out="",err="";child.stdout!.on("data",data=>out+=data);child.stderr!.on("data",data=>err+=data);(child.stdio[3] as NodeJS.WritableStream).end(text);
 await new Promise(resolve=>child.on("close",resolve));const line=out.split("\n").find(row=>row.startsWith("RESULT "));return line?JSON.parse(line.slice(7)):err;
}

/** 🧪️ Every derived site class resolves, and every undeclared one is refused with a named English and German text. */
test("the real loader serves derived packages and roots and refuses everything else with named texts",async()=>{
 expect(new Ajv({strict:false,allowUnionTypes:true}).validate(fixtureSchema,fixture)).toBe(true);
 const area=mkdtempSync(join(realpathSync(tmpdir()),"cs-")),put=(path:string,text:string)=>{mkdirSync(dirname(join(area,path)),{recursive:true});writeFileSync(join(area,path),text);};
 try{
  for(const[path,text]of Object.entries(fixture.files))put(path,text);
  const entry=`import{createRequire}from"node:module";import{readFileSync}from"node:fs";import{join}from"node:path";import helper from"./helper.ts";import fakeStatic from"fake-pkg";
export async function runCapturedBuildSourceV1(){const out:Record<string,unknown>={},req=createRequire(import.meta.url),root=import.meta.dir;void join;void readFileSync;
 const t=async(name:string,f:()=>unknown)=>{try{out[name]={status:"ok",value:await f()};}catch(error){out[name]={status:"refused",message:String((error as Error).message)};}};
${fixture.cases.map(row=>` await t(${JSON.stringify(row.name)},async()=>(${row.expression}));`).join("\n")}
 console.log("RESULT "+JSON.stringify(out));}`;
  put(fixture.entry,entry);
  const source=await captureNxAuthorizerSourcesV1(area,fixture.entry,new AbortController().signal,{installed:true,roots:new Map(Object.entries(fixture.roots))}),bytes=new Map(nxAuthorizerCapturedBytesV1(source)),imports=new Map(nxAuthorizerCapturedImportsV1(area,source));bytes.set(closureRefusalCatalogPathV1,Buffer.from(JSON.stringify(sitesCatalog)));imports.set(closureRefusalCatalogPathV1,{});
  const files=[...bytes].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))).map(([path,value])=>({path,sha256:createHash("sha256").update(value).digest("hex"),base64:Buffer.from(value).toString("base64"),imports:imports.get(path)!})),snapshot={version:1 as const,root:area,entry:fixture.entry,files},text=JSON.stringify(snapshot),sha=buildSourceClosureIdentityV1(snapshot),bare={...snapshot,files:files.filter(file=>file.path!==closureRefusalCatalogPathV1)},bareText=JSON.stringify(bare),bareSha=buildSourceClosureIdentityV1(bare);
  expect(imports.get(fixture.entry)!["fake-pkg"]).toBe("installed:fake-pkg@1.2.3");expect(imports.get(fixture.entry)!["@scoped/lib/main.js"]).toBe("installed:@scoped/lib@0.0.1");expect(imports.get(fixture.entry)!["semio-root:computed/a.mjs"]).toBe("computed/a.mjs");expect(files.map(file=>file.path)).not.toContain("computed/b.mjs");
  for(const[path,value]of Object.entries(fixture.late))put(path,value);
  const native=createRequire(join(area,fixture.entry));expect(native("fake-pkg").value).toBe("fake-1.2.3");
  for(const language of["en","de"]as const){
   const result=await runLoaderV1(area,text,sha,language);expect(typeof result,String(result)).toBe("object");
   for(const row of fixture.cases){const actual=(result as Record<string,Outcome>)[row.name]!;expect(actual,row.name).toBeDefined();
    if(row.outcome==="ok"){expect(actual,row.name).toEqual({status:"ok",value:row.value});}
    else{expect(actual.status,row.name).toBe("refused");const message=(actual as{message:string}).message;expect(message,row.name).toContain(prefix(row.code!,language));expect(message,row.name).not.toContain(prefix(row.code!,language==="en"?"de":"en"));expect(message,row.name).toContain("Fix:".replace("Fix:",language==="en"?"Fix:":"Behebung:"));}}
  }
  const bareResult=await runLoaderV1(area,bareText,bareSha,"en");expect(typeof bareResult,String(bareResult)).toBe("object");
  for(const row of fixture.cases)if(row.outcome==="refused"){const message=((bareResult as Record<string,Outcome>)[row.name] as{message:string}).message;expect(message,row.name).toStartWith(row.code+": {");expect(message,row.name).not.toContain("Fix:");}
  put(fixture.drift.file,fixture.drift.text);
  const drifted=await runLoaderV1(area,text,sha,"en");expect(typeof drifted).toBe("string");expect(drifted as string).toContain(prefix(fixture.drift.code,"en"));expect(drifted as string).toContain("fake-pkg@1.2.3 -> 9.9.9");
 }finally{rmSync(area,{recursive:true,force:true});}
},120000);

/** 📦️ The pinned system Node loader accepts the same derived installed-package targets and loads them natively. */
test("the Node loader serves installed package targets",async()=>{
 const node=Bun.which("node");if(!node)return;
 const area=mkdtempSync(join(realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"cn-")),put=(path:string,text:string)=>{mkdirSync(dirname(join(area,path)),{recursive:true});writeFileSync(join(area,path),text);};
 try{
  for(const[path,text]of Object.entries(fixture.files))put(path,text);
  put("node-entry.mjs",`import fake from"fake-pkg";import{createRequire}from"node:module";export async function runCapturedBuildSourceV1(){console.log("NODE "+JSON.stringify({fake:fake.value}));}`);
  const source=await captureNxAuthorizerSourcesV1(area,"node-entry.mjs",new AbortController().signal,{installed:true}),bytes=nxAuthorizerCapturedBytesV1(source),imports=nxAuthorizerCapturedImportsV1(area,source);
  const files=[...bytes].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))).map(([path,value])=>({path,sha256:createHash("sha256").update(value).digest("hex"),base64:Buffer.from(value).toString("base64"),imports:imports.get(path)!})),snapshot={version:1 as const,root:area,entry:"node-entry.mjs",files},policy=policyContract.definitions.Policy.const,launch=buildSourceClosureNodeArgumentsV1(snapshot,policy,512,node);
  const child=spawn(node,[...launch.argv],{cwd:area,stdio:["pipe","pipe","pipe"]});let out="",err="";child.stdout.on("data",data=>out+=data);child.stderr.on("data",data=>err+=data);child.stdin.end(launch.bootstrap);await new Promise(resolve=>child.on("close",resolve));
  expect(out.split("\n").find(line=>line.startsWith("NODE ")),err).toBe('NODE {"fake":"fake-1.2.3"}');
 }finally{rmSync(area,{recursive:true,force:true});}
},60000);
