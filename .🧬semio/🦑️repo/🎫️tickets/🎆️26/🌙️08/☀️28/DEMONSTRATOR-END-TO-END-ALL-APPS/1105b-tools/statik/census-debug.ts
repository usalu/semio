import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { lstatSync, readFileSync, readdirSync, statSync, writeFileSync, mkdirSync } from "node:fs";
import { basename, dirname, join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS";
const GEN = TICKET + "/🗑️generated/1105b";
const POLICY = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json";
const ONLY = process.env.ONLY ? process.env.ONLY.split(",") : null;
const PLATFORMS: (string | null)[] = [null, "wasm32-wasip2"];
const OPTS: [string, number][] = [["dev", 0], ["rel", 3]];
const FEATURE_SETS = (process.env.FSETS ?? "default,all").split(",") as ("default" | "all")[];
const SUFFIX = process.env.SUFFIX ?? "";

const findKey = (o: any, k: string): any => {
  if (o && typeof o === "object") {
    if (k in o) return o[k];
    for (const v of Object.values(o)) { const r = findKey(v, k); if (r !== undefined) return r; }
  }
  return undefined;
};
const GLOBAL: { maxSourceBytes: number; maxSourceFiles: number; maxActiveLines: number } = findKey(JSON.parse(readFileSync(POLICY, "utf8")), "sourceBoundary");

const git = Bun.spawnSync(["git", "ls-files", "-c", "-o", "--exclude-standard", "-z", "--", "*Cargo.toml"], { cwd: ROOT });
const manifests = git.stdout.toString().split("\0").filter(p => p && basename(p) === "Cargo.toml" && !p.startsWith(".🧬semio/"));

const isFile = (p: string) => { try { return statSync(p).isFile(); } catch { return false; } };
const isDir = (p: string) => { try { return statSync(p).isDirectory(); } catch { return false; } };
const autoFiles = (dir: string, sub: string): string[] => {
  const d = join(dir, sub);
  if (!isDir(d)) return [];
  return readdirSync(d, { withFileTypes: true }).flatMap(e =>
    e.isFile() && e.name.endsWith(".rs") ? [join(d, e.name)]
      : e.isDirectory() && isFile(join(d, e.name, "main.rs")) ? [join(d, e.name, "main.rs")] : []);
};

type Target = { kind: "lib" | "bin" | "test" | "example" | "bench" | "build"; entry: string; proc: boolean };

const targetsOf = (dir: string, toml: any): Target[] => {
  const pkg = toml.package, out: Target[] = [], seen = new Set<string>();
  const add = (kind: Target["kind"], entry: string, proc = false) => {
    const abs = join(dir, entry);
    if (!isFile(abs) || seen.has(abs)) return;
    seen.add(abs); out.push({ kind, entry: abs, proc });
  };
  const lib = toml.lib ?? {};
  if (lib.path || pkg.autolib !== false) add("lib", lib.path ?? "src/lib.rs", lib["proc-macro"] === true || lib.proc_macro === true);
  for (const b of toml.bin ?? []) add("bin", b.path ?? `src/bin/${b.name}.rs`);
  if (pkg.autobins !== false) { add("bin", "src/main.rs"); for (const f of autoFiles(dir, "src/bin")) add("bin", f.slice(dir.length + 1)); }
  for (const t of toml.test ?? []) add("test", t.path ?? `tests/${t.name}.rs`);
  if (pkg.autotests !== false) for (const f of autoFiles(dir, "tests")) add("test", f.slice(dir.length + 1));
  for (const e of toml.example ?? []) add("example", e.path ?? `examples/${e.name}.rs`);
  if (pkg.autoexamples !== false) for (const f of autoFiles(dir, "examples")) add("example", f.slice(dir.length + 1));
  for (const b of toml.bench ?? []) add("bench", b.path ?? `benches/${b.name}.rs`);
  if (pkg.autobenches !== false) for (const f of autoFiles(dir, "benches")) add("bench", f.slice(dir.length + 1));
  if (typeof pkg.build === "string") add("build", pkg.build);
  else if (pkg.build !== false) add("build", "build.rs");
  return out;
};

const featureSets = (toml: any): Record<(typeof FEATURE_SETS)[number], string[]> => {
  const feat: Record<string, string[]> = toml.features ?? {};
  const optional = new Set<string>();
  const tables = [toml.dependencies, ...Object.values(toml.target ?? {}).map((t: any) => t.dependencies)];
  for (const table of tables) for (const [n, v] of Object.entries(table ?? {})) if (v && typeof v === "object" && (v as any).optional) optional.add(n);
  const active = new Set<string>();
  const visit = (e: string) => {
    if (e.startsWith("dep:") || e.includes("/") || active.has(e)) return;
    if (feat[e]) { active.add(e); for (const x of feat[e]!) visit(x); }
    else if (optional.has(e)) active.add(e);
  };
  for (const e of feat.default ?? []) visit(e);
  const all = new Set([...Object.keys(feat).filter(f => f !== "default"), ...optional]);
  return { default: [...active], all: [...all] };
};

const baseCache = new Map<string, string[]>();
const baseCfg = (platform: string | null, opt: number, test: boolean, proc: boolean): string[] => {
  const key = JSON.stringify([platform, opt, test, proc]);
  const hit = baseCache.get(key);
  if (hit) return hit;
  const args = ["--print", "cfg", "-C", `opt-level=${opt}`];
  if (platform) args.push("--target", platform);
  if (test) args.push("--test");
  if (proc) args.push("--crate-type=proc-macro");
  const r = Bun.spawnSync(["rustc", ...args]);
  if (r.exitCode !== 0) throw Error("rustc --print cfg failed: " + r.stderr.toString());
  const cfg = r.stdout.toString().trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
  baseCache.set(key, cfg);
  return cfg;
};

const bytesCache = new Map<string, Uint8Array | null>();
const readGate = async (path: string): Promise<Uint8Array | null> => {
  if (bytesCache.has(path)) return bytesCache.get(path)!;
  let facts;
  try { facts = lstatSync(path); } catch (e: any) {
    if (e.code === "ENOENT" || e.code === "ENOTDIR") { bytesCache.set(path, null); return null; }
    throw e;
  }
  if (facts.isSymbolicLink() || !facts.isFile() || facts.nlink !== 1) throw Error("Actual single-link Cargo Source file required: " + path);
  const bytes = new Uint8Array(readFileSync(path));
  bytesCache.set(path, bytes);
  return bytes;
};

type Measure = { files: number; bytes: number; lines: number; largest: number; largestPath: string; cross: { files: number; bytes: number } | null; error?: string };

const measure = async (entry: string, manifestDir: string, cfg: string[], limits: typeof GLOBAL): Promise<Measure> => {
  const reads: [string, number][] = [], seen = new Set<string>();
  const read = async (path: string) => {
    const b = await readGate(path);
    if (b && !seen.has(path)) { seen.add(path); reads.push([path, b.byteLength]); }
    return b;
  };
  let run = 0, count = 0, cross: Measure["cross"] = null;
  try {
    const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, read, { manifestDirectory: manifestDir, generated: () => undefined });
    for (const [, n] of reads) { count++; run += n; if (!cross && (count > limits.maxSourceFiles || run > limits.maxSourceBytes)) cross = { files: count, bytes: run }; }
    const big = inputs.reduce((a, i) => (i.bytes > a.bytes ? i : a), inputs[0] ?? { bytes: 0, path: "" } as any);
    return {
      files: inputs.length,
      bytes: inputs.reduce((s, i) => s + i.bytes, 0),
      lines: inputs.reduce((s, i) => s + i.activeLines, 0),
      largest: big.bytes, largestPath: big.path, cross,
    };
  } catch (e: any) {
    return { files: 0, bytes: 0, lines: 0, largest: 0, largestPath: "", cross, error: String(e?.message ?? e).slice(0, 300) };
  }
};

