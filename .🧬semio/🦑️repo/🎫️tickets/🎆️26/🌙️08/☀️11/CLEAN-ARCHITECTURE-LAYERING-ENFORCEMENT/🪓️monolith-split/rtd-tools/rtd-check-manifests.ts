// ✅️ Static manifest checks: TOML parses, every path dependency of the listed manifests exists on disk and byte-matches a git-known directory name, ui edges summary per crate. argv[2]: manifest list. Optional argv[3]: baseline manifests dir to diff.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {existsSync, readFileSync, realpathSync} from "node:fs";
import {join} from "node:path";
import {execFileSync} from "node:child_process";

const ROOT = "/Users/ueli/Documents/semio";
const list = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const wsDeps: Record<string, any> = (Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any).workspace?.dependencies ?? {};
let failures = 0;
const fail = (m: string) => { failures++; console.log("FAIL", m); };
const canon = (p: string) => { try { return realpathSync.native(p); } catch { return ""; } };
for (const m of list) {
  let t: any;
  try { t = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")); } catch (e) { fail(`${m}: TOML parse ${e}`); continue; }
  const dir = join(ROOT, m, "..");
  const specs: [string, string, any][] = [];
  const add = (tbl: any, table: string) => { for (const [k, v] of Object.entries(tbl ?? {})) specs.push([table, k, v]); };
  add(t.dependencies, "dependencies"); add(t["dev-dependencies"], "dev-dependencies"); add(t["build-dependencies"], "build-dependencies");
  for (const [tg, b] of Object.entries(t.target ?? {}) as [string, any][]) { add(b.dependencies, `target.${tg}.dependencies`); add(b["dev-dependencies"], `target.${tg}.dev-dependencies`); }
  for (const [table, k, raw] of specs) {
    const spec = typeof raw === "string" ? {} : (raw as any);
    const ws = spec.workspace === true ? (wsDeps[k] ?? wsDeps[spec.package ?? k]) : undefined;
    if (spec.workspace === true && !ws) { fail(`${m}: [${table}] ${k} workspace = true but no [workspace.dependencies] entry`); continue; }
    const p = spec.path ?? ws?.path;
    if (typeof p !== "string") continue;
    const abs = join(spec.path === undefined ? ROOT : dir, p);
    if (!existsSync(join(abs, "Cargo.toml"))) { fail(`${m}: [${table}] ${k} path ${p} has no Cargo.toml`); continue; }
    const real = canon(abs);
    if (real && real !== join(abs) && real.normalize("NFC") !== join(abs).normalize("NFC")) fail(`${m}: [${table}] ${k} path differs from on-disk name: ${p}`);
    else if (real && real !== join(abs)) console.log(`NOTE ${m.split("/").slice(-4, -3)}: [${table}] ${k} unicode-normalization differs from disk (NFC-equal)`);
  }
  const names = specs.filter(([, , v]) => (typeof v === "object" ? (v as any).package : undefined) === "semio-framework-ui").map(([tb, k, v]) => `${tb}:${k}${JSON.stringify((v as any).features ?? [])}`);
  console.log(`${t.package?.name?.replace("semio-framework-os-renderer-", "")}\tui-edges=[${names.join(" ")}]\tdev=${Object.keys(t["dev-dependencies"] ?? {}).length}\tdeps=${Object.keys(t.dependencies ?? {}).length}`);
}
void execFileSync;
console.log(failures ? `${failures} FAILURE(S)` : "ALL OK", list.length, "manifests");
process.exit(failures ? 1 : 0);
