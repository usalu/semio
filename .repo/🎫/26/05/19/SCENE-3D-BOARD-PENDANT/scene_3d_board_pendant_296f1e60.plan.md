---
name: scene 3d board pendant
overview: "Build `@elements/scene` as the 3D counterpart of `@elements/board`: R3F-based React component with the same kinds, compatibility and connect mechanisms (Indirect / Connect / Proximity), swappable glb meshes, central object pool, chunking for infinite worlds, and relocate (Translate/Rotate/Scale) instead of drag. Ship a Nakagin play site at JSON parity with the board fixture by consuming checked-in `nakagin-capsule-tower.scene.json` (origins and quaternions already in three.js Y-up space). Semio is not a dependency of `@elements/scene`: a one-off bake script (ticket folder only, optional `@semio/js` there) generates that JSON once from repo fixture files; the scene library, play site, and tests never import semio."
todos:
  - id: bootstrap
    content: Create scene package.json, project.json, script.ts (dev/build/test only), vite.config.ts, index.html
    status: completed
  - id: runtime
    content: "Implement react/index.tsx: Scene, Object, Vortex, Magnet, Tie, Attraction with regions"
    status: completed
  - id: pool
    content: Implement MeshPool (useGLTF + InstancedMesh, refcounted) and Chunking (per-chunk groups, frustum + radius cull)
    status: completed
  - id: interact
    content: Implement Selection + Relocate (Translate/Rotate/Scale) + Connect/Indirect/Proximity compat checks against kindCatalogs
    status: completed
  - id: coords
    content: No semio-named helpers in scene; fixture stores final three.js origin/quaternion. Optional tiny pure `planeBasisToThreeJs` in scene only if unit tests need shared math without JSON (neutral naming, no semio types)
    status: completed
  - id: bake
    content: One-off script in active ticket folder (not under elements/scene) reads shallow design + kit JSON from repo paths, may use @semio/js flatten once, writes nakagin-capsule-tower.scene.json; run manually then commit JSON
    status: completed
  - id: play
    content: Build play site with @elements/ui shell, fixture shelf, selection inspector, relocate-mode toolbar
    status: completed
  - id: tests
    content: Add vitest specs (fixture roundtrip, pool, chunk, coord, connect) + playwright e2e for nakagin scene
    status: completed
isProject: false
---

# Scene 3D Board Pendant

## Terminology mapping

- Board → Scene
- Node → Object (+ `meshUrl` instead of `shape`/`iconKind`, `origin`+`orientation` instead of `x`/`y`)
- Edge → Tie (`source: "<objectId>:<vortexId>"` / `target`)
- Handle → Vortex (3D `position` + `direction` vec; keep `vortexKind` ≈ board `handleKind`)
- Wire → Attraction (compatibility + transient connect preview)
- Magnet (per-object snap geometry, same role as board)

Compatibility/connect rules reuse the board catalog format verbatim (`source`/`target`/`bidirectional`/`specificity`) so authors can copy them between board and scene with zero translation.

## Architecture

`./elements/client/lib/scene` (independent from board, no react-reconciler / Rust / WASM):

