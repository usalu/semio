// #region 🧲Header
/** @emoji ♻️ Trinity rewrite play harness on `@semio-tech/framework-playground-core`. */
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
  type CommandDescriptor,
  type UiNode,
  type WindowBodyViewContext,
  type WindowEngagement,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
  TRINITY_DEFAULT_FIXTURE_JSON,
  applyRewriteOnFixture,
  buildTrinityPlayCatalogueTree,
  buildTrinityPlayHierarchyTree,
  buildTrinityPlayInspectorTree,
  parseTrinityFixtureJson,
} from "@semio-tech/trinity-react";

export const TRINITY_REWRITE_PLAY_APP_ID = "trinity-rewrite-play";
export const TRINITY_REWRITE_PLAY_CONTROLLER_ID = "trinity-rewrite-play";
export const TRINITY_REWRITE_PLAY_SURFACE_ID = "trinity.rewrite.play/v1";
export const TRINITY_REWRITE_PLAY_BODY_KEY_MAIN = "trinity.rewrite.play.main";
export const TRINITY_REWRITE_PLAY_WINDOW_KIND_ID = "trinity-rewrite-main";
export const TRINITY_REWRITE_PLAY_DEFAULT_RULE_JSON = JSON.stringify({
  name: "label-core",
  lhs: { pattern: { leftVar: "a", leftKind: "Piece" }, whereClause: "a.name = 'b'" },
  rhs: { create: [], delete: [], set: [{ var: "a", prop: "label", value: "nakagin-core" }], merge: [] },
});

function trinityRewriteCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: TRINITY_REWRITE_PLAY_CONTROLLER_ID, command, args };
}

export function buildTrinityRewritePlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildTrinityWindowBody(TRINITY_REWRITE_PLAY_SURFACE_ID, TRINITY_REWRITE_PLAY_CONTROLLER_ID, TRINITY_REWRITE_PLAY_WINDOW_KIND_ID);
}

export function registerTrinityRewritePlayDeclarativeBodies(): void {
  registerWindowBody(TRINITY_REWRITE_PLAY_BODY_KEY_MAIN, buildTrinityRewritePlayMainDeclarativeBody);
}

export class TrinityRewritePlayController extends Controller {
  readonly mainMode = new ModeRuntime("explore", "Explore", undefined);
  private fixtureJson = TRINITY_DEFAULT_FIXTURE_JSON;
  private ruleJson = TRINITY_REWRITE_PLAY_DEFAULT_RULE_JSON;
  private selectedNodeIds: string[] = [];
  private reorganizeEpoch = 0;
  private interactionRevision = 0;

  constructor(commandBus: CommandBus, notify: () => void) {
    super(TRINITY_REWRITE_PLAY_CONTROLLER_ID, commandBus, notify);
    this.rebuildShellMode();
  }

  getFixtureJson(): string {
    return this.fixtureJson;
  }

  getRuleJson(): string {
    return this.ruleJson;
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
        id: "trinity-rewrite-rule",
        value: this.ruleJson,
        placeholder: "Rewrite rule JSON",
        onChange: trinityRewriteCmd("setRuleJson"),
        onSubmit: trinityRewriteCmd("applyRule"),
      },
      options: [{ id: "trinity-rewrite.apply", label: "Apply Rule", command: trinityRewriteCmd("applyRule") }],
    };
    this.mainMode.windowKinds = [
      new WindowKindRuntime(TRINITY_REWRITE_PLAY_WINDOW_KIND_ID, "Rewrite Graph", TRINITY_REWRITE_PLAY_BODY_KEY_MAIN, undefined, [], engagement),
    ];
    for (const windowKind of this.mainMode.windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Trinity rewrite play window "${windowKind.id}"`);
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
    if (command === "setRuleJson") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string") {
        this.ruleJson = value;
        this.bump();
      }
      return;
    }
    if (command === "applyRule") {
      const value = (args as { value?: string }).value;
      const rule = typeof value === "string" && value.trim() ? value : this.ruleJson;
      this.ruleJson = rule;
      console.log(`[DEBUG] trinity rewrite apply: ${rule}`);
      try {
        this.fixtureJson = applyRewriteOnFixture(this.fixtureJson, rule);
        console.log("[DEBUG] trinity rewrite applied");
      } catch (err) {
        console.log(`[DEBUG] trinity rewrite failed: ${String(err)}`);
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
    }
  }
}

function buildTrinityRewritePlayAppRuntime(ctrl: TrinityRewritePlayController): AppRuntime {
  const layout = createStackLayout([TRINITY_REWRITE_PLAY_WINDOW_KIND_ID], ["Rewrite Graph"]);
  return createPlayAppRuntime(TRINITY_REWRITE_PLAY_APP_ID, "semio · trinity · rewrite", ctrl, layout, ctrl.mainMode);
}

export class PlaygroundTrinityRewrite extends Playground {
  readonly id = TRINITY_REWRITE_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = createProductPlaygroundPlatform(this.id);
    const ctrl = new TrinityRewritePlayController(runtime.commandBus, () => runtime.notify());
    runtime.addApp(buildTrinityRewritePlayAppRuntime(ctrl));
    return runtime;
  }

  registerBodies(): void {
    registerTrinityRewritePlayDeclarativeBodies();
  }
}

export { buildTrinityPlayHierarchyTree, buildTrinityPlayCatalogueTree, buildTrinityPlayInspectorTree };

if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "trinity-rewrite"
) {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootTrinityRewritePlay } = await import("@semio-tech/framework-playground-renderer-react/trinity-rewrite");
    bootTrinityRewritePlay(new PlaygroundTrinityRewrite());
  })();
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("buildTrinityRewritePlayMainDeclarativeBody", () => {
    it("returns a trinity host surface", () => {
      const node = buildTrinityRewritePlayMainDeclarativeBody({
        runtime: new Platform({ id: "test" }),
        windowKindId: TRINITY_REWRITE_PLAY_WINDOW_KIND_ID,
        bodyKey: TRINITY_REWRITE_PLAY_BODY_KEY_MAIN,
        activeModeId: "explore",
        generation: 0,
      });
      expect(node.type).toBe("trinity");
    });
  });

  describe("TrinityRewritePlayController", () => {
    it("applyRule updates fixture via wasm", () => {
      const bus = new CommandBus();
      const ctrl = new TrinityRewritePlayController(bus, () => {});
      const before = ctrl.getFixtureJson();
      ctrl.run("applyRule", { value: TRINITY_REWRITE_PLAY_DEFAULT_RULE_JSON });
      expect(ctrl.getFixtureJson()).not.toBe(before);
    });
  });
}
