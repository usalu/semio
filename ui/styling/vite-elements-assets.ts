// #region 🧲Header
/** @emoji 🌐 Vite plugin: serve and copy `ui/asset` at `/asset/*` (fonts, cursors, …). */
// #endregion 🧲Header

// #region 🔌Adapters
import mdx from "@mdx-js/rollup";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import { createServer, type Server } from "node:http";
import { cpSync, createReadStream, existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Connect, Plugin } from "vite";
import { defineConfig, type UserConfig } from "vite";
import {
  PLAYGROUND_LOCKED_EXAMPLE_ENV,
  PLAYGROUND_PORTS,
  PLAYGROUND_SITE_DEV_PORTS,
  PLAYGROUND_SITE_HOSTS,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  playgroundDevPortString,
  playgroundEmbedUrl,
  playgroundLockedExampleIdFromEnv,
  playgroundPlayViteDefine,
  playgroundPortEnv,
  playgroundTestPort,
  playgroundTestPortString,
  type PlaygroundHostKind,
  type PlaygroundSiteKind,
} from "../../repo/lib/js/index.ts";
// #endregion 🔌Adapters

export {
  PLAYGROUND_PORTS,
  PLAYGROUND_SITE_DEV_PORTS,
  PLAYGROUND_SITE_HOSTS,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  playgroundDevPortString,
  playgroundEmbedUrl,
  playgroundPortEnv,
  playgroundTestPort,
  playgroundTestPortString,
  type PlaygroundHostKind,
  type PlaygroundSiteKind,
};

//#region 🔖ViteElementsAssets
/** @emoji 📦 Relative-base Vite build defaults for playground static sites (iframe + subdomain safe). */
export function playgroundStaticSiteBuildOptions(overrides?: UserConfig["build"]): NonNullable<UserConfig["build"]> {
  return {
    target: "esnext",
    outDir: "dist",
    emptyOutDir: true,
    ...overrides,
  };
}

/** @emoji 🔗 True when a request targets Vite prebundled `node_modules/.vite/deps` chunks. */
export function isPlaygroundOptimizedDepUrl(url: string): boolean {
  return url.includes("/node_modules/.vite/deps/");
}

/** @emoji 🧱 Stubs vitest and testing-library when test regions enter the browser graph. */
export function playgroundVitestDevStubPlugin(): Plugin {
  const vitestStubId = "\0playground-vitest-dev-stub";
  const testingLibraryStubId = "\0playground-testing-library-dev-stub";
  return {
    name: "playground-vitest-dev-stub",
    enforce: "pre",
    resolveId(id) {
      if (id === "vitest" || id.startsWith("vitest/") || id.startsWith("@vitest/")) return vitestStubId;
      if (id === "@testing-library/react" || id.startsWith("@testing-library/")) return testingLibraryStubId;
      return undefined;
    },
    load(id) {
      if (id === vitestStubId) {
        return "export default {}; export const describe = () => {}; export const it = () => {}; export const expect = () => ({ toBe: () => {}, toEqual: () => {} }); export const vi = { fn: () => {}, mock: () => {}, spyOn: () => {} };";
      }
      if (id === testingLibraryStubId) {
        return "export default {}; export const render = () => ({}); export const screen = {}; export const fireEvent = {}; export const waitFor = async (fn) => fn();";
      }
    },
  };
}

const PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID = "\0playground-playwright-dev-stub";

const PLAYGROUND_COMPOSE_SKETCHPAD_STUB_ID = "\0playground-compose-sketchpad-stub";
const PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID = "\0playground-compose-sketchpad-mdx-stub";

const PLAYGROUND_WASM_STUB_PREFIX = "\0playground-wasm-stub/";

