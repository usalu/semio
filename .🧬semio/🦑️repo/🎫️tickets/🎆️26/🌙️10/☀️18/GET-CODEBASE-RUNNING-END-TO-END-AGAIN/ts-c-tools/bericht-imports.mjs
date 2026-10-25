import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const owner = "/Users/ueli/Documents/semio/♻️mit-bestand/📋️bericht";
for (const entry of ["🔨️modules/📄️documents/📜️script.ts","🔨️modules/👥️actor-network/📜️script.ts"]) {
  const bundle = await require("esbuild").build({ entryPoints: [owner + "/" + entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
  console.log(entry.slice(-30), Object.keys(bundle.metafile.inputs).length);
}
