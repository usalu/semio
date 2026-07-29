// #region 🧲Header
// 💻 .storybook/stories/cad/Renderer.stories.tsx
// Specs: Host `InteractionCanvas`/`InteractionSpatialView` from `@semio-tech/cad-js-renderer` against the real
// `primitive.box` interaction shipped at `cad/asset/modelDefinition/spatial.shape/interaction/box.json` — no
// `cad/plugin/rs` wasm exists yet (verified: only `Cargo.toml`/`lib.rs`, no `pkg/`), so this drives the pure-TS
// interaction state machine (`pureTsStateEngineProvider`) with a story-local `StoryBoxKernel` that copies every
// preview-math method off `r3fPreviewKernel` (`R3FPreviewKernel extends PreciseSpatialKernelMath`, see
// `cad/renderer/js/index.tsx`'s `⚡R3FPreviewKernel` region) via `Object.assign` and only overrides the four
// solid-producing members — the exact pattern the core test suite itself uses
// (`cad/core/js/index.ts` "runs box workflow with a recording kernel stub (no solid modeling in core)").
// Summary: `Idle` shows the fresh ground-pick plane; `CommittedBox` scripts corner → corner → height → confirm
// on mount (same coordinates as the passing core test) so a real committed box mesh renders through
// `useTessellation`, then leaves the ground-pick canvas and a manual toolbar (height, confirm, undo) live for
// further exploration.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

// #region 🔌Adapters
import type { Meta, StoryObj } from "@storybook/react";
import { useEffect, useMemo, useRef, useState, type ReactElement } from "react";
import { InteractionCanvas, InteractionSpatialView, r3fPreviewKernel, useInteractionRuntime, useInteractionSnapshot, useTessellation } from "@semio-tech/cad-js-renderer";
import { Model, loadSpatialInteraction, solidRef, type InteractionRuntimeOptions, type InteractionSpec, type MeshTransfer, type SpatialKernel, type Vec3 } from "@semio-tech/cad-js-core";
// #endregion 🔌Adapters

// #region 🧊StoryBoxKernel
type StoryBoxInput = { readonly cornerA: Vec3; readonly cornerB: Vec3; readonly height: number };

/** @emoji 📦 Builds a real (double-sided, so winding never hides it) box `MeshTransfer` purely from JS math — no OpenCascade wasm. */
function buildStoryBoxMesh({ cornerA, cornerB, height }: StoryBoxInput): MeshTransfer {
  const minX = Math.min(cornerA[0], cornerB[0]);
  const maxX = Math.max(cornerA[0], cornerB[0]);
  const minY = Math.min(cornerA[1], cornerB[1]);
  const maxY = Math.max(cornerA[1], cornerB[1]);
  const z0 = Math.min(cornerA[2], cornerA[2] + height);
  const z1 = Math.max(cornerA[2], cornerA[2] + height);
  const faces: { readonly verts: readonly Vec3[]; readonly normal: Vec3 }[] = [
    { verts: [[minX, minY, z0], [maxX, minY, z0], [maxX, maxY, z0], [minX, maxY, z0]], normal: [0, 0, -1] },
    { verts: [[minX, minY, z1], [minX, maxY, z1], [maxX, maxY, z1], [maxX, minY, z1]], normal: [0, 0, 1] },
    { verts: [[minX, minY, z0], [minX, maxY, z0], [minX, maxY, z1], [minX, minY, z1]], normal: [-1, 0, 0] },
    { verts: [[maxX, minY, z0], [maxX, minY, z1], [maxX, maxY, z1], [maxX, maxY, z0]], normal: [1, 0, 0] },
    { verts: [[minX, minY, z0], [minX, minY, z1], [maxX, minY, z1], [maxX, minY, z0]], normal: [0, -1, 0] },
    { verts: [[minX, maxY, z0], [maxX, maxY, z0], [maxX, maxY, z1], [minX, maxY, z1]], normal: [0, 1, 0] },
  ];
  const position: number[] = [];
  const normal: number[] = [];
  const index: number[] = [];
  faces.forEach((face, faceIndex) => {
    const base = faceIndex * 4;
    face.verts.forEach((v) => {
      position.push(v[0], v[1], v[2]);
      normal.push(face.normal[0], face.normal[1], face.normal[2]);
    });
    index.push(base, base + 1, base + 2, base, base + 2, base + 3, base + 2, base + 1, base, base + 3, base + 2, base);
  });
  return {
    position: new Float32Array(position),
    normal: new Float32Array(normal),
    index: new Uint32Array(index),
    edges: new Float32Array(0),
    faceGroups: [],
    edgeGroups: [],
    faceInfos: [],
    edgeInfos: [],
  };
}

/** @emoji 🧊 Story-local `SpatialKernel` stub: preview math inherited from `r3fPreviewKernel`, solid operations hand-authored — mirrors `RecordingStubKernel` in `cad/core/js/index.ts`'s interaction test suite. */
class StoryBoxKernel {
  readonly id = "story-box-kernel";
  readonly operations = ["solid.createBox", "entity.tessellate"] as const;
  lastBox: StoryBoxInput | null = null;

  constructor() {
    Object.assign(this, r3fPreviewKernel);
  }

