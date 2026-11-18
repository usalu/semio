import{plugin}from"bun";
import{existsSync}from"node:fs";
import{resolve,dirname}from"node:path";
const S=process.env.R176_STAGED!,R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/";
const map:Record<string,string>={[R+"🌲️process-tree/🟦️.ts"]:S+"/tree.ts",[R+"🌲️process-tree/🧬️schema/🔣️.json"]:S+"/tree.json",[R+"🟦️.ts"]:S+"/ready.ts",[R+"🧪️tests/🕰️stale-stage/🟦️.ts"]:S+"/stale-test.ts"};
plugin({name:"r176",setup(b){b.onResolve({filter:/.*/},a=>{if(!a.importer||!a.path.startsWith("."))return;const back=Object.entries(map).find(([,v])=>v===a.importer),imp=back?back[0]:a.importer.replace(/^file:\/\//,""),abs=resolve(dirname(imp),a.path);const m=map[abs];if(m)return{path:m};if(back&&existsSync(abs))return{path:abs};});}});
