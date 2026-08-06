/**
 * @emoji 🚚 One-shot migrator: plugin-root 📚️examples → artifact 📚️examples/♻️reuse/…/*.semio
 * Run: `bun .🦑️repo/🎫️tickets/26/08/06/UNIVERSAL-SEMIO-FORMAT-AND-ARTIFACT-OWNED-EXAMPLES/migrate-plugin-examples.mjs <pluginDir>`
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";

const pluginRoot = process.argv[2];
if (!pluginRoot) {
  console.error("usage: bun migrate-plugin-examples.mjs <absolute-plugin-root>");
  process.exit(1);
}

const wrapBinary = (token, payload) => {
  const magic = Buffer.from([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
  const tb = Buffer.from(token, "utf8");
  const len = Buffer.alloc(4);
  len.writeUInt32LE(tb.length, 0);
  return Buffer.concat([magic, len, tb, payload]);
};

const artifactsDir = join(pluginRoot, "🗿️artifacts");
const legacyExamples = join(pluginRoot, "📚️examples");
if (!existsSync(artifactsDir) || !existsSync(legacyExamples)) {
  console.error("[migrate] missing artifacts or legacy examples dir");
  process.exit(1);
}

for (const artifact of readdirSync(artifactsDir)) {
  const artifactDir = join(artifactsDir, artifact);
  const legacyDir = legacyExamples;
  const files = readdirSync(legacyDir).filter((name) => !name.startsWith("."));
  const match = files.find((f) => f.includes(artifact.replace(/^[^\w]+/, "")) || f.endsWith(".semio"));
  if (!match) continue;
  const pluginId = basename(pluginRoot).replace(/^[^\w]+/, "") || "plugin";
  const artifactSlug = artifact.replace(/^[^\w]+/, "");
  const envelope = `${pluginId}.${artifactSlug}`;
  const ex = join(artifactDir, "📚️examples/♻️reuse");
  const dslPath = join(ex, "🗣️dsls/♻️reuse", `🧬️component.${envelope}.dsl.semio`);
  mkdirSync(dirname(dslPath), { recursive: true });
  const body = readFileSync(join(legacyDir, match), "utf8");
  writeFileSync(dslPath, `semio ${envelope}.dsl v1\n${body.replace(/^semio[^\n]*\n/, "")}`, "utf8");
  for (const [comp, dir] of [
    ["op", "🔧️ops", `semio ${envelope}.op v1\nedit reuse started="0" actor=example\n`],
    ["pack", "🎒️packs", null],
    ["spr", "📡️sprs", null],
  ]) {
    const p = join(ex, dir, "♻️reuse", `🧬️component.${envelope}.${comp}.semio`);
    mkdirSync(dirname(p), { recursive: true });
    if (comp === "op") writeFileSync(p, dir[2], "utf8");
    else writeFileSync(p, wrapBinary(`${envelope}.${comp} v1`, Buffer.from([0])));
  }
  console.log(`[migrate] ${envelope} → ${dslPath}`);
}
