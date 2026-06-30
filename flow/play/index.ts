// #region 🧲Header
/** @emoji 🌊 Flow play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
  AppRuntime,
  CommandBus,
  Controller,
  ModeRuntime,
  Platform,
  Playground,
  WindowKindRuntime,
  buildFlowWindowBody,
  buildFormsWindowBody,
  createPlayAppRuntime,
  createProductPlaygroundPlatform,
  createStackLayout,
  enforcePlaygroundWindowEngagementInput,
  registerWindowBody,
  type CommandDescriptor,
  type WindowBodyViewContext,
  type WindowEngagement,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  uiDeclarativeSectionsToTree,
  type UiNode,
  type UiSectionNode,
  type UiTreeItemNode,
  type UiTreeSectionNode,
} from "@semio-tech/framework-playground-core";

import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
  DAG_LOD_MODE_AUTOMATIC,
  dagPlayLodTiers,
  FLOW_DEFAULT_FIXTURE,
  dagLodAutomaticSelectLabel,
  flowExtensionHost,
  flowFixtureToJson,
  flowPlayCatalogueItemDragData,
  isDagDrawLodKind,
  dagPlayLodTierMenuLabel,
  buildCatalogueKindsTreeSections,
  type CatalogueSection,
  type DagDrawLodKind,
  type DagLodModeKind,
  buildFlowContextMenuItems,
  FLOW_DEFAULT_PROXIMITY_DISTANCE,
  type FlowCanvasCommandRequest,
  type FlowCanvasContextMenuContext,
  type FlowContextMenuDispatch,
  type FlowExtensionEntry,
  type FlowFixtureV1,
  type FlowReorganizeRequest,
  type FlowWidgetV1,
} from "@semio-tech/flow-react";
import { createFormId } from "@semio-tech/forms-core";
import { applyGenerationValuesToFixture, flowFixtureToFormSpec, type FlowGeneration } from "@semio-tech/forms-react";
import { FlowOrchestratorClient } from "../worker-client.ts";
import type { ContextMenuItem } from "@semio-tech/ui-react";
import type { WindowMeasure } from "@semio-tech/framework-playground-core";

export const FLOW_PLAY_APP_ID = "flow-play";
export const FLOW_PLAY_CONTROLLER_ID = "flow-play";
export const FLOW_PLAY_SURFACE_ID = "flow.play/v1";
export const FLOW_PLAY_BODY_KEY_MAIN = "flow.play.main";
export const FLOW_PLAY_BODY_KEY_GENERATE = "flow.play.generate";
export const FLOW_PLAY_SURFACE_ID_GENERATE = "flow.play.generate/v1";
export const FLOW_PLAY_WINDOW_KIND_ID = "flow-main";

export const FLOW_ENGAGEMENT_REORGANIZE_ID = "flow.tool.reorganize";
export const FLOW_ENGAGEMENT_ORIENTATION_LR_ID = "flow.layout.leftRight";
export const FLOW_ENGAGEMENT_ORIENTATION_TB_ID = "flow.layout.topBottom";

export type FlowLayoutOrientation = "leftRight" | "topBottom";

const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;

export const FLOW_PLAY_DEFAULT_FIXTURE: FlowFixtureV1 = FLOW_DEFAULT_FIXTURE;
export const FLOW_PLAY_DEFAULT_FIXTURE_JSON = flowFixtureToJson(FLOW_PLAY_DEFAULT_FIXTURE);

export const FLOW_PLAY_LAYOUT = createStackLayout([FLOW_PLAY_WINDOW_KIND_ID], ["Flow"]);
export const FLOW_PLAY_KINDS_BODY_KEY = "flow.play.kinds";
export const FLOW_PLAY_KINDS_TAB_ID = "flow-play-kinds";
export const FLOW_PLAY_EXTENSIONS_TAB_ID = "flow-play-extensions";
export const FLOW_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const FLOW_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const FLOW_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

/** @emoji 📚 Neuron module section ids expected in the flow play workbench catalogue. */
export const FLOW_NEURON_MODULE_IDS = ["dictionary", "list", "logic", "math", "text"] as const;

/** @emoji ✅ True when every registered neuron module section is present. */
export function flowPlayCatalogueIncludesAllNeuronModules(sections: readonly CatalogueSection[]): boolean {
  const ids = new Set(sections.map((section) => section.id));
  return FLOW_NEURON_MODULE_IDS.every((id) => ids.has(id));
}

/** @emoji ✅ True when every active neuron module section is present. */
export function flowPlayCatalogueIncludesActiveNeuronModules(sections: readonly CatalogueSection[], activeModuleIds: readonly string[]): boolean {
  const ids = new Set(sections.map((section) => section.id));
  return activeModuleIds.every((id) => ids.has(id));
}

