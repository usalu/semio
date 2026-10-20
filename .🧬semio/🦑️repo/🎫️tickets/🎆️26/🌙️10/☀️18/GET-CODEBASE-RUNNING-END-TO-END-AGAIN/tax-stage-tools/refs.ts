// 🔎 Resolves every quoted path reference to one of the listed scripts. usage: bun refs.ts <files.txt> <targets.txt> <out.json>
import {readFileSync, writeFileSync, existsSync} from "node:fs";
import {dirname, join, resolve, relative} from "node:path";
const REPO = "/Users/ueli/Documents/semio";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const targets = new Set(readFileSync(process.argv[3]!, "utf8").split("\n").filter(Boolean));
const out: Record<string, {file: string; line: number; spec: string; kind: string}[]> = {};
const byBase = new Map<string, string[]>();
for (const t of targets) { const parent = t.split("/").slice(-2).join("/"); (byBase.get(parent) ?? byBase.set(parent, []).get(parent)!).push(t); }
let scanned = 0;
for (const f of files) {
  let text: string; try { text = readFileSync(join(REPO, f), "utf8"); } catch { continue; }
  scanned++;
  const lines = text.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]!;
    if (!line.includes("📜️script.ts")) continue;
    for (const m of line.matchAll(/(["'`])((?:\\.|(?!\1)[^\n\\])*?📜️script\.ts)\1/gu)) {
      const spec = m[2]!;
      let abs: string | null = null;
      if (spec.startsWith(".")) abs = relative(REPO, resolve(dirname(join(REPO, f)), spec));
      else {
        const sibling = relative(REPO, resolve(dirname(join(REPO, f)), spec));
        abs = sibling;
        if (!byBase.has(sibling.split("/").slice(-2).join("/")) || ![...(byBase.get(sibling.split("/").slice(-2).join("/")) ?? [])].includes(sibling)) abs = spec.includes("/") ? spec.replace(/^\/Users\/ueli\/Documents\/semio\//, "") : null;
      }
      if (!abs) continue;
      const tail = abs.split("/").slice(-2).join("/");
      for (const t of byBase.get(tail) ?? []) if (t === abs || abs.endsWith(t)) (out[t] ??= []).push({file: f, line: i + 1, spec, kind: spec.startsWith(".") ? "relative" : "path"});
    }
  }
}
writeFileSync(process.argv[4]!, JSON.stringify(out, null, 1));
console.error(`scanned ${scanned} files; targets with refs: ${Object.keys(out).length}/${targets.size}`);
