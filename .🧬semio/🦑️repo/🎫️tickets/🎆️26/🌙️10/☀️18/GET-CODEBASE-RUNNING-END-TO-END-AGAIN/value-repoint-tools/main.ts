import { existsSync, mkdirSync, readFileSync, writeFileSync, statSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { initParser, findWasm } from "../rs-sweep-tools/parse.ts";
import { Package, loadPackage } from "../rs-sweep-tools/cargo.ts";
import { FACADE_DIR, ROOT, loadOwners } from "./resolve-owner.ts";
import { Moved, Note, transform } from "./transform.ts";
import { planManifests } from "./manifests.ts";
import { buildFileMap } from "./filemap.ts";

const args = process.argv.slice(2);
const has = (n: string) => args.includes(n);
const flag = (n: string) => (args.indexOf(n) >= 0 ? args[args.indexOf(n) + 1] : undefined);
const apply = has("--apply");
const familyMode = has("--family");
const C = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const G = flag("--out") ?? `${C}/🗑️generated/value-repoint`;
mkdirSync(`${G}/backup`, { recursive: true });
const moved: Moved = JSON.parse(readFileSync(`${import.meta.dir}/moved.json`, "utf8"));
await initParser(findWasm(ROOT)!);
const libs = [...new Set<string>([...Object.values(moved.modules), ...Object.values(moved.functions)])];
const owners = loadOwners(libs);
const t0 = Date.now();
const lap = (m: string) => console.error(`[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`);

const listFile = `${G}/files.nul`;
if (has("--refresh") || !existsSync(listFile)) {
  const r = Bun.spawnSync(["git", "ls-files", "-z", "-co", "--exclude-standard", "--", "*.rs", "*Cargo.toml"], { cwd: ROOT, maxBuffer: 1 << 30 });
  writeFileSync(listFile, r.stdout);
}
const skipPath = (f: string) => f.includes("/.🧬semio/") || f.includes("/🗑️generated/") || f.includes("/node_modules/") || f.includes("/target/");
const all = readFileSync(listFile, "utf8").split("\0").filter(Boolean).map((f) => join(ROOT, f)).filter((f) => !skipPath(f) && existsSync(f));
const rsFiles = all.filter((f) => f.endsWith(".rs"));
const VALUE_DIR = `${ROOT}/🧰️framework/🔨️modules/🌱️value/`;
lap(`${rsFiles.length} rust files`);

const manifestsAll = all.filter((f) => f.endsWith("/Cargo.toml"));
const fileMap = buildFileMap(manifestsAll, `${G}/filemap.json`, has("--refresh-map"));
lap(`file map: ${fileMap.size} files mounted by packages`);
const rootDirs = new Map<string, Package[]>();
for (const m of manifestsAll) {
  const p = loadPackage(dirname(m));
  if (!p) continue;
  for (const t of p.targets) if (t.kind !== "build" && t.path) {
    const d = dirname(t.path);
    (rootDirs.get(d) ?? rootDirs.set(d, []).get(d)!).push(p);
  }
}
function pkgsOf(file: string): Package[] {
  const direct = (fileMap.get(file) ?? []).map((d) => loadPackage(d)).filter((p): p is Package => !!p);
  if (direct.length) return direct;
  for (let d = dirname(file); d.length > ROOT.length; d = dirname(d)) {
    const l = rootDirs.get(d);
    if (l) return [...new Set(l)];
  }
  return [];
}
const isFamilyPkg = (p: Package) => p.name.startsWith("semio-framework-value");

type FileRes = { file: string; pkg: string | null; notes: Note[]; rewrites: number; libs: string[]; changed: boolean };
const results: FileRes[] = [];
const orphan: string[] = [];
const mixedOwners: string[] = [];
const libsByPkg = new Map<string, { pkg: Package; libs: Set<string> }>();
const familyHits: { file: string; pkg: string | null; rewrites: number; notes: number }[] = [];
const backupLedger = `${G}/backup/ledger.json`;
const ledger: Record<string, string> = existsSync(backupLedger) ? JSON.parse(readFileSync(backupLedger, "utf8")) : {};
const aliasHeads = ["pack", "protocol", "replication"];
const globHeads = ["protocol", "semio_framework_replication", "replication"];
const ctx = { moved, facade: "semio_framework_value", verify: owners.verify, aliasHeads, globHeads };
let scanned = 0;
for (const f of rsFiles) {
  let src: string;
  try {
    src = readFileSync(f, "utf8");
  } catch {
    continue;
  }
  if (!src.includes("semio_framework_value") && !aliasHeads.some((h) => src.includes(`${h}::value::`) || src.includes(`${h}::__value::`)) && !globHeads.some((h) => src.includes(`${h}::`))) continue;
  const owners_ = pkgsOf(f);
  const p = owners_[0] ?? null;
  const fam = owners_.length ? owners_.every(isFamilyPkg) : f.startsWith(VALUE_DIR);
  const mixed = owners_.some(isFamilyPkg) && !fam;
  if (mixed) mixedOwners.push(relative(ROOT, f));
  if (!owners_.length) orphan.push(relative(ROOT, f));
  const ownersNonFamily = familyMode ? owners_ : owners_.filter((q) => !isFamilyPkg(q));
  if (fam !== familyMode) {
    if (fam) {
      const r = transform(src, ctx);
      if (r.rewrites || r.notes.length) familyHits.push({ file: relative(ROOT, f), pkg: owners_.map((q) => q.name).join(",") || null, rewrites: r.rewrites, notes: r.notes.length });
    }
    continue;
  }
  scanned++;
  const r = transform(src, ctx);
  if (!r.rewrites && !r.notes.length) continue;
  results.push({ file: relative(ROOT, f), pkg: owners_.map((q) => q.name).join(",") || null, notes: r.notes, rewrites: r.rewrites, libs: [...r.libs], changed: r.out !== src });
  if (r.out !== src && apply) {
    const fresh = readFileSync(f, "utf8");
    if (fresh !== src) {
      results[results.length - 1].notes.push({ kind: "stale-skip", line: 0, text: "file changed during run" });
      continue;
    }
    if (!ledger[f]) {
      const h = createHash("sha1").update(f).digest("hex").slice(0, 16);
      writeFileSync(`${G}/backup/${h}`, src);
      ledger[f] = h;
    }
    writeFileSync(f, r.out);
  }
  if (r.out !== src) for (const q of ownersNonFamily) {
    const e = libsByPkg.get(q.dir) ?? { pkg: q, libs: new Set<string>() };
    for (const l of r.libs) e.libs.add(l);
    libsByPkg.set(q.dir, e);
  }
}
writeFileSync(backupLedger, JSON.stringify(ledger, null, 1));
lap(`scanned ${scanned} files, ${results.length} with rewrites/notes, ${libsByPkg.size} packages`);

const man = planManifests(libsByPkg, owners, apply, G);
writeFileSync(`${G}/results${familyMode ? "-family" : ""}.json`, JSON.stringify({ results, familyHits, manifests: man, orphan, mixedOwners }, null, 1));
const kinds: Record<string, number> = {};
for (const r of results) for (const n of r.notes) kinds[n.kind] = (kinds[n.kind] ?? 0) + 1;
console.log(JSON.stringify({ apply, familyMode, files: results.filter((r) => r.changed).length, rewrites: results.reduce((a, r) => a + r.rewrites, 0), packages: libsByPkg.size, noteKinds: kinds, familyFilesWithHits: familyHits.length, orphanFiles: orphan.length, manifests: { edited: man.edited, wsDecls: man.wsDecls, issues: man.issues.length } }, null, 1));
