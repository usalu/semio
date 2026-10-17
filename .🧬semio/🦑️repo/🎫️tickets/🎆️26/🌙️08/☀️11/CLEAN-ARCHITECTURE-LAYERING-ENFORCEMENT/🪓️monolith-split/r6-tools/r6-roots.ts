// 🔭️ Hand check for the oracle blind spot: every `crate::<seg>` first segment used by a crate's closure must appear in that crate's own root files (lib root and mirror root).
import {readFileSync, existsSync} from "node:fs";
import {dirname, resolve, join} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
const strip = (t: string) => t.replace(/\/\*[\s\S]*?\*\//gu, "").split("\n").map(l => l.replace(/\/\/.*$/u, "")).join("\n");
const manifests: string[] = [];
for (const dir of process.argv.slice(2)) for (const f of new Bun.Glob("**/📦️packages/🦀️rust/Cargo.toml").scanSync({cwd: resolve(ROOT, dir)})) if (!f.includes("node_modules") && !f.includes("🏭️bridge") && !f.includes("🔮️oracles")) manifests.push(join(dir, dirname(f)));
for (const manifestArg of manifests) {
  const manifest = resolve(ROOT, manifestArg, "Cargo.toml");
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const entry = resolve(dirname(manifest), parsed.lib?.path ?? "src/lib.rs");
  const rootTexts = [entry, join(dirname(entry), "🪞️mirror/🦀️.rs")].filter(existsSync).map(f => strip(readFileSync(f, "utf8"))).join("\n");
  const inputs = await readRustSourceClosureV1([entry], ROOT, [...hostCfg, "debug_assertions", "test"], read, {manifestDirectory: dirname(manifest), generated: () => undefined});
  const missing = new Map<string, Set<string>>();
  for (const input of inputs) {
    if (input.kind !== "source") continue;
    const text = strip(readFileSync(input.path, "utf8"));
    for (const m of text.matchAll(/\bcrate::(\w+)/gu)) {
      const seg = m[1]!;
      if (new RegExp(`\\b${seg}\\b`, "u").test(rootTexts)) continue;
      (missing.get(seg) ?? missing.set(seg, new Set()).get(seg)!).add(input.path.replace(ROOT + "/", "").split("/").slice(-4).join("/"));
    }
  }
  console.log(`${parsed.package.name}: ${missing.size ? [...missing].map(([s, f]) => `${s}(${f.size}) e.g. ${[...f][0]}`).join("; ") : "ok"}`);
}
