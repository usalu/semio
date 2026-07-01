// #region 🧲Header
/** @emoji 🃏 Trinity jack play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
  AppRuntime,
  CommandBus,
  Controller,
  ModeRuntime,
  Platform,
  Playground,
  WindowKindRuntime,
  buildTableWindowBody,
  buildTrinityWindowBody,
  buildWriterWindowBody,
  createPlayAppRuntime,
  createProductPlaygroundPlatform,
  createWindowLayout,
  registerWindowBody,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  type PlaygroundFixtureCatalog,
  type PlaygroundFixtureHost,
  type UiNode,
  type WindowBodyViewContext,
  type WindowLayout,
  type WindowMeasure,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
  DocumentVcsStore,
  applyJsonReplaceOp,
  createDocumentVcsEnvelope,
  recordJsonProjectionChange,
  type JsonReplaceOp,
} from "@semio-tech/framework-core";
import { createWriterDocument, type WriterDocumentV1 } from "@semio-tech/writer-core";
import {
  TRINITY_DEFAULT_FIXTURE,
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
  type TrinityFixtureV1,
  type TrinityLodModeKind,
} from "@semio-tech/trinity-react";
import {
  TRINITY_JACK_PLAY_FIXTURE_DEFAULT_ID,
  TRINITY_JACK_PLAY_PRESET_QUERIES,
  resolveTrinityJackPlayFixtureSlug,
} from "./fixture-slugs.ts";

export const TRINITY_JACK_PLAY_APP_ID = "trinity-jack-play";
export const TRINITY_JACK_PLAY_CONTROLLER_ID = "trinity-jack-play";
export const TRINITY_JACK_PLAY_SURFACE_ID = "trinity.jack.play/v1";
export const TRINITY_JACK_PLAY_EDITOR_SURFACE_ID = "trinity.jack.editor/v1";
export const TRINITY_JACK_PLAY_RESULTS_SURFACE_ID = "trinity.jack.results/v1";
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

const trinityFixtureModules = import.meta.glob("../../fixture/*.trinity.json", { eager: true }) as Record<string, { default: unknown }>;

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
  Object.entries(trinityFixtureModules).map(([path, mod]) => {
    const id = trinityFixtureIdFromGlobPath(path);
    const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
    return [id, json];
  }),
);

const TRINITY_JACK_PLAY_PRESET_IDS = ["nakagin", "branch-chain"] as const;

export const TRINITY_JACK_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> =
  TRINITY_JACK_PLAY_PRESET_IDS.map((id) => ({ id, label: trinityFixtureLabelFromPresetId(id) }));

function trinityJackPlayFixtureJsonForPreset(presetId: string): string | undefined {
  const fileId = resolveTrinityJackPlayFixtureSlug(presetId);
  if (!fileId) return undefined;
  return TRINITY_JACK_PLAY_FILE_FIXTURE_JSON_BY_FILE_ID[fileId];
}

function parseTrinityJackPlayFixtureJson(json: string): TrinityFixtureV1 | null {
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
        items: TRINITY_JACK_PLAY_FIXTURE_OPTIONS.map((row) => ({
          id: `trinity-jack-catalogue.fixture.${row.id}`,
          label: row.label,
          description: TRINITY_JACK_PLAY_PRESET_QUERIES[row.id] ?? "",
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

export function registerTrinityJackPlayDeclarativeBodies(): void {
  registerWindowBody(TRINITY_JACK_PLAY_BODY_KEY_MAIN, buildTrinityJackPlayMainDeclarativeBody);
  registerWindowBody(TRINITY_JACK_PLAY_BODY_KEY_EDITOR, buildTrinityJackPlayEditorDeclarativeBody);
  registerWindowBody(TRINITY_JACK_PLAY_BODY_KEY_RESULTS, buildTrinityJackPlayResultsDeclarativeBody);
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

export class TrinityJackPlayController extends Controller implements PlaygroundFixtureHost {
  readonly mainMode = new ModeRuntime("explore", "Explore", undefined);
  private readonly docStore = new DocumentVcsStore<TrinityFixtureV1, JsonReplaceOp<TrinityFixtureV1>>({
    envelope: createDocumentVcsEnvelope("trinity.fixture/v1", "trinity-jack-play", TRINITY_DEFAULT_FIXTURE),
    applyOp: applyJsonReplaceOp,
  });
  private activeFixtureId = TRINITY_JACK_PLAY_FIXTURE_DEFAULT_ID;
  private jackQuery = TRINITY_JACK_PLAY_PRESET_QUERIES[TRINITY_JACK_PLAY_FIXTURE_DEFAULT_ID] ?? "MATCH (a:Piece) RETURN a.name";
  private jackResultJson = "";
  private selectedNodeIds: string[] = [];
  private reorganizeEpoch = 0;
  private interactionRevision = 0;
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
    return trinityFixtureToJson(this.projection());
  }

  getDocumentVcsStore(): DocumentVcsStore<TrinityFixtureV1, JsonReplaceOp<TrinityFixtureV1>> {
    return this.docStore;
  }

  private projection(): TrinityFixtureV1 {
    return this.docStore.projection();
  }

  private commitFixture(next: TrinityFixtureV1): void {
    recordJsonProjectionChange(this.docStore, next);
  }

  getJackQuery(): string {
    return this.jackQuery;
  }

  getWriterDocument(): WriterDocumentV1 {
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
    return this.selectedNodeIds;
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

  getActiveFixtureId(): string {
    return this.activeFixtureId;
  }

  getFixtureCatalog(): PlaygroundFixtureCatalog {
    return {
      activeId: this.activeFixtureId,
      options: TRINITY_JACK_PLAY_FIXTURE_OPTIONS,
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

  private windowMeasures(): readonly WindowMeasure[] {
    return [this.lodMeasure(TRINITY_JACK_PLAY_WINDOW_KIND_ID)];
  }

  private bump(): void {
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  private rebuildShellMode(): void {
    this.mainMode.windowKinds = [
      new WindowKindRuntime(TRINITY_JACK_PLAY_WINDOW_KIND_ID, "Nakagin Graph", TRINITY_JACK_PLAY_BODY_KEY_MAIN, undefined, this.windowMeasures()),
      new WindowKindRuntime(TRINITY_JACK_PLAY_EDITOR_WINDOW_KIND_ID, "Jack Query", TRINITY_JACK_PLAY_BODY_KEY_EDITOR),
      new WindowKindRuntime(TRINITY_JACK_PLAY_RESULTS_WINDOW_KIND_ID, "Results", TRINITY_JACK_PLAY_BODY_KEY_RESULTS),
    ];
  }

  run(command: string, args?: unknown): void {
    if (command === "setFixtureJson") {
      const json = (args as { json?: string }).json;
      const parsed = typeof json === "string" ? parseTrinityFixtureJson(json) : null;
      if (parsed) {
        this.commitFixture(parsed);
        this.bump();
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
    if (command === "runJackQuery") {
      const value = (args as { value?: string } | undefined)?.value;
      const query = typeof value === "string" && value.trim() ? value : this.jackQuery;
      this.jackQuery = query;
      try {
        const beforeJson = this.getFixtureJson();
        const result = runJackOnFixture(beforeJson, query);
        this.jackResultJson = JSON.stringify(result);
        if (result.fixtureJson && result.fixtureJson !== beforeJson) {
          const parsed = parseTrinityFixtureJson(result.fixtureJson);
          if (parsed) {
            this.commitFixture(parsed);
          }
        }
      } catch (err) {
        this.jackResultJson = JSON.stringify({ kind: "table", columns: ["error"], rows: [[String(err)]], fixtureJson: this.getFixtureJson() });
      }
      this.bump();
      return;
    }
    if (command === "setSelection") {
      const ids = (args as { ids?: string[] }).ids ?? [];
      this.selectedNodeIds = [...ids];
      this.bump();
      return;
    }
    if (command === "setActiveFixture") {
      const fixtureId = (args as { fixtureId?: string }).fixtureId;
      if (typeof fixtureId !== "string") return;
      const json = trinityJackPlayFixtureJsonForPreset(fixtureId);
      const parsed = json ? parseTrinityJackPlayFixtureJson(json) : null;
      if (!parsed) return;
      this.activeFixtureId = fixtureId;
      this.jackQuery = TRINITY_JACK_PLAY_PRESET_QUERIES[fixtureId] ?? this.jackQuery;
      this.commitFixture(parsed);
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

function buildTrinityJackPlayAppRuntime(ctrl: TrinityJackPlayController): AppRuntime {
  return createPlayAppRuntime(TRINITY_JACK_PLAY_APP_ID, "Trinity Jack", ctrl, buildTrinityJackPlayLayout(), ctrl.mainMode);
}

export class PlaygroundTrinityJack extends Playground {
  readonly id = TRINITY_JACK_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = createProductPlaygroundPlatform(this.id);
    const ctrl = new TrinityJackPlayController(runtime.commandBus, () => runtime.notify());
    runtime.addApp(buildTrinityJackPlayAppRuntime(ctrl));
    return runtime;
  }

  registerBodies(): void {
    registerTrinityJackPlayDeclarativeBodies();
  }
}

export { buildTrinityPlayHierarchyTree, buildTrinityPlayInspectorTree, parseTrinityFixtureJson, trinityFixtureToJson };
export type { TrinityFixtureV1 };

if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "trinity-jack"
) {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootTrinityJackPlay } = await import("@semio-tech/framework-playground-renderer-react/trinity-jack");
    bootTrinityJackPlay(new PlaygroundTrinityJack());
  })();
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
    it("runJackQuery returns nakagin table row", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      ctrl.run("runJackQuery", { value: "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name" });
      const result = JSON.parse(ctrl.getJackResultJson()) as { kind: string; rows: unknown[][] };
      expect(result.kind).toBe("table");
      expect(result.rows.length).toBe(1);
    });

    it("branch-chain preset returns graph result", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      ctrl.run("setActiveFixture", { fixtureId: "branch-chain" });
      const result = JSON.parse(ctrl.getJackResultJson()) as { kind: string; graphFixture?: { nodes: unknown[] } };
      expect(result.kind).toBe("graph");
      expect(result.graphFixture?.nodes.length).toBeGreaterThan(0);
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
  });
}
