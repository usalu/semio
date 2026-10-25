import json, os, sys
ROOT = "/Users/ueli/Documents/semio"
G = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"
PIN = G + "/🧷️pin"
MJS, DMTS, SCHEMA, TESTS = "🟨️.mjs", "🟨️.d.mts", "🧬️schema", "🧪️tests"
JSONF = "🔣️.json"
os.makedirs(PIN + "/" + SCHEMA, exist_ok=True)

MESSAGES = {
 "en": {
  "uncovered": "graph: this command runs inside a build whose pinned graph generation {generation} does not cover {count} of the projects it needs ({names}); it does not plan a graph of its own: add them to the selection of the invoking build or run the command outside of it",
  "retired": "graph: the graph generation {generation} pinned by the invoking build is no longer kept; this command does not plan a graph of its own: run the invoking build again",
  "identity": "graph: the graph generation {generation} pinned by the invoking build belongs to another repository identity; this command does not plan a graph of its own",
  "malformed": "graph: the generation pin of the invoking build is not a closed pin ({reason}); this command does not plan a graph of its own",
 },
 "de": {
  "uncovered": "Graph: dieser Befehl läuft in einem Build, dessen angeheftete Graph-Generation {generation} {count} der benötigten Projekte nicht abdeckt ({names}); er plant keinen eigenen Graphen: die Projekte in die Auswahl des aufrufenden Builds aufnehmen oder den Befehl außerhalb davon ausführen",
  "retired": "Graph: die vom aufrufenden Build angeheftete Graph-Generation {generation} wird nicht mehr aufbewahrt; dieser Befehl plant keinen eigenen Graphen: den aufrufenden Build erneut ausführen",
  "identity": "Graph: die vom aufrufenden Build angeheftete Graph-Generation {generation} gehört zu einer anderen Repository-Identität; dieser Befehl plant keinen eigenen Graphen",
  "malformed": "Graph: die Generations-Anheftung des aufrufenden Builds ist keine geschlossene Anheftung ({reason}); dieser Befehl plant keinen eigenen Graphen",
 },
}

schema = {
 "$schema": "https://json-schema.org/draft/2020-12/schema",
 "title": "Generation pin of a nested build command",
 "description": "A nested `repo run` inside an orchestrated action of an Original run consumes the graph generation the Original already loaded: the Original names it in the environment variable `environment` as one closed JSON document (the pin) and every descendant inherits it with the rest of the environment. The pin names the sealed generation by the digest of its manifest, the invocation that owns it (`owner`, the pid the descendants carry in `invocationEnvironment` or in the chain `chainEnvironment`) and, as a verified stamp over the owners of that manifest, exactly the projects the Original served from it. A nested command binds the pin only while `owner` is one of the invocations it answers to and that process is alive; any other pin is not its own and is ignored. A bound pin is the graph of the command: it reads that manifest by digest, never verifies the live tree, never takes the builder custody and never plans; when the generation was retired, belongs to another repository identity or lacks an owner of the closure the command needs, the command refuses in both languages instead of planning a graph of its own.",
 "definitions": {
  "Policy": {"const": {"version": 1, "environment": "SEMIO_BUILD_GENERATION_PIN", "invocationEnvironment": "SEMIO_BUILD_INVOCATION_PID", "chainEnvironment": "SEMIO_BUILD_INVOCATION_PIDS", "maximumBytes": 32768}},
  "Messages": {"const": MESSAGES},
  "Pin": {
   "type": "object", "additionalProperties": False,
   "required": ["version", "owner", "generation", "manifest", "covered"],
   "properties": {
    "version": {"const": 1},
    "owner": {"type": "integer", "minimum": 1, "maximum": 9007199254740991},
    "generation": {"type": "integer", "minimum": 0, "maximum": 9007199254740991},
    "manifest": {"type": "string", "pattern": "^[a-f0-9]{64}$"},
    "covered": {"type": "object", "additionalProperties": False, "required": ["atMs", "count", "owners"], "properties": {"atMs": {"type": "integer", "minimum": 0}, "count": {"type": "integer", "minimum": 0, "maximum": 65536}, "owners": {"type": "string"}}},
   },
  },
 },
}
open(PIN + "/" + SCHEMA + "/" + JSONF, "w").write(json.dumps(schema, ensure_ascii=False, indent=1) + "\n")

