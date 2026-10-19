import { build } from "esbuild";
const [entry, ...rest] = process.argv.slice(2);
const r = await build({ entryPoints: [entry!], absWorkingDir: process.cwd(), bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external" });
for (const k of Object.keys(r.metafile!.inputs)) if (!k.startsWith("🧰️framework/🔨️modules/")) console.log(k);
console.log(Object.keys(r.metafile!.inputs).length);
