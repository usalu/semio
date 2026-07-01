// #region 🧲Header
/** @emoji ♻️ Trinity rewrite play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
  AppRuntime,
  CommandBus,
  Controller,
  ModeRuntime,
  Platform,
  Playground,
  WindowKindRuntime,
  buildFormsWindowBody,
  buildPuzzle2dWindowBody,
  buildTrinityWindowBody,
  buildWriterWindowBody,
  createPlayAppRuntime,
  createProductPlaygroundPlatform,
  createWindowLayout,
  registerWindowBody,
  type UiNode,
  type WindowBodyViewContext,
  type WindowLayout,
  type WindowMeasure,
  type WindowEngagement,
  type AppTools,
  toolCollection,
  enforcePlaygroundWindowEngagementInput,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
  DocumentVcsStore,
  createDocumentVcsEnvelope,
  recordProjectionChange,
} from "@semio-tech/framework-core";
import { createWriterDocument, type WriterDocumentV1 } from "@semio-tech/writer-core";
import type { Puzzle2dFixtureV1 } from "@semio-tech/puzzle-2d-react";
import {
  REWRITE_DEFAULT_LHS_FIXTURE,
  REWRITE_DEFAULT_LHS_FIXTURE_JSON,
  REWRITE_DEFAULT_RHS_FIXTURE,
  REWRITE_DEFAULT_RHS_FIXTURE_JSON,
  parseRewriteGraphFixtureJson,
  rewriteLhsGraphToJson,
  rewriteRhsGraphToJson,
} from "@semio-tech/trinity-rewrite-react";
import {
  type FormSpec,
  type FormQuestion,
  type FormValues,
} from "@semio-tech/forms-core";
import {
  TRINITY_DEFAULT_FIXTURE,
  TRINITY_DEFAULT_FIXTURE_JSON,
  TRINITY_LOD_MODE_AUTOMATIC,
  applyRewriteOnFixture,
  buildTrinityPlayCatalogueTree,
  buildTrinityPlayHierarchyTree,
  buildTrinityPlayInspectorTree,
  isTrinityDrawLodKind,
  parseTrinityFixtureJson,
  ruleQueryOnFixture,
  trinityFixtureToJson,
  trinityLodAutomaticSelectLabel,
  trinityPlayLodTierMenuLabel,
  trinityPlayLodTiers,
  type RuleParameterV1,
  type TrinityDrawLodKind,
  type TrinityFixtureV1,
  type TrinityLodModeKind,
} from "@semio-tech/trinity-react";

type TrinityFixtureEditOp = { readonly op: "setDocument"; readonly document: TrinityFixtureV1 };

function applyTrinityFixtureEditOp(_fixture: TrinityFixtureV1, op: TrinityFixtureEditOp): TrinityFixtureV1 {
  return op.document;
}

export const TRINITY_REWRITE_PLAY_APP_ID = "trinity-rewrite-play";
export const TRINITY_REWRITE_PLAY_CONTROLLER_ID = "trinity-rewrite-play";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_BEFORE = "trinity.rewrite.before/v1";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_AFTER = "trinity.rewrite.after/v1";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_LHS = "trinity.rewrite.lhs/v1";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_RHS = "trinity.rewrite.rhs/v1";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_JACK = "trinity.rewrite.jack/v1";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_PARAMETERS = "trinity.rewrite.parameters/v1";
export const TRINITY_REWRITE_PLAY_BODY_KEY_BEFORE = "trinity.rewrite.play.before";
export const TRINITY_REWRITE_PLAY_BODY_KEY_AFTER = "trinity.rewrite.play.after";
export const TRINITY_REWRITE_PLAY_BODY_KEY_LHS = "trinity.rewrite.play.lhs";
export const TRINITY_REWRITE_PLAY_BODY_KEY_RHS = "trinity.rewrite.play.rhs";
export const TRINITY_REWRITE_PLAY_BODY_KEY_JACK = "trinity.rewrite.play.jack";
export const TRINITY_REWRITE_PLAY_BODY_KEY_PARAMETERS = "trinity.rewrite.play.parameters";
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE = "trinity-rewrite-before";
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER = "trinity-rewrite-after";

function trinityRewritePlayCmd(command: string, args?: Record<string, unknown>) {
  return { controllerId: TRINITY_REWRITE_PLAY_CONTROLLER_ID, command, args };
}

/** @emoji 🧰 Trinity rewrite play footer toolbar. */
export function buildTrinityRewritePlayToolbarTools(controllerId: string): AppTools {
  return [
    toolCollection("rewrite", "repeat", [
      { kind: "button", id: "trinity-rewrite.reorganize", label: "Reorganize", iconId: "refresh-cw", controllerId, command: "reorganize" },
    ]),
  ];
}
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS = "trinity-rewrite-lhs";
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS = "trinity-rewrite-rhs";
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_JACK = "trinity-rewrite-jack";
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_PARAMETERS = "trinity-rewrite-parameters";
export const TRINITY_REWRITE_PLAY_RULE_NAME = "label-core";