function playgroundWasmStubKey(cleanId: string): string {
  return cleanId.replace(/\//g, "__");
}

function playgroundWasmStubKeyDecode(key: string): string {
  return key.replace(/__/g, "/");
}

const PLAYGROUND_WASM_JS_STUB = `const wasmMissing = () => { throw new Error("wasm pkg not built — run the matching nx wasm target"); };
const wasmJson = () => "{}";
const dagLodScaleJson = () => ${JSON.stringify(JSON.stringify([
  { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; fill only.", maxZoom: 0.4 },
  { id: "overview", name: "Overview", description: "Node icons only.", maxZoom: 0.6 },
  { id: "compact", name: "Compact", description: "Horizontal abbreviations.", maxZoom: 0.8 },
  { id: "normal", name: "Normal", description: "Vertical names with sections; channel abbreviations on ports.", maxZoom: 1.5 },
  { id: "detail", name: "Detail", description: "Channel names on ports, port handles, and control text.", maxZoom: 2.75 },
  { id: "micro", name: "Micro", description: "Full channel names on ports and maximum node fidelity.", maxZoom: Number.MAX_VALUE },
]))};
export default async function initWasm() {}
export const initSync = () => {};
export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } }
export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } cameraJson() { return '{"x":0,"y":0,"zoom":1}'; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }
export class EditorSession { syncFromSceneJson() {} setText() {} text() { return ''; } caret() { return 0; } anchor() { return 0; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScrollScreen() {} insertText() {} backspace() {} deleteForward() {} selectAll() {} replaceSelection() {} selectionText() { return ''; } hoverTokenRangeJson() { return 'null'; } setHoverRange() {} cameraJson() { return '{}'; } }
export class DagSession { lodScaleJson() { return dagLodScaleJson(); } }
export class BoardSession { lodScaleJson() { return dagLodScaleJson(); } }
export class WriterSession {}
export class ImperativeSession {}
export class SequenceSession {}
export class RasterSession {}
export class MapSession {}
export class Puzzle3dPrecomputeSession {}
export class TrinitySession {}
export class JackLspSession {}
export const render_drawing_scene = wasmMissing;
export const export_drawing_svg = wasmMissing;
export const export_drawing_pdf = wasmMissing;
export const dispose_drawing = () => {};
export const trace_drawing_bitmap = wasmMissing;
export const boolean_drawing_segments = wasmMissing;
export const tessellate = async () => JSON.stringify({ positions: [], normals: [], index: [], edges: [], points: [], faceGroups: [] });
export const dispose = () => {};
export const evaluate = wasmMissing;
export const ruleQueryJson = wasmJson;
export const boardComputeEdgeBezier = wasmJson;
export const boardHandlePositionCircle = wasmJson;
export const boardHandlePositionRectangle = wasmJson;
export const boardRedrawHandlesFixtureJson = wasmJson;
export const boardRedrawLayoutFixtureJson = wasmJson;
`;

/** @emoji 🧱 Stubs missing wasm pkg imports until `nx run …:wasm` artifacts exist. */
export function playgroundFlowWasmDevStubPlugin(repoRoot: string): Plugin {
  return {
    name: "playground-flow-wasm-dev-stub",
    enforce: "pre",
    resolveId(id, importer) {
      if (!importer || id.startsWith(PLAYGROUND_WASM_STUB_PREFIX)) return undefined;
      const cleanId = id.split("?", 1)[0] ?? id;
      const isWasmPkg =
        cleanId.includes("/pkg/") ||
        cleanId.endsWith(".wasm") ||
        cleanId === "@semio-tech/flow-core/pkg/flow_core.js";
      if (!isWasmPkg) return undefined;
      const abs = cleanId.startsWith(".")
        ? resolve(dirname(importer), cleanId)
        : cleanId.startsWith("@semio-tech/flow-core/pkg/")
          ? resolve(repoRoot, "flow/core/rs/pkg", cleanId.slice("@semio-tech/flow-core/pkg/".length))
          : cleanId.startsWith("@semio-tech/framework-graph-rs/pkg/")
            ? resolve(repoRoot, "framework/graph/rs/pkg", cleanId.slice("@semio-tech/framework-graph-rs/pkg/".length))
            : cleanId.startsWith("@semio-tech/framework-editor-rs/pkg/")
              ? resolve(repoRoot, "framework/editor/rs/pkg", cleanId.slice("@semio-tech/framework-editor-rs/pkg/".length))
              : resolve(repoRoot, cleanId.replace(/^@semio-tech\/[^/]+\//, ""));
      if (existsSync(abs)) return undefined;
      return `${PLAYGROUND_WASM_STUB_PREFIX}${playgroundWasmStubKey(cleanId)}`;
    },
    load(id) {
      if (!id.startsWith(PLAYGROUND_WASM_STUB_PREFIX)) return undefined;
      const cleanId = playgroundWasmStubKeyDecode(id.slice(PLAYGROUND_WASM_STUB_PREFIX.length).split("?", 1)[0] ?? "");
      if (cleanId.endsWith(".wasm")) return `export default "";`;
      return PLAYGROUND_WASM_JS_STUB;
    },
  };
}

/** @emoji 🧱 Stubs compose-sketchpad when the monolithic play renderer is bundled outside the s playground. */
export function playgroundComposeSketchpadStubPlugin(repoRoot: string): Plugin {
  const sketchpadRoot = resolve(repoRoot, "compose/client/lib/sketchpad/js");
  const sketchpadIndex = resolve(sketchpadRoot, "index.ts");
  return {
    name: "playground-compose-sketchpad-stub",
    enforce: "pre",
    resolveId(id) {
      const cleanId = id.split("?", 1)[0] ?? id;
      if (cleanId.endsWith(".mdx") && cleanId.includes("sketchpad") && cleanId.includes("/page/")) {
        return PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID;
      }
      if (cleanId.includes("/compose/client/lib/sketchpad/js/page/") && cleanId.endsWith(".mdx")) {
        return PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID;
      }
      if (
        id === "@semio-tech/compose-sketchpad" ||
        id === "@semio-tech/compose-sketchpad/boot" ||
        id.startsWith("@semio-tech/compose-sketchpad/") ||
        id === sketchpadIndex ||
        id === sketchpadRoot ||
        (id.startsWith(`${sketchpadRoot}/`) && !id.endsWith(".mdx"))
      ) {
        return PLAYGROUND_COMPOSE_SKETCHPAD_STUB_ID;
      }
      return undefined;
    },
    load(id) {
      if (id === PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID) {
        return "export default function SketchpadMdxStub() { return null; }";
      }
      if (id !== PLAYGROUND_COMPOSE_SKETCHPAD_STUB_ID) return;
      return `export const COMPOSE_SKETCHPAD_PROGRAM_ID = "compose.sketchpad";
export async function ensureSketchpadPlatform() {
  throw new Error("compose-sketchpad is only available in the s playground");
}
export function buildSketchpadProgramDefinition() {
  return { id: COMPOSE_SKETCHPAD_PROGRAM_ID, name: "Compose Sketchpad", apiVersion: "1", apps: [], createPlatformApi: () => ({}) };
}`;
    },
  };
}

/** @emoji 📄 MDX support for sketchpad when manifest declares `sketchpad-mdx`, or a stub elsewhere. */
export function playgroundComposeSketchpadVitePlugins(repoRoot: string, enableSketchpadMdx: boolean): Plugin[] {
  const sketchpadIndex = resolve(repoRoot, "compose/client/lib/sketchpad/js/index.ts");
  const mdxStubPlugins: Plugin[] = [
    {
      name: "playground-compose-sketchpad-mdx-stub-load",
      enforce: "pre",
      resolveId(id) {
        const cleanId = id.split("?", 1)[0] ?? id;
        if (cleanId.endsWith(".mdx") && cleanId.includes("sketchpad") && cleanId.includes("/page/")) {
          return PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID;
        }
        return undefined;
      },
      load(id) {
        if (id === PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID) {
          return "export default function SketchpadMdxStub() { return null; }";
        }
        return undefined;
      },
    },
  ];
  if (enableSketchpadMdx) {
    const sketchpadMdxStubSource = "export default function SketchpadMdxStub() { return null; }";
    return [
      {
        name: "playground-compose-sketchpad-docs-mdx-stub",
        enforce: "pre",
        resolveId(id) {
          const cleanId = id.split("?", 1)[0] ?? id;
          if (cleanId.endsWith(".mdx") && cleanId.includes("/compose/client/lib/sketchpad/")) {
            return PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID;
          }
          return undefined;
        },
        load(id) {
          const cleanId = id.split("?", 1)[0] ?? id;
          if (id === PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID) {
            return sketchpadMdxStubSource;
          }
          if (cleanId.endsWith(".mdx") && cleanId.includes("/compose/client/lib/sketchpad/")) {
            return sketchpadMdxStubSource;
          }
          return undefined;
        },
      },
    ];
  }
  return [
    playgroundComposeSketchpadStubPlugin(repoRoot),
    ...mdxStubPlugins,
  ];
}

/** @emoji 🧱 Stubs Playwright when test-only regions are pulled into the browser graph. */
export function playgroundPlaywrightDevStubPlugin(): Plugin {
  return {
    name: "playground-playwright-dev-stub",
    enforce: "pre",
    resolveId(id) {
      if (id === "@playwright/test" || id === "playwright" || id === "playwright-core" || id === "chromium-bidi") {
        return PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID;
      }
      return undefined;
    },
    load(id) {
      if (id !== PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID) return;
      return "export default {}; export const test = () => {}; export const expect = () => ({ toBe: () => {}, toEqual: () => {} });";
    },
  };
}

/** @emoji 🔄 Full-reload connected clients when a stale optimized-dep chunk returns 504. */
export function playgroundStaleOptimizeDepPlugin(): Plugin {
  return {
    name: "playground-stale-optimize-dep",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!isPlaygroundOptimizedDepUrl(req.url ?? "")) {
          next();
          return;
        }
        res.on("finish", () => {
          if (res.statusCode === 504) {
            server.ws.send({ type: "full-reload", path: "*" });
          }
        });
        next();
      });
    },
  };
}

/** @emoji 🖼 Dev/preview CSP so playgrounds can be iframe-embedded locally. */
export function playgroundIframeEmbedHeadersPlugin(): Plugin {
  const useHeaders: Connect.NextHandleFunction = (_req, res, next) => {
    res.setHeader("Content-Security-Policy", "frame-ancestors *");
    res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
    res.setHeader("Cross-Origin-Embedder-Policy", "credentialless");
    next();
  };
  return {
    name: "playground-iframe-embed-headers",
    configureServer(server) {
      server.middlewares.use(useHeaders);
    },
    configurePreviewServer(server) {
      server.middlewares.use(useHeaders);
    },
  };
}

function contentTypeForUiAsset(filePath: string): string | undefined {
  if (filePath.endsWith(".woff2")) {
    return "font/woff2";
  }
  if (filePath.endsWith(".svg")) {
    return "image/svg+xml";
  }
  if (filePath.endsWith(".wasm")) {
    return "application/wasm";
  }
  return undefined;
}

