#!/usr/bin/env node

import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";
import { applyKitDiff, deepEqual } from "../js/js/semio.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT = path.join(__dirname, "..");

async function main() {
  const kitPath = path.join(ROOT, "assets", "semio", "kit_metabolism.json");
  const diffPath = path.join(ROOT, "assets", "semio", "diff_kit_metabolism.json");
  const diffedPath = path.join(ROOT, "assets", "semio", "kit_metabolism_diffed.json");

  const kit = JSON.parse(await fs.readFile(kitPath, "utf-8"));
  const diff = JSON.parse(await fs.readFile(diffPath, "utf-8"));
  const expected = JSON.parse(await fs.readFile(diffedPath, "utf-8"));

  console.log("Applying diff...");
  const applied = applyKitDiff(kit, diff);

  console.log("\nApplied name:", applied.name);
  console.log("Expected name:", expected.name);
  console.log("Match:", applied.name === expected.name);

  console.log("\nApplied version:", applied.version);
  console.log("Expected version:", expected.version);
  console.log("Match:", applied.version === expected.version);

  console.log("\nApplied authors count:", applied.authors?.length);
  console.log("Expected authors count:", expected.authors?.length);

  console.log("\nApplied attributes count:", applied.attributes?.length);
  console.log("Expected attributes count:", expected.attributes?.length);

  console.log("\nDeep equal:", deepEqual(applied, expected));

  if (!deepEqual(applied, expected)) {
    console.log("\n=== Finding differences ===");
    
    const keys = new Set([...Object.keys(applied), ...Object.keys(expected)]);
    for (const key of keys) {
      if (!deepEqual(applied[key], expected[key])) {
        console.log(`\nDifference in '${key}':`);
        if (Array.isArray(applied[key]) && Array.isArray(expected[key])) {
          console.log(`  Applied length: ${applied[key].length}`);
          console.log(`  Expected length: ${expected[key].length}`);
        }
      }
    }
  }
}

main().catch(console.error);