export {
  REWRITE_DEFAULT_LHS_FIXTURE,
  REWRITE_DEFAULT_LHS_FIXTURE_JSON,
  REWRITE_DEFAULT_RHS_FIXTURE,
  REWRITE_DEFAULT_RHS_FIXTURE_JSON,
  rewriteLhsKindCatalogs,
  rewriteRhsKindCatalogs,
} from "@semio-tech/trinity-rewrite-react";

export const TRINITY_REWRITE_PLAY_DEFAULT_LHS_JSON = rewriteLhsGraphToJson(REWRITE_DEFAULT_LHS_FIXTURE);

export const TRINITY_REWRITE_PLAY_DEFAULT_RHS_JSON = rewriteRhsGraphToJson(REWRITE_DEFAULT_RHS_FIXTURE);

function parseRhsParameters(rhsJson: string): readonly RuleParameterV1[] {
  try {
    const rhs = JSON.parse(rhsJson) as { parameters?: readonly RuleParameterV1[] };
    return rhs.parameters ?? [];
  } catch {
    return [];
  }
}

function parameterDefaultValues(parameters: readonly RuleParameterV1[]): FormValues {
  const values: Record<string, FormValues[string]> = {};
  for (const param of parameters) {
    values[param.name] = param.default as FormValues[string];
  }
  return values;
}

function buildParameterFormSpec(parameters: readonly RuleParameterV1[]): FormSpec {
  const questions: FormQuestion[] = parameters.map((param) => {
    const base = { id: param.name, label: param.name, description: param.kind };
    if (param.kind === "number") {
      return { ...base, kind: "number" as const, default: typeof param.default === "number" ? param.default : 0 };
    }
    if (param.kind === "boolean") {
      return { ...base, kind: "boolean" as const, default: Boolean(param.default) };
    }
    return { ...base, kind: "text" as const, default: String(param.default ?? "") };
  });
  return {
    schema: "forms.form/v1",
    id: "trinity-rewrite-parameters",
    version: "1",
    title: "Parameters",
    steps: [{ id: "params", title: "Parameters", questions }],
  };
}

function formValuesToBindings(values: FormValues): Record<string, string | number | boolean | null> {
  const bindings: Record<string, string | number | boolean | null> = {};
  for (const [key, value] of Object.entries(values)) {
    if (value === null || typeof value === "string" || typeof value === "number" || typeof value === "boolean") {
      bindings[key] = value;
    }
  }
  return bindings;
}

function buildRuleJson(name: string, lhsJson: string, rhsJson: string): string {
  const lhs = JSON.parse(lhsJson);
  const rhs = JSON.parse(rhsJson);
  return JSON.stringify({ name, lhs, rhs });
}