- [elements/client/lib/scene/react/index.tsx](elements/client/lib/scene/react/index.tsx) — declarative R3F components.
  - `//#region 🔖Kinds` shared prop types (mirrors `BoardCanvasProps` shape: `kindCatalogs`, `kindCompatibility`, `selectionMode`, `selectionTargets`, `lodZoomThresholds`, `onSelect`, `onConnect`, `onProximityConnect`, `onIndirectConnect`, `onRelocate`, …).
  - `//#region 🎬Scene` `<Scene>` = `<Canvas>` (R3F) + `<SceneRoot>` providing context, camera, lighting, chunking grid, fog.
  - `//#region 🧊Object` `<Object>` renders a pooled mesh via `meshUrl` + origin/quaternion. Children = `<Vortex>` (visual ring on a 3D position) and `<Magnet>` (snap surface).
  - `//#region 🪢Tie` `<Tie>` draws a 3D line between two vortex world positions (resolved via context registry).
  - `//#region 🧲Attraction` `<Attraction>` = transient connect wire shown during relocate-connect.
  - `//#region 🎯Selection` selection store + picking (R3F raycaster) honoring `selectionTargets`.
  - `//#region ✋Relocate` replaces board "drag". Modes: `Translate`, `Rotate`, `Scale` (DREI `<TransformControls>` per mode). Pointer-down on a selected object enters relocate; modifier or toolbar switches mode. Commit emits `onRelocate({mode, before, after})`. While translating, runs proximity/connect compat checks against neighboring vortices and emits `onProximityConnect` / `onConnect` / `onIndirectConnect` mirroring board semantics.
  - `//#region 🏊Pool` central `MeshPool` (Map<url, {gltf, geometries, materials, refCount}>) using `useGLTF.preload` + `InstancedMesh` per unique mesh — objects acquire/release on mount/unmount; identical `meshUrl` share one instanced draw call.
  - `//#region 🧱Chunking` world partitioned into fixed-size cubes (default 256 m). `useChunkVisibility` culls objects/ties outside camera frustum + LOD radius from camera. Each chunk owns its own `<group>` so React can mount/unmount per chunk; `onCamera` triggers reactive chunk visibility.
  - `//#region 🧾Fixture` `parseSceneFixtureV1` / `encodeSceneFixtureForDragV1` mirroring `parseBoardFixtureV1`.

- [elements/client/lib/scene/play/index.tsx](elements/client/lib/scene/play/index.tsx) + `index.html` + `vite.config.ts` — copy of board play harness shape (UI shell, fixture drag shelf, selection inspector, toolbar with Select/Translate/Rotate/Scale + Connect/Indirect/Proximity toggles, level provider) but mounting `<Scene>` and reading `nakagin-capsule-tower.scene.json`.

- [elements/client/lib/scene/package.json](elements/client/lib/scene/package.json) deps: `react`, `react-dom`, `three`, `@react-three/fiber`, `@react-three/drei`, `@elements/ui`, `lucide-react`. No Rust/wasm.

- [elements/client/lib/scene/project.json](elements/client/lib/scene/project.json) targets: `dev`, `build`, `test` calling `bun ./script.ts`. `SCENE_PLAY_PORT=6013` (dev), `6028` (test).

- [elements/client/lib/scene/script.ts](elements/client/lib/scene/script.ts) modeled on board's `script.ts` (no cargo/wasm step): `dev` → vite, `build` → vite build, `test` → vitest + playwright. **No `bake-*` subcommands** — fixture generation is not part of this package's surface.

- [elements/client/lib/scene/fixtures/meshes/](elements/client/lib/scene/fixtures/meshes/) — optional empty placeholder; play `vite.config.ts` aliases `/meshes/*` to repo `semio/assets/fixtures/metabolism/representations/*.glb` via `server.fs.allow` (static files only, no semio JavaScript).

## Coordinate system (fixture authoring only, not scene runtime)

Checked-in `nakagin-capsule-tower.scene.json` stores `origin` and `orientation` already in **three.js** Y-up, right-handed space. The one-off bake tool (ticket folder) may read design JSON whose planes use a different authoring basis; that tool applies `(x, y, z)_authoring → (x, z, -y)_three` and `Matrix4.makeBasis` → quaternion **once** when writing JSON. `@elements/scene` only reads numbers from JSON — it does not implement or import that conversion path.

## Scene fixture schema (parity with board)

```json
{
  "schema": "elements.scene.fixture/v1",
  "camera": { "position": [x,y,z], "target": [x,y,z], "zoom": 1 },
  "kindCatalogs": { /* same shape as board: vortices/attractions/objects/ties + compat */ },
  "ties": [{ "id", "source": "<objectId>:<vortexId>", "target": "<objectId>:<vortexId>" }],
  "objects": [{
    "id", "label", "meshUrl", "objectKind", "scale",
    "origin": [x,y,z], "orientation": [qx,qy,qz,qw],
    "vortices": [{ "id", "vortexKind", "position": [x,y,z], "direction": [x,y,z], "radius" }]
  }]
}
```

Keys, ids, kindCatalogs and tie wiring are 1:1 with `nakagin-capsule-tower.board.json` so the same Nakagin compat catalog drives both surfaces.

## Strict isolation from semio

`@elements/scene` (runtime + play site + tests + `script.ts`) has **zero** semio dependency: no `@semio/*` in [elements/client/lib/scene/package.json](elements/client/lib/scene/package.json), no imports from `@semio/*` or `semio/` TypeScript modules anywhere under `elements/client/lib/scene/`.

