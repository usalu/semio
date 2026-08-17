import { readFileSync, writeFileSync, mkdirSync, rmSync, readdirSync } from "fs";
import { join, dirname } from "path";
import { execSync } from "child_process";

const coreDir = execSync(
  `find ✏️s/🔌️plugins/📐️cad/🔨️modules -maxdepth 1 -type d -name '*core' | head -1`,
  { encoding: "utf8" },
).trim();
if (!coreDir) throw new Error("CAD core dir not found");

const srcName = readdirSync(coreDir).find((n) => n.endsWith(".ts"));
if (!srcName) throw new Error("CAD core component not found");
const srcFile = join(coreDir, srcName);
const modulesDir = dirname(coreDir);
const lines = readFileSync(srcFile, "utf8").split("\n");
const header = lines.slice(0, 9).join("\n");
const coreBase = coreDir.split("/").pop();

const groups = {
  "📔️registry": [10, 173],
  "📐️geometry": [174, 3551],
  "🧬️typology": [3552, 3701],
  "🗺️spatial": [3702, 3941],
  "🎬️actions": [3942, 5766],
  "📄️document": [5767, 6740],
  "🧪tests": [6741, lines.length],
};

function slice1(start, end) {
  return lines.slice(start - 1, end).join("\n");
}

const written = [];
for (const [folder, [start, end]] of Object.entries(groups)) {
  const dir = join(modulesDir, folder);
  mkdirSync(dir, { recursive: true });
  const body = slice1(start, end);
  const content = `${header}\n\n// #region 📦️${folder}\n${body}\n// #endregion 📦️${folder}\n`;
  writeFileSync(join(dir, srcName), content);
  written.push({ folder, lines: content.split("\n").length });
}

const indexName = "🟦️index.ts";
const barrel = [
  "// #region 🧲️Header",
  "/** @emoji 🧭️ CAD modules barrel — former core split into concept folders. */",
  "// #endregion 🧲️Header",
  "",
  `export * from "./📔️registry/${srcName}";`,
  `export * from "./📐️geometry/${srcName}";`,
  `export * from "./🧬️typology/${srcName}";`,
  `export * from "./🗺️spatial/${srcName}";`,
  `export * from "./🎬️actions/${srcName}";`,
  `export * from "./📄️document/${srcName}";`,
  "",
].join("\n");
writeFileSync(join(modulesDir, indexName), barrel);

const consumers = execSync(
  `rg -l --fixed-strings ${JSON.stringify(coreBase)} ✏️s/🔌️plugins/📐️cad -g '!node_modules/**' -g '!target/**' || true`,
  { encoding: "utf8" },
)
  .trim()
  .split("\n")
  .filter(Boolean);

let updated = 0;
for (const f of consumers) {
  if (f === srcFile || f.startsWith(coreDir + "/")) continue;
  let t = readFileSync(f, "utf8");
  const o = t;
  t = t.replaceAll(`../${coreBase}/${srcName}`, `../${indexName}`);
  t = t.replaceAll(`../../🔨️modules/${coreBase}/${srcName}`, `../../🔨️modules/${indexName}`);
  t = t.replaceAll(`🔨️modules/${coreBase}/${srcName}`, `🔨️modules/${indexName}`);
  if (t !== o) {
    writeFileSync(f, t);
    updated++;
    console.log("updated", f);
  }
}

rmSync(coreDir, { recursive: true, force: true });
console.log(JSON.stringify({ coreDir, written, updated, coreBase }, null, 2));
