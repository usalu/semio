import { existsSync, readdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";

const root = "/Users/ueli/Documents/semio";
const args = process.argv.slice(2);
const apply = args.includes("--apply");
const [oldRel, newName] = args.filter((a) => !a.startsWith("--"));
if (!oldRel || !newName) throw new Error("usage: movedir.ts <old dir relative to repo> <new leaf name> [--apply]");
const oldDir = resolve(root, oldRel);
const oldName = basename(oldDir);
const newDir = join(dirname(oldDir), newName);
if (!existsSync(oldDir)) throw new Error(`missing ${oldDir}`);
if (existsSync(newDir)) throw new Error(`target exists ${newDir}`);
const skip = new Set(["node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache", "bazel-out", "temp", "🎫️tickets", "🎯️goals", "🧊️archive"]);
const exts = /\.(ts|tsx|mts|cts|json|jsonc|mjs|cjs|js|rs|toml|go|md|py|sh|yml|yaml|bzl|bazel|proto|graphql|sql|txt|feature|ebnf|g4|html|css|svelte|lock)$/u;
const named = new Set(["BUILD", "Dockerfile", "CODEOWNERS", "MODULE.bazel", "go.work"]);
const scanRoots = ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", "🎓️teaching", ".vscode", ".github", ".devcontainer", ".cargo", ".🧬semio/🦑️repo/🔨️modules"];
const pathChar = (c: string) => !/[\s"'`(),;=<>|{}\[\]]/u.test(c);
const files: string[] = [];
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (exts.test(e.name) || named.has(e.name)) files.push(p);
  }
};
for (const r of scanRoots) { const p = join(root, r); if (existsSync(p)) walk(p); }
for (const f of readdirSync(root, { withFileTypes: true })) if (f.isFile() && (exts.test(f.name) || named.has(f.name))) files.push(join(root, f.name));
const inside = (p: string) => p === oldDir || p.startsWith(oldDir + "/");
const changes: { file: string; count: number }[] = [];
const residuals: string[] = [];
const edits = new Map<string, string>();
for (const f of files) {
  let t: string; try { if (statSync(f).size > 12_000_000) continue; t = readFileSync(f, "utf8"); } catch { continue; }
  if (!t.includes(oldName)) continue;
  const fdir = dirname(f);
  let out = "", last = 0, count = 0, from = 0, resid = 0;
  for (;;) {
    const i = t.indexOf(oldName, from);
    if (i < 0) break;
    from = i + oldName.length;
    const before = i === 0 ? "" : t[i - 1]!;
    const after = t[i + oldName.length] ?? "";
    const okBefore = i === 0 || before === "/" || !pathChar(before);
    const okAfter = after === "" || after === "/" || !pathChar(after) || after === ".";
    if (!okBefore || !okAfter) { continue; }
    let j = i;
    while (j > 0 && pathChar(t[j - 1]!)) j--;
    const prefix = t.slice(j, i);
    const ancestors: string[] = [];
    for (let a = fdir; a.startsWith(root) && a.length >= root.length; a = dirname(a)) { ancestors.push(a); if (a === root) break; }
    const candidates = prefix.startsWith("/") ? [resolve(prefix + oldName), resolve(root, "." + prefix + oldName)] : [...ancestors.map((a) => resolve(a, prefix + oldName)), resolve(root, prefix + oldName)];
    let ok = false;
    for (const c of candidates) { if (c === oldDir) { ok = true; break; } }
    if (ok) { out += t.slice(last, i) + newName; last = i + oldName.length; count++; }
    else { resid++; const ln = t.slice(0, i).split("\n").length; if (residuals.length < 400) residuals.push(`${f.slice(root.length + 1)}:${ln}  ${t.slice(Math.max(0, i - 50), i + oldName.length + 20).replace(/\n/g, " ")}`); }
  }
  if (count) { out += t.slice(last); edits.set(f, out); changes.push({ file: f, count }); }
}
console.log(`dir ${oldDir.slice(root.length + 1)} -> ${newName}`);
console.log(`files to rewrite: ${changes.length}`);
for (const c of changes.slice(0, 80)) console.log(`  ${c.file.slice(root.length + 1)} x${c.count}`);
if (changes.length > 80) console.log(`  … ${changes.length - 80} more`);
console.log(`residual unresolved mentions: ${residuals.length}`);
for (const r of residuals.slice(0, 60)) console.log("  ?", r);
if (apply) {
  for (const [f, t] of edits) writeFileSync(f, t);
  renameSync(oldDir, newDir);
  console.log("renamed and rewritten");
}
