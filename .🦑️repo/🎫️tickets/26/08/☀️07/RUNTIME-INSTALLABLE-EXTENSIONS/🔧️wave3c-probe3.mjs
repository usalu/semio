import fs from "fs";
import path from "path";

const ticket = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS";

function readResolved(rootHint, ...partsContaining) {
  // walk from rootHint finding path whose segments include all partsContaining
  function walk(dir, depth = 0) {
    if (depth > 8) return null;
    let entries;
    try {
      entries = fs.readdirSync(dir, { withFileTypes: true });
    } catch {
      return null;
    }
    for (const e of entries) {
      const p = path.join(dir, e.name);
      if (e.isDirectory()) {
        if (["node_modules", "target", ".git"].includes(e.name)) continue;
        const hit = walk(p, depth + 1);
        if (hit) return hit;
      } else if (partsContaining.every((s) => p.includes(s))) {
        return p;
      }
    }
    return null;
  }
  return walk(rootHint);
}

// Dump procedural3d imports and tessellate usage context
const p3d = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/⚙️engine/🦀️component.rs";
const text = fs.readFileSync(p3d, "utf8").split("\n");
fs.writeFileSync(path.join(ticket, "procedural3d-head.rs"), text.slice(0, 80).join("\n"));
fs.writeFileSync(path.join(ticket, "procedural3d-tessellate-ctx.rs"), text.slice(360, 420).join("\n"));
fs.writeFileSync(path.join(ticket, "procedural3d-doc-tess.rs"), text.slice(540, 580).join("\n"));

// flow core retain/dispose call sites
const flowRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const coreDir = fs.readdirSync(flowRoot).find((n) => n.includes("core") && fs.statSync(path.join(flowRoot, n)).isDirectory() && !n.includes("extension"));
// actually core is 🙰core
const coreName = fs.readdirSync(flowRoot).find((n) => /core/.test(n) && !n.includes("extension"));
const coreFile = path.join(flowRoot, coreName, fs.readdirSync(path.join(flowRoot, coreName)).find((n) => n.includes("component")));
const core = fs.readFileSync(coreFile, "utf8").split("\n");

function dumpHits(label, needles, radius = 15) {
  let out = "";
  for (const needle of needles) {
    core.forEach((l, i) => {
      if (l.includes(needle)) {
        out += `\n===== ${needle} @ ${i + 1} =====\n`;
        out += core.slice(Math.max(0, i - radius), i + radius).join("\n") + "\n";
      }
    });
  }
  fs.writeFileSync(path.join(ticket, label), out);
  console.log("wrote", label);
}

dumpHits("core-brep-api-uses.txt", [
  "flow_extension_brep::retain_geometry_handles",
  "flow_extension_brep::dispose_geometry",
  "flow_extension_brep::tessellate_geometry",
  "flow_extension_brep::register",
  "flow_extension_brep::",
]);

// Check procedural3d/playbook Cargo.toml for flow_extension_brep dep
for (const [label, root] of [
  ["procedural-rust", "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust"],
  ["playbook-procedural-ext", "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook"],
]) {
  function findCargo(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, e.name);
      if (e.isDirectory() && !["node_modules", "target"].includes(e.name)) {
        const hit = findCargo(p);
        if (hit) return hit;
      } else if (e.name === "Cargo.toml" && fs.readFileSync(p, "utf8").includes("flow_extension_brep")) {
        return p;
      }
    }
    return null;
  }
  const cargo = findCargo(root);
  console.log(label, cargo);
  if (cargo) {
    const c = fs.readFileSync(cargo, "utf8");
    fs.writeFileSync(path.join(ticket, `cargo-${label}.toml`), c);
    c.split("\n").forEach((l, i) => {
      if (/flow|brep|3d|os-flow/.test(l)) console.log(" ", i + 1, l);
    });
  }
}

// Check glue of procedural for extern crate
const procGlue = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs";
console.log("\nproc glue:\n", fs.readFileSync(procGlue, "utf8").slice(0, 2000));

const playGlue = findFileDeep("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook", (n, p) => n.includes("glue") && p.includes("procedural") && p.includes("extensions"));
function findFileDeep(dir, pred) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory() && !["node_modules", "target"].includes(e.name)) {
      const hit = findFileDeep(p, pred);
      if (hit) return hit;
    } else if (pred(e.name, p)) return p;
  }
  return null;
}
console.log("play glue", playGlue);
if (playGlue) console.log(fs.readFileSync(playGlue, "utf8").slice(0, 1500));

// How many lines / structure of brep - regions
const brepPath = fs.readFileSync(path.join(ticket, "brep-component.path"), "utf8").trim();
const brepFile = path.join(brepPath, fs.readdirSync(brepPath).find((n) => n.includes("component")));
const brep = fs.readFileSync(brepFile, "utf8");
const regions = [...brep.matchAll(/\/\/ #region (.+)/g)].map((m) => m[1]);
console.log("\nbrep regions:", regions);
console.log("brep lines", brep.split("\n").length);

// Check 3d TS imports
const tsIndex = "/Users/ueli/Documents/semio/✏️s/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📦️index.ts";
const vitest = "/Users/ueli/Documents/semio/✏️s/🔨️modules/🧊️3d/📦️packages/🟦️typescript/🧪️vitest.config.ts";
console.log("\n=== 3d index brep refs ===");
fs.readFileSync(tsIndex, "utf8")
  .split("\n")
  .forEach((l, i) => {
    if (/brep|flow_extension/.test(l)) console.log(i + 1, l);
  });
console.log("\n=== vitest ===");
console.log(fs.readFileSync(vitest, "utf8"));

// Check for pkg/standalone in brep folder
console.log("\nbrep dir listing:", fs.readdirSync(brepPath));

// Look at how flow core currently exposes things - is there already a geometry kernel region?
core.forEach((l, i) => {
  if (/#region/.test(l) && /[Gg]eometr|[Bb]rep|[Kk]ernel|[Tt]essell/.test(l)) console.log("core region", i + 1, l);
});

// Check builtin_flow_extensions list function
core.forEach((l, i) => {
  if (/builtin_flow_extensions|FlowExtensionSpec|flow_extension_brep/.test(l)) console.log(i + 1, l.trim());
});
