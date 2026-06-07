// #region 🧲Header
/** @emoji 🌳 DAG play harness on `@framework/playground/core`. */
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
  createStackLayout,
  enforcePlaygroundWindowEngagementInput,
  registerWindowBody,
  type CommandDescriptor,
  type WindowBodyViewContext,
  type WindowEngagement,
  type UiNode,
} from "@framework/playground/core";

import { bootstrapElementsSurfaceChromeDocument } from "@ui/react";
import { DAG_DEFAULT_FIXTURE, dagFixtureToJson, type DagFixtureV1, type DagReorganizeRequest } from "@dag/react";

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

export const DAG_PLAY_LAYOUT = createStackLayout([DAG_PLAY_WINDOW_KIND_ID], ["DAG"]);

function dagPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: DAG_PLAY_CONTROLLER_ID, command, args };
}

function buildDagLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: DagLayoutOrientation): string {
  return JSON.stringify({ layerSpacing, siblingGap, orientation });
}

/** @emoji 🎛 DAG play shell controller. */
export class DagPlayController extends Controller {
  readonly mainMode = new ModeRuntime("main", "DAG", undefined);
  private fixtureJson = DAG_PLAY_DEFAULT_FIXTURE_JSON;
  private engagementInput = "";
  private layerSpacing = DEFAULT_LAYER_SPACING;
  private siblingGap = DEFAULT_SIBLING_GAP;
  private orientation: DagLayoutOrientation = "leftRight";
  private reorganizeEpoch = 0;
  private reorganizeOptionsJson = buildDagLayoutOptionsJson(DEFAULT_LAYER_SPACING, DEFAULT_SIBLING_GAP, "leftRight");

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(DAG_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.rebuildShellMode();
  }

  getFixtureJson(): string {
    return this.fixtureJson;
  }

  getReorganize(): DagReorganizeRequest {
    return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
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
      sessionActive: true,
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
      new WindowKindRuntime(DAG_PLAY_WINDOW_KIND_ID, "DAG", DAG_PLAY_BODY_KEY_MAIN, undefined, [], this.windowEngagement()),
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
      if (typeof json === "string" && json !== this.fixtureJson) {
        this.fixtureJson = json;
        this.emit();
      }
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
  const app = new AppRuntime(DAG_PLAY_APP_ID, "DAG", undefined, controller, DAG_PLAY_LAYOUT, []);
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  return app;
}

export class PlaygroundDag extends Playground {
  readonly id = DAG_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = new Platform({ id: this.id });
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
  });
}
// #endregion 🧪Tests

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "dag") {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootDagPlay } = await import("@framework/playground/renderer/react/dag");
    bootDagPlay(new PlaygroundDag());
  })();
}
// #endregion 🔖Boot
