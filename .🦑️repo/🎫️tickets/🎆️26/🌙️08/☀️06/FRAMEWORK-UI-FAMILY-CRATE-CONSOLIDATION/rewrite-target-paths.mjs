#!/usr/bin/env bun
// 🔁️ Scratch: rewrites the two former root-crate lib.rs files (and their co-located 🧱️elements
// sources) for life inside the merged `semio-framework-ui` crate — `crate::` becomes
// `crate::<target>::`, and the per-target feature gates get their new namespaced names.
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const UI = "🧰️framework/🔨️modules/🖱️ui";
const RUST = `${UI}/📦️packages/🦀️rust`;

const rules = [
  { file: `${RUST}/🎯️targets/⌨️tui/📦️lib.rs`, subs: [[/crate::/g, "crate::tui::"], [/feature = "terminal"/g, 'feature = "tui-terminal"'], [/feature = "bindgen"/g, 'feature = "tui-bindgen"']] },
  { file: `${RUST}/🎯️targets/🧊️wgpu/📦️lib.rs`, subs: [[/crate::/g, "crate::wgpu::"], [/feature = "engine"/g, 'feature = "wgpu-engine"']] },
];

function elementSources() {
  const out = [];
  const walk = (dir) => {
    for (const entry of readdirSync(dir)) {
      const full = join(dir, entry);
      if (statSync(full).isDirectory()) walk(full);
      else if (entry === "⌨️component.rs") out.push({ file: full, subs: [[/crate::/g, "crate::tui::"], [/feature = "terminal"/g, 'feature = "tui-terminal"']] });
      else if (entry === "🧊️component.rs") out.push({ file: full, subs: [[/crate::/g, "crate::wgpu::"], [/feature = "engine"/g, 'feature = "wgpu-engine"']] });
    }
  };
  walk(`${UI}/🧱️elements`);
  return out;
}

let total = 0;
for (const { file, subs } of [...rules, ...elementSources()]) {
  const before = readFileSync(file, "utf8");
  let after = before;
  let hits = 0;
  for (const [pattern, replacement] of subs) {
    hits += (after.match(pattern) ?? []).length;
    after = after.replace(pattern, replacement);
  }
  if (after !== before) writeFileSync(file, after);
  total += hits;
  console.log(`${hits.toString().padStart(4)}  ${file}`);
}
console.log(`total substitutions: ${total}`);
