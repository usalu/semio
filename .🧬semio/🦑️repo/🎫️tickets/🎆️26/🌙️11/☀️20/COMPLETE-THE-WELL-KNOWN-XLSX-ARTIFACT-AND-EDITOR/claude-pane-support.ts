import { cpSync, existsSync, mkdirSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
/** 🛠️ [DEBUG] Native `support` target: republishes the preview2 vendor shims and the shard worker of the prototype plugin-modules root from the current sources, as `SupportScript` does. */
const root = "/Users/ueli/Documents/semio";
const bundle = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle");
const m = join(bundle, "🏗️materialization");
const { ensurePreview2ShimVendorAt } = await import(join(m, "🩹️preview2-shim-patches/🟦️.ts"));
const { shardWorkerSource } = await import(join(m, "🧵️shard-worker-source/🟦️.ts"));
const { SHARD_WORKER_FILE } = await import(join(m, "🧾️contract/🟦️.ts"));
const { PREVIEW2_VENDOR_RELATIVE } = await import(join(bundle, "🕸️imports/🟦️.ts"));
const { MODULE_SHARD_DIRECTORY } = await import("@semio-tech/plugin-registry/deployment");
const modules = join(root, ".cache/b/p/m/browser/prototype/modules");
const stage = join(root, ".cache/b/p/m/tmp/claude-pane-support");
rmSync(stage, { recursive: true, force: true });
const vendor = join(stage, "vendor"), shard = join(stage, "shard");
mkdirSync(shard, { recursive: true });
ensurePreview2ShimVendorAt(vendor, root);
writeFileSync(join(shard, SHARD_WORKER_FILE), shardWorkerSource());
const walk = (d: string, p = ""): string[] => readdirSync(d, { withFileTypes: true }).flatMap(e => e.isDirectory() ? walk(join(d, e.name), `${p}${e.name}/`) : [`${p}${e.name}`]).sort();
for (const [relative, owner, source] of [[PREVIEW2_VENDOR_RELATIVE, "browser-support:preview2", vendor], [MODULE_SHARD_DIRECTORY, "browser-support:shard", shard]] as const) {
  const target = join(modules, relative);
  rmSync(target, { recursive: true, force: true });
  mkdirSync(target, { recursive: true });
  cpSync(source, target, { recursive: true });
  writeFileSync(join(target, ".semio-artifact.json"), JSON.stringify({ version: 1, owner, files: walk(source) }));
  console.log(`[DEBUG] published ${relative}`);
}
rmSync(stage, { recursive: true, force: true });
