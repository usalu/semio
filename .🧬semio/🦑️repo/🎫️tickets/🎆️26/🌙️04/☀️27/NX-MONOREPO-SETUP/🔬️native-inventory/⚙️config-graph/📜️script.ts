import { createRequire } from "node:module";
import { join } from "node:path";
import { writeFileSync } from "node:fs";
const root = process.env.SEMIO_REPO_ROOT!, require = createRequire(join(root, "package.json"));
const { build } = require("esbuild");
const entry = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts";
const result = await build({ entryPoints: [entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: root, plugins: [{ name: "external-packages", setup(builder: any) { builder.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }: any) => kind === "entry-point" || path.startsWith("/") ? null : { external: true }); } }] });
writeFileSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "selected-vite-config-graph.json"), JSON.stringify(result.metafile, null, 2));
const paths = new Map<string, string[]>([[entry, [entry]]]), queue = [entry];
for (const name of queue) for (const dependency of result.metafile.inputs[name]?.imports ?? []) {
  if (dependency.external || paths.has(dependency.path)) continue;
  paths.set(dependency.path, [...paths.get(name)!, dependency.path]); queue.push(dependency.path);
}
console.log(JSON.stringify([...paths].filter(([name]) => name.includes("/🔍️discovery/") || name.includes("/🧪️tests/") || name.includes("/🚀️bootstrap/")), null, 2));
