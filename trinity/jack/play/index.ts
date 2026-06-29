// #region 🧲Header
/** @emoji 🃏 Trinity jack play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
  AppRuntime,
  CommandBus,
  Controller,
  ModeRuntime,
  Platform,
  Playground,
  WindowKindRuntime,
  buildTrinityWindowBody,
  createPlayAppRuntime,
  createProductPlaygroundPlatform,
  createStackLayout,
  enforcePlaygroundWindowEngagementInput,
  registerWindowBody,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  type CommandDescriptor,
  type UiNode,
  type WindowBodyViewContext,
  type WindowEngagement,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
  TRINITY_DEFAULT_FIXTURE_JSON,
  buildTrinityPlayCatalogueTree,
  buildTrinityPlayHierarchyTree,
  buildTrinityPlayInspectorTree,
  parseTrinityFixtureJson,
  runJackOnFixture,
  trinityFixtureToJson,
  type TrinityFixtureV1,
} from "@semio-tech/trinity-react";

export const TRINITY_JACK_PLAY_APP_ID = "trinity-jack-play";
export const TRINITY_JACK_PLAY_CONTROLLER_ID = "trinity-jack-play";
export const TRINITY_JACK_PLAY_SURFACE_ID = "trinity.jack.play/v1";
export const TRINITY_JACK_PLAY_BODY_KEY_MAIN = "trinity.jack.play.main";
export const TRINITY_JACK_PLAY_WINDOW_KIND_ID = "trinity-jack-main";
export const TRINITY_JACK_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const TRINITY_JACK_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const TRINITY_JACK_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON = TRINITY_DEFAULT_FIXTURE_JSON;

function trinityJackCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID, command, args };
}

export function buildTrinityJackPlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildTrinityWindowBody(TRINITY_JACK_PLAY_SURFACE_ID, TRINITY_JACK_PLAY_CONTROLLER_ID, TRINITY_JACK_PLAY_WINDOW_KIND_ID);
}

export function registerTrinityJackPlayDeclarativeBodies(): void {
  registerWindowBody(TRINITY_JACK_PLAY_BODY_KEY_MAIN, buildTrinityJackPlayMainDeclarativeBody);
}

export class TrinityJackPlayController extends Controller {
  readonly mainMode = new ModeRuntime("explore", "Explore", undefined);
  private fixtureJson = TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON;
  private jackQuery = "MATCH (a:Piece) RETURN a.name";
  private jackResultJson = "";
  private selectedNodeIds: string[] = [];
  private reorganizeEpoch = 0;
  private interactionRevision = 0;

  constructor(commandBus: CommandBus, notify: () => void) {
    super(TRINITY_JACK_PLAY_CONTROLLER_ID, commandBus, notify);
    this.rebuildShellMode();
  }

  getFixtureJson(): string {
    return this.fixtureJson;
  }

  getJackQuery(): string {
    return this.jackQuery;
  }

  getJackResultJson(): string {
    return this.jackResultJson;
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

  private bump(): void {
    this.interactionRevision += 1;
    this.rebuildShellMode();
    this.emit();
  }

  private rebuildShellMode(): void {
    const engagement: WindowEngagement = {
      input: {
        id: "trinity-jack-query",
        value: this.jackQuery,
        placeholder: "MATCH (a:Piece) RETURN a.name",
        onChange: trinityJackCmd("setJackQuery"),
        onSubmit: trinityJackCmd("runJackQuery"),
      },
      options: [
        { id: "trinity-jack.reorganize", label: "Reorganize", command: trinityJackCmd("reorganize") },
        { id: "trinity-jack.run", label: "Run Jack", command: trinityJackCmd("runJackQuery") },
      ],
    };
    this.mainMode.windowKinds = [
      new WindowKindRuntime(TRINITY_JACK_PLAY_WINDOW_KIND_ID, "Nakagin Graph", TRINITY_JACK_PLAY_BODY_KEY_MAIN, undefined, [], engagement),
    ];
    for (const windowKind of this.mainMode.windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Trinity jack play window "${windowKind.id}"`);
    }
  }

  run(command: string, args?: unknown): void {
    if (command === "setFixtureJson") {
      const json = (args as { json?: string }).json;
      if (typeof json === "string" && parseTrinityFixtureJson(json)) {
        this.fixtureJson = json;
        this.bump();
      }
      return;
    }
    if (command === "setJackQuery") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string") {
        this.jackQuery = value;
        this.bump();
      }
      return;
    }
    if (command === "runJackQuery") {
      const value = (args as { value?: string }).value;
      const query = typeof value === "string" && value.trim() ? value : this.jackQuery;
      this.jackQuery = query;
      console.log(`[DEBUG] trinity jack query: ${query}`);
      try {
        const result = runJackOnFixture(this.fixtureJson, query);
        this.jackResultJson = JSON.stringify(result);
        console.log(`[DEBUG] trinity jack result rows=${result.rows.length}`);
      } catch (err) {
        this.jackResultJson = JSON.stringify({ columns: ["error"], rows: [[String(err)]] });
        console.log(`[DEBUG] trinity jack query failed: ${String(err)}`);
      }
      this.bump();
      return;
    }
    if (command === "setSelection") {
      const ids = (args as { ids?: string[] }).ids ?? [];
      this.selectedNodeIds = [...ids];
      this.bump();
      return;
    }
    if (command === "reorganize") {
      this.reorganizeEpoch += 1;
      this.bump();
      return;
    }
  }
}

function buildTrinityJackPlayAppRuntime(ctrl: TrinityJackPlayController): AppRuntime {
  const layout = createStackLayout([TRINITY_JACK_PLAY_WINDOW_KIND_ID], ["Nakagin Graph"]);
  return createPlayAppRuntime(TRINITY_JACK_PLAY_APP_ID, "semio · trinity · jack", ctrl, layout, ctrl.mainMode);
}

export class PlaygroundTrinityJack extends Playground {
  readonly id = TRINITY_JACK_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = createProductPlaygroundPlatform(this.id);
    const ctrl = new TrinityJackPlayController(runtime.commandBus, () => runtime.notify());
    runtime.addApp(buildTrinityJackPlayAppRuntime(ctrl));
    return runtime;
  }

  registerBodies(): void {
    registerTrinityJackPlayDeclarativeBodies();
  }
}

export { buildTrinityPlayHierarchyTree, buildTrinityPlayCatalogueTree, buildTrinityPlayInspectorTree, parseTrinityFixtureJson, trinityFixtureToJson };
export type { TrinityFixtureV1 };

if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "trinity-jack"
) {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootTrinityJackPlay } = await import("@semio-tech/framework-playground-renderer-react/trinity-jack");
    bootTrinityJackPlay(new PlaygroundTrinityJack());
  })();
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("buildTrinityJackPlayMainDeclarativeBody", () => {
    it("returns a trinity host surface", () => {
      const node = buildTrinityJackPlayMainDeclarativeBody({
        runtime: new Platform({ id: "test" }),
        windowKindId: TRINITY_JACK_PLAY_WINDOW_KIND_ID,
        bodyKey: TRINITY_JACK_PLAY_BODY_KEY_MAIN,
        activeModeId: "explore",
        generation: 0,
      });
      expect(node).toEqual({
        type: "trinity",
        componentKind: "trinity",
        surfaceId: TRINITY_JACK_PLAY_SURFACE_ID,
        controllerId: TRINITY_JACK_PLAY_CONTROLLER_ID,
        paneId: TRINITY_JACK_PLAY_WINDOW_KIND_ID,
      });
    });
  });

  describe("TrinityJackPlayController", () => {
    it("runJackQuery returns nakagin core row", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityJackPlayController(bus, () => {});
      ctrl.run("runJackQuery", { value: "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name" });
      const result = JSON.parse(ctrl.getJackResultJson()) as { rows: unknown[][] };
      expect(result.rows.length).toBe(1);
    });
  });
}
