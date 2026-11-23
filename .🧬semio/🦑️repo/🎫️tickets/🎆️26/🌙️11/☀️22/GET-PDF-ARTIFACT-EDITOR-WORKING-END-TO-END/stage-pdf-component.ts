import { cp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { join } from "node:path";

/** 📦️ Stages a built PDF plugin component for the native dev host with the browser-bundle pipeline's own transpile steps (native transpile, async-result and asset-URL rewrites, preview2 shim relocation).
 * Usage: `bun stage-pdf-component.ts <component.wasm> <work-dir>` from the workspace root.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const [component, work] = process.argv.slice(2) as [string, string];
const base = "semio_hub_stdio_pdf_component";
const modules = join(root, ".cache/b/p/m/browser/prototype/modules");
const stage = join(modules, "📘️stdio-pdf");
const materialization = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization");
const { transpilePluginComponentModulesAsync, rewritePreview2ShimImports } = await import(join(materialization, "🟦️.ts"));
const stripped = join(work, "component.wasm");
const out = join(work, "out");
await rm(work, { recursive: true, force: true });
await mkdir(out, { recursive: true });
const proc = Bun.spawnSync(["node", join(import.meta.dir, "strip-component-names.mjs"), component, stripped], { stdout: "inherit", stderr: "inherit" });
if (proc.exitCode !== 0) throw new Error("strip failed");
await transpilePluginComponentModulesAsync(stripped, out, base, { repoRoot: root, mode: "prototype", environment: process.env, preview2VendorDir: join(modules, "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim") });
for (const file of [`${base}.core.wasm`, `${base}.js`, `${base}.d.ts`]) await cp(join(out, file), join(stage, file));
rewritePreview2ShimImports(join(stage, `${base}.js`), join(modules, "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim"));
await cp(join(out, "interfaces"), join(stage, "interfaces"), { recursive: true });
const bytes = await readFile(stripped);
const receiptPath = join(stage, "🧾️receipt.json");
const receipt = JSON.parse(await readFile(receiptPath, "utf8"));
receipt.wasmSha256 = createHash("sha256").update(bytes).digest("hex");
receipt.wasmBytes = bytes.byteLength;
await writeFile(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
console.log(`staged ${stage} (${bytes.byteLength} bytes, sha256 ${receipt.wasmSha256})`);
