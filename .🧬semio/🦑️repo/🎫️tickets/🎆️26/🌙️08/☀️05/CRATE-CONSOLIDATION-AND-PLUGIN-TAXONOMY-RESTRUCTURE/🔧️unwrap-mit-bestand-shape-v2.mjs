#!/usr/bin/env bun
/** 🔧 Unwrap ♻️mit-bestand singular ⚡️implementation sandwiches → Shape V2 (scratch driver). */
import { existsSync, mkdirSync, readdirSync, renameSync, rmSync, readFileSync, writeFileSync, statSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const ticketDir = import.meta.dir;
const logPath = join(ticketDir, "🧪️unwrap-mit-bestand-shape-v2-log.json");

const filesChanged = [];

function logChange(kind, from, to) {
  filesChanged.push({ kind, from: relative(repoRoot, from), to: to ? relative(repoRoot, to) : undefined });
}

function ensureDir(p) {
  if (!existsSync(p)) mkdirSync(p, { recursive: true });
}

function moveEntry(from, to) {
  if (!existsSync(from)) throw new Error(`missing: ${from}`);
  ensureDir(dirname(to));
  if (existsSync(to)) throw new Error(`collision: ${to}`);
  renameSync(from, to);
  logChange("move", from, to);
}

function writeText(path, content) {
  const prior = existsSync(path) ? readFileSync(path, "utf8") : null;
  if (prior === content) return;
  ensureDir(dirname(path));
  writeFileSync(path, content);
  logChange(prior == null ? "create" : "update", path);
}

function removeTree(path) {
  if (!existsSync(path)) return;
  rmSync(path, { recursive: true, force: true });
  logChange("delete", path);
}

function removeEmptyParents(start, stop) {
  let cur = start;
  while (cur.startsWith(stop) && cur !== stop) {
    try {
      if (!existsSync(cur)) break;
      if (readdirSync(cur).length > 0) break;
      rmSync(cur);
      logChange("delete", cur);
    } catch {
      break;
    }
    cur = dirname(cur);
  }
}

function migrateBericht() {
  const owner = join(repoRoot, "♻️mit-bestand/📋️bericht");
  const oldTs = join(owner, "⚡️implementation/🟦️typescript");
  const pkg = join(owner, "📦️packages/🟦️typescript");
  ensureDir(pkg);

  moveEntry(join(oldTs, "📋️zwischenbericht"), join(owner, "📋️zwischenbericht"));

  for (const name of ["package.json", "📋️project.json", "📜️script.ts"]) {
    moveEntry(join(oldTs, name), join(pkg, name));
  }

  const scriptPath = join(pkg, "📜️script.ts");
  let script = readFileSync(scriptPath, "utf8");
  script = script.replace(
    'import { buildPrintDocument, fetchPrintFonts } from "../../../../🧰️framework/🛍️products/📓️print/⚡️implementations/🟦️typescript/📜️script.ts";',
    'import { buildPrintDocument, fetchPrintFonts } from "../../../../🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📜️script.ts";',
  );
  script = script.replace(
    'const berichtRoot = import.meta.dir;\nconst defaultTex = join(berichtRoot, "📋️zwischenbericht/zwischenbericht.tex");',
    'const packageRoot = import.meta.dir;\nconst ownerRoot = join(packageRoot, "../..");\nconst defaultTex = join(ownerRoot, "📋️zwischenbericht/zwischenbericht.tex");',
  );
  script = script.replaceAll("berichtRoot", "packageRoot");
  script = script.replace(
    'join(packageRoot, "../../../../🧰️framework/🛍️products/📓️print/⚡️implementations/🖋️latex")',
    'join(packageRoot, "../../../../🧰️framework/🛍️products/📓️print/🖋️latex")',
  );
  writeText(scriptPath, script);

  const pkgJsonPath = join(pkg, "package.json");
  let pkgJson = readFileSync(pkgJsonPath, "utf8");
  pkgJson = pkgJson.replace('"$schema": "../../../node_modules/nx/schemas/project-schema.json"', '"$schema": "../../../../node_modules/nx/schemas/project-schema.json"');
  writeText(pkgJsonPath, pkgJson);

  const projPath = join(pkg, "📋️project.json");
  let proj = readFileSync(projPath, "utf8");
  proj = proj.replaceAll("♻️mit-bestand/📋️bericht/⚡️implementation/🟦️typescript", "♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript");
  proj = proj.replace('"$schema": "../../../../node_modules/nx/schemas/project-schema.json"', '"$schema": "../../../../../node_modules/nx/schemas/project-schema.json"');
  writeText(projPath, proj);

  removeTree(join(owner, "⚡️implementation"));
}

function migrateProjektetage() {
  const owner = join(repoRoot, "♻️mit-bestand/🎤️präsentation/📅️33.projektetage");
  const oldTs = join(owner, "⚡️implementation/🟦️typescript");
  const pkg = join(owner, "📦️packages/🟦️typescript");
  ensureDir(pkg);

  for (const name of ["slide", "🌐️public", "🎨️globals.css"]) {
    moveEntry(join(oldTs, name), join(owner, name));
  }

  for (const name of ["package.json", "📋️project.json", "📜️script.ts", "📦️index.ts", "⚙️vite.config.ts", "🧪️vitest.config.ts", "🌐️index.html"]) {
    if (existsSync(join(oldTs, name))) moveEntry(join(oldTs, name), join(pkg, name));
  }

  const indexPath = join(pkg, "📦️index.ts");
  let index = readFileSync(indexPath, "utf8");
  index = index.replace('import "../🎨️globals.css";', 'import "../../🎨️globals.css";');
  index = index.replace(
    'const slideModuleLoaders = import.meta.glob<{ default: SlideFile }>("../slide/**/*.ts");',
    'const slideModuleLoaders = import.meta.glob<{ default: SlideFile }>("../../slide/**/*.ts");',
  );
  writeText(indexPath, index);

  const vitePath = join(pkg, "⚙️vite.config.ts");
  let vite = readFileSync(vitePath, "utf8");
  vite = vite.replace('publicDir: resolve(bundleRoot, "public"),', 'publicDir: resolve(bundleRoot, "../../🌐️public"),');
  vite = vite.replace(
    'replacement: resolve(repoRoot, "./✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/📦️index.ts")',
    'replacement: resolve(repoRoot, "./✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/📺️renderer/⚛️react/🟦️component.tsx")',
  );
  writeText(vitePath, vite);

  const vitestPath = join(pkg, "🧪️vitest.config.ts");
  let vitest = readFileSync(vitestPath, "utf8");
  vitest = vitest.replace(
    'replacement: resolve(repoRoot, "./✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/📦️index.ts")',
    'replacement: resolve(repoRoot, "./✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/📺️renderer/⚛️react/🟦️component.tsx")',
  );
  writeText(vitestPath, vitest);

  const scriptPath = join(pkg, "📜️script.ts");
  let script = readFileSync(scriptPath, "utf8");
  writeText(scriptPath, script);

  const pkgJsonPath = join(pkg, "package.json");
  let pkgJson = readFileSync(pkgJsonPath, "utf8");
  pkgJson = pkgJson.replace('"$schema": "../../../../node_modules/nx/schemas/project-schema.json"', '"$schema": "../../../../../node_modules/nx/schemas/project-schema.json"');
  writeText(pkgJsonPath, pkgJson);

  const projPath = join(pkg, "📋️project.json");
  let proj = readFileSync(projPath, "utf8");
  proj = proj.replaceAll("♻️mit-bestand/🎤️präsentation/📅️33.projektetage/⚡️implementation/🟦️typescript", "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript");
  proj = proj.replace('"$schema": "../../../../../node_modules/nx/schemas/project-schema.json"', '"$schema": "../../../../../../node_modules/nx/schemas/project-schema.json"');
  if (!proj.includes("namedInputs")) {
    proj = proj.replace(
      '"targets": {',
      '"namedInputs": {\n    "default": [\n      "{projectRoot}/**/*",\n      "{workspaceRoot}/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/slide/**/*.ts",\n      "{workspaceRoot}/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/**/*"\n    ]\n  },\n  "targets": {',
    );
  }
  writeText(projPath, proj);

  removeTree(join(oldTs, "dist"));
  removeTree(join(owner, "⚡️implementation"));
}

function patchRootPackageJson() {
  const path = join(repoRoot, "package.json");
  let text = readFileSync(path, "utf8");
  const replacements = [
    ["♻️mit-bestand/🎤️präsentation/📅️33.projektetage/⚡️implementation/🟦️typescript", "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript"],
    ["♻️mit-bestand/📋️bericht/⚡️implementation/🟦️typescript", "♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript"],
  ];
  for (const [from, to] of replacements) {
    if (text.includes(from)) text = text.replace(from, to);
  }
  writeText(path, text);
}

function findRemainingImplDirs() {
  const mit = join(repoRoot, "♻️mit-bestand");
  const hits = [];
  function walk(dir) {
    for (const ent of readdirSync(dir, { withFileTypes: true })) {
      const p = join(dir, ent.name);
      if (ent.isDirectory()) {
        if (ent.name === "⚡️implementation" || ent.name === "⚡️implementations") hits.push(relative(repoRoot, p));
        else walk(p);
      }
    }
  }
  walk(mit);
  return hits.sort();
}

migrateBericht();
migrateProjektetage();
patchRootPackageJson();

const remainingImplDirs = findRemainingImplDirs();
const result = {
  status: remainingImplDirs.length === 0 ? "ok" : "partial",
  remainingImplDirs,
  filesChanged: filesChanged.map((e) => e.to ?? e.from),
};
writeFileSync(logPath, `${JSON.stringify(result, null, 2)}\n`);
console.log(JSON.stringify(result));
