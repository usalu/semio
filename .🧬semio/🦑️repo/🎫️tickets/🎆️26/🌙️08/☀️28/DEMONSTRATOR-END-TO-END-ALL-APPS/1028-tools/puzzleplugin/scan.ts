import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { readFileSync } from "node:fs";
import { dirname } from "node:path";
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { normalize, join } from "node:path";
const manifests = execFileSync("rg", ["-l", "semio-framework-plugin-decl-views", "-g", "Cargo.toml", "/Users/ueli/Documents/semio"], { encoding: "utf8", maxBuffer: 1 << 26 }).split("\n").filter(x => x && !x.includes("node_modules") && !x.includes("/.cache/"));
const jobs = manifests.map(m => {
  const d = dirname(m); const t: any = Bun.TOML.parse(readFileSync(m, "utf8")); const entries: string[] = [];
  const lib = t.lib?.path ?? (existsSync(d + "/src/lib.rs") ? "src/lib.rs" : null);
  if (lib) entries.push(normalize(join(d, lib)));
  for (const k of ["test", "bin", "example", "bench"]) for (const x of t[k] ?? []) if (x.path) entries.push(normalize(join(d, x.path)));
  return { manifest: m, entries };
});
const cfg = [...execFileSync("rustc", ["--print", "cfg"], { encoding: "utf8" }).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q))), "debug_assertions", "test"];
const pats: Record<string, RegExp> = {
  media: /semio_framework_plugin_decl_media/, tool: /semio_framework_plugin_decl_tool_contract/, staged: /semio_framework_plugin_decl_staged_form/,
};
const read = async (p: string) => { try { return readFileSync(p); } catch (e: any) { if (e.code === "ENOENT") return null; throw e; } };
const result: any[] = [];
for (const job of jobs) {
  const need = new Set<string>(); const files: string[] = []; let err = "";
  for (const entry of job.entries) {
    try {
      const inputs = await readRustSourceClosureV1([entry], "/", cfg, read, { manifestDirectory: dirname(entry), generated: () => undefined });
      for (const i of inputs) { const s = readFileSync("/" + i.path.replace(/^\//, ""), "utf8"); for (const [k, r] of Object.entries(pats)) if (r.test(s)) { need.add(k); files.push(k + ":" + i.path); } }
    } catch (e: any) { err += String(e.message).slice(0, 120) + "; "; }
  }
  result.push({ manifest: job.manifest, need: [...need], err, files: files.slice(0, 3) });
}
console.log(JSON.stringify(result));
