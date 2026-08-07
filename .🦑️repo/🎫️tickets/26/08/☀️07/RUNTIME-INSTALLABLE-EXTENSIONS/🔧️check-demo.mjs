import fs from "fs";
const t = fs.readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs", "utf8");
const i = t.indexOf("fn demo_user");
console.log(JSON.stringify(t.slice(i, i + 180)));
const j = t.indexOf("InstallProgram { plugin_id: \"cad\"");
console.log("install at", j);
console.log(t.slice(j - 80, j + 250));
