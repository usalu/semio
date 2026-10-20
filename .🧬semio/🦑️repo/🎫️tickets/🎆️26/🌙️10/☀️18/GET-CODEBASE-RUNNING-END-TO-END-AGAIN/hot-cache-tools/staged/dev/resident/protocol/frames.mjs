/** 📨 Frame codec and the schema interpreter that makes `schema.json` the runtime authority for every frame. */

/** 🚨 A protocol violation with a machine code the service maps to a refusal. */
export class ProtocolErrorV1 extends Error {
 constructor(code,message){super(message);this.name="ProtocolErrorV1";this.code=code;}
}

/** 📤 Encodes one frame as a single JSON line. */
export function encodeFrameV1(frame){return JSON.stringify(frame)+"\n";}

/** 📥 Splits a byte stream into JSON lines; a line above the bound or invalid JSON is reported once and ends the stream. */
export function createFrameDecoderV1({maximumFrameBytes,onFrame,onError}){
 let pending=[],pendingBytes=0,failed=false;
 const fail=(code,message)=>{if(!failed){failed=true;pending=[];pendingBytes=0;onError(new ProtocolErrorV1(code,message));}};
 const line=bytes=>{
  if(bytes.length===0)return;
  let value;
  try{value=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes));}catch{fail("invalid","frame is not valid UTF-8 JSON");return;}
  onFrame(value);
 };
 return{
  push(chunk){
   if(failed)return;
   const bytes=typeof chunk==="string"?Buffer.from(chunk,"utf8"):Buffer.isBuffer(chunk)?chunk:Buffer.from(chunk);
   let start=0;
   for(;;){
    const at=bytes.indexOf(10,start);
    if(at<0){
     const rest=bytes.subarray(start);
     if(pendingBytes+rest.length>maximumFrameBytes){fail("invalid","frame exceeds "+maximumFrameBytes+" bytes");return;}
     if(rest.length){pending.push(rest);pendingBytes+=rest.length;}
     return;
    }
    const tail=bytes.subarray(start,at);
    if(pendingBytes+tail.length>maximumFrameBytes){fail("invalid","frame exceeds "+maximumFrameBytes+" bytes");return;}
    const whole=pending.length?Buffer.concat([...pending,tail]):tail;
    pending=[];pendingBytes=0;
    line(whole);
    if(failed)return;
    start=at+1;
   }
  },
  end(){if(!failed&&pendingBytes>0)fail("invalid","stream ended inside a frame");},
 };
}

const TYPE_CHECK={
 object:value=>value!==null&&typeof value==="object"&&!Array.isArray(value),
 array:Array.isArray,
 string:value=>typeof value==="string",
 integer:Number.isInteger,
 number:value=>typeof value==="number"&&Number.isFinite(value),
 boolean:value=>typeof value==="boolean",
 null:value=>value===null,
};

function same(left,right){return JSON.stringify(left)===JSON.stringify(right);}

