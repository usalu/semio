import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { initParser, findWasm } from "./parse.ts";
import { buildTree, CrateTree, parseStats } from "./tree.ts";
import { Finding, Package, loadPackage } from "./cargo.ts";
import { setTreeBuilder, libTree } from "./resolve.ts";
import { ROOT, checkManifest, checkSource, checkWorkspace, libIndex, regIndex, aliasIndex, rel, setErrorHook, setDupHook, Stats } from "./checks.ts";
import { plan, planDup, applyAll, stats as repointStats, suggestions } from "./repoint.ts";

const args = process.argv.slice(2);
const flag = (n: string): string | undefined => {
  const i = args.indexOf(n);
  return i >= 0 ? args[i + 1] : undefined;
};
const has = (n: string): boolean => args.includes(n);

const C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const G = flag("--out") ?? `${C}/🗑️generated/cargo-fix`;
mkdirSync(G, { recursive: true });
const since = new Date(flag("--since") ?? "2026-10-19T00:00:00").getTime();
const only = flag("--pkg")?.split(",");
const wasm = flag("--wasm") ?? findWasm(ROOT);
if (!wasm) throw new Error("no tree-sitter-rust.wasm found (node_modules/tree-sitter-wasms or the tool caches)");
await initParser(wasm);

const t0 = Date.now();
const lap = (m: string) => console.error(`[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`);

const listFile = `${G}/files.nul`;
if (has("--refresh") || !existsSync(listFile)) {
  const r = Bun.spawnSync(["git", "ls-files", "-z", "-co", "--exclude-standard", "--", "*.rs", "*Cargo.toml"], { cwd: ROOT, maxBuffer: 1 << 30 });
  writeFileSync(listFile, r.stdout);
}
const files = readFileSync(listFile, "utf8").split("\0").filter(Boolean).map((f) => join(ROOT, f));
lap(`listed ${files.length} files`);

const skipDir = (f: string) => f.includes("/.🧬semio/🦑️repo/🎫️tickets/") || f.includes("/.🧬semio/🦑️repo/⚡️cache/") || f.includes("/🗑️generated/");
const manifests = files.filter((f) => f.endsWith("/Cargo.toml") && !skipDir(f) && existsSync(f));
const pkgs: Package[] = [];
for (const m of manifests) {
  const p = loadPackage(dirname(m));
  if (p) pkgs.push(p);
}
for (const p of pkgs) if (p.libName) {
  const l = libIndex.get(p.libName);
  if (l) l.push(p);
  else libIndex.set(p.libName, [p]);
}
lap(`loaded ${pkgs.length} packages`);

const touchedFile = new Set<string>();
for (const f of files) {
  try {
    if (statSync(f).mtimeMs >= since) touchedFile.add(f);
  } catch {}
}
lap(`${touchedFile.size} files touched since ${new Date(since).toISOString()}`);

const trees = new Map<string, CrateTree>();
function treeOf(pkg: Package, t: Package["targets"][number]): CrateTree {
  const id = `${pkg.name}${t.kind === "lib" ? "" : `#${t.kind}:${t.name}`}`;
  const key = `${pkg.dir}|${t.kind}|${t.name}|${t.path}`;
  let tr = trees.get(key);
  if (!tr) {
    tr = buildTree(pkg, t, id);
    trees.set(key, tr);
  }
  return tr;
}
setTreeBuilder((pkg) => {
  const t = pkg.targets.find((x) => x.kind === "lib");
  return t && t.path && existsSync(t.path) ? treeOf(pkg, t) : null;
});

const touchedPkgs = new Set<Package>();
const scopeFile = flag("--scope");
const scope = scopeFile ? new Set(readFileSync(scopeFile, "utf8").split("\n").filter(Boolean)) : null;
const forced = has("--all");
const wsRoots = new Set<string>();
for (const p of pkgs) {
  if (only && !only.includes(p.name)) continue;
  if (scope) {
    if (scope.has(rel(p.manifest))) touchedPkgs.add(p);
    continue;
  }
  if (forced || only) {
    touchedPkgs.add(p);
    continue;
  }
  if (touchedFile.has(p.manifest)) {
    touchedPkgs.add(p);
    continue;
  }
  for (const t of p.targets) {
    if (!t.path || !existsSync(t.path)) continue;
    if (touchedFile.has(t.path)) {
      touchedPkgs.add(p);
      break;
    }
  }
}
for (const m of manifests) if (touchedFile.has(m) || forced) {
  try {
    const t = (Bun as any).TOML.parse(readFileSync(m, "utf8"));
    if (t.workspace) wsRoots.add(dirname(m));
  } catch {}
}
lap(`${touchedPkgs.size} packages touched by manifest/root; scanning module closures`);

if (!forced && !only && !scope) {
  const dirty = pkgs.filter((p) => !touchedPkgs.has(p));
  for (const p of dirty) {
    for (const t of p.targets) {
      if (!t.path || !existsSync(t.path)) continue;
      const tr = treeOf(p, t);
      let hit = false;
      for (const f of tr.files) if (touchedFile.has(f)) hit = true;
      if (hit) {
        touchedPkgs.add(p);
        break;
      }
    }
    if (!touchedPkgs.has(p)) for (const t of p.targets) trees.delete(`${p.dir}|${t.kind}|${t.name}|${t.path}`);
  }
}
lap(`${touchedPkgs.size} touched packages; parsed ${parseStats.files} files (${(parseStats.bytes / 1e6).toFixed(0)} MB), ${parseStats.errors} with syntax errors`);

