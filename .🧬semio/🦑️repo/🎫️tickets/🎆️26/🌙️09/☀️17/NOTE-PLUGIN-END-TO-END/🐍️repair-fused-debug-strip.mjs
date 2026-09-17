#!/usr/bin/env bun
/** 🩹️ Repairs files damaged by the fused-`if` bug of 26/09/17/DEMONSTRATOR-REMOVE-DEBUG-CONSOLE's codemod: for each file,
 * buggy(HEAD) → fixed(HEAD) is merged into the working tree (`git merge-file --ours`: later hand-edits win), so unrelated peer edits survive.
 * Usage: bun 🐍️repair-fused-debug-strip.mjs <files...> (run from the repo root). */
import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, writeFileSync, copyFileSync } from "node:fs";
import { join, basename } from "node:path";
import { tmpdir } from "node:os";

const buggy = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/DEMONSTRATOR-REMOVE-DEBUG-CONSOLE/🐍️strip-debug-console.mjs";
const fixed = join(import.meta.dir, "🐍️strip-debug-console-fixed.mjs");
const work = mkdtempSync(join(process.env.SEMIO_REPAIR_TMP ?? tmpdir(), "fused-"));
for (const [index, file] of process.argv.slice(2).entries()) {
  const ext = basename(file).endsWith(".tsx") ? ".tsx" : ".ts";
  const head = execFileSync("git", ["show", `HEAD:${file}`], { maxBuffer: 1 << 28 });
  const base = join(work, `${index}-buggy${ext}`), target = join(work, `${index}-fixed${ext}`);
  writeFileSync(base, head);
  writeFileSync(target, head);
  execFileSync("bun", [buggy, base], { stdio: "pipe" });
  execFileSync("bun", [fixed, target], { stdio: "pipe" });
  copyFileSync(file, join(work, `${index}-before${ext}`));
  const into = process.env.SEMIO_REPAIR_DRY === "1" ? join(work, `${index}-before${ext}`) : file;
  const merge = spawnSync("git", ["merge-file", "--ours", "-L", "current", "-L", "buggy", "-L", "fixed", into, base, target], { encoding: "utf8" });
  const delta = spawnSync("git", ["diff", "--no-index", "--numstat", base, target], { encoding: "utf8" }).stdout.trim().split(/\s+/).slice(0, 2).join("/");
  console.log(`[DEBUG] ${merge.status === 0 ? "clean" : `conflicts=${merge.status}`} fix=${delta || "none"} ${file}`);
}
console.log(`[DEBUG] work ${work}`);
