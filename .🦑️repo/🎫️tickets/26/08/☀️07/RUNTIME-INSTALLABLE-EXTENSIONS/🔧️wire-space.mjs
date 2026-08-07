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

const glue = find((p, n) => p.includes("💻️os/🖥️host/📦️packages/🦀️rust") && n.includes("glue"));
let text = fs.readFileSync(glue, "utf8");
if (text.includes("🪐️space/🦀️component.rs")) {
  console.log("space already wired");
  process.exit(0);
}

const old = `#[cfg(feature = "os-host-full")]
#[path = "../../../🔨️modules/🔁️workflow/🦀️component.rs"]
pub mod workflow_kernel;

#[path = "../../🦀️component.rs"]
mod host_core;
pub use host_core::*;
`;

const neu = `#[cfg(feature = "os-host-full")]
#[path = "../../../🔨️modules/🔁️workflow/🦀️component.rs"]
pub mod workflow_kernel;

#[cfg(feature = "os-host-full")]
#[path = "../../../🔨️modules/🪐️space/🦀️component.rs"]
pub mod space;

#[path = "../../🦀️component.rs"]
mod host_core;
pub use host_core::*;
`;

if (!text.includes(old)) throw new Error("glue marker not found: " + JSON.stringify(text));
fs.writeFileSync(glue, text.replace(old, neu));
console.log("wired space module into", glue);
