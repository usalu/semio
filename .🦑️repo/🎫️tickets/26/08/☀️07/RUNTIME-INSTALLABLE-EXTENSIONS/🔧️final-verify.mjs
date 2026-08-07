import fs from "fs";
import path from "path";

function find(pred) {
  function walk(dir) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "target", ".git"].includes(e.name)) continue;
      const p = path.join(dir, e.name);
      try {
        if (e.isDirectory()) {
          const hit = walk(p);
          if (hit) return hit;
        } else if (pred(p, e.name)) return p;
      } catch {}
    }
    return null;
  }
  return walk(".");
}

const osComp = find((p, n) => p.endsWith("💻️os/🦀️component.rs"));
console.log(osComp);
const lines = fs.readFileSync(osComp, "utf8").split("\n");
for (let i = 0; i < 120; i++) {
  if (/mod space|path.*space|🪐️space/.test(lines[i])) console.log(i + 1, lines[i]);
}

// Verify space ops section integrity
const space = find((p, n) => p.includes("modules/🪐️space/🦀️component.rs"));
const s = fs.readFileSync(space, "utf8");
const checks = [
  "pub extensions: Vec<InstalledExtension>",
  "struct InstalledExtension",
  "InstallExtension {",
  "UninstallExtension {",
  "SetExtensionEnabled {",
  "install_extension: Option<InstalledExtension>",
  "uninstall_extension_id",
  "set_extension_enabled_id",
  "demo_extension(",
  "SpaceOperation::InstallExtension",
  "SpaceOperation::UninstallExtension",
  "SpaceOperation::SetExtensionEnabled",
];
for (const c of checks) console.log(c, s.includes(c));

// Shell type-in-function issue: move type? check for syntax around lifecycle
const shell = find((p, n) => p.includes("ShellHost") && n.endsWith(".tsx"));
const t = fs.readFileSync(shell, "utf8");
const i = t.indexOf("type ExtensionLedgerEntry");
console.log("\nledger type context:\n", t.slice(i - 100, i + 200));
