import { join } from "node:path";
const root = process.argv[2], entry = process.argv[3];
const b = await (await import("esbuild")).build({ entryPoints: [join(root, entry)], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
for (const k of Object.keys(b.metafile.inputs)) console.log(k);
