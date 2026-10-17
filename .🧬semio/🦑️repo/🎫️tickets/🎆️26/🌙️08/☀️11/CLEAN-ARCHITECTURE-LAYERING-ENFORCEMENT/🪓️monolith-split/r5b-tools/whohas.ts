// 🔎️ Lists the crates of the pdf artifact whose lib closure contains a file (substring match).
import {readFileSync, existsSync, readdirSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const art = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf");
const needles = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const manifests: string[] = [];
const walk = (dir: string) => { for (const e of readdirSync(dir, {withFileTypes: true})) { if (e.name === "node_modules" || e.name === "dist" || e.name.startsWith(".")) continue; const p = join(dir, e.name); if (e.isDirectory()) walk(p); else if (e.name === "Cargo.toml") manifests.push(p); } };
walk(art);
for (const m of manifests) {
  const parsed = Bun.TOML.parse(readFileSync(m, "utf8")) as any;
  if (!parsed.package?.name) continue;
  const entry = resolve(dirname(m), parsed.lib?.path ?? "src/lib.rs");
  if (!existsSync(entry)) continue;
  try {
    const inputs = await readRustSourceClosureV1([entry], root, cfg, async (p: string) => { try { return readFileSync(p); } catch { return null; } }, {manifestDirectory: dirname(m), generated: () => undefined});
    for (const n of needles) if (inputs.some(i => i.path.includes(n))) console.log(parsed.package.name, "has", n);
  } catch (e) { console.error("skip", parsed.package.name, (e as Error).message.slice(0, 80)); }
}
