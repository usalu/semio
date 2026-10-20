/** 📦️ Lists workspace packages whose exports use condition maps, and how many specifier keys use wildcards. */
import {diskZeroTouchTreeV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
const tree=diskZeroTouchTreeV1(process.argv[2]!);let conditional=0,wildcard=0,plain=0;const samples:string[]=[];
for(const [name,pkg] of Object.entries(tree.packages))for(const [key,value] of Object.entries(pkg.exports)){
  if(key.includes("*"))wildcard++;
  if(typeof value==="string")plain++;else{conditional++;if(samples.length<12)samples.push(`${name} ${key} ${JSON.stringify(value)}`);}
}
console.log(JSON.stringify({packages:Object.keys(tree.packages).length,plain,conditional,wildcard,samples},null,1));
