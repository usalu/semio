import fs from "fs";
import path from "path";

const rustpkg = process.env.RUSTPKG;
const rel = process.env.REL;
const p = path.join(rustpkg, "Cargo.toml");
let t = fs.readFileSync(p, "utf8");
if (t.includes("semio-s-3d")) {
  console.log("already has semio-s-3d");
  process.exit(0);
}
if (!t.includes('"dep:arboard"')) {
  console.error("unexpected Cargo.toml shape");
  process.exit(1);
}
t = t.replace('"dep:arboard",\n]', '"dep:arboard",\n    "dep:semio-s-3d",\n]');
const taffyLine = t.match(/^taffy = \{[^\n]+\n/m);
if (!taffyLine) {
  console.error("no taffy line");
  process.exit(1);
}
t = t.replace(taffyLine[0], taffyLine[0] + `semio-s-3d = { path = "${rel}", optional = true }\n`);
// Expand temporary workspace members so workspace=true fields on semio-s-3d resolve.
// Also need transitive workspace deps — may still fail; try with member first.
if (t.includes('members = ["."]')) {
  t = t.replace('members = ["."]', `members = [".", "${rel}"]`);
}
fs.writeFileSync(p, t);
console.log("patched");
