// #region 🧲Header
/** @emoji ♻️ `@semio-tech/trinity-rewrite-core` — Trinity Rewrite app logic. */
// #endregion 🧲Header

import {
	createPlaygroundApp,
  AppRuntime,
  CommandBus,
  Controller,
  ModeRuntime,
  Platform,
  WindowKindRuntime,
  buildFormsWindowBody,
  buildPuzzle2dWindowBody,
  buildTrinityWindowBody,
  buildWriterWindowBody,
  createPlayAppRuntime,
  createWindowLayout,
  registerWindowBody,
  registerSidePanelBody,
  buildControllerTreeSidePanelBody,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  type SideTabSpec,
  type UiNode,
  type UiTreeNode,
  type WindowBodyViewContext,
  type WindowLayout,
  type WindowMeasure,
  type WindowEngagement,
  type AppTools,
  toolCollection,
  enforcePlaygroundWindowEngagementInput} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { createWriterDocument, jackSymbolAtOffset, jackVariableOccurrences, type WriterDocument } from "@semio-tech/writer-core";
import type { Puzzle2dFixture, Puzzle2dPreselectSnapshot } from "@semio-tech/puzzle-2d-react";
import {
  REWRITE_DEFAULT_LHS_FIXTURE,
  REWRITE_DEFAULT_LHS_FIXTURE_JSON,
  REWRITE_DEFAULT_RHS_FIXTURE,
  REWRITE_DEFAULT_RHS_FIXTURE_JSON,
  parseRewriteGraphFixtureJson,
  rewriteLhsGraphToJson,
  rewriteLhsMatchQuery,
  rewriteNodeIdsForVar,
  rewriteRhsGraphToJson,
  rewriteVarForNodeId,
} from "@semio-tech/trinity-rewrite-react";
import {
  type FormSpec,
  type FormQuestion,
  type FormValues,
} from "@semio-tech/forms-core";
import {
  TRINITY_DEFAULT_FIXTURE_JSON,
  TRINITY_LOD_MODE_AUTOMATIC,
  applyRewriteOnFixture,
  buildTrinityPlayCatalogueTree,
  buildTrinityPlayHierarchyTree,
  buildTrinityPlayInspectorTree,
  isTrinityDrawLodKind,
  parseTrinityFixtureJson,
  ruleQueryOnFixture,
  runJackOnFixture,
  trinityLodAutomaticSelectLabel,
  trinityPlayLodTierMenuLabel,
  trinityPlayLodTiers,
  type RuleParameter,
  type TrinityDrawLodKind,
  type TrinityFixture,
  type TrinityLodModeKind,
  type TrinityVcsCommandKind,
  type TrinityJackDispatchRequest,
  type TrinityVcsRequest,
} from "@semio-tech/trinity-react";

export {
  buildTrinityPlayCatalogueTree,
  buildTrinityPlayHierarchyTree,
  buildTrinityPlayInspectorTree,
} from "@semio-tech/trinity-react";

export const TRINITY_REWRITE_PLAY_APP_ID = "trinity-rewrite-play";
export const TRINITY_REWRITE_PLAY_CONTROLLER_ID = "trinity-rewrite-play";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_BEFORE = "trinity.rewrite.before";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_AFTER = "trinity.rewrite.after";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_LHS = "trinity.rewrite.lhs";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_RHS = "trinity.rewrite.rhs";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_JACK = "trinity.rewrite.jack";
export const TRINITY_REWRITE_PLAY_SURFACE_ID_PARAMETERS = "trinity.rewrite.parameters";
export const TRINITY_REWRITE_PLAY_BODY_KEY_BEFORE = "trinity.rewrite.play.before";
export const TRINITY_REWRITE_PLAY_BODY_KEY_AFTER = "trinity.rewrite.play.after";
export const TRINITY_REWRITE_PLAY_BODY_KEY_LHS = "trinity.rewrite.play.lhs";
export const TRINITY_REWRITE_PLAY_BODY_KEY_RHS = "trinity.rewrite.play.rhs";
export const TRINITY_REWRITE_PLAY_BODY_KEY_JACK = "trinity.rewrite.play.jack";
export const TRINITY_REWRITE_PLAY_BODY_KEY_PARAMETERS = "trinity.rewrite.play.parameters";
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE = "trinity-rewrite-before";
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER = "trinity-rewrite-after";
export const TRINITY_REWRITE_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const TRINITY_REWRITE_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const TRINITY_REWRITE_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const TRINITY_REWRITE_PLAY_HIERARCHY_BODY_KEY = "trinity.rewrite.play.hierarchy";
export const TRINITY_REWRITE_PLAY_CATALOGUE_BODY_KEY = "trinity.rewrite.play.catalogue";
export const TRINITY_REWRITE_PLAY_INSPECTION_BODY_KEY = "trinity.rewrite.play.inspection";

