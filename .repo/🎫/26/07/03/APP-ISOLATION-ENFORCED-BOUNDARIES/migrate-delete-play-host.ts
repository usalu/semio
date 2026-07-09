#!/usr/bin/env bun
import { existsSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { existsSync, readFileSync, readdirSync, statSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const REPO = resolve(import.meta.dir, "../../../../../..");

function scanManifests(): Array<{ kind: string; corePackageJsonPath: string }> {
  const entries: Array<{ kind: string; corePackageJsonPath: string }> = [];
  const skip = new Set(["node_modules", ".git", ".nx", "dist", "target", "storybook-static", ".repo-cache"]);
  const rootPkg = resolve(REPO, "package.json");
  const walk = (dir: string): void => {
    const pkgPath = join(dir, "package.json");
    if (existsSync(pkgPath) && pkgPath !== rootPkg) {
      try {
        const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as { semio?: { app?: { kind?: string }; playgroundApp?: { kind?: string } } };
        const manifest = pkg.semio?.app ?? pkg.semio?.playgroundApp;
        if (manifest?.kind) entries.push({ kind: manifest.kind, corePackageJsonPath: pkgPath });
      } catch {
        /* ignore */
      }
    }
    for (const entry of readdirSync(dir)) {
      if (skip.has(entry)) continue;
      const full = join(dir, entry);
      if (statSync(full).isDirectory()) walk(full);
    }
  };
  walk(REPO);
  return entries;
}

const RENDERER_BY_KIND: Record<string, { rendererPackage: string; rendererExport: string }> = {
  draw: { rendererPackage: "@semio-tech/draw-react", rendererExport: "drawAppRenderer" },
  note: { rendererPackage: "@semio-tech/note-react", rendererExport: "noteAppRenderer" },
  writer: { rendererPackage: "@semio-tech/writer-react", rendererExport: "writerAppRenderer" },
  forms: { rendererPackage: "@semio-tech/forms-react", rendererExport: "formsAppRenderer" },
  raster: { rendererPackage: "@semio-tech/raster-react", rendererExport: "rasterAppRenderer" },
  flow: { rendererPackage: "@semio-tech/flow-react", rendererExport: "flowAppRenderer" },
  "gis-2d": { rendererPackage: "@semio-tech/gis-2d-react", rendererExport: "mapAppRenderer" },
  "procedural-2d": { rendererPackage: "@semio-tech/procedural-2d-react", rendererExport: "procedural2dAppRenderer" },
  "procedural-3d": { rendererPackage: "@semio-tech/procedural-3d-react", rendererExport: "proceduralAppRenderer" },
  shooting: { rendererPackage: "@semio-tech/shooting-react", rendererExport: "shootingAppRenderer" },
  "trinity-rewrite": { rendererPackage: "@semio-tech/trinity-react", rendererExport: "trinityRewriteAppRenderer" },
  "trinity-jack": { rendererPackage: "@semio-tech/trinity-react", rendererExport: "trinityJackAppRenderer" },
  "puzzle-2d": { rendererPackage: "@semio-tech/puzzle-2d-react", rendererExport: "puzzle2dAppRenderer" },
  "puzzle-3d": { rendererPackage: "@semio-tech/puzzle-3d-react", rendererExport: "puzzle3dAppRenderer" },
  "puzzle-5d": { rendererPackage: "@semio-tech/puzzle-5d-react", rendererExport: "puzzle5dAppRenderer" },
  presentation: { rendererPackage: "@semio-tech/framework-presentation-renderer-react", rendererExport: "presentationAppRenderer" },
  sequence: { rendererPackage: "@semio-tech/sequence-react", rendererExport: "sequenceAppRenderer" },
  layout: { rendererPackage: "@semio-tech/layout-react", rendererExport: "layoutAppRenderer" },
  imperative: { rendererPackage: "@semio-tech/imperative-react", rendererExport: "imperativeAppRenderer" },
  lowpoly: { rendererPackage: "@semio-tech/lowpoly-react", rendererExport: "lowpolyAppRenderer" },
  vcs: { rendererPackage: "@semio-tech/vcs-react", rendererExport: "vcsAppRenderer" },
  cad: { rendererPackage: "@semio-tech/cad-js-renderer-react", rendererExport: "cadAppRenderer" },
  s: { rendererPackage: "@semio-tech/s-react", rendererExport: "sAppRenderer" },
  dag: { rendererPackage: "@semio-tech/dag-react", rendererExport: "dagAppRenderer" },
  wires: { rendererPackage: "@semio-tech/puzzle-2d-react", rendererExport: "wiresAppRenderer" },
  sketchpad: { rendererPackage: "@semio-tech/compose-sketchpad", rendererExport: "sketchpadAppRenderer" },
};

function updateCoreManifests(): void {
  for (const manifest of scanManifests()) {
    const renderer = RENDERER_BY_KIND[manifest.kind];
    if (!renderer) {
      console.log(`SKIP manifest ${manifest.kind} — no renderer mapping`);
      continue;
    }
    const pkgPath = manifest.corePackageJsonPath;
    const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as {
      semio?: { app?: Record<string, unknown>; playgroundApp?: Record<string, unknown> };
    };
    const key = pkg.semio?.app ? "app" : "playgroundApp";
    const block = pkg.semio?.[key];
    if (!block) continue;
    block.rendererPackage = renderer.rendererPackage;
    block.rendererExport = renderer.rendererExport;
    writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
    console.log(`manifest ${manifest.kind}`);
  }
}

function stripLoadRenderer(content: string): string {
  return content.replace(/\n\tloadRenderer: async \(\) => \(await import\([^)]+\)\)\.\w+,/g, "").replace(/\n\tloadRenderer: async \(\) => \w+AppRenderer,/g, "");
}

function convertCreateRuntimeToBootstrap(content: string, buildFn: string): string {
  const pattern = new RegExp(`createRuntime: \\(\\) => \\{[\\s\\S]*?runtime\\.addApp\\(${buildFn}\\([^)]+\\)\\);[\\s\\S]*?return runtime;\\s*\\},`, "m");
  const match = content.match(
    /createRuntime: \(\) => \{[\s\S]*?const runtime = createProductPlaygroundPlatform\([^)]+\);[\s\S]*?const ctrl = (new \w+\([^)]+\));[\s\S]*?ctrl\.run\("setActiveExample"[\s\S]*?runtime\.addApp\((\w+)\(ctrl\)\);[\s\S]*?return runtime;\s*\},/,
  );
  if (!match) return content;
  const createController = match[1]!;
  const buildAppRuntime = match[2]!;
  const exampleMatch = content.match(/const (\w+): \w+ = \{[\s\S]*?defaultId:[\s\S]*?\};/);
  const exampleVar = exampleMatch?.[1];
  let runtimeBootstrap = `\truntimeBootstrap: {
\t\tcreateController: (bus, notify) => ${createController.replace("runtime.commandBus", "bus").replace("() => runtime.notify()", "notify")},
\t\tbuildAppRuntime: ${buildAppRuntime},`;
  if (exampleVar) {
    runtimeBootstrap += `
\t\texample: { defaultId: ${exampleVar}.defaultId, hasExample: (id) => Boolean(${exampleVar}.fileJsonById[id]) },`;
  }
  runtimeBootstrap += `\n\t},`;
  return content.replace(pattern, runtimeBootstrap);
}

