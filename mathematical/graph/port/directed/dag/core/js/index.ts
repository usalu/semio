// #region 🧲Header
/** @emoji 🌳 `@semio-tech/dag-host-core` — DAG app logic. */
// #endregion 🧲Header

import {
	createPlaygroundApp,
	createProductPlaygroundPlatform,
  AppRuntime,
  CommandBus,
  Controller,
  ModeRuntime,
  WindowKindRuntime,
  buildDagWindowBody,
  buildWriterWindowBody,
  createPlayAppRuntime,
  createStackLayout,
  createJackPlayWindowEngagement,
  enforcePlaygroundWindowEngagementInput,
  JackHoverBridge,
  registerWindowBody,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  uiDeclarativeSectionsToTree,
  UI_INSPECTOR_MIXED_PLACEHOLDER,
  uiInspectorGroupsToTree,
  uiInspectorMixedNumber,
  uiInspectorMixedText,
  uiInspectorReadonlyField,
  type UiInspectorFieldGroup,
  type CommandDescriptor,
  type WindowBodyViewContext,
  type WindowEngagement,
  type UiNode,
  type UiTreeItemNode,
  type UiTreeSectionNode,
  type AppTools,
  type ToolLeaf,
  toolCollection} from "@semio-tech/framework-playground-core";

import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { DocumentVcsStore, createDocumentVcsEnvelope, recordProjectionChange } from "@semio-tech/vcs-core/internal";
import {
  DAG_DEFAULT_FIXTURE,
  DAG_LOD_MODE_AUTOMATIC,
  applyDagFixtureEditOp,
  backwardsDagFixtureEditOp,
  diffDagFixtureEditOp,
  dagPlayLodTiers,
  dagFixtureToJson,
  dagLodAutomaticSelectLabel,
  dagPlayLodTierMenuLabel,
  isDagDrawLodKind,
  type DagDrawLodKind,
  type DagFixtureEditOp,
  type DagFixture,
  type DagLodModeKind,
  type DagNode,
  type DagReorganizeRequest,
} from "@semio-tech/dag-react";
import type { WindowMeasure } from "@semio-tech/framework-playground-core";
import { createWriterDocument, type WriterDocument } from "@semio-tech/writer-core";
import { runJackOnBoardFixture } from "@semio-tech/graph-dsl-core";

export const DAG_PLAY_APP_ID = "dag-play";
export const DAG_PLAY_CONTROLLER_ID = "dag-play";
export const DAG_PLAY_SURFACE_ID = "dag.play";
export const DAG_PLAY_BODY_KEY_MAIN = "dag.play.main";
export const DAG_PLAY_WINDOW_KIND_ID = "dag-main";
export const DAG_PLAY_WINDOW_KIND_JACK = "dag-jack";
export const DAG_PLAY_SURFACE_ID_JACK = "dag.play.jack";
export const DAG_PLAY_BODY_KEY_JACK = "dag.play.jack";
export const DAG_PLAY_DEFAULT_JACK_QUERY = "MATCH (n:computation) RETURN n.name";

export const DAG_ENGAGEMENT_REORGANIZE_ID = "dag.tool.reorganize";
export const DAG_ENGAGEMENT_ORIENTATION_LR_ID = "dag.layout.leftRight";
export const DAG_ENGAGEMENT_ORIENTATION_TB_ID = "dag.layout.topBottom";

export type DagLayoutOrientation = "leftRight" | "topBottom";

const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;

export const DAG_PLAY_DEFAULT_FIXTURE: DagFixture = DAG_DEFAULT_FIXTURE;
export const DAG_PLAY_DEFAULT_FIXTURE_JSON = dagFixtureToJson(DAG_PLAY_DEFAULT_FIXTURE);

export const DAG_PLAY_LAYOUT = createStackLayout([DAG_PLAY_WINDOW_KIND_ID, DAG_PLAY_WINDOW_KIND_JACK], ["DAG", "Jack"]);
export const DAG_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const DAG_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const DAG_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

