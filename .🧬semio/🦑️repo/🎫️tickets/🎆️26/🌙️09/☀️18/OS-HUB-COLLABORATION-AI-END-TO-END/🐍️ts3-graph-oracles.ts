/** ⚖️ Compares esbuild's PARSED inputs, esbuild's RETAINED output inputs and Bun's sourcemap sources. */
import { build } from "esbuild";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative, resolve } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const entry = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts");
const rel = (p: string) => relative(repoRoot, resolve(repoRoot, p)).replaceAll("\\", "/");
const externalizeDeps = { name: "externalize-deps", setup(b: any) { b.onResolve({ filter: /.*/ }, (a: any) => (a.kind === "entry-point" || a.path.startsWith(".") || a.path.startsWith("/") ? undefined : { external: true })); } };
const result = await build({ entryPoints: [entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: repoRoot, plugins: [externalizeDeps as any] });
const parsed = Object.keys(result.metafile!.inputs).map(rel).sort();
const retained = Object.values(result.metafile!.outputs).flatMap((o: any) => Object.keys(o.inputs)).map(rel).sort();
const outDir = mkdtempSync(join(tmpdir(), "semio-config-graph-"));
const status = spawnSync("bun", ["build", entry, "--target=bun", "--packages=external", "--sourcemap=external", "--outdir", outDir], { cwd: repoRoot, encoding: "utf8" });
if (status.status !== 0) throw new Error(status.stderr || status.stdout);
const map = readdirSync(outDir).find((n) => n.endsWith(".map"))!;
const bun = (JSON.parse(readFileSync(join(outDir, map), "utf8")) as { sources: string[] }).sources.map(rel).sort();
rmSync(outDir, { recursive: true, force: true });
const ts = (list: readonly string[]) => list.filter((m) => /\.[cm]?[jt]sx?$/u.test(m));
console.log("parsed", parsed.length, "parsed-ts", ts(parsed).length, "retained", retained.length, "retained-ts", ts(retained).length, "bun", bun.length);
console.log("retained-ts minus bun:", ts(retained).filter((m) => !bun.includes(m)));
console.log("bun minus retained-ts:", bun.filter((m) => !ts(retained).includes(m)));
