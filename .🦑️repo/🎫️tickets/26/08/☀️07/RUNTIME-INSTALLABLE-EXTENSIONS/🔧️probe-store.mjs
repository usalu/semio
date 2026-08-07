import fs from "fs";

function walk(dir, pred, acc = []) {
  let entries;
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true });
  } catch {
    return acc;
  }
  for (const e of entries) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = `${dir}/${e.name}`;
    if (e.isDirectory()) walk(p, pred, acc);
    else if (pred(p, e.name)) acc.push(p);
  }
  return acc;
}

const hits = walk(".", (p, n) => {
  if (!/\.(ts|tsx|mjs)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/") || p.includes("/target/")) return false;
  try {
    const t = fs.readFileSync(p, "utf8");
    return /installFromUrl|ExtensionStore|createExtensionStore|extensions\/install/.test(t);
  } catch {
    return false;
  }
});
console.log("STORE", hits.join("\n") || "(none)");

const targets = walk(".", (p, n) => {
  if (!/\.(ts|tsx)$/.test(n)) return false;
  if (p.includes("/.🦑️repo/") || p.includes("/node_modules/")) return false;
  try {
    return fs.readFileSync(p, "utf8").includes("EXTENSION_TARGETS");
  } catch {
    return false;
  }
});
console.log("TARGETS", targets.join("\n"));
for (const p of targets.slice(0, 5)) {
  const t = fs.readFileSync(p, "utf8");
  const i = t.indexOf("EXTENSION_TARGETS");
  console.log("\n====", p, "====");
  console.log(t.slice(i, i + 600));
}