  async createBoxFromCorners(input: StoryBoxInput): Promise<ReturnType<typeof solidRef>> {
    this.lastBox = input;
    return solidRef("story-box");
  }

  async createBoxFromCornersDiff(input: StoryBoxInput) {
    const solid = await this.createBoxFromCorners(input);
    return { diff: r3fPreviewKernel.boxModelDiff(input, solid), solid };
  }

  async volume(): Promise<number> {
    return 0;
  }

  async tessellate(): Promise<MeshTransfer> {
    return this.lastBox ? buildStoryBoxMesh(this.lastBox) : { position: new Float32Array(0), normal: new Float32Array(0), index: new Uint32Array(0), edges: new Float32Array(0), faceGroups: [], edgeGroups: [], faceInfos: [], edgeInfos: [] };
  }
}
// #endregion 🧊StoryBoxKernel

// #region 🎛️StoryHost
const BOX_INTERACTION_ID = "primitive.box";

function CadBoxInteractionStory({ autoRun }: { readonly autoRun: boolean }): ReactElement {
  const specRef = useRef<InteractionSpec | null>(null);
  if (!specRef.current) specRef.current = loadSpatialInteraction(BOX_INTERACTION_ID);
  const spec = specRef.current;

  const modelRef = useRef<Model | null>(null);
  if (!modelRef.current) modelRef.current = new Model();
  const kernelRef = useRef<StoryBoxKernel | null>(null);
  if (!kernelRef.current) kernelRef.current = new StoryBoxKernel();

  const opts = useMemo<InteractionRuntimeOptions>(
    () => ({ kernel: kernelRef.current as unknown as SpatialKernel, document: { model: modelRef.current!, nodes: [] } }),
    [],
  );

  if (!spec) {
    return <div style={{ padding: 16, fontSize: 13 }}>Missing spatial interaction fixture: {BOX_INTERACTION_ID}</div>;
  }

  return <CadBoxInteractionRuntimeHost spec={spec} opts={opts} kernel={kernelRef.current!} autoRun={autoRun} />;
}

function CadBoxInteractionRuntimeHost({ spec, opts, kernel, autoRun }: { readonly spec: InteractionSpec; readonly opts: InteractionRuntimeOptions; readonly kernel: StoryBoxKernel; readonly autoRun: boolean }): ReactElement {
  const rt = useInteractionRuntime(spec, opts);
  const snapshot = useInteractionSnapshot(rt);
  const [heightInput, setHeightInput] = useState(4);

  const solidId = (snapshot.lastResponse?.ok && (snapshot.lastResponse.data as { solid?: string } | null)?.solid) || null;
  const committedMesh = useTessellation(opts.kernel, solidId ? solidRef(solidId) : null, 0.1);

  const ranAutoRun = useRef(false);
  useEffect(() => {
    if (!autoRun || ranAutoRun.current) return;
    ranAutoRun.current = true;
    void (async () => {
      await rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await rt.send({ kind: "set.height", value: 4, modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
    })();
  }, [autoRun, rt]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <InteractionCanvas frameloop="always">
          <InteractionSpatialView
            snapshot={snapshot}
            onGroundPick={() => {}}
            onScenePointerMove={() => {}}
            onInteractionEvent={(event) => void rt.send(event)}
            committedMesh={committedMesh}
            geometry={opts.document.model}
            autoFitMeshes
            autoFitBehavior="changes"
          />
        </InteractionCanvas>
      </div>
      <div style={{ width: 240, flex: "0 0 auto", padding: 12, fontSize: 12, display: "flex", flexDirection: "column", gap: 8, borderLeft: "1px solid rgba(128,128,128,0.3)", overflowY: "auto" }}>
        <div>
          <strong>primitive.box</strong>
          <div>state: {snapshot.state}</div>
          <div>revision: {snapshot.revision}</div>
          <div>canUndo: {String(snapshot.capabilities.canUndo)}</div>
          <div>solid: {kernel.lastBox ? solidId ?? "(pending)" : "(none)"}</div>
        </div>
        <label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
          height
          <input type="number" step={0.1} value={heightInput} onChange={(e) => setHeightInput(Number(e.target.value))} />
        </label>
        <button type="button" onClick={() => void rt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} })}>
          corner A (0,0,0)
        </button>
        <button type="button" onClick={() => void rt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} })}>
          corner B (2,3,0)
        </button>
        <button type="button" onClick={() => void rt.send({ kind: "set.height", value: heightInput, modifiers: {} })}>
          set height
        </button>
        <button type="button" disabled={!snapshot.capabilities.canCommit} onClick={() => void rt.send({ kind: "confirm", modifiers: {} })}>
          confirm
        </button>
        <button type="button" disabled={!snapshot.capabilities.canUndo} onClick={() => rt.undo()}>
          undo
        </button>
        <pre data-testid="cad-box-debug" style={{ margin: 0, whiteSpace: "pre-wrap", opacity: 0.7 }}>
          {JSON.stringify({ state: snapshot.state, lastBox: kernel.lastBox })}
        </pre>
      </div>
    </div>
  );
}
// #endregion 🎛️StoryHost

const meta = {
  title: "📐cad",
  component: CadBoxInteractionStory,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof CadBoxInteractionStory>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Idle: Story = {
  args: { autoRun: false },
};

export const CommittedBox: Story = {
  args: { autoRun: true },
};
