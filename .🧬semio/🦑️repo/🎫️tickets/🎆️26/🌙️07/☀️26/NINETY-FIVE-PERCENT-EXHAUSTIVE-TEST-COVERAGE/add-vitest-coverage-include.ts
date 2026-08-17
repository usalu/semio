#!/usr/bin/env bun
/** 📊️One-off codemod: adds `coverage: { include: [...] }` (mirroring each config's own `include`/`includeSource`
 * array) to every real per-project `vitest.config.ts`/`vite.config.ts` under the repo, so untested source files
 * aren't invisible to the exhaustive-level coverage denominator. Idempotent — skips files that already have a
 * `coverage:` block. Ticket: 26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE. */
import { readFileSync, writeFileSync } from "node:fs";

const files = [
  "animate/present/core/js/vitest.config.ts",
  "animate/present/renderer/react/vitest.config.ts",
  "cad/core/js/vitest.config.ts",
  "cad/kernel/brepjs/js/vitest.config.ts",
  "cad/machine/stately/js/vitest.config.ts",
  "cad/module/aec-building-energy/js/vitest.config.ts",
  "cad/module/aec-building-structure/js/vitest.config.ts",
  "cad/module/aec-building/js/vitest.config.ts",
  "cad/module/spatial-shape/js/vitest.config.ts",
  "cad/query/js/vitest.config.ts",
  "cad/renderer/js/vitest.config.ts",
  "cad/runtime/js/vitest.config.ts",
  "compose/client/lib/sketchpad/js/vitest.config.ts",
  "compose/client/lib/js/vite.config.ts",
  "compose/dev/algorithm/js/vitest.config.ts",
  "framework/core/js/vitest.config.ts",
  "framework/product/os/core/js/vitest.config.ts",
  "infinite/canvas/react-renderer/vitest.config.ts",
  "infinite/world/r3f/vitest.config.ts",
  "kernel/2d/js/vitest.config.ts",
  "kernel/3d/brep/js/vitest.config.ts",
  "mathematical/graph/dsl/core/js/vitest.config.ts",
  "mit-bestand/präsentation/33.projektetage/js/vitest.config.ts",
  "ui/js/react/vitest.config.ts",
  "ui/styling/vitest.config.ts",
];

// Run from the repo root: `bun .repo/🎫️/26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE/add-vitest-coverage-include.ts`
const repoRoot = process.cwd();

let changed = 0;
let skipped = 0;
for (const rel of files) {
  const path = `${repoRoot}/${rel}`;
  const text = readFileSync(path, "utf8");
  if (text.includes("coverage:")) {
    console.log(`[skip] ${rel} — already has a coverage block`);
    skipped++;
    continue;
  }
  const match = /^(\s*)(include|includeSource): (\[[^\]]*\]),\n/m.exec(text);
  if (!match) {
    console.error(`[MISS] ${rel} — no include/includeSource line found, needs manual coverage.include`);
    continue;
  }
  const [line, indent, , arrayLiteral] = match;
  const insertion = `${indent}coverage: { include: ${arrayLiteral} },\n`;
  const next = text.slice(0, match.index) + line + insertion + text.slice(match.index + line.length);
  writeFileSync(path, next);
  console.log(`[ok]   ${rel}`);
  changed++;
}
console.log(`\n${changed} changed, ${skipped} already had coverage.`);
