#!/usr/bin/env bun
import { writeFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
const ticket = dirname(fileURLToPath(import.meta.url));
const repo = "/Users/ueli/Documents/semio";
process.chdir(repo);
// Dynamic import of script policy internals is hard; instead reimplement thin wrappers by reading and eval? 
// Call bun script with a timeout via spawning policy is slow.
// Inline: import the module and call policy - but it runs ALL rules.
const mod = await import(repo + "/📜️script.ts");
// policy export returns all breaches - filter handcrafted
const all = mod.policy?.({} ) ?? mod.default?.({}) ?? null;
console.log("policy typeof", typeof mod.policy, Object.keys(mod).slice(0,20));