module = r'''import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{compareCodePointsV1}from"../🟨️.mjs";
import{decodeVerifiedScopeV1,encodeVerifiedScopeV1,readVerifiedStampV1}from"../🔭️coverage/🟨️.mjs";

/** 🧷️ The declared constants of the generation pin; the schema file is the single source. */
export const GENERATION_PIN_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
/** 🗣️ The bilingual refusals of a nested command; the schema file declares the table. */
export const GENERATION_PIN_MESSAGES_V1=Object.freeze({en:Object.freeze({...schema.definitions.Messages.const.en}),de:Object.freeze({...schema.definitions.Messages.const.de})});
const P=GENERATION_PIN_POLICY_V1,HEX=/^[a-f0-9]{64}$/u,isObject=value=>value!==null&&typeof value==="object"&&!Array.isArray(value),listed=names=>names.length>5?names.slice(0,5).join(", ")+", …":names.join(", ");

/** 🗣️ Words one refusal in one language. */
export function renderGenerationPinMessageV1(key,fields,language){
 const text=GENERATION_PIN_MESSAGES_V1[language]?.[key];if(text===undefined)throw Error("Known generation pin message required: "+key);
 return text.replace(/\{(\w+)\}/gu,(_,name)=>String(fields[name]??""));
}

/** 🚫️ The refusal of a nested command whose pinned generation cannot serve it: one error carrying both languages, never a replacement graph. */
export function generationPinRefusalV1(key,fields={}){
 return Object.assign(Error(renderGenerationPinMessageV1(key,fields,"en")+"\n"+renderGenerationPinMessageV1(key,fields,"de")),{code:"GENERATION_PIN_REFUSED",reason:key,fields});
}

/** 🔏️ Encodes the pin of a generation the Original served: the owner invocation, the manifest and the owners (`covered`, every owner when undefined) of the manifest it served. */
export function encodeGenerationPinV1({owner,generation,manifest,owners,covered,atMs}){
 if(!Number.isSafeInteger(owner)||owner<1||!Number.isSafeInteger(generation)||generation<0||typeof manifest!=="string"||!HEX.test(manifest)||!Array.isArray(owners))throw Error("Closed generation pin input required");
 const stamp=readVerifiedStampV1({atMs,...encodeVerifiedScopeV1(owners,covered===undefined?new Set(owners):covered)}),text=JSON.stringify({version:1,owner,generation,manifest,covered:{atMs:stamp.atMs,count:stamp.count,owners:stamp.owners}});
 if(Buffer.byteLength(text)>P.maximumBytes)throw Error("Bounded generation pin required");
 return text;
}

/** 🔍️ Admits one closed pin. */
export function readGenerationPinV1(text){
 const malformed=reason=>generationPinRefusalV1("malformed",{reason});
 if(typeof text!=="string"||Buffer.byteLength(text)>P.maximumBytes)throw malformed("size");
 let value;try{value=JSON.parse(text);}catch{throw malformed("json");}
 if(!isObject(value))throw malformed("object");
 for(const key of Object.keys(value))if(!["version","owner","generation","manifest","covered"].includes(key))throw malformed("unknown "+key);
 for(const key of["version","owner","generation","manifest","covered"])if(!Object.hasOwn(value,key))throw malformed("missing "+key);
 if(value.version!==1)throw malformed("version");
 if(!Number.isSafeInteger(value.owner)||value.owner<1)throw malformed("owner");
 if(!Number.isSafeInteger(value.generation)||value.generation<0)throw malformed("generation");
 if(typeof value.manifest!=="string"||!HEX.test(value.manifest))throw malformed("manifest");
 let covered;try{covered=readVerifiedStampV1(value.covered);}catch(error){throw malformed(String(error.message));}
 return Object.freeze({version:1,owner:value.owner,generation:value.generation,manifest:value.manifest,covered});
}

/** 🧑‍🤝‍🧑 The invocation pid a command answers to: the outermost build invocation it was started under, itself when it is the outermost. */
export function generationPinOwnerV1(environment,pid=process.pid){
 const value=Number(environment[P.invocationEnvironment]);return Number.isSafeInteger(value)&&value>0?value:pid;
}

const processAlive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code==="EPERM";}};

/** 🧭️ Whether the environment holds a pin this command is bound by: `none` without a pin, `bound` while the owner is one of the invocations the command answers to and alive, `foreign` otherwise (an orphan, a daemon that cleared its invocation, a pin of another tree); a malformed pin refuses. */
export function generationPinStateV1(environment,{alive=processAlive}={}){
 const text=environment[P.environment];if(text===undefined||text==="")return{state:"none"};
 const pin=readGenerationPinV1(text),owners=[environment[P.invocationEnvironment],...(environment[P.chainEnvironment]??"").split(",")].map(Number);
 return{state:owners.includes(pin.owner)&&alive(pin.owner)?"bound":"foreign",pin};
}

/** ➖️ The owners of `needed` (every owner of the manifest when undefined) the pin does not cover, in code point order. */
export function generationPinOpenV1({pin,owners,needed}){
 let covered;try{covered=decodeVerifiedScopeV1(owners,pin.covered);}catch{throw generationPinRefusalV1("malformed",{reason:"owners"});}
 return[...(needed===undefined?owners:needed)].filter(name=>!covered.has(name)).sort(compareCodePointsV1);
}

/** 📌️ Serves a request from exactly the generation a pin names: the manifest is read by its digest, the closure is collected inside it and every owner of that closure must be covered; nothing is verified against the live tree, built or planned. */
export async function readPinnedGenerationV1({engine,pin,identity,roots,closure,signal}){
 const manifest=await engine.readManifestAt(pin.manifest,signal);
 if(manifest===undefined)throw generationPinRefusalV1("retired",{generation:pin.generation});
 if(manifest.generation!==pin.generation||manifest.identity!==identity)throw generationPinRefusalV1("identity",{generation:pin.generation});
 const current=Object.freeze({pointer:Object.freeze({version:1,generation:manifest.generation,manifest:pin.manifest,parent:manifest.parent}),manifest}),owners=Object.keys(manifest.owners).sort(compareCodePointsV1);
 let names;
 if(roots!==undefined||closure!==undefined){
  try{names=closure?await closure(current,{signal}):await engine.closureOf(current,roots,{signal});}
  catch(error){
   if(error?.code==="GENERATION_UNKNOWN_OWNER"){const unknown=String(error.message).replace(/^Unknown project in the closure: /u,"");throw generationPinRefusalV1("uncovered",{generation:pin.generation,count:1,names:unknown});}
   if(error?.code==="GENERATION_INCOMPLETE")throw generationPinRefusalV1("retired",{generation:pin.generation});
   throw error;
  }
 }
 const open=generationPinOpenV1({pin,owners,needed:names});
 if(open.length)throw generationPinRefusalV1("uncovered",{generation:pin.generation,count:open.length,names:listed(open)});
 return{current,served:"pinned",built:false,published:false,computed:0,...(names?{names}:{})};
}
'''
open(PIN + "/" + MJS, "w").write(module)

