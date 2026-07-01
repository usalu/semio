// #region 🧲Header
/** @emoji 🌳 DAG play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
  AppRuntime,
  CommandBus,
  Controller,
  ModeRuntime,
  Platform,
  Playground,
  WindowKindRuntime,
  buildDagWindowBody,
  createPlayAppRuntime,
  createProductPlaygroundPlatform,
  createStackLayout,
  enforcePlaygroundWindowEngagementInput,
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
} from "@semio-tech/framework-playground-core";

import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
  DocumentVcsStore,
  createDocumentVcsEnvelope,
  recordProjectionChange,
} from "@semio-tech/framework-core";
import {
  DAG_DEFAULT_FIXTURE,
  DAG_LOD_MODE_AUTOMATIC,
  dagPlayLodTiers,
  dagFixtureToJson,
  dagLodAutomaticSelectLabel,
  dagPlayLodTierMenuLabel,
  isDagDrawLodKind,
  type DagDrawLodKind,
  type DagFixtureV1,
  type DagLodModeKind,
  type DagNodeV1,
  type DagReorganizeRequest,
} from "@semio-tech/dag-react";
import type { WindowMeasure } from "@semio-tech/framework-playground-core";

export const DAG_PLAY_APP_ID = "dag-play";
export const DAG_PLAY_CONTROLLER_ID = "dag-play";
export const DAG_PLAY_SURFACE_ID = "dag.play/v1";
export const DAG_PLAY_BODY_KEY_MAIN = "dag.play.main";
export const DAG_PLAY_WINDOW_KIND_ID = "dag-main";

export const DAG_ENGAGEMENT_REORGANIZE_ID = "dag.tool.reorganize";
export const DAG_ENGAGEMENT_ORIENTATION_LR_ID = "dag.layout.leftRight";
export const DAG_ENGAGEMENT_ORIENTATION_TB_ID = "dag.layout.topBottom";

export type DagLayoutOrientation = "leftRight" | "topBottom";

const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;

export const DAG_PLAY_DEFAULT_FIXTURE: DagFixtureV1 = DAG_DEFAULT_FIXTURE;
export const DAG_PLAY_DEFAULT_FIXTURE_JSON = dagFixtureToJson(DAG_PLAY_DEFAULT_FIXTURE);

type DagFixtureEditOp = { readonly op: "setDocument"; readonly document: DagFixtureV1 };

function applyDagFixtureEditOp(_fixture: DagFixtureV1, op: DagFixtureEditOp): DagFixtureV1 {
  return op.document;
}

export const DAG_PLAY_LAYOUT = createStackLayout([DAG_PLAY_WINDOW_KIND_ID], ["DAG"]);
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

function buildDagLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: DagLayoutOrientation): string {
  return JSON.stringify({ layerSpacing, siblingGap, orientation });
}

// #region 🔖DagPlayPanels
export function parseDagPlayFixtureJson(json: string): DagFixtureV1 | null {
  try {
    const parsed = JSON.parse(json) as DagFixtureV1;
    if (parsed.schema !== "dag.fixture/v1" || !Array.isArray(parsed.nodes) || !Array.isArray(parsed.edges)) {
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

function dagPlayInspectorKindGroup(kind: DagNodeV1["kind"], nodes: readonly DagNodeV1[]): UiInspectorFieldGroup | null {
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

function dagPlayInspectorBaseGroup(nodes: readonly DagNodeV1[]): UiInspectorFieldGroup {
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
    .filter((node): node is DagNodeV1 => Boolean(node));
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
  private readonly docStore = new DocumentVcsStore<DagFixtureV1, DagFixtureEditOp>({
    envelope: createDocumentVcsEnvelope("dag.fixture/v1", "dag-play", DAG_PLAY_DEFAULT_FIXTURE),
    applyOp: applyDagFixtureEditOp,
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
  private selectedNodeIds: string[] = [];
  private interactionRevision = 0;
  private readonly snapshotListeners = new Set<() => void>();

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(DAG_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.rebuildShellMode();
  }

  getFixtureJson(): string {
    return dagFixtureToJson(this.projection());
  }

  getDocumentVcsStore(): DocumentVcsStore<DagFixtureV1, DagFixtureEditOp> {
    return this.docStore;
  }

  private projection(): DagFixtureV1 {
    return this.docStore.projection();
  }

  private commitFixture(next: DagFixtureV1): void {
    const previous = this.projection();
    recordProjectionChange(this.docStore, [{ op: "setDocument", document: next }]);
  }

  getReorganize(): DagReorganizeRequest {
    return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
  }

  lodModeForScope(scopeId: string): DagLodModeKind {
    return this.lodModeByInstance[scopeId] ?? this.lodMode;
  }

  getSelectedNodeIds(): readonly string[] {
    return this.selectedNodeIds;
  }

  getInteractionRevision(): number {
    return this.interactionRevision;
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

  private applyFixtureJson(json: string): void {
    const parsed = parseDagPlayFixtureJson(json);
    if (!parsed || dagFixtureToJson(parsed) === this.getFixtureJson()) return;
    this.commitFixture(parsed);
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  private renameDagNode(oldId: string, newId: string): void {
    const trimmed = newId.trim();
    if (!trimmed || trimmed === oldId) return;
    const fixture = this.projection();
    if (fixture.nodes.some((node) => node.id === trimmed)) return;
    const nodes = fixture.nodes.map((node) => (node.id === oldId ? ({ ...node, id: trimmed } as DagNodeV1) : node));
    const remapPort = (port: string) => (port.startsWith(`${oldId}:`) ? `${trimmed}:${port.slice(oldId.length + 1)}` : port);
    const edges = fixture.edges.map((edge) => ({
      ...edge,
      source: remapPort(edge.source),
      target: remapPort(edge.target),
    }));
    this.selectedNodeIds = this.selectedNodeIds.map((id) => (id === oldId ? trimmed : id));
    this.commitFixture({ ...fixture, nodes, edges });
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  private patchDagNode(nodeId: string, field: string, value: unknown): void {
    const fixture = this.projection();
    const nodes = fixture.nodes.map((node) => {
      if (node.id !== nodeId) return node;
      if (field === "value" || field === "min" || field === "max" || field === "step" || field === "selected") {
        const numeric = typeof value === "number" ? value : Number(value);
        if (!Number.isFinite(numeric)) return node;
        return { ...node, [field]: numeric } as DagNodeV1;
      }
      if (typeof value !== "string") return node;
      return { ...node, [field]: value } as DagNodeV1;
    });
    this.commitFixture({ ...fixture, nodes });
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
    return [this.lodMeasure(DAG_PLAY_WINDOW_KIND_ID)];
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
      controls: [
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
      ],
      status: [{ id: "dag-layout-orientation", text: this.orientation === "leftRight" ? "Left to right" : "Top to bottom" }],
    };
  }

  private rebuildShellMode(): void {
    this.mainMode.windowKinds = [
      new WindowKindRuntime(DAG_PLAY_WINDOW_KIND_ID, "DAG", DAG_PLAY_BODY_KEY_MAIN, undefined, this.windowMeasures(), this.windowEngagement()),
    ];
    for (const windowKind of this.mainMode.windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `DAG play window "${windowKind.id}"`);
    }
  }

  override run(command: string, args?: unknown): void {
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
      const next = [...new Set(ids.filter((id) => typeof id === "string"))];
      if (JSON.stringify(next) === JSON.stringify(this.selectedNodeIds)) return;
      this.selectedNodeIds = next;
      this.interactionRevision += 1;
      this.notifySnapshot();
      this.emit();
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
      let next = this.projection();
      for (const nodeId of nodeIds) {
        const fixture = next;
        const nodes = fixture.nodes.map((node) => {
          if (node.id !== nodeId) return node;
          if (field === "value" || field === "min" || field === "max" || field === "step" || field === "selected") {
            const numeric = typeof value === "number" ? value : Number(value);
            if (!Number.isFinite(numeric)) return node;
            return { ...node, [field]: numeric } as DagNodeV1;
          }
          if (typeof value !== "string") return node;
          return { ...node, [field]: value } as DagNodeV1;
        });
        next = { ...fixture, nodes };
      }
      this.commitFixture(next);
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

export function registerDagPlayDeclarativeBodies(): void {
  registerWindowBody(DAG_PLAY_BODY_KEY_MAIN, buildDagPlayMainDeclarativeBody);
}

export function buildDagPlayAppRuntime(controller: DagPlayController): AppRuntime {
  return createPlayAppRuntime(DAG_PLAY_APP_ID, "DAG", controller, DAG_PLAY_LAYOUT, controller.mainMode);
}

export class PlaygroundDag extends Playground {
  readonly id = DAG_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = createProductPlaygroundPlatform(this.id);
    const ctrl = new DagPlayController(runtime.commandBus, () => runtime.notify());
    runtime.addApp(buildDagPlayAppRuntime(ctrl));
    return runtime;
  }

  registerBodies(): void {
    registerDagPlayDeclarativeBodies();
  }
}

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

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "dag") {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootDagPlay } = await import("@semio-tech/framework-playground-renderer-react/dag");
    bootDagPlay(new PlaygroundDag());
  })();
}
// #endregion 🔖Boot
