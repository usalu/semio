import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
const disc = await import(P + "/🔎️discovery/🟦️.ts");
const ver = await import(P + "/✅️catalog-verification/🟦️.ts");
const root = "/Users/ueli/Documents/semio";
let n = 0;
for (const manifest of disc.findPluginCargoFiles(root)) {
  const src = disc.parseComponentSourceOwnerV1(manifest, root);
  const path = resolve(root, src.cratePath, ...disc.DESCRIPTOR_JSON_REL_PATH);
  
  try { ver.verifySemanticDescriptorPairBytesV1(readFileSync(path), readFileSync(join(dirname(path), "🛂️.descriptor.semio"))); n++; } catch (e) { console.log("MISMATCH", src.pluginId, String(e).slice(0, 120), path.slice(root.length)); }
}

console.log("ok", n);
