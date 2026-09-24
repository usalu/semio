/** 📐️ Measures the vite config's static import graph: module count, source bytes and who imports whom. */
import { build } from "esbuild";
import { relative, resolve } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const entry = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts");
const externalizeDeps = {
  name: "externalize-deps",
  setup(build: any) {
    build.onResolve({ filter: /.*/ }, (args: any) => (args.kind === "entry-point" || args.path.startsWith(".") || args.path.startsWith("/") ? undefined : { external: true }));
  },
};
const result = await build({ entryPoints: [entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: repoRoot, plugins: [externalizeDeps as any] });
const rel = (p: string) => relative(repoRoot, resolve(repoRoot, p)).replaceAll("\\", "/");
const inputs = result.metafile!.inputs;
const rows = Object.entries(inputs).map(([k, v]) => ({ module: rel(k), bytes: (v as any).bytes }));
console.log("modules", rows.length, "sourceBytes", rows.reduce((t, r) => t + r.bytes, 0));
const importers = new Map<string, string[]>();
for (const [k, v] of Object.entries(inputs)) for (const imp of (v as any).imports) {
  if (!imp.path || imp.external) continue;
  const list = importers.get(rel(imp.path)) ?? []; list.push(rel(k) + " [" + imp.kind + "]"); importers.set(rel(imp.path), list);
}
for (const row of rows.sort((a, b) => b.bytes - a.bytes)) console.log(String(row.bytes).padStart(8), row.module, "<=", (importers.get(row.module) ?? ["(entry)"]).join(" | "));
