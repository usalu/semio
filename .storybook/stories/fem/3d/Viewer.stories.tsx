// #region 🧲️Header
// 💻️ .storybook/stories/fem/3d/Viewer.stories.tsx
// Specs: Host the framework renderer's `🌐️World3dHost` for the fem3d VIEWER's `fem3d-view-model` window
// (body key `fem3d.view.model`, controller `fem3d-view`), driven by the REAL shipped example document.
// Summary: The viewer's `render` (`🗿️artifacts/🧊️3d/…/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs`) is the
// read-only counterpart of the editor's model window — the exact same scene, rebuilt rather than imported
// (`policyViewerPurityBreaches` forbids reaching into the sibling editor), at a hardcoded
// `FemCamera::default()` since a viewer has `Config = NoConfig`. There is no results window on the viewer
// side at all, and no tool to dispatch, so this story wires no toolbar.
// 🕳️ Honest note on what the RUNTIME viewer paints: its `render` hands `world3d_scene` the literal `"[]"`
// for both `meshes_json` and `instances_json` and carries geometry only on `scene.snapshot`, the
// `live_visual` page lease — so a real viewer session with no reconciled lease paints an EMPTY world. This
// story renders the editor-side `fem3d_scene_parts` projection instead (what the geometry WOULD be), which
// is the assertable content; the lease path itself has no browser equivalent.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { World3dHost } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { buildFem3dSceneNode, fem3dStoryStateFor, fem3dStructuralInstances, fem3dSummaryLines, FEM_STORY_LOCALES, FEM3D_DEFAULT_CAMERA_JSON, FEM3D_SOLID_MESH_OMISSION, FEM3D_STORY_EXAMPLE_ID, FEM3D_STORY_SHIPPED_EXAMPLE_ID, femStoryOmissions, type FemStoryLocale } from "../scene";

//#region StoryHost
const FEM3D_VIEW_BODY_KEY = "fem3d.view.model";
const FEM3D_VIEW_CONTROLLER_ID = "fem3d-view";

/** @emoji 🕳️ What the runtime viewer window actually hands `world3d_scene` before the `live_visual` lease is applied. */
const FEM3D_RUNTIME_EMPTY_SCENE_PARTS = { meshesJson: "[]", instancesJson: "[]" };

function Fem3dViewerStoryHost({ initialExampleId, locale }: { readonly initialExampleId: string; readonly locale: FemStoryLocale }): ReactElement {
  const snapshot = useMemo(() => fem3dStoryStateFor(initialExampleId, locale).snapshot, [initialExampleId, locale]);
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((descriptor: ActionDescriptor): void => setLastAction(descriptor), []);

  const instances = useMemo(() => fem3dStructuralInstances(snapshot), [snapshot]);
  const node = useMemo(() => buildFem3dSceneNode(instances, FEM3D_DEFAULT_CAMERA_JSON, FEM3D_VIEW_BODY_KEY, FEM3D_VIEW_CONTROLLER_ID), [instances]);
  const debug = useMemo(
    () =>
      JSON.stringify({
        exampleId: initialExampleId,
        locale,
        instanceCount: instances.length,
        emitsMutations: false,
        runtimeSceneWithoutLiveVisualLease: FEM3D_RUNTIME_EMPTY_SCENE_PARTS,
        omitted: femStoryOmissions([FEM3D_SOLID_MESH_OMISSION]),
        lastAction,
      }),
    [initialExampleId, locale, instances, lastAction],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", minHeight: "24rem", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }} data-testid="fem3d-viewer-world">
        <World3dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="fem3d-viewer-window" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {fem3dSummaryLines(snapshot, locale).join("\n")}
      </pre>
      <pre data-testid="fem3d-viewer-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🏗️fem🧊️3d/Viewer",
  component: Fem3dViewerStoryHost,
  parameters: { layout: "fullscreen" },
  argTypes: { locale: { control: "inline-radio", options: FEM_STORY_LOCALES } },
  tags: ["autodocs"],
} satisfies Meta<typeof Fem3dViewerStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 👁️ The bundled fixture, read-only, at the hardcoded default camera. */
export const DefaultExampleEnglish: Story = {
  args: { initialExampleId: FEM3D_STORY_EXAMPLE_ID, locale: "en-US" },
};

/** 🇩🇪️ The same read-only render with German panel labels. */
export const DefaultExampleGerman: Story = {
  args: { initialExampleId: FEM3D_STORY_EXAMPLE_ID, locale: "de-DE" },
};

/** 🕳️ An empty document — the viewer paints nothing and has no `setActiveExample` tool of its own to recover with. */
export const EmptyDocument: Story = {
  args: { initialExampleId: FEM3D_STORY_SHIPPED_EXAMPLE_ID, locale: "en-US" },
};
