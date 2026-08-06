#!/usr/bin/env bun
/** 🔎️ Confirms the shared discovery contract sees the consolidated crate as a `role = "framework"` package. */
import { generateFrameworkPackageRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts";

const packages = generateFrameworkPackageRegistry();
for (const pkg of packages) console.log(JSON.stringify(pkg));
console.log(`framework packages total: ${packages.length}`);