dts = '''/** 🧷️ The declared constants of the generation pin. */
export interface GenerationPinPolicyV1{readonly version:1;readonly environment:"SEMIO_BUILD_GENERATION_PIN";readonly invocationEnvironment:"SEMIO_BUILD_INVOCATION_PID";readonly chainEnvironment:"SEMIO_BUILD_INVOCATION_PIDS";readonly maximumBytes:number}
export const GENERATION_PIN_POLICY_V1:GenerationPinPolicyV1;
export type GenerationPinMessageKeyV1="uncovered"|"retired"|"identity"|"malformed";
export const GENERATION_PIN_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<GenerationPinMessageKeyV1,string>>>>;
export function renderGenerationPinMessageV1(key:GenerationPinMessageKeyV1,fields:Readonly<Record<string,string|number>>,language:"en"|"de"):string;
/** 🚫️ The refusal of a nested command whose pinned generation cannot serve it. */
export interface GenerationPinRefusalV1 extends Error{readonly code:"GENERATION_PIN_REFUSED";readonly reason:GenerationPinMessageKeyV1;readonly fields:Readonly<Record<string,string|number>>}
export function generationPinRefusalV1(key:GenerationPinMessageKeyV1,fields?:Readonly<Record<string,string|number>>):GenerationPinRefusalV1;
/** 📌️ A closed pin: the owning invocation, the sealed manifest and the verified stamp of the owners it served. */
export interface GenerationPinV1{readonly version:1;readonly owner:number;readonly generation:number;readonly manifest:string;readonly covered:{readonly atMs:number;readonly count:number;readonly owners:string}}
export function encodeGenerationPinV1(input:{owner:number;generation:number;manifest:string;owners:readonly string[];covered?:ReadonlySet<string>;atMs:number}):string;
export function readGenerationPinV1(text:string):GenerationPinV1;
export function generationPinOwnerV1(environment:Readonly<Record<string,string|undefined>>,pid?:number):number;
export type GenerationPinStateV1={readonly state:"none"}|{readonly state:"bound"|"foreign";readonly pin:GenerationPinV1};
export function generationPinStateV1(environment:Readonly<Record<string,string|undefined>>,options?:{alive?:(pid:number)=>boolean}):GenerationPinStateV1;
export function generationPinOpenV1(input:{pin:GenerationPinV1;owners:readonly string[];needed?:Iterable<string>}):string[];
export function readPinnedGenerationV1(input:{engine:{readManifestAt(digest:string,signal?:AbortSignal):Promise<unknown>;closureOf(current:unknown,roots:readonly string[],options?:{signal?:AbortSignal}):Promise<Set<string>>};pin:GenerationPinV1;identity:string;roots?:readonly string[];closure?:(current:unknown,options?:{signal?:AbortSignal})=>Promise<Set<string>>;signal?:AbortSignal}):Promise<{current:{readonly pointer:{readonly version:1;readonly generation:number;readonly manifest:string;readonly parent:string|null};readonly manifest:any};served:"pinned";built:false;published:false;computed:0;names?:Set<string>}>;
'''
open(PIN + "/" + DMTS, "w").write(dts)
print("ok")
