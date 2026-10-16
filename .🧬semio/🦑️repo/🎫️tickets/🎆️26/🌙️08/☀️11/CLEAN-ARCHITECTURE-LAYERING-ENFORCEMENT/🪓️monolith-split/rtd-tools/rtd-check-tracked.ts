// 🔗️ Verifies that every ui-producer path dependency of the listed manifests names a git-known directory (byte-exact, via git ls-files). argv[2]: manifest list.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {readFileSync} from "node:fs";
import {join, normalize, relative} from "node:path";
import {execFileSync} from "node:child_process";
const ROOT = "/Users/ueli/Documents/semio";
const tracked = new Set(execFileSync("git", ["-c", "core.quotepath=false", "ls-files", "--", "*Cargo.toml"], {cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28}).split("\n"));
let n = 0, bad = 0;
for (const m of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const t = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any;
  const dir = join(ROOT, m, "..");
  for (const table of ["dependencies", "dev-dependencies"]) for (const [k, v] of Object.entries(t[table] ?? {}) as [string, any][]) {
    if (typeof v !== "object" || typeof v.path !== "string" || !k.startsWith("semio-framework-ui")) continue;
    n++;
    const rel = relative(ROOT, normalize(join(dir, v.path, "Cargo.toml")));
    if (!tracked.has(rel)) { bad++; console.log("NOT-IN-GIT-INDEX", t.package.name, k, rel.slice(-90)); }
  }
}
console.log(n, "ui producer path deps checked;", bad, "not in the git index");
