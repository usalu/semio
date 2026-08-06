/**
 * @emoji 🚚 Migrates puzzle, block, procedural plugin examples to artifact-owned .semio.
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";

const wrapBinary = (token, payload) => {
  const magic = Buffer.from([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
  const tb = Buffer.from(token, "utf8");
  const len = Buffer.alloc(4);
  len.writeUInt32LE(tb.length, 0);
  return Buffer.concat([magic, len, tb, payload]);
};

const LEAF_RS = {
  dsl: `//! Example — DSL \`.semio\` leaf.

/// @emoji 📜️ Bundled DSL example bytes (envelope is in the file content).
pub const EXAMPLE: &str = include_str!("🧬️component.{envelope}.dsl.semio");
`,
  op: `//! Example — op \`.semio\` leaf.

/// @emoji 🔧 Bundled op example text.
pub const EXAMPLE: &str = include_str!("🧬️component.{envelope}.op.semio");
`,
  pack: `//! Example — pack \`.semio\` leaf.

/// @emoji 📦️ Bundled pack example bytes.
pub const EXAMPLE: &[u8] = include_bytes!("🧬️component.{envelope}.pack.semio");
`,
  spr: `//! Example — spr \`.semio\` leaf.

/// @emoji 📡 Bundled spr example bytes.
pub const EXAMPLE: &[u8] = include_bytes!("🧬️component.{envelope}.spr.semio");
`,
};

const PLUGIN_CONFIG = {
  puzzle: {
    id: "puzzle",
    artifacts: {
      puzzle2d: "◻2d",
      puzzle3d: "🧊️3d",
      puzzle5d: "🖐️5d",
    },
    suffixes: [".puzzle2d", ".puzzle3d", ".puzzle5d"],
  },
  block: {
    id: "block",
    artifacts: {
      block2d: "◻2d",
      block3d: "🧊️3d",
      block5d: "🖐️5d",
    },
    suffixes: [".block2d", ".block3d", ".block5d"],
  },
  procedural: {
    id: "procedural",
    artifacts: {
      procedural2d: "🌀️procedural2d",
      procedural3d: "🧊️procedural3d",
    },
    suffixes: [".procedural2d", ".procedural3d"],
  },
};

function stripEmojiPrefix(name) {
  return name.replace(/^[^\w]+/, "");
}

function exampleSlugFromFile(fileName) {
  const base = stripEmojiPrefix(fileName);
  for (const suf of [".puzzle2d", ".puzzle3d", ".puzzle5d", ".block2d", ".block3d", ".block5d", ".procedural2d", ".procedural3d"]) {
    if (base.endsWith(suf)) return base.slice(0, -suf.length);
  }
  return base;
}

function ensureExampleTree(artifactDir, exampleSlug, envelope) {
  const exRoot = join(artifactDir, "📚️examples", exampleSlug);
  const dirs = [
    ["dsl", "🗣️dsls/♻️reuse"],
    ["op", "🔧️ops/♻️reuse"],
    ["pack", "🎒️packs/♻️reuse"],
    ["spr", "📡️sprs/♻️reuse"],
  ];
  for (const [comp, rel] of dirs) {
    const dir = join(exRoot, rel);
    mkdirSync(dir, { recursive: true });
    const leaf = join(dir, "🦀️component.rs");
    if (!existsSync(leaf)) {
      writeFileSync(leaf, LEAF_RS[comp].replaceAll("{envelope}", envelope), "utf8");
    }
  }
  return exRoot;
}

function migratePlugin(pluginRoot, cfg) {
  const legacyExamples = join(pluginRoot, "📚️examples");
  if (!existsSync(legacyExamples)) {
    console.log(`[skip] no legacy examples: ${pluginRoot}`);
    return [];
  }
  const moved = [];
  const files = readdirSync(legacyExamples).filter((n) => !n.startsWith("."));
  for (const file of files) {
    let artifactSlug = null;
    for (const suf of cfg.suffixes) {
      if (file.endsWith(suf)) {
        artifactSlug = stripEmojiPrefix(suf.slice(1));
        break;
      }
    }
    if (!artifactSlug) {
      console.warn(`[warn] no artifact for ${file}`);
      continue;
    }
    const artifactFolder = cfg.artifacts[artifactSlug];
    if (!artifactFolder) {
      console.warn(`[warn] unknown artifact slug ${artifactSlug}`);
      continue;
    }
    const artifactDir = join(pluginRoot, "🗿️artifacts", artifactFolder);
    const envelope = `${cfg.id}.${artifactSlug}`;
    const exampleSlug = exampleSlugFromFile(file);
    ensureExampleTree(artifactDir, exampleSlug, envelope);

    const body = readFileSync(join(legacyExamples, file), "utf8").replace(/^semio[^\n]*\n/, "");
    const dslPath = join(
      artifactDir,
      "📚️examples",
      exampleSlug,
      "🗣️dsls/♻️reuse",
      `🧬️component.${envelope}.dsl.semio`,
    );
    writeFileSync(dslPath, `semio ${envelope}.dsl v1\n${body}`, "utf8");

    const opPath = join(
      artifactDir,
      "📚️examples",
      exampleSlug,
      "🔧️ops/♻️reuse",
      `🧬️component.${envelope}.op.semio`,
    );
    writeFileSync(opPath, `semio ${envelope}.op v1\nedit reuse started="0" actor=example\n`, "utf8");

    const packPath = join(
      artifactDir,
      "📚️examples",
      exampleSlug,
      "🎒️packs/♻️reuse",
      `🧬️component.${envelope}.pack.semio`,
    );
    writeFileSync(packPath, wrapBinary(`${envelope}.pack v1`, Buffer.from([0])));

    const sprPath = join(
      artifactDir,
      "📚️examples",
      exampleSlug,
      "📡️sprs/♻️reuse",
      `🧬️component.${envelope}.spr.semio`,
    );
    writeFileSync(sprPath, wrapBinary(`${envelope}.spr v1`, Buffer.from([0])));

    rmSync(join(legacyExamples, file));
    moved.push({ file, artifactSlug, exampleSlug, dslPath });
    console.log(`[ok] ${file} → ${dslPath}`);
  }
  const remaining = readdirSync(legacyExamples).filter((n) => !n.startsWith("."));
  if (remaining.length === 0) {
    rmSync(legacyExamples, { recursive: true });
  }
  return moved;
}

const repo = "/Users/ueli/Documents/semio";
const plugins = [
  join(repo, "✏️s/🔌️plugins/🧩️puzzle"),
  join(repo, "✏️s/🔌️plugins/🧱️block"),
  join(repo, "✏️s/🔌️plugins/🌀️procedural"),
];

const log = [];
for (const root of plugins) {
  const name = stripEmojiPrefix(basename(root));
  log.push(...migratePlugin(root, PLUGIN_CONFIG[name]));
}
writeFileSync(
  join(repo, ".🦑️repo/🎫️tickets/26/08/06/UNIVERSAL-SEMIO-FORMAT-AND-ARTIFACT-OWNED-EXAMPLES/migrate-log.json"),
  JSON.stringify(log, null, 2),
  "utf8",
);