/** @emoji 🧩 Workbench extensions tab: installed modules with enable/disable toggles. */
export function buildFlowPlayExtensionsTree(entries: readonly FlowExtensionEntry[]): UiNode {
  if (!entries.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "flow-play-extensions.empty",
          label: "Extensions",
          defaultOpen: false,
          items: [{ id: "flow-play-extensions.empty.msg", label: "Loading extensions…" }],
        },
      ],
    };
  }
  const commandItems = flowExtensionHost.activeCommands().map((command) => ({
    id: `flow-play-extensions.command.${command.id}`,
    label: command.title,
    description: command.id,
    command: flowPlayCmd("runExtensionCommand", { commandId: command.id }),
  }));
  const sections: UiTreeSectionNode[] = [
    {
      id: "flow-play-extensions.installed",
      label: "Installed",
      defaultOpen: false,
      items: entries.map((entry) => ({
        id: `flow-play-extensions.${entry.id}`,
        label: entry.manifest.name,
        description: `${entry.manifest.version} · ${entry.active ? "enabled" : "disabled"} · ${entry.manifest.contributes.operators.length} operators · ${entry.manifest.contributes.schemas.length} schemas · ${entry.manifest.contributes.commands.length} commands`,
        command: flowPlayCmd("toggleExtension", { id: entry.id, enabled: !entry.active }),
      })),
    },
  ];
  if (commandItems.length) {
    sections.push({
      id: "flow-play-extensions.commands",
      label: "Commands",
      defaultOpen: false,
      items: commandItems,
    });
  }
  return { type: "tree", sections };
}

function flowPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: FLOW_PLAY_CONTROLLER_ID, command, args };
}

function flowPlayPanelCmd(controllerId: string, command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId, command, args };
}

function buildFlowLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: FlowLayoutOrientation): string {
  return JSON.stringify({ layerSpacing, siblingGap, orientation });
}

/** @emoji 🖱️ Flow play canvas right-click menu. */
export function buildFlowPlayCanvasContextMenu(ctx: FlowCanvasContextMenuContext, dispatch: FlowContextMenuDispatch): ContextMenuItem[] {
  return [...buildFlowContextMenuItems(ctx, dispatch)];
}

/** @emoji 🏷️ Workbench catalogue tab: module sections plus Inputs and Outputs. */
export function buildFlowPlayKindsTree(sections: readonly CatalogueSection[]): UiNode {
  if (!sections.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "flow-play-kinds.empty",
          label: "Catalogue",
          defaultOpen: false,
          items: [{ id: "flow-play-kinds.empty.msg", label: "Loading catalogue…" }],
        },
      ],
    };
  }
  const treeSections: UiTreeSectionNode[] = buildCatalogueKindsTreeSections(sections, "flow-play-kinds", flowPlayCatalogueItemDragData);
  return { type: "tree", sections: treeSections };
}

// #region 🔖FlowPlayPanels
/** @emoji 🧩 Parses flow play fixture JSON. */
export function parseFlowPlayFixtureJson(json: string): FlowFixtureV1 | null {
  try {
    const parsed = JSON.parse(json) as FlowFixtureV1;
    if (parsed.schema !== "flow.fixture/v1" || !Array.isArray(parsed.widgets) || !Array.isArray(parsed.synapses)) {
      return null;
    }
    return parsed;
  } catch {
    return null;
  }
}

function flowPlayWidgetTreeLabel(widget: FlowWidgetV1): string {
  switch (widget.kind) {
    case "neuron":
      return widget.neuronKind;
    case "inputSlider":
      return `Slider (${widget.value})`;
    case "inputStepper":
      return `Stepper (${widget.schema})`;
    case "inputNote":
      return widget.text?.trim() || widget.id;
    case "inputImage":
      return widget.id;
    case "variable":
      return widget.name || widget.id;
    case "outputPreview":
      return "Preview";
    case "outputAction":
      return widget.action;
    case "cluster":
      return widget.name || widget.id;
    default:
      return widget.id;
  }
}

/** @emoji 🌳 Workbench hierarchy: widgets and synapses from the live fixture. */
export function buildFlowPlayHierarchyTree(
  fixtureJson: string,
  selectedNodeIds: readonly string[],
  controllerId: string = FLOW_PLAY_CONTROLLER_ID,
): UiNode {
  const fixture = parseFlowPlayFixtureJson(fixtureJson);
  if (!fixture) {
    return {
      type: "tree",
      sections: [
        {
          id: "flow-play-hierarchy.invalid",
          label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
          defaultOpen: true,
          items: [{ id: "flow-play-hierarchy.invalid.msg", label: "Invalid flow fixture" }],
        },
      ],
    };
  }
  const widgetItems: UiTreeItemNode[] = fixture.widgets.map((widget) => ({
    id: `flow-play-hierarchy.widget.${widget.id}`,
    label: flowPlayWidgetTreeLabel(widget),
    description: widget.kind,
    command: flowPlayPanelCmd(controllerId, "setSelection", { ids: [widget.id] }),
  }));
  const synapseItems: UiTreeItemNode[] = fixture.synapses.map((synapse) => ({
    id: `flow-play-hierarchy.synapse.${synapse.id}`,
    label: `${synapse.from} → ${synapse.to}`,
    description: [synapse.fromPort, synapse.toPort].filter(Boolean).join(" → ") || synapse.id,
  }));
  return {
    type: "tree",
    sections: [
      {
        id: "flow-play-hierarchy.widgets",
        label: "Widgets",
        defaultOpen: true,
        items: widgetItems.length ? widgetItems : [{ id: "flow-play-hierarchy.widgets.empty", label: "(none)" }],
      },
      {
        id: "flow-play-hierarchy.synapses",
        label: "Synapses",
        defaultOpen: false,
        items: synapseItems.length ? synapseItems : [{ id: "flow-play-hierarchy.synapses.empty", label: "(none)" }],
      },
    ],
    selectedIds: selectedNodeIds.map((id) => `flow-play-hierarchy.widget.${id}`),
  };
}