const DAG_PLAY_CATALOGUE_NODE_KINDS = [
  { kind: "computation", label: "Computation" },
  { kind: "slider", label: "Slider" },
  { kind: "select", label: "Select" },
  { kind: "screen", label: "Screen" },
] as const;

function dagPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: DAG_PLAY_CONTROLLER_ID, command, args };
}

/** @emoji 🧰 DAG play footer toolbar. */
export function buildDagPlayToolbarTools(controllerId: string, orientation: DagLayoutOrientation): AppTools {
  const layoutToggle = (id: string, label: string, value: DagLayoutOrientation): ToolLeaf => ({
    id,
    kind: "toggle",
    label,
    iconId: value === "leftRight" ? "arrow-right" : "arrow-down",
    pressed: orientation === value,
    controllerId,
    command: "setOrientation",
    args: { orientation: value },
  });
  return [
    toolCollection("layout", "layout-grid", [
      { kind: "button", id: "dag.reorganize", label: "Reorganize", iconId: "refresh-cw", controllerId, command: "reorganize" },
      layoutToggle("dag.orientation.lr", "Left to right", "leftRight"),
      layoutToggle("dag.orientation.tb", "Top to bottom", "topBottom"),
    ]),
  ];
}

function buildDagLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: DagLayoutOrientation): string {
  return JSON.stringify({ layerSpacing, siblingGap, orientation });
}

// #region 🔖DagPlayPanels
export function parseDagPlayFixtureJson(json: string): DagFixture | null {
  try {
    const parsed = JSON.parse(json) as DagFixture;
    if (parsed.schema !== "dag.fixture" || !Array.isArray(parsed.nodes) || !Array.isArray(parsed.edges)) {
      return null;
    }
    return parsed;
  } catch {
    return null;
  }
}

export function buildDagPlayHierarchyTree(fixtureJson: string, selectedNodeIds: readonly string[]): UiNode {
  const fixture = parseDagPlayFixtureJson(fixtureJson);
  if (!fixture) {
    return {
      type: "tree",
      sections: [
        {
          id: "dag-play-hierarchy.invalid",
          label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
          defaultOpen: true,
          items: [{ id: "dag-play-hierarchy.invalid.msg", label: "Invalid DAG fixture" }],
        },
      ],
    };
  }
  const nodeItems: UiTreeItemNode[] = fixture.nodes.map((node) => ({
    id: `dag-play-hierarchy.node.${node.id}`,
    label: node.name || node.id,
    description: node.kind,
    command: dagPlayCmd("setSelection", { ids: [node.id] }),
  }));
  const edgeItems: UiTreeItemNode[] = fixture.edges.map((edge) => ({
    id: `dag-play-hierarchy.edge.${edge.id}`,
    label: `${edge.source} → ${edge.target}`,
    description: edge.id,
  }));
  return {
    type: "tree",
    sections: [
      {
        id: "dag-play-hierarchy.nodes",
        label: "Nodes",
        defaultOpen: true,
        items: nodeItems.length ? nodeItems : [{ id: "dag-play-hierarchy.nodes.empty", label: "(none)" }],
      },
      {
        id: "dag-play-hierarchy.edges",
        label: "Edges",
        defaultOpen: false,
        items: edgeItems.length ? edgeItems : [{ id: "dag-play-hierarchy.edges.empty", label: "(none)" }],
      },
    ],
    selectedIds: selectedNodeIds.map((id) => `dag-play-hierarchy.node.${id}`),
  };
}

export function buildDagPlayCatalogueTree(): UiNode {
  return {
    type: "tree",
    sections: [
      {
        id: "dag-play-catalogue.node-kinds",
        label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
        defaultOpen: true,
        items: DAG_PLAY_CATALOGUE_NODE_KINDS.map((entry) => ({
          id: `dag-play-catalogue.kind.${entry.kind}`,
          label: entry.label,
          description: entry.kind,
        })),
      },
    ],
  };
}

function dagPlayInspectorPatch(nodeIds: readonly string[], field: string) {
  return dagPlayCmd("patchDagNodes", { nodeIds, field });
}

