// #region 🧲Header
/** @emoji 🃏 `@semio-tech/trinity-jack-host-core` — Trinity Jack app logic. */
// #endregion 🧲Header

import {
  AppRuntime,
  CommandBus,
  Controller,
  ModeRuntime,
  Platform,
  WindowKindRuntime,
  buildTableWindowBody,
  buildTrinityWindowBody,
  buildWriterWindowBody,
  createPlayAppRuntime,
  createPlaygroundApp,
  createProductPlaygroundPlatform,
  eagerPlayExampleGlob,
  createWindowLayout,
  registerWindowBody,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  type PlaygroundExampleCatalog,
  type PlaygroundExampleHost,
  type UiNode,
  type WindowBodyViewContext,
  type WindowLayout,
  type WindowMeasure,
  type WindowEngagement,
  type AppTools,
  type ToolLeaf,
  toolCollection,
  createWindowCommandEngagement,
  enforcePlaygroundWindowEngagementInput,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { createWriterDocument, type WriterDocument } from "@semio-tech/writer-core";
import {
  TRINITY_DEFAULT_FIXTURE_JSON,
  TRINITY_LOD_MODE_AUTOMATIC,
  buildTrinityPlayHierarchyTree,
  buildTrinityPlayInspectorTree,
  isTrinityDrawLodKind,
  parseTrinityFixtureJson,
  runJackOnFixture,
  trinityFixtureToJson,
  trinityLodAutomaticSelectLabel,
  trinityPlayLodTierMenuLabel,
  trinityPlayLodTiers,
  type TrinityDrawLodKind,
  type TrinityFixture,
  type TrinityJackDispatchRequest,
  type TrinityLodModeKind,
  type TrinityVcsCommandKind,
  type TrinityVcsRequest,
} from "@semio-tech/trinity-react";

export { buildTrinityPlayHierarchyTree, buildTrinityPlayInspectorTree } from "@semio-tech/trinity-react";

import {
  TRINITY_JACK_PLAY_DEFAULT_QUERY,
  TRINITY_JACK_PLAY_EXAMPLE_QUERIES,
  TRINITY_JACK_PLAY_FIXTURE_DEFAULT_ID,
  TRINITY_JACK_PLAY_PRESET_QUERIES,
  resolveTrinityJackPlayExampleSlug,
} from "./example-slugs.ts";

export {
  TRINITY_JACK_PLAY_DEFAULT_QUERY,
  TRINITY_JACK_PLAY_EXAMPLE_QUERIES,
} from "./example-slugs.ts";

export const TRINITY_JACK_PLAY_APP_ID = "trinity-jack-play";
export const TRINITY_JACK_PLAY_CONTROLLER_ID = "trinity-jack-play";
export const TRINITY_JACK_PLAY_SURFACE_ID = "trinity.jack.play";
export const TRINITY_JACK_PLAY_EDITOR_SURFACE_ID = "trinity.jack.editor";
export const TRINITY_JACK_PLAY_RESULTS_SURFACE_ID = "trinity.jack.results";
export const TRINITY_JACK_PLAY_BODY_KEY_MAIN = "trinity.jack.play.main";
export const TRINITY_JACK_PLAY_BODY_KEY_EDITOR = "trinity.jack.play.editor";
export const TRINITY_JACK_PLAY_BODY_KEY_RESULTS = "trinity.jack.play.results";
export const TRINITY_JACK_PLAY_WINDOW_KIND_ID = "trinity-jack-graph";
export const TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID = "trinity-jack-editor";
export const TRINITY_JACK_PLAY_RESULTS_WINDOW_KIND_ID = "trinity-jack-results";
export const TRINITY_JACK_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const TRINITY_JACK_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const TRINITY_JACK_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON = TRINITY_DEFAULT_FIXTURE_JSON;

const trinityExampleModules = eagerPlayExampleGlob("../../example/*.trinity.json");

function trinityFixtureIdFromGlobPath(globPath: string): string {
  const base = globPath.split("/").pop() ?? globPath;
  return base.replace(/\.trinity\.json$/, "");
}

function trinityFixtureLabelFromPresetId(id: string): string {
  if (id === "nakagin") return "Nakagin — Table";
  if (id === "branch-chain") return "Branch — Graph";
  return id
    .split("-")
    .filter(Boolean)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

const TRINITY_JACK_PLAY_FILE_FIXTURE_JSON_BY_FILE_ID: Record<string, string> = Object.fromEntries(
	Object.entries(trinityExampleModules).map(([path, mod]) => {
    const id = trinityFixtureIdFromGlobPath(path);
    const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
    return [id, json];
  }),
);

const TRINITY_JACK_PLAY_PRESET_IDS = ["nakagin", "branch-chain"] as const;

export const TRINITY_JACK_PLAY_EXAMPLE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> =
  TRINITY_JACK_PLAY_PRESET_IDS.map((id) => ({ id, label: trinityFixtureLabelFromPresetId(id) }));

function trinityJackPlayFixtureJsonForPreset(presetId: string): string | undefined {
  const fileId = resolveTrinityJackPlayExampleSlug(presetId);
  if (!fileId) return undefined;
  return TRINITY_JACK_PLAY_FILE_FIXTURE_JSON_BY_FILE_ID[fileId];
}

function parseTrinityJackPlayFixtureJson(json: string): TrinityFixture | null {
  return parseTrinityFixtureJson(json);
}

export function buildTrinityJackPlayCatalogueTree(activeFixtureId?: string): UiNode {
  return {
    type: "tree",
    sections: [
      {
        id: "trinity-jack-catalogue.fixtures",
        label: "Fixtures",
        defaultOpen: true,
        items: TRINITY_JACK_PLAY_EXAMPLE_OPTIONS.map((row) => ({
          id: `trinity-jack-catalogue.fixture.${row.id}`,
          label: row.label,
          description: TRINITY_JACK_PLAY_PRESET_QUERIES[row.id] ?? "",
        })),
      },
      {
        id: "trinity-jack-catalogue.examples",
        label: "Example queries",
        defaultOpen: true,
        items: TRINITY_JACK_PLAY_EXAMPLE_QUERIES.map((row) => ({
          id: `trinity-jack-catalogue.example.${row.id}`,
          label: row.label,
          description: row.query,
          command: {
            controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID,
            command: "loadExampleQuery",
            args: { query: row.query },
          },
        })),
      },
      {
        id: "trinity-jack-catalogue.kinds",
        label: "Manifest kinds",
        defaultOpen: false,
        items: [
          { id: "trinity-jack-catalogue.piece", label: "Piece", description: "node" },
          { id: "trinity-jack-catalogue.connection", label: "Connection", description: "edge" },
          { id: "trinity-jack-catalogue.connector", label: "Connector", description: "port" },
        ],
      },
    ],
    selectedIds: activeFixtureId ? [`trinity-jack-catalogue.fixture.${activeFixtureId}`] : [],
  };
}

export function buildTrinityJackPlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildTrinityWindowBody(TRINITY_JACK_PLAY_SURFACE_ID, TRINITY_JACK_PLAY_CONTROLLER_ID, TRINITY_JACK_PLAY_WINDOW_KIND_ID);
}

export function buildTrinityJackPlayEditorDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildWriterWindowBody(TRINITY_JACK_PLAY_EDITOR_SURFACE_ID, TRINITY_JACK_PLAY_CONTROLLER_ID, TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID);
}

export function buildTrinityJackPlayResultsDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildTableWindowBody(TRINITY_JACK_PLAY_RESULTS_SURFACE_ID, TRINITY_JACK_PLAY_CONTROLLER_ID, TRINITY_JACK_PLAY_RESULTS_WINDOW_KIND_ID);
}

export const trinityJackPlayWindowBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").WindowBodyViewContext) => UiNode>> = {
  [TRINITY_JACK_PLAY_BODY_KEY_MAIN]: buildTrinityJackPlayMainDeclarativeBody,
  [TRINITY_JACK_PLAY_BODY_KEY_EDITOR]: buildTrinityJackPlayEditorDeclarativeBody,
  [TRINITY_JACK_PLAY_BODY_KEY_RESULTS]: buildTrinityJackPlayResultsDeclarativeBody,
};

export function registerTrinityJackPlayDeclarativeBodies(): void {
  for (const [key, build] of Object.entries(trinityJackPlayWindowBodies)) registerWindowBody(key, build);
}

export function buildTrinityJackPlayLayout(): WindowLayout {
  return {
    root: {
      kind: "row",
      children: [
        {
          kind: "stack",
          size: 0.6,
          children: [createWindowLayout(TRINITY_JACK_PLAY_WINDOW_KIND_ID, "Nakagin Graph")],
        },
        {
          kind: "column",
          size: 0.4,
          children: [
            {
              kind: "stack",
              size: 0.55,
              children: [createWindowLayout(TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID, "Jack Query")],
            },
            {
              kind: "stack",
              size: 0.45,
              children: [createWindowLayout(TRINITY_JACK_PLAY_RESULTS_WINDOW_KIND_ID, "Results")],
            },
          ],
        },
      ],
    },
  };
}

