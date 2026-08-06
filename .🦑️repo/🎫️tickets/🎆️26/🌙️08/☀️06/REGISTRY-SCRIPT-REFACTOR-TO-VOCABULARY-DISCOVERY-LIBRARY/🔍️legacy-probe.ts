#!/usr/bin/env bun
/** 🔍️ Ticket-local probe: every legacy-sandwich `Cargo.toml` that declares a wasm component package,
 * classified by the pre-refactor regexes, so a generalized vocabulary-driven matcher can be proven to
 * select exactly the same set. Read-only. */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

const root = getWorkspaceRoot();
const hits: string[] = [];
function walk(dir: string) {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || name === "node_modules" || name === "🤖️generated" || name === "target") continue;
    const path = join(dir, name);
    let st: ReturnType<typeof statSync>;
    try {
      st = statSync(path);
    } catch {
      continue;
    }
    if (st.isDirectory()) walk(path);
    else if (name === "Cargo.toml") hits.push(path);
  }
}
walk(root);

const isModuleCrate = (p: string) => /\/🔨️modules\/[^/]+\/⚡️implementations\/🦀️rust\/Cargo\.toml$/.test(p);
const isExtensionCrate = (p: string) => /\/✏️s\/🔌️plugins\/[^/]+\/🧩️extensions\/[^/]+\/⚡️implementations\/🦀️rust\/Cargo\.toml$/.test(p);
const isBundleCrate = (p: string) => /\/✏️s\/🔌️plugins\/[^/]+\/🛂️manifest\/🗿️artifact\/⚡️implementations\/🦀️rust\/Cargo\.toml$/.test(p);
const isAnySandwich = (p: string) => /\/⚡️implementations?\/🦀️rust\/Cargo\.toml$/.test(p);
const hasComponent = (p: string) => /\[package\.metadata\.component\][\s\S]*?^package = "semio:([^"]+)"/m.test(readFileSync(p, "utf8"));

const sandwichWithComponent = hits.filter((p) => isAnySandwich(p) && hasComponent(p));
console.log(`sandwich Cargo.toml with [package.metadata.component]: ${sandwichWithComponent.length}`);
for (const p of sandwichWithComponent.sort()) {
  const tags = [isModuleCrate(p) && "module", isExtensionCrate(p) && "extension", isBundleCrate(p) && "bundle"].filter(Boolean);
  console.log(`  ${tags.length ? tags.join("+") : "*** UNMATCHED-BY-OLD-REGEXES ***"}  ${p.slice(root.length + 1)}`);
}
const nonSandwichWithComponent = hits.filter((p) => !isAnySandwich(p) && hasComponent(p));
console.log(`\nnon-sandwich Cargo.toml with [package.metadata.component]: ${nonSandwichWithComponent.length}`);
for (const p of nonSandwichWithComponent.sort()) console.log(`  ${p.slice(root.length + 1)}`);
