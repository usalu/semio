import { readFileSync } from "fs";
const lines = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧩core/🟦️component.ts","utf8").split("\n");
for (let i=1700; i<=2300; i++) {
  const line = lines[i-1];
  if (/region|endregion|^export |^import |^\/\/#|^\/\/ #/.test(line) || i%50===0) {
    console.log(`${i}: ${line.slice(0,140)}`);
  }
}
console.log("\n--- 2400-2700 ---");
for (let i=2400; i<=2700; i++) {
  const line = lines[i-1];
  if (/region|endregion|^export /.test(line)) console.log(`${i}: ${line.slice(0,140)}`);
}
console.log("\n--- check pendingWindowUiNode deps ---");
for (let i=187; i<=200; i++) console.log(`${i}: ${lines[i-1].slice(0,140)}`);
