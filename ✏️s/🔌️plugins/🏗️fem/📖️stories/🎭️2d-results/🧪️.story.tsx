// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/🏗️fem/📖️stories/🎭️2d-results/🧪️.story.tsx
// Specs: Host the framework renderer's `📐️Canvas2dHost` for the fem2d play app's `fem2d-results` window
// (body key `fem2d.play.results`), driven by the REAL shipped example document and by the same
// `Fem2dConfig` result-display triple (`result_source_id`/`result_mode`/`result_mode_index`) the window's
// Rust `render` dispatches on.
// Summary: `render_static`/`render_modal`/`render_buckling` all begin with the faint undeformed backdrop
// `fem2d_structure_layers(doc, "#334155", "#334155", "#334155")`, which this story reproduces exactly, and
// then add layers derived from `crate::fem2d_engine::fem2d_solve_all` — a Rust-only solver with no browser
// build, so the deformed-shape polylines, reaction labels, moment diagram and von-Mises contour bands are
// COUNTED OMISSIONS in the debug panel, never faked. The `render_static` guard branch is reproduced for
// real: with no load case the Rust returns a bare `built_text_node(Label::data("No load case defined"))` —
// no canvas surface at all — so `NoLoadCase` renders that text node and mounts no host, matching the shape
// the shell would receive.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Canvas2dHost } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import type { ActionDescriptor } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

import {
  buildFem2dSceneNode,
  fem2dStructureLayers,
  fem2dStoryStateFor,
  fem2dSummaryLines,
  femResolveResultCase,
  femResultCaptionLine,
  FEM_STORY_LOCALES,
  FEM2D_SOLVER_OMISSION,
  FEM2D_MESH_PREVIEW_OMISSION,
  FEM2D_STORY_EXAMPLE_ID,
  femStoryLabel,
  femStoryOmissions,
  reduceFem2dStoryAction,
  type Fem2dStoryState,
  type FemStoryLocale,
} from "../🧭️coordination/🧫️fixtures/🧫️scene/🟦️.ts";

//#region StoryHost
const FEM2D_RESULTS_BODY_KEY = "fem2d.play.results";
const FEM2D_EDITOR_CONTROLLER_ID = "fem2d-play";
const FEM2D_CLEARED_EXAMPLE_ID = "none";

/** @emoji 👁️ The three `DisplayMode` discriminants `config_result_display` maps `Fem2dConfig::result_mode` onto. */
const FEM2D_RESULT_MODES: readonly string[] = ["static", "modal", "buckling"];

function Fem2dResultsStoryHost({ initialExampleId, locale, initialMode }: { readonly initialExampleId: string; readonly locale: FemStoryLocale; readonly initialMode: string }): ReactElement {
  const [state, setState] = useState<Fem2dStoryState>(() => {
    const seed = fem2dStoryStateFor(initialExampleId, locale);
    return { ...seed, resultDisplay: { ...seed.resultDisplay, mode: initialMode } };
  });
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setLastAction(descriptor);
    setState((current) => reduceFem2dStoryAction(current, descriptor.action, descriptor.args));
  }, []);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void => onAction({ controllerId: FEM2D_EDITOR_CONTROLLER_ID, action, args: { surfaceId: FEM2D_RESULTS_BODY_KEY, ...args } }),
    [onAction],
  );

  const caseIds = useMemo(() => [...state.snapshot.loadCases.map((entry) => entry.id), ...state.snapshot.combinations.map((entry) => entry.id)], [state.snapshot]);
  const caseId = useMemo(() => femResolveResultCase(state.resultDisplay, caseIds), [state.resultDisplay, caseIds]);
  const layers = useMemo(() => fem2dStructureLayers(state.snapshot, "#334155", "#334155", "#334155"), [state.snapshot]);
  const node = useMemo(() => buildFem2dSceneNode(layers, state.camera, FEM2D_RESULTS_BODY_KEY, FEM2D_EDITOR_CONTROLLER_ID), [layers, state.camera]);
  const debug = useMemo(
    () =>
      JSON.stringify({
        exampleId: state.exampleId,
        locale: state.locale,
        resultDisplay: state.resultDisplay,
        resolvedCaseId: caseId,
        backdropLayerCount: layers.length,
        omitted: femStoryOmissions([FEM2D_SOLVER_OMISSION, FEM2D_MESH_PREVIEW_OMISSION]),
        lastAction,
      }),
    [state, caseId, layers, lastAction],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", flexWrap: "wrap", gap: 4, padding: 4 }}>
        {FEM2D_RESULT_MODES.map((mode) => (
          <button key={mode} type="button" data-testid={`fem2d-result-mode-${mode}`} aria-pressed={state.resultDisplay.mode === mode} onClick={() => dispatch("setResultDisplay", { sourceId: caseId ?? undefined, mode, modeIndex: 0 })}>
            {mode}
          </button>
        ))}
        {state.snapshot.loadCases.map((loadCase) => (
          <button key={loadCase.id} type="button" data-testid={`fem2d-result-case-${loadCase.id}`} onClick={() => dispatch("setResultDisplay", { sourceId: loadCase.id, mode: state.resultDisplay.mode, modeIndex: state.resultDisplay.modeIndex })}>
            {loadCase.name}
          </button>
        ))}
        <button type="button" data-testid="fem2d-results-clear-example" onClick={() => dispatch("setActiveExample", { exampleId: FEM2D_CLEARED_EXAMPLE_ID })}>
          {femStoryLabel("clearExample", state.locale)}
        </button>
      </div>
      <pre data-testid="fem2d-results-caption" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {femResultCaptionLine(caseId, state.resultDisplay, state.locale)}
      </pre>
      {caseId === null ? (
        <div data-testid="fem2d-results-placeholder" style={{ flex: "1 1 auto", padding: 8, fontSize: 12 }}>
          {femStoryLabel("noLoadCase", state.locale)}
        </div>
      ) : (
        <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }} data-testid="fem2d-results-canvas">
          <Canvas2dHost node={node} onAction={onAction} />
        </div>
      )}
      <pre data-testid="fem2d-results-window" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {fem2dSummaryLines(state.snapshot, state.locale).join("\n")}
      </pre>
      <pre data-testid="fem2d-results-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🏗️fem◻️2d/Results",
  component: Fem2dResultsStoryHost,
  parameters: { layout: "fullscreen" },
  argTypes: { locale: { control: "inline-radio", options: FEM_STORY_LOCALES }, initialMode: { control: "inline-radio", options: FEM2D_RESULT_MODES } },
  tags: ["autodocs"],
} satisfies Meta<typeof Fem2dResultsStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 📊️ `DisplayMode::Static` on the bundled document: the faint undeformed backdrop plus the resolved case caption (`dead`, the first load case). */
export const StaticEnglish: Story = {
  args: { initialExampleId: FEM2D_STORY_EXAMPLE_ID, locale: "en-US", initialMode: "static" },
};

/** 🇩🇪️ `DisplayMode::Modal(0)` with German captions — the mode-shape polyline itself is a solver omission, listed in the debug panel. */
export const ModalGerman: Story = {
  args: { initialExampleId: FEM2D_STORY_EXAMPLE_ID, locale: "de-DE", initialMode: "modal" },
};

/** 🕳️ `render_static`'s guard branch: an empty document has no load case, so the window is a bare text node ("No load case defined"), not a canvas surface. */
export const NoLoadCase: Story = {
  args: { initialExampleId: FEM2D_CLEARED_EXAMPLE_ID, locale: "en-US", initialMode: "static" },
};