function createUiAssetsMiddleware(assetsRoot: string): Connect.NextHandleFunction {
  const assetsRootResolved = resolve(assetsRoot);
  return (req, res, next) => {
    if (!req.url?.startsWith("/asset/")) {
      next();
      return;
    }
    const rel = decodeURIComponent(req.url.slice("/asset/".length).split(/[?#]/, 1)[0] ?? "");
    const filePath = resolve(assetsRootResolved, rel);
    const relToRoot = relative(assetsRootResolved, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    const contentType = contentTypeForUiAsset(filePath);
    if (contentType) {
      res.setHeader("Content-Type", contentType);
    }
    createReadStream(filePath).pipe(res);
  };
}

/** @emoji 📂 Kit fixture GLB roots for puzzle 3d `/mesh/*` URLs. */
export function puzzle3dKitMeshRoots(repoRoot: string): { readonly meshRoots: readonly string[]; readonly placeholderMesh: string } {
  const metabolismMeshCandidates = [
    resolve(repoRoot, "asset/metabolism/representation"),
    resolve(repoRoot, "asset/metabolism/representations"),
  ];
  const metabolismMeshRoot = metabolismMeshCandidates.find((candidate) => existsSync(candidate)) ?? metabolismMeshCandidates[0]!;
  return {
    meshRoots: [metabolismMeshRoot, resolve(repoRoot, "asset/abbau-aufbau")],
    placeholderMesh: resolve(repoRoot, "asset/mesh/placeholder.glb"),
  };
}

/** @emoji 🌐 Connect middleware: serve kit GLBs at `/mesh/<name>.glb` (first matching root wins). */
export function createPuzzle3dMeshesMiddleware(meshRoots: readonly string[], placeholderMesh: string): Connect.NextHandleFunction {
  const rootsResolved = meshRoots.map((root) => resolve(root));
  const placeholderResolved = resolve(placeholderMesh);
  return (req, res, next) => {
    if (!req.url?.startsWith("/mesh/")) {
      next();
      return;
    }
    const rawName = decodeURIComponent(req.url.slice("/mesh/".length).split(/[?#]/, 1)[0] ?? "");
    if (rawName === "placeholder.glb") {
      if (!existsSync(placeholderResolved) || !statSync(placeholderResolved).isFile()) {
        next();
        return;
      }
      res.setHeader("Content-Type", "model/gltf-binary");
      createReadStream(placeholderResolved).pipe(res);
      return;
    }
    for (const meshRootResolved of rootsResolved) {
      const filePath = resolve(meshRootResolved, rawName);
      const relToRoot = relative(meshRootResolved, filePath);
      if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
        continue;
      }
      if (!existsSync(filePath) || !statSync(filePath).isFile()) {
        continue;
      }
      res.setHeader("Content-Type", "model/gltf-binary");
      createReadStream(filePath).pipe(res);
      return;
    }
    next();
  };
}

const PUZZLE_3D_LOCKED_FIXTURE_JSON_REL: Readonly<Record<string, readonly string[]>> = {};

/** @emoji 🔒 Resolves locked-example fixture paths for puzzle 3d play bundles. */
export function resolvePuzzle3dLockedFixtureJsonRel(_repoRoot: string): Readonly<Record<string, readonly string[]>> {
  return PUZZLE_3D_LOCKED_FIXTURE_JSON_REL;
}

/** @emoji 🔎 Collects `/mesh/*.glb` basenames referenced anywhere in fixture JSON. */
export function puzzle3dMeshBasenamesInJson(value: unknown, out = new Set<string>()): Set<string> {
  if (typeof value === "string") {
    const match = /^\/mesh\/([^?#]+\.glb)$/i.exec(value.trim());
    if (match) {
      out.add(decodeURIComponent(match[1]!));
    }
    return out;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      puzzle3dMeshBasenamesInJson(item, out);
    }
    return out;
  }
  if (value && typeof value === "object") {
    for (const entry of Object.values(value as Record<string, unknown>)) {
      puzzle3dMeshBasenamesInJson(entry, out);
    }
  }
  return out;
}

/** @emoji 🔒 GLB basenames required by {@link PLAYGROUND_LOCKED_EXAMPLE_ENV}, if set. */
export function puzzle3dLockedExampleMeshBasenames(repoRoot: string): Set<string> | undefined {
  const exampleId = playgroundLockedExampleIdFromEnv();
  if (!exampleId) {
    return undefined;
  }
  const relPaths = resolvePuzzle3dLockedFixtureJsonRel(repoRoot)[exampleId];
  if (!relPaths?.length) {
    return undefined;
  }
  const basenames = new Set<string>();
  for (const rel of relPaths) {
    const filePath = resolve(repoRoot, rel);
    if (!existsSync(filePath)) {
      continue;
    }
    try {
      puzzle3dMeshBasenamesInJson(JSON.parse(readFileSync(filePath, "utf8")), basenames);
    } catch {
      continue;
    }
  }
  basenames.add("placeholder.glb");
  return basenames;
}

/** @emoji 📦 Copies kit GLBs into a static `dist/mesh` tree (optional basename filter). */
export function copyPuzzle3dKitGlbs(meshRoots: readonly string[], dest: string, onlyBasenames?: ReadonlySet<string>): void {
  mkdirSync(dest, { recursive: true });
  const copied = new Set<string>();
  const want = (basename: string) => !onlyBasenames || onlyBasenames.has(basename);
  for (const meshRoot of meshRoots) {
    if (!existsSync(meshRoot)) {
      continue;
    }
    for (const entry of readdirSync(meshRoot)) {
      if (!entry.endsWith(".glb") || !want(entry) || copied.has(entry)) {
        continue;
      }
      const src = resolve(meshRoot, entry);
      if (!statSync(src).isFile()) {
        continue;
      }
      cpSync(src, resolve(dest, entry));
      copied.add(entry);
    }
  }
}

/** @emoji 🧊 Vite: serve and copy kit meshes at `/mesh/*` for puzzle 3d play and sketchpad. */
export function puzzle3dMeshesVitePlugin(repoRoot: string): Plugin[] {
  const { meshRoots, placeholderMesh } = puzzle3dKitMeshRoots(repoRoot);
  const serveMeshes = createPuzzle3dMeshesMiddleware(meshRoots, placeholderMesh);
  let viteRoot = process.cwd();
  return [
    {
      name: "puzzle-3d-meshes-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveMeshes);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveMeshes);
      },
    },
    {
      name: "puzzle-3d-meshes-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        const dest = resolve(viteRoot, "dist", "mesh");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        copyPuzzle3dKitGlbs(meshRoots, dest, puzzle3dLockedExampleMeshBasenames(repoRoot));
        if (existsSync(placeholderMesh)) {
          cpSync(placeholderMesh, resolve(dest, "placeholder.glb"));
        }
      },
    },
  ];
}

/** @emoji 🎬 Inline shell paint before Tailwind finishes compiling the play stylesheet. */
export const PLAYGROUND_PLAY_BOOT_INLINE_STYLE =
  "html{color-scheme:light dark}html,body,#root{height:100%;margin:0}body{background-color:#f7f3e3;color:#001117}html.dark body{background-color:#001117;color:#f7f3e3}html:not([data-semio-styled]) body{visibility:hidden}";

/** @emoji 🌓 Synchronous system theme bootstrap for play `index.html` heads. */
export const PLAYGROUND_PLAY_BOOT_THEME_SCRIPT = `(function(){var d=document.documentElement,m=window.matchMedia("(prefers-color-scheme: dark)");var dark=m.matches;d.classList.toggle("dark",dark);d.dataset.uiTheme=dark?"dark":"light";d.style.colorScheme=dark?"dark":"light";if(document.body){document.body.style.colorScheme=dark?"dark":"light";document.body.style.backgroundColor=dark?"#001117":"#f7f3e3";document.body.style.color=dark?"#f7f3e3":"#001117";}})();`;

/** @emoji 👁️ Reveals the play shell after the linked globals stylesheet finishes loading. */
export const PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT = `(function(){function reveal(){document.documentElement.dataset.semioStyled="ready"}var link=document.getElementById("semio-play-styles");if(link){if(link.sheet)reveal();else link.addEventListener("load",reveal,{once:true})}else{reveal()}setTimeout(reveal,8000)})();`;

/** @emoji 🎬 Vite: inject early theme + stylesheet link into play `index.html` to avoid unstyled flashes. */
export function playgroundPlayBootHtmlPlugin(): Plugin {
  return {
    name: "playground-play-boot-html",
    transformIndexHtml: {
      order: "pre",
      handler() {
        return {
          tags: [
            { tag: "style", children: PLAYGROUND_PLAY_BOOT_INLINE_STYLE, injectTo: "head-prepend" },
            { tag: "script", children: PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, injectTo: "head-prepend" },
            { tag: "link", attrs: { rel: "stylesheet", href: "./globals.css", id: "semio-play-styles" }, injectTo: "head" },
            { tag: "script", children: PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, injectTo: "head" },
          ],
        };
      },
    },
  };
}

/** @emoji 🔖 Canonical semio emblem favicon `<link>` tags for playground and app `index.html` heads. */
export const SEMIO_FAVICON_HEAD_HTML =
  `<link rel="icon" href="./favicon.svg" type="image/svg+xml" />\n    <link rel="icon" href="./favicon.ico" sizes="any" />`;

/** @emoji 🔖 Repo-root paths for the round dark emblem SVG and ICO fallback (matches {@link SemioLogo}). */
export function semioFaviconSources(repoRoot: string): { readonly svg: string; readonly ico: string } {
  const logoRoot = resolve(repoRoot, "asset/logo");
  return {
    svg: resolve(logoRoot, "emblem_dark_round.svg"),
    ico: resolve(logoRoot, "favicon_dark_round_32x32.ico"),
  };
}

const SEMIO_FAVICON_BLEED_RECT = '<rect width="350" height="350" fill="#001117"/>';

/** @emoji 🔖 Favicon SVG with opaque bleed so ICO rasterization avoids white matte outside the round emblem. */
export function semioFaviconSvgMarkup(svgPath: string): string | undefined {
  if (!existsSync(svgPath)) {
    return undefined;
  }
  const raw = readFileSync(svgPath, "utf8");
  if (raw.includes(SEMIO_FAVICON_BLEED_RECT)) {
    return raw;
  }
  const open = raw.match(/<svg[^>]*>/)?.[0];
  if (!open) {
    return raw;
  }
  return raw.replace(open, `${open}${SEMIO_FAVICON_BLEED_RECT}`);
}

function createSemioFaviconMiddleware(favicons: { readonly svg: string; readonly ico: string }): Connect.NextHandleFunction {
  return (req, res, next) => {
    const url = req.url?.split(/[?#]/, 1)[0];
    if (url === "/favicon.svg") {
      const markup = semioFaviconSvgMarkup(favicons.svg);
      if (markup) {
        res.setHeader("Content-Type", "image/svg+xml");
        res.end(markup);
        return;
      }
    }
    if (url === "/favicon.ico" && existsSync(favicons.ico)) {
      res.setHeader("Content-Type", "image/x-icon");
      createReadStream(favicons.ico).pipe(res);
      return;
    }
    next();
  };
}

/** @emoji 🔖 Vite: serve and copy semio emblem favicons at `/favicon.svg` and `/favicon.ico`. */
export function semioFaviconVitePlugin(repoRoot: string): Plugin[] {
  const favicons = semioFaviconSources(repoRoot);
  const serveFavicon = createSemioFaviconMiddleware(favicons);
  let viteRoot = process.cwd();
  return [
    {
      name: "semio-favicon-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFavicon);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFavicon);
      },
    },
    {
      name: "semio-favicon-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        const dist = resolve(viteRoot, "dist");
        mkdirSync(dist, { recursive: true });
        const markup = semioFaviconSvgMarkup(favicons.svg);
        if (markup) {
          writeFileSync(resolve(dist, "favicon.svg"), markup);
        }
        if (existsSync(favicons.ico)) {
          cpSync(favicons.ico, resolve(dist, "favicon.ico"));
        }
      },
    },
  ];
}

/** @emoji 🌐 Vite: serve and copy `ui/asset` at `/asset/*` for palette fonts and cursors. */
export function uiAssetsVitePlugin(assetsRoot: string): Plugin[] {
  let viteRoot = process.cwd();
  const serveAssets = createUiAssetsMiddleware(assetsRoot);
  return [
    {
      name: "ui-assets-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveAssets);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveAssets);
      },
    },
    {
      name: "ui-assets-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        if (!existsSync(assetsRoot)) {
          return;
        }
        const dest = resolve(viteRoot, "dist", "asset");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        cpSync(assetsRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🛝 Playground app kind for Vite play harness config (validated against manifest scan). */
export type PlaygroundRendererPuzzleKind = string;

function namedImportSpecifiersForModule(source: string, moduleId: string): string[] {
  const escaped = moduleId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const re = new RegExp(`import\\s*\\{([^}]+)\\}\\s*from\\s*["']${escaped}["']`, "gs");
  const names: string[] = [];
  let match: RegExpExecArray | null;
  while ((match = re.exec(source))) {
    for (const part of match[1].split(",")) {
      const trimmed = part.trim();
      if (!trimmed) continue;
      const name = trimmed.replace(/^type\s+/, "").split(/\s+as\s+/)[0]?.trim();
      if (name) names.push(name);
    }
  }
  return names;
}

/** @emoji 🔁 Named import specifiers duplicated within the same module import block(s). */
export function duplicateNamedImportsForModule(source: string, moduleId: string): string[] {
  const names = namedImportSpecifiersForModule(source, moduleId);
  const seen = new Set<string>();
  const dupes: string[] = [];
  for (const name of names) {
    if (seen.has(name)) dupes.push(name);
    else seen.add(name);
  }
  return dupes;
}

const PRESENTATION_RENDERER_VITEST_START = "//#region 🧪Tests";

/** @emoji ✂️ Drops vitest regions (and hoisted slide globs) from presentation renderer in browser dev. */
export function presentationRendererVitestStripPlugin(presentationIndexPath: string): Plugin {
  return {
    name: "presentation-renderer-vitest-strip",
    enforce: "pre",
    load(id) {
      if (process.env.VITEST) return;
      const filePath = id.split("?")[0];
      if (filePath !== presentationIndexPath) return;
      const source = readFileSync(presentationIndexPath, "utf8");
      const testsStart = source.indexOf(PRESENTATION_RENDERER_VITEST_START);
      if (testsStart < 0) return source;
      return source.slice(0, testsStart);
    },
  };
}

export type PlaygroundPlayViteOptions = {
  readonly playDir: string;
  readonly repoRoot: string;
  /** @emoji 🎯 When set, `import.meta.env.PLAYGROUND_APP_KIND` gates browser boot in that play's `index.ts`. */
  readonly playEntryKind?: string;
  readonly extraAliases?: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }>;
  readonly extraPlugins?: readonly Plugin[];
  readonly watchIgnored?: readonly string[];
  readonly build?: UserConfig["build"];
  readonly server?: UserConfig["server"];
  readonly optimizeDeps?: UserConfig["optimizeDeps"];
  readonly resolveDedupe?: readonly string[];
};

/** @emoji 🎬 R3F packages that must resolve once with {@link sceneHostPort} and drei controls. */
export const PLAYGROUND_SCENE_HOST_DEDUPE = ["@react-three/fiber", "@react-three/drei"] as const;

/** @emoji 🎬 Vite aliases that pin R3F to a single node_modules entry (avoids duplicate Canvas stores). */
export function playgroundSceneHostResolveAliases(repoRoot: string): ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> {
  return [
    { find: /^@react-three\/fiber$/, replacement: resolve(repoRoot, "node_modules/@react-three/fiber/dist/react-three-fiber.esm.js") },
    { find: /^@react-three\/drei$/, replacement: resolve(repoRoot, "node_modules/@react-three/drei/index.js") },
  ];
}

//#region 🔖MapTileCache
/** @emoji 🗺 Compliant User-Agent for OSM / MapLibre demotiles in map play. */
export const GIS_MAP_TILE_USER_AGENT = "ComposeGisMapPlay/0.1 (+https://github.com/usalu/semio; dev playground)";

/** @emoji 🗺 Default dev prefetch bounds (Switzerland) for GIS map play. */
export const GIS_MAP_DEFAULT_PREFETCH_BOUNDS = {
  west: 5.95,
  south: 45.82,
  east: 10.52,
  north: 47.81,
} as const;

export type GisMapPrefetchBounds = {
  readonly west: number;
  readonly south: number;
  readonly east: number;
  readonly north: number;
};

export const GIS_MAP_OSM_TILE_MAX_Z = 19;
/** @emoji 🗺 OpenFreeMap / OpenMapTiles planet MVT (OSM); matches raster detail up to z14. */
export const GIS_MAP_VECTOR_TILE_MAX_Z = 14;
export const GIS_MAP_OPENFREEMAP_TILEJSON = "https://tiles.openfreemap.org/planet";
/** @emoji 🗺 Highest zoom prefetched for offline map play (matches `GIS_MAP_LOD_TILE_Z` building band). */
export const GIS_MAP_PREFETCH_RASTER_Z_MAX = 13;

/** @emoji 🗺 `fetch` loads missing tiles at runtime; `bundle` serves only cached tiles and copies them into `dist` on build. */
export type GisMapTileServeMode = "fetch" | "bundle";

export const GIS_MAP_TILE_SERVE_MODE_ENV = "GIS_MAP_TILE_SERVE_MODE";

export function resolveGisMapTileServeMode(value?: string): GisMapTileServeMode {
  return value === "bundle" ? "bundle" : "fetch";
}

export function mapTileCacheRoots(repoRoot: string): { readonly osm: string; readonly vt: string } {
  return {
    osm: resolve(repoRoot, ".repo-cache", "osm-tiles"),
    vt: resolve(repoRoot, ".repo-cache", "openfreemap-vt"),
  };
}

/** @emoji 🧭 Web Mercator tile index for a lon/lat at zoom `z`. */
export function lonLatToTileXY(lon: number, lat: number, z: number): { x: number; y: number } {
  const n = 2 ** z;
  const x = Math.floor(((lon + 180) / 360) * n);
  const latRad = (lat * Math.PI) / 180;
  const y = Math.floor(((1 - Math.log(Math.tan(latRad) + 1 / Math.cos(latRad)) / Math.PI) / 2) * n);
  return { x: Math.max(0, Math.min(n - 1, x)), y: Math.max(0, Math.min(n - 1, y)) };
}

/** @emoji 📐 Inclusive OSM tile index range covering `bounds` at zoom `z`. */
export function tileRangeForBounds(bounds: GisMapPrefetchBounds, z: number): { x0: number; x1: number; y0: number; y1: number } {
  const sw = lonLatToTileXY(bounds.west, bounds.south, z);
  const ne = lonLatToTileXY(bounds.east, bounds.north, z);
  return {
    x0: Math.min(sw.x, ne.x),
    x1: Math.max(sw.x, ne.x),
    y0: Math.min(sw.y, ne.y),
    y1: Math.max(sw.y, ne.y),
  };
}

export type GisMapTileCoord = { readonly z: number; readonly x: number; readonly y: number };

/** @emoji 📋 Lists every tile in `bounds` for zoom levels `zMin`…`zMax` (inclusive). */
export function listMapTilesForBounds(bounds: GisMapPrefetchBounds, zMin: number, zMax: number): GisMapTileCoord[] {
  const lo = Math.max(0, Math.min(zMin, zMax));
  const hi = Math.max(lo, zMax);
  const out: GisMapTileCoord[] = [];
  for (let z = lo; z <= hi; z++) {
    const { x0, x1, y0, y1 } = tileRangeForBounds(bounds, z);
    for (let x = x0; x <= x1; x++) {
      for (let y = y0; y <= y1; y++) {
        out.push({ z, x, y });
      }
    }
  }
  return out;
}

export type PrefetchMapTilesResult = {
  readonly downloaded: number;
  readonly skipped: number;
  readonly failed: number;
};

export type PrefetchMapTilesOptions = {
  readonly repoRoot: string;
  readonly bounds?: GisMapPrefetchBounds;
  readonly raster?: boolean;
  readonly vector?: boolean;
  readonly zMinRaster?: number;
  readonly zMaxRaster?: number;
  readonly zMinVector?: number;
  readonly zMaxVector?: number;
  readonly concurrency?: number;
  readonly skipExisting?: boolean;
  readonly delayMs?: number;
  readonly log?: (line: string) => void;
};

async function fetchOsmTileToCache(cacheRoot: string, z: number, x: number, y: number): Promise<boolean> {
  const rel = `${z}/${x}/${y}.png`;
  const filePath = resolve(cacheRoot, rel);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return false;
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const upstream = await fetch(`https://tile.openstreetmap.org/${z}/${x}/${y}.png`, {
    headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT },
  });
  if (!upstream.ok) {
    return false;
  }
  await writeFile(filePath, Buffer.from(await upstream.arrayBuffer()));
  return true;
}

let openFreeMapTileTemplate: string | null = null;
let openFreeMapTileTemplateAt = 0;
const OPENFREEMAP_TILE_TEMPLATE_TTL_MS = 7 * 24 * 60 * 60 * 1000;

async function resolveOpenFreeMapTileTemplate(): Promise<string> {
  const now = Date.now();
  if (openFreeMapTileTemplate && now - openFreeMapTileTemplateAt < OPENFREEMAP_TILE_TEMPLATE_TTL_MS) {
    return openFreeMapTileTemplate;
  }
  const res = await fetch(GIS_MAP_OPENFREEMAP_TILEJSON, { headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT } });
  if (!res.ok) {
    throw new Error(`OpenFreeMap TileJSON failed: ${res.status}`);
  }
  const json = (await res.json()) as { tiles?: string[] };
  const template = json.tiles?.[0];
  if (typeof template !== "string" || !template.includes("{z}")) {
    throw new Error("OpenFreeMap TileJSON missing tiles URL template");
  }
  openFreeMapTileTemplate = template;
  openFreeMapTileTemplateAt = now;
  return template;
}

async function fetchVtTileToCache(cacheRoot: string, z: number, x: number, y: number): Promise<boolean> {
  const rel = `${z}/${x}/${y}.pbf`;
  const filePath = resolve(cacheRoot, rel);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return false;
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const template = await resolveOpenFreeMapTileTemplate();
  const url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
  const upstream = await fetch(url, { headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT } });
  if (!upstream.ok) {
    return false;
  }
  const buf = Buffer.from(await upstream.arrayBuffer());
  if (buf.length === 0) {
    return false;
  }
  await writeFile(filePath, buf);
  return true;
}

/** @emoji ⬇️ Prefetch OSM PNG and MapLibre MVT tiles into `.repo-cache` for offline map play. */
export async function prefetchMapTiles(options: PrefetchMapTilesOptions): Promise<PrefetchMapTilesResult> {
  const {
    repoRoot,
    bounds = GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
    raster = true,
    vector = true,
    zMinRaster = 0,
    zMaxRaster = GIS_MAP_PREFETCH_RASTER_Z_MAX,
    zMinVector = 0,
    zMaxVector = GIS_MAP_VECTOR_TILE_MAX_Z,
    concurrency = 4,
    skipExisting = true,
    delayMs = 120,
    log = (line) => console.log(line),
  } = options;
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  const jobs: { kind: "osm" | "vt"; z: number; x: number; y: number }[] = [];
  if (raster) {
    for (const { z, x, y } of listMapTilesForBounds(bounds, zMinRaster, Math.min(zMaxRaster, GIS_MAP_OSM_TILE_MAX_Z))) {
      jobs.push({ kind: "osm", z, x, y });
    }
  }
  if (vector) {
    for (const { z, x, y } of listMapTilesForBounds(bounds, zMinVector, Math.min(zMaxVector, GIS_MAP_VECTOR_TILE_MAX_Z))) {
      jobs.push({ kind: "vt", z, x, y });
    }
  }
  const zoomLabel = `(raster z${zMinRaster}-${zMaxRaster}, vector z${zMinVector}-${zMaxVector})`;
  let skipped = 0;
  const pending = skipExisting
    ? jobs.filter((job) => {
        const cacheRoot = job.kind === "osm" ? osm : vt;
        const ext = job.kind === "osm" ? "png" : "pbf";
        const filePath = resolve(cacheRoot, `${job.z}/${job.x}/${job.y}.${ext}`);
        if (existsSync(filePath)) {
          skipped++;
          return false;
        }
        return true;
      })
    : jobs;
  log(
    `[gis/2d/play] prefetch ${jobs.length} tiles ${zoomLabel}` +
      (skipExisting ? ` (${skipped} cached, ${pending.length} to fetch)` : ""),
  );
  if (pending.length === 0) {
    log(`[gis/2d/play] prefetch done: downloaded=0 skipped=${skipped} failed=0`);
    return { downloaded: 0, skipped, failed: 0 };
  }
  let downloaded = 0;
  let failed = 0;
  const sleep = (ms: number) => new Promise<void>((r) => setTimeout(r, ms));
  for (let i = 0; i < pending.length; i += concurrency) {
    const batch = pending.slice(i, i + concurrency);
    await Promise.all(
      batch.map(async (job) => {
        const cacheRoot = job.kind === "osm" ? osm : vt;
        const ok =
          job.kind === "osm"
            ? await fetchOsmTileToCache(cacheRoot, job.z, job.x, job.y)
            : await fetchVtTileToCache(cacheRoot, job.z, job.x, job.y);
        if (ok) {
          downloaded++;
        } else {
          failed++;
        }
      }),
    );
    if (delayMs > 0 && i + concurrency < pending.length) {
      await sleep(delayMs);
    }
  }
  log(`[gis/2d/play] prefetch done: downloaded=${downloaded} skipped=${skipped} failed=${failed}`);
  return { downloaded, skipped, failed };
}
//#endregion 🔖MapTileCache

function createOsmTileMiddleware(cacheRoot: string, mode: GisMapTileServeMode): Connect.NextHandleFunction {
  return async (req, res, next) => {
    const match = req.url?.match(/^\/osm\/(\d+)\/(\d+)\/(\d+)\.png(?:\?.*)?$/);
    if (!match) {
      next();
      return;
    }
    const [, z, x, y] = match;
    const rel = `${z}/${x}/${y}.png`;
    const filePath = resolve(cacheRoot, rel);
    const relToRoot = relative(cacheRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
      next();
      return;
    }
    if (existsSync(filePath)) {
      res.setHeader("Content-Type", "image/png");
      createReadStream(filePath).pipe(res);
      return;
    }
    if (mode === "bundle") {
      res.statusCode = 404;
      res.end();
      return;
    }
    try {
      const ok = await fetchOsmTileToCache(cacheRoot, Number(z), Number(x), Number(y));
      if (!ok) {
        res.statusCode = 404;
        res.end();
        return;
      }
      res.setHeader("Content-Type", "image/png");
      createReadStream(filePath).pipe(res);
    } catch {
      res.statusCode = 502;
      res.end();
    }
  };
}

function createVtTileMiddleware(cacheRoot: string, mode: GisMapTileServeMode): Connect.NextHandleFunction {
  return async (req, res, next) => {
    const match = req.url?.match(/^\/vt\/(\d+)\/(\d+)\/(\d+)\.pbf(?:\?.*)?$/);
    if (!match) {
      next();
      return;
    }
    const [, z, x, y] = match;
    const rel = `${z}/${x}/${y}.pbf`;
    const filePath = resolve(cacheRoot, rel);
    const relToRoot = relative(cacheRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
      next();
      return;
    }
    if (existsSync(filePath)) {
      res.setHeader("Content-Type", "application/x-protobuf");
      createReadStream(filePath).pipe(res);
      return;
    }
    if (mode === "bundle") {
      res.statusCode = 404;
      res.end();
      return;
    }
    try {
      const ok = await fetchVtTileToCache(cacheRoot, Number(z), Number(x), Number(y));
      if (!ok) {
        res.statusCode = 404;
        res.end();
        return;
      }
      res.setHeader("Content-Type", "application/x-protobuf");
      createReadStream(filePath).pipe(res);
    } catch {
      res.statusCode = 502;
      res.end();
    }
  };
}

/** @emoji 🗺 Standalone HTTP server for GIS map raster/vector tiles (wgpu Trunk / native-bin dev). */
export function startGisMapTileProxyServer(
  repoRoot: string,
  port: number,
  mode: GisMapTileServeMode = "fetch",
  host = "127.0.0.1",
): Server {
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  const serveOsm = createOsmTileMiddleware(osm, mode);
  const serveVt = createVtTileMiddleware(vt, mode);
  const server = createServer((req, res) => {
    serveOsm(req, res, () => {
      serveVt(req, res, () => {
        res.statusCode = 404;
        res.end();
      });
    });
  });
  server.listen(port, host);
  return server;
}

/** @emoji 🗺 Vite plugins for GIS map raster/vector tiles (`fetch` or offline `bundle`). */
export function gisMapTilesVitePlugins(repoRoot: string, mode: GisMapTileServeMode = "fetch"): Plugin[] {
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  const serveOsm = createOsmTileMiddleware(osm, mode);
  const serveVt = createVtTileMiddleware(vt, mode);
  let viteRoot = process.cwd();
  const plugins: Plugin[] = [
    {
      name: "gis-2d-osm-tiles",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveOsm);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveOsm);
      },
    },
    {
      name: "gis-2d-vt-tiles",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveVt);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveVt);
      },
    },
  ];
  if (mode === "bundle") {
    plugins.push({
      name: "gis-2d-tiles-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        const dist = resolve(viteRoot, "dist");
        mkdirSync(dist, { recursive: true });
        if (existsSync(osm)) {
          cpSync(osm, resolve(dist, "osm"), { recursive: true });
        }
        if (existsSync(vt)) {
          cpSync(vt, resolve(dist, "vt"), { recursive: true });
        }
      },
    });
  }
  return plugins;
}

