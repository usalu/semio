/** Z2 probe: gitignored inputs in each entry's esbuild bundle closure (what a fresh clone lacks when the entry loads). */
import { spawnSync } from "node:child_process";
import { relative, resolve } from "node:path";
import { build, type Plugin } from "esbuild";
import { realpathSync } from "node:fs";
const root = process.cwd();
const workspacePackages: Plugin = { name: "workspace-packages", setup(builder) {
  builder.onResolve({ filter: /^[^./]/ }, (args) => {
    if (args.path.startsWith("node:")) return { path: args.path, external: true };
    try { const real = realpathSync(Bun.resolveSync(args.path, args.resolveDir)); return real.includes("/node_modules/") ? { path: args.path, external: true } : { path: real }; }
    catch { return { path: args.path, external: true }; }
  });
} };
for (const entry of process.argv.slice(2)) {
  const result = await build({ absWorkingDir: root, entryPoints: [entry], bundle: true, write: false, metafile: true, plugins: [workspacePackages], platform: "node", format: "esm", logLevel: "silent", loader: { ".node": "empty", ".wasm": "empty", ".css": "empty", ".svg": "empty", ".png": "empty" } }).catch((error) => ({ error }));
  if ("error" in result) { console.log(`== ${entry}\n  BUILD ERROR ${String(result.error).slice(0, 300)}`); continue; }
  const inputs = Object.keys(result.metafile.inputs).map((path) => relative(root, resolve(root, path)));
  const ignored = spawnSync("git", ["check-ignore", "--stdin"], { cwd: root, input: inputs.join("\n") + "\n", encoding: "utf8" }).stdout.split("\n").filter(Boolean);
  console.log(`== ${entry} inputs=${inputs.length} ignored=${ignored.length}`);
  for (const path of ignored) console.log(`  ${path}`);
}
