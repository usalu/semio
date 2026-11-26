import { readFile, readdir } from "node:fs/promises";
import { basename, dirname, join, relative, sep } from "node:path";
import { PREVIEW2_VENDOR_RELATIVE, rewritePreview2ShimImportSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";

import { browserCoreName, gzipCoreLoader, gzipCoreSuffix, isBrowserCoreFactory } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🗜️cores/🟦️.ts";

const [staged, deployed, ...files] = process.argv.slice(2);
const vendor = join(dirname(dirname(deployed)), "🔌️plugin-modules", PREVIEW2_VENDOR_RELATIVE);
let usesShardGzip = false;
for (const name of files) {
  const output = join(deployed, name);
  const prefix = relative(dirname(output), vendor).split(sep).join("/") + "/";
  let expected = await readFile(join(staged, name), "utf8");
  if (basename(dirname(deployed)) === "🧩️extension-modules") expected = rewritePreview2ShimImportSource(expected, prefix);
  const cores = (await readdir(dirname(output))).filter(file => file.endsWith(gzipCoreSuffix))
    .map(file => file.slice(0, -gzipCoreSuffix.length)).filter(core => core.replace(browserCoreName, ".js") === basename(name));
  if (cores.length) {
    usesShardGzip ||= isBrowserCoreFactory(expected);
    expected = gzipCoreLoader(expected, cores);
  }
  if (!Buffer.from(expected).equals(await readFile(output))) {
    console.error("Deployed binding differs from its distributed source: " + output);
    process.exitCode = 1;
  }
}
if (!process.exitCode) console.log(JSON.stringify(usesShardGzip));
