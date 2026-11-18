import{plugin}from"bun";
import{readFileSync}from"node:fs";
const S=process.env.R195_STAGED!;
const R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/";
const map:Record<string,string>={[R+"🟦️.ts"]:"ready.ts",[R+"🧬️schema/🔣️.json"]:"ready.schema.json",[R+"🌲️process-tree/🟦️.ts"]:"tree.ts",[R+"🌲️process-tree/🧬️schema/🔣️.json"]:"tree.schema.json",[R+"🌲️process-tree/🧹️containment/🟦️.ts"]:"cont.ts",[R+"🌲️process-tree/🧹️containment/🧬️schema/🔣️.json"]:"cont.schema.json"};
plugin({name:"r195",setup(b){const filter=new RegExp("^("+Object.keys(map).map(k=>k.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("|")+")$");b.onLoad({filter},a=>{const f=S+"/stage/"+map[a.path]!;const contents=readFileSync(f,"utf8");if(a.path.endsWith(".json"))return{contents:"export default "+contents+";",loader:"js"};return{contents,loader:"ts"};});}});
