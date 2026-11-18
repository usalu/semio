import{plugin}from"bun";
import{existsSync}from"node:fs";
import{resolve,dirname}from"node:path";
const S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R180",L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/";
const map:Record<string,string>={[L+"🟦️.ts"]:S+"/staged-live/nx.ts",[L+"📦️graph/🟦️.ts"]:S+"/staged-live/graph.ts",[L+"🤝️nested-custody/🟦️.ts"]:S+"/stage/🤝️nested-custody/🟦️.ts",[L+"📦️graph/🧯️release-failure/🟦️.ts"]:S+"/stage/🧯️release-failure/🟦️.ts"};
plugin({name:"r180",setup(b){b.onResolve({filter:/.*/},a=>{if(map[a.path])return{path:map[a.path]};if(!a.importer||!a.path.startsWith("."))return;const back=Object.entries(map).find(([,v])=>v===a.importer),imp=back?back[0]:a.importer.replace(/^file:\/\//,""),abs=resolve(dirname(imp),a.path);const m=map[abs];if(m)return{path:m};if(back&&existsSync(abs))return{path:abs};});}});
