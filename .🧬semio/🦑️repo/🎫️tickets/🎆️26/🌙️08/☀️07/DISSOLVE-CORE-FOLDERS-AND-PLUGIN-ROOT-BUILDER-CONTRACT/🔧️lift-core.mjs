import { mkdirSync, renameSync, existsSync, readdirSync, readFileSync, writeFileSync, copyFileSync, rmSync, statSync } from "fs";
import { join } from "path";

const MODULES = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const CORE = join(MODULES, "🧩core");

function assert(cond, msg) { if (!cond) throw new Error(msg); }

assert(existsSync(CORE), "core missing");
const coreKids = readdirSync(CORE);
console.log("core kids:", coreKids);

// Lift action-bus, platform, mesh
for (const name of ["🎯️action-bus", "🖥️platform", "🔺️mesh"]) {
  const src = join(CORE, name);
  const dst = join(MODULES, name);
  assert(existsSync(src), `missing ${name}`);
  assert(!existsSync(dst), `dst exists ${name}`);
  renameSync(src, dst);
  console.log("lifted", name);
}

// Create manifest from ui/component.rs
const UI = join(CORE, "🎩️ui");
// find exact ui dir
const uiName = readdirSync(CORE).find(n => n.endsWith("ui"));
const uiPath = join(CORE, uiName);
console.log("ui path", uiPath, "hex", Buffer.from(uiName).toString("hex"));

const MANIFEST = join(MODULES, "🛂️manifest");
mkdirSync(MANIFEST, { recursive: true });
renameSync(join(uiPath, "🦀️component.rs"), join(MANIFEST, "🦀️component.rs"));
console.log("moved ui component.rs -> manifest");

// Lift kernel
const kernelName = readdirSync(uiPath).find(n => n.includes("kernel"));
const kernelSrc = join(uiPath, kernelName);
const KERNEL = join(MODULES, "🎠️kernel");
assert(!existsSync(KERNEL), "kernel dst exists");
renameSync(kernelSrc, KERNEL);
console.log("lifted kernel");

// Move generated under manifest
const genName = readdirSync(CORE).find(n => n.includes("generated"));
const genSrc = join(CORE, genName);
const genDst = join(MANIFEST, "🤖️generated");
renameSync(genSrc, genDst);
console.log("moved generated -> manifest");

// Update #[path] inside manifest component.rs for kernel
const manifestRs = join(MANIFEST, "🦀️component.rs");
let rs = readFileSync(manifestRs, "utf8");
const oldPath = '#[path = "🎠️kernel/🦀️component.rs"]';
const newPath = '#[path = "../🎠️kernel/🦀️component.rs"]';
if (rs.includes(oldPath)) {
  rs = rs.replace(oldPath, newPath);
  console.log("updated kernel path in manifest");
} else if (rs.includes('#[path = "')) {
  // find kernel path line
  const m = rs.match(/#\[path = "[^"]*kernel[^"]*"\]/);
  console.log("kernel path match:", m && m[0]);
  if (m) {
    rs = rs.replace(m[0], newPath);
    console.log("updated kernel path via regex");
  }
}
// Also update region name from ui to manifest if present
rs = rs.replace("// #region ui\n", "// #region 🛂️Manifest\n");
rs = rs.replace("// #endregion ui\n", "// #endregion 🛂️Manifest\n");
rs = rs.replace("// #endregion ui", "// #endregion 🛂️Manifest");
writeFileSync(manifestRs, rs);

console.log("modules now:", readdirSync(MODULES));
console.log("core remaining:", readdirSync(CORE));
console.log("ui remaining:", existsSync(uiPath) ? readdirSync(uiPath) : "gone");