/** @emoji 📚 Workbench catalogue: neuron kinds plus extensions in one tab. */
export function buildFlowPlayCatalogueTree(sections: readonly CatalogueSection[], extensionEntries: readonly FlowExtensionEntry[]): UiNode {
  const kindsSections = buildFlowPlayKindsTree(sections).sections ?? [];
  const extensionSections = buildFlowPlayExtensionsTree(extensionEntries).sections ?? [];
  const merged = [...kindsSections, ...extensionSections];
  if (!merged.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "flow-play-catalogue.empty",
          label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
          defaultOpen: false,
          items: [{ id: "flow-play-catalogue.empty.msg", label: "Loading catalogue…" }],
        },
      ],
    };
  }
  return { type: "tree", sections: merged };
}

function flowPlayInspectorWidgetFields(widget: FlowWidgetV1, controllerId: string): UiNode[] {
  const widgetId = widget.id;
  const fields: UiNode[] = [
    {
      type: "field",
      id: "flow-play-inspector.id",
      label: "Id",
      child: {
        type: "input",
        id: "flow-play-inspector.id.input",
        inputKind: "text",
        value: widgetId,
        commit: "blur",
        onChange: flowPlayPanelCmd(controllerId, "renameFlowWidget", { oldId: widgetId }),
      },
    },
    {
      type: "field",
      id: "flow-play-inspector.kind",
      label: "Kind",
      child: { type: "text", value: widget.kind },
    },
  ];
  if (widget.kind === "neuron") {
    fields.push({
      type: "field",
      id: "flow-play-inspector.neuron-kind",
      label: "Neuron kind",
      child: {
        type: "input",
        id: "flow-play-inspector.neuron-kind.input",
        inputKind: "text",
        value: widget.neuronKind,
        onChange: flowPlayPanelCmd(controllerId, "patchFlowWidget", { widgetId, field: "neuronKind" }),
      },
    });
  }
  if (widget.kind === "inputSlider") {
    fields.push(
      {
        type: "field",
        id: "flow-play-inspector.slider-value",
        label: "Value",
        child: {
          type: "input",
          id: "flow-play-inspector.slider-value.input",
          inputKind: "number",
          value: String(widget.value),
          onChange: flowPlayPanelCmd(controllerId, "patchFlowWidget", { widgetId, field: "value" }),
        },
      },
      {
        type: "field",
        id: "flow-play-inspector.slider-min",
        label: "Min",
        child: {
          type: "input",
          id: "flow-play-inspector.slider-min.input",
          inputKind: "number",
          value: String(widget.min ?? 0),
          onChange: flowPlayPanelCmd(controllerId, "patchFlowWidget", { widgetId, field: "min" }),
        },
      },
      {
        type: "field",
        id: "flow-play-inspector.slider-max",
        label: "Max",
        child: {
          type: "input",
          id: "flow-play-inspector.slider-max.input",
          inputKind: "number",
          value: String(widget.max ?? 10),
          onChange: flowPlayPanelCmd(controllerId, "patchFlowWidget", { widgetId, field: "max" }),
        },
      },
    );
  }
  if (widget.kind === "inputNote") {
    fields.push({
      type: "field",
      id: "flow-play-inspector.note-text",
      label: "Text",
      child: {
        type: "input",
        id: "flow-play-inspector.note-text.input",
        inputKind: "text",
        value: widget.text,
        onChange: flowPlayPanelCmd(controllerId, "patchFlowWidget", { widgetId, field: "text" }),
      },
    });
  }
  if (widget.kind === "variable") {
    fields.push(
      {
        type: "field",
        id: "flow-play-inspector.variable-name",
        label: "Name",
        child: {
          type: "input",
          id: "flow-play-inspector.variable-name.input",
          inputKind: "text",
          value: widget.name,
          onChange: flowPlayPanelCmd(controllerId, "patchFlowWidget", { widgetId, field: "name" }),
        },
      },
      {
        type: "field",
        id: "flow-play-inspector.variable-schema",
        label: "Schema",
        child: {
          type: "input",
          id: "flow-play-inspector.variable-schema.input",
          inputKind: "text",
          value: widget.schema,
          onChange: flowPlayPanelCmd(controllerId, "patchFlowWidget", { widgetId, field: "schema" }),
        },
      },
    );
  }
  return fields;
}

/** @emoji 🔍 Details inspection: editable fields for the selected widget. */
export function buildFlowPlayInspectorTree(
  fixtureJson: string,
  selectedNodeIds: readonly string[],
  controllerId: string = FLOW_PLAY_CONTROLLER_ID,
): UiNode {
  const fixture = parseFlowPlayFixtureJson(fixtureJson);
  if (!fixture) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "flow-play-inspector.invalid", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Invalid flow fixture" }] },
    ]);
  }
  if (!selectedNodeIds.length) {
    return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "flow-play-inspector.empty",
        label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
        children: [{ type: "text", value: "Select a widget in the canvas or hierarchy." }],
      },
    ]);
  }
  if (selectedNodeIds.length > 1) {
    return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "flow-play-inspector.multi",
        label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
        children: [{ type: "text", value: `${selectedNodeIds.length} widgets selected` }],
      },
    ]);
  }
  const widget = fixture.widgets.find((entry) => entry.id === selectedNodeIds[0]);
  if (!widget) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "flow-play-inspector.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Widget not found" }] },
    ]);
  }
  return uiDeclarativeSectionsToTree([
    {
      type: "section",
      id: "flow-play-inspector.widget",
      label: flowPlayWidgetTreeLabel(widget),
      children: flowPlayInspectorWidgetFields(widget, controllerId),
    },
  ] as readonly UiSectionNode[]);
}
// #endregion 🔖FlowPlayPanels

