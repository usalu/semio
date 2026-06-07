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
  registerWindowBody,
  type WindowBodyViewContext,
  type UiNode,
} from "@framework/playground/core";

import { bootstrapElementsSurfaceChromeDocument } from "@ui/react";
import { DAG_DEFAULT_FIXTURE, dagFixtureToJson, type DagFixtureV1 } from "@dag/react";

export const DAG_PLAY_APP_ID = "dag-play";
export const DAG_PLAY_CONTROLLER_ID = "dag-play";
export const DAG_PLAY_SURFACE_ID = "dag.play/v1";
export const DAG_PLAY_BODY_KEY_MAIN = "dag.play.main";
export const DAG_PLAY_WINDOW_KIND_ID = "dag-main";

export const DAG_PLAY_DEFAULT_FIXTURE: DagFixtureV1 = DAG_DEFAULT_FIXTURE;
export const DAG_PLAY_DEFAULT_FIXTURE_JSON = dagFixtureToJson(DAG_PLAY_DEFAULT_FIXTURE);

export const DAG_PLAY_LAYOUT = createStackLayout([DAG_PLAY_WINDOW_KIND_ID], ["DAG"]);

/** @emoji 🎛 DAG play shell controller. */
export class DagPlayController extends Controller {
  readonly mainMode = new ModeRuntime("main", "DAG", undefined);
  private fixtureJson = DAG_PLAY_DEFAULT_FIXTURE_JSON;

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(DAG_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.mainMode.windowKinds = [new WindowKindRuntime(DAG_PLAY_WINDOW_KIND_ID, "DAG", DAG_PLAY_BODY_KEY_MAIN)];
  }

  getFixtureJson(): string {
    return this.fixtureJson;
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
    it("default fixture has three nodes", () => {
      expect(DAG_PLAY_DEFAULT_FIXTURE.nodes.length).toBe(3);
      expect(DAG_PLAY_DEFAULT_FIXTURE.edges.length).toBe(2);
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
