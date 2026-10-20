import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { createHash } from "node:crypto";
import { Finding, Package, loadPackage, readToml, wrongWorkspace } from "./cargo.ts";

const ROOT = process.env.CF_ROOT ?? "/Users/ueli/Documents/semio";
const C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const G = process.env.RS_FILES_DIR ?? `${C}/🗑️generated/cargo-fix`;
const args = process.argv.slice(2);
const apply = args.includes("--apply");
const input = args.find((a) => a.endsWith(".jsonl")) ?? `${G}/findings-before.jsonl`;
const backupDir = `${C}/🗑️generated/cargo-fix-x/backup`;
mkdirSync(backupDir, { recursive: true });

const findings: Finding[] = readFileSync(input, "utf8").trim().split("\n").filter(Boolean).map((l) => JSON.parse(l));
const files = readFileSync(`${G}/files.nul`, "utf8").split("\0").filter(Boolean).map((f) => join(ROOT, f));
const skipDir = (f: string) => f.includes("/.🧬semio/🦑️repo/🎫️tickets/") || f.includes("/.🧬semio/🦑️repo/⚡️cache/") || f.includes("/🗑️generated/");
const pkgs: Package[] = [];
for (const f of files) if (f.endsWith("/Cargo.toml") && !skipDir(f) && existsSync(f)) {
  const p = loadPackage(dirname(f));
  if (p) pkgs.push(p);
}
const byName = new Map<string, Package[]>();
for (const p of pkgs) (byName.get(p.name) ?? byName.set(p.name, []).get(p.name)!).push(p);
const pkgOf = (name: string): Package | null => {
  const l = byName.get(name);
  return l && l.length === 1 ? l[0] : null;
};

const ledgerPath = `${backupDir}/ledger.json`;
const ledger: Record<string, string> = existsSync(ledgerPath) ? JSON.parse(readFileSync(ledgerPath, "utf8")) : {};
function backup(file: string): void {
  if (ledger[file]) return;
  const h = createHash("sha1").update(file).digest("hex").slice(0, 16);
  writeFileSync(`${backupDir}/${h}`, readFileSync(file));
  ledger[file] = h;
  writeFileSync(ledgerPath, JSON.stringify(ledger, null, 1));
}

const normalGraph = new Map<string, Set<string>>();
for (const p of pkgs) {
  const s = new Set<string>();
  for (const d of p.deps) if (d.section === "dependencies" && d.path) {
    const t = loadPackage(d.path);
    if (t) s.add(t.dir);
  }
  normalGraph.set(p.dir, s);
}
function reaches(from: string, to: string): boolean {
  const seen = new Set<string>();
  const st = [from];
  while (st.length) {
    const d = st.pop()!;
    if (d === to) return true;
    if (seen.has(d)) continue;
    seen.add(d);
    for (const n of normalGraph.get(d) ?? []) st.push(n);
  }
  return false;
}

type Need = { pkg: Package; dep: string; normal: boolean };
const needs = new Map<string, Need>();
const aliasNeeds = new Map<string, { root: string; alias: string; of: string; pkg: Package; normal: boolean }>();
const skipped: string[] = [];
for (const f of findings) {
  if (!f.pkg) continue;
  const pkg = pkgOf(f.pkg);
  if (!pkg) {
    skipped.push(`${f.pkg}: ambiguous or unknown package name`);
    continue;
  }
  const test = f.envs === "test";
  if (f.kind === "undeclared-dependency" && f.dep) {
    if (f.ambiguous) {
      skipped.push(`${f.pkg}: ambiguous lib owner ${f.ambiguous.join("|")} for ${f.dep}`);
      continue;
    }
    const k = `${pkg.dir}|${f.dep}`;
    const n = needs.get(k) ?? { pkg, dep: f.dep, normal: false };
    if (!test) n.normal = true;
    needs.set(k, n);
  } else if (f.kind === "missing-extern-alias" && f.alias && f.aliasOf && f.root) {
    const libOwners = pkgs.filter((p) => p.libName === f.aliasOf);
    if (libOwners.length !== 1) {
      skipped.push(`${f.pkg}: alias ${f.alias} -> ${f.aliasOf} has ${libOwners.length} owners`);
      continue;
    }
    const k = `${f.root}|${f.alias}`;
    const a = aliasNeeds.get(k) ?? { root: f.root, alias: f.alias, of: f.aliasOf, pkg, normal: false };
    if (!test) a.normal = true;
    aliasNeeds.set(k, a);
    const dk = `${pkg.dir}|${libOwners[0].name}`;
    const n = needs.get(dk) ?? { pkg, dep: libOwners[0].name, normal: false };
    if (!test) n.normal = true;
    needs.set(dk, n);
  }
}

function depLine(pkg: Package, target: Package | null, name: string): string | null {
  const wsEntry = pkg.ws?.deps?.[name];
  if (wsEntry !== undefined) {
    if (target && wsEntry && typeof wsEntry === "object" && wsEntry.path && resolve(pkg.ws!.dir, wsEntry.path) !== target.dir) return `${name} = { path = ${JSON.stringify(relative(pkg.dir, target.dir))} }`;
    return `${name} = { workspace = true }`;
  }
  if (target) return `${name} = { path = ${JSON.stringify(relative(pkg.dir, target.dir))} }`;
  const counts = new Map<string, number>();
  for (const p of pkgs) for (const d of p.deps) if (!d.path && d.pkg === name && d.version && d.version !== "__missing_workspace_entry__") counts.set(d.version, (counts.get(d.version) ?? 0) + 1);
  const best = [...counts].sort((a, b) => b[1] - a[1])[0];
  return best ? `${name} = ${JSON.stringify(best[0])}` : null;
}

