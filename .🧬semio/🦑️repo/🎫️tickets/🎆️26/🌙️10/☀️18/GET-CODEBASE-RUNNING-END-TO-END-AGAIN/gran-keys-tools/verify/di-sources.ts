// 🔬 rustc's own dependency information (cargo check .d files, every unit of the crate's normal closure) against the sources the build graph declares for the same crates.
import {readFileSync, readdirSync} from "node:fs";
import {join, relative} from "node:path";
const SB = "/private/tmp/gk/sb", L = SB + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", OV = "/private/tmp/gk/di/overlay", TARGET = "/private/tmp/gk/di/target/debug/deps";
const crate = process.argv[2]!, WS = "🧰️framework";
const plugin: any = await import(L + "/🏗️build/🕸️graph/📥️inference/🟨️.mjs");
await plugin.libraryBootstrap;
const plan = await plugin.nativeSourceWatchPlanV1(crate, OV);
const declared = new Set<string>([...plan.files, ...plan.includes.filter((p: string) => !p.includes("*"))]);
const globs = plan.includes.filter((p: string) => p.includes("*"));
const names = new Map<string, string>(); // crate dir -> package name via the plan's roots (read manifests)
const roots = new Set<string>(plan.includes.filter((p: string) => p.endsWith("/Cargo.toml")).map((p: string) => p.slice(0, -"/Cargo.toml".length)));
const dfiles = readdirSync(TARGET).filter((f) => f.endsWith(".d"));
const local = new Map<string, Set<string>>();
for (const f of dfiles) {
  const text = readFileSync(join(TARGET, f), "utf8"), files = new Set<string>();
  for (const line of text.split("\n")) {
    if (!line.includes(": ")) continue;
    for (const part of line.slice(line.indexOf(": ") + 2).split(/(?<!\\) /u)) {
      const p = part.replace(/\\ /g, " ").trim();
      if (!p || p.startsWith("/")) continue;
      files.add(relative(OV, join(OV, WS, p)));
    }
  }
  local.set(f, files);
}
// crates of the normal closure: those whose directory contains a unit's root file
const unitsByCrate = new Map<string, Set<string>>();
const closureDirs = new Set<string>(plan.includes.filter((p: string) => p.endsWith("/Cargo.toml")).map((p: string) => p.slice(0, -"/Cargo.toml".length)));
const pkgName = (dir: string) => /^name\s*=\s*"([^"]+)"/mu.exec(readFileSync(join(OV, dir, "Cargo.toml"), "utf8"))?.[1]?.replace(/-/g, "_");
const closureNames = new Map([...closureDirs].filter((d) => { try { return !!pkgName(d); } catch { return false; } }).map((d) => [pkgName(d)!, d]));
let units = 0; const missing: string[] = []; let covered = 0, seen = new Set<string>();
for (const [f, files] of local) {
  const stem = f.replace(/-[0-9a-f]{16}\.d$/u, "");
  if (!closureNames.has(stem)) continue;
  units++;
  for (const file of files) { seen.add(file); if (!declared.has(file)) missing.push(`${stem}: ${file}`); else covered++; }
}
const extra = [...declared].filter((d) => d.endsWith(".rs") && !seen.has(d));
console.log(JSON.stringify({crate, closureCrates: closureNames.size, units, rustcLocalFiles: seen.size, declaredFiles: plan.files.length, missing: [...new Set(missing)], declaredButNotRead: extra.length, extraSample: extra.slice(0, 5), globs}, null, 1));