/** 🧪 Compiles the JSON Schema subset the protocol uses (draft-07 keywords, `#/definitions` references) into a function returning error strings. */
export function compileSchemaV1(document,name){
 const definitions=document.definitions;
 const resolve=reference=>{
  const match=/^#\/definitions\/([A-Za-z0-9]+)$/u.exec(reference);
  if(!match||!definitions[match[1]])throw new Error("Unresolved schema reference "+reference);
  return definitions[match[1]];
 };
 const regexes=new Map();
 const regex=pattern=>{let value=regexes.get(pattern);if(!value){value=new RegExp(pattern,"u");regexes.set(pattern,value);}return value;};
 const discriminant=schema=>{
  const target=schema.$ref?resolve(schema.$ref):schema;
  const constant=target?.properties?.t?.const;
  return typeof constant==="string"?constant:undefined;
 };
 const check=(schema,value,path,errors)=>{
  if(schema.$ref){check(resolve(schema.$ref),value,path,errors);return;}
  if("const" in schema&&!same(schema.const,value)){errors.push(path+": must equal "+JSON.stringify(schema.const).slice(0,60));return;}
  if(schema.enum&&!schema.enum.some(item=>same(item,value))){errors.push(path+": must be one of "+schema.enum.join("|"));return;}
  if(schema.type!==undefined){
   const types=Array.isArray(schema.type)?schema.type:[schema.type];
   if(!types.some(type=>TYPE_CHECK[type](value))){errors.push(path+": must be "+types.join("|"));return;}
  }
  if(typeof value==="string"){
   if(schema.minLength!==undefined&&[...value].length<schema.minLength)errors.push(path+": shorter than "+schema.minLength);
   if(schema.maxLength!==undefined&&[...value].length>schema.maxLength)errors.push(path+": longer than "+schema.maxLength);
   if(schema.pattern!==undefined&&!regex(schema.pattern).test(value))errors.push(path+": does not match "+schema.pattern);
  }
  if(typeof value==="number"){
   if(schema.minimum!==undefined&&value<schema.minimum)errors.push(path+": below "+schema.minimum);
   if(schema.maximum!==undefined&&value>schema.maximum)errors.push(path+": above "+schema.maximum);
  }
  if(Array.isArray(value)){
   if(schema.minItems!==undefined&&value.length<schema.minItems)errors.push(path+": fewer than "+schema.minItems+" items");
   if(schema.maxItems!==undefined&&value.length>schema.maxItems)errors.push(path+": more than "+schema.maxItems+" items");
   if(schema.uniqueItems&&new Set(value.map(item=>JSON.stringify(item))).size!==value.length)errors.push(path+": items must be unique");
   if(Array.isArray(schema.items)){for(let at=0;at<Math.min(schema.items.length,value.length);at++)check(schema.items[at],value[at],path+"/"+at,errors);}
   else if(schema.items){for(let at=0;at<value.length;at++)check(schema.items,value[at],path+"/"+at,errors);}
  }
  if(TYPE_CHECK.object(value)){
   const keys=Object.keys(value);
   if(schema.minProperties!==undefined&&keys.length<schema.minProperties)errors.push(path+": fewer than "+schema.minProperties+" properties");
   if(schema.maxProperties!==undefined&&keys.length>schema.maxProperties)errors.push(path+": more than "+schema.maxProperties+" properties");
   for(const key of schema.required??[])if(!(key in value))errors.push(path+": missing "+key);
   if(schema.propertyNames)for(const key of keys)check(schema.propertyNames,key,path+"/"+key+"(name)",errors);
   const properties=schema.properties??{};
   for(const key of keys){
    if(key in properties)check(properties[key],value[key],path+"/"+key,errors);
    else if(schema.additionalProperties===false)errors.push(path+": unexpected "+key);
    else if(schema.additionalProperties&&typeof schema.additionalProperties==="object")check(schema.additionalProperties,value[key],path+"/"+key,errors);
   }
  }
  if(schema.not){const inner=[];check(schema.not,value,path,inner);if(inner.length===0)errors.push(path+": matches a forbidden shape");}
  if(schema.oneOf){
   let branches=schema.oneOf;
   if(TYPE_CHECK.object(value)&&typeof value.t==="string"&&branches.every(branch=>discriminant(branch)!==undefined)){const direct=branches.filter(branch=>discriminant(branch)===value.t);if(direct.length)branches=direct;else{errors.push(path+": unknown frame type "+value.t.slice(0,40));return;}}
   let matches=0,first;
   for(const branch of branches){const inner=[];check(branch,value,path,inner);if(inner.length===0)matches++;else first??=inner;}
   if(matches!==1)errors.push(...(matches===0?first??[path+": matches no alternative"]:[path+": matches several alternatives"]));
  }
 };
 const root=name?resolve("#/definitions/"+name):document;
 return value=>{const errors=[];check(root,value,"",errors);return errors;};
}
