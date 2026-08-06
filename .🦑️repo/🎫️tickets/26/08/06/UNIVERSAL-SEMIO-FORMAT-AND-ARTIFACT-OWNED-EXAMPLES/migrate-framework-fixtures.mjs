/**
 * @emoji 🧫 Migrate framework/os examples + sync store fixtures to `.semio`.
 */
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

const wrapBinary = (token, payload = Buffer.from([0])) => {
  const magic = Buffer.from([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
  const tb = Buffer.from(token, "utf8");
  const len = Buffer.alloc(4);
  len.writeUInt32LE(tb.length, 0);
  return Buffer.concat([magic, len, tb, payload]);
};

const toTextSemio = (envelope, body) => `semio ${envelope} v1\n${body.replace(/^semio[^\n]*\n/, "")}`;

const migrateTextLeaf = (reuseRoot, envelope, comp, body) => {
  const leaf = join(
    reuseRoot,
    comp === "dsl" ? "🗣️dsls" : comp === "op" ? "🔧️ops" : comp === "pack" ? "🎒️packs" : "📡️sprs",
    "♻️reuse",
  );
  mkdirSync(leaf, { recursive: true });
  const name = `🧬️component.${envelope.replace(/\./g, ".")}.${comp}.semio`;
  const path = join(leaf, name);
  if (comp === "pack" || comp === "spr") writeFileSync(path, wrapBinary(`${envelope}.${comp} v1`));
  else if (comp === "op") writeFileSync(path, toTextSemio(`${envelope}.op`, `edit reuse started="0" actor=fixture\n`));
  else writeFileSync(path, toTextSemio(`${envelope}.dsl`, body), "utf8");
  return path;
};

const osDemo = join(REPO, "🧰️framework/🛍️products/💻️os/📚️examples/🎬️demo.workflow-document");
if (existsSync(osDemo)) {
  const body = readFileSync(osDemo, "utf8");
  const root = join(REPO, "🧰️framework/🛍️products/💻️os/📚️examples/♻️reuse");
  migrateTextLeaf(root, "os.workflow", "dsl", body);
  migrateTextLeaf(root, "os.workflow", "op", "");
  migrateTextLeaf(root, "os.workflow", "pack", "");
  migrateTextLeaf(root, "os.workflow", "spr", "");
  rmSync(osDemo, { force: true });
}

const spaceDemo = join(REPO, "✏️s/🔌️plugins/🪐️space/📚️examples/✏️demo.s");
const spaceDemoGit = () => {
  try {
    return Bun.spawnSync(["git", "show", `HEAD:${spaceDemo.replace(REPO + "/", "")}`], { cwd: REPO }).stdout.toString();
  } catch {
    return "";
  }
};
const spaceBody = existsSync(spaceDemo) ? readFileSync(spaceDemo, "utf8") : spaceDemoGit();
if (spaceBody.trim()) {
  const root = join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/📚️examples/♻️reuse");
  migrateTextLeaf(root, "space.studio", "dsl", spaceBody);
  migrateTextLeaf(root, "space.studio", "op", "");
  migrateTextLeaf(root, "space.studio", "pack", "");
  migrateTextLeaf(root, "space.studio", "spr", "");
  if (existsSync(spaceDemo)) rmSync(spaceDemo, { force: true });
  const legacyDir = dirname(spaceDemo);
  if (existsSync(legacyDir) && readdirSync(legacyDir).length === 0) rmSync(legacyDir, { recursive: true, force: true });
}

const flowLegacy = join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📚️examples/🌊️default.flow");
if (existsSync(flowLegacy)) {
  const body = readFileSync(flowLegacy, "utf8");
  const art = join(REPO, "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📚️examples/♻️reuse");
  migrateTextLeaf(art, "flow.flow", "dsl", body);
  rmSync(flowLegacy, { force: true });
}

const dagLegacy = join(
  REPO,
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/⚡️implementations/🦀️rust/📚️examples/🕸️demo.dag",
);
if (existsSync(dagLegacy)) {
  const body = readFileSync(dagLegacy, "utf8");
  const art = join(REPO, "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📚️examples/♻️reuse");
  migrateTextLeaf(art, "dag.dag", "dsl", body);
  rmSync(dagLegacy, { force: true });
}

const syncFixtures = join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚡️implementations/🦀️rust/🧫️fixtures");
for (const caseDir of readdirSync(syncFixtures)) {
  const dir = join(syncFixtures, caseDir);
  if (!existsSync(dir) || !statSync(dir).isDirectory()) continue;
  for (const f of readdirSync(dir)) {
    if (f.endsWith(".dsl")) {
      const body = readFileSync(join(dir, f), "utf8");
      const out = join(dir, f.replace(/\.dsl$/, ".dsl.semio").replace(/^🔣️fixture/, "🧬️component.sync.fixture"));
      if (!out.includes("🧬️")) {
        const semioName = `🧬️component.sync.${caseDir}.${f.replace(/\.dsl$/, "").replace(/^[^a-z]+/i, "")}.dsl.semio`;
        writeFileSync(join(dir, semioName), toTextSemio(`sync.${caseDir}.dsl`, body), "utf8");
      }
    }
    if (f.endsWith(".ops")) {
      const body = readFileSync(join(dir, f), "utf8");
      const semioName = `🧬️component.sync.${caseDir}.${f.replace(/\.ops$/, "").replace(/^[^a-z0-9]+/i, "")}.op.semio`;
      writeFileSync(join(dir, semioName), toTextSemio(`sync.${caseDir}.op`, body), "utf8");
    }
  }
}

console.log("[migrate-framework-fixtures] done");
