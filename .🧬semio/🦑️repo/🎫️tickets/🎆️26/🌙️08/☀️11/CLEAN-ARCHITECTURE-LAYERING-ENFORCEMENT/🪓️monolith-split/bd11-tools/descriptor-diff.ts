import { readFileSync } from "node:fs";
import { isDeepStrictEqual } from "node:util";
const P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/";
const ver = await import(P + "🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts");
const pack = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts");
const dir = "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/" + process.argv[2] + "/🧩️component/";
const json = JSON.parse(readFileSync(dir + "🔣️.json", "utf8"));
const packed = pack.decodePackValue(readFileSync(dir + "🛂️.descriptor.semio"));
const a = ver.normalizeCatalogDescriptorEnums(json), b = ver.normalizeCatalogDescriptorEnums(pack.packValueToExactJson(packed));
const out: string[] = [];
function walk(x: any, y: any, path: string) {
  if (isDeepStrictEqual(x, y)) return;
  if (x && y && typeof x === "object" && typeof y === "object") {
    for (const k of new Set([...Object.keys(x), ...Object.keys(y)])) { if (out.length < 12) walk(x[k], y[k], path + "/" + k); }
  } else out.push(path + " json=" + JSON.stringify(x)?.slice(0, 80) + " pack=" + JSON.stringify(y)?.slice(0, 80));
}
walk(a, b, "");
console.log(out.join("\n"));