function insertInSection(text: string, header: string, line: string): string {
  const lines = text.split("\n");
  const idx = lines.findIndex((l) => l.trim() === `[${header}]`);
  if (idx < 0) {
    const sep = text.endsWith("\n") ? "" : "\n";
    return `${text}${sep}\n[${header}]\n${line}\n`;
  }
  let end = lines.length;
  for (let i = idx + 1; i < lines.length; i++) if (/^\s*\[/.test(lines[i])) {
    end = i;
    break;
  }
  let at = end;
  while (at > idx + 1 && lines[at - 1].trim() === "") at--;
  lines.splice(at, 0, line);
  return lines.join("\n");
}

const report = { depsAdded: 0, depsNormal: 0, depsDev: 0, cyclesRefused: [] as string[], unresolvedTargets: [] as string[], aliases: 0, skipped, files: new Set<string>() };
const byPkg = new Map<string, Need[]>();
for (const n of needs.values()) (byPkg.get(n.pkg.dir) ?? byPkg.set(n.pkg.dir, []).get(n.pkg.dir)!).push(n);

for (const [dir, list] of byPkg) {
  const pkg = list[0].pkg;
  const manifest = pkg.manifest;
  const t = readToml(manifest);
  let text = readFileSync(manifest, "utf8");
  let changed = false;
  for (const n of list) {
    const targets = pkgs.filter((p) => p.name === n.dep);
    const target = targets.length === 1 ? targets[0] : null;
    const fresh = loadPackage(dir);
    const existing = (fresh ?? pkg).deps.find((d) => d.key === n.dep && d.section !== "build-dependencies");
    if (!target && !pkgs.some((p) => p.deps.some((d) => d.pkg === n.dep && !d.path))) {
      report.unresolvedTargets.push(`${pkg.name} -> ${n.dep}`);
      continue;
    }
    if (target && target.dir === dir) continue;
    if (target && wrongWorkspace(pkg, target)) {
      report.unresolvedTargets.push(`${pkg.name} -> ${n.dep} (wrong workspace)`);
      continue;
    }
    let normal = n.normal;
    if (existing && (existing.section === "dependencies" || !normal)) continue;
    if (normal && target) {
      if (reaches(target.dir, dir)) {
        report.cyclesRefused.push(`${pkg.name} -> ${target.name}`);
        continue;
      } else normalGraph.get(dir)?.add(target.dir);
    }
    const line = depLine(pkg, target, n.dep);
    if (!line) {
      report.unresolvedTargets.push(`${pkg.name} -> ${n.dep} (no version)`);
      continue;
    }
    if (new RegExp(`^${n.dep.replace(/[-]/g, "\\-")}\\s*=`, "m").test(sectionText(text, normal ? "dependencies" : "dev-dependencies"))) continue;
    text = insertInSection(text, normal ? "dependencies" : "dev-dependencies", line);
    changed = true;
    report.depsAdded++;
    if (normal) report.depsNormal++;
    else report.depsDev++;
  }
  if (changed) {
    report.files.add(relative(ROOT, manifest));
    if (apply) {
      const cur = readFileSync(manifest, "utf8");
      backup(manifest);
      writeFileSync(manifest, text);
      const re = Bun.spawnSync(["bun", "-e", `require("fs");Bun.TOML.parse(require("fs").readFileSync(process.argv[1],"utf8"))`, manifest]);
      if (re.exitCode !== 0) {
        writeFileSync(manifest, cur);
        console.error("TOML invalid after edit, restored", manifest);
      }
    }
  }
}

function sectionText(text: string, header: string): string {
  const lines = text.split("\n");
  const idx = lines.findIndex((l) => l.trim() === `[${header}]`);
  if (idx < 0) return "";
  let end = lines.length;
  for (let i = idx + 1; i < lines.length; i++) if (/^\s*\[/.test(lines[i])) {
    end = i;
    break;
  }
  return lines.slice(idx + 1, end).join("\n");
}

for (const a of aliasNeeds.values()) {
  const file = join(ROOT, a.root);
  if (!existsSync(file)) continue;
  const text = readFileSync(file, "utf8");
  const re = new RegExp(`extern\\s+crate\\s+${a.of}\\s+as\\s+${a.alias}\\s*;`);
  if (re.test(text)) continue;
  const line = `extern crate ${a.of} as ${a.alias};`;
  const lines = text.split("\n");
  let at = 0;
  let lastExtern = -1;
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    if (/^extern\s+crate\s/.test(l)) lastExtern = i;
  }
  if (lastExtern >= 0) at = lastExtern + 1;
  else {
    while (at < lines.length && (/^\s*\/\/[!\/]/.test(lines[at]) || /^\s*#!\[/.test(lines[at]) || lines[at].trim() === "")) at++;
  }
  lines.splice(at, 0, line);
  report.aliases++;
  report.files.add(a.root);
  if (apply) {
    backup(file);
    writeFileSync(file, lines.join("\n"));
  }
}

console.log(JSON.stringify({ apply, input: relative(ROOT, input), depsAdded: report.depsAdded, depsNormal: report.depsNormal, depsDev: report.depsDev, aliases: report.aliases, files: report.files.size, cyclesRefused: report.cyclesRefused.length, unresolvedTargets: report.unresolvedTargets.length, skipped: report.skipped.length }, null, 1));
writeFileSync(`${C}/🗑️generated/cargo-fix-x/fixdeps-report${apply ? `-${Date.now()}` : "-dry"}.json`, JSON.stringify({ ...report, files: [...report.files] }, null, 1));
