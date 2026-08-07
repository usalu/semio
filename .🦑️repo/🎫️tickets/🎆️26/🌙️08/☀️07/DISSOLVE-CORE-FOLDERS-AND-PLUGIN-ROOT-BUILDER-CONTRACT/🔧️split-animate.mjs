import { readFileSync, writeFileSync, mkdirSync, rmSync } from "fs";
import { dirname, join } from "path";
import { execSync } from "child_process";

const corePath = execSync(`find ✏️s/🔌️plugins/🎞️animate -type d -name '🎬️core' | head -1`, { encoding: "utf8" }).trim();
const file = join(corePath, "🦀️component.rs");
const engineDir = dirname(corePath);
console.log({ corePath, engineDir });
const lines = readFileSync(file, "utf8").split("\n");

const mods = [];
for (let i = 0; i < lines.length; i++) {
  const m = lines[i].match(/^mod ([a-z0-9_]+) \{/);
  if (m) mods.push({ name: m[1], start: i });
}
for (let i = 0; i < mods.length; i++) {
  const start = mods[i].start;
  let depth = 0;
  let end = start;
  for (let j = start; j < lines.length; j++) {
    for (const ch of lines[j]) {
      if (ch === "{") depth++;
      else if (ch === "}") depth--;
    }
    if (j > start && depth === 0) {
      end = j;
      break;
    }
  }
  mods[i].end = end;
  mods[i].body = lines.slice(start, end + 1).join("\n");
}
console.log(
  "mods",
  mods.map((m) => `${m.name}:${m.start}-${m.end}`),
);

const groups = {
  "🎞️animation": ["animation", "animations_catalog"],
  "🎬️scene": ["scene", "section", "sobject"],
  "📐️geometry": ["geometry", "three_d", "axes"],
  "🎥️camera": ["camera", "matrix"],
  "🔤️text": ["color", "text"],
  "⏱️rate": ["rate", "updater"],
  "🎛️config": ["config", "hash", "graph"],
};

const used = new Set();
for (const [folder, names] of Object.entries(groups)) {
  const dir = join(engineDir, folder);
  mkdirSync(dir, { recursive: true });
  const parts = [];
  for (const name of names) {
    const mod = mods.find((m) => m.name === name);
    if (!mod) {
      console.error("missing mod", name);
      continue;
    }
    used.add(name);
    parts.push(mod.body.replace(/^mod /, "pub mod "));
  }
  const out =
    `//! 🎞️ Animate engine facet: ${folder}\n\n#![allow(clippy::too_many_arguments, clippy::type_complexity)]\n\n` +
    parts.join("\n\n");
  writeFileSync(join(dir, "🦀️component.rs"), out);
  console.log("wrote", folder, out.split("\n").length);
}
console.log(
  "leftover",
  mods.filter((m) => !used.has(m.name)).map((m) => m.name),
);

const glue = execSync(`find ✏️s/🔌️plugins/🎞️animate/📦️packages -name '📦️glue.rs' | head -1`, {
  encoding: "utf8",
}).trim();
let g = readFileSync(glue, "utf8");
const newBlock = `            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎞️animation/🦀️component.rs"]
            pub mod animation;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎬️scene/🦀️component.rs"]
            pub mod scene;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/📐️geometry/🦀️component.rs"]
            pub mod geometry;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🔤️text/🦀️component.rs"]
            pub mod text;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/⏱️rate/🦀️component.rs"]
            pub mod rate;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎛️config/🦀️component.rs"]
            pub mod config;
            pub mod animate {
                pub use super::animation::*;
                pub use super::scene::*;
                pub use super::geometry::*;
                pub use super::camera::*;
                pub use super::text::*;
                pub use super::rate::*;
                pub use super::config::*;
            }`;
if (!g.includes("🎬️core")) throw new Error("no 🎬️core in glue");
g = g.replace(/#\[path = "[^"]*🎬️core[^"]*"\]\s*\n\s*pub mod animate;/, newBlock);
writeFileSync(glue, g);
console.log("glue updated");
rmSync(corePath, { recursive: true, force: true });
console.log("deleted", corePath);