/** @emoji 🗺 Dev/preview proxy for OpenStreetMap raster tiles at `/osm/:z/:x/:y.png`. */
export function osmTileProxyVitePlugin(cacheDir: string, mode: GisMapTileServeMode = "fetch"): Plugin {
  const { osm } = mapTileCacheRoots(cacheDir);
  const serveOsm = createOsmTileMiddleware(osm, mode);
  return {
    name: "osm-tile-proxy",
    enforce: "pre",
    configureServer(server) {
      server.middlewares.use(serveOsm);
    },
    configurePreviewServer(server) {
      server.middlewares.use(serveOsm);
    },
  };
}

/** @emoji 🗺 Dev/preview proxy for OpenFreeMap OSM MVT at `/vt/:z/:x/:y.pbf`. */
export function mapLibreVectorTileProxyVitePlugin(cacheDir: string, mode: GisMapTileServeMode = "fetch"): Plugin {
  const { vt } = mapTileCacheRoots(cacheDir);
  const serveVt = createVtTileMiddleware(vt, mode);
  return {
    name: "maplibre-vt-proxy",
    enforce: "pre",
    configureServer(server) {
      server.middlewares.use(serveVt);
    },
    configurePreviewServer(server) {
      server.middlewares.use(serveVt);
    },
  };
}

