// 🗺️ clmap.ts <workspace-Cargo.toml> : file -> member manifests whose lib or test closure holds it (JSON on stdout)
import {readFileSync, existsSync} from "node:fs";
import {dirname, relative, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
const ws = resolve(ROOT, process.argv[2]!);
const wsd = dirname(ws);
const parsed = Bun.TOML.parse(readFileSync(ws, "utf8")) as any;
const out: Record<string, string[]> = {};
for (const m of parsed.workspace.members as string[]) {
  const dir = resolve(wsd, m);
  const mf = resolve(dir, "Cargo.toml");
  const p = Bun.TOML.parse(readFileSync(mf, "utf8")) as any;
  const entry = resolve(dir, p.lib?.path ?? "src/lib.rs");
  if (!existsSync(entry)) continue;
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : []), "feature=testkit"];
    try {
      const got = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
      for (const g of got as any[]) { const f = relative(ROOT, resolve(ROOT, g.path)); (out[f] ??= []); if (!out[f]!.includes(relative(ROOT, mf))) out[f]!.push(relative(ROOT, mf)); }
    } catch (e) { console.error("ERR", mf, String((e as Error).message).slice(0, 100)); }
  }
}
console.log(JSON.stringify(out));
