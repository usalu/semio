import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";

function show(rel, n = 2000) {
  const p = path.join(root, rel);
  console.log("\n========", rel, "exists=" + fs.existsSync(p));
  if (!fs.existsSync(p)) return;
  const t = fs.readFileSync(p, "utf8");
  console.log("lines", t.split("\n").length);
  console.log(t.slice(0, n));
}

function listDir(rel) {
  const p = path.join(root, rel);
  console.log("\n--- DIR", rel, "---");
  if (!fs.existsSync(p)) {
    console.log("MISSING");
    return;
  }
  for (const e of fs.readdirSync(p, { withFileTypes: true })) {
    console.log(e.isDirectory() ? "D" : "F", e.name);
  }
}

show("🌎️hub/📦️packages/🦀️rust/📦️lib.rs");
show("✏️s/🔌️plugins/📜️imperative/🧩️extensions/