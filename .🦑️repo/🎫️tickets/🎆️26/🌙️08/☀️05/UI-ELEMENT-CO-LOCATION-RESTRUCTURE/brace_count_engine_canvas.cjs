const fs = require('fs');
const path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️lib.rs";
const lines = fs.readFileSync(path, 'utf8').split('\n');
const startLine = 20; // 1-indexed, "pub mod engine_canvas {"
let depth = 0;
let started = false;
for (let i = startLine - 1; i < lines.length; i++) {
  const line = lines[i];
  for (const ch of line) {
    if (ch === '{') { depth++; started = true; }
    else if (ch === '}') { depth--; }
  }
  if (started && depth === 0) {
    console.log("Closing brace at line", i + 1);
    process.exit(0);
  }
}
console.log("Not found");
