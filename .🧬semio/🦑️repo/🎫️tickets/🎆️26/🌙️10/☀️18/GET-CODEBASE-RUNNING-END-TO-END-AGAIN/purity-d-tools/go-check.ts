import { writeFileSync, mkdtempSync, rmSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { canonicalGoPlan } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts";
// usage: bun go-check.ts <vet|test> <module root (the dir with go.mod)> ...
const [, , action, ...roots] = process.argv;
const scratch = mkdtempSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/3d0a4ac3-5d70-4c6b-91df-262fe6307e4a/scratchpad/go-");
let failed = 0;
for (const root of roots) {
  const plan = canonicalGoPlan(root);
  const overlay = join(scratch, "overlay.json");
  writeFileSync(overlay, JSON.stringify({ Replace: plan.replacements }));
  const args = action === "build" ? ["build", `-overlay=${overlay}`, ...plan.packages] : action === "test" ? ["test", "-short", "-count=1", `-overlay=${overlay}`, ...plan.packages] : ["vet", `-overlay=${overlay}`, ...plan.packages];
  const run = spawnSync("go", args, { cwd: root, encoding: "utf8", env: { ...process.env }, maxBuffer: 1 << 26 });
  const ok = run.status === 0;
  if (!ok) failed += 1;
  console.log(ok ? "OK  " : "FAIL", root.replace(/^.*🔨️modules\//, ""), plan.packages.length, "packages");
  if (!ok) console.log((run.stderr + run.stdout).split("\n").slice(0, 12).join("\n"));
}
rmSync(scratch, { recursive: true, force: true });
process.exit(failed ? 1 : 0);