- Play site only `import`s checked-in JSON and loads glbs by URL; `vite.config.ts` `server.fs.allow` may include the repo path to `semio/assets/.../representations/` **for static `.glb` files only** (not for executing semio code).

## One-time Nakagin fixture bake (ticket folder only, outside `elements/scene`)

Authoring `nakagin-capsule-tower.scene.json` is **not** implemented inside `@elements/scene`. It is a disposable script kept **only** in the active ticket folder (per repo rules: temp tooling lives under `.repo/🎫/.../`), for example `.repo/🎫/YY/MM/DD/<slug>/bake-nakagin-scene.mts`.

That script may:

1. `readFileSync` shallow design + light kit JSON from `semio/assets/fixtures/...` (plain paths, no `@elements/scene` import required).
2. `import { flattenDesign } from "@semio/js"` (or equivalent single entry the monorepo already resolves for one-off dev runs) to flatten pieces and connector geometry.
3. Apply authoring-basis → three.js conversion locally inside the script (inline functions, ~15 lines of `three` math, or `import` from `three` only).
4. `readFileSync` `.storybook/fixtures/nakagin-capsule-tower.board.json` and copy `kindCatalogs` + `edges` into the scene fixture for id parity.
5. `writeFileSync` → [elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json](elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json).

Run once with `bun .repo/🎫/.../bake-nakagin-scene.mts`, then commit the JSON. Nothing under `elements/client/lib/scene` imports or re-exports that script.

## Pool, chunking, relocate details

- Pool: `MeshPool` uses `useGLTF` to load each unique `meshUrl` once; objects of the same kind share an `InstancedMesh`. Acquire/release driven by mount/unmount; LRU eviction on chunk unload.
- Chunking: world `chunkSize` is a scene prop (default 256). `<SceneChunks>` groups objects by `floor(origin / chunkSize)`. Per-frame visibility test via `camera.frustum` + max-distance ring; offscreen chunks unmount their `<Object>`s (releasing pool refs).
- Relocate: `useRelocate(objectId)` returns `{mode, setMode, start, update, commit, cancel}`. Translate uses DREI `TransformControls mode="translate"`; rotate/scale analogous. During translate, every frame compares each magnet's world AABB against neighboring magnets via the chunk's spatial index (proximity radius from `kindCatalogs`) and fires `onProximityConnect` on snap, `onConnect` on release-over-vortex, `onIndirectConnect` on follow-up click on indirect ring.

## Tests

Extend (do not create new) [elements/client/lib/board/vitest.config.ts](elements/client/lib/board/vitest.config.ts) sibling — i.e. add a single `vitest.config.ts` in `scene/` mirroring board's and add specs alongside `react/index.tsx` and `play/index.tsx` for:
- fixture round-trip (`parseSceneFixtureV1` ↔ JSON parity with board ids),
- pool acquire/release reference counting,
- chunk visibility cull math,
- relocate translate → proximity-connect emits `onProximityConnect` with the right tie payload.

Playwright play-site e2e: load Nakagin scene, assert glb meshes render, click an object, translate it, drop on a compatible vortex → assert `onConnect` callback observed via instrumentation div.

## Code blueprints

### Public types ([elements/client/lib/scene/react/index.tsx](elements/client/lib/scene/react/index.tsx))

