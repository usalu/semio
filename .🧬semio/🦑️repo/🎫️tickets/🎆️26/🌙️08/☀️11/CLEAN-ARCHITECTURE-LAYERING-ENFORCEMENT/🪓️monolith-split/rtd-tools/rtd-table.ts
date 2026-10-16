// 🧾️ Per-crate change table (markdown rows) from baseline vs live manifests and the pre/post reference scans. argv: base dir, manifest list, refs-before json, refs-after json.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {readFileSync} from "node:fs";
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const [base, list, beforeFile, afterFile] = process.argv.slice(2) as [string, string, string, string];
const testCount = (file: string) => {
  const out: Record<string, number> = {}, norm: Record<string, number> = {};
  for (const c of JSON.parse(readFileSync(file, "utf8")) as any[]) {
    out[c.crate] = c.refs.filter((r: any) => r.where === "testfile" || r.where === "testregion").length;
    norm[c.crate] = c.refs.filter((r: any) => r.where === "normal" || r.where === "testregion-feature").length;
  }
  return {out, norm};
};
const b = testCount(beforeFile), a = testCount(afterFile);
const keys = (t: any, tbl: string) => new Set(Object.keys(t[tbl] ?? {}));
const rows: string[] = [];
for (const m of readFileSync(list, "utf8").split("\n").filter(Boolean)) {
  const t0 = Bun.TOML.parse(readFileSync(join(base, "manifests", m), "utf8")) as any, t1 = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any;
  const name = t1.package.name as string;
  const dev0 = keys(t0, "dev-dependencies"), dev1 = keys(t1, "dev-dependencies"), dep0 = keys(t0, "dependencies"), dep1 = keys(t1, "dependencies");
  const addedDev = [...dev1].filter(k => !dev0.has(k)).map(k => k.replace("semio-framework-ui-", ""));
  const devRemoved = dev0.has("ui_wgpu") && !dev1.has("ui_wgpu"), normalRemoved = dep0.has("ui_wgpu") && !dep1.has("ui_wgpu");
  const uiNormal = dep1.has("ui_wgpu"), uiDev = dev1.has("ui_wgpu");
  const changed = devRemoved || normalRemoved || addedDev.length || b.out[name] !== a.out[name];
  if (!changed) continue;
  rows.push(`| \`${name.replace("semio-framework-os-renderer-", "")}\` | ${b.out[name] ?? 0} -> ${a.out[name] ?? 0} | ${devRemoved ? "removed" : dev0.has("ui_wgpu") ? "kept (testkit)" : "none"} | ${normalRemoved ? "removed" : uiNormal ? "kept" : uiDev ? "n/a" : "none"} | ${addedDev.length ? addedDev.join(", ") : "-"} |`);
}
console.log(rows.join("\n"));
console.log(rows.length, "rows");
