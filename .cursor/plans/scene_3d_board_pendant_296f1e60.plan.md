---
name: scene 3d board pendant
overview: "Build `@elements/scene` as the 3D counterpart of `@elements/board`: R3F-based React component with the same kinds, compatibility and connect mechanisms (Indirect / Connect / Proximity), swappable glb meshes, central object pool, chunking for infinite worlds, and relocate (Translate/Rotate/Scale) instead of drag. Ship a Nakagin play site at JSON parity with the board fixture, fed by a baked `nakagin-capsule-tower.scene.json` produced from the flattened semio design with planes converted into three.js (Y-up, right-handed) origins + orientation quaternions."
todos:
  - id: bootstrap
    content: Create scene package.json, project.json, script.ts (dev/build/test/bake-nakagin), vite.config.ts, index.html
    status: pending
  - id: runtime
    content: "Implement react/index.tsx: Scene, Object, Vortex, Magnet, Tie, Attraction with regions"
    status: pending
  - id: pool
    content: Implement MeshPool (useGLTF + InstancedMesh, refcounted) and Chunking (per-chunk groups, frustum + radius cull)
    status: pending
  - id: interact
    content: Implement Selection + Relocate (Translate/Rotate/Scale) + Connect/Indirect/Proximity compat checks against kindCatalogs
    status: pending
  - id: coords
    content: Implement semio-plane → three (Y-up) origin + quaternion conversion helper with golden test
    status: pending
  - id: bake
    content: Write bake-nakagin script that flattens shallow design + light kit, maps glbs, writes scene fixture with board-parity ids
    status: pending
  - id: play
    content: Build play site with @elements/ui shell, fixture shelf, selection inspector, relocate-mode toolbar
    status: pending
  - id: tests
    content: Add vitest specs (fixture roundtrip, pool, chunk, coord, connect) + playwright e2e for nakagin scene
    status: pending
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

- [elements/client/lib/scene/script.ts](elements/client/lib/scene/script.ts) modeled on board's `script.ts` (no cargo/wasm step): `dev` → vite, `build` → vite build, `test` → vitest + playwright.

- [elements/client/lib/scene/fixtures/meshes/](elements/client/lib/scene/fixtures/meshes/) — `package.json`-free static folder; play `vite.config.ts` aliases `/meshes/*` to `semio/assets/fixtures/metabolism/representations/*` via `server.fs.allow` (same pattern board uses for fixture json).

## Coordinate system

Semio plane `(origin, xAxis, yAxis)` is right-handed with `zAxis = xAxis × yAxis`. Three.js uses Y-up, right-handed. Apply the mapping `(x, y, z)_semio → (x, z, -y)_three` (rotate -90° about world X) consistently for origin, axes and derived quaternion. Quaternion is built from the rotated basis (`new Matrix4().makeBasis(x', y', z')` → `Quaternion.setFromRotationMatrix`).

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

`@elements/scene` (runtime + play site + tests) has **zero** semio dependency. It is a pure R3F React component package living under `./elements` and consumes only JSON. The Nakagin fixture is checked-in JSON — the play site `import`s it directly and never touches `@semio/*`.

- `package.json` MUST NOT list `@semio/js`, `@semio/react`, `@semio/rs`, or any other `@semio/*` package.
- `react/index.tsx` MUST NOT import from `@semio/*`.
- `play/index.tsx` MUST NOT import from `@semio/*`.
- `play/vite.config.ts` `server.fs.allow` only needs the elements root + `semio/assets/fixtures/metabolism/representations/` for static `.glb` serving (binary assets, no code).
- glb assets are referenced by URL only (`/meshes/<name>.glb` alias) — they are data, not a code dependency.

The only place semio is used is the offline bake tool (see next section), which lives in its own folder, is not part of the published surface and is not imported by anything in `scene/react`, `scene/play`, or `scene/fixtures`.

## Baking the Nakagin scene fixture (offline tool, isolated)

