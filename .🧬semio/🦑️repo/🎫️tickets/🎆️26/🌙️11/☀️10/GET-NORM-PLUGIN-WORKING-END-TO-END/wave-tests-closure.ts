import { execFileSync as gitExecFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const T = fileURLToPath(new URL(".", import.meta.url)).replace(/[\\/]$/, "");
const R = gitExecFileSync("git", ["-C", T, "rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, dirname, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const N = process.env.ROOT!;
const crates: { dir: string; name: string; lib: string }[] = [];
function walk(d: string) {
  for (const e of readdirSync(d)) {
    if (e === "target" || e === "node_modules") continue;
    const p = join(d, e);
    const s = statSync(p);
    if (s.isDirectory()) walk(p);
    else if (e === "Cargo.toml" && d.endsWith("🦀️rust")) {
      const tx = readFileSync(p, "utf8");
      const nm = tx.match(/^name\s*=\s*"([^"]+)"/m);
      const lm = tx.match(/\[lib\][^\[]*?path\s*=\s*"([^"]+)"/);
      if (nm) crates.push({ dir: d, name: nm[1], lib: resolve(d, lm ? lm[1] : "src/lib.rs") });
    }
  }
}
walk(N);
const cfg = ["test", 'feature="compliance-testing"', 'feature="test-support"', "debug_assertions", 'target_os="macos"', "unix"];
const out: string[] = [];
for (const c of crates) {
  try {
    const inputs = await readRustSourceClosureV1([c.lib], R, cfg, async (path: string) => { try { return new Uint8Array(readFileSync(path)); } catch { return null; } }, { manifestDirectory: c.dir, generated: () => undefined });
    const files = inputs.length, bytes = inputs.reduce((a: number, i: any) => a + i.bytes, 0), lines = inputs.reduce((a: number, i: any) => a + i.activeLines, 0);
    if (process.env.CRATE === c.name) for (const i of inputs as any[]) out.push(`  ${i.activeLines} ${i.bytes} ${i.path}`);
    out.push(`${files > 32 || bytes > 262144 || lines > 4000 ? "OVER" : "ok  "} files ${files} bytes ${bytes} lines ${lines} ${c.name}`);
  } catch (e) { out.push(`ERR ${c.name} ${String(e).slice(0, 120)}`); }
}
console.log(out.sort().join("\n"));
