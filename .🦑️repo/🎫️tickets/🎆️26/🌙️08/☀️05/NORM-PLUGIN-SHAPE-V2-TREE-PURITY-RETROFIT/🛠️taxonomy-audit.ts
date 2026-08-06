#!/usr/bin/env bun
/** 🧭️ Ticket-local mirror of the registry's `validateTaxonomyTree` + root `📜️script.ts`'s
 * `policyTaxonomyDirs`/`policyComponentFile`/`policySprNaming` lints, run against 📕️norm alone. The real
 * `registry check` needs a healthy root workspace + regenerated registry, which only exists after the
 * registrar pass — this reproduces the same findings locally so the migration can be proven green first.
 * Ticket-scoped throwaway (CLAUDE.md); nothing under ✏️s/ references it.
 *
 * 🔁️ UPDATED for the Shape V2 Tree Purity retrofit (ticket
 * 26/08/05/NORM-PLUGIN-SHAPE-V2-TREE-PURITY-RETROFIT): `📦️lib.rs` now lives at
 * `📦️packages/🦀️rust/📦️lib.rs`, not the plugin root, so every `#[path]` string is now written in full
 * relative to THAT directory (leaf paths carry a `../../` prefix to climb back to the owner root; the
 * flat `join(dirname(libRs), p)` model is correct here because every leaf spells out its complete path
 * from lib.rs's own directory — this plugin does not use raster's alternate "reset-once" style). */
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative, dirname } from "node:path";

const PLUGIN_ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm";
const LEAF = "🦀️component.rs";
const ARTIFACT_COMPONENTS = ["🔺️diff", "🗣️dsl", "🎒️pack", "🔧️op", "📡️spr"];
const ARTIFACT_COMPONENT_DIRS = [...ARTIFACT_COMPONENTS, "⚙️engine"];
const WINDOW_CHILDREN = new Set(["🍱️panes", "🪀️widgets", "🪛️utilities", "🎬️actions", "🎚️options"]);
const LEAF_PARENT_DIRS = [...ARTIFACT_COMPONENTS, ...WINDOW_CHILDREN, "🎮️commands", "🛠️tools", "📌️panels"];

const findings: string[] = [];
const dirs = (p: string): string[] => (existsSync(p) ? readdirSync(p).filter((n) => statSync(join(p, n)).isDirectory()) : []);

// 1️⃣ every artifact carries the five constitutional components, and only known component dirs.
const artifactsDir = join(PLUGIN_ROOT, "🗿️artifacts");
for (const artifact of dirs(artifactsDir)) {
  for (const component of ARTIFACT_COMPONENTS) {
    if (!existsSync(join(artifactsDir, artifact, component, LEAF))) findings.push(`artifact "${artifact}" is missing ${component}/${LEAF}`);
  }
  for (const child of dirs(join(artifactsDir, artifact))) {
    if (!ARTIFACT_COMPONENT_DIRS.includes(child)) findings.push(`"🗿️artifacts/${artifact}/${child}" is not a recognized artifact component dir`);
  }
}

// 2️⃣ window dirs may only contain the fixed child set.
const appsDir = join(PLUGIN_ROOT, "🎛️apps");
for (const app of dirs(appsDir)) {
  for (const mode of dirs(join(appsDir, app, "🎭️modes"))) {
    const windowsDir = join(appsDir, app, "🎭️modes", mode, "🪟️windows");
    for (const w of dirs(windowsDir)) {
      for (const child of dirs(join(windowsDir, w))) {
        if (!WINDOW_CHILDREN.has(child)) findings.push(`window "${app}/${mode}/${w}" has unexpected child "${child}"`);
      }
    }
  }
}

// 3️⃣ collect every component.rs, flag misnamed taxonomy leaves, flag retired "protocol" segments.
const componentFiles: string[] = [];
const walk = (dir: string): void => {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
    const path = join(dir, name);
    const stem = name.replace(/\.rs$/, "").replace(/[^\x00-\x7F]/g, "");
    if (/^protocol$/i.test(stem)) findings.push(`"${relative(PLUGIN_ROOT, path)}" uses the retired "protocol" name (the taxonomy concept is "spr")`);
    if (statSync(path).isDirectory()) {
      walk(path);
      continue;
    }
    if (!name.endsWith(".rs")) continue;
    if (name === LEAF) componentFiles.push(path);
    else if (LEAF_PARENT_DIRS.includes(dir.split("/").pop() ?? "")) findings.push(`taxonomy leaf file must be named ${LEAF}, found ${relative(PLUGIN_ROOT, path)}`);
  }
};
walk(PLUGIN_ROOT);

// 4️⃣ lib.rs declares a #[path] for every component.rs on disk, and none of them dangle. lib.rs now
// lives at 📦️packages/🦀️rust/📦️lib.rs (Shape V2) — every leaf #[path] string is written in full
// relative to THAT directory (verified for real via `cargo check`/`cargo test`, see ticket report).
const libRs = join(PLUGIN_ROOT, "📦️packages/🦀️rust/📦️lib.rs");
const libRsDir = dirname(libRs);
if (!existsSync(libRs)) findings.push("missing 📦️lib.rs at 📦️packages/🦀️rust/");
else {
  const declared = [...readFileSync(libRs, "utf8").matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)].map((m) => m[1]!).filter((p) => p !== ".");
  const declaredAbs = new Set(declared.map((p) => join(libRsDir, p)));
  for (const file of componentFiles) if (!declaredAbs.has(file)) findings.push(`${relative(PLUGIN_ROOT, file)} is not declared by any #[path] in 📦️lib.rs`);
  for (const p of declared) if (p.endsWith(".rs") && !existsSync(join(libRsDir, p))) findings.push(`📦️lib.rs declares #[path = "${p}"] but the file does not exist on disk`);
}

// 5️⃣ lib.rs must stay wiring-only (TaxonomyLibShape's spirit: no #[path] file bodies inlined back).
const libBody = existsSync(libRs) ? readFileSync(libRs, "utf8") : "";
const suspicious = libBody.split(/\r?\n/).filter((l) => /^\s*(pub\s+)?(struct|enum|trait|impl)\s/.test(l));
if (suspicious.length > 0) findings.push(`📦️lib.rs contains ${suspicious.length} type/impl declaration(s) — it must stay #[path] wiring + semio_plugin! only`);

console.log(`📦️ component files on disk: ${componentFiles.length}`);
console.log(`🗿️ artifacts: ${dirs(artifactsDir).length}   🎛️ apps: ${dirs(appsDir).length}`);
if (findings.length === 0) console.log("✅️ taxonomy tree clean: 📕️norm");
else {
  console.log(`❌️ ${findings.length} finding(s):`);
  for (const f of findings) console.log(`   - ${f}`);
  process.exit(1);
}