One-off offline bake to **author** the fixture once. Output JSON is checked in; the scene package never re-runs it. Re-bake only when the board fixture is regenerated.

- Lives in [elements/client/lib/scene/fixtures/bake/](elements/client/lib/scene/fixtures/bake/) — a sibling tool folder, not part of the scene runtime build.
  - `bake.ts` — the bake program. It is the only file in the repo allowed to mix `@semio/js` with scene types.
  - `bake.package.json` is not needed; it's invoked through a top-level dev script (see below) using the repo's hoisted `@semio/js`.
- Invoked via `bun ./script.ts bake-nakagin` in [elements/client/lib/scene/script.ts](elements/client/lib/scene/script.ts). The `bake-nakagin` subcommand is gated behind a `if (command === "bake-nakagin")` branch that `await import`s `./fixtures/bake/bake.ts` lazily so production `dev`/`build`/`test` paths never load semio.
- The bake performs:
  1. Load `semio/assets/fixtures/nakagin-capsule-tower.shallow.design.semio.json` + `semio/assets/fixtures/metabolism.kit.light.semio.json`.
  2. `flattenDesign(design, kit)` (existing JS impl, see `semio/client/lib/react/rendering/index.tsx:4839`) to materialize per-piece planes + connector world points.
  3. For each piece → emit a scene `object` using the same `id` as in the board fixture; vortex ids are `<pieceId>:<connectorId>` to match board tie endpoints exactly.
  4. Resolve `meshUrl` from each type's `.glb` representation → `/meshes/<filename>.glb` (URL string only).
  5. Convert each plane via `semioPlaneToThree` → `{ origin, orientation }`.
  6. Copy `kindCatalogs` + `edges` directly from the board fixture (ids align). Write `elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json`.

After bake completes, you can `rm -rf elements/client/lib/scene/fixtures/bake/node_modules` — runtime does not need any of it.

## Pool, chunking, relocate details

- Pool: `MeshPool` uses `useGLTF` to load each unique `meshUrl` once; objects of the same kind share an `InstancedMesh`. Acquire/release driven by mount/unmount; LRU eviction on chunk unload.
- Chunking: world `chunkSize` is a scene prop (default 256). `<SceneChunks>` groups objects by `floor(origin / chunkSize)`. Per-frame visibility test via `camera.frustum` + max-distance ring; offscreen chunks unmount their `<Object>`s (releasing pool refs).
- Relocate: `useRelocate(objectId)` returns `{mode, setMode, start, update, commit, cancel}`. Translate uses DREI `TransformControls mode="translate"`; rotate/scale analogous. During translate, every frame compares each magnet's world AABB against neighboring magnets via the chunk's spatial index (proximity radius from `kindCatalogs`) and fires `onProximityConnect` on snap, `onConnect` on release-over-vortex, `onIndirectConnect` on follow-up click on indirect ring.

## Tests

Extend (do not create new) [elements/client/lib/board/vitest.config.ts](elements/client/lib/board/vitest.config.ts) sibling — i.e. add a single `vitest.config.ts` in `scene/` mirroring board's and add specs alongside `react/index.tsx` and `play/index.tsx` for:
- fixture round-trip (`parseSceneFixtureV1` ↔ JSON parity with board ids),
- pool acquire/release reference counting,
- chunk visibility cull math,
- relocate translate → proximity-connect emits `onProximityConnect` with the right tie payload,
- coordinate conversion (plane → three quaternion) golden values.

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

### Coordinate conversion (semio plane → three)

```ts
//#region 📐Coords
import { Matrix4, Quaternion, Vector3 } from "three";

export const semioPointToThree = (p: { x: number; y: number; z: number }): Vec3 => [p.x, p.z, -p.y];
export const semioVectorToThree = semioPointToThree;

/** 🧭 Semio plane (Z-up RH) → three (Y-up RH) origin + quaternion. */
export const semioPlaneToThree = (plane: SemioPlane): { origin: Vec3; orientation: Quat } => {
  const x = new Vector3(...semioVectorToThree(plane.xAxis));
  const y = new Vector3(...semioVectorToThree(plane.yAxis));
  const z = new Vector3().crossVectors(x, y).normalize();
  const o = semioPointToThree(plane.origin);
  const q = new Quaternion().setFromRotationMatrix(new Matrix4().makeBasis(x, y, z));
  return { origin: o, orientation: [q.x, q.y, q.z, q.w] };
};
//#endregion 📐Coords
```

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

