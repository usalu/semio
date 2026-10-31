const fs=require("node:fs"),path=require("node:path"),crypto=require("node:crypto");
const directory=__dirname,output=path.join(directory,"🗑️generated"),audit=fs.readFileSync(path.join(directory,"schema-dialect-audit.md"),"utf8");
const initial=audit.split("\n").filter(line=>/^\| .+ \| \$defs \|/.test(line)).map(line=>line.split(" | ")[0].slice(2));
const annotations=new Set(["$schema","$id","$comment","title","description","default","examples","readOnly","writeOnly","deprecated"]);
const allowed=new Set([...annotations,"$ref","$defs","definitions","type","enum","const","allOf","anyOf","oneOf","not","if","then","else","properties","patternProperties","additionalProperties","propertyNames","required","minProperties","maxProperties","items","contains","minItems","maxItems","uniqueItems","minLength","maxLength","pattern","minimum","maximum","exclusiveMinimum","exclusiveMaximum","multipleOf","prefixItems","additionalItems"]);
function visit(node,pointer="",rows=[],tuples="none",resourceIDs=new Set(),vendor=new Set()){
 if(typeof node==="boolean"){rows.push({pointer,node});return rows;}if(!node||typeof node!=="object"||Array.isArray(node))throw Error("Schema object required at "+pointer);
 for(const key of Object.keys(node))if(!allowed.has(key)&&!vendor.has(key))throw Error("Unproved keyword "+key+" at "+pointer);
 if(pointer&&(Object.hasOwn(node,"$id")||Object.hasOwn(node,"$schema")))throw Error("Embedded resource at "+pointer);
 if(node.$ref!==undefined){if(typeof node.$ref!=="string"||!(node.$ref==="#"||node.$ref.startsWith("#/")||resourceIDs.has(node.$ref.split("#")[0])))throw Error("Nonlocal reference at "+pointer);for(const key of Object.keys(node))if(key!=="$ref"&&key!=="$defs"&&key!=="definitions"&&!annotations.has(key)&&!vendor.has(key))throw Error("Reference assertion sibling at "+pointer);}
 
 if(node.prefixItems&&tuples!=="source2020")throw Error("Unproved prefixItems at "+pointer);if(Array.isArray(node.items)&&tuples!=="draft07")throw Error("Legacy tuple at "+pointer);if(node.additionalItems!==undefined&&tuples!=="draft07")throw Error("Mixed tuple dialect at "+pointer);
 rows.push({pointer,node});
 for(const key of ["$defs","definitions","properties","patternProperties"])if(node[key]!==undefined){if(!node[key]||typeof node[key]!=="object"||Array.isArray(node[key]))throw Error("Schema map required");for(const[name,child]of Object.entries(node[key]))visit(child,pointer+"/"+key+"/"+name.replaceAll("~","~0").replaceAll("/","~1"),rows,tuples,resourceIDs,vendor);}
 for(const key of ["additionalProperties","propertyNames","items","additionalItems","contains","not","if","then","else"])if(node[key]!==undefined){if(Array.isArray(node[key]))node[key].forEach((child,index)=>visit(child,pointer+"/"+key+"/"+index,rows,tuples,resourceIDs,vendor));else visit(node[key],pointer+"/"+key,rows,tuples,resourceIDs,vendor);}
 for(const key of ["allOf","anyOf","oneOf","prefixItems"])if(node[key]!==undefined){if(!Array.isArray(node[key]))throw Error("Schema sequence required");node[key].forEach((child,index)=>visit(child,pointer+"/"+key+"/"+index,rows,tuples,resourceIDs,vendor));}
 return rows;
}
function convert(source,resourceIDs=new Set(),vendor=new Set()){
 const next=structuredClone(source),rows=visit(next,"",[],"source2020",resourceIDs,vendor);
 for(const{node}of rows){if(typeof node==="boolean")continue;if(node.prefixItems){const tail=node.items;node.items=node.prefixItems;delete node.prefixItems;if(tail!==undefined)node.additionalItems=tail;}}
 next.$schema="http://json-schema.org/draft-07/schema#";return next;
}
function inventory(){
 const results=[];for(const file of initial){const row={path:file};try{const status=fs.lstatSync(file);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>4*1024*1024)throw Error("Bounded ordinary Source required");const bytes=fs.readFileSync(file),schema=JSON.parse(bytes);row.sha256=crypto.createHash("sha256").update(bytes).digest("hex");if(schema.$schema!=="https://json-schema.org/draft/2020-12/schema")throw Error("Current draft2020 root required");const nodes=visit(schema);row.definitions=nodes.filter(({node})=>node.$defs).map(({pointer,node})=>({pointer,names:Object.keys(node.$defs)}));if(!row.definitions.length)throw Error("Definitions cohort required");row.nodes=nodes.length;row.eligible=true;}catch(error){row.eligible=false;row.reason=error.message;}results.push(row);}
 fs.writeFileSync(path.join(output,"schema-definitions-inventory.json"),JSON.stringify({version:1,results},null,2)+"\n");console.log(JSON.stringify({candidates:results.length,structural:results.filter(r=>r.eligible).length,excluded:results.filter(r=>!r.eligible)}));return results;
}
function prepare(source,file){
 const next=structuredClone(source);
 if(file==="🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json"){if(!next.$defs||!Object.hasOwn(next,"x-semio-option-syntax")||Object.hasOwn(next.definitions??{},"OptionSyntaxCatalog"))throw Error("Exact original Print metadata required");next.definitions??={};next.definitions.OptionSyntaxCatalog={const:next["x-semio-option-syntax"]};delete next["x-semio-option-syntax"];const guide=next.$defs.ChartGuide;guide.definitions??={};for(const[key,name]of[["x-semio-guide-defaults","Defaults"],["x-semio-guide-units","Units"],["x-semio-guide-geometry","Geometry"]]){if(!Object.hasOwn(guide,key)||Object.hasOwn(guide.definitions,name))throw Error("Exact original guide metadata required");guide.definitions[name]={const:guide[key]};delete guide[key];}}
 if(file==="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json"){const quota=next.definitions.OptionalQuota;for(const[key,name]of[["policy","DefaultPolicy"],["optionalPath","OptionalPath"],["protocol","Protocol"]]){if(!Object.hasOwn(quota,key)||Object.hasOwn(quota.definitions,name))throw Error("Exact original quota metadata required");quota.definitions[name]={const:quota[key]};delete quota[key];}const ledger=next.definitions.OptionalLedger;ledger.definitions??={};if(!Object.hasOwn(ledger,"protocol")||Object.hasOwn(ledger.definitions,"Protocol"))throw Error("Exact original ledger metadata required");ledger.definitions.Protocol={const:ledger.protocol};delete ledger.protocol;}
 return next;
}

module.exports={visit,convert,prepare,inventory,initial};if(require.main===module)inventory();
