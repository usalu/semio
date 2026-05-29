#!/usr/bin/env bun
/** 🧩 Nakagin kit-compat extraction: `bun ./script.ts extract [--write-board]`. */
const cmd = process.argv[2] ?? "extract";
if (cmd === "extract") {
  await import("./extract-nakagin-kit-compat.ts");
} else {
  console.error("usage: bun ./script.ts extract [--write-board]");
  process.exit(1);
}
