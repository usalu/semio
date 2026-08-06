/**
 * @emoji 📦 Ensure every migrated artifact has pack/op/spr/dsl example leaves + component.rs stubs.
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";

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

const resolveEnvelope = (artifactDir) => {
  for (const f of ["🦀️component.rs", "🗣️dsl/🦀️component.rs"]) {
    const p = join(artifactDir, f);
    if (!existsSync(p)) continue;
    const text = readFileSync(p, "utf8");
    const id = text.match(/#\[dsl\(id\s*=\s*"([^"]+)"/);
    if (id) return id[1];
    const ext = text.match(/#\[dsl\(extension\s*=\s*"([^"]+)"/);
    if (ext) {
      const parts = artifactDir.split("/");
      const pluginId = slug(parts[parts.indexOf("🔌️plugins") + 1]);
      const artifactId = slug(basename(artifactDir));
      const e = ext[1];
      if (e === pluginId || e === artifactId) return `${pluginId}.${artifactId}`;
      return `${pluginId}.${e}`;
    }
  }
  const parts = artifactDir.split("/");
  return `${slug(parts[parts.indexOf("🔌️plugins") + 1])}.${slug(basename(artifactDir))}`;
};

const ensure = (artifactDir) => {
  const envelope = resolveEnvelope(artifactDir);
  const ex = join(artifactDir, "📚️examples/♻️reuse");
  const dslDir = join(ex, "🗣️dsls/♻️reuse");
  const dsl = join(dslDir, `🧬️component.${envelope}.dsl.semio`);
  if (!existsSync(dsl)) {
    mkdirSync(dslDir, { recursive: true });
    writeFileSync(dsl, `semio ${envelope}.dsl v1\n`, "utf8");
  }
  const op = join(ex, "🔧️ops/♻️reuse", `🧬️component.${envelope}.op.semio`);
  if (!existsSync(op)) {
    mkdirSync(dirname(op), { recursive: true });
    writeFileSync(op, `semio ${envelope}.op v1\nedit reuse started="0" actor=example\n`, "utf8");
  }
  const pack = join(ex, "🎒️packs/♻️reuse", `🧬️component.${envelope}.pack.semio`);
  if (!existsSync(pack)) {
    mkdirSync(dirname(pack), { recursive: true });
    writeFileSync(pack, wrapBinary(`${envelope}.pack v1`));
  }
  const spr = join(ex, "📡️sprs/♻️reuse", `🧬️component.${envelope}.spr.semio`);
  if (!existsSync(spr)) {
    mkdirSync(dirname(spr), { recursive: true });
    writeFileSync(spr, wrapBinary(`${envelope}.spr v1`));
  }
};

const walk = (dir) => {
  const artifacts = join(dir, "🗿️artifacts");
  if (!existsSync(artifacts)) return;
  for (const a of readdirSync(artifacts)) {
    const ad = join(artifacts, a);
    if (statSync(ad).isDirectory()) ensure(ad);
  }
};

for (const p of readdirSync(PLUGINS)) {
  if (p.startsWith(".")) continue;
  walk(join(PLUGINS, p));
}
console.log("[ensure-four-kinds] done");
