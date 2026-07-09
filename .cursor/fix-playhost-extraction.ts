#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

const TARGETS: Array<{ path: string; pkg: string }> = [
  { path: "puzzle/2d/react/index.tsx", pkg: "@semio-tech/puzzle-2d-react" },
  { path: "puzzle/3d/react/index.tsx", pkg: "@semio-tech/puzzle-3d-react" },
  { path: "puzzle/5d/react/index.tsx", pkg: "@semio-tech/puzzle-5d-react" },
  { path: "gis/2d/react/index.tsx", pkg: "@semio-tech/gis-2d-react" },
  { path: "flow/react/index.tsx", pkg: "@semio-tech/flow-react" },
  { path: "mathematical/graph/port/directed/dag/react/index.tsx", pkg: "@semio-tech/dag-react" },
  { path: "imperative/react/index.tsx", pkg: "@semio-tech/imperative-react" },
  { path: "sequence/react/index.tsx", pkg: "@semio-tech/sequence-react" },
  { path: "layout/react/index.tsx", pkg: "@semio-tech/layout-react" },
  { path: "lowpoly/react/index.tsx", pkg: "@semio-tech/lowpoly-react" },
  { path: "trinity/react/index.tsx", pkg: "@semio-tech/trinity-react" },
  { path: "procedural/3d/react/index.tsx", pkg: "@semio-tech/procedural-3d-react" },
  { path: "procedural/2d/react/index.tsx", pkg: "@semio-tech/procedural-2d-react" },
  { path: "shooting/react/index.tsx", pkg: "@semio-tech/shooting-react" },
  { path: "forms/react/index.tsx", pkg: "@semio-tech/forms-react" },
  { path: "raster/react/index.tsx", pkg: "@semio-tech/raster-react" },
  { path: "draw/react/index.tsx", pkg: "@semio-tech/draw-react" },
  { path: "note/react/index.tsx", pkg: "@semio-tech/note-react" },
  { path: "cad/renderer/react/index.tsx", pkg: "@semio-tech/cad-js-renderer-react" },
  { path: "vcs/react/index.tsx", pkg: "@semio-tech/vcs-react" },
  { path: "writer/react/index.tsx", pkg: "@semio-tech/writer-react" },
  { path: "framework/product/presentation/renderer/react/index.tsx", pkg: "@semio-tech/framework-presentation-renderer-react" },
  { path: "s/react/index.tsx", pkg: "@semio-tech/s-react" },
];

const WRITER_IMPORT = `import { createWriterDocument } from "@semio-tech/writer-core";\nimport { WriterCanvas } from "@semio-tech/writer-react";\n`;

const NEEDS_WRITER = new Set(["flow/react/index.tsx", "puzzle/2d/react/index.tsx", "puzzle/5d/react/index.tsx", "sequence/react/index.tsx", "mathematical/graph/port/directed/dag/react/index.tsx", "s/react/index.tsx"]);

function extractPlayHostRegion(content: string): { before: string; region: string; after: string } | null {
  const marker = content.match(/\/\/#region 🔖\w+PlayHost/);
  if (!marker) return null;
  const startIdx = content.lastIndexOf(marker[0]);
  const endMarker = marker[0].replace("//#region", "//#endregion");
  const endIdx = content.indexOf(endMarker, startIdx);
  if (endIdx < 0) return null;
  const endLen = endMarker.length;
  return {
    before: content.slice(0, startIdx),
    region: content.slice(startIdx, endIdx + endLen),
    after: content.slice(endIdx + endLen),
  };
}

function removeSelfImports(region: string, pkg: string): string {
  const importRe = /^import\s+(?:type\s+)?\{([^}]+)\}\s+from\s+["']@semio-tech\/[^"']+["'];?\s*$/gm;
  return region
    .replace(importRe, (full, inner: string) => {
      const fromMatch = full.match(/from\s+["']([^"']+)["']/);
      if (!fromMatch || fromMatch[1] !== pkg) return full;
      return "";
    })
    .replace(/\n{3,}/g, "\n\n");
}

function removeLocalShootingRegister(region: string): string {
  return region.replace(/\nexport function registerUiShootingSurfaceHost[\s\S]*?\n\}\n/, "\n");
}

function fixShootingImport(region: string): string {
  return region.replace(", registerUiShootingSurfaceHost", "");
}

function addWriterImports(region: string): string {
  if (region.includes("WriterCanvas") && !region.includes('from "@semio-tech/writer-react"')) {
    const headerEnd = region.indexOf("\n", region.indexOf("//#region"));
    return region.slice(0, headerEnd + 1) + WRITER_IMPORT + region.slice(headerEnd + 1);
  }
  return region;
}

