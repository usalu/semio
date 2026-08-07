import fs from "fs";
import path from "path";

const ticket = path.dirname(new URL(import.meta.url).pathname);
const bimRoot = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow";
const ext = fs.readdirSync(bimRoot).find((n) => n.includes("extensions"));
const bim = fs.readdirSync(path.join(bimRoot, ext)).find((n) => n.includes("bim"));
const bimPath = path.join(bimRoot, ext, bim);
const rust = path.join(bimPath, "📦️packages", "🦀️rust");
const copies = {
  "bim-Cargo.toml": path.join(rust, "Cargo.toml"),
  "bim-glue.rs": path.join(rust, "📦️glue.rs"),
  "bim-script.ts": path.join(rust, "📜️script.ts"),
  "bim-project.json": path.join(rust, "📋️project.json"),
  "bim-component.rs": path.join(bimPath, "🦀️component.rs"),
};
for (const [name, src] of Object.entries(copies)) {
  fs.copyFileSync(src, path.join(ticket, name));
  console.log("copied", name, fs.statSync(src).size);
}
const fw = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const fwExt = fs.readdirSync(fw).find((n) => n.includes("extensions"));
const brep = fs.readdirSync(path.join(fw, fwExt)).find((n) => n.includes("brep"));
const brepPath = path.join(fw, fwExt, brep);
fs.copyFileSync(path.join(brepPath, "package.json"), path.join(ticket, "brep-package.json"));
const brepComp = fs.readFileSync(path.join(brepPath, "🦀️component.rs"), "utf8");
fs.writeFileSync(path.join(ticket, "brep-component.path"), brepPath + "\n");
fs.writeFileSync(path.join(ticket, "brep-lines.txt"), String(brepComp.split("\n").length));
fs.writeFileSync(path.join(ticket, "brep-head.rs"), brepComp.split("\n").slice(0, 250).join("\n"));
fs.writeFileSync(path.join(ticket, "brep-tail.rs"), brepComp.split("\n").slice(-250).join("\n"));
const apis = [
  "tessellate_geometry",
  "export_solid_json",
  "import_solid_json",
  "retain_geometry_handles",
  "dispose_geometry",
  "pub fn register",
  "ExtensionBundle",
  "standalone",
  "evaluate_json",
  "FlowExtension",
];
for (const a of apis) {
  const lines = [];
  brepComp.split("\n").forEach((line, i) => {
    if (line.includes(a)) lines.push(i + 1);
  });
  console.log(a, lines.slice(0, 20).join(","), "total", lines.length);
}
