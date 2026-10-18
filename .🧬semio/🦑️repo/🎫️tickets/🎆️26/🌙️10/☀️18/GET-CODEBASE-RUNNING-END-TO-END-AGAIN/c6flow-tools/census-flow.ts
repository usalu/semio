// 🧮️ c6flow own-closure census: runs the canonical closure counter over every flow crate (lib path of each Cargo.toml under the flow module) for the lib and cfg(test) closure; with files.tsv it lists the crates whose closure holds one of the tracked files; writes nothing.
// usage: bun census-flow.ts <root> [--owners] [name-substring]
import {readFileSync, readdirSync, statSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve, join, dirname} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = resolve(process.argv[2]!);
const owners = process.argv.includes("--owners");
const filter = process.argv.slice(3).find(a => !a.startsWith("--"));
const FL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const tracked = readFileSync(join(import.meta.dir, "files.tsv"), "utf8").trim().split("\n").map(l => l.split("\t") as [string, string]);
const manifests: string[] = [];
const walk = (dir: string) => {
  for (const e of readdirSync(dir)) {
    const p = join(dir, e);
    let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) { if (e === "node_modules" || e.startsWith(".")) continue; walk(p); }
    else if (e === "Cargo.toml") manifests.push(p);
  }
};
walk(join(root, FL));
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
for (const m of manifests) {
  const text = readFileSync(m, "utf8");
  const name = /^name\s*=\s*"([^"]+)"/m.exec(text)?.[1];
  const lib = /^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/ms.exec(text)?.[1];
  const tests = [...text.matchAll(/^\[\[test\]\][^\[]*?^path\s*=\s*"([^"]+)"/gms)].map(x => x[1]!);
  if (!name) continue;
  if (filter && !name.includes(filter)) continue;
  const mdir = dirname(m);
  const targets: [string, string][] = [];
  if (lib) targets.push(["lib", resolve(mdir, lib)]);
  for (const t of tests) targets.push(["bin-test", resolve(mdir, t)]);
  for (const [kind, rootFile] of targets) {
    for (const test of [false, true]) {
      const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])];
      try {
        const inputs = await readRustSourceClosureV1([rootFile], root, cfg, readFile, {manifestDirectory: mdir, generated: () => undefined});
        const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
        const paths = inputs.map(i => (i as any).path ?? (i as any).file ?? "");
        const held = owners ? tracked.filter(([, p]) => paths.some(x => String(x).endsWith(p))).map(([t]) => t) : [];
        if (owners && held.length === 0) continue;
        console.log(JSON.stringify({name, kind, test, files, bytes, activeLines: lines, ratio: +Math.max(files / 32, bytes / 262144, lines / 4000).toFixed(3), held}));
      } catch (error) {
        console.log(JSON.stringify({name, kind, test, error: String((error as Error).message ?? error).slice(0, 160)}));
      }
    }
  }
}