```ts
//#region 🔖Kinds
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export type SceneRelocateMode = "translate" | "rotate" | "scale";
export type SceneSelectionMode = "single" | "additive" | "subtractive" | "toggle";
export type SceneConnectKind = "indirect" | "connect" | "proximity";

export interface SceneCameraState {
  position: Vec3;
  target: Vec3;
  zoom: number;
}

export interface SceneVortexProps {
  id: string;
  vortexKind?: string;
  position: Vec3;
  direction?: Vec3;
  radius?: number;
  visible?: boolean;
}

export interface SceneMagnetProps {
  id: string;
  magnetKind?: string;
  position: Vec3;
  orientation?: Quat;
  size: Vec3;
}

export interface SceneObjectProps {
  id: string;
  objectKind?: string;
  meshUrl: string;
  origin: Vec3;
  orientation?: Quat;
  scale?: number | Vec3;
  label?: string;
  selected?: boolean;
  visible?: boolean;
  relocate?: SceneRelocateMode | false;
  children?: ReactNode;
  userData?: Record<string, unknown>;
}

export interface SceneTieProps {
  id: string;
  source: `${string}:${string}`;
  target: `${string}:${string}`;
  tieKind?: string;
}

export interface SceneCanvasProps {
  camera?: Partial<SceneCameraState>;
  chunkSize?: number;                            // default 256
  kindCatalogs?: SceneKindCatalogBundle;         // identical shape to BoardKindCatalogBundle
  kindCompatibility?: readonly SceneKindCompatEntry[];
  proximityRadius?: number;                      // default 0.5
  relocateMode?: SceneRelocateMode;
  selectionMode?: SceneSelectionMode;
  onCamera?: (s: SceneCameraState) => void;
  onSelect?: (snap: SceneSelectionSnapshot) => void;
  onRelocate?: (p: SceneRelocatePayload) => void;
  onConnect?: (p: SceneTieLinkPayload) => void;
  onIndirectConnect?: (p: SceneTieLinkPayload) => void;
  onProximityConnect?: (p: SceneTieLinkPayload) => void;
  children?: ReactNode;
}
//#endregion 🔖Kinds
```

### Scene root + chunking

```tsx
//#region 🎬Scene
export const Scene = ({ children, camera, chunkSize = 256, ...rest }: SceneCanvasProps) => {
  return (
    <Canvas dpr={[1, 2]} gl={{ antialias: true }}>
      <SceneProvider value={useSceneStore(rest)}>
        <PerspectiveCamera makeDefault position={camera?.position ?? [50, 50, 50]} />
        <CameraControls onCamera={rest.onCamera} />
        <ambientLight intensity={0.4} />
        <directionalLight position={[100, 200, 100]} intensity={0.8} />
        <SceneChunks chunkSize={chunkSize}>{children}</SceneChunks>
      </SceneProvider>
    </Canvas>
  );
};
//#endregion 🎬Scene

//#region 🧱Chunking
const chunkKey = (origin: Vec3, size: number) =>
  `${Math.floor(origin[0] / size)}|${Math.floor(origin[1] / size)}|${Math.floor(origin[2] / size)}`;

export const SceneChunks = ({ chunkSize, children }: { chunkSize: number; children: ReactNode }) => {
  const buckets = useMemo(() => {
    const map = new Map<string, ReactNode[]>();
    Children.forEach(children, (child) => {
      if (!isValidElement<SceneObjectProps>(child)) return;
      const k = chunkKey(child.props.origin, chunkSize);
      (map.get(k) ?? map.set(k, []).get(k)!).push(child);
    });
    return map;
  }, [children, chunkSize]);

  const visible = useVisibleChunks(buckets.keys(), chunkSize); // frustum + radius cull
  return (
    <>
      {[...buckets].map(([k, items]) =>
        visible.has(k) ? <group key={k} userData={{ chunk: k }}>{items}</group> : null,
      )}
    </>
  );
};
//#endregion 🧱Chunking
```

### Mesh pool (central, ref-counted, instanced)

```ts
//#region 🏊Pool
interface PoolEntry { gltf: GLTF; refCount: number; instanced?: InstancedMesh }
const pool = new Map<string, PoolEntry>();

export const useMesh = (url: string): Object3D | null => {
  const gltf = useGLTF(url);                                // suspends until loaded
  useEffect(() => {
    const e = pool.get(url) ?? { gltf, refCount: 0 };
    e.refCount += 1;
    pool.set(url, e);
    return () => {
      const cur = pool.get(url); if (!cur) return;
      cur.refCount -= 1;
      if (cur.refCount <= 0) { useGLTF.clear(url); pool.delete(url); }
    };
  }, [url]);
  return gltf?.scene ?? null;
};

useGLTF.preload = useGLTF.preload ?? (() => {});             // satisfies tree-shake
//#endregion 🏊Pool
```

`<Object>` consumes the pool entry; objects sharing a `meshUrl` are grouped into one `<Instances>` per chunk via DREI `<Instances>/<Instance>` so each unique mesh issues a single draw call per chunk.

### Coordinate conversion (not in `@elements/scene`)

