import { readFileSync } from "node:fs";
import { decodePackValue, packValueToExactJson } from "@semio-tech/framework/packs";
const dir = process.argv[2]!;
const j = JSON.parse(readFileSync(dir + "/🔣️.json", "utf8")), p = packValueToExactJson(decodePackValue(readFileSync(dir + "/🛂️.descriptor.semio")));
function diff(a: any, b: any, path: string, out: string[]) {
  if (out.length > 12) return;
  if (JSON.stringify(a) === JSON.stringify(b)) return;
  if (a && b && typeof a === "object" && typeof b === "object" && Array.isArray(a) === Array.isArray(b)) {
    const keys = new Set([...Object.keys(a), ...Object.keys(b)]);
    for (const k of keys) diff(a[k], b[k], path + "/" + k, out);
    return;
  }
  out.push(path + "  json=" + JSON.stringify(a)?.slice(0, 120) + "  pack=" + JSON.stringify(b)?.slice(0, 120));
}
const out: string[] = []; diff(j, p, "", out); console.log(out.join("\n"));
