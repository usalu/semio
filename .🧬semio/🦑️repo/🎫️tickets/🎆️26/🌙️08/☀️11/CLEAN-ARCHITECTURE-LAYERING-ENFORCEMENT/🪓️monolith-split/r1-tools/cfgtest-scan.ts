// 🔍️ Finds cfg(test)-only items of one crate that a different crate of the same artifact names (a dependency is built without cfg(test), so those names do not exist in a real build).
// usage: bun cfgtest-scan.ts <artifactDir relative to repo root>
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const art = process.argv[2]!;
const rustc = execFileSync("rustup", ["which", "rustc"], { encoding: "utf8" }).trim();
const host = execFileSync(rustc, ["--print", "cfg"], { encoding: "utf8" }).trim().split("\n").map((v) => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const manifests = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-co", "--exclude-standard", "-z", "--", art], { cwd: ROOT, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter((p) => p.endsWith("/Cargo.toml"));
type Crate = { name: string; lib: Set<string>; test: Set<string> };
const crates: Crate[] = [];
for (const m of manifests) {
  const dir = dirname(join(ROOT, m));
  const parsed: any = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8"));
  if (!parsed.package?.name || !parsed.lib?.path) continue;
  const entry = resolve(dir, parsed.lib.path);
  if (!existsSync(entry)) continue;
  const pick = async (test: boolean) => new Set(((await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", ...(test ? ["test"] : [])], readFile, { manifestDirectory: dir, generated: () => undefined })) as any[]).filter((i) => i.kind === "source").map((i) => String(i.path ?? i.file ?? i.name)));
  crates.push({ name: parsed.package.name, lib: await pick(false), test: await pick(true) });
}
const text = new Map<string, string>();
const read = (f: string) => { let t = text.get(f); if (t === undefined) { t = readFileSync(f, "utf8"); text.set(f, t); } return t; };
const itemRe = /^\s*pub(?:\([^)]*\))?\s+(?:async\s+)?(?:unsafe\s+)?(?:fn|const|static|struct|enum|type|trait|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)/gmu;
const gatedRe = /#\[cfg\(([^\]\n]*)\)\]((?:\s*(?:#\[[^\]\n]*\]|\/\/[^\n]*))*)\s*pub(?:\([^)]*\))?\s+(?:async\s+)?(?:unsafe\s+)?(?:fn|const|static|struct|enum|type|trait|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)/gmu;
const testy = (pred: string) => /\btest\b/u.test(pred.replace(/not\(\s*test\s*\)/gu, ""));
const stop = new Set(["tests", "test", "fixture", "fixtures", "support", "helper", "mod", "new", "main", "run", "check", "value", "entry", "build", "snapshot", "grant", "step_grant", "close_job"]);
const gated = new Map<string, Map<string, string>>();
for (const c of crates) {
  const names = new Map<string, string>();
  for (const f of c.test) {
    const t = read(f);
    const testOnlyFile = !c.lib.has(f);
    if (testOnlyFile) for (const m of t.matchAll(itemRe)) { const before = t.slice(Math.max(0, (m.index ?? 0) - 160), m.index); if (/#\[(?:test|[\w:]*async_test)\b/u.test(before.split("\n").slice(-3).join("\n"))) continue; names.set(m[1]!, f); }
    else for (const m of t.matchAll(gatedRe)) if (testy(m[1]!)) {
      names.set(m[2]!, f);
      const open = t.indexOf("{", (m.index ?? 0) + m[0].length - 1);
      if (/\bmod\s+\w+\s*$/u.test(m[0]) && open >= 0 && t.slice((m.index ?? 0) + m[0].length, open).trim() === "") {
        let depth = 0, end = open;
        for (; end < t.length; end++) { if (t[end] === "{") depth++; else if (t[end] === "}" && --depth === 0) break; }
        for (const inner of t.slice(open, end).matchAll(itemRe)) names.set(inner[1]!, f);
      }
    }
  }
  for (const f of c.test) {
    const t = read(f);
    for (const m of t.matchAll(/#\[cfg\(([^\]\n]*)\)\]\s*pub(?:\([^)]*\))?\s+use\s+([^;]+);/gu)) if (testy(m[1]!)) for (const id of m[2]!.matchAll(/[A-Za-z_][A-Za-z0-9_]*/gu)) if (!["crate", "self", "super", "as"].includes(id[0]!)) names.set(id[0]!, f);
    for (const m of t.matchAll(/#\[cfg\(([^\]\n]*)\)\]\s*impl\b[^{;]*\{/gu)) if (testy(m[1]!)) {
      const open = (m.index ?? 0) + m[0].length - 1; let depth = 0, end = open;
      for (; end < t.length; end++) { if (t[end] === "{") depth++; else if (t[end] === "}" && --depth === 0) break; }
      for (const inner of t.slice(open, end).matchAll(itemRe)) names.set(inner[1]!, f);
    }
  }
  gated.set(c.name, names);
}
const esc = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
const declared = (c: Crate, name: string) => [...c.test].some((f) => new RegExp(`\\b(?:fn|const|static|struct|enum|type|trait|mod|macro_rules!)\\s+${esc(name)}\\b`, "u").test(read(f)) || new RegExp(`\\buse\\b[^;]*\\b${esc(name)}\\b`, "u").test(read(f)));
let hits = 0;
for (const c of crates) for (const [name, file] of gated.get(c.name)!) {
  if (name.length < 5 || stop.has(name)) continue;
  for (const d of crates) {
    if (d === c || declared(d, name)) continue;
    for (const f of d.test) if (new RegExp(`(?:::|[{,]\\s*)${esc(name)}\\b`, "u").test(read(f))) { hits++; console.log(`${c.name}\t${name}\t${file.replace(ROOT + "/", "").slice(-70)}\t<- ${d.name}\t${f.replace(ROOT + "/", "").slice(-70)}${d.lib.has(f) ? "" : " [test-only file]"}`); break; }
  }
}
console.error(`crates ${crates.length} gated names ${[...gated.values()].reduce((s, m) => s + m.size, 0)} hits ${hits}`);
