/** 🔬 Independently parse pinned published scope with parse5 and admit its owned schema with Ajv. */
import {parse} from "parse5";
import Ajv from "ajv/dist/2020.js";
import assert from "node:assert/strict";
import {readFile,writeFile,mkdir} from "node:fs/promises";
import {createHash} from "node:crypto";
import {resolve} from "node:path";

const root=resolve(import.meta.dir,"../../../../../../.."),base=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd");
const catalog=JSON.parse(await readFile(resolve(base,"🔣️.json"),"utf8")),schema=JSON.parse(await readFile(resolve(base,"schema.json"),"utf8"));
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
assert.equal(validate(catalog),true,JSON.stringify(validate.errors));
type Node={tagName?:string;value?:string;attrs?:{name:string;value:string}[];childNodes?:Node[]};
const attr=(node:Node,name:string)=>node.attrs?.find(row=>row.name===name)?.value;
const all=(node:Node,predicate:(node:Node)=>boolean):Node[]=>[...(predicate(node)?[node]:[]),...(node.childNodes??[]).flatMap(child=>all(child,predicate))];
const text=(node:Node):string=>node.value??(node.childNodes??[]).map(text).join("");
let checked=0;
for(let index=0;index<catalog.sources.length;index++){
 const source=catalog.sources[index],response=await fetch(source.url);assert.equal(response.ok,true);
 const bytes=new Uint8Array(await response.arrayBuffer());assert.equal(createHash("sha256").update(bytes).digest("hex"),source.sha256);
 const dom=parse(new TextDecoder().decode(bytes)) as unknown as Node,rows=all(dom,node=>node.tagName==="div"&&attr(node,"class")==="wrapper");
 const entity=index===0,expected=catalog[entity?"entities":"concepts"];
 const extracted=rows.flatMap(row=>{
  const link=all(row,node=>node.tagName==="a")[0],href=link&&attr(link,"href");
  if(!href||(entity?!href.startsWith("lexical/Ifc"):!href.endsWith("content.html")))return [];
  const name=text(link!).trim().replace(/\s+/g," "),badges=all(row,node=>node.tagName==="abbr"),views:Record<string,string[]>={};let universal=false;
  for(const badge of badges){const label=text(badge).trim();universal ||=label.startsWith("product");const view=label.startsWith("RV")?"ReferenceView":label.startsWith("AbRV")?"Alignment-basedReferenceView":undefined;if(view)views[view]=entity?(attr(badge,"title")??"").replace(/^Included in /,"").split(",").map(value=>value.trim()):[];}
  return [{id:entity?name.toUpperCase():href.replace(/\/?content\.html$/,"").replace(/\/$/,""),name,href,universal,views}];
 });
 assert.deepEqual(extracted,expected);checked+=extracted.length;
}
const generated=resolve(import.meta.dir,"🗑️generated");await mkdir(generated,{recursive:true});
await writeFile(resolve(generated,"mvd-oracle.json"),JSON.stringify({checked,schemaValid:true,sources:catalog.sources},null,2)+"\n");
console.log(JSON.stringify({checked,schemaValid:true}));
