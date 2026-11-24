import { copyFileSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { ensurePreview2ShimVendorAt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🩹️preview2-shim-patches/🟦️.ts";
import { SHARD_WORKER_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import { shardWorkerSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts";
import { MODULE_SHARD_DIRECTORY } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { GUESTSLIM_FONT_RELATIVE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import { fontAssetPublicationRootV1 } from "../../../../../../../🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️publication/🟦️.ts";
import { FONT_ASSET } from "../../../../../../../🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/🟦️.ts";
import { ensureGuestSlimTypstFontsAt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🔤️guestslim-fonts/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";

/** 🧺️ Native support stager: publishes the preview2 vendor shims, the shard worker and the guest slim typst fonts (module root and the dev server's font publication root) into the prototype plugin-modules root exactly as the build driver's `support` stage does, without the repo graph.
 * Usage: `[FONT_PACKER=<pack-typst-font-assets>] bun native-stage-support.ts` from the workspace root before `native-stage-modules.ts`; the packer runs when no font seed is staged.
 * @see ./native-stage-modules.ts */

const root = process.cwd(), modules = join(root, ".cache/b/p/m/browser/prototype/modules");
const vendor = join(modules, PREVIEW2_VENDOR_RELATIVE), shard = join(modules, MODULE_SHARD_DIRECTORY);
rmSync(vendor, { recursive: true, force: true }); rmSync(shard, { recursive: true, force: true });
mkdirSync(vendor, { recursive: true }); mkdirSync(shard, { recursive: true });
ensurePreview2ShimVendorAt(vendor, root);
writeFileSync(join(shard, SHARD_WORKER_FILE), shardWorkerSource());
const packer = process.env.FONT_PACKER ?? join(import.meta.dir, "🗑️generated/native-root/target/release/pack-typst-font-assets");
if (!ensureGuestSlimTypstFontsAt(modules, root) && spawnSync(packer, [join(modules, GUESTSLIM_FONT_RELATIVE)], { stdio: "inherit" }).status !== 0) throw Error(`guest slim typst fonts unavailable: no staged seed and packer ${packer} failed`);
const fonts = fontAssetPublicationRootV1(join(root, ".cache/b/p/m"), "prototype");
mkdirSync(fonts, { recursive: true }); copyFileSync(join(modules, GUESTSLIM_FONT_RELATIVE), join(fonts, FONT_ASSET));
console.log(`support staged: ${vendor}, ${shard}, ${fonts}`);
