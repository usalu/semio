import { readFileSync } from "fs";
const ui = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core/🎩️ui/🦀️component.rs";
const text = readFileSync(ui, "utf8");
const lines = text.split("\n");
console.log("UI lines:", lines.length);
console.log("--- regions/mods/key ---");
for (let i = 0; i < lines.length; i++) {
  const line = lines[i];
  if (/region|endregion|pub mod|mod kernel|AppDefinition|PluginManifest|#\[path|#\[cfg/i.test(line) || /^\/\/#/.test(line) || /^\/\//!/.test(line) || /^pub (struct|enum|type|fn|trait|use|const)/.test(line.trim()) && i < 200) {
    console.log(`${i+1}: ${line.slice(0, 180)}`);
  }
}
console.log("\n--- ALL regions ---");
for (let i = 0; i < lines.length; i++) {
  if (/#region|#endregion|pub mod /.test(lines[i])) console.log(`${i+1}: ${lines[i].slice(0,180)}`);
}
console.log("\n--- first 100 ---");
for (let i = 0; i < 100; i++) console.log(`${i+1}: ${lines[i].slice(0,180)}`);