### Bake-nakagin (offline tool, lives in [elements/client/lib/scene/fixtures/bake/bake.ts](elements/client/lib/scene/fixtures/bake/bake.ts))

This is the only file in the scene tree that touches semio. It is invoked manually via `bun ./script.ts bake-nakagin`; the scene runtime, play site and tests do not import it.

```ts
// fixtures/bake/bake.ts — offline only. NEVER imported by scene/react, scene/play, or scene tests.
import { readFileSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";
import { flattenDesign } from "@semio/js";                         // 🚧 offline-only import
import { semioPlaneToThree, semioPointToThree, semioVectorToThree, type SceneFixtureV1 } from "../../react/index";

const repo = (p: string) => join(import.meta.dir, "../../../../../..", p);

export const bakeNakaginScene = () => {
  const design = JSON.parse(readFileSync(repo("semio/assets/fixtures/nakagin-capsule-tower.shallow.design.semio.json"), "utf8"));
  const kit    = JSON.parse(readFileSync(repo("semio/assets/fixtures/metabolism.kit.light.semio.json"), "utf8"));
  const board  = JSON.parse(readFileSync(repo(".storybook/fixtures/nakagin-capsule-tower.board.json"), "utf8"));
  const flat   = flattenDesign(design, kit);

  const meshUrl = (typeId: string) => {
    const t = kit.types.find((x: any) => x.id === typeId);
    const rep = t.representations.find((r: any) => r.file.endsWith(".glb"));
    return `/meshes/${basename(rep.file)}`;
  };

  const objects = flat.pieces.map((p: any) => {
    const { origin, orientation } = semioPlaneToThree(p.pose.plane);
    return {
      id: p.id, objectKind: `semio.metabolism.light.node.${p.type.id}`,
      label: p.name, meshUrl: meshUrl(p.type.id), origin, orientation,
      vortices: p.type.connectors.map((c: any) => ({
        id: `${p.id}:${c.id}`, vortexKind: `semio.metabolism.light.handle.${c.id}`,
        position: semioPointToThree(c.point), direction: semioVectorToThree(c.direction), radius: 0.3,
      })),
    };
  });

  const scene: SceneFixtureV1 = {
    schema: "elements.scene.fixture/v1",
    camera: { position: [80, 80, 80], target: [0, 20, 0], zoom: 1 },
    kindCatalogs: board.kindCatalogs,                              // 🪞 reuse 1:1 (ids match)
    ties: board.edges,                                             // 🪞 source/target ids identical
    objects,
  };
  writeFileSync(repo("elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json"), JSON.stringify(scene, null, 2));
};

if (import.meta.main) bakeNakaginScene();
```

In `scene/script.ts` the gate stays lazy so semio is only resolved when explicitly baking:

```ts
if (command === "bake-nakagin") {
  const { bakeNakaginScene } = await import("./fixtures/bake/bake.ts");
  bakeNakaginScene();
  process.exit(0);
}
```

Note: `semioPlaneToThree` / `semioPointToThree` / `semioVectorToThree` are pure math helpers exported from `scene/react/index.tsx` (no semio runtime — just three.js + numbers). The bake tool reuses them so the conversion lives in exactly one place.

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
4. Implement bake script + run it to produce `nakagin-capsule-tower.scene.json`.
5. Implement play site reusing the `@elements/ui` `UI` shell, toolbar, fixture shelf.
6. Cargo/wasm not needed; ensure `script.ts test` runs vitest + playwright green.
7. `ticket_close` with file list.