const rows: any[] = [], vanished: string[] = [];
const started = Date.now();
for (const rel of manifests) {
  if (ONLY && !ONLY.some(o => rel.includes(o))) continue;
  const abs = join(ROOT, rel), dir = dirname(abs);
  let text: string;
  try { text = readFileSync(abs, "utf8"); } catch { vanished.push(rel); continue; }
  const toml: any = Bun.TOML.parse(text);
  if (!toml.package) continue;
  const limits = {
    maxSourceBytes: Math.min(GLOBAL.maxSourceBytes, toml.package.metadata?.semio?.["rust-source-boundary"]?.maxSourceBytes ?? Infinity),
    maxSourceFiles: Math.min(GLOBAL.maxSourceFiles, toml.package.metadata?.semio?.["rust-source-boundary"]?.maxSourceFiles ?? Infinity),
    maxActiveLines: Math.min(GLOBAL.maxActiveLines, toml.package.metadata?.semio?.["rust-source-boundary"]?.maxActiveLines ?? Infinity),
  };
  const boundary = toml.package.metadata?.semio?.["rust-source-boundary"] ?? null;
  const targets = targetsOf(dir, toml);
  const fsets = featureSets(toml);
  const variants: any[] = [];
  let worst: any = { ratio: 0 };
  let maxBytes = 0, maxFiles = 0, maxLines = 0, maxBytesV = "", maxFilesV = "", maxLinesV = "";
  let baseline: any = null;
  const errors: string[] = [];
  let crossWorst: any = null;
  const byFs: Record<string, { bytes: number; files: number; lines: number }> = {};
  for (const fs of FEATURE_SETS) byFs[fs] = { bytes: 0, files: 0, lines: 0 };
  for (const t of targets) {
    for (const platform of PLATFORMS) for (const [optName, opt] of OPTS) for (const fs of FEATURE_SETS) {
      const test = t.kind === "test" || t.kind === "bench";
      const cfg = [...baseCfg(platform, opt, test, t.proc), ...fsets[fs].map(f => `feature=${f}`)];
      const label = `${t.kind}:${basename(t.entry)}|${platform ?? "host"}-${optName}-${fs}`;
      const m = await measure(t.entry, dir, cfg, limits);
      variants.push({ unit: label, ...m }); if (platform===null && optName==="dev" && fs==="default") console.log("[DEBUG] "+t.entry.slice(-90)+" files="+m.files+" bytes="+m.bytes);
      if (m.error) errors.push(label + ": " + m.error);
      if (!m.error) {
        if (m.bytes > maxBytes) { maxBytes = m.bytes; maxBytesV = label; }
        if (m.files > maxFiles) { maxFiles = m.files; maxFilesV = label; }
        if (m.lines > maxLines) { maxLines = m.lines; maxLinesV = label; }
      }
      if (!m.error) {
        const fm = byFs[fs];
        fm.bytes = Math.max(fm.bytes, m.bytes); fm.files = Math.max(fm.files, m.files); fm.lines = Math.max(fm.lines, m.lines);
      }
      if (m.cross && (!crossWorst || m.cross.bytes > crossWorst.bytes)) crossWorst = { ...m.cross, unit: label };
      if (!baseline && t.kind === "lib" && platform === null && optName === "dev" && fs === "default") baseline = { unit: label, files: m.files, bytes: m.bytes, lines: m.lines };
    }
  }
  if (!baseline && targets.length) { const v = variants.find(x => !x.error && x.unit.includes("host-dev-default")); if (v) baseline = { unit: v.unit, files: v.files, bytes: v.bytes, lines: v.lines }; }
  const ratio = Math.max(maxBytes / limits.maxSourceBytes, maxFiles / limits.maxSourceFiles, maxLines / limits.maxActiveLines);
  const over = maxBytes > limits.maxSourceBytes || maxFiles > limits.maxSourceFiles || maxLines > limits.maxActiveLines;
  rows.push({
    manifest: rel, package: toml.package.name, targets: targets.length, boundary,
    limits, maxBytes, maxBytesV, maxFiles, maxFilesV, maxLines, maxLinesV, ratio, byFs,
    status: errors.length ? "error" : over ? "OVER" : ratio >= 0.98 ? "NEAR" : "ok",
    baseline, crossWorst, errors, variants: variants.length,
  });
}
rows.sort((a, b) => b.ratio - a.ratio);
mkdirSync(GEN, { recursive: true });
writeFileSync(GEN + `/aud-census${SUFFIX}-out.json`, JSON.stringify({ global: GLOBAL, measured: rows.length, vanished, rows }, null, 1));
const tsv = ["package\tmanifest\tstatus\tratio\tbytes\tbytesLimit\tfiles\tfilesLimit\tlines\tlinesLimit\tbytesVariant\tbaselineBytes\tgateCross\terrors"];
for (const r of rows) tsv.push([r.package, r.manifest, r.status, r.ratio.toFixed(4), r.maxBytes, r.limits.maxSourceBytes, r.maxFiles, r.limits.maxSourceFiles, r.maxLines, r.limits.maxActiveLines, r.maxBytesV, r.baseline?.bytes ?? "", r.crossWorst ? `${r.crossWorst.bytes}B/${r.crossWorst.files}f@${r.crossWorst.unit}` : "", r.errors.length].join("\t"));
writeFileSync(GEN + `/aud-census${SUFFIX}-rows.tsv`, tsv.join("\n") + "\n");
console.log(`measured=${rows.length} over=${rows.filter(r => r.status === "OVER").length} near=${rows.filter(r => r.status === "NEAR").length} errors=${rows.filter(r => r.status === "error").length} seconds=${((Date.now() - started) / 1000).toFixed(1)}`);