function trinityRewritePlayCmd(command: string, args?: Record<string, unknown>) {
  return { controllerId: TRINITY_REWRITE_PLAY_CONTROLLER_ID, command, args };
}

/** @emoji 🧰 Trinity rewrite play footer toolbar. */
export function buildTrinityRewritePlayToolbarTools(controllerId: string): AppTools {
  return [
    toolCollection("history", "history", [
      { kind: "button", id: "trinity-rewrite.undo", label: "Undo", iconId: "undo-2", controllerId, command: "undo" },
      { kind: "button", id: "trinity-rewrite.redo", label: "Redo", iconId: "redo-2", controllerId, command: "redo" },
      { kind: "button", id: "trinity-rewrite.checkpoint", label: "Checkpoint", iconId: "git-commit", controllerId, command: "commitCheckpoint" },
    ]),
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
  parseRewriteGraphFixtureJson,
  rewriteLhsMatchQuery,
  rewriteNodeIdsForVar,
  rewriteVarForNodeId,
} from "@semio-tech/trinity-rewrite-react";

export const TRINITY_REWRITE_PLAY_DEFAULT_LHS_JSON = rewriteLhsGraphToJson(REWRITE_DEFAULT_LHS_FIXTURE);

export const TRINITY_REWRITE_PLAY_DEFAULT_RHS_JSON = rewriteRhsGraphToJson(REWRITE_DEFAULT_RHS_FIXTURE);

function parseRhsParameters(rhsJson: string): readonly RuleParameter[] {
  try {
    const rhs = JSON.parse(rhsJson) as { parameters?: readonly RuleParameter[] };
    return rhs.parameters ?? [];
  } catch {
    return [];
  }
}

function parameterDefaultValues(parameters: readonly RuleParameter[]): FormValues {
  const values: Record<string, FormValues[string]> = {};
  for (const param of parameters) {
    values[param.name] = param.default as FormValues[string];
  }
  return values;
}

function buildParameterFormSpec(parameters: readonly RuleParameter[]): FormSpec {
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
    schema: "forms.form",
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

export const trinityRewritePlayWindowBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").WindowBodyViewContext) => UiNode>> = {
  [TRINITY_REWRITE_PLAY_BODY_KEY_BEFORE]: buildTrinityRewritePlayBeforeDeclarativeBody,
  [TRINITY_REWRITE_PLAY_BODY_KEY_AFTER]: buildTrinityRewritePlayAfterDeclarativeBody,
  [TRINITY_REWRITE_PLAY_BODY_KEY_LHS]: buildTrinityRewritePlayLhsDeclarativeBody,
  [TRINITY_REWRITE_PLAY_BODY_KEY_RHS]: buildTrinityRewritePlayRhsDeclarativeBody,
  [TRINITY_REWRITE_PLAY_BODY_KEY_JACK]: buildTrinityRewritePlayJackDeclarativeBody,
  [TRINITY_REWRITE_PLAY_BODY_KEY_PARAMETERS]: buildTrinityRewritePlayParametersDeclarativeBody,
};

export function registerTrinityRewritePlayDeclarativeBodies(): void {
  for (const [key, build] of Object.entries(trinityRewritePlayWindowBodies)) registerWindowBody(key, build);
  for (const [key, build] of Object.entries(trinityRewritePlaySidePanelBodies)) registerSidePanelBody(key, build);
}

function buildTrinityRewritePlayHierarchyPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
  return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
    const rewriteCtrl = ctrl as TrinityRewritePlayController;
    return buildTrinityPlayHierarchyTree(rewriteCtrl.getBeforeFixtureJson(), rewriteCtrl.getSelectedNodeIds()) as UiTreeNode;
  });
}

function buildTrinityRewritePlayCataloguePanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
  return buildControllerTreeSidePanelBody(ctx, () => buildTrinityPlayCatalogueTree() as UiTreeNode);
}

function buildTrinityRewritePlayInspectionPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
  return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
    const rewriteCtrl = ctrl as TrinityRewritePlayController;
    return buildTrinityPlayInspectorTree(
      rewriteCtrl.getBeforeFixtureJson(),
      rewriteCtrl.getSelectedNodeIds(),
      TRINITY_REWRITE_PLAY_CONTROLLER_ID,
    ) as UiTreeNode;
  });
}

export const trinityRewritePlaySidePanelBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext) => UiTreeNode>> = {
  [TRINITY_REWRITE_PLAY_HIERARCHY_BODY_KEY]: buildTrinityRewritePlayHierarchyPanelBody,
  [TRINITY_REWRITE_PLAY_CATALOGUE_BODY_KEY]: buildTrinityRewritePlayCataloguePanelBody,
  [TRINITY_REWRITE_PLAY_INSPECTION_BODY_KEY]: buildTrinityRewritePlayInspectionPanelBody,
};

export class TrinityRewritePlayController extends Controller {
  readonly mainMode = new ModeRuntime("explore", "Explore", undefined);
  private beforeFixtureJson = TRINITY_DEFAULT_FIXTURE_JSON;
  private lhsFixture: Puzzle2dFixture = REWRITE_DEFAULT_LHS_FIXTURE;
  private rhsFixture: Puzzle2dFixture = REWRITE_DEFAULT_RHS_FIXTURE;
  private parameterValues: FormValues = parameterDefaultValues(parseRhsParameters(TRINITY_REWRITE_PLAY_DEFAULT_RHS_JSON));
  private jackQueryText = "";
  private afterFixtureJson = TRINITY_DEFAULT_FIXTURE_JSON;
  private vcsEpoch = 0;
  private vcsKind: TrinityVcsCommandKind = "undo";
  private vcsMessage = "";
  private beforeJackDispatchEpoch = 0;
  private beforeJackDispatchQuery = "";
  private storeGeneration = 0;
  private activeHoverVar: string | null = null;
  private activeSelectVar: string | null = null;
  private hoverEpoch = 0;
  private selectEpoch = 0;
  private lhsHoveredNodeId: string | null = null;
  private rhsHoveredNodeId: string | null = null;
  private lhsSelectedNodeIds: string[] = [];
  private rhsSelectedNodeIds: string[] = [];
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
    return this.beforeFixtureJson;
  }

  getFixtureJson(): string {
    return this.getBeforeFixtureJson();
  }

  getAfterFixtureJson(): string {
    return this.afterFixtureJson;
  }

  getVcsRequest(): TrinityVcsRequest | undefined {
    return this.vcsEpoch > 0 ? { kind: this.vcsKind, epoch: this.vcsEpoch, message: this.vcsMessage || undefined } : undefined;
  }

  getBeforeJackDispatch(): TrinityJackDispatchRequest | undefined {
    return this.beforeJackDispatchEpoch > 0 ? { query: this.beforeJackDispatchQuery, epoch: this.beforeJackDispatchEpoch } : undefined;
  }

  getStoreGeneration(): number {
    return this.storeGeneration;
  }

  private setBeforeFixtureJson(next: string): void {
    this.beforeFixtureJson = next;
  }

  private requestVcs(kind: TrinityVcsCommandKind, message = ""): void {
    this.vcsKind = kind;
    this.vcsMessage = message;
    this.vcsEpoch += 1;
  }

  private dispatchBeforeJackQuery(query: string): void {
    this.beforeJackDispatchQuery = query;
    this.beforeJackDispatchEpoch += 1;
  }

  onBeforeJackDispatchComplete(resultJson: string): void {
    try {
      const result = JSON.parse(resultJson) as { fixtureJson?: string };
      if (typeof result.fixtureJson === "string" && parseTrinityFixtureJson(result.fixtureJson)) {
        this.setBeforeFixtureJson(result.fixtureJson);
      }
    } catch {
      /* dispatch result unavailable */
    }
    this.bump();
  }

  onVcsApplied(generation: number): void {
    this.storeGeneration = generation;
    this.bump();
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

  getWriterDocumentJack(): WriterDocument {
    return createWriterDocument({
      id: "rewrite-jack",
      languageId: "jack",
      uri: "writer://rewrite-jack",
      text: this.jackQueryText,
    });
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

  getHoverEpoch(): number {
    return this.hoverEpoch;
  }

  getSelectEpoch(): number {
    return this.selectEpoch;
  }

  getActiveHoverVar(): string | null {
    return this.activeHoverVar;
  }

  getActiveSelectVar(): string | null {
    return this.activeSelectVar;
  }

  getLhsHoveredNodeId(): string | null {
    return this.lhsHoveredNodeId;
  }

  getRhsHoveredNodeId(): string | null {
    return this.rhsHoveredNodeId;
  }

  getLhsHoveredNodeIds(): readonly string[] {
    return this.activeHoverVar ? rewriteNodeIdsForVar(this.lhsFixture, this.activeHoverVar) : [];
  }

  getRhsHoveredNodeIds(): readonly string[] {
    return this.activeHoverVar ? rewriteNodeIdsForVar(this.rhsFixture, this.activeHoverVar) : [];
  }

  getLhsVarPreselection(): Puzzle2dPreselectSnapshot {
    if (!this.activeHoverVar) return { ids: [], removedIds: [] };
    const ids = rewriteNodeIdsForVar(this.lhsFixture, this.activeHoverVar);
    const removedIds = this.lhsHoveredNodeId ? ids.filter((id) => id !== this.lhsHoveredNodeId) : [...ids];
    return { ids: [], removedIds };
  }

  getRhsVarPreselection(): Puzzle2dPreselectSnapshot {
    if (!this.activeHoverVar) return { ids: [], removedIds: [] };
    const ids = rewriteNodeIdsForVar(this.rhsFixture, this.activeHoverVar);
    const removedIds = this.rhsHoveredNodeId ? ids.filter((id) => id !== this.rhsHoveredNodeId) : [...ids];
    return { ids: [], removedIds: [] };
  }

  getLhsVarSelection(): readonly string[] {
    return this.activeSelectVar ? rewriteNodeIdsForVar(this.lhsFixture, this.activeSelectVar) : [];
  }

  getRhsVarSelection(): readonly string[] {
    return this.activeSelectVar ? rewriteNodeIdsForVar(this.rhsFixture, this.activeSelectVar) : [];
  }

  getJackHoverOccurrences(): readonly { readonly start: number; readonly end: number }[] {
    if (!this.activeHoverVar) return [];
    return jackVariableOccurrences(this.jackQueryText, this.activeHoverVar);
  }

  getJackSelectOccurrences(): readonly { readonly start: number; readonly end: number }[] {
    if (!this.activeSelectVar) return [];
    return jackVariableOccurrences(this.jackQueryText, this.activeSelectVar);
  }

  getBeforeHighlightedNodeIds(): readonly string[] {
    return this.boundNodeIdsForActiveVar(this.beforeFixtureJson);
  }

  getAfterHighlightedNodeIds(): readonly string[] {
    return this.boundNodeIdsForActiveVar(this.afterFixtureJson);
  }

  private boundNodeIdsForActiveVar(fixtureJson: string): readonly string[] {
    const activeVar = this.activeHoverVar ?? this.activeSelectVar;
    if (!activeVar) return [];
    try {
      const query = rewriteLhsMatchQuery(this.getLhsJson(), activeVar);
      const result = runJackOnFixture(fixtureJson, query);
      if (result.kind === "graph" && result.graphFixture?.nodes) {
        return result.graphFixture.nodes.map((node) => node.id);
      }
    } catch {
      /* match preview unavailable */
    }
    return [];
  }

  private setActiveHoverVar(next: string | null): void {
    if (this.activeHoverVar === next) return;
    this.activeHoverVar = next;
    this.hoverEpoch += 1;
    this.notifySnapshot();
    this.emit();
  }

  private setActiveSelectVar(next: string | null): void {
    if (this.activeSelectVar === next) return;
    this.activeSelectVar = next;
    this.selectEpoch += 1;
    this.notifySnapshot();
    this.emit();
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
      if (typeof json === "string" && parseTrinityFixtureJson(json)) {
        this.setBeforeFixtureJson(json);
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
      this.pointerFocus.setSelection([...ids]);
      this.bump();
      return;
    }
    if (command === "setLhsGraphHover") {
      const id = (args as { id?: string | null }).id ?? null;
      this.lhsHoveredNodeId = id;
      this.setActiveHoverVar(id ? rewriteVarForNodeId(this.lhsFixture, id) : null);
      return;
    }
    if (command === "setRhsGraphHover") {
      const id = (args as { id?: string | null }).id ?? null;
      this.rhsHoveredNodeId = id;
      this.setActiveHoverVar(id ? rewriteVarForNodeId(this.rhsFixture, id) : null);
      return;
    }
    if (command === "setJackHover") {
      const offset = (args as { offset?: number | null }).offset;
      if (offset == null) {
        this.lhsHoveredNodeId = null;
        this.rhsHoveredNodeId = null;
        this.setActiveHoverVar(null);
      } else {
        const symbol = jackSymbolAtOffset(this.jackQueryText, offset);
        this.setActiveHoverVar(symbol?.kind === "variable" ? symbol.name : null);
      }
      return;
    }
    if (command === "setLhsGraphSelect") {
      const ids = (args as { ids?: string[] }).ids ?? [];
      this.lhsSelectedNodeIds = [...ids];
      this.setActiveSelectVar(ids.length === 1 ? rewriteVarForNodeId(this.lhsFixture, ids[0]!) : null);
      return;
    }
    if (command === "setRhsGraphSelect") {
      const ids = (args as { ids?: string[] }).ids ?? [];
      this.rhsSelectedNodeIds = [...ids];
      this.setActiveSelectVar(ids.length === 1 ? rewriteVarForNodeId(this.rhsFixture, ids[0]!) : null);
      return;
    }
    if (command === "setJackSelect") {
      const { start, end } = args as { start?: number; end?: number };
      if (start == null || end == null) {
        this.setActiveSelectVar(null);
        return;
      }
      const offset = Math.floor((start + end) / 2);
      const symbol = jackSymbolAtOffset(this.jackQueryText, offset);
      this.setActiveSelectVar(symbol?.kind === "variable" ? symbol.name : null);
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
      const fixture = parseTrinityFixtureJson(this.beforeFixtureJson);
      if (!fixture) return;
      const queries = nodeIds
        .map((id) => {
          const node = fixture.nodes.find((row) => row.id === id);
          if (!node) return null;
          return `MATCH (n:${node.kind}) WHERE n.id = '${id}' SET n.name = '${escaped}'`;
        })
        .filter((query): query is string => query !== null);
      if (queries.length) this.dispatchBeforeJackQuery(queries.join("\n"));
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

export function buildTrinityRewritePlayAppRuntime(ctrl: TrinityRewritePlayController): AppRuntime {
  const app = createPlayAppRuntime(TRINITY_REWRITE_PLAY_APP_ID, "Trinity Rewrite", ctrl, buildTrinityRewritePlayLayout(), ctrl.mainMode);
  app.panelTabs = [
    { id: TRINITY_REWRITE_PLAY_HIERARCHY_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, panel: "workbench", order: 0, bodyKey: TRINITY_REWRITE_PLAY_HIERARCHY_BODY_KEY, label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL },
    { id: TRINITY_REWRITE_PLAY_CATALOGUE_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, panel: "workbench", order: 1, bodyKey: TRINITY_REWRITE_PLAY_CATALOGUE_BODY_KEY, label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL },
    { id: TRINITY_REWRITE_PLAY_INSPECTION_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, panel: "details", order: 0, bodyKey: TRINITY_REWRITE_PLAY_INSPECTION_BODY_KEY, label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL },
  ] satisfies SideTabSpec[];
  return app;
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
      const dispatch = ctrl.getBeforeJackDispatch();
      expect(dispatch).toBeDefined();
      const result = runJackOnFixture(ctrl.getBeforeFixtureJson(), dispatch!.query);
      ctrl.onBeforeJackDispatchComplete(JSON.stringify(result));
      if (result.fixtureJson !== ctrl.getBeforeFixtureJson()) {
        ctrl.run("setFixtureJson", { json: result.fixtureJson });
      }
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

    it("lhs hover bridges variable to jack occurrences and rhs set node", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      ctrl.run("setLhsGraphHover", { id: "match-a" });
      expect(ctrl.getActiveHoverVar()).toBe("a");
      expect(ctrl.getLhsHoveredNodeIds()).toEqual(["match-a", "where-b"]);
      expect(ctrl.getRhsHoveredNodeIds()).toEqual(["set-label"]);
      expect(ctrl.getJackHoverOccurrences().length).toBeGreaterThan(0);
    });

    it("jack hover bridges variable to lhs nodes", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      const offset = ctrl.getJackQueryText().indexOf("a");
      expect(offset).toBeGreaterThanOrEqual(0);
      ctrl.run("setJackHover", { offset });
      expect(ctrl.getActiveHoverVar()).toBe("a");
      expect(ctrl.getLhsHoveredNodeIds()).toContain("match-a");
    });

    it("active variable highlights bound before nodes", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      ctrl.run("setLhsGraphHover", { id: "match-a" });
      const highlighted = ctrl.getBeforeHighlightedNodeIds();
      expect(highlighted.length).toBeGreaterThan(0);
    });
  });
}

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for trinity rewrite. */
export function buildTrinityRewriteProgramDefinition(): PlatformDefinition {
	return {
		id: "trinity.rewrite",
		name: "Trinity Rewrite",
		apiVersion: "1",
		apps: [{ id: "trinity-rewrite", label: "Trinity Rewrite", controllerId: TRINITY_REWRITE_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖OsProgram
import { mergeOsProgramDefinition, osBaselineResource, registerAppVcsHandler } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";

const trinityRewriteProgramContributionResources = {
		"trinity-rewrite": osBaselineResource("graph.trinity", "trinity.graph", "trinityRewrite", [{ id: "edit", label: "Edit" }]),
	};

/** @emoji 🧩 OS program contribution for trinity.rewrite. */
export const trinityRewriteProgramContribution: OsProgramContribution = {
	programId: "trinity.rewrite",
	register() {
		mergeOsProgramDefinition("trinity.rewrite", buildTrinityRewriteProgramDefinition(), trinityRewriteProgramContributionResources);
		registerAppVcsHandler(createTrinityGraphAppVcsHandler());
	},
};
//#endregion 🔖OsProgram

//#region 🔖Play

/** @emoji 🛝 Trinity Rewrite playground app. */


export const trinityRewritePlayAppDefinition = createPlaygroundApp({
	id: TRINITY_REWRITE_PLAY_APP_ID,
	label: "Trinity Rewrite",
	controllerId: TRINITY_REWRITE_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "trinity-rewrite",
		resolveDedupe: ["react", "react-dom", "three"],
		watchIgnored: ["../engine/lib.rs", "../engine/target/**", "../engine/Cargo.toml"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime"] },
	},
	runtimeBootstrap: {
		createController: (bus, notify) => new TrinityRewritePlayController(bus, notify),
		buildAppRuntime: buildTrinityRewritePlayAppRuntime,
	},
});
//#endregion 🔖Play
//#region 🔖DocumentVcs
import { createTypedAppVcsHandler } from "@semio-tech/framework-os-core";

/** @emoji 🔺 S app VCS handler for trinity graph documents. */
export function createTrinityGraphAppVcsHandler() {
	type Doc = { readonly nodes: readonly unknown[] };
	type Op = { readonly op: "setNodes"; readonly nodes: readonly unknown[] };
	return createTypedAppVcsHandler<Doc, Op>("trinity.graph", "trinity.graph", () => ({ nodes: [] }), (doc, op) => ({ nodes: op.nodes }));
}
//#endregion 🔖DocumentVcs
