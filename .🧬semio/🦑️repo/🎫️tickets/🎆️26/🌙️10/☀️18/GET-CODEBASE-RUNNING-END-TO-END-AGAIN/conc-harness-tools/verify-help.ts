import {readFileSync} from "node:fs";import {join} from "node:path";import Ajv from "ajv";
const root=process.argv[2]!,B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations",read=(p:string)=>JSON.parse(readFileSync(join(root,B,p),"utf8"));
const catalog=read("🔣️catalog.json"),schema=read("🧬️schema/🔣️.json"),fixture=read("🧪️tests/🧫️fixtures/📖️help/🔣️.json");
const help=await import(join(root,B,"📖️help/🟦️.ts"));
const ajv=new Ajv({strict:true,allowUnionTypes:true});const ok=ajv.compile(schema)(catalog);console.log("catalog valid:",ok,ok?"":JSON.stringify(ajv.errors));
let bad=0,count=0;
for(const lang of catalog.languages){for(const key of Object.keys(fixture.snapshots[lang])){const args=key==="root"?[]:key.split(" ");const got=help.renderBuildHelpV1(catalog,help.readBuildHelpRequestV1(catalog,args),lang).split("\n");count++;if(JSON.stringify(got)!==JSON.stringify(fixture.snapshots[lang][key])){bad++;console.log("DIFF",lang,key);}}
 const missing=[...["root",...catalog.operations.flatMap((o:any)=>[o.name,...(o.subcommands??[]).map((s:any)=>o.name+" "+s.name)])]].filter(k=>!(k in fixture.snapshots[lang]));if(missing.length)console.log("no snapshot",lang,missing);}
console.log("snapshots",count,"mismatches",bad);
const names=catalog.operations.map((o:any)=>o.name);console.log("enum equals names:",JSON.stringify([...names].sort())===JSON.stringify([...schema.definitions.OperationName.enum].sort()));
let de=0;const texts=(v:any):any[]=>v&&typeof v==="object"?("en" in v&&"de" in v&&typeof v.en==="string"?[v]:Object.values(v).flatMap(texts)):[];for(const t of texts(catalog))if(t.de===t.en)de++;console.log("texts with de==en:",de);
