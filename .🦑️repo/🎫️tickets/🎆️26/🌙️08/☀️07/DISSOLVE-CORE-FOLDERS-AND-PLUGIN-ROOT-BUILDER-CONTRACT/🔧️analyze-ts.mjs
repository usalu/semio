import { readFileSync } from "fs";
import { join } from "path";

const core = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core";
const ts = readFileSync(join(core, "🟦️component.ts"), "utf8");
const lines = ts.split("\n");
console.log("TS lines:", lines.length);
console.log("--- regions / section markers ---");
for (let i = 0; i < lines.length; i++) {
  const line = lines[i];
  if (/region|endregion|#region|#endregion/i.test(line) || /^\/\/ =+/.test(line) || /^\/\/#/.test(line)) {
    console.log(`${i+1}: ${line.slice(0, 140)}`);
  }
}
console.log("--- first 120 lines ---");
for (let i = 0; i < Math.min(120, lines.length); i++) {
  console.log(`${i+1}: ${lines[i].slice(0, 140)}`);
}
