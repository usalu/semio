// 🧪️ Finds names of #[cfg(test)] items of one produced crate that files of another produced crate mention (cross-crate cfg(test) use).
import {readFileSync, readdirSync, existsSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = process.env.ROOT ?? "/Users/ueli/Documents/semio";
const artifact = process.argv[2]!;
const prefix = process.argv[3]!;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const manifests = (process.env.ROOT ? execFileSync("find", [artifact, "-name", "Cargo.toml"], {cwd: ROOT, maxBuffer: 1 << 28}).toString("utf8").split("\n") : execFileSync("git", ["-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-co", "--exclude-standard", "-z", "--", artifact], {cwd: ROOT, maxBuffer: 1 << 28}).toString("utf8").split("\0")).filter(p => p.endsWith("/Cargo.toml") && p.includes("📦️packages/🦀️rust/"));
const readFile = async (path: string) => { try { return readFileSync(path); } catch { return null; } };
type Crate = {name: string; dir: string; lib: Set<string>; test: Set<string>};
const crates: Crate[] = [];
for (const m of manifests) {
  const abs = join(ROOT, m); const parsed = Bun.TOML.parse(readFileSync(abs, "utf8")) as any;
  const name = parsed.package?.name as string | undefined; if (!name?.startsWith(prefix)) continue;
  const dir = dirname(abs); const entry = resolve(dir, parsed.lib?.path ?? "src/lib.rs"); if (!existsSync(entry)) continue;
  const sets: Set<string>[] = [];
  for (const test of [false, true]) {
    const inputs = await readRustSourceClosureV1([entry], ROOT, [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])], readFile, {manifestDirectory: dir, generated: () => undefined});
    sets.push(new Set(inputs.filter((i: any) => String(i.path ?? "").endsWith(".rs")).map((i: any) => String(i.path))));
  }
  crates.push({name, dir, lib: sets[0]!, test: sets[1]!});
}
const defRe = /#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(mod|fn|struct|enum|const|static|type|trait)\s+([A-Za-z_][A-Za-z0-9_]*)/gu;
const ignore = new Set(["tests", "leaf_tests", "hoisted", "super", "crate", "self", "std"]);
const textOf = new Map<string, string>();
const text = (p: string) => { let t = textOf.get(p); if (t === undefined) { try { t = readFileSync(join(ROOT, p), "utf8"); } catch { try { t = readFileSync(p, "utf8"); } catch { t = ""; } } textOf.set(p, t); } return t; };
for (const x of crates) {
  const names = new Map<string, string>();
  for (const f of x.test) if (!f.includes("🪞️mirror") && !f.includes("📦️packages/🦀️rust/")) for (const m of text(f).matchAll(defRe)) { const n = m[2]!; if (!ignore.has(n) && !n.startsWith("hoisted_")) names.set(n, f); }
  for (const y of crates) {
    if (y === x) continue;
    for (const f of y.test) {
      if (x.test.has(f)) continue;
      const body = text(f);
      for (const [n, def] of names) if (new RegExp(`(?:::|\\{|,|\\s)${n}(?:::|\\b)`, "u").test(body) && !y.test.has(def)) console.log(`${x.name} defines cfg(test) ${n} (${def.split("/").slice(-4).join("/")}) <- ${y.name} ${f.split("/").slice(-5).join("/")}`);
    }
  }
}
console.log("crates", crates.length);
