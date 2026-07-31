// #region 🧲️Header
// 💻️ .storybook/framework/hosts/iconRenderFixture.ts
// Specs: Self-contained (no dev server, no `scopes.ts` asset route) glTF fixture for `IconRenderHost` stories.
// Summary: `IconRenderRequest.assetUrl` is normally a served GLB path; `framework/hosts` registers no static-dir asset route (unlike `framework/os`'s `/plugin-modules`), so this hand-builds a minimal valid glTF 2.0 JSON document (a two-triangle quad, embedded `POSITION`/indices buffer as its own `data:` URI) and base64-encodes the whole document as a second `data:` URI at call time — `GLTFLoader`'s `FileLoader` fetches both via the browser's native `data:` URL support (see `LoaderUtils.resolveURL`, which passes `data:` URIs through unresolved), so the real default `iconRenderPort` (three.js GLTFLoader + WebGL/SVG renderer, `framework/ui/js/react/index.tsx`) renders it with zero network/backend dependency.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

//#region QuadBuffer
/** 🔺️ Two-triangle unit quad: 4×vec3 float32 positions (48 bytes) + 6×uint16 indices (12 bytes), precomputed once (`node -e` Float32Array/Uint16Array → base64) since the bytes never change. */
const QUAD_BUFFER_BASE64 = "AAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAgD8AAAAAAACAPwAAgD8AAAAAAAABAAIAAQADAAIA";
const QUAD_BUFFER_BYTE_LENGTH = 60;
const QUAD_POSITION_BYTE_LENGTH = 48;
const QUAD_INDEX_BYTE_LENGTH = 12;
//#endregion QuadBuffer

//#region GltfDocument
/** 🧩️ Minimal valid glTF 2.0 document three.js's `GLTFLoader` can parse standalone (no `.bin` sibling — the buffer is itself a `data:` URI). */
function buildQuadGltfDocument(): Record<string, unknown> {
  return {
    asset: { version: "2.0", generator: "semio-storybook-icon-render-fixture" },
    scene: 0,
    scenes: [{ nodes: [0] }],
    nodes: [{ mesh: 0, name: "storyQuad" }],
    meshes: [{ name: "storyQuad", primitives: [{ attributes: { POSITION: 0 }, indices: 1, mode: 4 }] }],
    accessors: [
      { bufferView: 0, componentType: 5126, count: 4, type: "VEC3", min: [0, 0, 0], max: [1, 1, 0] },
      { bufferView: 1, componentType: 5123, count: 6, type: "SCALAR" },
    ],
    bufferViews: [
      { buffer: 0, byteOffset: 0, byteLength: QUAD_POSITION_BYTE_LENGTH, target: 34962 },
      { buffer: 0, byteOffset: QUAD_POSITION_BYTE_LENGTH, byteLength: QUAD_INDEX_BYTE_LENGTH, target: 34963 },
    ],
    buffers: [{ byteLength: QUAD_BUFFER_BYTE_LENGTH, uri: `data:application/octet-stream;base64,${QUAD_BUFFER_BASE64}` }],
  };
}

let cachedAssetUrl: string | null = null;

/** 🖼️ Lazily builds (and caches) the `data:model/gltf+json` URL a story can hand to `IconRenderRequest.assetUrl`. `btoa` is browser-only, matching where Storybook stories actually execute. */
export function iconRenderPlaceholderAssetUrl(): string {
  if (cachedAssetUrl) return cachedAssetUrl;
  const json = JSON.stringify(buildQuadGltfDocument());
  cachedAssetUrl = `data:model/gltf+json;base64,${btoa(json)}`;
  return cachedAssetUrl;
}
//#endregion GltfDocument
