/**
 * @emoji 🔗 Point legacy `include_str!(…📚️examples…)` at artifact-owned `.semio` leaves.
 */
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const ROOTS = [join(REPO, "✏️s/🔌️plugins"), join(REPO, "🧰️framework/🛍️products/💻️os"), join(REPO, "✏️s/🔨️modules")];

const norm = (s) => s.replace(/\uFE0F/g, "").replace(/[^a-z0-9]+/gi, "").toLowerCase();

const dslSemioDir = (artifactDir) => join(artifactDir, "📚️examples/♻️reuse/🗣️dsls/♻️reuse");

const pickSemio = (artifactDir, legacyPath) => {
  const dir = dslSemioDir(artifactDir);
  if (!existsSync(dir)) return null;
  const files = readdirSync(dir).filter((n) => n.endsWith(".dsl.semio"));
  if (!files.length) return null;
  const legacyKey = norm(legacyPath.split("/").pop() ?? "");
  const ranked = files
    .map((f) => ({ f, k: norm(f) }))
    .sort((a, b) => {
      const as = legacyKey && a.k.includes(legacyKey) ? 1 : 0;
      const bs = legacyKey && b.k.includes(legacyKey) ? 1 : 0;
      if (as !== bs) return bs - as;
      const aPrimary = /\.component\.[^.]+\.[^.]+\.dsl\.semio$/.test(a.f) ? 1 : 0;
      const bPrimary = /\.component\.[^.]+\.[^.]+\.dsl\.semio$/.test(b.f) ? 1 : 0;
      return bPrimary - aPrimary;
    });
  return join(dir, ranked[0].f);
};

const relInclude = (fromFile, semioAbs) => {
  const rel = relative(join(fromFile, ".."), semioAbs).split("\\").join("/");
  return rel.startsWith(".") ? rel : `./${rel}`;
};

const artifactForFile = (file) => {
  const parts = file.split("/");
  const i = parts.indexOf("🗿️artifacts");
  if (i < 0) return null;
  return parts.slice(0, i + 2).join("/");
};

const patchFile = (file) => {
  if (!/\.(rs|ts)$/.test(file)) return false;
  let text = readFileSync(file, "utf8");
  if (!text.includes("include_str!") || !text.includes("📚️examples")) return false;
  const art = artifactForFile(file);
  const orig = text;
  text = text.replace(/include_str!\("([^"]*)"\)/g, (m, legacyPath) => {
    if (!legacyPath.includes("📚️examples") && !legacyPath.includes(".flow") && !legacyPath.includes(".dag")) return m;
    let semio = art ? pickSemio(art, legacyPath) : null;
    if (!semio && legacyPath.includes("🌊️flow")) {
      semio = pickSemio(join(REPO, "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"), legacyPath);
    }
    if (!semio && legacyPath.includes("🕸️dag")) {
      semio = pickSemio(join(REPO, "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag"), legacyPath);
    }
    if (!semio && legacyPath.includes("✒️writer")) {
      semio = pickSemio(join(REPO, "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer"), legacyPath);
    }
    if (!semio && legacyPath.includes("🖍️draw")) {
      semio = pickSemio(join(REPO, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw"), legacyPath);
    }
    if (!semio) return m;
    return `include_str!("${relInclude(file, semio)}")`;
  });
  if (text !== orig) {
    writeFileSync(file, text);
    return true;
  }
  return false;
};

const walk = (dir, out = []) => {
  if (!existsSync(dir)) return out;
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    if (ent.name.startsWith(".") || ent.name === "node_modules" || ent.name === "target") continue;
    const p = join(dir, ent.name);
    if (ent.isDirectory()) walk(p, out);
    else out.push(p);
  }
  return out;
};

let n = 0;
for (const root of ROOTS) {
  for (const f of walk(root)) {
    if (patchFile(f)) n++;
  }
}
console.log(`[fix-includes] patched ${n} files`);
