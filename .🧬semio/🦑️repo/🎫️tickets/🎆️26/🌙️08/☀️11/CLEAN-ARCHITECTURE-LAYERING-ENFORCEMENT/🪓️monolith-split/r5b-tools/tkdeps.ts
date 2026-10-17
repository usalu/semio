// 🔎️ Checks that every crate whose sources name `schema::testkit` depends on the testkit crate (dependencies or dev-dependencies) with the feature enabled where it is a dev-dependency.
import {readFileSync, existsSync, readdirSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const art = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf");
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const manifests: string[] = [];
const walk = (dir: string) => { for (const e of readdirSync(dir, {withFileTypes: true})) { if (e.name === "node_modules" || e.name === "dist" || e.name.startsWith(".")) continue; const p = join(dir, e.name); if (e.isDirectory()) walk(p); else if (e.name === "Cargo.toml") manifests.push(p); } };
walk(art);
const TK = "semio-s-artifact-stdio-pdf-testkit";
let bad = 0, refs = 0;
for (const m of manifests) {
  const parsed = Bun.TOML.parse(readFileSync(m, "utf8")) as any;
  const name = parsed.package?.name;
  if (!name || name === TK || !name.startsWith("semio-s-artifact-stdio-pdf")) continue;
  const entry = resolve(dirname(m), parsed.lib?.path ?? "src/lib.rs");
  if (!existsSync(entry)) continue;
  const inputs = await readRustSourceClosureV1([entry], root, [...cfg, "test", 'feature=testkit'], async (p: string) => { try { return readFileSync(p); } catch { return null; } }, {manifestDirectory: dirname(m), generated: () => undefined});
  const uses = inputs.filter(i => i.path.endsWith(".rs") && /schema::testkit\b/u.test(readFileSync(resolve(root, i.path), "utf8")));
  if (!uses.length) continue;
  refs++;
  const dep = parsed.dependencies?.[TK], dev = parsed["dev-dependencies"]?.[TK];
  const devOk = dev && (dev.features ?? []).includes("testkit");
  if (!dep && !dev) { bad++; console.log("NO DEPENDENCY", name, uses.length, "files"); }
  else if (!devOk && !dep?.features?.includes("testkit")) { bad++; console.log("FEATURE NOT ENABLED", name); }
}
console.log("crates referencing testkit", refs, "problems", bad);
