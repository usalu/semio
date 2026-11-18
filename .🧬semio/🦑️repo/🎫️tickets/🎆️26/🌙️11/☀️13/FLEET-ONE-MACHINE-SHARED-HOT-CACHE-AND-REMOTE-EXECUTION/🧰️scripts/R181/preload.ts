import{plugin}from"bun";
import{readFileSync}from"node:fs";
const S=process.env.R181_STAGED!,L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",E=L+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/";
const map:Record<string,string>=JSON.parse(readFileSync(S+"/"+(process.env.R181_MAP??"map.json"),"utf8"));
plugin({name:"r181",setup(b){const filter=new RegExp("^("+Object.keys(map).map(k=>k.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("|")+")$");b.onLoad({filter},a=>{const f=map[a.path]!;let contents=readFileSync(f,"utf8");contents=contents.replace('join(import.meta.dir,"🐍️oracle/🐍️.py")',JSON.stringify(S+"/stage/oracle.py"));if(a.path.endsWith(".json"))return{contents:"export default "+contents+";",loader:"js"};const loader=a.path.endsWith(".ts")?"ts":"js";return{contents,loader};});}});
