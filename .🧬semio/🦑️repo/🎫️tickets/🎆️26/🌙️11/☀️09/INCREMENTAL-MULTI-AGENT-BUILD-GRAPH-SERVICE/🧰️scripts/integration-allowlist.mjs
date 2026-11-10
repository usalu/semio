import {readFileSync,writeFileSync} from "node:fs";
import {languageLawAllowlistV1,readLanguageLawSourcesV1} from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts";
const file="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json";
const cur=JSON.parse(readFileSync(file,"utf8"));
const gen=languageLawAllowlistV1(readLanguageLawSourcesV1("/home/deck/semio"),{ceiling:cur.ceiling,defaultsCeiling:cur.defaultsCeiling});
const mine=k=>k.includes("🏗️build/🕸️graph/")&&/🕸️graph\/(🟦️\.ts|[^/]*\/🟦️\.ts)$/u.test(k);
const keys=[...new Set([...Object.keys(cur.files),...Object.keys(gen.files)])].filter(mine);
const changes=[];for(const k of keys)if(cur.files[k]!==gen.files[k])changes.push([k.split("🕸️graph/")[1],cur.files[k],gen.files[k]]);
console.log(JSON.stringify(changes));
if(process.argv[2]==="write"){const files={...cur.files};for(const [,,]of changes);for(const k of keys){if(gen.files[k]===undefined)delete files[k];else files[k]=gen.files[k];}
 const sorted=Object.fromEntries(Object.entries(files).sort(([a],[b])=>Object.keys(gen.files).indexOf(a)-Object.keys(gen.files).indexOf(b)));
 const text=readFileSync(file,"utf8");
 writeFileSync(file,JSON.stringify({...cur,files:sorted},null,1)+(text.endsWith("\n")?"\n":""));}