function dagPlayInspectorNumberField(nodeIds: readonly string[], fieldId: string, label: string, values: readonly number[], field: string): UiNode {
  const mixed = uiInspectorMixedNumber(values);
  return {
    type: "field",
    id: fieldId,
    label,
    child: {
      type: "input",
      id: `${fieldId}.input`,
      inputKind: "number",
      value: mixed.uniform ? String(mixed.value) : "",
      placeholder: mixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER,
      onChange: dagPlayInspectorPatch(nodeIds, field),
    },
  };
}

function dagPlayInspectorTextField(nodeIds: readonly string[], fieldId: string, label: string, values: readonly string[], field: string): UiNode {
  const mixed = uiInspectorMixedText(values);
  return {
    type: "field",
    id: fieldId,
    label,
    child: {
      type: "input",
      id: `${fieldId}.input`,
      inputKind: "text",
      value: mixed.value,
      placeholder: mixed.placeholder,
      commit: "blur",
      onChange: dagPlayInspectorPatch(nodeIds, field),
    },
  };
}

function dagPlayInspectorKindGroup(kind: DagNode["kind"], nodes: readonly DagNode[]): UiInspectorFieldGroup | null {
  if (!nodes.length) return null;
  const nodeIds = nodes.map((entry) => entry.id);
  const fields: UiNode[] = [];
  if (kind === "slider") {
    fields.push(
      dagPlayInspectorNumberField(nodeIds, "dag-play-inspector.slider-value", "Value", nodes.map((entry) => (entry.kind === "slider" ? entry.value : 0)), "value"),
      dagPlayInspectorNumberField(nodeIds, "dag-play-inspector.slider-min", "Min", nodes.map((entry) => (entry.kind === "slider" ? entry.min : 0)), "min"),
      dagPlayInspectorNumberField(nodeIds, "dag-play-inspector.slider-max", "Max", nodes.map((entry) => (entry.kind === "slider" ? entry.max : 0)), "max"),
    );
  }
  if (kind === "select") {
    fields.push(
      dagPlayInspectorNumberField(nodeIds, "dag-play-inspector.select-index", "Selected option", nodes.map((entry) => (entry.kind === "select" ? entry.selected : 0)), "selected"),
    );
  }
  if (!fields.length) return null;
  return { id: `dag-play-inspector.kind.${kind}`, label: kind, fields };
}

function dagPlayInspectorBaseGroup(nodes: readonly DagNode[]): UiInspectorFieldGroup {
  const nodeIds = nodes.map((entry) => entry.id);
  const names = nodes.map((entry) => entry.name);
  const kinds = nodes.map((entry) => entry.kind);
  const kindMixed = uiInspectorMixedText(kinds);
  const fields: UiNode[] = [];
  if (nodeIds.length === 1) {
    const nodeId = nodeIds[0]!;
    fields.push({
      type: "field",
      id: "dag-play-inspector.id",
      label: "Id",
      child: {
        type: "input",
        id: "dag-play-inspector.id.input",
        inputKind: "text",
        value: nodeId,
        commit: "blur",
        onChange: dagPlayCmd("renameDagNode", { oldId: nodeId }),
      },
    });
  } else {
    fields.push(uiInspectorReadonlyField("dag-play-inspector.id", "Id", `${nodeIds.length} selected`));
  }
  fields.push(
    dagPlayInspectorTextField(nodeIds, "dag-play-inspector.name", "Name", names, "name"),
    uiInspectorReadonlyField(
      "dag-play-inspector.kind",
      "Kind",
      kindMixed.uniform ? (kinds[0] ?? "") : (kindMixed.placeholder ?? UI_INSPECTOR_MIXED_PLACEHOLDER),
    ),
  );
  return { id: "dag-play-inspector.base", label: "Node", fields };
}