Plane → three.js basis conversion lives **only** in the ticket `bake-nakagin-scene.mts` as private inline helpers (or `import` from `three` there). Do not export `semio*` or `SemioPlane` from [elements/client/lib/scene/react/index.tsx](elements/client/lib/scene/react/index.tsx).

### Relocate + connect (replaces drag)

```tsx
//#region ✋Relocate
export const useRelocate = (objectId: string) => {
  const store = useSceneStore();
  const mode = store.relocateModeFor(objectId);
  const start = useCallback((m: SceneRelocateMode) => store.beginRelocate(objectId, m), [objectId]);
  const update = (next: { origin?: Vec3; orientation?: Quat; scale?: Vec3 }) =>
    store.updateRelocate(objectId, next);
  const commit = () => {
    const cand = store.findBestConnectCandidate(objectId); // proximity radius + compat catalog
    if (cand) {
      store.emit("proximityConnect", cand);                 // 🧲 snap on release
      store.snapObjectToVortex(objectId, cand);
    }
    store.endRelocate(objectId);
  };
  return { mode, start, update, commit, cancel: () => store.cancelRelocate(objectId) };
};
//#endregion ✋Relocate
```

```tsx
//#region 🪝Object + TransformControls
export const Object = (props: SceneObjectProps) => {
  const mesh = useMesh(props.meshUrl);
  const ref = useRef<Group>(null!);
  const { mode, start, update, commit } = useRelocate(props.id);
  return (
    <>
      <group
        ref={ref}
        position={props.origin}
        quaternion={props.orientation}
        scale={props.scale}
        onClick={(e) => (e.stopPropagation(), start(props.relocate || "translate"))}
        userData={{ sceneObjectId: props.id }}
      >
        {mesh && <primitive object={mesh.clone()} />}
        {props.children /* vortices, magnets */}
      </group>
      {props.selected && mode && (
        <TransformControls
          object={ref}
          mode={mode}
          onObjectChange={() => update({
            origin: ref.current.position.toArray() as Vec3,
            orientation: ref.current.quaternion.toArray() as Quat,
          })}
          onMouseUp={commit}
        />
      )}
    </>
  );
};
//#endregion 🪝Object
```

### Compatibility lookup (parity with board)

```ts
//#region 🧩Compat
export const isCompatible = (
  a: { kind?: string }, b: { kind?: string },
  table: readonly SceneKindCompatEntry[],
) => table.some((e) =>
  (e.source === a.kind && e.target === b.kind) ||
  (e.bidirectional && e.source === b.kind && e.target === a.kind),
);
//#endregion 🧩Compat
```

### Fixture I/O (1:1 with board ids)

```ts
//#region 🧾Fixture
export const SCENE_FIXTURE_DRAG_V1_MIME = "application/x-elements-scene-fixture+json;v=1";
export interface SceneFixtureV1 {
  schema: "elements.scene.fixture/v1";
  camera: SceneCameraState;
  kindCatalogs: SceneKindCatalogBundle;
  ties: SceneTieProps[];
  objects: Array<SceneObjectProps & { vortices: SceneVortexProps[]; magnets?: SceneMagnetProps[] }>;
}
export const parseSceneFixtureV1 = (raw: unknown): SceneFixtureV1 => { /* schema-guarded parse */ };
export const encodeSceneFixtureForDragV1 = (f: SceneFixtureV1) => JSON.stringify(f);
//#endregion 🧾Fixture
```

### Ticket-only bake script blueprint (`.repo/🎫/.../bake-nakagin-scene.mts`)

Not part of `@elements/scene`. May use `@semio/js` and `three` here only. Writes into `elements/client/lib/scene/fixtures/` as the single allowed cross-folder write from the ticket.