function fixCadSelfImport(region: string): string {
  return region.replace(/import \{ CadPlayRoot, registerCadPlaySurfaceHosts \} from "@semio-tech\/cad-js-renderer-react";\n/, "");
}

for (const { path, pkg } of TARGETS) {
  const fullPath = join(REPO, path);
  let content = readFileSync(fullPath, "utf8");
  const parts = extractPlayHostRegion(content);
  if (!parts) {
    console.log(`SKIP no PlayHost: ${path}`);
    continue;
  }
  let region = parts.region;
  region = removeSelfImports(region, pkg);
  if (path === "shooting/react/index.tsx") {
    region = removeLocalShootingRegister(region);
    region = fixShootingImport(region);
  }
  if (path === "cad/renderer/react/index.tsx") {
    region = fixCadSelfImport(region);
  }
  if (NEEDS_WRITER.has(path)) {
    region = addWriterImports(region);
  }
  writeFileSync(fullPath, parts.before + region + parts.after);
  console.log(`Fixed ${path}`);
}

// Add registerUiShootingSurfaceHost to shell
const shellPath = join(REPO, "framework/product/playground/renderer/react/index.tsx");
let shell = readFileSync(shellPath, "utf8");
if (!shell.includes("registerUiShootingSurfaceHost")) {
  shell = shell.replace(
    `/** @emoji 🔺 Binds \`surfaceId\` from {@link UiTrinityHostSurfaceNode} to a trinity canvas. */
export function registerUiTrinitySurfaceHost(surfaceId: string, Component: TrinitySurfaceHost): void {
  trinitySurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📊 Binds`,
    `/** @emoji 🔺 Binds \`surfaceId\` from {@link UiTrinityHostSurfaceNode} to a trinity canvas. */
export function registerUiTrinitySurfaceHost(surfaceId: string, Component: TrinitySurfaceHost): void {
  trinitySurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📸 Binds \`surfaceId\` from {@link UiShootingHostSurfaceNode} to a shooting canvas. */
export function registerUiShootingSurfaceHost(surfaceId: string, Component: ShootingSurfaceHost): void {
  shootingSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📊 Binds`,
  );
  writeFileSync(shellPath, shell);
  console.log("Added registerUiShootingSurfaceHost to shell");
}

// Fix playground renderer test
shell = readFileSync(shellPath, "utf8");
shell = shell.replace(
  `  describe("playground renderer slices", () => {
    it("keeps cross-dimensional brush host imports with their consumers", async () => {
      const { readFileSync } = await import("node:fs");
      const source = readFileSync("index.tsx", "utf8");
      const hostRegion = (kind: "2d" | "5d") => {
        const start = \`//#region 🔖Puzzle\${kind}PlayHost\`;
        return source.slice(source.indexOf(start), source.indexOf(\`//#endregion 🔖Puzzle\${kind}PlayHost\`));
      };
      const puzzle2d = hostRegion("2d");
      const puzzle5d = hostRegion("5d");
      expect(puzzle2d).toMatch(
        /import\\s*\\{[^}]*puzzle2dSetBrushPlaceCommitHandler[^}]*\\}\\s*from\\s*["']@semio-tech\\/puzzle-2d-react["']/,
      );
    });
  });`,
  `  describe("playground renderer slices", () => {
    it("keeps cross-dimensional brush host imports with their consumers", async () => {
      const { readFileSync } = await import("node:fs");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const rendererDir = dirname(fileURLToPath(import.meta.url));
      const puzzle2dSource = readFileSync(join(rendererDir, "../../../../puzzle/2d/react/index.tsx"), "utf8");
      const start = "//#region 🔖Puzzle2dPlayHost";
      const puzzle2d = puzzle2dSource.slice(puzzle2dSource.indexOf(start), puzzle2dSource.indexOf("//#endregion 🔖Puzzle2dPlayHost"));
      expect(puzzle2d).toMatch(
        /import\\s*\\{[^}]*puzzle2dSetBrushPlaceCommitHandler[^}]*\\}\\s*from\\s*["']@semio-tech\\/puzzle-2d-react["']/,
      );
    });
  });`,
);
writeFileSync(shellPath, shell);
console.log("Fixed playground renderer test");

// Add framework-core to renderer package.json
const rendererPkgPath = join(REPO, "framework/product/playground/renderer/react/package.json");
const rendererPkg = JSON.parse(readFileSync(rendererPkgPath, "utf8"));
rendererPkg.dependencies["@semio-tech/framework-core"] = "workspace:*";
writeFileSync(rendererPkgPath, JSON.stringify(rendererPkg, null, 2) + "\n");

console.log("DONE fixes");
