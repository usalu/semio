import fs from "fs";
import path from "path";

const root = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite";

function walk(d, files = []) {
  let ents;
  try {
    ents = fs.readdirSync(d, { withFileTypes: true });
  } catch {
    return files;
  }
  for (const e of ents) {
    const p = path.join(d, e.name);
    if (e.isDirectory() && e.name !== "target") walk(p, files);
    else if (e.isFile() && e.name.endsWith(".rs")) files.push(p);
  }
  return files;
}

const files = walk(root);
console.log("files", files.length);

// Sample crate::infinite usages
let n = 0;
for (const f of files) {
  const lines = fs.readFileSync(f, "utf8").split("\n");
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes("crate::infinite") && n < 30) {
      console.log(`${f}:${i + 1}: ${lines[i].trim()}`);
      n++;
    }
  }
}

console.log("\n--- crate::os_* samples ---");
n = 0;
for (const f of files) {
  const lines = fs.readFileSync(f, "utf8").split("\n");
  for (let i = 0; i < lines.length; i++) {
    if (/crate::os_(store|dsl|spr)/.test(lines[i]) && n < 25) {
      console.log(`${f}:${i + 1}: ${lines[i].trim()}`);
      n++;
    }
  }
}

// board structure - self:: unresolved
console.log("\n--- board ports directed_normal first 80 lines ---");
const dn = path.join(root, "🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs");
console.log(fs.readFileSync(dn, "utf8").split("\n").slice(0, 80).join("\n"));

console.log("\n--- board ports component first 60 ---");
const ports = path.join(root, "🎲️board/🔌️ports/🦀️component.rs");
console.log(fs.readFileSync(ports, "utf8").split("\n").slice(0, 60).join("\n"));
