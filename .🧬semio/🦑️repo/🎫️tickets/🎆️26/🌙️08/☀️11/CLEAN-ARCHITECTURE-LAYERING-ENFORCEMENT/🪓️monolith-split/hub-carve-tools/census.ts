// 🧮️ Hub-workspace census: every target of the named hub crates under the canonical counter, for the plain, test and integration-fixtures selections.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const lenient = process.env.LENIENT === "1";
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return lenient && !p.endsWith(".rs") ? new Uint8Array() : null; throw e; } };
const hub = Bun.TOML.parse(readFileSync(join(ROOT, "🌎️hub/Cargo.toml"), "utf8")) as any;
const only = (process.env.ONLY ?? "").split(",").filter(Boolean);
const variants: [string, string[]][] = [["plain", []], ["test", ["test"]], ["test+fix", ["test", "feature=integration-fixtures"]], ["fix", ["feature=integration-fixtures"]]];
for (const rel of hub.workspace.members as string[]) {
  const dir = resolve(ROOT, "🌎️hub", rel);
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) continue;
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const name = parsed.package?.name as string;
  if (only.length && !only.includes(name)) continue;
  const targets: [string, string][] = [];
  const lib = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  if (existsSync(lib)) targets.push(["lib", lib]);
  for (const b of parsed.bin ?? []) targets.push(["bin:" + b.name, resolve(dir, b.path)]);
  for (const t of parsed.test ?? []) targets.push(["test:" + t.name, resolve(dir, t.path)]);
  for (const [tn, entry] of targets) for (const [vn, extra] of variants) {
    if (tn.startsWith("test:") && vn !== "plain") continue;
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      console.log([name, tn, vn, "files", inputs.length, "bytes", bytes, "lines", lines, "emb", emb.length, (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : "ok"].join("\t"));
    } catch (e) { console.log([name, tn, vn, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
