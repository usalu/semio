/**
 * @emoji 🚚 Migrates norm (15 artifacts), cad, draw, note to artifact-owned .semio examples (GIS shape).
 * Run from repo root: `bun .🦑️repo/🎫️tickets/26/08/06/UNIVERSAL-SEMIO-FORMAT-AND-ARTIFACT-OWNED-EXAMPLES/migrate-norm-cad-draw-note.mjs`
 */
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";

const REPO = join(import.meta.dir, "../../../../../../");

const wrapBinary = (token, payload) => {
  const magic = Buffer.from([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
  const tb = Buffer.from(token, "utf8");
  const len = Buffer.alloc(4);
  len.writeUInt32LE(tb.length, 0);
  return Buffer.concat([magic, len, tb, payload]);
};

const slug = (name) => name.replace(/^[^\p{L}\p{N}]+/u, "");

const leafRs = (emoji, kind, envelope, comp, isBinary) => `//! ${emoji} Example — ${comp} \`.semio\` leaf.

/// @emoji ${isBinary ? "📦️" : "📜️"} Bundled ${envelope}.${comp} example ${isBinary ? "bytes" : "text"}.
pub const EXAMPLE: ${isBinary ? "&[u8]" : "&str"} = ${isBinary ? "include_bytes" : "include_str"}!("🧬️component.${envelope}.${comp}.semio");
`;

const writeExampleSet = (artifactDir, pluginId, artifactSlug, exampleLeaf, dslBody) => {
  const envelope = `${pluginId}.${artifactSlug}`;
  const exRoot = join(artifactDir, "📚️examples", exampleLeaf);
  const dslPath = join(exRoot, "🗣️dsls", exampleLeaf, `🧬️component.${envelope}.dsl.semio`);
  const opPath = join(exRoot, "🔧️ops", exampleLeaf, `🧬️component.${envelope}.op.semio`);
  const packPath = join(exRoot, "🎒️packs", exampleLeaf, `🧬️component.${envelope}.pack.semio`);
  const sprPath = join(exRoot, "📡️sprs", exampleLeaf, `🧬️component.${envelope}.spr.semio`);

  const body = dslBody.replace(/^semio[^\n]*\n/, "");
  mkdirSync(dirname(dslPath), { recursive: true });
  writeFileSync(dslPath, `semio ${envelope}.dsl v1\n${body}`, "utf8");

  mkdirSync(dirname(opPath), { recursive: true });
  writeFileSync(opPath, `semio ${envelope}.op v1\nedit reuse started="0" actor=example\n`, "utf8");

  mkdirSync(dirname(packPath), { recursive: true });
  writeFileSync(packPath, wrapBinary(`${envelope}.pack v1`, Buffer.from([0])));

  mkdirSync(dirname(sprPath), { recursive: true });
  writeFileSync(sprPath, wrapBinary(`${envelope}.spr v1`, Buffer.from([0])));

  for (const [p, comp, bin] of [
    [join(exRoot, "🗣️dsls", exampleLeaf, "🦀️component.rs"), "dsl", false],
    [join(exRoot, "🔧️ops", exampleLeaf, "🦀️component.rs"), "op", false],
    [join(exRoot, "🎒️packs", exampleLeaf, "🦀️component.rs"), "pack", true],
    [join(exRoot, "📡️sprs", exampleLeaf, "🦀️component.rs"), "spr", true],
  ]) {
    const emojis = { dsl: "📜️", op: "🔧", pack: "📦️", spr: "📡" };
    writeFileSync(p, leafRs(emojis[comp], comp, envelope, comp, bin), "utf8");
  }

  return { envelope, dslRel: `../../📚️examples/${exampleLeaf}/🗣️dsls/${exampleLeaf}/🧬️component.${envelope}.dsl.semio` };
};

const fixtureDir = join(import.meta.dir, "fixtures");
const wireAfter = join(
  REPO,
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/NORM-PLUGIN-SHAPE-V2-TREE-PURITY-RETROFIT/🧪️wire-after.txt",
);

const readDefaultBody = (artifactSlug) => {
  const p = join(fixtureDir, `norm.${artifactSlug}.dsl.body`);
  if (existsSync(p)) return readFileSync(p, "utf8");
  if (!existsSync(wireAfter)) throw new Error(`missing default dsl source for ${artifactSlug}`);
  const prefix = `${artifactSlug} | set-document/default | set-document document { `;
  const line = readFileSync(wireAfter, "utf8").split("\n").find((l) => l.startsWith(prefix));
  if (!line) throw new Error(`no wire-after default line for ${artifactSlug}`);
  const start = line.indexOf(prefix) + prefix.length;
  const end = line.lastIndexOf(" } |");
  if (end < start) throw new Error(`failed to parse wire-after body for ${artifactSlug}`);
  return line.slice(start, end);
};

const migrateNorm = () => {
  const pluginRoot = join(REPO, "✏️s/🔌️plugins/📕️norm");
  const legacyRoot = join(pluginRoot, "📚️examples");
  const artifactsDir = join(pluginRoot, "🗿️artifacts");
  const pluginId = "norm";

  const legacyByArtifact = {
    en1990: { leaf: "📕️high-consequence-office", file: "📘️en1990/📕️high-consequence-office.en1990" },
    en1991: { leaf: "📕️retail-hydrocarbon-fire", file: "📘️en1991/📕️retail-hydrocarbon-fire.en1991" },
    en1992: { leaf: "📕️liquid-retaining-fem-anchor", file: "📘️en1992/📕️liquid-retaining-fem-anchor.en1992" },
    en1993: { leaf: "📕️high-strength-connection", file: "📘️en1993/📕️high-strength-connection.en1993" },
    en1994: { leaf: "📕️composite-bridge-girder", file: "📘️en1994/📕️composite-bridge-girder.en1994" },
    en1995: { leaf: "📕️glulam-footbridge", file: "📘️en1995/📕️glulam-footbridge.en1995" },
    en1996: { leaf: "📕️loadbearing-wall", file: "📘️en1996/📕️loadbearing-wall.en1996" },
    en1997: { leaf: "📕️default", file: "📘️en1997/📕️default.en1997" },
    en1998: { leaf: "📕️seismic-rc-frame", file: "📘️en1998/📕️seismic-rc-frame.en1998" },
    en1999: { leaf: "📕️aluminium-roof-purlin", file: "📘️en1999/📕️aluminium-roof-purlin.en1999" },
    iso16757: { leaf: "📕️default", file: "📓️iso16757/📕️default.iso16757" },
  };

  const defaultArtifacts = ["din18599", "din16798", "din4108", "vdi3805"];

  const dslPaths = {};
  for (const artifactName of readdirSync(artifactsDir)) {
    const artifactSlug = slug(artifactName);
    const artifactDir = join(artifactsDir, artifactName);
    let body;
    let exampleLeaf;
    if (legacyByArtifact[artifactSlug]) {
      const { leaf, file } = legacyByArtifact[artifactSlug];
      exampleLeaf = leaf;
      body = readFileSync(join(legacyRoot, file), "utf8");
    } else if (defaultArtifacts.includes(artifactSlug)) {
      exampleLeaf = "♻️default";
      body = readDefaultBody(artifactSlug);
    } else {
      console.warn(`[norm] skip unknown artifact ${artifactSlug}`);
      continue;
    }
    const { dslRel } = writeExampleSet(artifactDir, pluginId, artifactSlug, exampleLeaf, body);
    dslPaths[artifactSlug] = dslRel;
    console.log(`[norm] ${pluginId}.${artifactSlug} ← ${exampleLeaf}`);
  }

  if (existsSync(legacyRoot)) rmSync(legacyRoot, { recursive: true, force: true });
  return dslPaths;
};

const migrateSingleArtifactPlugin = (pluginEmojiFolder, artifactEmojiFolder, pluginId, artifactSlug, legacyFile, exampleLeaf) => {
  const pluginRoot = join(REPO, "✏️s/🔌️plugins", pluginEmojiFolder);
  const artifactDir = join(pluginRoot, "🗿️artifacts", artifactEmojiFolder);
  const legacyPath = join(pluginRoot, "📚️examples", legacyFile);
  const body = readFileSync(legacyPath, "utf8");
  const { dslRel } = writeExampleSet(artifactDir, pluginId, artifactSlug, exampleLeaf, body);
  rmSync(join(pluginRoot, "📚️examples"), { recursive: true, force: true });
  console.log(`[${pluginId}] ${pluginId}.${artifactSlug}`);
  return dslRel;
};

migrateNorm();
migrateSingleArtifactPlugin("📐️cad", "📐️cad", "cad", "cad", "📐️default.cad", "♻️default");
migrateSingleArtifactPlugin("🖍️draw", "🖍️draw", "draw", "draw", "🖍️semio.draw", "♻️semio");
migrateSingleArtifactPlugin("🗒️note", "🗒️note", "note", "note", "🗒️semio.note", "♻️semio");
console.log("[migrate] done");