export function buildDagPlayInspectorTree(fixtureJson: string, selectedNodeIds: readonly string[]): UiNode {
  const fixture = parseDagPlayFixtureJson(fixtureJson);
  if (!fixture) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "dag-play-inspector.invalid", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Invalid DAG fixture" }] },
    ]);
  }
  if (!selectedNodeIds.length) {
    return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "dag-play-inspector.empty",
        label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
        children: [{ type: "text", value: "Select a node in the hierarchy." }],
      },
    ]);
  }
  const nodes = selectedNodeIds
    .map((id) => fixture.nodes.find((entry) => entry.id === id))
    .filter((node): node is DagNode => Boolean(node));
  if (!nodes.length) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "dag-play-inspector.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Node not found" }] },
    ]);
  }
  const groups: UiInspectorFieldGroup[] = [];
  const kinds = [...new Set(nodes.map((entry) => entry.kind))];
  for (const kind of kinds) {
    const kindNodes = nodes.filter((entry) => entry.kind === kind);
    const kindGroup = dagPlayInspectorKindGroup(kind, kindNodes);
    if (kindGroup) groups.push(kindGroup);
  }
  groups.push(dagPlayInspectorBaseGroup(nodes));
  return uiInspectorGroupsToTree(groups);
}
// #endregion 🔖DagPlayPanels

/** @emoji 🎛 DAG play shell controller. */
export class DagPlayController extends Controller {
  readonly mainMode = new ModeRuntime("main", "Edit", undefined);
  private readonly docStore = new DocumentVcsStore<DagFixture, DagFixtureEditOp>({
    envelope: createDocumentVcsEnvelope("dag.fixture", "dag-play", DAG_PLAY_DEFAULT_FIXTURE),
    applyOp: applyDagFixtureEditOp,
    backwardsOp: backwardsDagFixtureEditOp,
    diffOp: diffDagFixtureEditOp,
  });
  private engagementInput = "";
  private layerSpacing = DEFAULT_LAYER_SPACING;
  private siblingGap = DEFAULT_SIBLING_GAP;
  private orientation: DagLayoutOrientation = "leftRight";
  private reorganizeEpoch = 0;
  private reorganizeOptionsJson = buildDagLayoutOptionsJson(DEFAULT_LAYER_SPACING, DEFAULT_SIBLING_GAP, "leftRight");
  private lodMode: DagLodModeKind = DAG_LOD_MODE_AUTOMATIC;
  private lodModeByInstance: Record<string, DagLodModeKind> = {};
  private effectiveLod: DagDrawLodKind = "normal";
  private interactionRevision = 0;
  private readonly jackBridge = new JackHoverBridge();
  private jackEngagementInput = "";
  private readonly snapshotListeners = new Set<() => void>();

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(DAG_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.jackBridge.setJackQueryText(DAG_PLAY_DEFAULT_JACK_QUERY);
    this.jackBridge.setFixtureJson(this.getFixtureJson());
    this.jackBridge.bindPointerFocus(this.pointerFocus);
    this.rebuildShellMode();
  }

  getFixtureJson(): string {
    return dagFixtureToJson(this.projection());
  }

  getDocumentVcsStore(): DocumentVcsStore<DagFixture, DagFixtureEditOp> {
    return this.docStore;
  }

  private projection(): DagFixture {
    return this.docStore.projection();
  }

  private commitFixture(next: DagFixture): void {
    this.applyFixtureEdit({ op: "setDocument", document: next });
  }

  private applyFixtureEdit(op: DagFixtureEditOp): void {
    recordProjectionChange(this.docStore, [op]);
  }

  getReorganize(): DagReorganizeRequest {
    return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
  }

  lodModeForScope(scopeId: string): DagLodModeKind {
    return this.lodModeByInstance[scopeId] ?? this.lodMode;
  }

  getSelectedNodeIds(): readonly string[] {
    return this.pointerFocus.getSnapshot().selection;
  }

