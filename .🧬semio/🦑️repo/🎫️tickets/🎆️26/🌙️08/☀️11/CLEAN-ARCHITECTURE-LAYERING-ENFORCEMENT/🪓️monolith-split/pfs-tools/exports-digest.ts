// usage: bun exports-digest.ts <out.tsv> <module>...   one row per export: name, typeof, sha1 of the function source (or of the JSON value)
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
const [out, ...modules] = process.argv.slice(2);
const sha = (text: string) => createHash("sha1").update(text).digest("hex").slice(0, 12);
const rows: string[] = [];
const seen = new Map<string, string>();
for (const path of modules) {
  const mod = (await import(path)) as Record<string, unknown>;
  for (const [name, value] of Object.entries(mod)) {
    const digest = typeof value === "function" ? sha(Function.prototype.toString.call(value)) : sha(JSON.stringify(value, (_k, v) => (typeof v === "bigint" ? String(v) : v)) ?? "undefined");
    if (seen.has(name)) throw new Error(`export ${name} defined by two modules: ${seen.get(name)} and ${path}`);
    seen.set(name, path);
    rows.push(`${name}\t${typeof value}\t${digest}`);
  }
}
writeFileSync(out, rows.sort().join("\n") + "\n");
console.log(`exports=${rows.length}`);
