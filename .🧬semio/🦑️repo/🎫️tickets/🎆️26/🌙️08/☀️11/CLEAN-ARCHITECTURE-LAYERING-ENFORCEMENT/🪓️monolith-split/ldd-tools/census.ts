/** 🧮️ Source-boundary census of selected crates (own mounted closure per lib target) at a git revision or the working tree, with the canonical closure counter. Read-only. */
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { execFileSync, spawnSync } from "node:child_process";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const [revision, ...dirs] = process.argv.slice(2);
const LIMIT = { files: 32, bytes: 262144, lines: 4000 };
const rustc = execFileSync("rustup", ["which", "rustc"], { encoding: "utf8" }).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], { encoding: "utf8" }).trim().split("\n").map((value) => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));

const read = (path: string): Uint8Array | null => {
  if (revision === "WORKTREE") { try { return readFileSync(path); } catch { return null; } }
  const rel = relative(ROOT, path);
  const out = spawnSync("git", ["-c", "core.quotepath=false", "show", `${revision}:${rel}`], { cwd: ROOT, maxBuffer: 1 << 28 });
  return out.status === 0 ? new Uint8Array(out.stdout) : null;
};

for (const dir of dirs) {
  const abs = resolve(ROOT, dir);
  const manifestText = read(join(abs, "Cargo.toml"));
  if (!manifestText) { console.log(`${dir}\tABSENT at ${revision}`); continue; }
  const parsed = Bun.TOML.parse(new TextDecoder().decode(manifestText)) as any;
  const declared = parsed.package?.metadata?.semio?.["rust-source-boundary"];
  const limit = declared ? { files: Math.min(LIMIT.files, declared.maxSourceFiles), bytes: Math.min(LIMIT.bytes, declared.maxSourceBytes), lines: Math.min(LIMIT.lines, declared.maxActiveLines) } : LIMIT;
  const entry = resolve(abs, parsed.lib?.path ?? "src/lib.rs");
  const features = ["default", ...Object.keys(parsed.features ?? {}).filter((f) => f === "default")];
  const cfg = [...hostCfg, "debug_assertions", ...features.map((f) => "feature=" + f)];
  try {
    const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, async (path: string) => read(path), { manifestDirectory: abs, generated: () => undefined });
    const files = inputs.length, bytes = inputs.reduce((s: number, i: any) => s + i.bytes, 0), lines = inputs.reduce((s: number, i: any) => s + i.activeLines, 0);
    const over = files > limit.files || bytes > limit.bytes || lines > limit.lines;
    console.log(`${parsed.package?.name}\t${files}/${limit.files} files\t${bytes}/${limit.bytes} bytes\t${lines}/${limit.lines} lines\t${over ? "OVER" : "ok"}`);
  } catch (error) {
    console.log(`${parsed.package?.name}\tREFUSED ${String((error as Error).message ?? error).replaceAll("\n", " ").slice(0, 160)}`);
  }
}
