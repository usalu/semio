import fs from "fs";
import path from "path";
const p = path.join(process.env.RUSTPKG, "Cargo.toml");
let t = fs.readFileSync(p, "utf8");
t = t.replace(/\n    "dep:semio-s-3d",/g, "");
t = t.replace(/^semio-s-3d = \{ path = "[^"]+", optional = true \}\n/m, "");
t = t.replace(
  'members = [".", "../../../../../✏️s/🔨️modules/📊️3d/📦️packages/🦀️rust"]',
  'members = ["."]',
);
// fix emoji in replace - use regex
t = t.replace(/members = \["\.", "[^"]+3d[^"]+"\]/, 'members = ["."]');
fs.writeFileSync(p, t);
console.log("reverted");
console.log("still has semio-s-3d?", t.includes("semio-s-3d"));
console.log("members line:", t.match(/members = \[[^\]]+\]/)?.[0]);
