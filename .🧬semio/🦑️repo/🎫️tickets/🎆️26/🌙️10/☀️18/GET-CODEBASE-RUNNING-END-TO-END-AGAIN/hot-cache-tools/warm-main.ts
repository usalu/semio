// main script: reports how long the imports of the real closure take once the process is up
import {pathToFileURL} from "node:url";
import {join} from "node:path";
const root="/Users/ueli/Documents/semio",L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",B=L+"/🏗️build";
const mods=JSON.parse(await Bun.file("/private/tmp/hot-cache/real/mods.json").text());
const t0=Bun.nanoseconds();let ok=0;
for(const m of mods){try{await import(pathToFileURL(join(root,L,m)).href);ok++;}catch(e){}}
console.log(JSON.stringify({argv:process.argv.slice(2),importedMs:Math.round((Bun.nanoseconds()-t0)/1e5)/10,ok,of:mods.length,processAgeMs:Math.round(Bun.nanoseconds()/1e6),rssMB:Math.round(process.memoryUsage().rss/1048576)}));
