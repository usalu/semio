import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(ticketDir, "../../../../../../..");
const logPath = path.join(ticketDir, "🧪️repoint-log.json");

const mathPkgRelFromRoot = (() => {
  const modulesDir = path.join(repoRoot, "🧰️framework/🔨️modules");
  const mathDir = fs.readdirSync(modulesDir).find((e) => e.includes("math"));
  if (!mathDir) throw new Error("math module dir missing");
  const rel = path.join("🧰️framework/🔨️modules", mathDir, "📦️packages/🦀️rust");
  if (!fs.existsSync(path.join(repoRoot, rel, "Cargo.toml"))) throw new Error(`math crate missing: ${rel}`);
  return rel;
})();

const mathPkgAbs = path.join(repoRoot, mathPkgRelFromRoot);

const ALIAS_REWRITES = [
  ["mathematical_graph_port_directed_normal", "math::graph::ports::directed::normal"],
  ["mathematical_graph_normal_undirected", "math::graph::normal::undirected"],
  ["mathematical_graph_normal_directed", "math::graph::normal::directed"],
  ["mathematical_graph_port_undirected", "math::graph::ports::undirected"],
  ["mathematical_graph_manifest", "math::graph::manifest"],
  ["mathematical_graph_drawing", "math::graph::drawing"],
  ["mathematical_graph_operators", "math::graph::operators"],
  ["mathematical_graph_traversal", "math::graph::traversal"],
  ["mathematical_graph_dsl", "math::graph::dsl"],
  ["mathematical_polynomial", "math::polynomial"],
  ["mathematical_probability", "math::probability"],
  ["mathematical_statistics", "math::statistics"],
  ["mathematical_sampling", "math::sampling"],
  ["mathematical_tabular", "math::tabular"],
  ["mathematical_algebra", "math::algebra"],
  ["mathematical_geometry", "math::geometry"],
  ["mathematical_optimize", "math::optimize"],
  ["mathematical_random", "math::random"],
  ["mathematical_signal", "math::signal"],
  ["mathematical_spatial", "math::spatial"],
  ["mathematical_number", "math::number"],
  ["mathematical_lie", "math::lie"],
  ["mathematical_cas", "math::cas"],
  ["mathematical_causal", "math::causal"],
  ["mathematical_entropy", "math::entropy"],
  ["mathematical_fuzzy", "math::fuzzy"],
  ["mathematical_wfc", "math::wfc"],
  ["mathematical_graph", "math::graph"],
];

function relMathPath(cargoTomlPath) {
  const dir = path.dirname(cargoTomlPath);
  let rel = path.relative(dir, mathPkgAbs);
  if (!rel.startsWith(".")) rel = `./${rel}`;
  return rel.split(path.sep).join("/");
}

function collapseCargoToml(cargoPath) {
  const raw = fs.readFileSync(cargoPath, "utf8");
  const lines = raw.split("\n");
  let removed = 0;
  let hasMath = false;
  const out = [];
  for (const line of lines) {
    if (/^\s*math\s*=\s*\{/.test(line)) {
      hasMath = true;
      out.push(line);
      continue;
    }
    if (/^\s*mathematical_[a-z0-9_]+\s*=\s*\{/.test(line)) {
      removed++;
      continue;
    }
    out.push(line);
  }
  if (removed === 0) return null;
  if (!hasMath) {
    const depIdx = out.findIndex((l) => l.trim() === "[dependencies]");
    const mathLine = `math = { path = "${relMathPath(cargoPath)}", package = "semio-framework-math" }`;
    if (depIdx >= 0) out.splice(depIdx + 1, 0, mathLine);
    else out.push("", "[dependencies]", mathLine);
  }
  const next = out.join("\n");
  if (next !== raw) fs.writeFileSync(cargoPath, next);
  return { removed, addedMath: !hasMath };
}

function rewriteRust(source) {
  let s = source;
  for (const [from, to] of ALIAS_REWRITES) {
    s = s.split(from).join(to);
  }
  return s;
}

function walkRs(root, skipDir) {
  const files = [];
  if (!fs.existsSync(root)) return files;
  const stack = [root];
  while (stack.length) {
    const dir = stack.pop();
    for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
      if (ent.name === "target" || ent.name === "node_modules" || ent.name === ".git") continue;
      const full = path.join(dir, ent.name);
      if (skipDir?.(full)) continue;
      if (ent.isDirectory()) stack.push(full);
      else if (ent.name.endsWith(".rs")) files.push(full);
    }
  }
  return files;
}

const cargoRoots = [
  "✏️s/🔌️plugins",
  "✏️s/🔨️modules",
  "🧰️framework",
  "compose/client/lib/query/rs",
];

const cargoChanged = [];
for (const rootRel of cargoRoots) {
  const root = path.join(repoRoot, rootRel);
  if (!fs.existsSync(root)) continue;
  for (const cargo of walkRs(root, (p) => p.includes(".🦑️repo"))) {
    if (!cargo.endsWith("Cargo.toml")) continue;
  }
}

function allCargoTomls(dir, acc = []) {
  if (!fs.existsSync(dir)) return acc;
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    if (ent.name === "target" || ent.name === "node_modules" || ent.name === ".🦑️repo") continue;
    const full = path.join(dir, ent.name);
    if (ent.isDirectory()) allCargoTomls(full, acc);
    else if (ent.name === "Cargo.toml") acc.push(full);
  }
  return acc;
}

for (const cargo of allCargoTomls(repoRoot)) {
  if (cargo.includes(`${path.sep}.🦑️repo${path.sep}`)) continue;
  const r = collapseCargoToml(cargo);
  if (r) cargoChanged.push({ cargo: path.relative(repoRoot, cargo), ...r });
}

const rsRoots = [
  path.join(repoRoot, "✏️s/🔌️plugins"),
  path.join(repoRoot, "✏️s/🔨️modules"),
  path.join(repoRoot, "🧰️framework/🛍️products/💻️os"),
  path.join(repoRoot, "🧰️framework/🔨️modules/🗺️surface"),
  path.join(repoRoot, "compose/client/lib/query/rs"),
];

const rsChanged = [];
for (const root of rsRoots) {
  for (const file of walkRs(root, (p) => p.includes(".🦑️repo"))) {
    const raw = fs.readFileSync(file, "utf8");
    if (!ALIAS_REWRITES.some(([from]) => raw.includes(from))) continue;
    const next = rewriteRust(raw);
    if (next !== raw) {
      fs.writeFileSync(file, next);
      rsChanged.push(path.relative(repoRoot, file));
    }
  }
}

const report = {
  mathPkgRelFromRoot,
  cargoChangedCount: cargoChanged.length,
  cargoChanged,
  rsChangedCount: rsChanged.length,
  rsChanged,
};
fs.writeFileSync(logPath, JSON.stringify(report, null, 2));
console.log(JSON.stringify({ cargoChangedCount: report.cargoChangedCount, rsChangedCount: report.rsChangedCount }, null, 2));
