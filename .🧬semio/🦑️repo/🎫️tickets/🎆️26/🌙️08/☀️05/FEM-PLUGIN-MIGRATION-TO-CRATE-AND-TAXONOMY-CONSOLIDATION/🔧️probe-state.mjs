import { readdirSync, existsSync, readFileSync } from "fs";
import { join } from "path";
const ui = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust";
const targets = join(ui, "🎯️targets");
for (const n of readdirSync(targets)) {
  const cargo = join(targets, n, "Cargo.toml");
  console.log("target", JSON.stringify(n), "cargo", existsSync(cargo));
}
const uiCargo = join(ui, "Cargo.toml");
console.log("ui package", existsSync(uiCargo), "name=", existsSync(uiCargo) ? readFileSync(uiCargo,"utf8").match(/name = "(.*)"/)?.[1] : null);
// root members for ui
const root = readFileSync("/Users/ueli/Documents/semio/Cargo.toml","utf8");
for (const line of root.split("\n")) {
  if (line.includes("🖱️ui") || line.includes("ui-wgpu") || line.includes("framework-ui")) console.log("ROOT:", line.trim());
}
