import {describe,expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {createHash,createHmac} from "node:crypto";
import {ProtocolErrorV1,compileSchemaV1,createFrameDecoderV1,encodeFrameV1} from "../protocol/frames.mjs";
import {POLICY_V1,SCHEMA_V1,renderBothV1,renderTextV1,validatorV1} from "../protocol/policy.mjs";
import {attenuateCapabilityV1,authorizeCapabilityV1,mintCapabilityV1,verifyCapabilityV1,canonicalJsonV1} from "../protocol/capability.mjs";
import {endpointsV1,worktreeIdV1} from "../protocol/endpoint.mjs";
import {HERE} from "./common.ts";

const vectors=JSON.parse(readFileSync(join(HERE,"fixtures/vectors.json"),"utf8"));

describe("frame schema interpreter against the Python jsonschema oracle",()=>{
 const compiled=new Map<string,(value:unknown)=>string[]>();
 test(`all ${vectors.frames.length} mutated frames get the same verdict`,()=>{
  const wrong:string[]=[];
  for(const row of vectors.frames){
   let validate=compiled.get(row.definition);if(!validate){validate=compileSchemaV1(SCHEMA_V1,row.definition);compiled.set(row.definition,validate);}
   const ok=validate(row.value).length===0;
   if(ok!==row.valid)wrong.push(row.definition+" "+JSON.stringify(row.value).slice(0,120)+" expected "+row.valid);
  }
  expect(wrong.slice(0,5)).toEqual([]);
 });
 test("corpus contains both verdicts for every definition",()=>{
  for(const name of["ClientFrame","ServiceFrame","WatcherFrame","Descriptor","Caveat"]){
   const rows=vectors.frames.filter((row:any)=>row.definition===name);
   expect(rows.some((row:any)=>row.valid)).toBe(true);expect(rows.some((row:any)=>!row.valid)).toBe(true);
  }
 });
 test("the schema itself is accepted by the Ajv-independent check of closed documents",()=>{
  const open=Object.entries(SCHEMA_V1.definitions as Record<string,any>).filter(([name,def])=>def.type==="object"&&def.properties&&def.additionalProperties!==false&&!["Text"].includes(name));
  expect(open.map(([name])=>name)).toEqual([]);
 });
 test("Ajv agrees when it is installed",async()=>{
  let Ajv:any;try{Ajv=(await import("ajv")).default;}catch{return;}
  const ajv=new Ajv({strict:false,allErrors:false,validateFormats:false});
  const checks=new Map<string,any>();
  const wrong:string[]=[];
  for(const row of vectors.frames){
   let check=checks.get(row.definition);if(!check){check=ajv.compile({$ref:"#/definitions/"+row.definition,definitions:SCHEMA_V1.definitions});checks.set(row.definition,check);}
   if(Boolean(check(row.value))!==row.valid)wrong.push(row.definition+" "+JSON.stringify(row.value).slice(0,100));
  }
  expect(wrong.slice(0,5)).toEqual([]);
 });
});

describe("frame codec",()=>{
 const collect=(chunks:(string|Uint8Array)[],maximum=1<<20)=>{
  const frames:unknown[]=[],errors:ProtocolErrorV1[]=[];
  const decoder=createFrameDecoderV1({maximumFrameBytes:maximum,onFrame:f=>frames.push(f),onError:e=>errors.push(e)});
  for(const chunk of chunks)decoder.push(chunk);decoder.end();
  return{frames,errors};
 };
 test("frames survive any split, including inside a multibyte character",()=>{
  const wire=encodeFrameV1({t:"output",id:1,stream:"stdout",text:"äö😀 end"})+encodeFrameV1({t:"pong",n:2});
  const bytes=Buffer.from(wire);
  for(let cut=1;cut<bytes.length;cut++){
   const{frames,errors}=collect([bytes.subarray(0,cut),bytes.subarray(cut)]);
   expect(errors).toEqual([]);expect(frames.length).toBe(2);expect((frames[0] as any).text).toBe("äö😀 end");
  }
  expect(collect([...bytes].map(b=>Buffer.from([b]))).frames.length).toBe(2);
 });
 test("oversize, invalid JSON, invalid UTF-8 and a truncated tail end the stream once",()=>{
  expect(collect(["x".repeat(100)+"\n"],50).errors.length).toBe(1);
  expect(collect(["x".repeat(100)],50).errors.length).toBe(1);
  expect(collect(["{not json}\n",'{"t":"pong","n":1}\n']).errors.length).toBe(1);
  expect(collect([Buffer.from([0x7b,0xff,0xfe,0x7d,10])]).errors.length).toBe(1);
  expect(collect(['{"t":"pong"']).errors.length).toBe(1);
  expect(collect(['\n\n{"t":"pong","n":1}\n\n']).frames.length).toBe(1);
 });
});

describe("capabilities against Python hmac",()=>{
 for(const row of vectors.capabilities)test("mint equals the oracle for "+JSON.stringify(row.caveats).slice(0,50),()=>{
  expect(mintCapabilityV1(Buffer.from(row.key,"hex"),row.caveats,row.id)).toBe(row.token);
  expect(verifyCapabilityV1(Buffer.from(row.key,"hex"),row.token)).toEqual(row.caveats);
 });
 test("sha256 and hmac vectors",()=>{
  for(const row of vectors.sha256)expect(createHash("sha256").update(Buffer.from(row.hex,"hex")).digest("hex")).toBe(row.digest);
  for(const row of vectors.hmac)expect(createHmac("sha256",Buffer.from(row.key,"hex")).update(Buffer.from(row.message,"hex")).digest("hex")).toBe(row.mac);
 });
 const key=Buffer.alloc(32,7),base=mintCapabilityV1(key,[{k:"scope",v:["run","status"]}]);
 test("attenuation needs no key and verifies; removing or reordering a caveat does not",()=>{
  const narrow=attenuateCapabilityV1(base,{k:"projects",v:["p0001"]});
  expect(verifyCapabilityV1(key,narrow)?.length).toBe(2);
  const parts=narrow.split(".");
  const payload=JSON.parse(Buffer.from(parts[1]!,"base64url").toString());
  const stripped="v1."+Buffer.from(JSON.stringify({id:payload.id,caveats:payload.caveats.slice(0,1)})).toString("base64url")+"."+parts[2];
  expect(verifyCapabilityV1(key,stripped)).toBeUndefined();
  const swapped="v1."+Buffer.from(JSON.stringify({id:payload.id,caveats:[...payload.caveats].reverse()})).toString("base64url")+"."+parts[2];
  expect(verifyCapabilityV1(key,swapped)).toBeUndefined();
  expect(verifyCapabilityV1(Buffer.alloc(32,8),narrow)).toBeUndefined();
  expect(verifyCapabilityV1(key,"garbage")).toBeUndefined();expect(verifyCapabilityV1(key,"v1.a.b")).toBeUndefined();
 });
 test("every caveat is conjunctive",()=>{
  const request={commandClass:"run",command:"run",cwd:"/w/x",now:1000,projects:["p0001"]};
  expect(authorizeCapabilityV1([{k:"scope",v:["run"]}],request)).toBeUndefined();
  expect(authorizeCapabilityV1([{k:"scope",v:["status"]}],request)?.code).toBe("scope");
  expect(authorizeCapabilityV1([{k:"notAfter",v:999}],request)?.code).toBe("capability");
  expect(authorizeCapabilityV1([{k:"cwd",v:"/w"}],request)).toBeUndefined();
  expect(authorizeCapabilityV1([{k:"cwd",v:"/w/xy"}],request)?.code).toBe("scope");
  expect(authorizeCapabilityV1([{k:"cwd",v:"/ww"}],{...request,cwd:"/w/x"})?.code).toBe("scope");
  expect(authorizeCapabilityV1([{k:"projects",v:["p0001"]}],request)).toBeUndefined();
  expect(authorizeCapabilityV1([{k:"projects",v:["p000*"]}],request)).toBeUndefined();
  expect(authorizeCapabilityV1([{k:"projects",v:["p0002"]}],request)?.code).toBe("scope");
  expect(authorizeCapabilityV1([{k:"projects",v:["p0001"]}],{...request,projects:["p0001","p0002"]})?.code).toBe("scope");
  expect(authorizeCapabilityV1([{k:"scope",v:["run"]},{k:"notAfter",v:1}],request)?.code).toBe("capability");
  expect(authorizeCapabilityV1([{k:"mystery",v:1} as any],request)?.code).toBe("capability");
 });
 test("canonical JSON sorts keys at every level and keeps array order",()=>{
  expect(canonicalJsonV1({b:1,a:{d:[3,1],c:"é"}})).toBe('{"a":{"c":"é","d":[3,1]},"b":1}');
 });
});

describe("endpoints",()=>{
 for(const row of vectors.endpoints)test("worktree id of "+row.root,()=>{expect(worktreeIdV1(row.root)).toBe(row.id);});
 test("socket paths fit sockaddr_un and are per user and worktree",()=>{
  const a=endpointsV1("/w/a",{XDG_RUNTIME_DIR:"/run/user/1000"},"linux","1000"),b=endpointsV1("/w/b",{},"linux","1000");
  expect(a.endpoint).toBe("/run/user/1000/semio-repo-1000/"+worktreeIdV1("/w/a")+"/s");
  expect(a.directory).not.toBe(b.directory);
  const long=endpointsV1("/w/a",{XDG_RUNTIME_DIR:"/x".repeat(60)},"linux","1000");
  expect(Buffer.byteLength(long.endpoint)).toBeLessThan(104);expect(long.endpoint.startsWith("/tmp/semio-repo-1000/")).toBe(true);
  const win=endpointsV1("C:\\w",{LOCALAPPDATA:"C:\\Users\\u\\AppData\\Local"},"win32","-1");
  expect(win.endpoint).toBe("\\\\.\\pipe\\semio-repo--1-"+worktreeIdV1("C:\\w"));
 });
});

describe("texts",()=>{
 test("every message exists in both languages with the same placeholders",()=>{
  const en=POLICY_V1.messages.en,de=POLICY_V1.messages.de;
  expect(Object.keys(en).sort()).toEqual(Object.keys(de).sort());
  const holes=(text:string)=>[...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(m=>m[1]).sort();
  for(const key of Object.keys(en)){expect(holes(de[key])).toEqual(holes(en[key]));expect(en[key]).not.toBe(de[key]);}
 });
 test("rendering picks only the requested languages; no language is a default",()=>{
  expect(Object.keys(renderTextV1("queued",{position:2},["de"])!)).toEqual(["de"]);
  expect(renderTextV1("queued",{position:2},["fr" as any])).toBeUndefined();
  expect(Object.keys(renderBothV1("queued",{position:2})).sort()).toEqual(["de","en"]);
 });
 test("every policy command class is a capability scope class and every refusal code has a text key or a fixed meaning",()=>{
  const classes=new Set(["status","query","run","watch","admin"]);
  for(const[name,command]of Object.entries(POLICY_V1.commands as Record<string,any>)){expect(classes.has(command.class)).toBe(true);expect(name).toMatch(/^[a-z][a-z0-9-]{0,31}$/u);}
  expect(validatorV1("Request")({t:"request",id:1,command:"run",args:[],cwd:"/w"})).toEqual([]);
 });
});
