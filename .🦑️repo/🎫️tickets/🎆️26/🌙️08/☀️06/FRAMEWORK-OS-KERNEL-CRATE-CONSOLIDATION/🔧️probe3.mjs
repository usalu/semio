import fs from "fs";
import path from "path";
const OS = fs.readFileSync("/tmp/os-path.txt", "utf8").trim();
const modules = path.join(OS, fs.readdirSync(OS).find((x) => x.includes("modules")));

function walk(dir, acc = [], depth = 0) {
  if (depth > 6) return acc;
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isFile() && e.name.includes("component") && !p.includes("implementations")) acc.push(p);
    if (e.isDirectory() && !e.name.includes("implementations") && e.name !== "target") walk(p, acc, depth + 1);
  }
  return acc;
}
for (const f of walk(modules)) {
  const t = fs.readFileSync(f, "utf8");
  if (t.includes("struct OperationMeta") || t.includes("struct HistoryOpMeta")) {
    console.log(f.replace(OS, ""), "has meta");
    const i = Math.max(t.indexOf("struct OperationMeta"), t.indexOf("struct HistoryOpMeta"));
    console.log(t.slice(i, i + 500));
  }
}
// old store_sync cargo
const store = path.join(modules, fs.readdirSync(modules).find((x) => x.includes("store")));
function findCargo(dir, acc = [], depth = 0) {
  if (depth > 5) return acc;
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isFile() && e.name === "Cargo.toml") acc.push(p);
    if (e.isDirectory()) findCargo(p, acc, depth + 1);
  }
  return acc;
}
for (const c of findCargo(store)) {
  console.log("\n===", c.replace(OS, ""), "===");
  console.log(fs.readFileSync(c, "utf8").slice(0, 1500));
}
