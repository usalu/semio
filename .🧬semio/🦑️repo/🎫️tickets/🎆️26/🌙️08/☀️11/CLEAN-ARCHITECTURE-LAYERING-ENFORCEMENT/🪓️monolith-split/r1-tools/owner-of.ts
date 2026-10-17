import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [art, ...needles] = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], { encoding: "utf8" }).trim();
const host = execFileSync(rustc, ["--print", "cfg"], { encoding: "utf8" }).trim().split("\n").map((v) => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
const manifests = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-co", "--exclude-standard", "-z", "--", art!], { cwd: ROOT, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter((p) => p.endsWith("/Cargo.toml"));
for (const m of manifests) {
  const dir = dirname(join(ROOT, m)); const parsed: any = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8"));
  if (!parsed.package?.name || !parsed.lib?.path) continue;
  const entry = resolve(dir, parsed.lib.path); if (!existsSync(entry)) continue;
  const inputs = (await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", "test"], readFile as any, { manifestDirectory: dir, generated: () => undefined })) as any[];
  for (const i of inputs) for (const n of needles) if (String(i.path).endsWith(n)) console.log(parsed.package.name, "<-", m.replace(/\/Cargo.toml$/u, "").slice(-60), "|", n.slice(-60));
}
