#!/usr/bin/env bun
import { existsSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const REPO = resolve(import.meta.dir, "../../../../../..");

const APPS = [
  { reactDir: "mathematical/graph/port/directed/dag/react", manifestCore: "mathematical/graph/port/directed/dag/core/package.json", rendererPkg: "@semio-tech/dag-react", rendererExport: "dagAppRenderer" },
  { reactDir: "flow/react", manifestCore: "flow/core/package.json", rendererPkg: "@semio-tech/flow-react", rendererExport: "flowAppRenderer" },
  { reactDir: "procedural/3d/react", manifestCore: "procedural/3d/core/package.json", rendererPkg: "@semio-tech/procedural-3d-react", rendererExport: "proceduralAppRenderer" },
  { reactDir: "puzzle/3d/react", manifestCore: "puzzle/3d/core/package.json", rendererPkg: "@semio-tech/puzzle-3d-react", rendererExport: "puzzle3dAppRenderer" },
];

for (const app of APPS) {
  const reactDir = join(REPO, app.reactDir);
  const playHostPath = join(reactDir, "play-host.tsx");
  const indexPath = join(reactDir, "index.tsx");
  if (!existsSync(playHostPath)) {
    console.log(`skip no play-host ${app.reactDir}`);
    continue;
  }
  let playHost = readFileSync(playHostPath, "utf8");
  let index = readFileSync(indexPath, "utf8");
  playHost = playHost.replace(/^\/\/ #region 🧲️Header[\s\S]*?\/\/ #endregion 🧲️Header\n\n?/m, "");
  playHost = playHost.replace(/from "\.\/index\.tsx";\n/g, "");
  if (index.includes("//#region 🔖️PlayHost")) {
    console.log(`skip already merged ${app.reactDir}`);
    continue;
  }
  const testMatch = index.match(/\n\/\/ #region 🧪️Tests/);
  if (testMatch?.index !== undefined) {
    index = `${index.slice(0, testMatch.index)}\n\n//#region 🔖️PlayHost\n${playHost.trim()}\n//#endregion 🔖️PlayHost\n${index.slice(testMatch.index)}`;
  } else {
    index = `${index.trimEnd()}\n\n//#region 🔖️PlayHost\n${playHost.trim()}\n//#endregion 🔖️PlayHost\n`;
  }
  writeFileSync(indexPath, index);
  unlinkSync(playHostPath);
  const reactPkgPath = join(reactDir, "package.json");
  const reactPkg = JSON.parse(readFileSync(reactPkgPath, "utf8")) as { exports?: Record<string, string> };
  if (reactPkg.exports?.["./play-host"]) {
    delete reactPkg.exports["./play-host"];
    writeFileSync(reactPkgPath, `${JSON.stringify(reactPkg, null, 2)}\n`);
  }
  const corePkgPath = join(REPO, app.manifestCore);
  const corePkg = JSON.parse(readFileSync(corePkgPath, "utf8")) as { semio?: { app?: Record<string, string>; playgroundApp?: Record<string, string> } };
  const semio = corePkg.semio?.app ?? corePkg.semio?.playgroundApp;
  if (semio) {
    semio.rendererPackage = app.rendererPkg;
    semio.rendererExport = app.rendererExport;
    writeFileSync(corePkgPath, `${JSON.stringify(corePkg, null, 2)}\n`);
  }
  console.log(`merged ${app.reactDir}`);
}
