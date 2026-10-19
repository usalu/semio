import { readFileSync } from "node:fs";
import { decodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import { normalizeCatalogDescriptorEnums } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🛡️guards/🟦️.ts";
const dirs = process.argv.slice(2);
const diff = (a: any, b: any, path: string, out: string[]) => {
  if (out.length > 8) return;
  if (typeof a !== typeof b || (a === null) !== (b === null) || Array.isArray(a) !== Array.isArray(b)) { out.push(path + " type " + JSON.stringify(a)?.slice(0, 80) + " vs " + JSON.stringify(b)?.slice(0, 80)); return; }
  if (a !== null && typeof a === "object") {
    const keys = new Set([...Object.keys(a), ...Object.keys(b)]);
    for (const k of keys) { if (!(k in a)) out.push(path + "/" + k + " missing in json, pack=" + JSON.stringify(b[k])?.slice(0, 100)); else if (!(k in b)) out.push(path + "/" + k + " missing in pack, json=" + JSON.stringify(a[k])?.slice(0, 100)); else diff(a[k], b[k], path + "/" + k, out); }
    return;
  }
  if (a !== b) out.push(path + " " + JSON.stringify(a)?.slice(0, 80) + " vs " + JSON.stringify(b)?.slice(0, 80));
};
for (const d of dirs) {
  const json = JSON.parse(readFileSync(d + "/🔣️.json", "utf8"));
  const pack = packValueToExactJson(decodePackValue(readFileSync(d + "/🛂️.descriptor.semio")));
  const out: string[] = [];
  diff(normalizeCatalogDescriptorEnums(json), normalizeCatalogDescriptorEnums(pack), "", out);
  console.log("[DEBUG]", d.split("/").slice(-2).join("/"), out.length ? out.join(" || ") : "EQUAL");
}
