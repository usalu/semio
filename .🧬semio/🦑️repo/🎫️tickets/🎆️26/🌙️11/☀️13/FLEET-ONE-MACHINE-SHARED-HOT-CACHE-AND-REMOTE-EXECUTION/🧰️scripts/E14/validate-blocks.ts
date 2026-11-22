import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {brotliDecompressSync} from "node:zlib";
import {createHash} from "node:crypto";
import {compileJsonSchemaSubset} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const schema=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧬️schema/🔣️.json","utf8"));
const validator=compileJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/observationRows"});
const base="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections";
let total=0,bad=0,renamed=0;const samples:string[]=[];
for(const sel of readdirSync(base))for(const sub of ["np","inference"]){const dir=join(base,sel,sub,"ob");let names:string[];try{names=readdirSync(dir);}catch{continue;}
 for(const name of names){if(!name.endsWith(".br"))continue;total++;try{const bytes=brotliDecompressSync(readFileSync(join(dir,name)));if(createHash("sha256").update(bytes).digest("hex")!==name.slice(0,-3)){renamed++;continue;}const errors=validator.validate(JSON.parse(bytes.toString("utf8")));if(errors.length){bad++;if(samples.length<5)samples.push(join(dir,name)+" "+JSON.stringify(errors.slice(0,2)).slice(0,400));}}catch(e){renamed++;}}}
console.log(JSON.stringify({total,bad,otherDamage:renamed,samples},null,1));