  private setSelectedNodeIds(ids: readonly string[]): void {
    const next = [...new Set(ids.filter((id) => typeof id === "string"))];
    if (JSON.stringify(next) === JSON.stringify(this.getSelectedNodeIds())) return;
    this.pointerFocus.setSelection(next);
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  getInteractionRevision(): number {
    return this.interactionRevision;
  }

  subscribeSnapshot(listener: () => void): () => void {
    this.snapshotListeners.add(listener);
    const unsubJack = this.jackBridge.subscribe(listener);
    return () => {
      this.snapshotListeners.delete(listener);
      unsubJack();
    };
  }

  getJackQueryText(): string {
    return this.jackBridge.getJackQueryText();
  }

  getWriterDocumentJack(): WriterDocument {
    return createWriterDocument({ id: "dag-jack", languageId: "jack", text: this.jackBridge.getJackQueryText() });
  }

  getJackHoverOccurrences(): readonly { readonly start: number; readonly end: number }[] {
    return this.jackBridge.getJackHoverOccurrences();
  }

  getJackSelectOccurrences(): readonly { readonly start: number; readonly end: number }[] {
    return this.jackBridge.getJackSelectOccurrences();
  }

  getHoverEpoch(): number {
    return this.jackBridge.getHoverEpoch();
  }

  getSelectEpoch(): number {
    return this.jackBridge.getSelectEpoch();
  }

  getGraphHighlightedNodeIds(): readonly string[] {
    return this.jackBridge.getGraphHoveredNodeIds();
  }

  private notifySnapshot(): void {
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  private applyFixtureJson(json: string): void {
    const parsed = parseDagPlayFixtureJson(json);
    if (!parsed || dagFixtureToJson(parsed) === this.getFixtureJson()) return;
    this.applyFixtureEdit({ op: "setDocument", document: parsed });
    this.jackBridge.setFixtureJson(this.getFixtureJson());
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  private renameDagNode(oldId: string, newId: string): void {
    const trimmed = newId.trim();
    if (!trimmed || trimmed === oldId) return;
    const fixture = this.projection();
    if (fixture.nodes.some((node) => node.id === trimmed)) return;
    this.setSelectedNodeIds(this.getSelectedNodeIds().map((id) => (id === oldId ? trimmed : id)));
    this.applyFixtureEdit({ op: "renameNode", oldId, newId: trimmed });
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  private patchDagNode(nodeId: string, field: string, value: unknown): void {
    this.applyFixtureEdit({ op: "patchNode", nodeId, field, value });
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  private lodMeasure(scopeId: string): WindowMeasure {
    return {
      kind: "select",
      id: `${scopeId}-lod`,
      label: "LOD",
      value: this.lodModeForScope(scopeId),
      items: [
        { id: "automatic", value: DAG_LOD_MODE_AUTOMATIC, label: dagLodAutomaticSelectLabel(this.effectiveLod) },
        ...dagPlayLodTiers().map((tier) => ({ id: tier, value: tier, label: dagPlayLodTierMenuLabel(tier) })),
      ],
      onChange: { controllerId: DAG_PLAY_CONTROLLER_ID, command: "setLodMode", args: { instanceId: scopeId } },
    };
  }

  private windowMeasures(): readonly WindowMeasure[] {
    return [
      this.lodMeasure(DAG_PLAY_WINDOW_KIND_ID),
      {
        kind: "slider",
        id: "dag-layer-spacing",
        label: "Layer spacing",
        value: this.layerSpacing,
        min: 40,
        max: 320,
        step: 10,
        onChange: dagPlayCmd("setSpacing", { field: "layerSpacing" }),
      },
      {
        kind: "slider",
        id: "dag-sibling-gap",
        label: "Sibling gap",
        value: this.siblingGap,
        min: 10,
        max: 160,
        step: 5,
        onChange: dagPlayCmd("setSpacing", { field: "siblingGap" }),
      },
    ];
  }

  private syncReorganizeOptionsJson(): void {
    this.reorganizeOptionsJson = buildDagLayoutOptionsJson(this.layerSpacing, this.siblingGap, this.orientation);
  }

  private triggerReorganize(): void {
    this.syncReorganizeOptionsJson();
    this.reorganizeEpoch += 1;
    this.rebuildShellMode();
    this.emit();
  }

  private windowEngagement(): WindowEngagement {
    return {
      sessionActive: false,
      input: {
        id: "engagement-input",
        value: this.engagementInput,
        placeholder: "Reorganize, lr, tb",
        onChange: dagPlayCmd("engagementInput"),
        onSubmit: dagPlayCmd("engagementSubmit"),
      },
      possibleEngagements: [
        { id: DAG_ENGAGEMENT_REORGANIZE_ID, label: "Reorganize", command: dagPlayCmd("reorganize") },
        { id: DAG_ENGAGEMENT_ORIENTATION_LR_ID, label: "Left to Right", command: dagPlayCmd("setOrientation", { orientation: "leftRight" }) },
        { id: DAG_ENGAGEMENT_ORIENTATION_TB_ID, label: "Top to Bottom", command: dagPlayCmd("setOrientation", { orientation: "topBottom" }) },
      ],
      status: [{ id: "dag-layout-orientation", text: this.orientation === "leftRight" ? "Left to right" : "Top to bottom" }],
    };
  }

  private jackEngagement(): WindowEngagement {
    return createJackPlayWindowEngagement(DAG_PLAY_WINDOW_KIND_JACK, DAG_PLAY_CONTROLLER_ID, this.jackEngagementInput);
  }

  private rebuildShellMode(): void {
    this.mainMode.tools = buildDagPlayToolbarTools(DAG_PLAY_CONTROLLER_ID, this.orientation);
    this.mainMode.windowKinds = [
      new WindowKindRuntime(DAG_PLAY_WINDOW_KIND_ID, "DAG", DAG_PLAY_BODY_KEY_MAIN, undefined, this.windowMeasures(), this.windowEngagement()),
      new WindowKindRuntime(DAG_PLAY_WINDOW_KIND_JACK, "Jack", DAG_PLAY_BODY_KEY_JACK, undefined, undefined, this.jackEngagement()),
    ];
    for (const windowKind of this.mainMode.windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `DAG play window "${windowKind.id}"`);
    }
  }

  override run(command: string, args?: unknown): void {
    if (command === "jackEngagementInput") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string" && value !== this.jackEngagementInput) {
        this.jackEngagementInput = value;
        this.rebuildShellMode();
        this.emit();
      }
      return;
    }
    if (command === "setJackQuery") {
      const text = (args as { text?: string }).text;
      if (typeof text === "string") {
        this.jackBridge.setJackQueryText(text);
        this.notifySnapshot();
        this.emit();
      }
      return;
    }
    if (command === "setJackHover") {
      this.jackBridge.setJackHover((args as { offset?: number | null }).offset ?? null);
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setJackSelect") {
      this.jackBridge.setJackSelect((args as { start: number; end: number } | null) ?? null);
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setGraphHover") {
      this.jackBridge.setGraphHover((args as { id?: string | null }).id ?? null);
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setGraphSelect") {
      const ids = (args as { ids?: readonly string[] }).ids ?? [];
      this.jackBridge.setGraphSelect(ids);
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "runJackQuery") {
      runJackOnBoardFixture(this.getFixtureJson(), this.jackBridge.getJackQueryText());
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "engagementInput") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string" && value !== this.engagementInput) {
        this.engagementInput = value;
        this.rebuildShellMode();
        this.emit();
      }
      return;
    }
    if (command === "engagementSubmit") {
      const value = (args as { value?: string }).value ?? this.engagementInput;
      this.applyEngagement(value);
      return;
    }
    if (command === "setSpacing") {
      const field = (args as { field?: string; value?: number }).field;
      const value = (args as { value?: number }).value;
      if (typeof value !== "number") return;
      if (field === "layerSpacing") this.layerSpacing = value;
      else if (field === "siblingGap") this.siblingGap = value;
      else return;
      this.syncReorganizeOptionsJson();
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setOrientation") {
      const orientation = (args as { orientation?: DagLayoutOrientation }).orientation;
      if (orientation !== "leftRight" && orientation !== "topBottom") return;
      this.orientation = orientation;
      this.syncReorganizeOptionsJson();
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "reorganize") {
      this.triggerReorganize();
      return;
    }
    if (command === "setFixtureJson") {
      const json = (args as { json?: string }).json;
      if (typeof json === "string") {
        this.applyFixtureJson(json);
      }
      return;
    }
    if (command === "setSelection") {
      const ids = (args as { ids?: string[] }).ids;
      if (!Array.isArray(ids)) return;
      this.setSelectedNodeIds(ids);
      return;
    }
    if (command === "renameDagNode") {
      const oldId = (args as { oldId?: string }).oldId;
      const value = (args as { value?: string }).value;
      if (typeof oldId === "string" && typeof value === "string") {
        this.renameDagNode(oldId, value);
      }
      return;
    }
    if (command === "patchDagNode") {
      const nodeId = (args as { nodeId?: string }).nodeId;
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value;
      if (typeof nodeId === "string" && typeof field === "string") {
        this.patchDagNode(nodeId, field, value);
      }
      return;
    }
    if (command === "patchDagNodes") {
      const nodeIds = (Array.isArray((args as { nodeIds?: string[] }).nodeIds) ? (args as { nodeIds?: string[] }).nodeIds : []).map(String).filter(Boolean);
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value ?? (args as { pressed?: boolean }).pressed;
      if (!nodeIds.length || typeof field !== "string") return;
      this.applyFixtureEdit({ op: "patchNodes", nodeIds, field, value });
      this.interactionRevision += 1;
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setLodMode") {
      const { value, instanceId } = args as { value?: string; instanceId?: string };
      const scopeId = instanceId ?? DAG_PLAY_WINDOW_KIND_ID;
      if (typeof value !== "string") return;
      if (value !== DAG_LOD_MODE_AUTOMATIC && !isDagDrawLodKind(value)) return;
      this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: value as DagLodModeKind };
      if (scopeId === DAG_PLAY_WINDOW_KIND_ID) {
        this.lodMode = value as DagLodModeKind;
      }
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setEffectiveLod") {
      const { lod, instanceId } = args as { lod?: DagDrawLodKind; instanceId?: string };
      const scopeId = instanceId ?? DAG_PLAY_WINDOW_KIND_ID;
      if (!lod || !isDagDrawLodKind(lod)) return;
      if (scopeId !== DAG_PLAY_WINDOW_KIND_ID) return;
      if (this.effectiveLod === lod) return;
      this.effectiveLod = lod;
      this.rebuildShellMode();
      this.emit();
      return;
    }
  }

  private applyEngagement(value: string): void {
    const trimmed = value.trim().toLowerCase();
    if (!trimmed) return;
    if (trimmed === "reorganize" || trimmed === "layout") {
      this.triggerReorganize();
      return;
    }
    if (trimmed === "lr" || trimmed === "left" || trimmed === "left to right") {
      this.orientation = "leftRight";
      this.syncReorganizeOptionsJson();
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (trimmed === "tb" || trimmed === "top" || trimmed === "top to bottom") {
      this.orientation = "topBottom";
      this.syncReorganizeOptionsJson();
      this.rebuildShellMode();
      this.emit();
      return;
    }
    this.engagementInput = "";
    this.rebuildShellMode();
    this.emit();
  }
}

function buildDagPlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildDagWindowBody(DAG_PLAY_SURFACE_ID, DAG_PLAY_CONTROLLER_ID, DAG_PLAY_WINDOW_KIND_ID);
}

function buildDagPlayJackDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildWriterWindowBody(DAG_PLAY_SURFACE_ID_JACK, DAG_PLAY_CONTROLLER_ID, DAG_PLAY_WINDOW_KIND_JACK);
}

export function registerDagPlayDeclarativeBodies(): void {
  registerWindowBody(DAG_PLAY_BODY_KEY_MAIN, buildDagPlayMainDeclarativeBody);
  registerWindowBody(DAG_PLAY_BODY_KEY_JACK, buildDagPlayJackDeclarativeBody);
}

export function buildDagPlayAppRuntime(controller: DagPlayController): AppRuntime {
  return createPlayAppRuntime(DAG_PLAY_APP_ID, "DAG", controller, DAG_PLAY_LAYOUT, controller.mainMode);
}

//#region 🔖Play

/** @emoji 🛝 DAG playground app. */


export const dagPlayAppDefinition = createPlaygroundApp({
	id: DAG_PLAY_APP_ID,
	label: "DAG",
	controllerId: DAG_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "dag",
		resolveDedupe: ["react", "react-dom"],
		watchIgnored: ["../lib.rs", "../target/**", "../Cargo.toml", "../Cargo.lock", "../script.ts"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(DAG_PLAY_APP_ID);
			const ctrl = new DagPlayController(runtime.commandBus, () => runtime.notify());
			runtime.addApp(buildDagPlayAppRuntime(ctrl));
			return runtime;
	},
	registerBodies: () => {
		registerDagPlayDeclarativeBodies();
	},
	bootRenderer: async (pg) => {
		const { bootDagPlay } = await import("@semio-tech/framework-playground-renderer-react/dag");
		bootDagPlay(pg);
	},
});
//#endregion 🔖Play

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("dag play shell", () => {
    it("default fixture has five nodes and four edges", () => {
      expect(DAG_PLAY_DEFAULT_FIXTURE.nodes.length).toBe(5);
      expect(DAG_PLAY_DEFAULT_FIXTURE.edges.length).toBe(4);
    });

    it("reorganize engagement bumps epoch", () => {
      const bus = new CommandBus();
      const ctrl = new DagPlayController(bus, () => {});
      expect(ctrl.getReorganize().epoch).toBe(0);
      ctrl.run("reorganize");
      expect(ctrl.getReorganize().epoch).toBe(1);
      expect(ctrl.getReorganize().optionsJson).toContain("leftRight");
    });

    it("lod window measure lists automatic and tiers", () => {
      const bus = new CommandBus();
      const ctrl = new DagPlayController(bus, () => {});
      const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
      expect(measures.some((measure) => measure.kind === "select" && measure.label === "LOD")).toBe(true);
    });

    it("hierarchy tree lists nodes from default fixture", () => {
      const tree = buildDagPlayHierarchyTree(DAG_PLAY_DEFAULT_FIXTURE_JSON, ["slider"]);
      expect(tree.sections?.some((section) => section.label === "Nodes")).toBe(true);
      expect(tree.selectedIds).toContain("dag-play-hierarchy.node.slider");
    });

    it("setSelection updates interaction revision", () => {
      const bus = new CommandBus();
      const ctrl = new DagPlayController(bus, () => {});
      ctrl.run("setSelection", { ids: ["slider"] });
      expect(ctrl.getSelectedNodeIds()).toEqual(["slider"]);
      expect(ctrl.getInteractionRevision()).toBeGreaterThan(0);
    });

    it("inspector tree exposes slider fields for single selection", () => {
      const tree = buildDagPlayInspectorTree(DAG_PLAY_DEFAULT_FIXTURE_JSON, ["slider"]);
      const serialized = JSON.stringify(tree);
      expect(serialized).toContain("Value");
      expect(serialized).toContain("patchDagNodes");
    });

    it("batch-patches shared fields across multiple nodes", () => {
      const bus = new CommandBus();
      const ctrl = new DagPlayController(bus, () => {});
      const fixture = parseDagPlayFixtureJson(DAG_PLAY_DEFAULT_FIXTURE_JSON);
      if (!fixture) return;
      const baseSlider = fixture.nodes.find((entry) => entry.kind === "slider");
      if (!baseSlider || baseSlider.kind !== "slider") return;
      const secondSlider = { ...baseSlider, id: "slider-copy" };
      const expanded = { ...fixture, nodes: [...fixture.nodes, secondSlider] };
      ctrl.run("setFixtureJson", { json: dagFixtureToJson(expanded) });
      ctrl.run("patchDagNodes", { nodeIds: [baseSlider.id, secondSlider.id], field: "value", value: 5 });
      const updated = parseDagPlayFixtureJson(ctrl.getFixtureJson());
      for (const id of [baseSlider.id, secondSlider.id]) {
        const node = updated?.nodes.find((entry) => entry.id === id);
        expect(node?.kind === "slider" ? node.value : undefined).toBe(5);
      }
    });
  });
}
// #endregion 🧪Tests

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for dag. */
export function buildDagProgramDefinition(): PlatformDefinition {
	return {
		id: "dag",
		name: "DAG",
		apiVersion: "1",
		apps: [{ id: "dag", label: "DAG", controllerId: DAG_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

