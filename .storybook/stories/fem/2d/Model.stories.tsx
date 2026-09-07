// #region 🧲️Header
// 💻️ .storybook/stories/fem/2d/Model.stories.tsx
// Specs: Host the framework renderer's `📐️Canvas2dHost` for the fem2d play app's `fem2d-model` window
// (body key `fem2d.play.model`), driven by the REAL shipped example document
// (`🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`).
// Summary: The window's Rust `render` encodes a `Canvas2dScene` through
// `crate::app_surface::canvas_2d_surface` (`SurfaceKind::Canvas2d`), which the shell dispatches to
// `Canvas2dHost` — and `Canvas2dHost` builds its OWN `JsonLayersCanvasSession`, a pure
// `CanvasRenderingContext2D` implementation, so this canvas paints for real with no plugin wasm and no
// `WASM_LOADERS` entry (`.storybook/preview.tsx` has none for `canvas-2d`, and needs none). The layers are
// a line-for-line port of `fem2d_structure_layers` (`../scene.ts`); the one half that cannot be
// reproduced — the `mesh-edge-*` overlay from `fem2d_region_triangles` → `fem2d_mesh_preview` — is
// reported as a counted omission in the debug panel instead of being approximated.
// A story-local reducer mirrors fem2d's own tool roster (`setActiveExample`, `addNode`, `setLocale`) so the
// example switch and the dispatched `addNode` round-trip with no dev server.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Canvas2dHost } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import {
  buildFem2dSceneNode,
  fem2dStructureLayers,
  fem2dStoryStateFor,
  fem2dSummaryLines,
  FEM_STORY_LOCALES,
  FEM2D_MESH_PREVIEW_OMISSION,
  FEM2D_STORY_EXAMPLE_ID,
  femStoryLabel,
  femStoryOmissions,
  reduceFem2dStoryAction,
  type Fem2dStoryState,
  type FemStoryLocale,
} from "../scene";

//#region StoryHost
/** @emoji 🪟️ `WINDOW_KIND_ID` / `BODY_KEY` / `FEM2D_APP_ID` as the Rust window declares them. */
const FEM2D_MODEL_BODY_KEY = "fem2d.play.model";
const FEM2D_EDITOR_CONTROLLER_ID = "fem2d-play";
/** @emoji 🕳️ Any id other than the bundled example's own resets fem2d to an empty document. */
const FEM2D_CLEARED_EXAMPLE_ID = "none";

function Fem2dModelStoryHost({ initialExampleId, locale }: { readonly initialExampleId: string; readonly locale: FemStoryLocale }): ReactElement {
  const [state, setState] = useState<Fem2dStoryState>(() => fem2dStoryStateFor(initialExampleId, locale));
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setLastAction(descriptor);
    setState((current) => reduceFem2dStoryAction(current, descriptor.action, descriptor.args));
  }, []);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void => onAction({ controllerId: FEM2D_EDITOR_CONTROLLER_ID, action, args: { surfaceId: FEM2D_MODEL_BODY_KEY, ...args } }),
    [onAction],
  );

  const layers = useMemo(() => fem2dStructureLayers(state.snapshot, "#38bdf8", "#94a3b8", "#f97316"), [state.snapshot]);
  const node = useMemo(() => buildFem2dSceneNode(layers, state.camera, FEM2D_MODEL_BODY_KEY, FEM2D_EDITOR_CONTROLLER_ID), [layers, state.camera]);
  const lines = useMemo(() => fem2dSummaryLines(state.snapshot, state.locale), [state.snapshot, state.locale]);
  const debug = useMemo(
    () => JSON.stringify({ exampleId: state.exampleId, locale: state.locale, camera: state.camera, layerCount: layers.length, nodeIds: state.snapshot.nodes.map((entry) => entry.id), omitted: femStoryOmissions([FEM2D_MESH_PREVIEW_OMISSION]), lastAction }),
    [state, layers, lastAction],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", flexWrap: "wrap", gap: 4, padding: 4 }}>
        <button type="button" data-testid="fem2d-load-example" onClick={() => dispatch("setActiveExample", { exampleId: FEM2D_STORY_EXAMPLE_ID })}>
          {femStoryLabel("loadExample", state.locale)}
        </button>
        <button type="button" data-testid="fem2d-clear-example" onClick={() => dispatch("setActiveExample", { exampleId: FEM2D_CLEARED_EXAMPLE_ID })}>
          {femStoryLabel("clearExample", state.locale)}
        </button>
        <button type="button" data-testid="fem2d-add-node" onClick={() => dispatch("addNode", { x: 4, y: 9 })}>
          {femStoryLabel("addNode", state.locale)}
        </button>
        {FEM_STORY_LOCALES.map((tag) => (
          <button key={tag} type="button" data-testid={`fem2d-set-locale-${tag}`} aria-pressed={state.locale === tag} onClick={() => dispatch("setLocale", { value: tag })}>
            {femStoryLabel("language", state.locale)}: {tag}
          </button>
        ))}
      </div>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }} data-testid="fem2d-model-canvas">
        <Canvas2dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="fem2d-model-window" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {lines.join("\n")}
      </pre>
      <pre data-testid="fem2d-model-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🏗️fem◻️2d/Model",
  component: Fem2dModelStoryHost,
  parameters: { layout: "fullscreen" },
  argTypes: { locale: { control: "inline-radio", options: FEM_STORY_LOCALES } },
  tags: ["autodocs"],
} satisfies Meta<typeof Fem2dModelStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🏗️ The bundled `demo` document — 12 nodes, 9 timber/steel beams, 4 supports and both load cases' red load vectors, in English. */
export const DemoEnglish: Story = {
  args: { initialExampleId: FEM2D_STORY_EXAMPLE_ID, locale: "en-US" },
};

/** 🇩🇪️ The same document with every panel label resolved through `de-DE` — the `setLocale` buttons dispatch fem2d's real `setLocale` tool, so switching back and forth round-trips through the reducer. */
export const DemoGerman: Story = {
  args: { initialExampleId: FEM2D_STORY_EXAMPLE_ID, locale: "de-DE" },
};

/** 🕳️ The `setActiveExample` fallback branch: any id other than `"demo"` loads `empty_fem2d_snapshot()`, so the canvas draws nothing until "Load example" is pressed. */
export const ClearedDocument: Story = {
  args: { initialExampleId: FEM2D_CLEARED_EXAMPLE_ID, locale: "en-US" },
};