/** @emoji 🎛 Flow play shell controller. */
export class FlowPlayController extends Controller {
  readonly mainMode = new ModeRuntime("main", "Edit", undefined);
  readonly generateMode = new ModeRuntime("generate", "Generate", undefined);
  private fixtureJson = FLOW_PLAY_DEFAULT_FIXTURE_JSON;
  private previewText = "—";
  private generatePreviewText = "—";
  private generations: FlowGeneration[] = [{ id: createFormId("generation"), name: "Generation 1", values: {} }];
  private selectedGenerationId: string | null = null;
  private evalClient: FlowOrchestratorClient | null = null;
  private catalogueSections: CatalogueSection[] = [];
  private catalogueRevision = 0;
  private readonly snapshotListeners = new Set<() => void>();
  private engagementInput = "";
  private layerSpacing = DEFAULT_LAYER_SPACING;
  private siblingGap = DEFAULT_SIBLING_GAP;
  private orientation: FlowLayoutOrientation = "leftRight";
  private reorganizeEpoch = 0;
  private reorganizeOptionsJson = buildFlowLayoutOptionsJson(DEFAULT_LAYER_SPACING, DEFAULT_SIBLING_GAP, "leftRight");
  private commandRequestEpoch = 0;
  private commandRequestPayload: Omit<FlowCanvasCommandRequest, "epoch"> = { command: "" };
  private extensionRevision = 0;
  private selectedNodeIds: string[] = [];
  private interactionRevision = 0;
  private lodMode: DagLodModeKind = DAG_LOD_MODE_AUTOMATIC;
  private lodModeByInstance: Record<string, DagLodModeKind> = {};
  private effectiveLod: DagDrawLodKind = "normal";
  private proximityDistance = FLOW_DEFAULT_PROXIMITY_DISTANCE;

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(FLOW_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.selectedGenerationId = this.generations[0]?.id ?? null;
    this.rebuildShellMode();
    this.rebuildGenerateMode();
  }

  private getEvalClient(): FlowOrchestratorClient {
    if (!this.evalClient) this.evalClient = new FlowOrchestratorClient();
    return this.evalClient;
  }

  getFixtureJson(): string {
    return this.fixtureJson;
  }

  getPreviewText(): string {
    return this.previewText;
  }

  getGenerations(): readonly FlowGeneration[] {
    return this.generations;
  }

  getSelectedGenerationId(): string | null {
    return this.selectedGenerationId;
  }

  getGeneratePreviewText(): string {
    return this.generatePreviewText;
  }

  getGenerateFormSpecJson(): string {
    return JSON.stringify(flowFixtureToFormSpec(this.fixtureJson));
  }

  getCatalogueSections(): readonly CatalogueSection[] {
    return this.catalogueSections;
  }

  getCatalogueRevision(): number {
    return this.catalogueRevision;
  }

  getExtensionRevision(): number {
    return this.extensionRevision;
  }

  getExtensionEntries(): readonly FlowExtensionEntry[] {
    return flowExtensionHost.listEntries();
  }

  getSelectedNodeIds(): readonly string[] {
    return this.selectedNodeIds;
  }

  getInteractionRevision(): number {
    return this.interactionRevision;
  }

