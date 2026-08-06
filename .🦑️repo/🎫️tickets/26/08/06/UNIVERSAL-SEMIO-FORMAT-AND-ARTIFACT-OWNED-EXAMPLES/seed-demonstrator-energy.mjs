/**
 * @emoji 🎪 Seed first artifacts for demonstrator + energy plugins.
 */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

const wrapBinary = (token) => {
  const magic = Buffer.from([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
  const tb = Buffer.from(token, "utf8");
  const len = Buffer.alloc(4);
  len.writeUInt32LE(tb.length, 0);
  return Buffer.concat([magic, len, tb, Buffer.from([0])]);
};

const seedArtifact = (pluginPath, artifactEmoji, pluginId, artifactId, dslBody) => {
  const art = join(pluginPath, "🗿️artifacts", artifactEmoji);
  const envelope = `${pluginId}.${artifactId}`;
  const ex = join(art, "📚️examples/♻️reuse");
  const dslDir = join(ex, "🗣️dsls/♻️reuse");
  mkdirSync(dslDir, { recursive: true });
  writeFileSync(join(dslDir, `🧬️component.${envelope}.dsl.semio`), `semio ${envelope}.dsl v1\n${dslBody}\n`, "utf8");
  writeFileSync(join(dslDir, "🦀️component.rs"), `//! ♻️ Example — DSL leaf.\n\n/// @emoji 📜️ Bundled DSL example.\npub const EXAMPLE: &str = include_str!("🧬️component.${envelope}.dsl.semio");\n`);
  for (const [comp, dir, body] of [
    ["op", "🔧️ops", `semio ${envelope}.op v1\nedit reuse started="0" actor=example\n`],
    ["pack", "🎒️packs", null],
    ["spr", "📡️sprs", null],
  ]) {
    const leaf = join(ex, dir, "♻️reuse");
    mkdirSync(leaf, { recursive: true });
    const p = join(leaf, `🧬️component.${envelope}.${comp}.semio`);
  if (comp === "op") writeFileSync(p, body, "utf8");
    else writeFileSync(p, wrapBinary(`${envelope}.${comp} v1`));
    const isBin = comp !== "op";
    writeFileSync(
      join(leaf, "🦀️component.rs"),
      isBin
        ? `//! ♻️ Example — ${comp} leaf.\n\npub const EXAMPLE: &[u8] = include_bytes!("🧬️component.${envelope}.${comp}.semio");\n`
        : `//! ♻️ Example — ${comp} leaf.\n\npub const EXAMPLE: &str = include_str!("🧬️component.${envelope}.${comp}.semio");\n`,
    );
  }
  const componentRs = join(art, "🦀️component.rs");
  if (!existsSync(componentRs)) {
    writeFileSync(
      componentRs,
      `//! 🎪 Artifact root for ${pluginId}.${artifactId}.\n\n/// @emoji 🔖️ Store schema id.\npub const SCHEMA: &str = "${pluginId}.${artifactId}";\n`,
    );
  }
};

seedArtifact(
  join(REPO, "✏️s/🔌️plugins/🎪️demonstrator"),
  "🎪️playground",
  "demonstrator",
  "playground",
  'schema=demonstrator.playground id=demo title="Demonstrator Playground"',
);
seedArtifact(
  join(REPO, "✏️s/🔌️plugins/🔋️energy"),
  "🔋️model",
  "energy",
  "model",
  "schema=energy.model id=office-skeleton title=\"Office skeleton\"",
);
console.log("[seed-demonstrator-energy] done");
