// #region 🧲Header
/** @emoji 🌊 Flow play harness on `@framework/playground/core`. */
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
  createStackLayout,
  enforcePlaygroundWindowEngagementInput,
  registerWindowBody,
  type CommandDescriptor,
  type WindowBodyViewContext,
  type WindowEngagement,
  type UiNode,
  type UiTreeItemNode,
  type UiTreeSectionNode,
} from "@framework/playground/core";

import { bootstrapElementsSurfaceChromeDocument } from "@ui/react";
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
  type CatalogueItem,
  type CatalogueSection,
  type DagDrawLodKind,
  type DagLodModeKind,
  type FlowExtensionEntry,
  type FlowFixtureV1,
  type FlowReorganizeRequest,
} from "@flow/react";
import type { WindowMeasure } from "@framework/playground/core";

export const FLOW_PLAY_APP_ID = "flow-play";
export const FLOW_PLAY_CONTROLLER_ID = "flow-play";
export const FLOW_PLAY_SURFACE_ID = "flow.play/v1";
export const FLOW_PLAY_BODY_KEY_MAIN = "flow.play.main";
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
          defaultOpen: true,
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
      defaultOpen: true,
      items: entries.map((entry) => ({
        id: `flow-play-extensions.${entry.id}`,
        label: entry.manifest.name,
        description: `${entry.manifest.version} · ${entry.active ? "enabled" : "disabled"} · ${entry.manifest.contributes.neuronKinds.length} kinds · ${entry.manifest.contributes.commands.length} commands`,
        command: flowPlayCmd("toggleExtension", { id: entry.id, enabled: !entry.active }),
      })),
    },
  ];
  if (commandItems.length) {
    sections.push({
      id: "flow-play-extensions.commands",
      label: "Commands",
      defaultOpen: true,
      items: commandItems,
    });
  }
  return { type: "tree", sections };
}

function flowPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: FLOW_PLAY_CONTROLLER_ID, command, args };
}

function buildFlowLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: FlowLayoutOrientation): string {
  return JSON.stringify({ layerSpacing, siblingGap, orientation });
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
          defaultOpen: true,
          items: [{ id: "flow-play-kinds.empty.msg", label: "Loading catalogue…" }],
        },
      ],
    };
  }
  const treeSections: UiTreeSectionNode[] = sections.map((section) => ({
    id: `flow-play-kinds.${section.id}`,
    label: section.title,
    defaultOpen: true,
    items: section.items.map((item, index) => flowPlayKindsTreeItem(section.id, index, item)),
  }));
  return { type: "tree", sections: treeSections };
}

function flowPlayKindsTreeItem(sectionId: string, index: number, item: CatalogueItem): UiTreeItemNode {
  return {
    id: `flow-play-kinds.${sectionId}.${index}.${item.neuronKind ?? item.kind}`,
    label: item.name,
    description: item.summary,
    draggable: true,
    dragData: flowPlayCatalogueItemDragData(item),
  };
}

/** @emoji 🎛 Flow play shell controller. */
export class FlowPlayController extends Controller {
  readonly mainMode = new ModeRuntime("main", "Flow", undefined);
  private fixtureJson = FLOW_PLAY_DEFAULT_FIXTURE_JSON;
  private previewText = "—";
  private catalogueSections: CatalogueSection[] = [];
  private catalogueRevision = 0;
  private readonly snapshotListeners = new Set<() => void>();
  private engagementInput = "";
  private layerSpacing = DEFAULT_LAYER_SPACING;
  private siblingGap = DEFAULT_SIBLING_GAP;
  private orientation: FlowLayoutOrientation = "leftRight";
  private reorganizeEpoch = 0;
  private reorganizeOptionsJson = buildFlowLayoutOptionsJson(DEFAULT_LAYER_SPACING, DEFAULT_SIBLING_GAP, "leftRight");
  private extensionRevision = 0;
  private lodMode: DagLodModeKind = DAG_LOD_MODE_AUTOMATIC;
  private lodModeByInstance: Record<string, DagLodModeKind> = {};
  private effectiveLod: DagDrawLodKind = "normal";

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(FLOW_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.rebuildShellMode();
  }

  getFixtureJson(): string {
    return this.fixtureJson;
  }

  getPreviewText(): string {
    return this.previewText;
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

  lodModeForScope(scopeId: string): DagLodModeKind {
    return this.lodModeByInstance[scopeId] ?? this.lodMode;
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

  private windowMeasures(): readonly WindowMeasure[] {
    return [this.lodMeasure(FLOW_PLAY_WINDOW_KIND_ID)];
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
      if (typeof json === "string" && json !== this.fixtureJson) {
        this.fixtureJson = json;
        this.emit();
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

export function registerFlowPlayDeclarativeBodies(): void {
  registerWindowBody(FLOW_PLAY_BODY_KEY_MAIN, buildFlowPlayMainDeclarativeBody);
}

export function buildFlowPlayAppRuntime(controller: FlowPlayController): AppRuntime {
  const app = new AppRuntime(FLOW_PLAY_APP_ID, "Flow", undefined, controller, FLOW_PLAY_LAYOUT, []);
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  return app;
}

export class PlaygroundFlow extends Playground {
  readonly id = FLOW_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = new Platform({ id: this.id });
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
        { id: "list", title: "List", items: [{ kind: "neuron", neuronKind: "list.get", name: "Get", summary: "Read index" }] },
        { id: "logic", title: "Logic", items: [{ kind: "neuron", neuronKind: "logic.not", name: "Not", summary: "Invert" }] },
        { id: "math", title: "Math", items: [{ kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" }] },
        { id: "text", title: "Text", items: [{ kind: "neuron", neuronKind: "text.upper", name: "Upper", summary: "Uppercase" }] },
        { id: "inputs", title: "Inputs", items: [{ kind: "inputSlider", name: "Slider", summary: "Number" }] },
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
              neuronKinds: [{ id: "math.add", module: "math", name: "Add", summary: "Sum", inputs: ["a"], outputs: ["number"] }],
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
  });
}
// #endregion 🧪Tests

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "flow") {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootFlowPlay } = await import("@framework/playground/renderer/react/flow");
    bootFlowPlay(new PlaygroundFlow());
  })();
}
// #endregion 🔖Boot
