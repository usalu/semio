import fs from "fs";
import path from "path";

const FLOW = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const ALL = [
  ["📄️document", "document"],
  ["📚️catalogue", "catalogue"],
  ["📔️registry", "registry"],
  ["🌉️bridge", "bridge"],
  ["🖥️host", "host"],
  ["🖍️drawing", "drawing"],
  ["🌉️wasm", "wasm_session"],
  ["🌿️vcs", "vcs"],
];

const defRe = /^pub(?:\([^)]*\))?\s+(?:(?:async\s+)?fn|struct|enum|type|const|static)\s+([A-Za-z0-9_]+)/;
const pubs = new Map(); // name -> [mods]
for (const [dir, rust] of ALL) {
  const text = fs.readFileSync(path.join(FLOW, dir, "🦀️component.rs"), "utf8");
  for (const line of text.split("\n")) {
    const m = line.match(defRe);
    if (!m) continue;
    if (!pubs.has(m[1])) pubs.set(m[1], []);
    pubs.get(m[1]).push(rust);
  }
}
const dups = [...pubs.entries()].filter(([, mods]) => new Set(mods).size > 1);
console.log("Duplicate pub names across modules:");
for (const [n, mods] of dups) console.log(" ", n, mods.join(","));

// Check core gone
console.log("core exists?", fs.readdirSync(FLOW).some((c) => c.includes("core") && !c.includes("extensions")));
console.log("brep exists?", fs.readdirSync(FLOW).some((c) => c.includes("brep-geometry")));
console.log("corrupted glue?", fs.readdirSync(path.join(FLOW, "📦️packages", "🦀️rust")).filter((c) => c.includes("glue")));

// Spot-check bad pub(crate) upgrades (impl lines)
for (const [dir] of ALL) {
  const text = fs.readFileSync(path.join(FLOW, dir, "🦀️component.rs"), "utf8");
  const bad = text.split("\n").filter((l) => /pub\(crate\)\s+impl\b/.test(l));
  if (bad.length) console.log("BAD impl upgrade", dir, bad);
}

// registry static
const reg = fs.readFileSync(path.join(FLOW, "📔️registry", "🦀️component.rs"), "utf8");
const st = reg.split("\n").find((l) => l.includes("FLOW_EXTENSION_STATE"));
console.log("FLOW_EXTENSION_STATE line:", st);

// deferred
const deferred = JSON.parse(fs.readFileSync(path.join(process.argv[1], "deferred-flow-core.json"), "utf8"));
console.log("deferred count", deferred.length);
console.log("sample", deferred[0]);

// Ensure no 🙀️core left under flow
function walk(d, acc=[]) {
  for (const n of fs.readdirSync(d)) {
    if (n === "target" || n === "node_modules") continue;
    const p = path.join(d, n);
    if (n.includes("core") && fs.statSync(p).isDirectory()) acc.push(p);
    if (fs.statSync(p).isDirectory()) walk(p, acc);
  }
  return acc;
}
console.log("dirs with core in name under flow:", walk(FLOW));
