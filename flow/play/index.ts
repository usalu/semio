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
} from "@framework/playground/core";

import { bootstrapElementsSurfaceChromeDocument } from "@ui/react";
import { FLOW_DEFAULT_FIXTURE, flowFixtureToJson, type FlowFixtureV1 } from "@flow/react";

export const FLOW_PLAY_APP_ID = "flow-play";
export const FLOW_PLAY_CONTROLLER_ID = "flow-play";
export const FLOW_PLAY_SURFACE_ID = "flow.play/v1";
export const FLOW_PLAY_BODY_KEY_MAIN = "flow.play.main";
export const FLOW_PLAY_WINDOW_KIND_ID = "flow-main";

export const FLOW_PLAY_DEFAULT_FIXTURE: FlowFixtureV1 = FLOW_DEFAULT_FIXTURE;
export const FLOW_PLAY_DEFAULT_FIXTURE_JSON = flowFixtureToJson(FLOW_PLAY_DEFAULT_FIXTURE);

export const FLOW_PLAY_LAYOUT = createStackLayout(FLOW_PLAY_WINDOW_KIND_ID, "Flow");

/** @emoji 🎛 Flow play shell controller. */
export class FlowPlayController extends Controller {
  readonly mainMode = new ModeRuntime("main", "Flow", undefined);
  private fixtureJson = FLOW_PLAY_DEFAULT_FIXTURE_JSON;
  private previewText = "—";

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

  override run(command: string, args?: unknown): void {
    if (command === "setPreviewText") {
      const text = (args as { text?: string }).text;
      if (typeof text === "string") {
        this.previewText = text;
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