function mergePlayHost(reactDir: string): void {
  const playHostPath = join(reactDir, "play-host.tsx");
  const indexPath = join(reactDir, "index.tsx");
  if (!existsSync(playHostPath)) return;
  let playHost = readFileSync(playHostPath, "utf8");
  let index = readFileSync(indexPath, "utf8");
  playHost = playHost.replace(/^\/\/ #region 🧲Header[\s\S]*?\/\/ #endregion 🧲Header\n\n?/m, "");
  playHost = playHost.replace(/import \{ DrawCanvas \} from "\.\/index\.tsx";\n/g, "");
  playHost = playHost.replace(/from "\.\/index\.tsx";\n/g, "");
  playHost = playHost.replace(/\/\/#region 🔖\w+PlayHost\n/g, "");
  playHost = playHost.replace(/\/\/#endregion 🔖\w+PlayHost\n?/g, "");
  if (!index.includes("//#region 🔖PlayHost")) {
    index = `${index.trimEnd()}\n\n//#region 🔖PlayHost\n${playHost.trim()}\n//#endregion 🔖PlayHost\n`;
  }
  writeFileSync(indexPath, index);
  unlinkSync(playHostPath);
  const pkgPath = join(reactDir, "package.json");
  if (existsSync(pkgPath)) {
    const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as { exports?: Record<string, string> };
    if (pkg.exports?.["./play"]) {
      delete pkg.exports["./play"];
      writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
    }
  }
  console.log(`merged ${reactDir}`);
}

function updateCoreFiles(): void {
  for (const manifest of scanManifests()) {
    const coreEntry = join(dirname(manifest.corePackageJsonPath), "js/index.ts");
    const altEntry = join(dirname(manifest.corePackageJsonPath), "index.ts");
    const corePath = existsSync(coreEntry) ? coreEntry : altEntry;
    if (!existsSync(corePath)) continue;
    let content = readFileSync(corePath, "utf8");
    const before = content;
    content = stripLoadRenderer(content);
    const buildMatch = content.match(/runtime\.addApp\((build\w+PlayAppRuntime)\(/);
    if (buildMatch) {
      content = convertCreateRuntimeToBootstrap(content, buildMatch[1]!);
    }
    if (content !== before) {
      writeFileSync(corePath, content);
      console.log(`core ${manifest.kind}`);
    }
  }
}

const PLAY_HOST_DIRS = [
  "draw/react",
  "note/react",
  "writer/react",
  "forms/react",
  "raster/react",
  "flow/react",
  "gis/2d/react",
  "procedural/2d/react",
  "procedural/3d/react",
  "shooting/react",
  "trinity/react",
  "puzzle/2d/react",
  "puzzle/3d/react",
  "puzzle/5d/react",
  "framework/product/presentation/renderer/react",
  "sequence/react",
  "layout/react",
  "imperative/react",
  "lowpoly/react",
  "vcs/react",
  "cad/renderer/react",
  "s/react",
  "mathematical/graph/port/directed/dag/react",
];

updateCoreManifests();
updateCoreFiles();
for (const dir of PLAY_HOST_DIRS) {
  mergePlayHost(join(REPO, dir));
}

console.log("DONE migrate-delete-play-host");
