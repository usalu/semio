// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/🏗️fem/📖️stories/🎭️2d-viewer/🧪️.story.tsx
// Specs: Host the framework renderer's `📐️Canvas2dHost` for the fem2d VIEWER's `fem2d-view-model` window
// (body key `fem2d.view.model`, controller `fem2d-view`), driven by the REAL shipped example document.
// Summary: The viewer's `render` (`🗿️artifacts/◻️2d/…/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs`) is the
// read-only counterpart of the editor's model window: the same structure layers in the same colors, a
// HARDCODED `FemCamera::default()` (a viewer has no `Config`, so there is no persisted camera and no
// `setCamera`), no selection, no gumball and no results overlay. It emits no mutations by construction
// (`ViewEmit`), so this story wires NO dispatching toolbar at all — the language selector is a story
// argument only, because the viewer surface has no `setLocale` tool to dispatch into.
// Like every fem2d window it encodes `SurfaceKind::Canvas2d`, which `Canvas2dHost` paints through its own
// `JsonLayersCanvasSession` — no plugin wasm, no `WASM_LOADERS` entry needed. The `mesh-edge-*` overlay
// (`fem2d_mesh_preview`, Rust-only) is a counted omission in the debug panel.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Canvas2dHost } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import type { ActionDescriptor } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

import { buildFem2dSceneNode, fem2dStructureLayers, fem2dStoryStateFor, fem2dSummaryLines, FEM_STORY_LOCALES, FEM2D_DEFAULT_CAMERA, FEM2D_MESH_PREVIEW_OMISSION, FEM2D_STORY_EXAMPLE_ID, femStoryOmissions, type FemStoryLocale } from "../🧭️coordination/🧫️fixtures/🧫️scene/🟦️.ts";

//#region StoryHost
const FEM2D_VIEW_BODY_KEY = "fem2d.view.model";
const FEM2D_VIEW_CONTROLLER_ID = "fem2d-view";
const FEM2D_CLEARED_EXAMPLE_ID = "none";

function Fem2dViewerStoryHost({ initialExampleId, locale }: { readonly initialExampleId: string; readonly locale: FemStoryLocale }): ReactElement {
  const snapshot = useMemo(() => fem2dStoryStateFor(initialExampleId, locale).snapshot, [initialExampleId, locale]);
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((descriptor: ActionDescriptor): void => setLastAction(descriptor), []);

  const layers = useMemo(() => fem2dStructureLayers(snapshot, "#38bdf8", "#94a3b8", "#f97316"), [snapshot]);
  const node = useMemo(() => buildFem2dSceneNode(layers, FEM2D_DEFAULT_CAMERA, FEM2D_VIEW_BODY_KEY, FEM2D_VIEW_CONTROLLER_ID), [layers]);
  const debug = useMemo(
    () => JSON.stringify({ exampleId: initialExampleId, locale, camera: FEM2D_DEFAULT_CAMERA, layerCount: layers.length, emitsMutations: false, omitted: femStoryOmissions([FEM2D_MESH_PREVIEW_OMISSION]), lastAction }),
    [initialExampleId, locale, layers, lastAction],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }} data-testid="fem2d-viewer-canvas">
        <Canvas2dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="fem2d-viewer-window" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {fem2dSummaryLines(snapshot, locale).join("\n")}
      </pre>
      <pre data-testid="fem2d-viewer-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🏗️fem◻️2d/Viewer",
  component: Fem2dViewerStoryHost,
  parameters: { layout: "fullscreen" },
  argTypes: { locale: { control: "inline-radio", options: FEM_STORY_LOCALES } },
  tags: ["autodocs"],
} satisfies Meta<typeof Fem2dViewerStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 👁️ The bundled `demo` document, read-only, at the hardcoded default camera. */
export const DemoEnglish: Story = {
  args: { initialExampleId: FEM2D_STORY_EXAMPLE_ID, locale: "en-US" },
};

/** 🇩🇪️ The same read-only render with German panel labels. */
export const DemoGerman: Story = {
  args: { initialExampleId: FEM2D_STORY_EXAMPLE_ID, locale: "de-DE" },
};

/** 🕳️ An empty document — the viewer paints nothing and offers no way to load an example, since it has no `setActiveExample` tool of its own. */
export const EmptyDocument: Story = {
  args: { initialExampleId: FEM2D_CLEARED_EXAMPLE_ID, locale: "en-US" },
};
