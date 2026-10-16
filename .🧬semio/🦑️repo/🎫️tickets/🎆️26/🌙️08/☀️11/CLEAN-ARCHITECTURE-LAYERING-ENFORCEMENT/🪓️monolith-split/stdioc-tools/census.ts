// 🧮️ Per-manifest closure census (lib, test, test+testing) for stdio-contract carve crates; MANIFESTS=comma-separated repo-relative paths.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as any).code === "ENOENT") return null; throw e; } };
const LIST = (process.env.MANIFESTS ?? "").split(",").filter(Boolean);
const variants: [string, string[]][] = [["lib", ["debug_assertions"]], ["test", ["debug_assertions", "test"]], ["test+feat", ["debug_assertions", "test", ...(process.env.FEATURES ?? "").split(",").filter(Boolean).map(f => "feature=" + f)]]];
for (const rel of LIST) {
  const manifest = join(ROOT, rel), dir = dirname(manifest);
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const entry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  for (const [label, extra] of variants) {
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...hostCfg, ...extra], readFile, {manifestDirectory: dir, generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      const over = files > 32 || bytes > 262144 || lines > 4000;
      console.log([parsed.package.name, label, "files", files, "bytes", bytes, "lines", lines, over ? "OVER" : "ok"].join("\t"));
      if (process.env.LIST_FILES && label === process.env.LIST_FILES) for (const i of inputs) console.log("   ", i.kind, i.bytes, i.activeLines, i.path.replace(/^.*?\/📇️registry\//u, ""));
    } catch (e) { console.log(parsed.package.name, label, "ERROR", String((e as Error).message).slice(0, 200)); }
  }
}
