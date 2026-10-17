// 🧫️ Serves the embedded fixtures of a crate's test files at run time: fixt.ts <manifest> [--apply] [--only=<substring>]. Uses the generator's fixturesToRuntime (tested) on every test-closure source file below a 🧪️tests directory; logs each (file, fixture, sha256) and verifies every referenced fixture exists relative to CARGO_MANIFEST_DIR.
import {readFileSync, writeFileSync, existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {dirname, relative, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {fixturesToRuntime, resolveFixturePlaceholders} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const args = process.argv.slice(2);
const manifest = resolve(ROOT, args.find(a => !a.startsWith("--"))!);
const apply = args.includes("--apply");
const only = args.find(a => a.startsWith("--only="))?.slice(7);
const dir = dirname(manifest);
const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
const feats: string[] = [];
const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
const cfg = [...hostCfg, "debug_assertions", "test", ...feats];
const entries: string[] = [libEntry];
for (const t of parsed.test ?? []) entries.push(resolve(dir, t.path ?? `tests/${t.name}.rs`));
const inputs: any[] = [];
for (const entry of entries) if (existsSync(entry)) inputs.push(...((await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined})) as any[]));
const seen = new Set<string>();
const included = new Set<string>();
for (const input of inputs) {
  if (input.kind !== "source") continue;
  const abs = resolve(ROOT, input.path ?? input.file ?? input.name);
  for (const m of readFileSync(abs, "utf8").matchAll(/include!\(\s*"([^"]+)"\s*\)/gu)) included.add(resolve(dirname(abs), m[1]!));
}
let total = 0;
for (const input of inputs) {
  if (input.kind !== "source") continue;
  const abs = resolve(ROOT, input.path ?? input.file ?? input.name);
  const rel = relative(ROOT, abs);
  if (included.has(abs)) { console.log("SKIP-INCLUDED\t" + rel); continue; }
  if (seen.has(abs) || !rel.includes("/🧪️tests/") || (only && !rel.includes(only))) continue;
  seen.add(abs);
  const text = readFileSync(abs, "utf8");
  const rewritten = fixturesToRuntime(text, abs, ROOT);
  if (rewritten === null) { if (process.env.HDR_DEBUG) console.log("NULL\t" + rel); continue; }
  const final = resolveFixturePlaceholders(rewritten, ROOT, dir);
  if (process.env.HDR_DEBUG) console.log("CAND\t" + rel.slice(-50) + "\t" + final.includes("runtime_fixture") + "\t" + rewritten.includes("@fixtures"));
  const refs = [...final.matchAll(/(?:runtime_fixture_(?:text|bytes)\(|fixture_(?:text|bytes|path)!\()"([^"]+)"\)/gu)].map(m => m[1]!);
  for (const ref of refs) {
    const path = resolve(dir, ref);
    const ok = existsSync(path);
    const sha = ok ? createHash("sha256").update(readFileSync(path)).digest("hex").slice(0, 16) : "MISSING";
    console.log(`${apply ? "APPLY" : "PLAN "}\t${rel}\t${ref}\t${sha}`);
    if (!ok) { console.log("ABORT missing fixture " + path); process.exit(2); }
  }
  total += refs.length;
  if (apply) writeFileSync(abs, final);
}
console.log(`fixt: ${total} fixture reads ${apply ? "applied" : "planned"} in ${seen.size} candidate test files`);
