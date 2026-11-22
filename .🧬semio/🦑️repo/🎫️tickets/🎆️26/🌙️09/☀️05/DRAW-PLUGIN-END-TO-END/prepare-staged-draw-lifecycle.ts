/** 🔬️ Assembles the retained Draw transpilation for an independent close probe. */
import { constants } from "node:fs";
import { copyFile, cp, mkdir, writeFile } from "node:fs/promises";
import { createReadStream } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";

const [stage, output] = Bun.argv.slice(2);
if (!stage || !output) throw Error("Expected a captured transpile stage and the Draw ticket output");
const root = process.cwd();
const browserBundle = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle");
const materialization = join(browserBundle, "🏗️materialization");
const { rewritePreview2ShimImports } = await import(join(materialization, "🟦️.ts"));
const { hostShimSource } = await import(join(materialization, "🪢️host-shim-source/🟦️.ts"));
const name = "semio_hub_draw_component";
const source = join(stage, name + ".core.wasm");
const vendor = join(output, "vendor");
const dest = join(output, "module");
await mkdir(dest, { recursive: true });
await cp(join(root, ".cache/b/p/m/browser/prototype/modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim"), vendor, { recursive: true });
await copyFile(source, join(dest, name + ".core.wasm"), constants.COPYFILE_FICLONE);
await copyFile(join(stage, name + ".js"), join(dest, name + ".js"), constants.COPYFILE_FICLONE);
await writeFile(join(dest, "🟨️.js"), hostShimSource());
rewritePreview2ShimImports(join(dest, name + ".js"), vendor);
const hash = createHash("sha256");
for await (const part of createReadStream(source)) hash.update(part);
console.log("[DEBUG] Staged Draw core", hash.digest("hex"), stage);
