#!/usr/bin/env bun
/** 🧭️ Ticket-local Shape V2 tree-purity audit for ✒️writer (mirrors norm retrofit audit). */
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const PLUGIN_ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer";
const LEAF = "🦀️component.rs";
const ARTIFACT_COMPONENTS = ["🔺️diff", "🗣️dsl", "🎒️pack", "🔧️op", "📡️spr"];
const ARTIFACT_COMPONENT_DIRS = [...ARTIFACT_COMPONENTS, "⚙️engine"];
const WINDOW_CHILDREN = new Set(["🍱️panes", "🪀️widgets", "🪛️utilities", "🎬️actions", "🎚️options"]);
const LEAF_PARENT_DIRS = [...ARTIFACT_COMPONENTS, ...WINDOW_CHILDREN, "🎮️commands", "🛠️tools", "📌️panels"];

const findings: string[] = [];
const dirs = (p: string): string[] => (existsSync(p) ? readdirSync(p).filter((n) => statSync(join(p, n)).isDirectory()) : []);

const artifactsDir = join(PLUGIN_ROOT, "🗿️artifacts");
for (const artifact of dirs(artifactsDir)) {
  for (const component of ARTIFACT_COMPONENTS) {
    if (!existsSync(join(artifactsDir, artifact, component, LEAF))) findings.push(`artifact "${artifact}" is missing ${component}/${LEAF}`);
  }
  for (const child of dirs(join(artifactsDir, artifact))) {
    if (!ARTIFACT_COMPONENT_DIRS.includes(child) && child !== "📚️examples") findings.push(`"🗿️artifacts/${artifact}/${child}" is not a recognized artifact component dir`);
  }
}

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

const componentFiles: string[] = [];
const walk = (dir: string): void => {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || name === "target" || name === "node_modules" || name === "📦️packages") continue;
    const path = join(dir, name);
    if (statSync(path).isDirectory()) {
      walk(path);
      continue;
    }
    if (!name.endsWith(".rs")) continue;
    if (name === LEAF) componentFiles.push(path);
    else if (name === "📦️lib.rs" && path.includes("/📦️packages/🦀️rust/")) continue;
    else if (LEAF_PARENT_DIRS.includes(dir.split("/").pop() ?? "")) findings.push(`taxonomy leaf file must be named ${LEAF}, found ${relative(PLUGIN_ROOT, path)}`);
  }
};
walk(PLUGIN_ROOT);

const libRs = join(PLUGIN_ROOT, "📦️packages/🦀️rust/📦️lib.rs");
const libRsDir = dirname(libRs);
if (!existsSync(libRs)) findings.push("missing 📦️lib.rs at 📦️packages/🦀️rust/");
else if (existsSync(join(PLUGIN_ROOT, "📦️lib.rs"))) findings.push("owner-root 📦️lib.rs still exists (Shape V2 violation)");

if (existsSync(libRs)) {
  const libText = readFileSync(libRs, "utf8");
  if (libText.includes('#[path = "../../."]')) findings.push('grouping #[path = "."] must not carry ../../ prefix');
  const declared = [...libText.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)].map((m) => m[1]!).filter((p) => p !== ".");
  const declaredAbs = new Set(declared.map((p) => join(libRsDir, p)));
  for (const file of componentFiles) {
    const rel = relative(PLUGIN_ROOT, file);
    if (rel.includes("/📚️examples/")) continue;
    if (!declaredAbs.has(file)) findings.push(`${rel} is not declared by any #[path] in 📦️lib.rs`);
  }
  for (const p of declared) if (p.endsWith(".rs") && !existsSync(join(libRsDir, p))) findings.push(`📦️lib.rs declares #[path = "${p}"] but the file does not exist on disk`);
}

const manifestRoot = join(PLUGIN_ROOT, "🛂️manifest.json");
if (!existsSync(manifestRoot)) findings.push("missing owner-root 🛂️manifest.json");

const libBody = existsSync(libRs) ? readFileSync(libRs, "utf8") : "";
const suspicious = libBody.split(/\r?\n/).filter((l) => /^\s*(pub\s+)?(struct|enum|trait|impl)\s/.test(l));
if (suspicious.length > 0) findings.push(`📦️lib.rs contains ${suspicious.length} type/impl declaration(s) — wiring-only expected`);

console.log(`📦️ component files on disk: ${componentFiles.length}`);
console.log(`🗿️ artifacts: ${dirs(artifactsDir).length}   🎛️ apps: ${dirs(appsDir).length}`);
if (findings.length === 0) console.log("✅️ taxonomy tree clean: ✒️writer");
else {
  console.log(`❌️ ${findings.length} finding(s):`);
  for (const f of findings) console.log(`   - ${f}`);
  process.exit(1);
}
