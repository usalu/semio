const fs=require("node:fs"),path=require("node:path"),crypto=require("node:crypto"),ts=require("typescript");
const directory=__dirname,inventory=JSON.parse(fs.readFileSync(path.join(directory,"🗑️generated/schema-definitions-inventory.json"),"utf8")),paths=fs.readFileSync(path.join(directory,"🗑️generated/schema-definitions-source-paths.bin"),"utf8").split("\0").filter(Boolean),candidates=new Map(inventory.results.map(row=>[path.resolve(row.path),row.path])),ids=new Map();
for(const row of inventory.results){const schema=JSON.parse(fs.readFileSync(row.path,"utf8"));if(schema.$id)ids.set(schema.$id,row.path);}
const incoming=[],code=[],scanKeys=["$defs","definitions","properties","patternProperties","dependentSchemas"];
function schemaReferences(node,pointer,owner){
 if(!node||typeof node!=="object"||Array.isArray(node))return;
 if(typeof node.$ref==="string"&&!node.$ref.startsWith("#")){const[uri,fragment]=node.$ref.split("#");if(ids.has(uri)&&fragment?.includes("/$defs/"))incoming.push({path:owner,pointer:pointer+"/$ref",reference:node.$ref,target:ids.get(uri)});}
 for(const key of scanKeys)if(node[key]&&typeof node[key]==="object")for(const[name,child]of Object.entries(node[key]))schemaReferences(child,pointer+"/"+key+"/"+name,owner);
 for(const key of ["additionalProperties","unevaluatedProperties","propertyNames","items","additionalItems","contains","not","if","then","else","unevaluatedItems"])if(node[key]!==undefined){if(Array.isArray(node[key]))node[key].forEach((child,i)=>schemaReferences(child,pointer+"/"+key+"/"+i,owner));else schemaReferences(node[key],pointer+"/"+key,owner);}
 for(const key of ["allOf","anyOf","oneOf","prefixItems"])if(Array.isArray(node[key]))node[key].forEach((child,i)=>schemaReferences(child,pointer+"/"+key+"/"+i,owner));
}
for(const file of paths){
 let text;try{const st=fs.lstatSync(file);if(!st.isFile()||st.isSymbolicLink()||st.size>8*1024*1024)continue;text=fs.readFileSync(file,"utf8");}catch{continue;}
 if(file.endsWith(".json")){if(!text.includes("$defs"))continue;try{const json=JSON.parse(text);if(file.includes("🧬️schema")||/^https?:\/\/json-schema.org\//.test(json.$schema??""))schemaReferences(json,"",file);}catch{}continue;}
 if(!/schema|🔣️\.json/.test(text))continue;const referenced=new Set();
 if(/\.(?:ts|mts|mjs|cjs)$/.test(file)){
  const ast=ts.createSourceFile(file,text,ts.ScriptTarget.Latest,true);function walk(node){if(ts.isStringLiteralLike(node)){const value=node.text;for(const name of [path.resolve(path.dirname(file),value),path.resolve(value)])if(candidates.has(name))referenced.add(candidates.get(name));}ts.forEachChild(node,walk);}walk(ast);
 }else{
  for(const match of text.matchAll(/["']([^"'\n]+)["']/g))for(const name of [path.resolve(path.dirname(file),match[1]),path.resolve(match[1])])if(candidates.has(name))referenced.add(candidates.get(name));
 }
 if(referenced.size)code.push({path:file,sha256:crypto.createHash("sha256").update(text).digest("hex"),schemas:[...referenced],definitions:text.includes("$defs"),ajv2020:/ajv\/dist\/2020/.test(text)});
}
const result={version:1,searched:paths.length,incoming,code};fs.writeFileSync(path.join(directory,"🗑️generated/schema-definitions-incoming.json"),JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify({searched:paths.length,incoming:incoming.length,incomingPaths:[...new Set(incoming.map(r=>r.path))],directCode:code.length,definitionConsumers:code.filter(r=>r.definitions).length,ajv2020:code.filter(r=>r.ajv2020).length}));