/** @emoji 🦀 Vite `optimizeDeps.exclude` entries for wasm-bindgen flow modules (must not be prebundled). */
export const FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE = [
  "@semio-tech/flow-module-core",
  "@semio-tech/flow-module-math",
  "@semio-tech/flow-module-text",
  "@semio-tech/flow-module-logic",
  "@semio-tech/flow-module-dictionary",
  "@semio-tech/flow-module-list",
  "@semio-tech/flow-module-brep",
  "@semio-tech/flow-module-draw",
] as const;

/** @emoji 🧭 Workspace Vite resolve preset: dedupe, fs.allow, optimizeDeps.exclude, scene-host aliases. */
export function createWorkspaceViteResolveConfig(
  repoRoot: string,
  extraAliases: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> = [],
): Pick<UserConfig, "resolve" | "server" | "optimizeDeps"> {
  return {
    resolve: {
      alias: [...extraAliases],
      dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
    },
    server: {
      fs: { allow: [repoRoot] },
    },
    optimizeDeps: {
      exclude: [...findWorkspacePackages(repoRoot), ...FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE],
    },
  };
}

/** @emoji 🖼️ Vite: serve and copy `cad/fixture` at `/cad-fixture/*` for CAD world reference planes. */
export function cadFixtureVitePlugin(repoRoot: string): Plugin[] {
  const fixtureRoot = resolve(repoRoot, "cad/fixture");
  const serveFixture: Connect.NextHandleFunction = (req, res, next) => {
    if (!req.url?.startsWith("/cad-fixture/")) {
      next();
      return;
    }
    const rel = decodeURIComponent(req.url.slice("/cad-fixture/".length).split(/[?#]/, 1)[0] ?? "");
    const filePath = resolve(fixtureRoot, rel);
    const relToRoot = relative(fixtureRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    if (filePath.endsWith(".png")) {
      res.setHeader("Content-Type", "image/png");
    } else if (filePath.endsWith(".jpg") || filePath.endsWith(".jpeg")) {
      res.setHeader("Content-Type", "image/jpeg");
    } else if (filePath.endsWith(".pdf")) {
      res.setHeader("Content-Type", "application/pdf");
    } else if (filePath.endsWith(".svg")) {
      res.setHeader("Content-Type", "image/svg+xml");
    }
    createReadStream(filePath).pipe(res);
  };
  let viteRoot = process.cwd();
  return [
    {
      name: "cad-fixture-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFixture);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFixture);
      },
    },
    {
      name: "cad-fixture-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        if (!existsSync(fixtureRoot)) {
          return;
        }
        const dest = resolve(viteRoot, "dist", "cad-fixture");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        cpSync(fixtureRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🌐 Vite: serve and copy `infinite/fixture` at `/infinite-fixture/*` for world reference planes. */
export function infiniteFixtureVitePlugin(repoRoot: string): Plugin[] {
  const fixtureRoot = resolve(repoRoot, "infinite/fixture");
  const serveFixture: Connect.NextHandleFunction = (req, res, next) => {
    if (!req.url?.startsWith("/infinite-fixture/")) {
      next();
      return;
    }
    const rel = decodeURIComponent(req.url.slice("/infinite-fixture/".length).split(/[?#]/, 1)[0] ?? "");
    const filePath = resolve(fixtureRoot, rel);
    const relToRoot = relative(fixtureRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    if (filePath.endsWith(".png")) {
      res.setHeader("Content-Type", "image/png");
    } else if (filePath.endsWith(".jpg") || filePath.endsWith(".jpeg")) {
      res.setHeader("Content-Type", "image/jpeg");
    } else if (filePath.endsWith(".pdf")) {
      res.setHeader("Content-Type", "application/pdf");
    } else if (filePath.endsWith(".svg")) {
      res.setHeader("Content-Type", "image/svg+xml");
    }
    createReadStream(filePath).pipe(res);
  };
  let viteRoot = process.cwd();
  return [
    {
      name: "infinite-fixture-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFixture);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFixture);
      },
    },
    {
      name: "infinite-fixture-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        if (!existsSync(fixtureRoot)) {
          return;
        }
        const dest = resolve(viteRoot, "dist", "infinite-fixture");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        cpSync(fixtureRoot, dest, { recursive: true });
      },
    },
  ];
}

export function findWorkspacePackages(repoRoot: string): string[] {
  const packages: string[] = [];
  const scan = (dir: string) => {
    for (const entry of readdirSync(dir)) {
      if (entry === "node_modules" || entry === ".git" || entry === ".nx" || entry === "dist" || entry === "target" || entry === "storybook-static") continue;
      const full = resolve(dir, entry);
      if (statSync(full).isDirectory()) {
        scan(full);
      } else if (entry === "package.json" && full !== resolve(repoRoot, "package.json")) {
        try {
          const pkg = JSON.parse(readFileSync(full, "utf8"));
          if (pkg.name && pkg.name.startsWith("@semio-tech/")) {
            packages.push(pkg.name);
          }
        } catch {}
      }
    }
  };
  scan(repoRoot);
  return packages;
}

/** @emoji 📄 Stubs all compose sketchpad MDX modules for s/os dev graphs (including worker bundles). */
function playgroundSketchpadMdxGlobalStubPlugin(): Plugin {
  const stub = "export default function SketchpadMdxStub() { return null; }";
  const isSketchpadMdx = (id: string): boolean => {
    const cleanId = id.split("?", 1)[0] ?? id;
    return cleanId.endsWith(".mdx") && cleanId.includes("/compose/client/lib/sketchpad/");
  };
  return {
    name: "playground-sketchpad-mdx-global-stub",
    enforce: "pre",
    resolveId(id) {
      if (!isSketchpadMdx(id)) return;
      return `\0playground-sketchpad-mdx:${id.split("?", 1)[0]}`;
    },
    load(id) {
      if (id.startsWith("\0playground-sketchpad-mdx:") || isSketchpadMdx(id)) return stub;
    },
  };
}

/** @emoji 🛝 `defineConfig` for `@puzzle/*-play` Vite entries with consistent renderer and core aliases. */
export function createPlaygroundPlayViteConfig(options: PlaygroundPlayViteOptions) {
  const { playDir, repoRoot, playEntryKind, extraAliases = [], extraPlugins = [], watchIgnored, build, server, optimizeDeps, resolveDedupe } = options;
  const uiAssetsRoot = resolve(repoRoot, "ui/asset");
  const osHubAliases =
    playEntryKind === "s"
      ? [
          {
            find: "@semio-tech/graph-dsl-core",
            replacement: resolve(repoRoot, "mathematical/graph/dsl/core/js/index.ts"),
          },
        ]
      : [];
  const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot, [...extraAliases, ...osHubAliases]);
  const workerStubPlugins = [playgroundPlaywrightDevStubPlugin(), playgroundVitestDevStubPlugin()];
  return defineConfig({
    root: playDir,
    base: "./",
    publicDir: resolve(playDir, "public"),
    assetsInclude: ["**/*.wasm"],
    worker: {
      format: "es",
      plugins: () => workerStubPlugins,
    },
    define: {
      ...playgroundPlayViteDefine(
        playEntryKind ? { "import.meta.env.PLAYGROUND_APP_KIND": JSON.stringify(playEntryKind) } : {},
      ),
    },
    plugins: [
      playgroundPlayBootHtmlPlugin(),
      playgroundFlowWasmDevStubPlugin(repoRoot),
      playgroundComposeSketchpadStubPlugin(repoRoot),
      ...uiAssetsVitePlugin(uiAssetsRoot),
      ...semioFaviconVitePlugin(repoRoot),
      ...cadFixtureVitePlugin(repoRoot),
      infiniteFixtureVitePlugin(repoRoot),
      tailwindcss(),
      react(),
      playgroundPlaywrightDevStubPlugin(),
      playgroundVitestDevStubPlugin(),
      playgroundIframeEmbedHeadersPlugin(),
      playgroundStaleOptimizeDepPlugin(),
      ...extraPlugins,
    ],
    build: playgroundStaticSiteBuildOptions(build),
    server: {
      ...workspaceResolve.server,
      ...(watchIgnored ? { watch: { ignored: watchIgnored } } : {}),
      ...server,
    },
    resolve: {
      ...workspaceResolve.resolve,
      dedupe: [...(workspaceResolve.resolve?.dedupe ?? []), ...(resolveDedupe ?? [])],
    },
    optimizeDeps: {
      ...workspaceResolve.optimizeDeps,
      ...optimizeDeps,
      exclude: [...(workspaceResolve.optimizeDeps?.exclude ?? []), ...(optimizeDeps?.exclude ?? [])],
    },
  });
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const repoRoot = resolve(fileURLToPath(new URL(".", import.meta.url)), "../..");

  describe("isPlaygroundOptimizedDepUrl", () => {
    it("matches Vite prebundle chunk URLs", () => {
      expect(isPlaygroundOptimizedDepUrl("/node_modules/.vite/deps/chunk-ABC.js?v=1")).toBe(true);
      expect(isPlaygroundOptimizedDepUrl("/index.ts")).toBe(false);
    });
  });

  describe("playgroundSceneHostResolveAliases", () => {
    it("pins fiber and drei to node_modules entries", () => {
      const aliases = playgroundSceneHostResolveAliases(repoRoot);
      expect(aliases.some((row) => String(row.find).includes("fiber") && row.replacement.endsWith("react-three-fiber.esm.js"))).toBe(true);
      expect(aliases.some((row) => String(row.find).includes("drei") && row.replacement.endsWith("@react-three/drei/index.js"))).toBe(true);
    });
  });

  describe("resolveGisMapTileServeMode", () => {
    it("defaults to fetch", () => {
      expect(resolveGisMapTileServeMode(undefined)).toBe("fetch");
      expect(resolveGisMapTileServeMode("")).toBe("fetch");
      expect(resolveGisMapTileServeMode("online")).toBe("fetch");
    });

    it("selects bundle only for bundle", () => {
      expect(resolveGisMapTileServeMode("bundle")).toBe("bundle");
    });
  });

  describe("gisMapTilesVitePlugins", () => {
    it("adds a build copy plugin only for bundle mode", () => {
      const fetchPlugins = gisMapTilesVitePlugins(repoRoot, "fetch");
      const bundlePlugins = gisMapTilesVitePlugins(repoRoot, "bundle");
      expect(fetchPlugins.some((plugin) => plugin.name === "gis-2d-tiles-build")).toBe(false);
      expect(bundlePlugins.some((plugin) => plugin.name === "gis-2d-tiles-build")).toBe(true);
    });
  });

  describe("listMapTilesForBounds", () => {
    it("covers Switzerland at z0 with a single world tile", () => {
      const tiles = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 0, 0);
      expect(tiles).toEqual([{ z: 0, x: 0, y: 0 }]);
    });

    it("returns more tiles at higher zoom", () => {
      const z2 = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 2, 2).length;
      const z4 = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 4, 4).length;
      expect(z4).toBeGreaterThan(z2);
    });
  });

  describe("prefetchMapTiles", () => {
    it("skips tiles already present in cache without fetching", async () => {
      const { osm } = mapTileCacheRoots(repoRoot);
      const tile = { z: 0, x: 0, y: 0 };
      const filePath = resolve(osm, `${tile.z}/${tile.x}/${tile.y}.png`);
      const hadCache = existsSync(filePath);
      if (!hadCache) {
        mkdirSync(resolve(filePath, ".."), { recursive: true });
        writeFileSync(filePath, Buffer.from([0x89, 0x50, 0x4e, 0x47]));
      }
      const lines: string[] = [];
      const result = await prefetchMapTiles({
        repoRoot,
        bounds: GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
        raster: true,
        vector: false,
        zMinRaster: 0,
        zMaxRaster: 0,
        concurrency: 4,
        delayMs: 0,
        log: (line) => lines.push(line),
      });
      expect(result.skipped).toBeGreaterThan(0);
      expect(result.downloaded).toBe(0);
      expect(lines.some((line) => line.includes("cached"))).toBe(true);
      if (!hadCache) {
        const { unlinkSync } = await import("node:fs");
        unlinkSync(filePath);
      }
    });
  });

  describe("playgroundPlayBootHtmlPlugin", () => {
    it("registers index html boot injection", () => {
      expect(playgroundPlayBootHtmlPlugin().name).toBe("playground-play-boot-html");
    });

    it("exposes inline theme and reveal scripts", () => {
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("prefers-color-scheme");
      expect(PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT).toContain("semio-play-styles");
      expect(PLAYGROUND_PLAY_BOOT_INLINE_STYLE).toContain("data-semio-styled");
    });
  });

  describe("semioFaviconVitePlugin", () => {
    it("points at round dark emblem svg and ico under asset/logo", () => {
      const { svg, ico } = semioFaviconSources(repoRoot);
      expect(svg).toBe(resolve(repoRoot, "asset/logo/emblem_dark_round.svg"));
      expect(ico).toBe(resolve(repoRoot, "asset/logo/favicon_dark_round_32x32.ico"));
      expect(existsSync(svg)).toBe(true);
      expect(existsSync(ico)).toBe(true);
    });

    it("registers serve and build plugins", () => {
      const plugins = semioFaviconVitePlugin(repoRoot);
      expect(plugins.map((plugin) => plugin.name)).toEqual(["semio-favicon-serve", "semio-favicon-build"]);
    });

    it("injects opaque bleed into round dark favicon svg", () => {
      const { svg } = semioFaviconSources(repoRoot);
      const markup = semioFaviconSvgMarkup(svg);
      expect(markup).toContain('<rect width="350" height="350" fill="#001117"/>');
    });
  });

  describe("puzzle3dKitMeshRoots", () => {
    it("points at metabolism and abbau-aufbau kit glbs plus shared placeholder", () => {
      const { meshRoots, placeholderMesh } = puzzle3dKitMeshRoots(repoRoot);
      expect(existsSync(resolve(meshRoots[0]!, "capsule_J.glb"))).toBe(true);
      expect(existsSync(resolve(meshRoots[0]!, "capsule-with-balcony_slash.glb"))).toBe(true);
      expect(existsSync(resolve(meshRoots[1]!, "hexagonal-cut-concrete-forest-left.glb"))).toBe(true);
      expect(existsSync(placeholderMesh)).toBe(true);
    });
  });

  describe("createWorkspaceViteResolveConfig", () => {
    it("pins scene hosts and excludes workspace packages from optimizeDeps", () => {
      const config = createWorkspaceViteResolveConfig(repoRoot);
      expect(config.resolve?.dedupe).toContain("react");
      expect(config.resolve?.dedupe).toContain("three");
      expect(config.server?.fs?.allow).toContain(repoRoot);
      expect(config.optimizeDeps?.exclude).toContain("@semio-tech/flow-module-core");
    });
  });
}
//#endregion 🔖ViteElementsAssets
