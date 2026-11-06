import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🧰️utility/🟦️.ts",source=readFileSync(path,"utf8"),before='script=fileURLToPath(import.meta.url),library=resolve(dirname(script),"..")',after='script=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts"),library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library")';
if(!source.includes(before))throw Error("Resident entry changed");writeFileSync(path,source.replace(before,after));
