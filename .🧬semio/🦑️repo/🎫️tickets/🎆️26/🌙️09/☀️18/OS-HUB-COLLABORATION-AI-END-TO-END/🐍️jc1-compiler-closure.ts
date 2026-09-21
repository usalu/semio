/** 📚️ Lists the exact module closure the browser codegen compiler capsule captures, per admitted root. */
import { dirname, join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { realpathSync } from "node:fs";

const entrypoint = fileURLToPath(import.meta.resolve("@bytecodealliance/jco/component"));
const vendor = fileURLToPath(import.meta.resolve("@bytecodealliance/jco-transpile/component"));
const shim = dirname(dirname(fileURLToPath(import.meta.resolve("@bytecodealliance/preview2-shim/io"))));
const roots = [
  { name: "@bytecodealliance/jco/dist", path: realpathSync(dirname(entrypoint)) },
  { name: "@bytecodealliance/jco-transpile/vendor", path: realpathSync(dirname(vendor)) },
  { name: "@bytecodealliance/preview2-shim/dist/browser", path: realpathSync(join(shim, "browser")) },
];
const result = await Bun.build({ entrypoints: [entrypoint], root: roots[0].path, target: "browser", format: "esm", splitting: false, minify: false, metafile: true, external: ["node:fs/promises"] });
if (!result.success || !result.metafile) throw new Error("build failed: " + JSON.stringify(result.logs));
const rows = Object.keys(result.metafile.inputs).map(path => {
  const resolved = realpathSync(path.startsWith("/") ? path : join(process.cwd(), path));
  const root = roots.find(root => resolved.startsWith(root.path + sep));
  return { logicalPath: root ? root.name + "/" + relative(root.path, resolved).split(sep).join("/") : "OUTSIDE:" + resolved, admitted: Boolean(root) };
}).sort((left, right) => left.logicalPath < right.logicalPath ? -1 : 1);
console.log(JSON.stringify({ count: rows.length, outputBytes: result.outputs[0].size, admittedRoots: roots.map(root => root.name), rows }, null, 2));
