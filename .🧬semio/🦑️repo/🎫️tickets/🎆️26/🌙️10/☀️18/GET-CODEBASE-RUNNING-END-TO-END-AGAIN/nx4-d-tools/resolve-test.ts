import {existsSync} from "node:fs";
import {dirname} from "node:path";
const from=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts";
const spec="../../../../../../../🏗️build/📥️inputs/💾️storage/🔧️custody/🟨️.mjs";
try{const r=Bun.resolveSync(spec,dirname(from));console.log(r,existsSync(r));}catch(e){console.log("ERR",String(e).slice(0,200))}
