// 🔎️ fixcheck.ts <manifest>... : verifies that every manifest-dir-relative fixture literal in the lib and test closures (runtime_fixture_*, fixture_*!, env!(CARGO_MANIFEST_DIR)) resolves to an existing path.
import {readFileSync, existsSync} from "node:fs";
import {dirname, relative, resolve, normalize} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
let bad = 0, checked = 0;
for (const arg of process.argv.slice(2)) {
  const manifest = resolve(ROOT, arg), dir = dirname(manifest);
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const entries = [resolve(dir, parsed.lib?.path ?? "src/lib.rs"), ...(parsed.test ?? []).map((t: any) => resolve(dir, t.path ?? `tests/${t.name}.rs`))].filter(existsSync);
  const seen = new Set<string>();
  for (const entry of entries) {
    let inputs: any[];
    try { inputs = await readRustSourceClosureV1([entry], ROOT, [...hostCfg, "debug_assertions", "test"], readFile, {manifestDirectory: dir, generated: () => undefined}) as any[]; } catch { continue; }
    for (const i of inputs) {
      if (i.kind !== "source") continue;
      const f = resolve(ROOT, i.path);
      if (seen.has(f)) continue; seen.add(f);
      const t = readFileSync(f, "utf8");
      for (const m of t.matchAll(/(?:runtime_fixture_(?:text|bytes)\(|fixture_(?:text|bytes|path)!\(|env!\("CARGO_MANIFEST_DIR"\),\s*)"\/?(\.\.\/[^"]+)"/gu)) {
        checked += 1;
        const p = normalize(resolve(dir, m[1]!));
        if (!existsSync(p)) { bad += 1; console.log(`MISSING ${arg.slice(-60)} :: ${relative(ROOT, f).slice(-50)} -> ${m[1]!.slice(-60)}`); }
      }
    }
  }
}
console.log(`fixcheck: ${checked} manifest-relative fixture literals, ${bad} missing`);
