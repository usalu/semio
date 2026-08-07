import fs from "fs";
import path from "path";

const ticket = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS";

function findFile(dir, pred) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) {
      if (e.name === "node_modules" || e.name === "target") continue;
      const hit = findFile(p, pred);
      if (hit) return hit;
    } else if (pred(e.name, p)) return p;
  }
  return null;
}

const flowRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const coreDir = fs.readdirSync(flowRoot).find((n) => n.includes("core") && !n.includes("extensions"));
const corePath = path.join(flowRoot, coreDir, fs.readdirSync(path.join(flowRoot, coreDir)).find((n) => n.includes("component")));
console.log("corePath", corePath);
const core = fs.readFileSync(corePath, "utf8");
const lines = core.split("\n");

const keys = [
  "install_builtin_flow_extensions",
  "tessellate_geometry",
  "export_solid_json",
  "import_solid_json",
  "retain_geometry_handles",
  "dispose_geometry",
  "render_scene_json",
  "flow_extension_brep",
  "extensions::brep",
  "brep::",
  "register_brep",
  "crate::brep",
  "pub use",
];

for (const k of keys) {
  const hits = [];
  lines.forEach((l, i) => {
    if (l.includes(k)) hits.push(i + 1);
  });
  if (hits.length) console.log(k, hits.slice(0, 30).join(","), "total", hits.length);
}

function dumpAround(label, needle, radius = 25) {
  const idx = lines.findIndex((l) => l.includes(needle));
  if (idx < 0) {
    console.log("NOT FOUND", label);
    return;
  }
  const chunk = lines.slice(Math.max(0, idx - radius), idx + radius).join("\n");
  fs.writeFileSync(path.join(ticket, label), chunk);
  console.log("wrote", label, "around line", idx + 1);
}

dumpAround("core-install-builtin.txt", "install_builtin_flow_extensions", 40);
dumpAround("core-tessellate.txt", "tessellate_geometry", 20);
dumpAround("core-render-scene.txt", "render_scene_json", 20);

// Call site snippets from procedural3d / playbook / cad
const callSites = [
  "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/⚙️engine/🦀️component.rs",
  "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/️️extensions/🌀️procedural/🦀️component.rs",
  "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/📥️geometry-import/🦀️component.rs",
];

// resolve playbook path via walk
const playbookRoot = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook";
const playbookComp = findFile(playbookRoot, (n, p) => n.includes("component") && p.includes("procedural") && p.includes("extensions") && p.endsWith(".rs") && !p.includes("packages"));
console.log("playbookComp", playbookComp);

for (const [name, p] of [
  ["caller-procedural3d.txt", callSites[0]],
  ["caller-playbook.txt", playbookComp],
  ["caller-cad-import.txt", callSites[2]],
]) {
  if (!p || !fs.existsSync(p)) {
    console.log("missing", name, p);
    continue;
  }
  const text = fs.readFileSync(p, "utf8");
  const ls = text.split("\n");
  const apis = ["tessellate_geometry", "export_solid_json", "import_solid_json", "retain_geometry_handles", "dispose_geometry", "flow_extension_brep", "semio_framework_os_flow"];
  let out = `FILE ${p}\n`;
  for (const a of apis) {
    ls.forEach((l, i) => {
      if (l.includes(a)) out += `${i + 1}: ${l}\n`;
    });
  }
  fs.writeFileSync(path.join(ticket, name), out);
  console.log("wrote", name);
}

// BIM extension_guest rest
const bim = fs.readFileSync(path.join(ticket, "bim-component.rs"), "utf8").split("\n");
const start = bim.findIndex((l) => l.includes("// #region") && l.includes("ExtensionGuest"));
fs.writeFileSync(path.join(ticket, "bim-extension-guest.rs"), bim.slice(start, start + 120).join("\n"));
console.log("wrote bim-extension-guest.rs from", start + 1);

// Check root Cargo.toml for members pattern and package.json workspaces for brep
const rootCargo = fs.readFileSync("/Users/ueli/Documents/semio/Cargo.toml", "utf8");
const rootPkg = fs.readFileSync("/Users/ueli/Documents/semio/package.json", "utf8");
console.log("\n--- Cargo members with flow-extension or brep ---");
rootCargo.split("\n").forEach((l, i) => {
  if (/flow-extension|brep|bim/.test(l) && /extension|brep|bim/.test(l)) console.log(i + 1, l);
});
console.log("\n--- package.json workspaces with brep/flow-module ---");
rootPkg.split("\n").forEach((l, i) => {
  if (/brep|flow-module|flow-extension/.test(l)) console.log(i + 1, l);
});

// flow Cargo.toml deps
const flowCargo = path.join(flowRoot, "📦️packages", "🦀️rust", "Cargo.toml");
console.log("\nflow cargo exists", fs.existsSync(flowCargo));
const fc = fs.readFileSync(flowCargo, "utf8");
fs.writeFileSync(path.join(ticket, "flow-Cargo.toml"), fc);
fc.split("\n").forEach((l, i) => {
  if (/3d|brep|2d|draw|s-3d|s-2d/.test(l)) console.log("flowCargo", i + 1, l);
});

// Check if wave3a already moved any light extensions
const extDir = path.join(flowRoot, fs.readdirSync(flowRoot).find((n) => n.includes("extensions")));
console.log("\nextensions still in framework:", fs.readdirSync(extDir));
const pluginExt = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow";
const pext = fs.readdirSync(pluginExt).find((n) => n.includes("extensions"));
console.log("plugin flow extensions:", fs.readdirSync(path.join(pluginExt, pext)));