  private applyFixtureJson(json: string): void {
    if (!json.includes("flow.fixture/v1") || json === this.fixtureJson) return;
    this.fixtureJson = json;
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  private renameFlowWidget(oldId: string, newId: string): void {
    const trimmed = newId.trim();
    if (!trimmed || trimmed === oldId) return;
    const fixture = parseFlowPlayFixtureJson(this.fixtureJson);
    if (!fixture || fixture.widgets.some((widget) => widget.id === trimmed)) return;
    const widgets = fixture.widgets.map((widget) => (widget.id === oldId ? ({ ...widget, id: trimmed } as FlowWidgetV1) : widget));
    const synapses = fixture.synapses.map((synapse) => ({
      ...synapse,
      from: synapse.from === oldId ? trimmed : synapse.from,
      to: synapse.to === oldId ? trimmed : synapse.to,
    }));
    this.selectedNodeIds = this.selectedNodeIds.map((id) => (id === oldId ? trimmed : id));
    this.fixtureJson = flowFixtureToJson({ ...fixture, widgets, synapses });
    this.interactionRevision += 1;
    this.commandRequestPayload = { command: "setSelection", argsJson: JSON.stringify({ ids: [...this.selectedNodeIds] }) };
    this.commandRequestEpoch += 1;
    this.notifySnapshot();
    this.emit();
  }

  private patchFlowWidget(widgetId: string, field: string, value: unknown): void {
    const fixture = parseFlowPlayFixtureJson(this.fixtureJson);
    if (!fixture) return;
    const widgets = fixture.widgets.map((widget) => {
      if (widget.id !== widgetId) return widget;
      if (field === "value" || field === "min" || field === "max" || field === "step") {
        const numeric = typeof value === "number" ? value : Number(value);
        if (!Number.isFinite(numeric)) return widget;
        return { ...widget, [field]: numeric } as FlowWidgetV1;
      }
      if (typeof value !== "string") return widget;
      return { ...widget, [field]: value } as FlowWidgetV1;
    });
    this.fixtureJson = flowFixtureToJson({ ...fixture, widgets });
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.emit();
  }

  /** @emoji 🔔 Subscribes to catalogue updates for workbench kinds panel refresh. */
  subscribeSnapshot(listener: () => void): () => void {
    this.snapshotListeners.add(listener);
    return () => this.snapshotListeners.delete(listener);
  }

  private notifySnapshot(): void {
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  getReorganize(): FlowReorganizeRequest {
    return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
  }

  getCommandRequest(): FlowCanvasCommandRequest {
    return { epoch: this.commandRequestEpoch, ...this.commandRequestPayload };
  }

  lodModeForScope(scopeId: string): DagLodModeKind {
    return this.lodModeByInstance[scopeId] ?? this.lodMode;
  }

  proximityDistanceValue(): number {
    return this.proximityDistance;
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
      onChange: { controllerId: FLOW_PLAY_CONTROLLER_ID, command: "setLodMode", args: { instanceId: scopeId } },
    };
  }

  private proximityMeasure(): WindowMeasure {
    return {
      kind: "slider",
      id: "flow-proximity-distance",
      label: "Proximity",
      value: this.proximityDistance,
      min: 0,
      max: 240,
      step: 4,
      onChange: { controllerId: FLOW_PLAY_CONTROLLER_ID, command: "setProximityDistance" },
    };
  }

  private windowMeasures(): readonly WindowMeasure[] {
    return [this.lodMeasure(FLOW_PLAY_WINDOW_KIND_ID), this.proximityMeasure()];
  }

  private syncReorganizeOptionsJson(): void {
    this.reorganizeOptionsJson = buildFlowLayoutOptionsJson(this.layerSpacing, this.siblingGap, this.orientation);
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
        onChange: flowPlayCmd("engagementInput"),
        onSubmit: flowPlayCmd("engagementSubmit"),
      },
      possibleEngagements: [
        { id: FLOW_ENGAGEMENT_REORGANIZE_ID, label: "Reorganize", command: flowPlayCmd("reorganize") },
        { id: FLOW_ENGAGEMENT_ORIENTATION_LR_ID, label: "Left to Right", command: flowPlayCmd("setOrientation", { orientation: "leftRight" }) },
        { id: FLOW_ENGAGEMENT_ORIENTATION_TB_ID, label: "Top to Bottom", command: flowPlayCmd("setOrientation", { orientation: "topBottom" }) },
      ],
      controls: [
        {
          kind: "slider",
          id: "flow-layer-spacing",
          label: "Layer spacing",
          value: this.layerSpacing,
          min: 40,
          max: 320,
          step: 10,
          onChange: flowPlayCmd("setSpacing", { field: "layerSpacing" }),
        },
        {
          kind: "slider",
          id: "flow-sibling-gap",
          label: "Sibling gap",
          value: this.siblingGap,
          min: 10,
          max: 160,
          step: 5,
          onChange: flowPlayCmd("setSpacing", { field: "siblingGap" }),
        },
      ],
      status: [{ id: "flow-layout-orientation", text: this.orientation === "leftRight" ? "Left to right" : "Top to bottom" }],
    };
  }

  private rebuildShellMode(): void {
    this.mainMode.windowKinds = [
      new WindowKindRuntime(FLOW_PLAY_WINDOW_KIND_ID, "Flow", FLOW_PLAY_BODY_KEY_MAIN, undefined, this.windowMeasures(), this.windowEngagement()),
    ];
    for (const windowKind of this.mainMode.windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Flow play window "${windowKind.id}"`);
    }
  }

  private rebuildGenerateMode(): void {
    this.generateMode.windowKinds = [new WindowKindRuntime(FLOW_PLAY_WINDOW_KIND_ID, "Generate", FLOW_PLAY_BODY_KEY_GENERATE)];
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
      const orientation = (args as { orientation?: FlowLayoutOrientation }).orientation;
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
    if (command === "canvasCommand") {
      const canvasCommand = (args as { command?: string; argsJson?: string }).command;
      if (typeof canvasCommand !== "string" || !canvasCommand) return;
      const argsJson = (args as { argsJson?: string }).argsJson;
      this.commandRequestPayload = { command: canvasCommand, ...(argsJson !== undefined ? { argsJson } : {}) };
      this.commandRequestEpoch += 1;
      this.emit();
      return;
    }
    if (command === "setPreviewText") {
      const text = (args as { text?: string }).text;
      if (typeof text === "string" && text !== this.previewText) {
        this.previewText = text;
        this.emit();
      }
      return;
    }
    if (command === "setCatalogueSections") {
      const sections = (args as { sections?: CatalogueSection[] }).sections;
      if (Array.isArray(sections)) {
        this.catalogueSections = sections;
        this.catalogueRevision += 1;
        this.notifySnapshot();
        this.emit();
      }
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
      this.commandRequestPayload = { command: "setSelection", argsJson: JSON.stringify({ ids: next }) };
      this.commandRequestEpoch += 1;
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "renameFlowWidget") {
      const oldId = (args as { oldId?: string }).oldId;
      const value = (args as { value?: string }).value;
      if (typeof oldId === "string" && typeof value === "string") {
        this.renameFlowWidget(oldId, value);
      }
      return;
    }
    if (command === "patchFlowWidget") {
      const widgetId = (args as { widgetId?: string }).widgetId;
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value;
      if (typeof widgetId === "string" && typeof field === "string") {
        this.patchFlowWidget(widgetId, field, value);
      }
      return;
    }
    if (command === "setLodMode") {
      const { value, instanceId } = args as { value?: string; instanceId?: string };
      const scopeId = instanceId ?? FLOW_PLAY_WINDOW_KIND_ID;
      if (typeof value !== "string") return;
      if (value !== DAG_LOD_MODE_AUTOMATIC && !isDagDrawLodKind(value)) return;
      this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: value as DagLodModeKind };
      if (scopeId === FLOW_PLAY_WINDOW_KIND_ID) {
        this.lodMode = value as DagLodModeKind;
      }
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setEffectiveLod") {
      const { lod, instanceId } = args as { lod?: DagDrawLodKind; instanceId?: string };
      const scopeId = instanceId ?? FLOW_PLAY_WINDOW_KIND_ID;
      if (!lod || !isDagDrawLodKind(lod)) return;
      if (scopeId !== FLOW_PLAY_WINDOW_KIND_ID) return;
      if (this.effectiveLod === lod) return;
      this.effectiveLod = lod;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setProximityDistance") {
      const value = (args as { value?: number }).value;
      if (typeof value !== "number" || !Number.isFinite(value)) return;
      const next = Math.max(0, value);
      if (this.proximityDistance === next) return;
      this.proximityDistance = next;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "toggleExtension") {
      const id = (args as { id?: string }).id;
      const enabled = (args as { enabled?: boolean }).enabled;
      if (typeof id !== "string" || typeof enabled !== "boolean") return;
      void flowExtensionHost.setActive(id, enabled).then(() => {
        this.extensionRevision += 1;
        this.notifySnapshot();
        this.emit();
      });
      return;
    }
    if (command === "runExtensionCommand") {
      const commandId = (args as { commandId?: string }).commandId;
      if (typeof commandId !== "string") return;
      const result = flowExtensionHost.executeCommand(commandId);
      console.log(`[DEBUG] flow extension command ${commandId}: ${result}`);
      this.emit();
      return;
    }
    if (command === "addGeneration" || command === "removeGeneration" || command === "selectGeneration" || command === "renameGeneration" || command === "updateGenerationValues") {
      void runGenerationCommand({
        command,
        args,
        generations: this.generations,
        selectedGenerationId: this.selectedGenerationId,
        fixtureJson: this.fixtureJson,
        client: this.getEvalClient(),
      }).then((next) => {
        if (!next) return;
        this.generations = [...next.generations];
        this.selectedGenerationId = next.selectedGenerationId;
        if (next.generatePreviewText) this.generatePreviewText = next.generatePreviewText;
        this.notifySnapshot();
        this.emit();
      });
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

function buildFlowPlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildFlowWindowBody(FLOW_PLAY_SURFACE_ID, FLOW_PLAY_CONTROLLER_ID, FLOW_PLAY_WINDOW_KIND_ID);
}

function buildFlowPlayGenerateDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildFormsWindowBody(FLOW_PLAY_SURFACE_ID_GENERATE, FLOW_PLAY_CONTROLLER_ID, "generate");
}

export function registerFlowPlayDeclarativeBodies(): void {
  registerWindowBody(FLOW_PLAY_BODY_KEY_MAIN, buildFlowPlayMainDeclarativeBody);
  registerWindowBody(FLOW_PLAY_BODY_KEY_GENERATE, buildFlowPlayGenerateDeclarativeBody);
}

export function buildFlowPlayAppRuntime(controller: FlowPlayController): AppRuntime {
  const app = createPlayAppRuntime(FLOW_PLAY_APP_ID, "Flow", controller, FLOW_PLAY_LAYOUT, controller.mainMode);
  app.addMode(controller.generateMode);
  return app;
}

// #region GenerateHelpers
/** @emoji ⚡ Shared generation evaluation for flow and procedural generate modes. */
export async function evaluateGenerationFixture(
  client: FlowOrchestratorClient,
  baseFixtureJson: string,
  values: FlowGeneration["values"],
): Promise<string> {
  const json = applyGenerationValuesToFixture(baseFixtureJson, values);
  await client.loadFixtureJson(json);
  const result = await client.evaluate();
  try {
    const preview = await client.previewText();
    return preview || result.outputsJson.slice(0, 4000);
  } catch {
    return result.outputsJson.slice(0, 4000);
  }
}

export function createDefaultGenerations(): FlowGeneration[] {
  return [{ id: createFormId("generation"), name: "Generation 1", values: {} }];
}

export async function runGenerationCommand(input: {
  readonly command: string;
  readonly args?: unknown;
  readonly generations: readonly FlowGeneration[];
  readonly selectedGenerationId: string | null;
  readonly fixtureJson: string;
  readonly client: FlowOrchestratorClient;
}): Promise<{ readonly generations: FlowGeneration[]; readonly selectedGenerationId: string | null; readonly generatePreviewText: string } | null> {
  const generations = [...input.generations];
  let selectedGenerationId = input.selectedGenerationId;
  if (input.command === "addGeneration") {
    const id = createFormId("generation");
    generations.push({ id, name: `Generation ${generations.length + 1}`, values: {} });
    selectedGenerationId = id;
  } else if (input.command === "removeGeneration") {
    const id = (input.args as { id?: string }).id;
    if (typeof id !== "string") return null;
    const next = generations.filter((generation) => generation.id !== id);
    generations.splice(0, generations.length, ...next);
    if (selectedGenerationId === id) selectedGenerationId = generations[0]?.id ?? null;
  } else if (input.command === "selectGeneration") {
    const id = (input.args as { id?: string }).id;
    if (typeof id !== "string") return null;
    selectedGenerationId = id;
  } else if (input.command === "renameGeneration") {
    const id = (input.args as { id?: string }).id;
    const name = (input.args as { name?: string }).name;
    if (typeof id !== "string" || typeof name !== "string") return null;
    for (let index = 0; index < generations.length; index += 1) {
      if (generations[index]?.id === id) generations[index] = { ...generations[index]!, name };
    }
    return { generations, selectedGenerationId, generatePreviewText: "" };
  } else if (input.command === "updateGenerationValues") {
    const id = (input.args as { id?: string }).id;
    const values = (input.args as { values?: FlowGeneration["values"] }).values;
    if (typeof id !== "string" || values == null) return null;
    for (let index = 0; index < generations.length; index += 1) {
      if (generations[index]?.id === id) generations[index] = { ...generations[index]!, values };
    }
    selectedGenerationId = id;
  } else {
    return null;
  }
  const active = generations.find((generation) => generation.id === selectedGenerationId) ?? generations[0];
  const generatePreviewText = active
    ? await evaluateGenerationFixture(input.client, input.fixtureJson, active.values)
    : "—";
  console.log("[DEBUG] generation command", input.command, generatePreviewText.slice(0, 120));
  return { generations, selectedGenerationId, generatePreviewText };
}
// #endregion GenerateHelpers

export class PlaygroundFlow extends Playground {
  readonly id = FLOW_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = createProductPlaygroundPlatform(this.id);
    const ctrl = new FlowPlayController(runtime.commandBus, () => runtime.notify());
    runtime.addApp(buildFlowPlayAppRuntime(ctrl));
    return runtime;
  }

  registerBodies(): void {
    registerFlowPlayDeclarativeBodies();
  }
}

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("flow play shell", () => {
    it("default fixture is slider add preview", () => {
      expect(FLOW_PLAY_DEFAULT_FIXTURE.widgets.length).toBe(3);
      expect(FLOW_PLAY_DEFAULT_FIXTURE.synapses.length).toBe(2);
    });

    it("kinds tree marks catalogue rows draggable", () => {
      const tree = buildFlowPlayKindsTree([
        {
          id: "math",
          title: "Math",
          items: [{ kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" }],
        },
      ]);
      expect(tree.type).toBe("tree");
      const item = tree.sections?.[0]?.items?.[0];
      expect(item?.draggable).toBe(true);
      expect(item?.dragData).toBeDefined();
    });

    it("kinds tree lists every neuron module section", () => {
      const sections: CatalogueSection[] = [
        { id: "dictionary", title: "Dictionary", items: [{ kind: "neuron", neuronKind: "dictionary.get", name: "Get", summary: "Read key" }] },
        { id: "list", title: "List", items: [{ kind: "neuron", neuronKind: "list.get", name: "Get", summary: "Read consecutive indices" }] },
        { id: "logic", title: "Logic", items: [{ kind: "neuron", neuronKind: "logic.not", name: "Not", summary: "Invert" }] },
        { id: "math", title: "Math", items: [{ kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" }] },
        { id: "text", title: "Text", items: [{ kind: "neuron", neuronKind: "text.upper", name: "Upper", summary: "Uppercase" }] },
        { id: "inputs", title: "Inputs", items: [{ kind: "inputSlider", name: "Slider", summary: "Number" }, { kind: "inputStepper", name: "Stepper", summary: "Composite" }, { kind: "inputImage", name: "Image", summary: "Image" }] },
        { id: "outputs", title: "Outputs", items: [{ kind: "outputPreview", name: "Preview", summary: "Preview" }] },
      ];
      expect(flowPlayCatalogueIncludesAllNeuronModules(sections)).toBe(true);
      const tree = buildFlowPlayKindsTree(sections);
      const labels = tree.sections?.map((section) => section.label) ?? [];
      for (const moduleId of FLOW_NEURON_MODULE_IDS) {
        expect(labels.some((label) => label.toLowerCase().includes(moduleId))).toBe(true);
      }
      expect(labels.some((label) => /inputs/i.test(label))).toBe(true);
      expect(labels.some((label) => /outputs/i.test(label))).toBe(true);
    });

    it("catalogue revision bumps when sections arrive", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      expect(ctrl.getCatalogueRevision()).toBe(0);
      ctrl.run("setCatalogueSections", {
        sections: [{ id: "math", title: "Math", items: [{ kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" }] }],
      });
      expect(ctrl.getCatalogueRevision()).toBe(1);
    });

    it("extensions tree lists installed modules", () => {
      const tree = buildFlowPlayExtensionsTree([
        {
          id: "math",
          active: true,
          manifest: {
            schema: "flow.module/v1",
            id: "math",
            name: "Math",
            version: "0.1.0",
            activationEvents: ["onStartup"],
            contributes: {
              schemas: [],
              operators: [{ id: "math.add", module: "math", name: "Add", abbreviation: "Add", icon: "emoji:+", summary: "Sum", inputs: [], outputs: [] }],
              widgets: [],
              commands: [{ id: "math.showHelp", title: "Math: Show Help" }],
              settings: [],
            },
          },
        },
      ]);
      const labels = tree.sections?.flatMap((section) => section.items?.map((item) => item.label) ?? []) ?? [];
      expect(labels).toContain("Math");
      expect(tree.sections?.every((section) => (section.items?.length ?? 0) > 0)).toBe(true);
    });

    it("active catalogue reflects enabled modules only", () => {
      const allSections: CatalogueSection[] = [
        { id: "math", title: "Math", items: [{ kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" }] },
        { id: "text", title: "Text", items: [{ kind: "neuron", neuronKind: "text.upper", name: "Upper", summary: "Uppercase" }] },
      ];
      expect(flowPlayCatalogueIncludesActiveNeuronModules(allSections, ["math", "text"])).toBe(true);
      expect(flowPlayCatalogueIncludesActiveNeuronModules(allSections, ["math", "logic"])).toBe(false);
    });

    it("reorganize engagement bumps epoch", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      expect(ctrl.getReorganize().epoch).toBe(0);
      ctrl.run("reorganize");
      expect(ctrl.getReorganize().epoch).toBe(1);
      expect(ctrl.getReorganize().optionsJson).toContain("leftRight");
    });

    it("canvasCommand bumps command request epoch", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      expect(ctrl.getCommandRequest().epoch).toBe(0);
      ctrl.run("canvasCommand", { command: "selectAll" });
      expect(ctrl.getCommandRequest().epoch).toBe(1);
      expect(ctrl.getCommandRequest().command).toBe("selectAll");
    });

    it("buildFlowPlayCanvasContextMenu delegates to flow context menu", () => {
      const items = buildFlowPlayCanvasContextMenu(
        {
          hoveredNodeId: null,
          selectedNodeIds: [],
          isImageWidget: false,
          isBackground: true,
          previewOffNodeIds: [],
          screen: { x: 0, y: 0 },
          world: { x: 0, y: 0 },
          clientX: 0,
          clientY: 0,
        },
        () => {},
      );
      expect(items.some((item) => item.id === "flow.ctx.add")).toBe(true);
    });

    it("lod window measure lists automatic and tiers", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
      const lodSelect = measures.find((measure) => measure.kind === "select" && measure.label === "LOD");
      expect(lodSelect?.kind).toBe("select");
      if (lodSelect?.kind === "select") {
        expect(lodSelect.items.some((item) => item.value === DAG_LOD_MODE_AUTOMATIC)).toBe(true);
        expect(lodSelect.items.some((item) => item.value === "detail")).toBe(true);
      }
    });

    it("setEffectiveLod refreshes automatic select label", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      ctrl.run("setEffectiveLod", { lod: "detail" });
      const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
      const lodSelect = measures.find((measure) => measure.kind === "select" && measure.label === "LOD");
      const automatic = lodSelect?.kind === "select" ? lodSelect.items.find((item) => item.value === DAG_LOD_MODE_AUTOMATIC) : undefined;
      expect(automatic?.label).toContain("Detail");
    });

    it("proximity window measure defaults and updates via command", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      expect(ctrl.proximityDistanceValue()).toBe(FLOW_DEFAULT_PROXIMITY_DISTANCE);
      const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
      const proximity = measures.find((measure) => measure.kind === "slider" && measure.label === "Proximity");
      expect(proximity?.kind).toBe("slider");
      if (proximity?.kind === "slider") {
        expect(proximity.value).toBe(FLOW_DEFAULT_PROXIMITY_DISTANCE);
      }
      ctrl.run("setProximityDistance", { value: 0 });
      expect(ctrl.proximityDistanceValue()).toBe(0);
      const updated = ctrl.mainMode.windowKinds[0]?.measures?.find((measure) => measure.kind === "slider" && measure.label === "Proximity");
      expect(updated?.kind).toBe("slider");
      if (updated?.kind === "slider") {
        expect(updated.value).toBe(0);
      }
    });

    it("hierarchy tree lists widgets and synapses", () => {
      const tree = buildFlowPlayHierarchyTree(FLOW_PLAY_DEFAULT_FIXTURE_JSON, ["slider"]);
      expect(tree.sections?.some((section) => section.label === "Widgets")).toBe(true);
      expect(tree.sections?.some((section) => section.label === "Synapses")).toBe(true);
      expect(tree.selectedIds).toContain("flow-play-hierarchy.widget.slider");
    });

    it("catalogue tree merges kinds and extensions sections", () => {
      const tree = buildFlowPlayCatalogueTree(
        [{ id: "math", title: "Math", items: [{ kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" }] }],
        [],
      );
      expect((tree.sections?.length ?? 0) >= 1).toBe(true);
    });

    it("setSelection updates selected node ids and interaction revision", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      ctrl.run("setSelection", { ids: ["slider"] });
      expect(ctrl.getSelectedNodeIds()).toEqual(["slider"]);
      expect(ctrl.getInteractionRevision()).toBeGreaterThan(0);
    });

    it("inspector tree exposes slider value field for single selection", () => {
      const tree = buildFlowPlayInspectorTree(FLOW_PLAY_DEFAULT_FIXTURE_JSON, ["slider"]);
      const serialized = JSON.stringify(tree);
      expect(serialized).toContain("Value");
      expect(serialized).toContain("patchFlowWidget");
    });

    it("registers generate mode and maps fixture widgets to form spec", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      expect(ctrl.generateMode.id).toBe("generate");
      const spec = JSON.parse(ctrl.getGenerateFormSpecJson()) as { schema: string; steps: { questions: unknown[] }[] };
      expect(spec.schema).toBe("forms.form/v1");
      expect(spec.steps[0]?.questions.length).toBeGreaterThan(0);
    });

    it("adds generations without starting worker", () => {
      const bus = new CommandBus();
      const ctrl = new FlowPlayController(bus, () => {});
      const initial = ctrl.getGenerations().length;
      ctrl.run("addGeneration");
      expect(ctrl.getGenerations().length).toBe(initial + 1);
    });
  });
}
// #endregion 🧪Tests

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "flow") {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootFlowPlay } = await import("@semio-tech/framework-playground-renderer-react/flow");
    bootFlowPlay(new PlaygroundFlow());
  })();
}
// #endregion 🔖Boot
