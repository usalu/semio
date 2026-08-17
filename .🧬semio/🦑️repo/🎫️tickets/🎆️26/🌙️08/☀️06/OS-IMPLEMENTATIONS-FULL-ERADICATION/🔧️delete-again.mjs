import fs from "fs";
import path from "path";

const OS = fs.readFileSync("/tmp/os-path.txt","utf8").trim();
const TICKET = fs.readFileSync("/tmp/os-ticket-path.txt","utf8").trim();
const REPO = "/Users/ueli/Documents/semio";
const log = [];
const note = (m) => { console.log(m); log.push(m); };

// Promote flow extensions out of sandwich into owner tree
const flowExtSrc = path.join(OS, "🔨️modules/🌊️flow/⚡️implementations/🦀️rust/🧩️extensions");
const flowExtDst = path.join(OS, "🔨️modules/🌊️flow/🧩️extensions");
if (fs.existsSync(flowExtSrc)) {
  fs.mkdirSync(flowExtDst, { recursive: true });
  for (const name of fs.readdirSync(flowExtSrc)) {
    const src = path.join(flowExtSrc, name);
    if (!fs.statSync(src).isDirectory()) continue;
    const destDir = path.join(flowExtDst, name);
    fs.mkdirSync(destDir, { recursive: true });
    const lib = path.join(src, "📦️lib.rs");
    if (fs.existsSync(lib)) {
      const dest = path.join(destDir, "🦀️component.rs");
      if (!fs.existsSync(dest)) {
        fs.writeFileSync(dest, fs.readFileSync(lib));
        note("PROMOTED_FLOW_EXT " + name);
      }
    }
    // Also keep package scaffolding under packages if Cargo.toml exists - fold into host later
  }
}

// Promote neural dag
const neuralDag = path.join(OS, "🔨️modules/🧠️neural/⚡️implementations/🦀️rust/🕸️dag/📦️lib.rs");
const neuralComp = path.join(OS, "🔨️modules/🧠️neural/🦀️component.rs");
if (fs.existsSync(neuralDag) && !fs.existsSync(neuralComp)) {
  fs.writeFileSync(neuralComp, fs.readFileSync(neuralDag));
  note("PROMOTED neural dag component");
}

// Promote flow root facade component if missing
const flowRootComp = path.join(OS, "🔨️modules/🌊️flow/🦀️component.rs");
const flowCoreExt = path.join(flowExtDst, "🟙core", "🦀️component.rs");
// try find core extension
for (const name of fs.existsSync(flowExtDst) ? fs.readdirSync(flowExtDst) : []) {
  if (name.includes("core")) {
    const c = path.join(flowExtDst, name, "🦀️component.rs");
    if (fs.existsSync(c) && !fs.existsSync(flowRootComp)) {
      // Don't copy core as flow root - flow root is different. Leave missing.
    }
  }
}

// Delete ALL implementations under os
const implDirs = [];
function walk(dir) {
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, ent.name);
    if (!ent.isDirectory()) continue;
    if (ent.name === "⚡️implementations") implDirs.push(p);
    else if (ent.name !== "target" && ent.name !== "node_modules") walk(p);
  }
}
walk(OS);
implDirs.sort((a,b) => b.length - a.length);
let deleted = 0;
for (const d of implDirs) {
  fs.rmSync(d, { recursive: true, force: true });
  deleted++;
  note("DELETED " + path.relative(REPO, d));
}
// Also delete weird pkg/implementation leftovers under flow core
const weird = path.join(OS, "🔨️modules/🌊️flow/🟙core/pkg");
// find any leftover implementation-named dirs
function walkWeird(dir) {
  if (!fs.existsSync(dir)) return;
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, ent.name);
    if (!ent.isDirectory()) continue;
    if (ent.name === "⚡️implementation" || ent.name === "⚡️implementations" || ent.name === "pkg") {
      // remove implementation sandwiches only
      if (ent.name.startsWith("⚡️")) {
        fs.rmSync(p, { recursive: true, force: true });
        note("DELETED_WEIRD " + path.relative(REPO, p));
      } else walkWeird(p);
    } else walkWeird(p);
  }
}
walkWeird(path.join(OS, "🔨️modules/🌊️flow"));

let remaining = 0;
function count(dir) {
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, ent.name);
    if (!ent.isDirectory()) continue;
    if (ent.name === "⚡️implementations") remaining++;
    else if (ent.name !== "target") count(p);
  }
}
count(OS);
fs.writeFileSync(path.join(TICKET, "🧪delete-again-log.txt"), log.join("\n")+"\n");
fs.writeFileSync(path.join(TICKET, "🧪impl-dirs-remaining.txt"), `remaining=${remaining}\ndeleted=${deleted}\n`);
console.log("DONE remaining="+remaining+" deleted="+deleted);
