// 🧪️ Per crate: compile the combined layout with cfg(test) true only for this crate's own files (every other crate is built as a dependency would be) and report the diagnostics that the all-test baseline does not have.
// usage: bun cfgtest-isolate.ts <artifactDir> <keptCombinedSandbox> [crateNameSubstring]
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { spawnSync } from "node:child_process";
import { diffDiagnostics } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const SCRATCH = "/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/";
const [art, sbx, only] = process.argv.slice(2) as [string, string, string | undefined];
if (!resolve(sbx).startsWith(SCRATCH)) throw new Error("sandbox must live under the scratchpad");
const rustc = execFileSync("rustup", ["which", "rustc"], { encoding: "utf8" }).trim();
const host = execFileSync(rustc, ["--print", "cfg"], { encoding: "utf8" }).trim().split("\n").map((v) => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const manifests = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-co", "--exclude-standard", "-z", "--", art], { cwd: ROOT, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter((p) => p.endsWith("/Cargo.toml"));
type Crate = { name: string; files: Set<string>; testkit: Set<string> };
const toSandbox = (p: string): string => { const abs = p.startsWith("/") ? p : join(ROOT, p); const out = abs.startsWith(ROOT + "/") ? join(resolve(sbx), abs.slice(ROOT.length + 1)) : abs; if (!out.startsWith(SCRATCH)) throw new Error("path outside the sandbox: " + out); return out; };
const crates: Crate[] = [];
for (const m of manifests) {
  const dir = dirname(join(ROOT, m));
  const parsed: any = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8"));
  if (!parsed.package?.name || !parsed.lib?.path) continue;
  const entry = resolve(dir, parsed.lib.path);
  if (!existsSync(entry)) continue;
  const inputs = (await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", "test"], readFile, { manifestDirectory: dir, generated: () => undefined })) as any[];
  const dev = (parsed["dev-dependencies"] ?? {}) as Record<string, any>;
  const kit = new Set<string>(Object.entries(dev).filter(([, v]) => Array.isArray(v?.features) && v.features.includes("testkit")).map(([k]) => k));
  kit.add(parsed.package.name);
  crates.push({ testkit: kit, name: parsed.package.name, files: new Set(inputs.filter((i) => i.kind === "source").map((i) => toSandbox(String(i.path ?? i.file ?? i.name)))) });
}
const guard = (f: string): void => { if (!f.startsWith(SCRATCH)) throw new Error("refusing to write outside the scratchpad: " + f); };
const neutralize = (text: string, off: boolean): string => {
  let out = ""; let i = 0;
  const re = /\bcfg(?:_attr)?!?\(/gu;
  for (let m = re.exec(text); m; m = re.exec(text)) {
    let depth = 0, j = m.index + m[0].length - 1;
    for (; j < text.length; j++) { if (text[j] === "(") depth++; else if (text[j] === ")" && --depth === 0) break; }
    out += text.slice(i, m.index) + text.slice(m.index, j + 1).replace(/(?<![\w"])test(?![\w"])/gu, "__r1_never").replace(off ? /feature\s*=\s*"testkit"/gu : /$^/gu, 'feature = "__r1_off"');
    i = j + 1; re.lastIndex = i;
  }
  return out + text.slice(i);
};
if (process.env.R1_DEBUG) for (const c of crates) console.log("[DEBUG] crate", c.name, c.files.size, [...c.files].filter((f) => f.endsWith("packages/🦀️rust/🦀️.rs")).map((f) => f.slice(-70)).join(" | "));
const combined = join(resolve(sbx), "__combined.rs");
const run = (): { diagnostics: string[] } => {
  const result = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "combined", "--emit=metadata", "--error-format=json", "-o", "/dev/null", "--test", "--cfg", 'feature="testkit"', combined], { cwd: resolve(sbx), encoding: "utf8", maxBuffer: 1 << 30 });
  const out: string[] = [];
  for (const line of (result.stderr ?? "").split("\n")) {
    if (!line.startsWith("{")) continue;
    try {
      const m = JSON.parse(line) as { level?: string; code?: { code?: string } | null; message?: string; spans?: { file_name: string; is_primary: boolean; line_start?: number }[] };
      if (m.level !== "error" || (m.message ?? "").startsWith("aborting due to")) continue;
      const primary = m.spans?.find((s) => s.is_primary);
      out.push([m.code?.code ?? "", (m.message ?? "").replaceAll(resolve(sbx), "<sandbox>"), primary ? `${primary.file_name}:${primary.line_start}` : ""].join("\t"));
    } catch { continue; }
  }
  return { diagnostics: out };
};
const base = run();
console.log(`baseline diagnostics ${base.diagnostics.length}`);
let bad = 0;
for (const c of crates) {
  if (only && !c.name.includes(only)) continue;
  const saved = new Map<string, string>();
  const mine = c.files;
  try {
    for (const d of crates) if (d !== c) for (const f of d.files) {
      if (process.env.R1_DEBUG && f.endsWith("🎬️demo/📦️packages/🦀️rust/🦀️.rs")) console.log("[DEBUG] examples root", d.name, "mine", mine.has(f), "saved", saved.has(f), "exists", existsSync(f));
      if (mine.has(f) || saved.has(f) || !existsSync(f)) continue;
      const t = readFileSync(f, "utf8"); const n = neutralize(t, !c.testkit.has(d.name));
      if (process.env.R1_DEBUG && f.endsWith("🎬️demo/📦️packages/🦀️rust/🦀️.rs")) console.log("[DEBUG] root text", t.length, t.includes("r1_probe"), n !== t, JSON.stringify(f));
      if (n !== t) { saved.set(f, t); guard(f); writeFileSync(f, n); }
    }
    if (process.env.R1_DEBUG) console.log("[DEBUG] neutralized", [...saved.keys()].map((f) => f.slice(-60)).join(" ; "));
    const r = run();
    const delta = diffDiagnostics(base.diagnostics, r.diagnostics);
    console.log(`${c.name}\tfiles-neutralized ${saved.size}\tdiagnostics ${r.diagnostics.length}\textra ${delta.onlyCandidate.length}`);
    for (const line of delta.onlyCandidate.slice(0, 12)) { bad++; console.log("   EXTRA " + line.slice(0, 300)); }
  } finally { if (!process.env.R1_KEEP) for (const [f, t] of saved) { guard(f); writeFileSync(f, t); } else console.log("[DEBUG] kept"); }
}
console.log(bad ? `FOUND ${bad} extra diagnostics` : "NO cross-crate cfg(test) dependency found");
