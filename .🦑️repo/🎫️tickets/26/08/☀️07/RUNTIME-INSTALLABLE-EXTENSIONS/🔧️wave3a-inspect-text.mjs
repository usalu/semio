import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
for (const id of ["text", "math", "core"]) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  const t = fs.readFileSync(path.join(pluginExt, dir, "🦀️component.rs"), "utf8");
  console.log("\n====", id, "tail ====");
  console.log(t.slice(-2500));
  console.log("\n====", id, "manifest region ====");
  const m = t.match(/\/\/ #region 🔖️Manifest[\s\S]*?\/\/ #endregion 🔖️Manifest/);
  console.log(m?.[0]);
}
// testkit
const app = path.join(root, sDir, "🔌️plugins", "🌊️flow", "🎛️apps", "🌊️flow", "🦀️component.rs");
const at = fs.readFileSync(app, "utf8");
const tk = at.match(/\/\/#region 🧪️Testkit[\s\S]*?\/\/#endregion 🧪️Testkit/);
console.log("\n==== testkit ====\n", tk?.[0]?.slice(0, 1500));
