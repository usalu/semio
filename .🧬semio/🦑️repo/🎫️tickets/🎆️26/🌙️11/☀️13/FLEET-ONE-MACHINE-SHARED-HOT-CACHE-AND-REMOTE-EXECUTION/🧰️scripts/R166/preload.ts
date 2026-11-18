import{plugin}from"bun";
import{existsSync}from"node:fs";
import{resolve,dirname}from"node:path";
const S=process.env.R166_STAGED!,L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",C=L+"⚡️caching/🚀️bootstrap/🛠️tools/";
const map:Record<string,string>={[C+"📦️sdk/🎟️capacity/🧮️formula/🟦️.ts"]:S+"/formula.ts",[C+"📦️sdk/🎟️capacity/📋️policy.json"]:S+"/policy.json",[C+"📦️sdk/🎟️capacity/🧬️schema.json"]:S+"/schema.json",[L+"🏗️build/📖️texts/🔣️.json"]:S+"/texts.json",[C+"🧹️retention/📏️headroom/🟦️.ts"]:S+"/headroom.ts",[C+"🟦️.ts"]:S+"/tools.ts",[C+"📦️sdk/🎟️capacity/🧾️sealed-plan/🟦️.ts"]:S+"/sealed.ts"};
plugin({name:"r166",setup(b){b.onResolve({filter:/.*/},a=>{if(!a.importer||!a.path.startsWith("."))return;const back=Object.entries(map).find(([,v])=>v===a.importer),imp=back?back[0]:a.importer.replace(/^file:\/\//,""),abs=resolve(dirname(imp),a.path);const m=map[abs];if(m)return{path:m};if(back&&existsSync(abs))return{path:abs};});}});