export function buildTrinityRewritePlayLayout(): WindowLayout {
  return {
    root: {
      kind: "column",
      children: [
        {
          kind: "row",
          size: 0.5,
          children: [
            { kind: "stack", size: 0.34, children: [createWindowLayout(TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS, "LHS")] },
            { kind: "stack", size: 0.34, children: [createWindowLayout(TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS, "RHS")] },
            { kind: "stack", size: 0.32, children: [createWindowLayout(TRINITY_REWRITE_PLAY_WINDOW_KIND_JACK, "Jack")] },
          ],
        },
        {
          kind: "row",
          size: 0.5,
          children: [
            { kind: "stack", size: 0.34, children: [createWindowLayout(TRINITY_REWRITE_PLAY_WINDOW_KIND_PARAMETERS, "Parameters")] },
            { kind: "stack", size: 0.33, children: [createWindowLayout(TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE, "Before")] },
            { kind: "stack", size: 0.33, children: [createWindowLayout(TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER, "After")] },
          ],
        },
      ],
    },
  };
}

export function buildTrinityRewritePlayBeforeDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildTrinityWindowBody(TRINITY_REWRITE_PLAY_SURFACE_ID_BEFORE, TRINITY_REWRITE_PLAY_CONTROLLER_ID, TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE);
}

export function buildTrinityRewritePlayAfterDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildTrinityWindowBody(TRINITY_REWRITE_PLAY_SURFACE_ID_AFTER, TRINITY_REWRITE_PLAY_CONTROLLER_ID, TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER);
}

export function buildTrinityRewritePlayLhsDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildPuzzle2dWindowBody(TRINITY_REWRITE_PLAY_SURFACE_ID_LHS, TRINITY_REWRITE_PLAY_CONTROLLER_ID, TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS);
}

export function buildTrinityRewritePlayRhsDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildPuzzle2dWindowBody(TRINITY_REWRITE_PLAY_SURFACE_ID_RHS, TRINITY_REWRITE_PLAY_CONTROLLER_ID, TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS);
}

export function buildTrinityRewritePlayJackDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildWriterWindowBody(TRINITY_REWRITE_PLAY_SURFACE_ID_JACK, TRINITY_REWRITE_PLAY_CONTROLLER_ID, TRINITY_REWRITE_PLAY_WINDOW_KIND_JACK);
}

export function buildTrinityRewritePlayParametersDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildFormsWindowBody(TRINITY_REWRITE_PLAY_SURFACE_ID_PARAMETERS, TRINITY_REWRITE_PLAY_CONTROLLER_ID, "preview");
}

export function registerTrinityRewritePlayDeclarativeBodies(): void {
  registerWindowBody(TRINITY_REWRITE_PLAY_BODY_KEY_BEFORE, buildTrinityRewritePlayBeforeDeclarativeBody);
  registerWindowBody(TRINITY_REWRITE_PLAY_BODY_KEY_AFTER, buildTrinityRewritePlayAfterDeclarativeBody);
  registerWindowBody(TRINITY_REWRITE_PLAY_BODY_KEY_LHS, buildTrinityRewritePlayLhsDeclarativeBody);
  registerWindowBody(TRINITY_REWRITE_PLAY_BODY_KEY_RHS, buildTrinityRewritePlayRhsDeclarativeBody);
  registerWindowBody(TRINITY_REWRITE_PLAY_BODY_KEY_JACK, buildTrinityRewritePlayJackDeclarativeBody);
  registerWindowBody(TRINITY_REWRITE_PLAY_BODY_KEY_PARAMETERS, buildTrinityRewritePlayParametersDeclarativeBody);
}

