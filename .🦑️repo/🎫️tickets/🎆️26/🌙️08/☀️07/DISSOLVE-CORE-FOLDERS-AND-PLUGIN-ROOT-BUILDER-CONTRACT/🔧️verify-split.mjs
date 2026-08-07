import { readFileSync, readdirSync, existsSync, statSync } from "fs";
import { join } from "path";

const MODULES = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const dirs = readdirSync(MODULES);
console.log("modules:", dirs.filter((n) => /manifest|kernel|action|platform|mesh|core/.test(n)));
console.log("has core?", dirs.some((n) => n === "🧩core" || (n.endsWith("core") && !n.includes("manifest") && Buffer.from(n).toString("hex").includes("a7a9"))));

for (const needle of ["manifest", "kernel", "action-bus", "platform", "mesh"]) {
  const d = dirs.find((n) => n.includes(needle));
  const files = readdirSync(join(MODULES, d));
  console.log(d, files);
  const ts = files.find((f) => f.endsWith(".ts"));
  if (ts) {
    const t = readFileSync(join(MODULES, d, ts), "utf8");
    console.log(" ", ts, "lines", t.split("\n").length, "badPath", t.includes("\uFFFD") || t.includes("�️️"));
  }
}

// platform Store
const platform = dirs.find((n) => n.includes("platform"));
const pt = readFileSync(join(MODULES, platform, "🟦️component.ts"), "utf8");
console.log("platform has class Store?", /class Store\b/.test(pt), "extends Store?", /extends Store</.test(pt));
const storeIdx = pt.split("\n").findIndex((l) => /class Store\b|type Store\b|interface Store\b|function createStore/.test(l));
console.log("Store def line", storeIdx >= 0 ? storeIdx + 1 : "MISSING", storeIdx >= 0 ? pt.split("\n")[storeIdx].slice(0, 120) : "");

// Check original for Store
const ticketFiles = readdirSync(process.env.TICKET);
const orig = ticketFiles.find((n) => n.includes("original-component"));
if (orig) {
  const ot = readFileSync(join(process.env.TICKET, orig), "utf8");
  for (let i = 0; i < ot.split("\n").length; i++) {
    const l = ot.split("\n")[i];
    if (/export (abstract )?class Store\b|class Store</.test(l)) console.log("orig Store at", i + 1, l.slice(0, 120));
  }
}

// glue.rs sanity
const glue = readFileSync("/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust/📦️glue.rs", "utf8");
for (const l of glue.split("\n")) if (l.includes("path") || l.includes("pub mod") || l.includes("pub use manifest")) console.log("glue:", l);

// package.json
const pkg = readFileSync("/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/package.json", "utf8");
console.log("pkg name", JSON.parse(pkg).name);

const cargo = readFileSync("/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust/Cargo.toml", "utf8");
console.log("cargo name", cargo.match(/name = "([^"]+)"/)[1], "id", cargo.match(/id = "([^"]+)"/)?.[1]);

// kernel path in manifest rs
const manifest = dirs.find((n) => n.includes("manifest"));
const rs = readFileSync(join(MODULES, manifest, "🦀️component.rs"), "utf8");
const pathLine = rs.split("\n").find((l) => l.includes("#[path") && l.includes("kernel"));
console.log("kernel path", pathLine);
console.log("kernel path exists?", existsSync(join(MODULES, manifest, pathLine.match(/"([^"]+)"/)[1])) || existsSync(join(MODULES, "🎠️kernel", "🦀️component.rs")));
