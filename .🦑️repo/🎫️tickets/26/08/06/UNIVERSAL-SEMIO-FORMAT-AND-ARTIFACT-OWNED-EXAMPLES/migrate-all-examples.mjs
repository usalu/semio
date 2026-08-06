/**
 * @emoji 🚚 Bulk migrator: legacy 📚️examples → artifact 📚️examples/♻️reuse/{dsls,packs,ops,sprs}/*.semio
 * Run from repo root: `bun .🦑️repo/🎫️tickets/26/08/06/UNIVERSAL-SEMIO-FORMAT-AND-ARTIFACT-OWNED-EXAMPLES/migrate-all-examples.mjs`
 */
import {
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { basename, dirname, join, relative } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const PLUGINS = join(REPO, "✏️s/🔌️plugins");

const slug = (name) => name.replace(/\uFE0F/g, "").replace(/^[^\p{L}\p{N}]+/u, "");

const wrapBinary = (token, payload = Buffer.from([0])) => {
  const magic = Buffer.from([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
  const tb = Buffer.from(token, "utf8");
  const len = Buffer.alloc(4);
  len.writeUInt32LE(tb.length, 0);
  return Buffer.concat([magic, len, tb, payload]);
};

const stripPreamble = (body) => body.replace(/^semio[^\n]*\n/, "");

const walkFiles = (dir, acc = []) => {
  if (!existsSync(dir)) return acc;
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    if (ent.name.startsWith(".")) continue;
    const p = join(dir, ent.name);
    if (ent.isDirectory()) walkFiles(p, acc);
    else acc.push(p);
  }
  return acc;
};

const resolveEnvelope = (pluginRoot, artifactDir) => {
  const pluginId = slug(basename(pluginRoot));
  const artifactId = slug(basename(artifactDir));
  const artifactRs = join(artifactDir, "🦀️component.rs");
  const dslRs = join(artifactDir, "🗣️dsl/🦀️component.rs");
  for (const f of [artifactRs, dslRs]) {
    if (!existsSync(f)) continue;
    const text = readFileSync(f, "utf8");
    const id = text.match(/#\[dsl\(id\s*=\s*"([^"]+)"/);
    if (id) return id[1];
    const ext = text.match(/#\[dsl\(extension\s*=\s*"([^"]+)"/);
    if (ext) {
      const e = ext[1];
      if (e === pluginId || e === artifactId) return `${pluginId}.${artifactId}`;
      return `${pluginId}.${e}`;
    }
  }
  return `${pluginId}.${artifactId}`;
};

const envelopeParts = (envelope) => {
  const dot = envelope.lastIndexOf(".");
  return { plugin: envelope.slice(0, dot), artifact: envelope.slice(dot + 1) };
};

const exampleMatchesArtifact = (filePath, artifactDir, envelope) => {
  const { artifact } = envelopeParts(envelope);
  const base = basename(filePath);
  const ext = base.includes(".") ? base.slice(base.lastIndexOf(".")) : "";
  const stem = ext ? base.slice(0, -ext.length) : base;
  const artifactSlug = slug(basename(artifactDir));
  if (base.endsWith(`.${artifact}`) || base.endsWith(`.${artifactSlug}`)) return true;
  if (stem.includes(artifact) || stem.includes(artifactSlug)) return true;
  const rel = relative(artifactDir, dirname(filePath));
  if (rel && !rel.startsWith("..") && slug(rel.split(/[/\\]/)[0]) === artifactSlug) return true;
  return false;
};

const writeLeafComponent = (leafDir, comp, envelope, isBinary) => {
  const rs = join(leafDir, "🦀️component.rs");
  const semioName = `🧬️component.${envelope}.${comp}.semio`;
  if (isBinary) {
    writeFileSync(
      rs,
      `//! ♻️ Example — ${comp} \`.semio\` leaf.\n\n/// @emoji 📦️ Bundled ${comp} example bytes.\npub const EXAMPLE: &[u8] = include_bytes!("${semioName}");\n`,
      "utf8",
    );
  } else {
    writeFileSync(
      rs,
      `//! ♻️ Example — ${comp} \`.semio\` leaf.\n\n/// @emoji 📜️ Bundled ${comp} example text (envelope is in the file content).\npub const EXAMPLE: &str = include_str!("${semioName}");\n`,
      "utf8",
    );
  }
};

const migrateArtifactExamples = (pluginRoot, artifactDir, legacyFiles, artifactOnlyFiles, force = false) => {
  const envelope = resolveEnvelope(pluginRoot, artifactDir);
  const exRoot = join(artifactDir, "📚️examples/♻️reuse");
  const candidates = force
    ? [...legacyFiles, ...artifactOnlyFiles]
    : [...legacyFiles, ...artifactOnlyFiles].filter((f) => exampleMatchesArtifact(f, artifactDir, envelope));
  if (!candidates.length && !legacyFiles.length) return null;

  const dslDir = join(exRoot, "🗣️dsls/♻️reuse");
  mkdirSync(dslDir, { recursive: true });

  const primary =
    candidates.find((f) => /default|sample|demo|semio|base/i.test(basename(f))) ?? candidates[0];
  if (primary) {
    const body = readFileSync(primary, "utf8");
    const dslPath = join(dslDir, `🧬️component.${envelope}.dsl.semio`);
    writeFileSync(dslPath, `semio ${envelope}.dsl v1\n${stripPreamble(body)}`, "utf8");
    for (const extra of candidates.filter((f) => f !== primary)) {
      const tag = slug(basename(extra)).replace(/\./g, "-").slice(0, 40);
      const p = join(dslDir, `🧬️component.${envelope}.${tag}.dsl.semio`);
      const b = readFileSync(extra, "utf8");
      writeFileSync(p, `semio ${envelope}.dsl v1\n${stripPreamble(b)}`, "utf8");
    }
  } else {
    const dslPath = join(dslDir, `🧬️component.${envelope}.dsl.semio`);
    writeFileSync(dslPath, `semio ${envelope}.dsl v1\n`, "utf8");
  }

  const opPath = join(exRoot, "🔧️ops/♻️reuse", `🧬️component.${envelope}.op.semio`);
  mkdirSync(dirname(opPath), { recursive: true });
  writeFileSync(opPath, `semio ${envelope}.op v1\nedit reuse started="0" actor=example\n`, "utf8");

  const packPath = join(exRoot, "🎒️packs/♻️reuse", `🧬️component.${envelope}.pack.semio`);
  mkdirSync(dirname(packPath), { recursive: true });
  writeFileSync(packPath, wrapBinary(`${envelope}.pack v1`));

  const sprPath = join(exRoot, "📡️sprs/♻️reuse", `🧬️component.${envelope}.spr.semio`);
  mkdirSync(dirname(sprPath), { recursive: true });
  writeFileSync(sprPath, wrapBinary(`${envelope}.spr v1`));

  writeLeafComponent(dslDir, "dsl", envelope, false);
  writeLeafComponent(dirname(opPath), "op", envelope, false);
  writeLeafComponent(dirname(packPath), "pack", envelope, true);
  writeLeafComponent(dirname(sprPath), "spr", envelope, true);

  return { envelope, dslRel: relative(artifactDir, join(dslDir, `🧬️component.${envelope}.dsl.semio`)) };
};

const migratePlugin = (pluginRoot) => {
  const artifactsDir = join(pluginRoot, "🗿️artifacts");
  if (!existsSync(artifactsDir)) return [];
  const legacyRoot = join(pluginRoot, "📚️examples");
  const legacyFiles = walkFiles(legacyRoot).filter((f) => !f.includes("♻️reuse"));
  const results = [];
  for (const artifact of readdirSync(artifactsDir)) {
    const artifactDir = join(artifactsDir, artifact);
    if (!statSync(artifactDir).isDirectory()) continue;
    const artifactExamples = join(artifactDir, "📚️examples");
    const artifactOnly = walkFiles(artifactExamples).filter(
      (f) => !f.includes("♻️reuse") && !f.endsWith(".semio"),
    );
    const r = migrateArtifactExamples(pluginRoot, artifactDir, legacyFiles, artifactOnly);
    if (r) {
      results.push(r);
      if (existsSync(artifactExamples) && !artifactExamples.includes("♻️reuse")) {
        for (const f of artifactOnly) rmSync(f, { force: true });
        try {
          const left = readdirSync(artifactExamples);
          if (left.length === 0) rmSync(artifactExamples, { recursive: true, force: true });
        } catch {
          /* */
        }
      }
    }
  }
  if (existsSync(legacyRoot)) {
    rmSync(legacyRoot, { recursive: true, force: true });
  }
  return results;
};

const migrateFromSourceFile = (pluginRoot, artifactName, sourceAbs) => {
  const artifactDir = join(pluginRoot, "🗿️artifacts", artifactName);
  if (!existsSync(artifactDir)) return null;
  return migrateArtifactExamples(pluginRoot, artifactDir, [sourceAbs], [], true);
};

const migrateFrameworkModule = (frameworkFile, pluginEmoji, artifactEmoji) => {
  const pluginRoot = join(PLUGINS, pluginEmoji);
  return migrateFromSourceFile(pluginRoot, artifactEmoji, frameworkFile);
};

const TARGET_PLUGINS = [
  "✒️writer",
  "🎞️animate",
  "🌿️vcs",
  "🎥️shooting",
  "🎬️sequence",
  "🏗️fem",
  "🏛️architect",
  "💠️lowpoly",
  "💡️reasoning",
  "📋️forms",
  "📏️layout",
  "📖️playbook",
  "📜️imperative",
  "🔱️trinity",
  "🖨️raster",
  "🪐️space",
  "🪵️sourcing",
  "📸️remodel",
  "🌊️flow",
  "🕸️dag",
  "➗️mathematical",
  "🏭️process",
];

for (const p of TARGET_PLUGINS) {
  const root = join(PLUGINS, p);
  if (!existsSync(root)) {
    console.warn(`[skip] missing ${p}`);
    continue;
  }
  const r = migratePlugin(root);
  console.log(`[ok] ${p} → ${r.length} artifact(s)`);
}

migrateFrameworkModule(
  join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📚️examples/🌊️default.flow"),
  "🌊️flow",
  "🌊️flow",
);
migrateFrameworkModule(
  join(
    REPO,
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/⚡️implementations/🦀️rust/📚️examples/🕸️demo.dag",
  ),
  "🕸️dag",
  "🕸️dag",
);

const mathArt = join(PLUGINS, "➗️mathematical/🗿️artifacts/➗️mathematical");
if (existsSync(mathArt)) {
  const body = `graph directed=true algorithm=topo nodes=[ id=a label=A x=40 y=60 id=b label=B x=240 y=20 ] edges=[ a->b ]\n`;
  const tmp = join(REPO, ".🦑️repo/🎫️tickets/26/08/06/UNIVERSAL-SEMIO-FORMAT-AND-ARTIFACT-OWNED-EXAMPLES/🧬️math.seed.mathematical");
  writeFileSync(tmp, body, "utf8");
  migrateFromSourceFile(join(PLUGINS, "➗️mathematical"), "➗️mathematical", tmp);
}

const remodelArt = join(PLUGINS, "📸️remodel/🗿️artifacts/📸️remodel");
if (existsSync(remodelArt)) {
  const body = `schema=remodel.project id=demo title=Demo\n`;
  const tmp = join(REPO, ".🦑️repo/🎫️tickets/26/08/06/UNIVERSAL-SEMIO-FORMAT-AND-ARTIFACT-OWNED-EXAMPLES/🧬️remodel.seed.remodel");
  writeFileSync(tmp, body, "utf8");
  migrateFromSourceFile(join(PLUGINS, "📸️remodel"), "📸️remodel", tmp);
}