export class TrinityRewritePlayController extends Controller {
  readonly mainMode = new ModeRuntime("explore", "Explore", undefined);
  private readonly docStore = new DocumentVcsStore<TrinityFixtureV1, TrinityFixtureEditOp>({
    envelope: createDocumentVcsEnvelope("trinity.fixture/v1", "trinity-rewrite-play", TRINITY_DEFAULT_FIXTURE),
    applyOp: applyTrinityFixtureEditOp,
  });
  private lhsFixture: Puzzle2dFixtureV1 = REWRITE_DEFAULT_LHS_FIXTURE;
  private rhsFixture: Puzzle2dFixtureV1 = REWRITE_DEFAULT_RHS_FIXTURE;
  private parameterValues: FormValues = parameterDefaultValues(parseRhsParameters(TRINITY_REWRITE_PLAY_DEFAULT_RHS_JSON));
  private jackQueryText = "";
  private afterFixtureJson = TRINITY_DEFAULT_FIXTURE_JSON;
  private selectedNodeIds: string[] = [];
  private reorganizeEpoch = 0;
  private interactionRevision = 0;
  private lodMode: TrinityLodModeKind = TRINITY_LOD_MODE_AUTOMATIC;
  private lodModeByInstance: Record<string, TrinityLodModeKind> = {};
  private effectiveLod: TrinityDrawLodKind = "normal";
  private readonly snapshotListeners = new Set<() => void>();

