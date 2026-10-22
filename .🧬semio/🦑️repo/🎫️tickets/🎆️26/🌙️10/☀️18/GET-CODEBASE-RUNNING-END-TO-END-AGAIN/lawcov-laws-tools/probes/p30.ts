import { copyFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
const LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const REAL = "/Users/ueli/Documents/semio/" + LIB;
const root = mkdtempSync(join(realpathSync(tmpdir()), "p30-"));
for (const rel of ["🔣️taxonomy.json", "🧬️schema/🔣️.json"]) { mkdirSync(dirname(join(root, LIB, rel)), { recursive: true }); copyFileSync(join(REAL, rel), join(root, LIB, rel)); }
writeFileSync(join(root, "README.md"), "x\n");
spawnSync("git", ["init", "--quiet"], { cwd: root }); spawnSync("git", ["add", "-A"], { cwd: root });
process.env.SEMIO_WORKSPACE_ROOT = root;
const { policyBreaches } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts");
const t = Date.now();
try { const b = policyBreaches(); console.log(b.length, Date.now() - t, "ms", JSON.stringify(b.slice(0, 8).map((x) => [x.id, x.kind]))); } catch (e) { console.log("ERR", String(e).slice(0, 400)); }
rmSync(root, { recursive: true, force: true });