/** @emoji 🧰 Trinity jack play footer toolbar. */
export function buildTrinityJackPlayToolbarTools(controllerId: string): AppTools {
  return [
    toolCollection("history", "history", [
      { kind: "button", id: "trinity-jack.undo", label: "Undo", iconId: "undo-2", controllerId, command: "undo" },
      { kind: "button", id: "trinity-jack.redo", label: "Redo", iconId: "redo-2", controllerId, command: "redo" },
      { kind: "button", id: "trinity-jack.checkpoint", label: "Checkpoint", iconId: "git-commit", controllerId, command: "commitCheckpoint" },
    ]),
    toolCollection("query", "terminal", [
      { kind: "button", id: "trinity-jack.run", label: "Run query", iconId: "play", controllerId, command: "runJackQuery" },
      { kind: "button", id: "trinity-jack.reorganize", label: "Reorganize", iconId: "refresh-cw", controllerId, command: "reorganize" },
    ]),
  ];
}

export class TrinityJackPlayController extends Controller implements PlaygroundExampleHost {
  readonly mainMode = new ModeRuntime("explore", "Explore", undefined);
  private fixtureJson = TRINITY_DEFAULT_FIXTURE_JSON;
  private activeExampleId = TRINITY_JACK_PLAY_FIXTURE_DEFAULT_ID;
  private jackQuery = TRINITY_JACK_PLAY_PRESET_QUERIES[TRINITY_JACK_PLAY_FIXTURE_DEFAULT_ID] ?? TRINITY_JACK_PLAY_DEFAULT_QUERY;
  private jackResultJson = "";
  private jackDispatchEpoch = 0;
  private jackDispatchQuery = "";
  private vcsEpoch = 0;
  private vcsKind: TrinityVcsCommandKind = "undo";
  private vcsMessage = "";
  private storeGeneration = 0;
  private reorganizeEpoch = 0;
  private interactionRevision = 0;
  private editorEngagementInput = "";
  private graphEngagementInput = "";
  private resultsEngagementInput = "";
  private lodMode: TrinityLodModeKind = TRINITY_LOD_MODE_AUTOMATIC;
  private lodModeByInstance: Record<string, TrinityLodModeKind> = {};
  private effectiveLod: TrinityDrawLodKind = "normal";
  private readonly snapshotListeners = new Set<() => void>();

  constructor(commandBus: CommandBus, notify: () => void) {
    super(TRINITY_JACK_PLAY_CONTROLLER_ID, commandBus, notify);
    this.rebuildShellMode();
    this.run("runJackQuery");
  }

  subscribeSnapshot(listener: () => void): () => void {
    this.snapshotListeners.add(listener);
    return () => this.snapshotListeners.delete(listener);
  }

