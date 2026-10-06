import {readFileSync,writeFileSync,renameSync} from "node:fs";
import {join} from "node:path";
const ticket=join(import.meta.dir,".."),root="C:/git/semio",input=join(ticket,"📥️authored-inputs/native-canonical-option-syntax"),read=(name:string)=>JSON.parse(readFileSync(join(input,name),"utf8").replace(/^\uFEFF/,""));
const schema=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json"),"utf8")),scopes=JSON.parse(readFileSync(join(ticket,"🗑️generated/printed-api-family-source-key-scopes.json"),"utf8")),shared=read("shared-source-owned-classes.json"),scientific=read("scientific-source-owned-classes.json"),local=read("family-source-owned-classes.json"),reviewed=read("root-reviewed-local-source-classes.json");
type Syntax={syntax:string,items?:string};
const flatten=(classes:Record<string,string>|undefined):Record<string,Syntax>=>Object.fromEntries(Object.entries(classes??{}).flatMap(([syntax,keys])=>keys.split(" ").filter(Boolean).map(key=>[key,{syntax}])));
const family:Record<string,Record<string,Syntax>>={},missing:string[]=[],counts:Record<string,number>={};
for(const[name,definition]of Object.entries(schema["x-semio-family-options"]) as [string,{options:Record<string,{type:string|string[]}>}][]){
 const inherited:Record<string,Syntax>={variant:{syntax:"identifier"}};
 for(const scope of Object.keys(scopes[name]??{}))Object.assign(inherited,flatten(shared[scope]),flatten(reviewed.shared[scope]));
 Object.assign(inherited,flatten(scientific.families[name]),flatten(local[name]),flatten(reviewed.family[name]));
 if(name.startsWith("sci-"))for(const[key,syntax]of Object.entries(scientific.sharedCanvas) as [string,string][])if(!inherited[key])inherited[key]={syntax};
 family[name]={};
 for(const[key,descriptor]of Object.entries(definition.options)){
  const annotation=inherited[key]??(["number","integer","boolean"].includes(String(descriptor.type))?{syntax:"scalar"}:undefined);
  if(!annotation){missing.push(`${name}.${key}`);continue;}
  const items=reviewed.items.family[`${name}.${key}`]??Object.keys(scopes[name]??{}).map(scope=>reviewed.items.shared[`${scope}.${key}`]).find(Boolean);
  family[name][key]={...annotation,...(items?{items}:{})};counts[annotation.syntax]=(counts[annotation.syntax]??0)+1;
 }
}
for(const[key,annotation]of Object.entries({columns:{syntax:"records",items:"identifier"},cells:{syntax:"records",items:"identifier"},headers:{syntax:"records",items:"text"}}))family.table![key]=annotation;
for(const[name]of Object.entries(family))if(scopes[name]?.["semio / viz / diagram"]?.includes("legend"))family[name]!.legend={syntax:"records",items:"text"};
const result={schemaSha256:new Bun.CryptoHasher("sha256").update(readFileSync(join(root,"🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json"))).digest("hex"),families:Object.keys(family).length,descriptors:Object.values(family).reduce((sum,options)=>sum+Object.keys(options).length,0),counts,missing,family};
writeFileSync(join(ticket,"🗑️generated/option-syntax-family-annotation-candidate.json"),JSON.stringify(result,null,2)+"\n");
console.log(`[DEBUG] ${result.families} families, ${result.descriptors} explicit descriptors, ${missing.length} unclassified`);
if(missing.length)throw Error(missing.join("\n"));
if(Bun.argv[2]==="write"){
 const sourcePath=join(root,"🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json"),source=readFileSync(sourcePath,"utf8"),ranges=new Map<string,{start:number,end:number}>();let cursor=0;
 const whitespace=()=>{while(/\s/.test(source[cursor]??"!")&&cursor<source.length)cursor++;};
 const quoted=():string=>{const start=cursor++;while(cursor<source.length){if(source[cursor++]==="\\")cursor++;else if(source[cursor-1]==='"')break;}return JSON.parse(source.slice(start,cursor));};
 const value=(path:string[])=>{whitespace();const start=cursor;if(source[cursor]==='{'){cursor++;whitespace();while(source[cursor]!=='}'){const key=quoted();whitespace();if(source[cursor++]!==':')throw Error("Expected colon");value([...path,key]);whitespace();if(source[cursor]!==',')break;cursor++;whitespace();}if(source[cursor++]!=='}')throw Error("Expected object close");}else if(source[cursor]==='['){cursor++;whitespace();let index=0;while(source[cursor]!==']'){value([...path,String(index++)]);whitespace();if(source[cursor]!==',')break;cursor++;}if(source[cursor++]!==']')throw Error("Expected array close");}else if(source[cursor]==='"')quoted();else while(cursor<source.length&&!/[,}\]\s]/.test(source[cursor]!))cursor++;ranges.set(JSON.stringify(path),{start,end:cursor});};value([]);
 const edits:{start:number,end:number,text:string}[]=[],range=(path:string[])=>{const node=ranges.get(JSON.stringify(path));if(!node)throw Error(`Missing ${path.join("/")}`);return node;},replace=(path:string[],data:unknown)=>{const node=range(path);edits.push({...node,text:JSON.stringify(data,null,2).replaceAll("\n","\r\n")});};
 for(const[name,options]of Object.entries(family))for(const[key,annotation]of Object.entries(options)){const node=range(["x-semio-family-options",name,"options",key]),indent=source.slice(node.start+1).match(/^\s*\n([ \t]*)/)?.[1]??"          ";edits.push({start:node.start+1,end:node.start+1,text:"\r\n"+Object.entries(annotation).map(([key,value])=>`${indent}${JSON.stringify(key)}: ${JSON.stringify(value)},`).join("\r\n")});}
 const classes=["text","identifier","expression","records","key-list","style","paint","number-list","scalar","value"],contract={type:"object",required:["syntax"],properties:{syntax:{enum:classes},items:{enum:["text","identifier"]}},anyOf:[{properties:{syntax:{const:"records"}}},{properties:{items:false}}],additionalProperties:false};
 const definition=structuredClone(schema.$defs.FamilyOptions),descriptor=definition.properties.options.additionalProperties;
 descriptor.required.push("syntax");descriptor.properties.syntax=contract.properties.syntax;descriptor.properties.items=contract.properties.items;descriptor.anyOf=contract.anyOf;descriptor.properties.type={anyOf:[{enum:["string","number","integer","boolean","null"]},{type:"array",minItems:1,uniqueItems:true,items:{enum:["string","number","integer","boolean","null"]}}]};descriptor.properties.default.type.push("null");replace(["$defs","FamilyOptions"],definition);
 const definitions=range(["$defs"]);edits.push({start:definitions.end-1,end:definitions.end-1,text:`,\r\n    "OptionSyntax": ${JSON.stringify(contract,null,2).replaceAll("\n","\r\n")}\r\n  `});
 const generic=read("generic-source-owned-classes.json"),native:Record<string,Record<string,Syntax>>={};for(const[owner,options]of Object.entries(generic) as [string,Record<string,string|Syntax>][])native[owner]=Object.fromEntries(Object.entries(options).map(([key,annotation])=>[key,typeof annotation==="string"?{syntax:annotation}:annotation]));
 edits.push({start:source.length-1-source.slice(source.lastIndexOf('}')+1).length,end:source.length-1-source.slice(source.lastIndexOf('}')+1).length,text:`,\r\n  "x-semio-option-syntax": ${JSON.stringify(native,null,2).replaceAll("\n","\r\n")}\r\n`});
 replace(["x-semio-family-options","arch-axonometric","options","angle-x","default"],-30);replace(["x-semio-family-options","arch-axonometric","options","angle-y","default"],30);
 let candidate=source;for(const edit of edits.sort((a,b)=>b.start-a.start))candidate=candidate.slice(0,edit.start)+edit.text+candidate.slice(edit.end);
 const parsed=JSON.parse(candidate);for(const[name,old]of Object.entries(schema["x-semio-family-options"]) as [string,any][]){const next=structuredClone(parsed["x-semio-family-options"][name]);for(const descriptor of Object.values(next.options) as any[]){delete descriptor.syntax;delete descriptor.items;}if(name==="arch-axonometric"){next.options["angle-x"].default=old.options["angle-x"].default;next.options["angle-y"].default=old.options["angle-y"].default;}if(JSON.stringify(next)!==JSON.stringify(old))throw Error(`Changed other descriptor ${name}`);}
 writeFileSync(join(input,"schema-before-syntax.json"),source);const temporary=join(ticket,"🗑️generated/option-syntax-schema-candidate.json");writeFileSync(temporary,candidate);if(readFileSync(sourcePath,"utf8")!==source)throw Error("Concurrent schema change");renameSync(temporary,sourcePath);
 console.log(`[DEBUG] Authored syntax descriptor write: ${new Bun.CryptoHasher("sha256").update(candidate).digest("hex")}`);
}
