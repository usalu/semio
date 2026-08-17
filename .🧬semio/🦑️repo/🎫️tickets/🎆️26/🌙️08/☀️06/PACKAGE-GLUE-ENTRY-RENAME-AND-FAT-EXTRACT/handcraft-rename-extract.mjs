import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";
const ticketDir = path.dirname(process.argv[1]);

const LIB = "📦️lib.rs";
const GLUE = "📦️glue.rs";
const COMPONENT = "🦀️component.rs";
const PACKAGES = "📦️packages";
const REPO_DIR = ".🦑️repo";

function walk(dir, name, underPackages, acc = []) {
  let ents;
  try { ents = fs.readdirSync(dir, { withFileTypes: true }); } catch { return acc; }
  for (const e of ents) {
    const p = path.join(dir, e.name);
    if (e.name === "node_modules" || e.name === ".git" || e.name === "target" || e.name === REPO_DIR) continue;
    if (e.isDirectory()) walk(p, name, underPackages, acc);
    else if (e.name === name && (!underPackages || p.includes(path.sep + PACKAGES + path.sep))) acc.push(p);
  }
  return acc;
}

function rel(p) { return path.relative(root, p); }

function updateCargoToml(dir) {
  const cargo = path.join(dir, "Cargo.toml");
  if (!fs.existsSync(cargo)) return { updated: false, reason: "no-cargo" };
  const before = fs.readFileSync(cargo, "utf8");
  let after = before.replace(/path\s*=\s*["']📦️lib\.rs["']/g, "path = \"" + GLUE + "\"");
  if (after === before && /\[lib\]/.test(after) && !/path\s*=/.test(after)) {
    after = after.replace(/\[lib\]\s*\n/, "[lib]\npath = \"" + GLUE + "\"\n");
  }
  if (after === before && !/\[lib\]/.test(after)) {
    after = after + "\n[lib]\npath = \"" + GLUE + "\"\n";
  }
  if (after !== before) { fs.writeFileSync(cargo, after); return { updated: true }; }
  return { updated: false, reason: "unchanged", snippet: (before.match(/\[lib\][\s\S]{0,120}/) || ["none"])[0] };
}

function renameOne(libPath) {
  const dir = path.dirname(libPath);
  const gluePath = path.join(dir, GLUE);
  if (fs.existsSync(gluePath)) return { status: "skip-glue-exists", libPath, gluePath };
  fs.renameSync(libPath, gluePath);
  const cargo = updateCargoToml(dir);
  return { status: "renamed", libPath, gluePath, cargo };
}

function ownerRootFromPackageFile(filePath) {
  let p = path.dirname(filePath);
  while (p !== root) {
    if (path.basename(p) === PACKAGES) return path.dirname(p);
    p = path.dirname(p);
  }
  throw new Error("owner not found for " + filePath);
}

function pathFromGlueToOwnerComponent(gluePath, ownerComponentPath) {
  return path.relative(path.dirname(gluePath), ownerComponentPath).split(path.sep).join("/");
}

function extractFat(gluePath) {
  const owner = ownerRootFromPackageFile(gluePath);
  const ownerComponent = path.join(owner, COMPONENT);
  if (fs.existsSync(ownerComponent)) {
    return { status: "blocked-owner-component-exists", gluePath: rel(gluePath), ownerComponent: rel(ownerComponent) };
  }
  if (!fs.existsSync(gluePath)) {
    return { status: "missing-glue", gluePath: rel(gluePath) };
  }
  const body = fs.readFileSync(gluePath, "utf8");
  const pathCount = (body.match(/#\[path/g) || []).length;
  if (pathCount > 0) {
    return { status: "skip-has-paths", gluePath: rel(gluePath), pathCount };
  }
  fs.writeFileSync(ownerComponent, body);
  const pathAttr = pathFromGlueToOwnerComponent(gluePath, ownerComponent);
  const thin = [
    "//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️component.rs.",
    "",
    '#[path = "' + pathAttr + '"]',
    "mod component;",
    "pub use component::*;",
    "",
  ].join("\n");
  fs.writeFileSync(gluePath, thin);
  return { status: "extracted", gluePath: rel(gluePath), ownerComponent: rel(ownerComponent), pathAttr, locMoved: body.split("\n").length };
}

const mode = process.argv[2] || "rename";

if (mode === "rename") {
  const libs = walk(root, LIB, true);
  const results = libs.map(renameOne);
  const out = {
    mode, count: libs.length,
    renamed: results.filter(r => r.status === "renamed").length,
    skipped: results.filter(r => r.status !== "renamed").length,
    results: results.map(r => ({ status: r.status, from: rel(r.libPath), to: r.gluePath ? rel(r.gluePath) : null, cargo: r.cargo })),
  };
  fs.writeFileSync(path.join(ticketDir, "🧪rename-results.json"), JSON.stringify(out, null, 2));
  console.log(JSON.stringify({ renamed: out.renamed, skipped: out.skipped, total: out.count }, null, 2));
} else if (mode === "list-fat") {
  const glues = walk(root, GLUE, true);
  const fat = [];
  for (const g of glues) {
    const t = fs.readFileSync(g, "utf8");
    const lines = t.split("\n").length;
    const paths = (t.match(/#\[path/g) || []).length;
    if (paths === 0) fat.push({ lines, paths, file: rel(g) });
  }
  fat.sort((a,b) => b.lines - a.lines);
  fs.writeFileSync(path.join(ticketDir, "🧪fat-glue.json"), JSON.stringify(fat, null, 2));
  console.log(JSON.stringify(fat, null, 2));
} else if (mode === "extract-priority") {
  const glues = walk(root, GLUE, true);
  const selected = [];
  for (const g of glues) {
    const r = rel(g);
    const isDrawFsm = r.includes("draw") && r.includes("fsm") && r.endsWith(GLUE);
    const isPlaybookProc = r.includes("playbook") && r.includes("procedural") && r.includes("extensions");
    const isFlowBim = r.includes("flow") && r.includes("bim") && r.includes("extensions");
    const isImpExt = r.includes("imperative") && r.includes("extensions") && r.endsWith(GLUE);
    const isSrcExt = r.includes("sourcing") && r.includes("extensions") && r.endsWith(GLUE);
    if (isDrawFsm || isPlaybookProc || isFlowBim || isImpExt || isSrcExt) selected.push(g);
  }
  const results = selected.map(extractFat);
  fs.writeFileSync(path.join(ticketDir, "🧪extract-results.json"), JSON.stringify(results, null, 2));
  console.log(JSON.stringify(results, null, 2));
} else {
  console.error("unknown mode", mode);
  process.exit(1);
}
