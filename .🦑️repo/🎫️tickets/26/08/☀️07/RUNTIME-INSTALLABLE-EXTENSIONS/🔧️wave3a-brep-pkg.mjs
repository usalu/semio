import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
const brepDir = fs.readdirSync(pluginExt).find((n) => n.includes("brep"));
console.log("brep plugin files", fs.readdirSync(path.join(pluginExt, brepDir)));
const cargo = path.join(pluginExt, brepDir, "📦️packages", "🦀️rust", "Cargo.toml");
console.log("has cargo", fs.existsSync(cargo));
if (fs.existsSync(cargo)) console.log(fs.readFileSync(cargo, "utf8"));
const fwBrep = path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow",
  fs.readdirSync(path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow")).find((n) => n.includes("extensions")),
  fs.readdirSync(path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow",
    fs.readdirSync(path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow")).find((n) => n.includes("extensions")))).find((n) => n.includes("brep")));
console.log("fw brep", fwBrep, fs.existsSync(fwBrep) && fs.readdirSync(fwBrep));
// glue again
console.log("glue\n", fs.readFileSync(path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow", "📦️packages", "🦀️rust", "📦️glue.rs"), "utf8"));
// cargo.toml flow for brep dep
console.log("flow cargo\n", fs.readFileSync(path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow", "📦️packages", "🦀️rust", "Cargo.toml"), "utf8"));
