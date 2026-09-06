// #region 🧲️Header
// 💻️ .storybook/stories/remodel/Panels.stories.tsx
// Specs: Component-level coverage for all seven panel tabs `create_remodeling_app` registers — document/pipeline,
// media, results, parameters, calibration, tracks, quality (`✏️editor/📌️panels/*/🦀️.rs`, dispatched by
// `ArtifactEditor::render`'s body-key match).
// Summary: Each panel is projected to a retained `BuiltNode` document by `../scene.ts`, minted into a
// `UiSnapshot` by the framework's own `builtNodeToSnapshot`, loaded into a real `UiDocumentStore`, and rendered
// by the real `InterpretedUiNode` — the same interpreter the shell runs, no wasm and no plugin runtime. The
// strings are formatted exactly as each panel's Rust `format!` calls produce them, from the locale-resolved
// label table, so switching the locale toolbar-style button re-renders every line in German the way
// `resolve_labels_for_locale` would for `locale: "de-DE"`.
// ⚠️ One honest translation step: remodel's panels still emit the LEGACY `UiNode` (`ui_stack_vertical`/`ui_text`/
// `ui_import_drop_zone`), and no TS adapter from that shape to the retained `BuiltNode` document exists anywhere
// in the repo (`builtNodeToSnapshot` is the only entry point, and it starts from `BuiltNode`). The Stack→
// `container(plain)` / Text→`text` / drop-zone→`container` + `dropOverlay` + `Trigger::Drop` binding mapping is
// therefore the STORY's, not the plugin's; the styling/layout tokens are this story's defaults, since the legacy
// `UiNode` carries none. Everything else — the text, the structure, the drop-zone action id — is the plugin's.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { InterpretedUiNode, UiDocumentStore, builtNodeToSnapshot } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { REMODEL_DEFAULT_CONFIG, REMODEL_EMPTY_SCENE, REMODEL_POPULATED_SCENE, remodelLabelsFor, type RemodelScene } from "./fixture";
import { REMODEL_EDITOR_CONTROLLER_ID, REMODEL_PANELS, reduceRemodelStoryAction, remodelPanelDocument, type RemodelPanelId } from "./scene";

const LOCALES = ["en-US", "de-DE"] as const;

//#region StoryHost
function RemodelPanelStoryHost({ panelId, document, locale }: { readonly panelId: RemodelPanelId; readonly document: RemodelScene; readonly locale: string }): ReactElement {
  const [config, setConfig] = useState({ ...REMODEL_DEFAULT_CONFIG, locale });
  const onAction = useCallback((descriptor: ActionDescriptor): void => setConfig((current) => reduceRemodelStoryAction(current, descriptor)), []);

  const built = useMemo(() => remodelPanelDocument(panelId, document, config), [panelId, document, config]);
  /** 🗄️ A fresh per-projection store: `builtNodeToSnapshot` mints DFS-local ids, and `loadSnapshot` is the store's whole-body hydration path. */
  const store = useMemo(() => {
    const created = new UiDocumentStore(`remodeling.play.${panelId}`);
    created.loadSnapshot(builtNodeToSnapshot(`remodeling.play.${panelId}`, built));
    return created;
  }, [panelId, built]);

  const tab = REMODEL_PANELS.find((entry) => entry.id === panelId);
  const labels = remodelLabelsFor(config.locale);
  const debug = useMemo(() => JSON.stringify({ panelId, bodyKey: tab?.bodyKey, tabLabel: tab ? labels[tab.labelKey] : null, locale: config.locale, nodeCount: store.getState().nodes.size }), [panelId, tab, labels, config.locale, store]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", gap: 6, padding: 4 }}>
        {LOCALES.map((candidate) => (
          <button key={candidate} type="button" data-testid={`remodel-panel-locale-${candidate}`} onClick={() => onAction({ controllerId: REMODEL_EDITOR_CONTROLLER_ID, action: "setLocale", args: { locale: candidate } })}>
            {candidate}
            {config.locale === candidate ? " ✓" : ""}
          </button>
        ))}
      </div>
      <div data-testid={`remodel-panel-${panelId}`} style={{ flex: "1 1 auto", minHeight: 0, overflow: "auto" }}>
        <InterpretedUiNode store={store} onAction={onAction} onIntent={() => undefined} />
      </div>
      <pre data-testid="remodel-panel-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "📸️remodel📌️Panels",
  component: RemodelPanelStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  args: { document: REMODEL_POPULATED_SCENE, locale: "en-US" },
} satisfies Meta<typeof RemodelPanelStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🗿️ The framework `document` tab — job stage/progress (`Bundle Adjusting (50%)`), the derived running status, and the active utility. */
export const DocumentPipeline: Story = { args: { panelId: "pipeline" } };

/** 🗂️ The media tab — the `importFramePayload` drop zone (a `dropOverlay` container bound to `Trigger::Drop`), stream/asset counts, and one line per stream plus its decoded container facts. */
export const Media: Story = { args: { panelId: "media" } };

/** 🧵️ The results tab — mesh source and its 0/0 counts (the composed mesh CHILD is unresolvable without a `durableArtifacts` map), then sparse/dense point counts, trajectory poses and geo availability. */
export const Results: Story = { args: { panelId: "results" } };

/** ⚙️ The parameters tab — all eight parameter groups, each formatted by its own Rust `format!`. */
export const Parameters: Story = { args: { panelId: "parameters" } };

/** 🎯️ The calibration tab — camera/rig counts, both calibrated cameras, and both GCPs with their observation counts. */
export const Calibration: Story = { args: { panelId: "calibration" } };

/** 🏃️ The tracks tab — the populated document's single `track-a`. */
export const Tracks: Story = { args: { panelId: "tracks" } };

/** ✅️ The quality tab — the QC report's four metrics, the GCP checkpoint RMSE, and the document's one warning. The `watertight` block is absent because this document's `qc.watertight` is null. */
export const Quality: Story = { args: { panelId: "quality" } };

/** 🇩🇪️ The quality tab in German — every line resolved through the `native_de` column of `app_labels! { RemodelingLabels }`. */
export const QualityGerman: Story = { args: { panelId: "quality", locale: "de-DE" } };

/** 🌱️ The boot document's tracks tab — the documented empty state: "No motion tracks" plus the engine-gap sentence. */
export const TracksEmptyState: Story = { args: { panelId: "tracks", document: REMODEL_EMPTY_SCENE } };

/** 🌱️ The boot document's quality tab — "No quality report yet", the only line `quality::render` emits without a `results.qc`. */
export const QualityEmptyState: Story = { args: { panelId: "quality", document: REMODEL_EMPTY_SCENE } };