  constructor(commandBus: CommandBus, notify: () => void) {
    super(TRINITY_REWRITE_PLAY_CONTROLLER_ID, commandBus, notify);
    this.recomputeDerived();
    this.rebuildShellMode();
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

  getBeforeFixtureJson(): string {
    return trinityFixtureToJson(this.projection());
  }

  getFixtureJson(): string {
    return this.getBeforeFixtureJson();
  }

  getAfterFixtureJson(): string {
    return this.afterFixtureJson;
  }

  getDocumentVcsStore(): DocumentVcsStore<TrinityFixtureV1, TrinityFixtureEditOp> {
    return this.docStore;
  }

  private projection(): TrinityFixtureV1 {
    return this.docStore.projection();
  }

  private commitFixture(next: TrinityFixtureV1): void {
    const previous = this.projection();
    recordProjectionChange(this.docStore, [{ op: "setDocument", document: next }]);
  }

  getLhsFixtureJson(): string {
    return JSON.stringify(this.lhsFixture);
  }

  getRhsFixtureJson(): string {
    return JSON.stringify(this.rhsFixture);
  }

  getLhsJson(): string {
    return rewriteLhsGraphToJson(this.lhsFixture);
  }

  getRhsJson(): string {
    return rewriteRhsGraphToJson(this.rhsFixture);
  }

  getRuleJson(): string {
    return buildRuleJson(TRINITY_REWRITE_PLAY_RULE_NAME, this.getLhsJson(), this.getRhsJson());
  }

  getBindingsJson(): string {
    return JSON.stringify(formValuesToBindings(this.parameterValues));
  }

  getParameterFormSpec(): FormSpec {
    return buildParameterFormSpec(parseRhsParameters(this.getRhsJson()));
  }

  getParameterValues(): FormValues {
    return this.parameterValues;
  }

  getJackQueryText(): string {
    return this.jackQueryText;
  }

  getWriterDocumentJack(): WriterDocumentV1 {
    return createWriterDocument({
      id: "rewrite-jack",
      languageId: "jack",
      uri: "writer://rewrite-jack",
      text: this.jackQueryText,
    });
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

  private recomputeDerived(): void {
    try {
      this.jackQueryText = ruleQueryOnFixture(this.getRuleJson(), this.getBindingsJson());
    } catch {
      this.jackQueryText = "";
    }
    try {
      const resultJson = applyRewriteOnFixture(this.getBeforeFixtureJson(), this.getRuleJson(), this.getBindingsJson());
      const parsed = JSON.parse(resultJson) as { fixture?: string };
      this.afterFixtureJson = typeof parsed.fixture === "string" ? parsed.fixture : this.getBeforeFixtureJson();
    } catch {
      this.afterFixtureJson = this.getBeforeFixtureJson();
    }
  }

  private syncParameterDefaultsFromRhs(): void {
    const defaults = parameterDefaultValues(parseRhsParameters(this.getRhsJson()));
    const next: FormValues = { ...defaults };
    for (const [key, value] of Object.entries(this.parameterValues)) {
      if (key in defaults) {
        next[key] = value;
      }
    }
    this.parameterValues = next;
  }

  private bump(): void {
    this.interactionRevision += 1;
    this.recomputeDerived();
    this.rebuildShellMode();
    this.notifySnapshot();
    this.emit();
  }

  private rewriteEngagement(scopeId: string): WindowEngagement {
    return {
      sessionActive: false,
      input: {
        id: `${scopeId}-engagement-input`,
        value: "",
        placeholder: "Reorganize",
        onChange: trinityRewritePlayCmd("rewriteEngagementInput", { scopeId }),
        onSubmit: trinityRewritePlayCmd("reorganize"),
      },
      possibleEngagements: [
        { id: `${scopeId}-reorganize`, label: "Reorganize", command: trinityRewritePlayCmd("reorganize") },
      ],
    };
  }

  private rebuildShellMode(): void {
    const lodMeasure = (scopeId: string): WindowMeasure => ({
      kind: "select" as const,
      id: `${scopeId}-lod`,
      label: "LOD",
      value: this.lodModeForScope(scopeId),
      items: [
        { id: "automatic", value: TRINITY_LOD_MODE_AUTOMATIC, label: trinityLodAutomaticSelectLabel(this.effectiveLod) },
        ...trinityPlayLodTiers().map((tier) => ({ id: tier, value: tier, label: trinityPlayLodTierMenuLabel(tier) })),
      ],
      onChange: { controllerId: TRINITY_REWRITE_PLAY_CONTROLLER_ID, command: "setLodMode", args: { instanceId: scopeId } },
    });
    this.mainMode.tools = buildTrinityRewritePlayToolbarTools(TRINITY_REWRITE_PLAY_CONTROLLER_ID);
    this.mainMode.windowKinds = [
      new WindowKindRuntime(TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS, "LHS", TRINITY_REWRITE_PLAY_BODY_KEY_LHS, undefined, [lodMeasure(TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS)], this.rewriteEngagement(TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS)),
      new WindowKindRuntime(TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS, "RHS", TRINITY_REWRITE_PLAY_BODY_KEY_RHS, undefined, [lodMeasure(TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS)], this.rewriteEngagement(TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS)),
      new WindowKindRuntime(TRINITY_REWRITE_PLAY_WINDOW_KIND_JACK, "Jack", TRINITY_REWRITE_PLAY_BODY_KEY_JACK, undefined, [lodMeasure(TRINITY_REWRITE_PLAY_WINDOW_KIND_JACK)], this.rewriteEngagement(TRINITY_REWRITE_PLAY_WINDOW_KIND_JACK)),
      new WindowKindRuntime(TRINITY_REWRITE_PLAY_WINDOW_KIND_PARAMETERS, "Parameters", TRINITY_REWRITE_PLAY_BODY_KEY_PARAMETERS, undefined, [{ kind: "slider", id: "trinity-rewrite-parameters-count", label: "Parameters", value: Object.keys(this.parameterValues).length, min: 0, max: Math.max(Object.keys(this.parameterValues).length, 1), step: 1, onChange: trinityRewritePlayCmd("reorganize") }], this.rewriteEngagement(TRINITY_REWRITE_PLAY_WINDOW_KIND_PARAMETERS)),
      new WindowKindRuntime(TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE, "Before", TRINITY_REWRITE_PLAY_BODY_KEY_BEFORE, undefined, [lodMeasure(TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE)], this.rewriteEngagement(TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE)),
      new WindowKindRuntime(TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER, "After", TRINITY_REWRITE_PLAY_BODY_KEY_AFTER, undefined, [lodMeasure(TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER)], this.rewriteEngagement(TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER)),
    ];
    for (const windowKind of this.mainMode.windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Trinity rewrite play window "${windowKind.id}"`);
    }
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
    if (command === "setLhsFixtureJson") {
      const json = (args as { json?: string }).json;
      const parsed = typeof json === "string" ? parseRewriteGraphFixtureJson(json) : null;
      if (parsed) {
        this.lhsFixture = parsed;
        this.bump();
      }
      return;
    }
    if (command === "setRhsFixtureJson") {
      const json = (args as { json?: string }).json;
      const parsed = typeof json === "string" ? parseRewriteGraphFixtureJson(json) : null;
      if (parsed) {
        this.rhsFixture = parsed;
        this.syncParameterDefaultsFromRhs();
        this.bump();
      }
      return;
    }
    if (command === "setParameterValues") {
      const values = (args as { values?: FormValues }).values;
      if (values && typeof values === "object") {
        this.parameterValues = values;
        this.bump();
      }
      return;
    }
    if (command === "setSelection") {
      const ids = (args as { ids?: string[] }).ids ?? [];
      this.selectedNodeIds = [...ids];
      this.bump();
      return;
    }
    if (command === "patchTrinityNodes") {
      const nodeIds = (args as { nodeIds?: readonly string[] }).nodeIds ?? [];
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value;
      if (!nodeIds.length || field !== "name" || typeof value !== "string") return;
      const targets = new Set(nodeIds);
      const fixture = this.projection();
      const nextName = value.trim();
      if (!nextName) return;
      this.commitFixture({
        ...fixture,
        nodes: fixture.nodes.map((node) => (targets.has(node.id) ? { ...node, name: nextName } : node)),
      });
      this.bump();
      return;
    }
    if (command === "setLodMode") {
      const { value, instanceId } = args as { value?: string; instanceId?: string };
      const scopeId = instanceId ?? TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE;
      if (typeof value !== "string") return;
      if (value !== TRINITY_LOD_MODE_AUTOMATIC && !isTrinityDrawLodKind(value)) return;
      this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: value as TrinityLodModeKind };
      if (scopeId === TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE) {
        this.lodMode = value as TrinityLodModeKind;
      }
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setEffectiveLod") {
      const { lod, instanceId } = args as { lod?: TrinityDrawLodKind; instanceId?: string };
      const scopeId = instanceId ?? TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE;
      if (!lod || !isTrinityDrawLodKind(lod)) return;
      if (scopeId !== TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE && scopeId !== TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER) return;
      if (this.effectiveLod === lod) return;
      this.effectiveLod = lod;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "reorganize") {
      this.reorganizeEpoch += 1;
      this.bump();
    }
  }
}

function buildTrinityRewritePlayAppRuntime(ctrl: TrinityRewritePlayController): AppRuntime {
  return createPlayAppRuntime(TRINITY_REWRITE_PLAY_APP_ID, "Trinity Rewrite", ctrl, buildTrinityRewritePlayLayout(), ctrl.mainMode);
}

export class PlaygroundTrinityRewrite extends Playground {
  readonly id = TRINITY_REWRITE_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = createProductPlaygroundPlatform(this.id);
    const ctrl = new TrinityRewritePlayController(runtime.commandBus, () => runtime.notify());
    runtime.addApp(buildTrinityRewritePlayAppRuntime(ctrl));
    return runtime;
  }

  registerBodies(): void {
    registerTrinityRewritePlayDeclarativeBodies();
  }
}

export { buildTrinityPlayHierarchyTree, buildTrinityPlayCatalogueTree, buildTrinityPlayInspectorTree };

if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "trinity-rewrite"
) {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootTrinityRewritePlay } = await import("@semio-tech/framework-playground-renderer-react/trinity-rewrite");
    bootTrinityRewritePlay(new PlaygroundTrinityRewrite());
  })();
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("buildTrinityRewritePlayLayout", () => {
    it("defines six window slots in two rows", () => {
      const layout = buildTrinityRewritePlayLayout();
      expect(layout.root.kind).toBe("column");
      expect(layout.root.children?.length).toBe(2);
    });
  });

  describe("registerTrinityRewritePlayDeclarativeBodies", () => {
    it("registers lhs, rhs, jack, parameters, before, and after bodies", () => {
      registerTrinityRewritePlayDeclarativeBodies();
      const ctx = {
        runtime: new Platform({ id: "test" }),
        windowKindId: TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS,
        bodyKey: TRINITY_REWRITE_PLAY_BODY_KEY_LHS,
        activeModeId: "explore",
        generation: 0,
      };
      expect(buildTrinityRewritePlayLhsDeclarativeBody(ctx).type).toBe("puzzle2d");
      expect(buildTrinityRewritePlayRhsDeclarativeBody(ctx).type).toBe("puzzle2d");
      expect(buildTrinityRewritePlayJackDeclarativeBody(ctx).type).toBe("writer");
      expect(buildTrinityRewritePlayParametersDeclarativeBody(ctx).type).toBe("forms");
      expect(buildTrinityRewritePlayBeforeDeclarativeBody(ctx).type).toBe("trinity");
      expect(buildTrinityRewritePlayAfterDeclarativeBody(ctx).type).toBe("trinity");
    });
  });

  describe("TrinityRewritePlayController", () => {
    it("exposes six window kinds", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      expect(ctrl.mainMode.windowKinds?.map((row) => row.id)).toEqual([
        TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS,
        TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS,
        TRINITY_REWRITE_PLAY_WINDOW_KIND_JACK,
        TRINITY_REWRITE_PLAY_WINDOW_KIND_PARAMETERS,
        TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE,
        TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER,
      ]);
    });

    it("default demo applies nakagin-core label in after preview without mutating before", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      const before = ctrl.getBeforeFixtureJson();
      const after = parseTrinityFixtureJson(ctrl.getAfterFixtureJson());
      expect(after).not.toBeNull();
      const core = after?.nodes.find((node) => node.name === "b");
      expect(core?.properties?.label).toBe("nakagin-core");
      expect(ctrl.getBeforeFixtureJson()).toBe(before);
      expect(ctrl.getJackQueryText()).toContain("SET a.label = 'nakagin-core'");
    });

    it("parameter value changes jack query and after preview", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      const before = ctrl.getBeforeFixtureJson();
      ctrl.run("setParameterValues", { values: { label: "override-core" } });
      expect(ctrl.getJackQueryText()).toContain("SET a.label = 'override-core'");
      const after = parseTrinityFixtureJson(ctrl.getAfterFixtureJson());
      const core = after?.nodes.find((node) => node.name === "b");
      expect(core?.properties?.label).toBe("override-core");
      expect(ctrl.getBeforeFixtureJson()).toBe(before);
    });

    it("patchTrinityNodes renames selected pieces in before fixture", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => undefined);
      const nodeId = parseTrinityFixtureJson(ctrl.getBeforeFixtureJson())!.nodes[0]!.id;
      ctrl.run("patchTrinityNodes", { nodeIds: [nodeId], field: "name", value: "rewrite-renamed" });
      const updated = parseTrinityFixtureJson(ctrl.getBeforeFixtureJson())!.nodes.find((node) => node.id === nodeId);
      expect(updated?.name).toBe("rewrite-renamed");
    });

    it("lhs graph edits recompile into jack query", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      const fixture = parseRewriteGraphFixtureJson(ctrl.getLhsFixtureJson());
      expect(fixture).not.toBeNull();
      const next = {
        ...fixture!,
        nodes: fixture!.nodes.map((node) => (node.id === "where-b" ? { ...node, text: "a.name = 'core'" } : node)),
      };
      ctrl.run("setLhsFixtureJson", { json: JSON.stringify(next) });
      expect(ctrl.getJackQueryText()).toContain("a.name = 'core'");
    });

    it("subscribeSnapshot notifies listeners", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      let revision = 0;
      ctrl.subscribeSnapshot(() => {
        revision = ctrl.getInteractionRevision();
      });
      ctrl.run("setLhsFixtureJson", { json: REWRITE_DEFAULT_LHS_FIXTURE_JSON });
      expect(revision).toBeGreaterThan(0);
    });
  });
}
