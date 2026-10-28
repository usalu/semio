/** ⏱️ [td-observe] times Bun.TOML against the portable owned parser on the repository Cargo.lock and the root Cargo.toml. usage: bun toml-bench.ts <root> */
import {readFileSync} from "node:fs";
const root=process.argv[2]!,{parseTomlV1}=await import(root+"/🧰️framework/🔨️modules/⚙️runtime/🪶️portable/📄️toml/🟦️.ts");
for(const name of ["Cargo.lock","Cargo.toml"]){const text=readFileSync(root+"/"+name,"utf8");for(const [label,f] of [["bun",Bun.TOML.parse],["portable",parseTomlV1]] as const){f(text);const t=performance.now();for(let i=0;i<20;i++)f(text);console.log("[DEBUG]",name,text.length,"chars",label,((performance.now()-t)/20).toFixed(2),"ms/parse");}console.log("[DEBUG] equal",JSON.stringify(Bun.TOML.parse(text))===JSON.stringify(parseTomlV1(text)));}
