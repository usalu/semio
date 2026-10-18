import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
const disc = await import(P + "/🔎️discovery/🟦️.ts");
const ver = await import(P + "/✅️catalog-verification/🟦️.ts");
const root = "/Users/ueli/Documents/semio";
for (const manifest of disc.findPluginCargoFiles(root)) {
  const src = disc.parseComponentSourceOwnerV1(manifest, root);
  if (!/writer/.test(src.pluginId)) continue;
  const path = resolve(root, src.cratePath, ...disc.DESCRIPTOR_JSON_REL_PATH);
  console.log(src.pluginId, path);
  try { ver.verifySemanticDescriptorPairBytesV1(readFileSync(path), readFileSync(join(dirname(path), "🛂️.descriptor.semio"))); console.log("OK"); } catch (e) { console.log(String(e).slice(0, 200)); }
}
