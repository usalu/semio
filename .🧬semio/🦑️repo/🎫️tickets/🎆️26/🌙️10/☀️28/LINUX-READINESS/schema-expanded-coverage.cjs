const fs=require("node:fs"),path=require("node:path"),{build,registry,required,nodes:walk}=require("./schema-definitions-resources.cjs"),{compileJsonSchemaSubset}=require(path.resolve("🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
const strings=["1/a","charts","demo","code","open","semio:a","rustc 1.0.0 ("+"a".repeat(40)+")","","a","ab","x","0","1","2","10","A","A_B","Pset_A","a/b","/a","/entities/a","C:\\a","a.xml",".","./a",".stage-"+"a".repeat(12),".workspace-events/segments/aaaaaaaa/a.retained","2026-10-29","2026-10-29T01:02:03Z","#ffffff","char:a","paste:a","a".repeat(8),"a".repeat(16),"a".repeat(40),"a".repeat(64),"0".repeat(64),"s00","a.sock","a".repeat(16)+".sock","/tmp/semio-dashboard-1","https://example.com/a","http://localhost:1234/","26/10/29/ABC","a@b","a>b","a:b>c","a:b","@a/b","./a.go","a.go","a.rs","a/🦀️.rs","semio-a-artifact-b","file:///a","canonical.json","no-cross-package-relative-a","private","pub","repo/breach/a","en","SEMIO_A","1.2.3","a.zip","repo run repo:test","repo ./📜️script.ts test a","repo://session/a","1.1","registry-invalid:a"];
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
const state=build(),documents=required(state),old=registry(documents),initial=new Set(JSON.parse(fs.readFileSync(path.join(__dirname,"🗑️generated/schema-definitions-inventory.json"),"utf8")).results.map(r=>r.path)),results=[];
const locations=new Map();for(const [file,document]of documents)walk(document,(node,pointer)=>{if(node&&typeof node==="object")locations.set(node,{file,document,pointer});});
for(const file of [...state.changed].filter(p=>!initial.has(p))){const row={path:file};try{
 const source=state.after.get(file),all=[],key=old.proofKey(file);walk(source,(node,pointer)=>all.push({node,pointer}));
 const memo=new Map(),checks=new Map(),check=node=>{if(typeof node==="boolean")return()=>node;const owner=locations.get(node);if(!owner)throw Error("Schema position owner required");if(!checks.has(node)){const compiled=old.getSchema(old.proofKey(owner.file)+"#"+owner.pointer);if(!compiled)throw Error("Declared schema position missing "+owner.file+owner.pointer);checks.set(node,compiled);}return checks.get(node);};
 const resolve=(ref,node)=>{const owner=locations.get(node);if(!owner)throw Error("Reference owner required");const at=ref.indexOf("#"),id=at<0?ref:ref.slice(0,at),fragment=at<0?"":ref.slice(at+1),document=id?state.after.get(state.ids.get(id)):owner.document;if(!document)throw Error("Declared reference document required "+ref);const value=!fragment?document:fragment.slice(1).split("/").reduce((n,k)=>n?.[k.replaceAll("~1","/").replaceAll("~0","~")],document);if(value===undefined)throw Error("Declared reference target required "+ref+" from "+owner.file);return value;};
  function witnesses(node,depth=0,seen=new Set()){
   if(node===false||depth>14||seen.has(node))return [];if(node===true)return [null];if(memo.has(node))return memo.get(node);const nextSeen=new Set(seen);nextSeen.add(node);let values=[];
   if(Object.hasOwn(node,"const"))values.push(node.const);if(node.enum)values.push(...node.enum);
   if(Object.hasOwn(node,"default"))values.push(node.default);if(node.examples)values.push(...node.examples);
   if(node.$ref)values.push(...witnesses(resolve(node.$ref,node),depth+1,nextSeen));
   for(const child of [...node.anyOf??[],...node.oneOf??[]])values.push(...witnesses(child,depth+1,nextSeen));
   if(node.allOf){const candidates=node.allOf.map(child=>witnesses(child,depth+1,nextSeen));if(candidates.every(v=>v.length))values.push(Object.assign({},...candidates.map(v=>v[0])));values.push(...candidates.flat());}
   const types=Array.isArray(node.type)?node.type:[node.type??(node.properties||node.required?"object":node.items?"array":undefined)];
   for(const type of types){
    if(type==="object"){
     const props=node.properties??{},make=all=>{const value={};for(const key of all?Object.keys(props):node.required??[]){const child=props[key]??node.additionalProperties??true,found=witnesses(child,depth+1,nextSeen)[0];if(found!==undefined)Object.defineProperty(value,key,{value:found,enumerable:true,writable:true,configurable:true});}for(let index=0;Object.keys(value).length<(node.minProperties??0)&&index<16;index++){const candidates=node.propertyNames?witnesses(node.propertyNames,depth+1,nextSeen).filter(v=>typeof v==="string"):["a","b","c","d"],key=candidates[index]??"key"+index;if(node.propertyNames&&!check(node.propertyNames)(key))continue;const found=witnesses(node.additionalProperties??true,depth+1,nextSeen)[0];if(found!==undefined)value[key]=found;}return value;};
     const bases=[make(false),make(true)];values.push(...bases);for(const base of bases)for(const[key,child]of Object.entries(props))for(const alternative of witnesses(child,depth+1,nextSeen).slice(0,8))values.push({...base,[key]:alternative});
    }else if(type==="array"){
     const tuple=Array.isArray(node.items)?node.items.map(child=>witnesses(child,depth+1,nextSeen)[0]):undefined;if(tuple&&tuple.every(value=>value!==undefined))values.push(tuple);const children=tuple?[]:witnesses(node.items??true,depth+1,nextSeen),length=Math.min(2048,node.minItems??0);values.push([]);if(length===0||children.length){values.push(Array.from({length},(_,i)=>children[i%children.length]??null));for(const child of children.slice(0,8))values.push(Array.from({length:Math.max(1,length)},()=>child));}
    }else if(type==="string"){
     const length=Math.min(1024,node.minLength??0);values.push(...strings,"a".repeat(length),"0".repeat(length));
    }else if(type==="number"||type==="integer"){
     let low=node.minimum??0;if(node.exclusiveMinimum!==undefined)low=Math.max(low,node.exclusiveMinimum+(type==="integer"?1:0.5));if(type==="integer")low=Math.ceil(low);if(node.multipleOf)low=Math.ceil(low/node.multipleOf)*node.multipleOf;values.push(low,low+1,low+2,0,1,-1,...[node.minimum,node.maximum,node.exclusiveMinimum,node.exclusiveMaximum].filter(v=>typeof v==="number"));
    }else if(type==="boolean")values.push(false,true);else if(type==="null")values.push(null);else values.push(...primitive);
   }
   const valid=unique(values).filter(value=>check(node)(value)).slice(0,32);if(valid.length)memo.set(node,valid);return valid;
  }
 const coverage=[];
 for(const target of all.filter(r=>r.pointer===""||/\/(?:\$defs|definitions)\/[^/]+$/.test(r.pointer))){
  const a=check(target.node),owned=compileJsonSchemaSubset(target.node,source,[...documents.values()].filter(d=>typeof d.$id==="string"&&d.$id!==source.$id)),positive=witnesses(target.node),samples=unique([...primitive,...positive,...positive.flatMap(v=>mutants(v))]);let valid=0,invalid=0;
  for(const value of samples){const expected=!!a(value),actual=owned.validate(value).length===0;if(actual!==expected)throw Error("Owned agreement "+target.pointer+" "+JSON.stringify(value).slice(0,400));if(expected)valid++;else invalid++;}
  const universal=target.node===true||Object.keys(target.node).every(k=>annotations.has(k));coverage.push({pointer:target.pointer,valid,invalid,nonvacuous:valid>0&&(invalid>0||universal)});
 }
 row.coverage=coverage;row.eligible=coverage.every(c=>c.nonvacuous);row.samples=coverage.reduce((n,c)=>n+c.valid+c.invalid,0);
 }catch(error){row.eligible=false;row.reason=error.message;}results.push(row);
}
fs.writeFileSync(path.join(__dirname,"🗑️generated/schema-expanded-coverage.json"),JSON.stringify({version:1,results},null,2)+"\n");
console.log(JSON.stringify({documents:results.length,eligible:results.filter(r=>r.eligible).length,samples:results.reduce((n,r)=>n+(r.samples??0),0),failures:results.filter(r=>!r.eligible).map(r=>({path:r.path,reason:r.reason,missing:r.coverage?.filter(c=>!c.nonvacuous).map(c=>c.pointer)}))}));
if(results.some(r=>!r.eligible))process.exitCode=1;
