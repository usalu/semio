import {readFileSync} from "node:fs";
const mod = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const text = readFileSync(process.argv[2]!, "utf8");
const f = (mod as any).inspectRustCompileReferences ?? null;
console.log(Object.keys(mod));
