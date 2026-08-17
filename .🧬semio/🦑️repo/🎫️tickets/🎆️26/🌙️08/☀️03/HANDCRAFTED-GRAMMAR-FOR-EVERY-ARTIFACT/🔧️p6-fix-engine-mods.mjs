#!/usr/bin/env bun
import { readFileSync, writeFileSync, readdirSync, statSync, existsSync } from "fs";
import { join } from "path";

const repo = "/Users/ueli/Documents/semio";
const pluginsRoot = join(repo, "✏️s/🔌️plugins");

function rustModForArtifact(artRel) {
  const parts = artRel.split("/");
  const pluginIdx = parts.indexOf("🔌️plugins");
  const artFolder = parts[parts.length - 1];
  const gluePath = join(
    repo,
    parts.slice(0, pluginIdx + 2).join("/"),
    "📦️packages/🦀️rust/📦️glue.rs",
  );
  const glue = readFileSync(gluePath, "utf8");
  const re =
    /pub mod (\w+) \{[\s\S]*?\[path = "\.\.\/\.\.\/🗿️artifacts\/([^"\]]+)\//g;
  const hits = [];
  let m;
  while ((m = re.exec(glue)) !== null) {
    hits.push({ mod: m[1], folder: m[2] });
  }
  const hit = hits.find((h) => h.folder === artFolder);
  if (!hit) throw new Error(`no mod for ${artRel} folder ${artFolder}`);
  return hit.mod;
}

const fixed = [];
for (const plugin of readdirSync(pluginsRoot)) {
  const artifacts = join(pluginsRoot, plugin, "🗿️artifacts");
  try {
    if (!statSync(artifacts).isDirectory()) continue;
  } catch {
    continue;
  }
  for (const art of readdirSync(artifacts)) {
    const artRel = `✏️s/🔌️plugins/${plugin}/🗿️artifacts/${art}`;
    const engine = join(artifacts, art, "⚙️engine/🦀️component.rs");
    if (!existsSync(engine)) continue;
    let t = readFileSync(engine, "utf8");
    if (!t.includes("fn register_pilot_languages")) continue;
    const mod = rustModForArtifact(artRel);
    const before = t;
    for (const wrong of ["artifacts", "core", "engine"]) {
      t = t.replaceAll(
        `crate::artifacts::${wrong}::`,
        `crate::artifacts::${mod}::`,
      );
    }
    if (t !== before) {
      writeFileSync(engine, t);
      fixed.push({ artRel, mod });
    }
  }
}

console.log(JSON.stringify({ fixedCount: fixed.length, fixed }, null, 2));
