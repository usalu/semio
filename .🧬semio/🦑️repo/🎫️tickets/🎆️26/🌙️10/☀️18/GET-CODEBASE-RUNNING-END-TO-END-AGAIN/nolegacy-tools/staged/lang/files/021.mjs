import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{compareCodePointsV1}from"../🟨️.mjs";
import{decodeVerifiedScopeV1,encodeVerifiedScopeV1,readVerifiedStampV1}from"../🔭️coverage/🟨️.mjs";

/** 🧷️ The declared constants of the generation pin; the schema file is the single source. */
export const GENERATION_PIN_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
/** 🗣️ The bilingual refusals of a nested command; the schema file declares the table. */
export const GENERATION_PIN_MESSAGES_V1=Object.freeze({en:Object.freeze({...schema.definitions.Messages.const.en}),de:Object.freeze({...schema.definitions.Messages.const.de})});
const P=GENERATION_PIN_POLICY_V1,HEX=/^[a-f0-9]{64}$/u,isObject=value=>value!==null&&typeof value==="object"&&!Array.isArray(value),listed=names=>names.length>5?names.slice(0,5).join(", ")+", …":names.join(", ");

/** 💬️ Words one refusal in one language. */
export function renderGenerationPinMessageV1(key,fields,language){
 const text=GENERATION_PIN_MESSAGES_V1[language]?.[key];if(text===undefined)throw Error("generation-pin-message-undeclared:"+key);
 return text.replace(/\{(\w+)\}/gu,(_,name)=>String(fields[name]??""));
}

/** 🚫️ The refusal of a nested command whose pinned generation cannot serve it: one error carrying both languages, never a replacement graph; its machine fields stay out of the printed error. */
export function generationPinRefusalV1(key,fields={}){
 return Object.defineProperties(Error(renderGenerationPinMessageV1(key,fields,"en")+"\n"+renderGenerationPinMessageV1(key,fields,"de")),{code:{value:"GENERATION_PIN_REFUSED"},reason:{value:key},fields:{value:Object.freeze({...fields})}});
}

/** 🔏️ Encodes the pin of a generation the Original served: the owner invocation, the manifest and the owners (`covered`, every owner when undefined) of the manifest it served. */
export function encodeGenerationPinV1({owner,generation,manifest,owners,covered,atMs}){
 if(!Number.isSafeInteger(owner)||owner<1||!Number.isSafeInteger(generation)||generation<0||typeof manifest!=="string"||!HEX.test(manifest)||!Array.isArray(owners))throw generationPinRefusalV1("inputInvalid");
 const stamp=readVerifiedStampV1({atMs,...encodeVerifiedScopeV1(owners,covered===undefined?new Set(owners):covered)}),text=JSON.stringify({version:1,owner,generation,manifest,covered:{atMs:stamp.atMs,count:stamp.count,owners:stamp.owners}});
 if(Buffer.byteLength(text)>P.maximumBytes)throw generationPinRefusalV1("unbounded",{maximum:P.maximumBytes});
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

/** 🧭️ Whether the environment holds a pin this command is bound by: `none` without a pin, `bound` while the owner is one of the invocations the command answers to, alive, not older than `maximumAgeMilliseconds` and not this process itself (the Original reads its own loads fresh; only its descendants are served from its pin), `foreign` otherwise (an orphan, a daemon that cleared its invocation, a pin of another tree); a malformed pin refuses. */
export function generationPinStateV1(environment,{alive=processAlive,self=process.pid,now=Date.now()}={}){
 const text=environment[P.environment];if(text===undefined||text==="")return{state:"none"};
 const pin=readGenerationPinV1(text),owners=[environment[P.invocationEnvironment],...(environment[P.chainEnvironment]??"").split(",")].map(Number);
 return{state:pin.owner!==self&&now-pin.covered.atMs<=P.maximumAgeMilliseconds&&owners.includes(pin.owner)&&alive(pin.owner)?"bound":"foreign",pin};
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
