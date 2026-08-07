import fs from "fs";
import path from "path";

const ticket = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS";
const cargo = fs.readFileSync("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml", "utf8");
fs.writeFileSync(path.join(ticket, "cargo-procedural.toml"), cargo);
cargo.split("\n").forEach((l, i) => {
  if (/flow|brep|3d|draw|os-flow|extension/.test(l)) console.log(i + 1, l);
});

// Check if flow core has a place for geometry kernel region already or draw side APIs
const flowRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const coreName = fs.readdirSync(flowRoot).find((n) => /core/.test(n));
const coreFile = path.join(flowRoot, coreName, fs.readdirSync(path.join(flowRoot, coreName)).find((n) => n.includes("component")));
const core = fs.readFileSync(coreFile, "utf8");
console.log("core lines", core.split("\n").length);
// last regions
const regions = [...core.matchAll(/\/\/ #region (.+)/g)].map((m) => m[1]);
console.log("last 20 regions", regions.slice(-20));

// How register works in brep - extract register fn and schemas
const brepPath = fs.readFileSync(path.join(ticket, "brep-component.path"), "utf8").trim();
const brepFile = path.join(brepPath, fs.readdirSync(brepPath).find((n) => n.includes("component")));
const brep = fs.readFileSync(brepFile, "utf8").split("\n");
// find register and module_registry
brep.forEach((l, i) => {
  if (/^pub fn register|^fn module_registry|^pub fn evaluate_json|^fn build_manifest|^pub fn schemas/.test(l.trim()) || l.includes("fn module_registry") || l.includes("pub fn register")) {
    console.log(i + 1, l);
  }
});
fs.writeFileSync(path.join(ticket, "brep-register.rs"), brep.slice(1000, 1100).join("\n"));

// Extract midfile around evaluate_json / module_registry near end before tests
const evalLine = brep.findIndex((l) => l.includes("fn module_registry") || l.includes("pub fn module_registry"));
console.log("module_registry line", evalLine + 1);
fs.writeFileSync(path.join(ticket, "brep-module-registry.rs"), brep.slice(Math.max(0, evalLine - 5), evalLine + 40).join("\n"));

// Check procedural3d examples with brep kinds
function walk(dir, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory() && !["node_modules", "target"].includes(e.name)) walk(p, out);
    else if (e.name.endsWith(".semio") || e.name.endsWith(".flow") || e.name.includes("semio")) out.push(p);
  }
  return out;
}
const examples = walk("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural").filter((p) => p.endsWith(".semio") || p.includes("example"));
console.log("example-ish files", examples.length);
const brepExamples = [];
for (const f of examples) {
  let t;
  try {
    t = fs.readFileSync(f, "utf8");
  } catch {
    continue;
  }
  if (/brep\./.test(t)) brepExamples.push(f);
}
console.log("brep examples", brepExamples.length);
fs.writeFileSync(path.join(ticket, "brep-examples.txt"), brepExamples.join("\n"));

// Also search broader for brep. kinds in semio files under procedural
const allSemio = [];
function walkSemio(dir) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory() && !["node_modules", "target", ".git"].includes(e.name)) walkSemio(p);
    else if (/\.semio$/.test(e.name)) allSemio.push(p);
  }
}
walkSemio("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural");
const withBrep = [];
for (const f of allSemio) {
  const t = fs.readFileSync(f, "utf8");
  if (t.includes("brep.")) withBrep.push(f);
}
console.log("procedural .semio with brep.", withBrep.length);
for (const f of withBrep) console.log(f);

// Check package.json workspace entry exact line context
const pkg = fs.readFileSync("/Users/ueli/Documents/semio/package.json", "utf8").split("\n");
pkg.forEach((l, i) => {
  if (l.includes("brep") || l.includes("flow-module")) console.log("pkg", i + 1, l);
});
