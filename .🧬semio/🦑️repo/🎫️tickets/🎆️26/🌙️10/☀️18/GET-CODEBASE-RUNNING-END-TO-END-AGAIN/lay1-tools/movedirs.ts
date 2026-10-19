import { existsSync, readdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";

const root = "/Users/ueli/Documents/semio";
const args = process.argv.slice(2);
const apply = args.includes("--apply");
const listFile = args.find((a) => !a.startsWith("--"))!;
type Ren = { oldDir: string; oldName: string; newName: string; newDir: string };
const rens: Ren[] = readFileSync(listFile, "utf8").split("\n").filter(Boolean).map((l) => {
  const [rel, newName] = l.split("|");
  const oldDir = resolve(root, rel!);
  if (!existsSync(oldDir)) throw new Error(`missing ${oldDir}`);
  return { oldDir, oldName: basename(oldDir), newName: newName!, newDir: join(dirname(oldDir), newName!) };
});
const byName = new Map<string, Ren[]>();
for (const r of rens) { const a = byName.get(r.oldName) ?? []; a.push(r); byName.set(r.oldName, a); }
const names = [...byName.keys()];
const skip = new Set(["captures", "node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache", "bazel-out", "temp", "🎫️tickets", "🎯️goals", "🧊️archive"]);
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
const edits = new Map<string, string>();
const perRen = new Map<Ren, number>();
const residuals: string[] = [];
for (const f of files) {
  let t: string; try { if (statSync(f).size > 12_000_000) continue; t = readFileSync(f, "utf8"); } catch { continue; }
  if (!names.some((n) => t.includes(n))) continue;
  const fdir = dirname(f);
  const ancestors: string[] = [];
  for (let a = fdir; a.startsWith(root); a = dirname(a)) { ancestors.push(a); if (a === root) break; }
  const reps: [number, number, string][] = [];
  for (const name of names) {
    let from = 0;
    for (;;) {
      const i = t.indexOf(name, from);
      if (i < 0) break;
      from = i + name.length;
      const before = i === 0 ? "" : t[i - 1]!;
      const after = t[i + name.length] ?? "";
      if (!(i === 0 || before === "/" || !pathChar(before)) || !(after === "" || after === "/" || !pathChar(after) || after === ".")) continue;
      let j = i;
      while (j > 0 && pathChar(t[j - 1]!)) j--;
      const prefix = t.slice(j, i);
      const cands = prefix.startsWith("/") ? [resolve(prefix + name), resolve(root, "." + prefix + name)] : [...ancestors.map((a) => resolve(a, prefix + name)), resolve(root, prefix + name)];
      if (prefix.includes("/")) { let k = prefix.indexOf("/"); while (k >= 0) { cands.push(resolve(root, prefix.slice(k + 1) + name)); k = prefix.indexOf("/", k + 1); } }
      let hit = byName.get(name)!.find((r) => cands.includes(r.oldDir));
      if (!hit && prefix !== "" && !prefix.split("/").some((seg) => seg === "." || seg === "..")) {
        const tail = prefix.replace(/^\/+/u, "") + name;
        hit = byName.get(name)!.find((r) => r.oldDir.endsWith("/" + tail));
      }
      if (hit) { reps.push([i, i + name.length, hit.newName]); perRen.set(hit, (perRen.get(hit) ?? 0) + 1); }
      else if (prefix !== "" && residuals.length < 3000) { const ln = t.slice(0, i).split("\n").length; residuals.push(`${name}\t${f.slice(root.length + 1)}:${ln}\t${t.slice(Math.max(0, i - 60), i + name.length + 20).replace(/\n/g, " ")}`); }
    }
  }
  if (reps.length && (basename(f) === "package.json" || basename(f) === "bun.lock")) { console.log("FROZEN-FILE-REFERENCE", f.slice(root.length + 1), reps.length); continue; }
  if (reps.length) { reps.sort((a, b) => b[0] - a[0]); let out = t; for (const [a, b, r] of reps) out = out.slice(0, a) + r + out.slice(b); edits.set(f, out); }
}
console.log(`files to rewrite: ${edits.size}`); if (process.env.LIST) for (const f of edits.keys()) console.log("  F", f.slice(root.length + 1));
for (const r of rens) console.log(`  ${r.newName}  <=  ...${r.oldDir.slice(-60).replace(/️/gu, "")}  x${perRen.get(r) ?? 0}`);
writeFileSync(listFile + ".residuals.tsv", residuals.join("\n"));
console.log(`residual unresolved mentions: ${residuals.length} (see ${listFile}.residuals.tsv)`);
if (apply) {
  for (const [f, t] of edits) writeFileSync(f, t);
  for (const r of [...rens].sort((a, b) => b.oldDir.length - a.oldDir.length)) {
    const parentRenamed = rens.find((o) => o !== r && r.oldDir.startsWith(o.oldDir + "/"));
    const cur = parentRenamed ? r.oldDir.replace(parentRenamed.oldDir, parentRenamed.oldDir) : r.oldDir;
    renameSync(cur, r.newDir);
  }
  console.log("renamed and rewritten");
}
