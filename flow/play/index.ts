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
  registerWindowBody,
  type WindowBodyViewContext,
  type UiNode,
  type UiTreeItemNode,
  type UiTreeSectionNode,
} from "@framework/playground/core";

import { bootstrapElementsSurfaceChromeDocument } from "@ui/react";
import {
  FLOW_DEFAULT_FIXTURE,
  flowFixtureToJson,
  flowPlayCatalogueItemDragData,
  type CatalogueItem,
  type CatalogueSection,
  type FlowFixtureV1,
} from "@flow/react";

export const FLOW_PLAY_APP_ID = "flow-play";
export const FLOW_PLAY_CONTROLLER_ID = "flow-play";
export const FLOW_PLAY_SURFACE_ID = "flow.play/v1";
export const FLOW_PLAY_BODY_KEY_MAIN = "flow.play.main";
export const FLOW_PLAY_WINDOW_KIND_ID = "flow-main";

export const FLOW_PLAY_DEFAULT_FIXTURE: FlowFixtureV1 = FLOW_DEFAULT_FIXTURE;
export const FLOW_PLAY_DEFAULT_FIXTURE_JSON = flowFixtureToJson(FLOW_PLAY_DEFAULT_FIXTURE);

export const FLOW_PLAY_LAYOUT = createStackLayout([FLOW_PLAY_WINDOW_KIND_ID], ["Flow"]);
export const FLOW_PLAY_KINDS_BODY_KEY = "flow.play.kinds";
export const FLOW_PLAY_KINDS_TAB_ID = "flow-play-kinds";

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

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(FLOW_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.mainMode.windowKinds = [new WindowKindRuntime(FLOW_PLAY_WINDOW_KIND_ID, "Flow", FLOW_PLAY_BODY_KEY_MAIN)];
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

  override run(command: string, args?: unknown): void {
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
    }
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
