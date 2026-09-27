#!/usr/bin/env bun
/** 🔏️ ST2 runner: executes the patched receipt law (structural Nx ownership + Nx replay oracle) against one workspace root. */
import { mkdirSync } from "node:fs";
import { join } from "node:path";

const workspace = process.argv[2] ?? "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-cx1-overlay";
const generated = join(import.meta.dir, "generated", "nx1b-law");
mkdirSync(generated, { recursive: true });
const { testGeneratorInputReceipt } = await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔏️receipt/🟦️.ts"));
await testGeneratorInputReceipt(workspace, generated);
