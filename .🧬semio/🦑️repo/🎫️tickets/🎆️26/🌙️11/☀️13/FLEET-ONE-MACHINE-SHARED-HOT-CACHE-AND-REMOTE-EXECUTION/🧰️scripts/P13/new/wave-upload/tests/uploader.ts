import {afterEach,expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import type {GenerationExchangeOutcomeV1} from "../../🌐️remote-exchange/🟦️.ts";
import {WAVE_UPLOAD_POLICY_V1 as policy,WAVE_UPLOAD_TEXTS_V1 as texts,createGenerationUploaderV1,type GenerationUploadSourceV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📡️ Contract: the uploader sends the newest generation at most once per interval, one upload at a time, remembers across a restart what it looked at, backs off after a failure, looks again at a generation the cache refused only after the retry time, and never lets a failure reach the caller. */
const directories:string[]=[];
afterEach(()=>{for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
const folder=()=>{const path=mkdtempSync(join(tmpdir(),"graph-uploader-"));directories.push(path);return path;};
const uploaded=(over:Partial<GenerationExchangeOutcomeV1>={}):GenerationExchangeOutcomeV1=>({status:"uploaded",generation:1,owners:100,documents:5104,sent:48,bytes:3*1048576,...over});
function rig(over:{outcome?:()=>Promise<GenerationExchangeOutcomeV1>|GenerationExchangeOutcomeV1;statePath?:string;may?:()=>Promise<boolean>;language?:"en"|"de"}={}){
 const calls:{generation:number;stale?:number;signal:AbortSignal}[]=[],lines:string[]=[],clock={at:1_800_000_000_000},world={generation:1 as number|undefined};
 const source:GenerationUploadSourceV1={pointer:async()=>world.generation,upload:async({stale,signal})=>{calls.push({generation:world.generation!,stale,signal});return(over.outcome??(()=>uploaded()))();}};
 const uploader=createGenerationUploaderV1({source,statePath:over.statePath,language:over.language??"en",say:line=>lines.push(line),now:()=>clock.at,may:over.may});
 return{calls,lines,clock,world,uploader,source};
}

test("many ticks while an upload runs make one upload, and the same generation is not sent again",async()=>{
 let release:(outcome:GenerationExchangeOutcomeV1)=>void=()=>undefined;
 const r=rig({outcome:()=>new Promise(resolve=>{release=resolve;})});
 for(let at=0;at<6;at++)r.uploader.tick();
 await Bun.sleep(10);expect(r.calls).toHaveLength(1);
 release(uploaded());await r.uploader.idle();
 r.clock.at+=policy.minimumIntervalMilliseconds*3;r.uploader.tick();await r.uploader.idle();
 expect(r.calls).toHaveLength(1);expect(r.lines).toHaveLength(1);
});

test("a wave inside the interval waits, the newest generation goes out after it, and only the stale count of that generation is passed",async()=>{
 const r=rig();
 r.uploader.wave({generation:1,done:48,total:200});r.uploader.tick();await r.uploader.idle();
 expect(r.calls.map(call=>[call.generation,call.stale])).toEqual([[1,152]]);
 r.world.generation=2;r.uploader.wave({generation:2,done:96,total:200});r.clock.at+=60_000;r.uploader.tick();await r.uploader.idle();
 expect(r.calls).toHaveLength(1);
 r.world.generation=3;r.clock.at+=policy.minimumIntervalMilliseconds;r.uploader.wave({generation:2,done:10,total:20});r.uploader.tick();await r.uploader.idle();
 expect(r.calls.map(call=>[call.generation,call.stale])).toEqual([[1,152],[3,undefined]]);
 expect(r.lines).toHaveLength(2);expect(r.lines[0]).toContain("152 projects still stale");expect(r.lines[1]).toContain("unknown projects still stale");
});

test("the memory of the last look survives a restart: the same generation is not sent again and the interval keeps running",async()=>{
 const path=join(folder(),"wave-upload.json"),first=rig({statePath:path});
 first.uploader.tick();await first.uploader.idle();expect(first.calls).toHaveLength(1);expect(existsSync(path)).toBe(true);
 const second=rig({statePath:path});second.clock.at=first.clock.at+10_000;
 second.uploader.tick();await second.uploader.idle();expect(second.calls).toHaveLength(0);
 second.world.generation=2;second.uploader.tick();await second.uploader.idle();expect(second.calls).toHaveLength(0);
 second.clock.at=first.clock.at+policy.minimumIntervalMilliseconds;second.uploader.tick();await second.uploader.idle();expect(second.calls.map(call=>call.generation)).toEqual([2]);
 writeFileGarbage(path);const third=rig({statePath:path});third.uploader.tick();await third.uploader.idle();expect(third.calls).toHaveLength(1);
 const on=JSON.parse(readFileSync(path,"utf8"));expect(on).toMatchObject({version:1,generation:1,uploaded:true});
});
function writeFileGarbage(path:string){require("node:fs").writeFileSync(path,"{broken");}

test("a failure backs off, never reaches the caller, and the same generation is tried again after the backoff",async()=>{
 let attempt=0;const r=rig({outcome:()=>{if(++attempt===1)throw new Error("connection lost");if(attempt===2)return{status:"failed",reason:"timeout"};return uploaded();}});
 r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(1);expect(r.lines[0]).toContain("failed (connection lost), next try in 120 s");
 r.clock.at+=30_000;r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(1);
 r.clock.at+=policy.failureBackoffMilliseconds;r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(2);expect(r.lines[1]).toContain("failed (timeout)");
 r.clock.at+=policy.failureBackoffMilliseconds;r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(3);expect(r.lines[2]).toContain("uploaded to the fleet cache as a delta, 48 of 5104 documents (3 MB)");
 const quiet=createGenerationUploaderV1({source:{pointer:async()=>1,upload:async()=>uploaded()},language:"en",say:()=>{throw new Error("log closed");}});
 quiet.tick();await quiet.idle();
});

test("a generation the cache refused is looked at again only after the retry time, a newer one after the interval",async()=>{
 const r=rig({outcome:()=>({status:"remote-better"})});
 r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(1);expect(r.lines[0]).toContain("the fleet cache holds a better one");
 r.clock.at+=policy.minimumIntervalMilliseconds;r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(1);
 r.clock.at+=policy.retryMilliseconds;r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(2);
 r.world.generation=9;r.clock.at+=policy.minimumIntervalMilliseconds;r.uploader.tick();await r.uploader.idle();expect(r.calls.map(call=>call.generation)).toEqual([1,1,9]);
});

test("nothing is read or sent while the machine may not, nothing without a sealed generation, and stop aborts an upload in flight",async()=>{
 let allowed=false;const r=rig({may:async()=>allowed});
 r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(0);
 allowed=true;r.world.generation=undefined;r.uploader.tick();await r.uploader.idle();expect(r.calls).toHaveLength(0);
 r.world.generation=4;
 const stuck=rig({outcome:()=>new Promise(()=>undefined)});stuck.uploader.tick();await Bun.sleep(10);
 expect(stuck.calls[0]!.signal.aborted).toBe(false);stuck.uploader.stop();expect(stuck.calls[0]!.signal.aborted).toBe(true);
 stuck.uploader.tick();
});

test("the lines come in English and German with the same placeholders, and the declared policy, state and texts are valid (checked with Ajv)",async()=>{
 const de=rig({language:"de"});de.uploader.wave({generation:1,done:2,total:7});de.uploader.tick();await de.uploader.idle();
 expect(de.lines[0]).toBe("graph-server: Generation 1 als Delta in den Flottencache hochgeladen, 48 von 5104 Dokumenten (3 MB) in 0 s, 5 Projekte noch veraltet");
 const names=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
 for(const key of Object.keys(texts.en) as (keyof typeof texts.en)[]){expect(names(texts.de[key])).toEqual(names(texts.en[key]));expect(texts.de[key]).not.toBe(texts.en[key]);}
 const ajv=new Ajv({strict:false});
 expect(ajv.validate({type:"object",required:["version","minimumIntervalMilliseconds","failureBackoffMilliseconds","retryMilliseconds","timeoutMilliseconds","minimumFreeMegabytes","stateFile"],additionalProperties:false,properties:{version:{const:1},minimumIntervalMilliseconds:{type:"integer",minimum:60000,maximum:900000},failureBackoffMilliseconds:{type:"integer",minimum:1000},retryMilliseconds:{type:"integer",minimum:60000},timeoutMilliseconds:{type:"integer",minimum:60000},minimumFreeMegabytes:{type:"integer",minimum:1024},stateFile:{type:"string",pattern:"^[a-z-]+\\.json$"}}},schema.definitions.Policy.const),JSON.stringify(ajv.errors)).toBe(true);
 const state=ajv.compile(schema.definitions.State),path=join(folder(),"s.json"),again=rig({statePath:path});again.uploader.tick();await again.uploader.idle();
 expect(state(JSON.parse(readFileSync(path,"utf8"))),JSON.stringify(state.errors)).toBe(true);
 expect(state({version:1,generation:-1,atMs:1,uploaded:true})).toBe(false);
});
