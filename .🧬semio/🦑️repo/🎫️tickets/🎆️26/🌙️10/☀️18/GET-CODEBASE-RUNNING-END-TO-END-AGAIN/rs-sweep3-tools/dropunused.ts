import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const [findingsFile, apply] = process.argv.slice(2);
const finds = readFileSync(findingsFile, "utf8").trim().split("\n").map((x) => JSON.parse(x)).filter((x) => x.kind === "unresolved-import" && /^use `/.test(x.msg));
const byFile = new Map<string, Map<number, any>>();
for (const f of finds) {
  const m = byFile.get(f.file) ?? new Map();
  m.set(f.line, f);
  byFile.set(f.file, m);
}
function rsFiles(dir: string, depth: number, out: string[]): void {
  let ents: string[] = [];
  try { ents = readdirSync(dir); } catch { return; }
  for (const e of ents) {
    const p = join(dir, e);
    let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) { if (depth > 0 && !e.startsWith(".") && e !== "node_modules") rsFiles(p, depth - 1, out); }
    else if (e.endsWith(".rs")) out.push(p);
  }
}
let removed = 0, kept = 0;
for (const [rel, lines] of byFile) {
  const file = join(ROOT, rel);
  const src = readFileSync(file, "utf8").split("\n");
  const near: string[] = [];
  rsFiles(dirname(file), 2, near);
  rsFiles(dirname(dirname(file)), 0, near);
  const texts = near.filter((p) => p !== file).map((p) => readFileSync(p, "utf8"));
  const drop = new Set<number>();
  for (const [ln, f] of lines) {
    const line = src[ln - 1] ?? "";
    const m = /^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;{}*]+?)(?:\s+as\s+(\w+))?\s*;\s*$/.exec(line);
    if (!m) { kept++; continue; }
    const name = m[2] ?? m[1].split("::").pop()!.trim();
    if (!/^[A-Za-z_]\w*$/.test(name)) { kept++; continue; }
    const re = new RegExp(`(?<![\\w:])${name}(?!\\w)`);
    const used = src.some((l, i) => i !== ln - 1 && !/^\s*(pub(\([^)]*\))?\s+)?use\s/.test(l) && !/^\s*\/\//.test(l) && re.test(l)) || texts.some((t) => re.test(t.replace(/^\s*\/\/.*$/gm, "")));
    if (used) { kept++; continue; }
    drop.add(ln - 1);
  }
  if (!drop.size) continue;
  removed += drop.size;
  console.log(String(drop.size).padStart(3), rel.split("/").slice(-4).join("/"));
  if (apply === "apply") writeFileSync(file, src.filter((_, i) => !drop.has(i)).join("\n"));
}
console.log({ removed, kept });
