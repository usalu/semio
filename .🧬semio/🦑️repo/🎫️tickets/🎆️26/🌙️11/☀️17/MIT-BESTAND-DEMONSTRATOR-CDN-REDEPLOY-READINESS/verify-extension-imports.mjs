import { readFile } from "node:fs/promises";
import { dirname, join, relative, sep } from "node:path";
import { PREVIEW2_VENDOR_RELATIVE, rewritePreview2ShimImportSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";

const [staged, deployed, ...files] = process.argv.slice(2);
const vendor = join(dirname(dirname(deployed)), "🔌️plugin-modules", PREVIEW2_VENDOR_RELATIVE);
for (const name of files) {
  const output = join(deployed, name);
  const prefix = relative(dirname(output), vendor).split(sep).join("/") + "/";
  const expected = rewritePreview2ShimImportSource(await readFile(join(staged, name), "utf8"), prefix);
  if (!Buffer.from(expected).equals(await readFile(output))) {
    console.error("Deployed extension script differs from its distributed source: " + output);
    process.exitCode = 1;
  }
}