```ts
#!/usr/bin/env bun
// .repo/🎫/YY/MM/DD/<slug>/bake-nakagin-scene.mts — run once; never imported by elements
import { readFileSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";
import { Matrix4, Quaternion, Vector3 } from "three";
import { flattenDesign } from "@semio/js";

const repoRoot = join(import.meta.dir, "../../../../.."); // adjust depth to repo root
const repo = (p: string) => join(repoRoot, p);

const authoringPointToThree = (p: { x: number; y: number; z: number }) => [p.x, p.z, -p.y] as const;
const planeToThree = (plane: { origin: any; xAxis: any; yAxis: any }) => {
  const x = new Vector3(...authoringPointToThree(plane.xAxis));
  const y = new Vector3(...authoringPointToThree(plane.yAxis));
  const z = new Vector3().crossVectors(x, y).normalize();
  const o = authoringPointToThree(plane.origin);
  const q = new Quaternion().setFromRotationMatrix(new Matrix4().makeBasis(x, y, z));
  return { origin: o, orientation: [q.x, q.y, q.z, q.w] as const };
};

const design = JSON.parse(readFileSync(repo("semio/assets/fixtures/nakagin-capsule-tower.shallow.design.semio.json"), "utf8"));
const kit = JSON.parse(readFileSync(repo("semio/assets/fixtures/metabolism.kit.light.semio.json"), "utf8"));
const board = JSON.parse(readFileSync(repo(".storybook/fixtures/nakagin-capsule-tower.board.json"), "utf8"));
const flat = flattenDesign(design, kit);

const meshUrl = (typeId: string) => {
  const t = kit.types.find((x: { id: string }) => x.id === typeId);
  const rep = t.representations.find((r: { file: string }) => r.file.endsWith(".glb"));
  return `/meshes/${basename(rep.file)}`;
};

const objects = flat.pieces.map((p: any) => {
  const { origin, orientation } = planeToThree(p.pose.plane);
  return {
    id: p.id,
    objectKind: `semio.metabolism.light.node.${p.type.id}`,
    label: p.name,
    meshUrl: meshUrl(p.type.id),
    origin,
    orientation,
    vortices: p.type.connectors.map((c: any) => ({
      id: `${p.id}:${c.id}`,
      vortexKind: `semio.metabolism.light.handle.${c.id}`,
      position: authoringPointToThree(c.point),
      direction: authoringPointToThree(c.direction),
      radius: 0.3,
    })),
  };
});

const scene = {
  schema: "elements.scene.fixture/v1",
  camera: { position: [80, 80, 80], target: [0, 20, 0], zoom: 1 },
  kindCatalogs: board.kindCatalogs,
  ties: board.edges,
  objects,
};

writeFileSync(repo("elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json"), JSON.stringify(scene, null, 2));
```

### Play site skeleton ([elements/client/lib/scene/play/index.tsx](elements/client/lib/scene/play/index.tsx))

```tsx
import fixture from "../fixtures/nakagin-capsule-tower.scene.json";
import { Scene, Object as SObject, Tie, Vortex, parseSceneFixtureV1 } from "../react/index";

const App = () => {
  const f = useMemo(() => parseSceneFixtureV1(fixture), []);
  const [mode, setMode] = useState<SceneRelocateMode>("translate");
  return (
    <UI {...uiConfig}>
      <RelocateToolbar value={mode} onChange={setMode} />
      <Scene camera={f.camera} kindCatalogs={f.kindCatalogs} kindCompatibility={f.kindCatalogs.compat}
             relocateMode={mode}
             onConnect={(p) => console.log("[scene] connect", p)}
             onProximityConnect={(p) => console.log("[scene] proximity", p)}>
        {f.objects.map((o) => (
          <SObject key={o.id} {...o} relocate={mode}>
            {o.vortices.map((v) => <Vortex key={v.id} {...v} />)}
          </SObject>
        ))}
        {f.ties.map((t) => <Tie key={t.id} {...t} />)}
      </Scene>
    </UI>
  );
};
```

## Ticket flow

1. `ticket_open` "Extend Elements With Scene 3D" under the most appropriate goal (read `repo://goals` first).
2. Wire `package.json` workspace entry + nx project graph; run `bun install`.
3. Implement scene runtime in `react/index.tsx` (regions: Kinds, Scene, Object, Vortex, Magnet, Tie, Attraction, Pool, Chunking, Selection, Relocate, Fixture).
4. Add ticket-only `bake-nakagin-scene.mts`, run once with `bun`, commit `nakagin-capsule-tower.scene.json` (scene package never references semio).
5. Implement play site reusing the `@elements/ui` `UI` shell, toolbar, fixture shelf.
6. Cargo/wasm not needed; ensure `script.ts test` runs vitest + playwright green.
7. `ticket_close` with file list.