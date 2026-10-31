const fs=require("node:fs"),path=require("node:path"),crypto=require("node:crypto"),assert=require("node:assert/strict");
const {visit,convert}=require("./schema-definitions-proof.cjs"),Ajv07=require("ajv").default,Ajv2020=require("ajv/dist/2020").default,{compileJsonSchemaSubset}=require(path.resolve("🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
const directory=__dirname,inventory=JSON.parse(fs.readFileSync(path.join(directory,"🗑️generated/schema-definitions-inventory.json"),"utf8")),options={strict:false,validateFormats:false,logger:false},hash=bytes=>crypto.createHash("sha256").update(bytes).digest("hex");
const strings=["","a","ab","x","0","1","2","10","A","A_B","Pset_A","a/b","/a","/entities/a","C:\\a","a.xml",".","./a",".stage-"+"a".repeat(12),".workspace-events/segments/aaaaaaaa/a.retained","2026-10-29","2026-10-29T01:02:03Z","#ffffff","char:a","paste:a","a".repeat(8),"a".repeat(16),"a".repeat(40),"a".repeat(64),"0".repeat(64),"s00","a.sock","a".repeat(16)+".sock","/tmp/semio-dashboard-1","https://example.com/a","http://localhost:1234/","26/10/29/ABC","a@b","a>b","a:b>c","a:b","@a/b","./a.go","a.go","a.rs","a/🦀️.rs","semio-a-artifact-b","file:///a","canonical.json","no-cross-package-relative-a","private","pub","repo/breach/a","en","SEMIO_A","1.2.3","a.zip","repo run repo:test","repo ./📜️script.ts test a","repo://session/a","1.1","registry-invalid:a"];
const primitive=[null,false,true,0,1,-1,2,0.5,"",{},[],...strings];
const annotations=new Set(["$schema","$id","$comment","title","description","default","examples","readOnly","writeOnly","deprecated","$defs","definitions"]);
function unique(values){const seen=new Set();return values.filter(value=>{if(value===undefined)return false;const key=JSON.stringify(value);if(seen.has(key))return false;seen.add(key);return true;});}
function mutants(value,depth=0){
 const values=[null,false,true,0,-1,"",[],{}];
 if(typeof value==="string")values.push(value+"!",value.slice(1));
 if(typeof value==="number")values.push(value-1,value+1);
 if(Array.isArray(value)){values.push([],value.slice(1),[...value,value[0]??null]);if(depth<2)for(let i=0;i<Math.min(value.length,4);i++)for(const child of mutants(value[i],depth+1).slice(0,6)){const copy=[...value];copy[i]=child;values.push(copy);}}
 else if(value&&typeof value==="object"){values.push({...value,__unexpected__:true});for(const key of Object.keys(value)){const copy={...value};delete copy[key];values.push(copy);if(depth<2)for(const child of mutants(value[key],depth+1).slice(0,6))values.push({...value,[key]:child});}}
 return values;
}
const results=[];
for(const candidate of inventory.results.filter(row=>row.eligible)){
 const row={path:candidate.path,sha256:candidate.sha256,eligible:false};
 try{
  const bytes=fs.readFileSync(candidate.path);assert.equal(hash(bytes),candidate.sha256,"Current exact Source required");const source=JSON.parse(bytes),next=convert(source),nodes=visit(source),nextNodes=visit(next),old=new Ajv2020(options),after=new Ajv07(options);
  assert.equal(old.validateSchema(source),true,JSON.stringify(old.errors));assert.equal(after.validateSchema(next),true,JSON.stringify(after.errors));old.addSchema(source,"proof");after.addSchema(next,"proof");
  const locations=new Map(nodes.map(({pointer,node})=>[node,pointer])),checks=new Map(),memo=new Map();
  const check=node=>{if(typeof node==="boolean")return ()=>node;const location=locations.get(node);if(location===undefined)throw Error("Schema location required");if(!checks.has(node))checks.set(node,old.getSchema("proof#"+location));return checks.get(node);};
  const resolve=ref=>ref==="#"?source:ref.slice(2).split("/").reduce((n,k)=>n[k.replaceAll("~1","/").replaceAll("~0","~")],source);
  function witnesses(node,depth=0,seen=new Set()){
   if(node===false||depth>14||seen.has(node))return [];if(node===true)return [null];if(memo.has(node))return memo.get(node);const nextSeen=new Set(seen);nextSeen.add(node);let values=[];
   if(Object.hasOwn(node,"const"))values.push(node.const);if(node.enum)values.push(...node.enum);
   if(Object.hasOwn(node,"default"))values.push(node.default);if(node.examples)values.push(...node.examples);
   if(node.$ref)values.push(...witnesses(resolve(node.$ref),depth+1,nextSeen));
   for(const child of [...node.anyOf??[],...node.oneOf??[]])values.push(...witnesses(child,depth+1,nextSeen));
   if(node.allOf){const candidates=node.allOf.map(child=>witnesses(child,depth+1,nextSeen));if(candidates.every(v=>v.length))values.push(Object.assign({},...candidates.map(v=>v[0])));values.push(...candidates.flat());}
   const types=Array.isArray(node.type)?node.type:[node.type??(node.properties||node.required?"object":node.items?"array":undefined)];
   for(const type of types){
    if(type==="object"){
     const props=node.properties??{},make=all=>{const value={};for(const key of all?Object.keys(props):node.required??[]){const child=props[key]??node.additionalProperties??true,found=witnesses(child,depth+1,nextSeen)[0];if(found!==undefined)Object.defineProperty(value,key,{value:found,enumerable:true,writable:true,configurable:true});}for(let index=0;Object.keys(value).length<(node.minProperties??0)&&index<16;index++){const key=["a","b","c","d"][index]??"key"+index;if(node.propertyNames&&!check(node.propertyNames)(key))continue;const found=witnesses(node.additionalProperties??true,depth+1,nextSeen)[0];if(found!==undefined)value[key]=found;}return value;};
     const bases=[make(false),make(true)];values.push(...bases);for(const base of bases)for(const[key,child]of Object.entries(props))for(const alternative of witnesses(child,depth+1,nextSeen).slice(0,8))values.push({...base,[key]:alternative});
    }else if(type==="array"){
     const children=witnesses(node.items??true,depth+1,nextSeen),length=Math.min(2048,node.minItems??0);values.push([]);if(length===0||children.length){values.push(Array.from({length},(_,i)=>children[i%children.length]??null));for(const child of children.slice(0,8))values.push(Array.from({length:Math.max(1,length)},()=>child));}
    }else if(type==="string"){
     const length=Math.min(1024,node.minLength??0);values.push(...strings,"a".repeat(length),"0".repeat(length));
    }else if(type==="number"||type==="integer"){
     let low=node.minimum??0;if(node.exclusiveMinimum!==undefined)low=Math.max(low,node.exclusiveMinimum+(type==="integer"?1:0.5));if(type==="integer")low=Math.ceil(low);if(node.multipleOf)low=Math.ceil(low/node.multipleOf)*node.multipleOf;values.push(low,0,1,-1,...[node.minimum,node.maximum,node.exclusiveMinimum,node.exclusiveMaximum].filter(v=>typeof v==="number"));
    }else if(type==="boolean")values.push(false,true);else if(type==="null")values.push(null);else values.push(...primitive);
   }
   const valid=unique(values).filter(value=>check(node)(value)).slice(0,32);if(valid.length)memo.set(node,valid);return valid;
  }
  const targets=nodes.filter(({pointer})=>pointer===""||/\/(?:\$defs|definitions)\/[^/]+$/.test(pointer)),coverage=[];
  for(const target of targets){
   const currentPointer=target.pointer,a=check(target.node),b=after.getSchema("proof#"+currentPointer),currentNode=nextNodes.find(row=>row.pointer===currentPointer)?.node,owned=compileJsonSchemaSubset(currentNode,next);
   const positive=witnesses(target.node),samples=unique([...primitive,...positive,...positive.flatMap(v=>mutants(v))]);let valid=0,invalid=0;
   for(const value of samples){const expected=!!a(value);assert.equal(!!b(value),expected,"Ajv dialect agreement at "+target.pointer);if((owned.validate(value).length===0)!==expected){row.mismatch={pointer:target.pointer,value,ajv:expected,owned:owned.validate(value).length===0,ajvErrors:a.errors,ownedErrors:owned.validate(value)};throw Error("Owned agreement at "+target.pointer);}if(expected)valid++;else invalid++;}
   const any=target.node?.anyOf,universal=Array.isArray(any)&&any.length===6&&["null","boolean","number","string","array","object"].every(type=>any.some(branch=>{if(branch.type!==type)return false;const keys=Object.keys(branch).sort().join(",");if(type==="array")return keys==="items,type"&&branch.items?.$ref==="#"+target.pointer;if(type==="object")return keys==="additionalProperties,type"&&branch.additionalProperties?.$ref==="#"+target.pointer;return keys==="type";}));const unconstrained=universal||target.node===true||target.node&&typeof target.node==="object"&&Object.keys(target.node).every(key=>annotations.has(key));
   coverage.push({pointer:target.pointer,valid,invalid,samples:samples.length,unconstrained:!!unconstrained,universalJson:!!universal,emptyLanguage:target.node===false,nonvacuous:target.node===false?valid===0&&invalid>0:valid>0&&(invalid>0||unconstrained)});
  }
  row.coverage=coverage;row.samples=coverage.reduce((n,r)=>n+r.samples,0);row.eligible=coverage.every(r=>r.nonvacuous);if(!row.eligible)row.reason="Missing positive or constraint-negative witnesses";
 }catch(error){row.reason=error.message;}
 results.push(row);
}
const summary={version:1,candidates:results.length,eligible:results.filter(r=>r.eligible).length,samples:results.reduce((n,r)=>n+(r.samples??0),0),results};
fs.writeFileSync(path.join(directory,"🗑️generated/schema-definitions-coverage.json"),JSON.stringify(summary,null,2)+"\n");console.log(JSON.stringify({candidates:summary.candidates,eligible:summary.eligible,samples:summary.samples,excluded:results.filter(r=>!r.eligible).map(({path,reason,coverage})=>({path,reason,uncovered:coverage?.filter(r=>!r.nonvacuous).map(r=>r.pointer)}))}));