  private notifySnapshot(): void {
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  getFixtureJson(): string {
    return this.fixtureJson;
  }

  getJackDispatch(): TrinityJackDispatchRequest | undefined {
    return this.jackDispatchEpoch > 0 ? { query: this.jackDispatchQuery, epoch: this.jackDispatchEpoch } : undefined;
  }

  getVcsRequest(): TrinityVcsRequest | undefined {
    return this.vcsEpoch > 0 ? { kind: this.vcsKind, epoch: this.vcsEpoch, message: this.vcsMessage || undefined } : undefined;
  }

  getStoreGeneration(): number {
    return this.storeGeneration;
  }

  private setFixtureJson(next: string): void {
    this.fixtureJson = next;
  }

  private dispatchJackQuery(query: string): void {
    this.jackDispatchQuery = query;
    this.jackDispatchEpoch += 1;
  }

  private requestVcs(kind: TrinityVcsCommandKind, message = ""): void {
    this.vcsKind = kind;
    this.vcsMessage = message;
    this.vcsEpoch += 1;
  }

  onJackDispatchComplete(resultJson: string): void {
    this.jackResultJson = resultJson;
    this.bump();
  }

  onVcsApplied(generation: number): void {
    this.storeGeneration = generation;
    this.bump();
  }

  getJackQuery(): string {
    return this.jackQuery;
  }

  getWriterDocument(): WriterDocument {
    return createWriterDocument({
      id: "jack-query",
      languageId: "jack",
      uri: "writer://jack-query",
      text: this.jackQuery,
    });
  }

  getJackResultJson(): string {
    return this.jackResultJson;
  }

  getSelectedNodeIds(): readonly string[] {
    return this.pointerFocus.getSnapshot().selection;
  }

  getReorganize(): { epoch: number; optionsJson: string } {
    return { epoch: this.reorganizeEpoch, optionsJson: "{}" };
  }

  getInteractionRevision(): number {
    return this.interactionRevision;
  }

  lodModeForScope(scopeId: string): TrinityLodModeKind {
    return this.lodModeByInstance[scopeId] ?? this.lodMode;
  }

  getActiveExampleId(): string {
    return this.activeExampleId;
  }

  getExampleCatalog(): PlaygroundExampleCatalog {
    return {
      activeExampleId: this.activeExampleId,
      options: TRINITY_JACK_PLAY_EXAMPLE_OPTIONS,
    };
  }

  private lodMeasure(scopeId: string): WindowMeasure {
    return {
      kind: "select",
      id: `${scopeId}-lod`,
      label: "LOD",
      value: this.lodModeForScope(scopeId),
      items: [
        { id: "automatic", value: TRINITY_LOD_MODE_AUTOMATIC, label: trinityLodAutomaticSelectLabel(this.effectiveLod) },
        ...trinityPlayLodTiers().map((tier) => ({ id: tier, value: tier, label: trinityPlayLodTierMenuLabel(tier) })),
      ],
      onChange: { controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID, command: "setLodMode", args: { instanceId: scopeId } },
    };
  }

  private windowMeasures(scopeId: string): readonly WindowMeasure[] {
    return [this.lodMeasure(scopeId)];
  }

  private graphEngagement(): WindowEngagement {
    return createWindowCommandEngagement("trinity-jack-graph-input", TRINITY_JACK_PLAY_CONTROLLER_ID, {
      placeholder: "Reorganize graph",
      inputCommand: "graphEngagementInput",
      submitCommand: "reorganize",
      value: this.graphEngagementInput,
      possibleEngagements: [
        { id: "trinity-jack-reorganize", label: "Reorganize", command: { controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID, command: "reorganize" } },
      ],
    });
  }

  private editorEngagement(): WindowEngagement {
    return createWindowCommandEngagement("trinity-jack-editor-input", TRINITY_JACK_PLAY_CONTROLLER_ID, {
      placeholder: "Run query, preset",
      inputCommand: "editorEngagementInput",
      submitCommand: "editorEngagementSubmit",
      value: this.editorEngagementInput,
      possibleEngagements: [
        { id: "trinity-jack-run", label: "Run query", command: { controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID, command: "runJackQuery" } },
        ...TRINITY_JACK_PLAY_EXAMPLE_OPTIONS.map((row) => ({
          id: `trinity-jack-preset-${row.id}`,
          label: row.label,
          command: { controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID, command: "setActiveExample", args: { exampleId: row.id } },
        })),
      ],
    });
  }

  private resultsEngagement(): WindowEngagement {
    return createWindowCommandEngagement("trinity-jack-results-input", TRINITY_JACK_PLAY_CONTROLLER_ID, {
      placeholder: "Run query",
      inputCommand: "resultsEngagementInput",
      submitCommand: "runJackQuery",
      value: this.resultsEngagementInput,
      status: [{ id: "trinity-jack-results-status", text: `${this.getSelectedNodeIds().length} selected` }],
    });
  }

  private bump(): void {
    this.interactionRevision += 1;
    this.rebuildShellMode();
    this.notifySnapshot();
    this.emit();
  }

  private rebuildShellMode(): void {
    this.mainMode.tools = buildTrinityJackPlayToolbarTools(TRINITY_JACK_PLAY_CONTROLLER_ID);
    this.mainMode.windowKinds = [
      new WindowKindRuntime(
        TRINITY_JACK_PLAY_WINDOW_KIND_ID,
        "Nakagin Graph",
        TRINITY_JACK_PLAY_BODY_KEY_MAIN,
        undefined,
        this.windowMeasures(TRINITY_JACK_PLAY_WINDOW_KIND_ID),
        this.graphEngagement(),
      ),
      new WindowKindRuntime(
        TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID,
        "Jack Query",
        TRINITY_JACK_PLAY_BODY_KEY_EDITOR,
        undefined,
        this.windowMeasures(TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID),
        this.editorEngagement(),
      ),
      new WindowKindRuntime(
        TRINITY_JACK_PLAY_RESULTS_WINDOW_KIND_ID,
        "Results",
        TRINITY_JACK_PLAY_BODY_KEY_RESULTS,
        undefined,
        [{ kind: "toggle", id: "trinity-jack-results-wrap", label: "Auto-run", iconId: "play", pressed: true, onChange: { controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID, command: "runJackQuery" } }],
        this.resultsEngagement(),
      ),
    ];
    for (const windowKind of this.mainMode.windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Trinity jack play window "${windowKind.id}"`);
    }
  }

  run(command: string, args?: unknown): void {
    if (command === "setFixtureJson") {
      const json = (args as { json?: string }).json;
      if (typeof json === "string" && parseTrinityFixtureJson(json)) {
        this.setFixtureJson(json);
        this.bump();
      }
      return;
    }
    if (command === "undo") {
      this.requestVcs("undo");
      this.bump();
      return;
    }
    if (command === "redo") {
      this.requestVcs("redo");
      this.bump();
      return;
    }
    if (command === "commitCheckpoint") {
      const message = (args as { message?: string }).message;
      this.requestVcs("commitCheckpoint", typeof message === "string" ? message : "");
      this.bump();
      return;
    }
    if (command === "loadExampleQuery") {
      const query = (args as { query?: string }).query;
      if (typeof query === "string") {
        this.jackQuery = query;
        this.bump();
        this.run("runJackQuery");
      }
      return;
    }
    if (command === "setJackQuery") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string") {
        this.jackQuery = value;
        this.bump();
      }
      return;
    }
    if (command === "graphEngagementInput") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string") {
        this.graphEngagementInput = value;
        this.rebuildShellMode();
        this.emit();
      }
      return;
    }
    if (command === "editorEngagementInput") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string") {
        this.editorEngagementInput = value;
        this.rebuildShellMode();
        this.emit();
      }
      return;
    }
    if (command === "editorEngagementSubmit") {
      const token = String((args as { value?: string }).value ?? this.editorEngagementInput).trim().toLowerCase();
      this.editorEngagementInput = "";
      if (!token || token === "run" || token === "run query") {
        this.run("runJackQuery");
        return;
      }
      const preset = TRINITY_JACK_PLAY_EXAMPLE_OPTIONS.find((row) => row.label.toLowerCase() === token || row.id === token);
      if (preset) {
        this.run("setActiveExample", { exampleId: preset.id });
        return;
      }
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "resultsEngagementInput") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string") {
        this.resultsEngagementInput = value;
        this.rebuildShellMode();
        this.emit();
      }
      return;
    }
    if (command === "runJackQuery") {
      const explicit = (args as { query?: string } | undefined)?.query;
      const query = typeof explicit === "string" && explicit.trim() ? explicit : this.jackQuery;
      this.dispatchJackQuery(query);
      this.resultsEngagementInput = "";
      this.bump();
      return;
    }
    if (command === "setSelection") {
      const ids = (args as { ids?: string[] }).ids ?? [];
      this.pointerFocus.setSelection([...ids]);
      this.bump();
      return;
    }
    if (command === "patchTrinityNodes") {
      const nodeIds = (args as { nodeIds?: readonly string[] }).nodeIds ?? [];
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value;
      if (!nodeIds.length || field !== "name" || typeof value !== "string") return;
      const nextName = value.trim();
      if (!nextName) return;
      const escaped = nextName.replace(/'/g, "\\'");
      const fixture = parseTrinityFixtureJson(this.fixtureJson);
      if (!fixture) return;
      const queries = nodeIds
        .map((id) => {
          const node = fixture.nodes.find((row) => row.id === id);
          if (!node) return null;
          return `MATCH (n:${node.kind}) WHERE n.id = '${id}' SET n.name = '${escaped}'`;
        })
        .filter((query): query is string => query !== null);
      if (queries.length) this.dispatchJackQuery(queries.join("\n"));
      return;
    }
    if (command === "setActiveExample") {
      const exampleId = (args as { exampleId?: string }).exampleId;
      if (typeof exampleId !== "string") return;
      const json = trinityJackPlayFixtureJsonForPreset(exampleId);
      const parsed = json ? parseTrinityJackPlayFixtureJson(json) : null;
      if (!parsed) return;
      this.activeExampleId = exampleId;
      this.jackQuery = TRINITY_JACK_PLAY_PRESET_QUERIES[exampleId] ?? this.jackQuery;
      this.setFixtureJson(json);
      this.bump();
      this.run("runJackQuery");
      return;
    }
    if (command === "setLodMode") {
      const { value, instanceId } = args as { value?: string; instanceId?: string };
      const scopeId = instanceId ?? TRINITY_JACK_PLAY_WINDOW_KIND_ID;
      if (typeof value !== "string") return;
      if (value !== TRINITY_LOD_MODE_AUTOMATIC && !isTrinityDrawLodKind(value)) return;
      this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: value as TrinityLodModeKind };
      if (scopeId === TRINITY_JACK_PLAY_WINDOW_KIND_ID) {
        this.lodMode = value as TrinityLodModeKind;
      }
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setEffectiveLod") {
      const { lod, instanceId } = args as { lod?: TrinityDrawLodKind; instanceId?: string };
      const scopeId = instanceId ?? TRINITY_JACK_PLAY_WINDOW_KIND_ID;
      if (!lod || !isTrinityDrawLodKind(lod)) return;
      if (scopeId !== TRINITY_JACK_PLAY_WINDOW_KIND_ID) return;
      if (this.effectiveLod === lod) return;
      this.effectiveLod = lod;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "reorganize") {
      this.reorganizeEpoch += 1;
      this.bump();
      return;
    }
  }
}

export function buildTrinityJackPlayAppRuntime(ctrl: TrinityJackPlayController): AppRuntime {
  return createPlayAppRuntime(TRINITY_JACK_PLAY_APP_ID, "Trinity Jack", ctrl, buildTrinityJackPlayLayout(), ctrl.mainMode);
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("buildTrinityJackPlayMainDeclarativeBody", () => {
    it("returns a trinity host surface", () => {
      const node = buildTrinityJackPlayMainDeclarativeBody({
        runtime: new Platform({ id: "test" }),
        windowKindId: TRINITY_JACK_PLAY_WINDOW_KIND_ID,
        bodyKey: TRINITY_JACK_PLAY_BODY_KEY_MAIN,
        activeModeId: "explore",
        generation: 0,
      });
      expect(node).toEqual({
        type: "trinity",
        componentKind: "trinity",
        surfaceId: TRINITY_JACK_PLAY_SURFACE_ID,
        controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID,
        paneId: TRINITY_JACK_PLAY_WINDOW_KIND_ID,
      });
    });
  });

  describe("registerTrinityJackPlayDeclarativeBodies", () => {
    it("registers graph, editor, and results bodies", () => {
      registerTrinityJackPlayDeclarativeBodies();
      expect(buildTrinityJackPlayEditorDeclarativeBody({
        runtime: new Platform({ id: "test" }),
        windowKindId: TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID,
        bodyKey: TRINITY_JACK_PLAY_BODY_KEY_EDITOR,
        activeModeId: "explore",
        generation: 0,
      }).type).toBe("writer");
      expect(buildTrinityJackPlayResultsDeclarativeBody({
        runtime: new Platform({ id: "test" }),
        windowKindId: TRINITY_JACK_PLAY_RESULTS_WINDOW_KIND_ID,
        bodyKey: TRINITY_JACK_PLAY_BODY_KEY_RESULTS,
        activeModeId: "explore",
        generation: 0,
      }).type).toBe("table");
    });
  });

  describe("TrinityJackPlayController", () => {
    function completeJackDispatch(ctrl: TrinityJackPlayController, query?: string): void {
      const q = query ?? ctrl.getJackQuery();
      const result = runJackOnFixture(ctrl.getFixtureJson(), q);
      ctrl.onJackDispatchComplete(JSON.stringify(result));
      if (result.fixtureJson !== ctrl.getFixtureJson()) {
        ctrl.run("setFixtureJson", { json: result.fixtureJson });
      }
    }

    it("catalogue tree lists example queries with commands", () => {
      const tree = buildTrinityJackPlayCatalogueTree("nakagin");
      const section = tree.sections.find((row) => row.id === "trinity-jack-catalogue.examples");
      expect(section?.items.length).toBe(TRINITY_JACK_PLAY_EXAMPLE_QUERIES.length);
      expect(section?.items.every((row) => row.command?.command === "loadExampleQuery")).toBe(true);
    });

    it("runJackQuery returns nakagin default table rows", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      const query = TRINITY_JACK_PLAY_DEFAULT_QUERY;
      ctrl.run("runJackQuery", { query });
      completeJackDispatch(ctrl, query);
      const result = JSON.parse(ctrl.getJackResultJson()) as { kind: string; rows: unknown[][] };
      expect(result.kind).toBe("table");
      expect(result.rows.length).toBe(2);
    });

    it.each(TRINITY_JACK_PLAY_EXAMPLE_QUERIES.map((row) => [row.id, row.query, row.label] as const))(
      "example query %s runs without error",
      (_id, query) => {
        const bus = new CommandBus();
        const ctrl = new TrinityJackPlayController(bus, () => {});
        ctrl.run("loadExampleQuery", { query });
        completeJackDispatch(ctrl, query);
        const result = JSON.parse(ctrl.getJackResultJson()) as { kind: string };
        expect(["table", "graph"]).toContain(result.kind);
      },
    );

    it("branch-chain preset returns graph result", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      ctrl.run("setActiveExample", { exampleId: "branch-chain" });
      completeJackDispatch(ctrl);
      const result = JSON.parse(ctrl.getJackResultJson()) as { kind: string; graphFixture?: { nodes: unknown[] } };
      expect(result.kind).toBe("graph");
      expect(result.graphFixture?.nodes.length).toBeGreaterThan(0);
    });

    it("patchTrinityNodes renames selected pieces", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => undefined);
      const nodeId = parseTrinityFixtureJson(ctrl.getFixtureJson())!.nodes[0]!.id;
      ctrl.run("patchTrinityNodes", { nodeIds: [nodeId], field: "name", value: "renamed-piece" });
      completeJackDispatch(ctrl, ctrl.getJackDispatch()?.query);
      const updated = parseTrinityFixtureJson(ctrl.getFixtureJson())!.nodes.find((node) => node.id === nodeId);
      expect(updated?.name).toBe("renamed-piece");
    });

    it("subscribeSnapshot notifies listeners", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      let revision = 0;
      ctrl.subscribeSnapshot(() => {
        revision = ctrl.getInteractionRevision();
      });
      ctrl.run("setJackQuery", { value: "MATCH (a:Piece) RETURN a.name" });
      expect(revision).toBeGreaterThan(0);
    });

    it("exposes three window kinds", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      expect(ctrl.mainMode.windowKinds?.map((row) => row.id)).toEqual([
        TRINITY_JACK_PLAY_WINDOW_KIND_ID,
        TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID,
        TRINITY_JACK_PLAY_RESULTS_WINDOW_KIND_ID,
      ]);
    });

    it("keeps jack query out of the editor window command line", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      const editor = ctrl.mainMode.windowKinds?.find((row) => row.id === TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID);
      expect(editor?.engagement?.input?.value).toBe("");
      expect(editor?.engagement?.input?.value).not.toBe(ctrl.getJackQuery());
      ctrl.run("setJackQuery", { value: "MATCH (n) RETURN n" });
      const next = ctrl.mainMode.windowKinds?.find((row) => row.id === TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID);
      expect(next?.engagement?.input?.value).toBe("");
    });

    it("runJackQuery uses writer document text, not command-line input", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      ctrl.run("setJackQuery", { value: "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name" });
      ctrl.run("runJackQuery", { value: "MATCH (a:Piece) RETURN a.name" });
      expect(ctrl.getJackDispatch()?.query).toBe("MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name");
    });
  });
}

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for trinity jack. */
export function buildTrinityProgramDefinition(): PlatformDefinition {
	return {
		id: "trinity",
		name: "Trinity",
		apiVersion: "1",
		apps: [{ id: "trinity-jack", label: "Trinity Jack", controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖OsProgram
import { mergeOsProgramDefinition, osBaselineResource, registerAppVcsHandler } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";
import { createTrinityGraphAppVcsHandler } from "@semio-tech/trinity-rewrite-core";

const trinityProgramContributionResources = {
		"trinity-jack": osBaselineResource("graph.trinity", "trinity.graph", "trinity", [{ id: "query", label: "Query" }]),
	};

/** @emoji 🧩 OS program contribution for trinity. */
export const trinityProgramContribution: OsProgramContribution = {
	programId: "trinity",
	register() {
		mergeOsProgramDefinition("trinity", buildTrinityProgramDefinition(), trinityProgramContributionResources);
		registerAppVcsHandler(createTrinityGraphAppVcsHandler());
	},
};
//#endregion 🔖OsProgram

//#region 🔖Play

/** @emoji 🛝 Trinity Jack playground app. */

export const trinityJackPlayAppDefinition = createPlaygroundApp({
	id: TRINITY_JACK_PLAY_APP_ID,
	label: "Trinity Jack",
	controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "trinity-jack",
		resolveDedupe: ["react", "react-dom", "three"],
		watchIgnored: ["../../rewrite/engine/lib.rs", "../../rewrite/engine/target/**", "../../rewrite/engine/Cargo.toml"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(TRINITY_JACK_PLAY_APP_ID);
			const ctrl = new TrinityJackPlayController(runtime.commandBus, () => runtime.notify());
			runtime.addApp(buildTrinityJackPlayAppRuntime(ctrl));
			return runtime;
	},
	loadRenderer: async () => (await import("@semio-tech/trinity-react/play")).trinityJackAppRenderer,
});
//#endregion 🔖Play