for (const p of pkgs) for (const d of p.deps) {
  if (d.path) continue;
  const en = d.key.replace(/-/g, "_");
  let m = regIndex.get(en);
  if (!m) regIndex.set(en, (m = new Map()));
  m.set(d.pkg, (m.get(d.pkg) ?? 0) + 1);
}
for (const p of touchedPkgs) for (const t of p.targets) if (t.kind !== "build" && t.path && existsSync(t.path)) treeOf(p, t);
for (const tr of trees.values()) for (const e of tr.root.entries) {
  if (e.kind !== "extern_crate" || e.name === e.leaf!.segs[0]) continue;
  let m = aliasIndex.get(e.name);
  if (!m) aliasIndex.set(e.name, (m = new Map()));
  m.set(e.leaf!.segs[0], (m.get(e.leaf!.segs[0]) ?? 0) + 1);
}
const out: Finding[] = [];
const repointMode = has("--repoint");
if (repointMode) {
  for (const p of pkgs) {
    const t = p.targets.find((x) => x.kind === "lib");
    if (t && t.path && existsSync(t.path)) treeOf(p, t);
  }
  lap(`prebuilt all lib trees for global candidates (${trees.size})`);
  setErrorHook(plan);
  setDupHook(planDup);
}
const stats: Stats = { unknown: 0, checked: 0, unknownSamples: [] };
for (const d of wsRoots) checkWorkspace(d, out);
const list = [...touchedPkgs].sort((a, b) => a.name.localeCompare(b.name));
let i = 0;
for (const p of list) {
  checkManifest(p, out);
  for (const t of p.targets) {
    if (t.kind === "build" || !t.path || !existsSync(t.path)) continue;
    checkSource(treeOf(p, t), out, stats);
  }
  if (++i % 200 === 0) lap(`${i}/${list.length} packages checked, ${out.length} findings`);
}
lap(`done: ${out.length} findings; ${stats.checked} paths resolved, ${stats.unknown} unknown (macro-opaque)`);

if (repointMode) {
  const r = applyAll(has("--apply"));
  writeFileSync(`${G}/suggest-${flag("--tag") ?? "run"}.jsonl`, suggestions.join("\n") + "\n");
  console.log(JSON.stringify({ repoint: r, relocated: repointStats.relocated, imported: repointStats.imported, widened: repointStats.widened, dropped: repointStats.dropped, skipped: Object.fromEntries(repointStats.skipped) }));
}
if (has("--syntax")) {
  const fset = new Set<string>();
  for (const tr of trees.values()) if (touchedPkgs.has(tr.pkg)) for (const f of tr.files) fset.add(f);
  const all = [...fset];
  let next = 0;
  let bad = 0;
  const worker = async () => {
    while (next < all.length) {
      const f = all[next++];
      const pr = Bun.spawn(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--config", "skip_children=true", f], { stdout: "ignore", stderr: "pipe" });
      const err = await new Response(pr.stderr).text();
      await pr.exited;
      const m = /^error[^\n]*\n\s*--> [^:\n]+:(\d+)/m.exec(err);
      if (m) {
        bad++;
        out.push({ crate: "(syntax)", kind: "syntax-error", sev: "error", file: rel(f), line: +m[1], msg: err.split("\n")[0].slice(0, 200) });
      } else if (/^error/m.test(err)) {
        bad++;
        out.push({ crate: "(syntax)", kind: "syntax-error", sev: "error", file: rel(f), line: 0, msg: err.split("\n")[0].slice(0, 200) });
      }
    }
  };
  await Promise.all(Array.from({ length: 8 }, worker));
  lap(`rustfmt syntax gate: ${all.length} files, ${bad} with syntax errors`);
}
const parseFiles = [...new Set(out.filter((f) => f.kind === "parse-error").map((f) => f.file))];
const realSyntax = new Set<string>();
for (const f of parseFiles) {
  const r = Bun.spawnSync(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--config", "skip_children=true", join(ROOT, f)], { cwd: ROOT });
  if (/^error/m.test(r.stderr.toString())) realSyntax.add(f);
}
for (let k = out.length - 1; k >= 0; k--) {
  const f = out[k];
  if (f.kind !== "parse-error") continue;
  if (realSyntax.has(f.file)) f.kind = "syntax-error";
  else out.splice(k, 1);
}
lap(`parse-error cross-check: ${parseFiles.length} files, ${realSyntax.size} confirmed by rustfmt`);
const tag = flag("--tag") ?? "run";
writeFileSync(`${G}/findings-${tag}.jsonl`, out.map((f) => JSON.stringify(f)).join("\n") + "\n");
writeFileSync(`${G}/unknown-${tag}.txt`, stats.unknownSamples.join("\n") + "\n");
writeFileSync(`${G}/touched-${tag}.txt`, list.map((p) => rel(p.manifest)).join("\n") + "\n");
const byKind: Record<string, number> = {};
const byCrate: Record<string, number> = {};
for (const f of out) {
  byKind[f.kind] = (byKind[f.kind] ?? 0) + 1;
  const c = f.crate.split("#")[0];
  byCrate[c] = (byCrate[c] ?? 0) + 1;
}
console.log(JSON.stringify({ touchedPackages: list.length, findings: out.length, byKind, crates: Object.keys(byCrate).length, resolved: stats.checked, unknown: stats.unknown }, null, 1));
