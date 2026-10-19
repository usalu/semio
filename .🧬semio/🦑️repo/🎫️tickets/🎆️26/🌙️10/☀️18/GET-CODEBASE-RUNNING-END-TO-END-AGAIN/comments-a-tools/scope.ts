import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname } from "node:path";
import { initParser, findWasm } from "../rs-sweep-tools/parse.ts";
import { buildTree } from "../rs-sweep-tools/tree.ts";
import { loadPackage } from "../rs-sweep-tools/cargo.ts";
const ROOT = "/Users/ueli/Documents/semio/";
await initParser(findWasm(ROOT)!);
const batches = (process.argv[2] ?? "modules-rs,os-rs,rest-rs").split(",");
const files = new Set(batches.flatMap((b) => readFileSync(`batch-${b}.txt`, "utf8").split("\n").filter(Boolean)).map((f) => ROOT + f));
const all = readFileSync("all.nul", "utf8").split("\0").filter((f) => f.endsWith("Cargo.toml") && !f.includes("📚️library") && !f.includes("🎫️tickets"));
const ms = new Set<string>();
const covered = new Set<string>();
for (const m of all) {
  const p = loadPackage(ROOT + dirname(m));
  if (!p) continue;
  for (const t of p.targets) {
    if (!t.path || !existsSync(t.path) || t.kind === "build") continue;
    let tr;
    try { tr = buildTree(p, t, p.name); } catch { continue; }
    for (const f of tr.files) if (files.has(f)) { ms.add(m); covered.add(f); }
  }
}
writeFileSync("scope.txt", [...ms].join("\n") + "\n");
console.log(ms.size, "manifests cover", covered.size, "of", files.size, "files");
const un = [...files].filter((f) => !covered.has(f));
writeFileSync("scope-unmapped.txt", un.join("\n") + "\n");
console.log(un.slice(0, 5));
