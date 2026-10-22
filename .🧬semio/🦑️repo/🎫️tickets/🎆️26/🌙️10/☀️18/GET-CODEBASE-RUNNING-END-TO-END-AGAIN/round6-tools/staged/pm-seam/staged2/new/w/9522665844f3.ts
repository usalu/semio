import type {ManagerDescriptorV1} from "../🟦️.ts";

/** 🔭️ The named values a schema base may refer to; derived from one descriptor and nothing else. */
export type SchemaViewV1=Readonly<Record<string,unknown>>;

const tokenV1=/\{\{([A-Za-z0-9_.]+)(?:\|(re|lower))?\}\}/gu;
const escapePatternV1=(text:string):string=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
const isExpressionV1=(node:unknown):node is Readonly<{$manager:string;extend?:unknown}>=>node!==null&&typeof node==="object"&&!Array.isArray(node)&&"$manager" in node;

/** 🪞️ Derives the schema view of one manager: its descriptor plus the few values every schema base needs. */
export function schemaViewV1(descriptor:ManagerDescriptorV1):SchemaViewV1{
 const primary=descriptor.locks.find(lock=>lock.role==="primary")!,recipe=descriptor.recipe as Readonly<Record<string,unknown>>,inputs=descriptor.producerInputs;
 const stripped=(name:string)=>name.replace(/^\./u,"").replace(/\.[A-Za-z0-9]+$/u,"");
 return Object.freeze({
  ...descriptor,lockFile:primary.name,scopeConfig:descriptor.configurations.scope[0]??"",
  storeRoleFiles:{[String(recipe.home)]:inputs.ambientHomeFiles,[String(recipe.config)]:inputs.ambientConfigFiles},
  producerFileLabel:[...new Set([...inputs.ambientHomeFiles,...inputs.ambientConfigFiles].map(stripped))].join(" and "),
  resolutionOracles:[descriptor.resolution.oracle,"typescript","node","node-missing","none"],
  managerProtocol:{manager:descriptor.id,label:descriptor.label,lockFile:primary.name,scopeConfig:descriptor.configurations.scope[0]??"",generationDirectory:descriptor.generationDirectory,strictProfilePrefix:descriptor.resolution.strictPrefix??"",hostVersionKey:descriptor.runtime.hostVersionKey??"",cacheVariable:Object.entries(descriptor.environment.set).find(([name,spec])=>spec==="role:cache"&&name!=="XDG_CACHE_HOME")?.[0]??""},
 });
}

function lookupV1(view:SchemaViewV1,path:string):unknown{
 let value:unknown=view;
 for(const key of path.split(".")){
  if(value===null||typeof value!=="object"||!(key in (value as object)))throw Error("Declared manager value required: "+path);
  value=(value as Record<string,unknown>)[key];
 }
 return value;
}

/** 🧱️ Materializes a neutral schema base for one manager: `{{path}}` fills strings (`|re` escapes for patterns, `|lower` lowercases), `{"$manager":path,"extend":…}` replaces a node by a declared value extended by an array or an object. */
export function materializeSchemaV1(base:unknown,view:SchemaViewV1):unknown{
 const walk=(node:unknown):unknown=>{
  if(typeof node==="string")return node.replace(tokenV1,(_,path:string,mode?:string)=>{
   const value=lookupV1(view,path);
   if(typeof value!=="string")throw Error("Declared manager text required: "+path);
   return mode==="re"?escapePatternV1(value):mode==="lower"?value.toLowerCase():value;
  });
  if(Array.isArray(node))return node.map(walk);
  if(isExpressionV1(node)){
   const value=structuredClone(lookupV1(view,node.$manager));
   if(node.extend===undefined)return value;
   if(Array.isArray(value)&&Array.isArray(node.extend))return [...value,...node.extend.map(walk)];
   if(value!==null&&typeof value==="object"&&node.extend!==null&&typeof node.extend==="object")return {...value,...(walk(node.extend) as object)};
   throw Error("Extendable manager value required: "+node.$manager);
  }
  if(node!==null&&typeof node==="object")return Object.fromEntries(Object.entries(node).map(([key,value])=>[key,walk(value)]));
  return node;
 };
 return walk(base);
}
