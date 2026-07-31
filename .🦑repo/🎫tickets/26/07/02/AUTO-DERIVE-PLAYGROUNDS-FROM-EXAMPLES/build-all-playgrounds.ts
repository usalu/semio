/** @emoji ✅ Builds all 24 playground apps and records results. */
import { join } from "node:path";
import { writeFileSync } from "node:fs";

const repoRoot = join(import.meta.dir, "../../../../../../");
const bunBin = process.execPath;
const devDir = join(repoRoot, "framework/product/playground/dev");
const apps = [
  "draw",
  "note",
  "writer",
  "raster",
  "forms",
  "flow",
  "dag",
  "imperative",
  "sequence",
  "layout",
  "lowpoly",
  "procedural-2d",
  "procedural-3d",
  "shooting",
  "s",
  "vcs",
  "gis-2d",
  "wires",
  "trinity-jack",
  "trinity-rewrite",
  "presentation",
  "cad",
  "2d",
  "3d",
  "5d",
];

const lines: string[] = [`# Playground Build Verification — ${new Date().toISOString()}`, ""];
let failed = 0;

for (const app of apps) {
  const proc = Bun.spawn([bunBin, "run", "script.ts", "build", "--app", app], {
    cwd: devDir,
    stdout: "pipe",
    stderr: "pipe",
    env: { ...process.env },
  });
  const code = await proc.exited;
  const err = await new Response(proc.stderr).text();
  const ok = code === 0;
  if (!ok) failed += 1;
  lines.push(`- ${app}: ${ok ? "✅" : "❌"} (exit ${code})`);
  if (!ok && err) {
    const snippet = err
      .split("\n")
      .filter((l) => /error|not exported|Could not resolve/i.test(l))
      .slice(-3)
      .join(" | ");
    if (snippet) lines.push(`  - ${snippet}`);
  }
  console.log(`${app}: ${ok ? "ok" : "FAIL"}`);
}

lines.push("", `**Summary:** ${apps.length - failed}/${apps.length} passed`);
writeFileSync(join(import.meta.dir, "build-all-verify-log.md"), lines.join("\n"));
process.exit(failed > 0 ? 1 : 0);
