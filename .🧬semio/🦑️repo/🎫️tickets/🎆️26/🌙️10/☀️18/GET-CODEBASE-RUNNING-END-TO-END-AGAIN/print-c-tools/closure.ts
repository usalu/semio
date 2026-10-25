import { build } from "esbuild";
const root = "/Users/ueli/Documents/semio";
for (const e of process.argv.slice(2)) {
  const r = await build({ absWorkingDir: root, entryPoints: [e], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  console.log(Object.keys(r.metafile!.inputs).length, e.split("/").slice(-5).join("/"));
}
