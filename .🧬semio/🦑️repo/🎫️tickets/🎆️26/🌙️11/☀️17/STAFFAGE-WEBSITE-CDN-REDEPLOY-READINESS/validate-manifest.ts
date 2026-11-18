/** 📋️ Validates the manifest against every catalog asset with `validateManifest` and prints every issue. */
import { Glob } from "bun";
import { join, resolve } from "node:path";
import { validateManifest } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const manifest = await Bun.file(join(root, "🔣️.json")).json();
const assets = [];
for (const path of [...new Glob("*/*/🔣️.json").scanSync(root)].sort()) assets.push((await Bun.file(join(root, path)).json()).asset);
const issues = validateManifest(manifest, assets);
for (const issue of issues) console.log(`${issue.path}\t${issue.code}\t${issue.message.en}`);
console.log(`manifest issues ${issues.length}`);
