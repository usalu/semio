// #region 🧲️Header
// 💻️ .storybook/stories/fem/3d/Results.stories.tsx
// Specs: Host the framework renderer's `🌐️World3dHost` for the fem3d play app's `fem3d-results` window
// (body key `fem3d.play.results`), driven by the REAL shipped example document and by the same
// `Fem3dConfig` result-display triple `config_result_display` maps onto `DisplayMode`.
// Summary: `render_static`/`render_modal`/`render_buckling` all build the SAME node/member/solid instances
// the model window renders and then offset them by `fem3d_solve_all`'s displacements (and, for static,
// color solid vertices by nodal von Mises). That solver is Rust-only with no browser build, so this story
// renders the undeformed, uncolored instance set and lists the displacement offsets and vertex colors as
// counted omissions. `render_static`'s two guard branches ARE reproduced for real: with no load case the
// Rust returns a bare `built_text_node(Label::data("No load case defined"))` and mounts no world at all;
// with a case it wraps the world in a `with_caption` column whose caption is `Case: <id>`, which the story
// renders as its own caption line above the host.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { World3dHost } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import {
  buildFem3dSceneNode,
  fem3dStoryStateFor,
  fem3dStructuralInstances,
  fem3dSummaryLines,
  femResolveResultCase,
  femResultCaptionLine,
  FEM_STORY_LOCALES,
  FEM3D_SOLID_MESH_OMISSION,
  FEM3D_SOLVER_OMISSION,
  FEM3D_STORY_EXAMPLE_ID,
  FEM3D_STORY_SHIPPED_EXAMPLE_ID,
  femStoryLabel,
  femStoryOmissions,
  reduceFem3dStoryAction,
  type Fem3dStoryState,
  type FemStoryLocale,
} from "../scene";

//#region StoryHost
const FEM3D_RESULTS_BODY_KEY = "fem3d.play.results";
const FEM3D_EDITOR_CONTROLLER_ID = "fem3d-play";
const FEM3D_RESULT_MODES: readonly string[] = ["static", "modal", "buckling"];

function Fem3dResultsStoryHost({ initialExampleId, locale, initialMode }: { readonly initialExampleId: string; readonly locale: FemStoryLocale; readonly initialMode: string }): ReactElement {
  const [state, setState] = useState<Fem3dStoryState>(() => {
    const seed = fem3dStoryStateFor(initialExampleId, locale);
    return { ...seed, resultDisplay: { ...seed.resultDisplay, mode: initialMode } };
  });
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setLastAction(descriptor);
    setState((current) => reduceFem3dStoryAction(current, descriptor.action, descriptor.args));
  }, []);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void => onAction({ controllerId: FEM3D_EDITOR_CONTROLLER_ID, action, args: { surfaceId: FEM3D_RESULTS_BODY_KEY, ...args } }),
    [onAction],
  );

  const caseIds = useMemo(() => [...state.snapshot.loadCases.map((entry) => entry.id), ...state.snapshot.combinations.map((entry) => entry.id)], [state.snapshot]);
  const caseId = useMemo(() => femResolveResultCase(state.resultDisplay, caseIds), [state.resultDisplay, caseIds]);
  const instances = useMemo(() => fem3dStructuralInstances(state.snapshot), [state.snapshot]);
  const node = useMemo(() => buildFem3dSceneNode(instances, state.cameraJson, FEM3D_RESULTS_BODY_KEY, FEM3D_EDITOR_CONTROLLER_ID), [instances, state.cameraJson]);
  const debug = useMemo(
    () => JSON.stringify({ exampleId: state.exampleId, locale: state.locale, resultDisplay: state.resultDisplay, resolvedCaseId: caseId, instanceCount: instances.length, omitted: femStoryOmissions([FEM3D_SOLVER_OMISSION, FEM3D_SOLID_MESH_OMISSION]), lastAction }),
    [state, caseId, instances, lastAction],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", minHeight: "24rem", flexDirection: "column" }}>
      <div style={{ display: "flex", flexWrap: "wrap", gap: 4, padding: 4 }}>
        {FEM3D_RESULT_MODES.map((mode) => (
          <button key={mode} type="button" data-testid={`fem3d-result-mode-${mode}`} aria-pressed={state.resultDisplay.mode === mode} onClick={() => dispatch("setResultDisplay", { sourceId: caseId ?? undefined, mode, modeIndex: 0 })}>
            {mode}
          </button>
        ))}
        {state.snapshot.loadCases.map((loadCase) => (
          <button key={loadCase.id} type="button" data-testid={`fem3d-result-case-${loadCase.id}`} onClick={() => dispatch("setResultDisplay", { sourceId: loadCase.id, mode: state.resultDisplay.mode, modeIndex: state.resultDisplay.modeIndex })}>
            {loadCase.name}
          </button>
        ))}
        <button type="button" data-testid="fem3d-results-clear-example" onClick={() => dispatch("setActiveExample", { exampleId: FEM3D_STORY_SHIPPED_EXAMPLE_ID })}>
          {femStoryLabel("clearExample", state.locale)}
        </button>
      </div>
      <pre data-testid="fem3d-results-caption" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {femResultCaptionLine(caseId, state.resultDisplay, state.locale)}
      </pre>
      {caseId === null ? (
        <div data-testid="fem3d-results-placeholder" style={{ flex: "1 1 auto", padding: 8, fontSize: 12 }}>
          {femStoryLabel("noLoadCase", state.locale)}
        </div>
      ) : (
        <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }} data-testid="fem3d-results-world">
          <World3dHost node={node} onAction={onAction} />
        </div>
      )}
      <pre data-testid="fem3d-results-window" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {fem3dSummaryLines(state.snapshot, state.locale).join("\n")}
      </pre>
      <pre data-testid="fem3d-results-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🏗️fem🧊️3d/Results",
  component: Fem3dResultsStoryHost,
  parameters: { layout: "fullscreen" },
  argTypes: { locale: { control: "inline-radio", options: FEM_STORY_LOCALES }, initialMode: { control: "inline-radio", options: FEM3D_RESULT_MODES } },
  tags: ["autodocs"],
} satisfies Meta<typeof Fem3dResultsStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 📊️ `DisplayMode::Static` on the bundled fixture: the undeformed instance set plus the `Case: dead` caption `with_caption` prints. */
export const StaticEnglish: Story = {
  args: { initialExampleId: FEM3D_STORY_EXAMPLE_ID, locale: "en-US", initialMode: "static" },
};

/** 🇩🇪️ `DisplayMode::Buckling(0)` with German captions — the load-factor caption and the buckled offsets are solver omissions, listed in the debug panel. */
export const BucklingGerman: Story = {
  args: { initialExampleId: FEM3D_STORY_EXAMPLE_ID, locale: "de-DE", initialMode: "buckling" },
};

/** 🕳️ `render_static`'s guard branch: an empty document has no load case, so the window is a bare text node ("No load case defined"), not a world surface. */
export const NoLoadCase: Story = {
  args: { initialExampleId: FEM3D_STORY_SHIPPED_EXAMPLE_ID, locale: "en-US", initialMode: "static" },
};
