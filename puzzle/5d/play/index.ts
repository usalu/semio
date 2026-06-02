// #region 🧲Header
// 💻 puzzle/5d/play/index.ts — Puzzle 5d play on `@framework/playground/core`: unified 5d fixture, LOD measures, relocate tools (no React).
// #endregion 🧲Header

import {
  CommandBus,
  Controller,
  Store,
  Platform,
  AppRuntime,
  ModeRuntime,
  WindowKindRuntime,
  buildPuzzle2dWindowBody,
  buildPuzzle3dWindowBody,
  createDefaultLayout,
  type ToolItem,
  type WindowBodyViewContext,
  type CommandDescriptor,
  type WindowEngagement,
  type WindowMeasure,
  type UiNode,
  Playground,
  playgroundTreePanelRootItems,
  type UiTreeItemNode,
  type UiTreeNode,
  enforcePlaygroundWindowEngagementInput,
} from "@framework/playground/core";

import { buildPuzzle2dPlayHierarchySections } from "../../2d/play/index.ts";
import nakagin2dJson from "../../2d/fixture/nakagin-capsule-tower.2d.json";
import { PUZZLE_2D_LOD_MODE_AUTOMATIC, puzzle2dLodAutomaticSelectLabel, puzzle2dLodCanvasProps, isPuzzle2dDrawLodKind, parsePuzzle2dFixtureV1, type Puzzle2dDrawLodKind, type Puzzle2dFixtureV1, type Puzzle2dLodModeKind, type CameraState } from "../../2d/react/index.tsx";
import nakagin3dJson from "../../3d/fixture/nakagin-capsule-tower.3d.json";
import { buildPuzzle3dPlayHierarchyTree, PUZZLE_3D_PLAY_EMPTY_SELECTION } from "../../3d/play/index.ts";
import {
  DEFAULT_MANUAL_LOD,
  PUZZLE_3D_LOD_SLIDER_MAX,
  PUZZLE_3D_LOD_SLIDER_MIN,
  formatLod,
  lodFromSliderValue,
  parseFixtureV1,
  puzzle3dLodCanvasProps,
  sliderValueFromLod,
  type FixtureV1 as Puzzle3dFixtureV1,
  type RelocateMode as Puzzle3dRelocateMode,
} from "../../3d/react/index.tsx";
import { createStore, parseV1, project2d, project3d, compose5d, sharedKindsFromMetas, type Store as Puzzle5dStore, type StoreSnapshot as Puzzle5dStoreSnapshot, type V1 as Puzzle5dV1 } from "../react/index.tsx";
import nakagin5dJson from "../fixture/nakagin-capsule-tower.5d.json";

//#region 🔖Ids
export const PUZZLE_5D_PLAY_APP_ID = "puzzle-5d-play";
export const PUZZLE_5D_PLAY_CONTROLLER_ID = "puzzle-5d-play";
export const PUZZLE_5D_PLAY_2D_WINDOW_ID = "puzzle-5d-2d";
export const PUZZLE_5D_PLAY_3D_WINDOW_ID = "puzzle-5d-3d";
export const PUZZLE_5D_PLAY_2D_WINDOW_LABEL = "Puzzle 2d";
export const PUZZLE_5D_PLAY_3D_WINDOW_LABEL = "Puzzle 3d";
export const PUZZLE_5D_PLAY_2D_BODY_KEY = "puzzle.5d.play.2d";
export const PUZZLE_5D_PLAY_3D_BODY_KEY = "puzzle.5d.play.3d";
export const PUZZLE_5D_PLAY_2D_SURFACE_ID = "puzzle.5d.play.2d/v1";
export const PUZZLE_5D_PLAY_3D_SURFACE_ID = "puzzle.5d.play.3d/v1";
export const PUZZLE_5D_PLAY_HIERARCHY_TAB_ID = "puzzle-5d-play-hierarchy";

const PUZZLE_5D_PLAY_LOD_TIERS_2D: readonly Puzzle2dDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

function puzzle5dPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command, args: args as never };
}
//#endregion 🔖Ids

//#region 🔖Puzzle5dPlayHierarchy
export interface Puzzle5dPlayHierarchySelectHandlers {
  readonly onSelect2d: (id: string) => void;
  readonly onSelect3dObject: (objectId: string) => void;
  readonly onSelect3dVortex: (vortexFullId: string) => void;
  readonly onSelect3dAttraction: (attractionId: string) => void;
}

/** @emoji 🌳 Puzzle 5d hierarchy: manifest → 2d + 3d composition subtrees. */
export function buildPuzzle5dPlayHierarchySections(snapshot: Puzzle5dPlaySnapshot, handlers: Puzzle5dPlayHierarchySelectHandlers): UiTreeNode {
  const branches: UiTreeItemNode[] = [];
  if (snapshot.fixture2d) {
    const root2d = buildPuzzle2dPlayHierarchySections(snapshot.fixture2d, [...snapshot.selected2d], handlers.onSelect2d).sections[0]?.items?.[0];
    branches.push({
      id: "puzzle-5d-play-hierarchy.2d",
      label: "2d",
      defaultOpen: true,
      items: root2d?.items ?? [{ id: "puzzle-5d-play-hierarchy.2d.empty", label: "(empty)" }],
    });
  }
  if (snapshot.fixture3d) {
    const selection3d = snapshot.selected3d ? { ...PUZZLE_3D_PLAY_EMPTY_SELECTION, objectIds: [snapshot.selected3d] } : PUZZLE_3D_PLAY_EMPTY_SELECTION;
    const root3d = buildPuzzle3dPlayHierarchyTree(snapshot.fixture3d, selection3d).sections[0]?.items?.[0];
    branches.push({
      id: "puzzle-5d-play-hierarchy.3d",
      label: "3d",
      defaultOpen: true,
      items: root3d?.items ?? [{ id: "puzzle-5d-play-hierarchy.3d.empty", label: "(empty)" }],
    });
  }
  const root5d: UiTreeItemNode = {
    id: "puzzle-5d-play-hierarchy.5d",
    label: snapshot.manifestLabel ?? "5d",
    defaultOpen: true,
    items: branches.length ? branches : [{ id: "puzzle-5d-play-hierarchy.5d.empty", label: "(no fixtures)" }],
  };
  return playgroundTreePanelRootItems("puzzle-5d-play-hierarchy.root", [root5d]);
}
//#endregion 🔖Puzzle5dPlayHierarchy

//#region 🔖Helpers
function puzzle5dPlayLodTierMenuLabel(tier: string): string {
  return tier.charAt(0).toUpperCase() + tier.slice(1);
}

function puzzle5dControllerFromContext(ctx: WindowBodyViewContext): Puzzle5dPlayShellController | undefined {
  return ctx.runtime.getActiveApp()?.controller as Puzzle5dPlayShellController | undefined;
}

function sameCamera(a: CameraState | null, b: CameraState): boolean {
  return Boolean(a && a.x === b.x && a.y === b.y && a.zoom === b.zoom);
}
//#endregion 🔖Helpers

//#region 🔖Controller
export interface Puzzle5dPlaySnapshot {
  readonly manifestLabel: string | undefined;
  readonly fixture2d: Puzzle2dFixtureV1 | null;
  readonly fixture3d: Puzzle3dFixtureV1 | null;
  readonly selected2d: ReadonlySet<string>;
  readonly camera2d: CameraState | null;
  readonly camera3d: CameraState | null;
  readonly selected3d: string | null;
  readonly relocateMode: Puzzle3dRelocateMode;
  readonly lod3dTag: number;
  readonly lod2dTag: Puzzle2dDrawLodKind;
  readonly lod2dProps: ReturnType<typeof puzzle2dLodCanvasProps>;
  readonly lod3dProps: ReturnType<typeof puzzle3dLodCanvasProps>;
  readonly automaticLod3d: boolean;
  readonly depthVariableLod3d: boolean;
  readonly lod3dSlider: number;
  readonly sharedKinds: ReturnType<typeof sharedKindsFromMetas>;
  readonly connect2d: number;
  readonly connect3d: number;
  readonly proximity2d: number;
  readonly proximity3d: number;
}

function loadNakagin5dModel(): Puzzle5dV1 {
  const model = parseV1(nakagin5dJson as unknown);
  if (!model) throw new Error("nakagin-capsule-tower.5d.json must use schema puzzle.5d/v1");
  return model;
}

export const PUZZLE_5D_PLAY_STORE_ID = "puzzle-5d";

/** @emoji 🔗 Adapts {@link Puzzle5dStore} to {@link Store} for controller-owned registration. */
export class Puzzle5dStoreBridge extends Store<Puzzle5dStoreSnapshot> {
  private detach?: () => void;

  constructor(readonly inner: Puzzle5dStore) {
    super();
    this.detach = inner.subscribe(() => this.notify());
  }

  override getSnapshot(): Puzzle5dStoreSnapshot {
    return this.inner.getSnapshot();
  }

  override dispose(): void {
    this.detach?.();
    super.dispose();
  }
}

/** @emoji 🎛 Puzzle 5d play shell controller shared by declarative 2d and 3d windows. */
export class Puzzle5dPlayShellController extends Controller {
  readonly mainMode = new ModeRuntime("main", "Puzzle 5d", undefined);
  readonly puzzle5dStore: Puzzle5dStore = createStore(loadNakagin5dModel());
  readonly puzzle5dStoreBridge: Puzzle5dStoreBridge;
  private relocateMode: Puzzle3dRelocateMode = "translate";
  private selected2d: ReadonlySet<string> = new Set();
  private selected3d: string | null = null;
  private camera2d: CameraState | null = { ...this.puzzle5dStore.read().camera2d };
  private camera3d: CameraState | null = { ...this.puzzle5dStore.read().camera3d };
  private lod3dTag = DEFAULT_MANUAL_LOD;
  private automaticLod3d = true;
  private depthVariableLod3d = false;
  private manualLod3d = DEFAULT_MANUAL_LOD;
  private lod3dSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
  private lod2dTag: Puzzle2dDrawLodKind = "normal";
  private lod2dMode: Puzzle2dLodModeKind = PUZZLE_2D_LOD_MODE_AUTOMATIC;
  private connect2d = 0;
  private connect3d = 0;
  private proximity2d = 0;
  private proximity3d = 0;
  private engagementInputByWindow: Record<string, string> = {
    [PUZZLE_5D_PLAY_2D_WINDOW_ID]: "",
    [PUZZLE_5D_PLAY_3D_WINDOW_ID]: "",
  };

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(PUZZLE_5D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.puzzle5dStoreBridge = new Puzzle5dStoreBridge(this.puzzle5dStore);
    this.provideStore(PUZZLE_5D_PLAY_STORE_ID, this.puzzle5dStoreBridge);
    this.puzzle5dStore.subscribe(() => this.emit());
    this.rebuildShellMode();
  }

  private rebuildShellMode(): void {
    const relocateTools: ToolItem[] = (["translate", "rotate", "scale"] as const).map((mode, order) => ({
      id: `puzzle5d.relocate.${mode}`,
      kind: "toggle" as const,
      text: mode.charAt(0).toUpperCase() + mode.slice(1),
      order,
      pressed: this.relocateMode === mode,
      controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID,
      command: "setRelocateMode",
      args: { mode },
    }));
    this.mainMode.tools = { actions: relocateTools };
    this.mainMode.windowKinds = this.getWindowKinds();
  }

  private lod2dMeasure(): WindowMeasure {
    return {
      kind: "select",
      id: `${PUZZLE_5D_PLAY_2D_WINDOW_ID}-lod`,
      value: this.lod2dMode,
      items: [{ id: "automatic", label: puzzle2dLodAutomaticSelectLabel(this.lod2dTag), value: PUZZLE_2D_LOD_MODE_AUTOMATIC }, ...PUZZLE_5D_PLAY_LOD_TIERS_2D.map((tier) => ({ id: tier, label: puzzle5dPlayLodTierMenuLabel(tier), value: tier }))],
      onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set2dLodMode" },
    };
  }

  private lod3dMeasures(): readonly WindowMeasure[] {
    return [
      {
        kind: "toggle",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-auto`,
        text: "Auto zoom",
        pressed: this.automaticLod3d,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dAutoLod" },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-depth`,
        text: "Depth-variable",
        pressed: this.depthVariableLod3d,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dDepthLod" },
      },
      {
        kind: "slider",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-lod`,
        label: formatLod(this.lod3dTag),
        value: this.lod3dSlider,
        min: PUZZLE_3D_LOD_SLIDER_MIN,
        max: PUZZLE_3D_LOD_SLIDER_MAX,
        step: 1,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dManualLod" },
      },
    ];
  }

  private windowEngagementFor(windowId: string): WindowEngagement {
    return {
      input: {
        id: "engagement-input",
        value: this.engagementInputByWindow[windowId] ?? "",
        placeholder: "Command",
        onChange: puzzle5dPlayCmd("engagementInput", { windowId }),
        onSubmit: puzzle5dPlayCmd("engagementSubmit", { windowId }),
        onAbort: puzzle5dPlayCmd("engagementAbort", { windowId }),
      },
    };
  }

  getWindowKinds(): readonly WindowKindRuntime[] {
    const windowKinds = [
      new WindowKindRuntime(PUZZLE_5D_PLAY_2D_WINDOW_ID, PUZZLE_5D_PLAY_2D_WINDOW_LABEL, PUZZLE_5D_PLAY_2D_BODY_KEY, undefined, [{ kind: "group", id: `${PUZZLE_5D_PLAY_2D_WINDOW_ID}-lod`, label: "LOD", children: [this.lod2dMeasure()] }], this.windowEngagementFor(PUZZLE_5D_PLAY_2D_WINDOW_ID)),
      new WindowKindRuntime(PUZZLE_5D_PLAY_3D_WINDOW_ID, PUZZLE_5D_PLAY_3D_WINDOW_LABEL, PUZZLE_5D_PLAY_3D_BODY_KEY, undefined, [{ kind: "group", id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-lod`, label: "LOD", children: this.lod3dMeasures() }], this.windowEngagementFor(PUZZLE_5D_PLAY_3D_WINDOW_ID)),
    ];
    for (const windowKind of windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Puzzle 5D play window "${windowKind.id}"`);
    }
    return windowKinds;
  }

  override run(command: string, args?: unknown): void {
    let changed = true;
    switch (command) {
      case "set2dLodMode": {
        const value = (args as { value?: string }).value;
        if ((value === PUZZLE_2D_LOD_MODE_AUTOMATIC || (typeof value === "string" && isPuzzle2dDrawLodKind(value))) && this.lod2dMode !== value) this.lod2dMode = value as Puzzle2dLodModeKind;
        else changed = false;
        break;
      }
      case "set3dAutoLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean" && this.automaticLod3d !== pressed) this.automaticLod3d = pressed;
        else changed = false;
        break;
      }
      case "set3dDepthLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean" && this.depthVariableLod3d !== pressed) this.depthVariableLod3d = pressed;
        else changed = false;
        break;
      }
      case "set3dManualLod": {
        const value = (args as { value?: number }).value;
        if (typeof value === "number" && Number.isFinite(value)) {
          this.lod3dSlider = value;
          this.manualLod3d = lodFromSliderValue(value);
        } else changed = false;
        break;
      }
      case "set2dLodTag": {
        const lod = (args as { lod: Puzzle2dDrawLodKind }).lod;
        if (this.lod2dTag !== lod) this.lod2dTag = lod;
        else changed = false;
        break;
      }
      case "set3dLodTag": {
        const lod = (args as { lod: number }).lod;
        if (typeof lod === "number" && Number.isFinite(lod) && lod > 0) {
          this.lod3dTag = lod;
        }
        changed = false;
        break;
      }
      case "set2dSelection": {
        const ids = (args as { ids: readonly string[] }).ids;
        if (ids.length !== this.selected2d.size || ids.some((id) => !this.selected2d.has(id))) this.selected2d = new Set(ids);
        else changed = false;
        break;
      }
      case "set3dSelection": {
        const selected = (args as { objectIds: readonly string[] }).objectIds[0] ?? null;
        if (this.selected3d !== selected) this.selected3d = selected;
        else changed = false;
        break;
      }
      case "set2dCamera": {
        const camera = (args as { camera: CameraState }).camera;
        if (!sameCamera(this.camera2d, camera)) this.camera2d = { ...camera };
        else changed = false;
        break;
      }
      case "set3dCamera": {
        const camera = (args as { camera: CameraState }).camera;
        if (!sameCamera(this.camera3d, camera)) this.camera3d = { ...camera };
        else changed = false;
        break;
      }
      case "setRelocateMode": {
        const mode = (args as { mode: Puzzle3dRelocateMode }).mode;
        if (this.relocateMode !== mode) this.relocateMode = mode;
        else changed = false;
        break;
      }
      case "note2dConnect":
        this.connect2d += 1;
        break;
      case "note3dConnect":
        this.connect3d += 1;
        break;
      case "note2dProximity":
        this.proximity2d += 1;
        break;
      case "note3dProximity":
        this.proximity3d += 1;
        break;
      case "engagementInput": {
        const { windowId, value } = args as { windowId?: string; value?: string };
        if (!windowId || !(windowId in this.engagementInputByWindow)) {
          changed = false;
          break;
        }
        this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: String(value ?? "") };
        break;
      }
      case "engagementSubmit":
      case "engagementAbort": {
        const { windowId } = args as { windowId?: string };
        if (!windowId || !(windowId in this.engagementInputByWindow)) {
          changed = false;
          break;
        }
        this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: "" };
        break;
      }
      default:
        changed = false;
        break;
    }
    if (changed) {
      this.rebuildShellMode();
      this.emit();
    }
  }

  getSnapshot(): Puzzle5dPlaySnapshot {
    const model = this.puzzle5dStore.read();
    const fixture2d = project2d(model);
    const fixture3d = project3d(model);
    return {
      manifestLabel: model.label,
      fixture2d,
      fixture3d,
      selected2d: this.selected2d,
      camera2d: this.camera2d,
      camera3d: this.camera3d,
      selected3d: this.selected3d,
      relocateMode: this.relocateMode,
      lod3dTag: this.lod3dTag,
      lod2dTag: this.lod2dTag,
      lod2dProps: puzzle2dLodCanvasProps(this.lod2dMode),
      lod3dProps: puzzle3dLodCanvasProps({
        automaticLod: this.automaticLod3d,
        depthVariableLod: this.depthVariableLod3d,
        manualLod: this.manualLod3d,
      }),
      automaticLod3d: this.automaticLod3d,
      depthVariableLod3d: this.depthVariableLod3d,
      lod3dSlider: this.lod3dSlider,
      sharedKinds: sharedKindsFromMetas({ meta2d: fixture2d.meta, meta3d: fixture3d.meta }),
      connect2d: this.connect2d,
      connect3d: this.connect3d,
      proximity2d: this.proximity2d,
      proximity3d: this.proximity3d,
    };
  }
}
//#endregion 🔖Controller

//#region 🔖Puzzle5dPlayRuntime
export function buildPuzzle5dPlayAppRuntime(controller: Puzzle5dPlayShellController): AppRuntime {
  const app = new AppRuntime(
    PUZZLE_5D_PLAY_APP_ID,
    "Puzzle 5d play",
    undefined,
    controller,
    createDefaultLayout([PUZZLE_5D_PLAY_2D_WINDOW_ID, PUZZLE_5D_PLAY_3D_WINDOW_ID], "row", [50, 50], [PUZZLE_5D_PLAY_2D_WINDOW_LABEL, PUZZLE_5D_PLAY_3D_WINDOW_LABEL]) as never,
    controller.getWindowKinds(),
  );
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  app.panelTabs = [];
  return app;
}

export function buildPuzzle5dPlayRuntime(initialPanelVisibility?: { leftSidePanel: boolean; rightSidePanel: boolean }): Platform {
  const runtime = new Platform({ initialPanelVisibility });
  const controller = new Puzzle5dPlayShellController(runtime.commandBus, () => runtime.notify());
  runtime.addApp(buildPuzzle5dPlayAppRuntime(controller));
  return runtime;
}

/** @emoji 🛝 Puzzle 5d play harness as a single {@link Playground} instance. */
export class Playground5d extends Playground {
  readonly id = PUZZLE_5D_PLAY_APP_ID;
  readonly initialPanelVisibility = { leftSidePanel: true, rightSidePanel: true };

  createRuntime(): Platform {
    return buildPuzzle5dPlayRuntime(this.initialPanelVisibility);
  }

  registerBodies(): void {
    /* window bodies registered with surface hosts in {@link registerPuzzle5dPlaySurfaceHosts} */
  }
}
//#endregion 🔖Puzzle5dPlayRuntime

//#region 🔖DeclarativeBodies
export function buildPuzzle5d2dDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = puzzle5dControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap?.fixture2d) return { type: "text", value: "Invalid 2d fixture" };
  return buildPuzzle2dWindowBody(PUZZLE_5D_PLAY_2D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_2D_WINDOW_ID);
}

export function buildPuzzle5d3dDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = puzzle5dControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap?.fixture3d) return { type: "text", value: "Invalid 3d fixture" };
  return buildPuzzle3dWindowBody(PUZZLE_5D_PLAY_3D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID);
}
//#endregion 🔖DeclarativeBodies

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("puzzle 5d play hierarchy", () => {
    it("buildPuzzle5dPlayHierarchySections includes 2d and 3d branches", () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      expect(controller).toBeTruthy();
      const tree = buildPuzzle5dPlayHierarchySections(controller!.getSnapshot(), {
        onSelect2d: () => {},
        onSelect3dObject: () => {},
        onSelect3dVortex: () => {},
        onSelect3dAttraction: () => {},
      });
      const root5d = tree.sections[0]?.items?.[0];
      expect(root5d?.label).toBeTruthy();
      const labels = root5d?.items?.map((row) => row.label);
      expect(labels).toContain("2d");
      expect(labels).toContain("3d");
    });
  });

  describe("puzzle 5d play fixtures", () => {
    it("parses nakagin 2d and 3d fixtures", () => {
      const fixture2d = parsePuzzle2dFixtureV1(nakagin2dJson as unknown);
      const fixture3d = parseFixtureV1(nakagin3dJson as unknown);
      expect(fixture2d?.nodes.length).toBeGreaterThan(0);
      expect(fixture3d?.objects.length).toBeGreaterThan(0);
    });
    it("parses nakagin unified puzzle 5d v1", () => {
      const model = parseV1(nakagin5dJson as unknown);
      expect(model?.schema).toBe("puzzle.5d/v1");
      expect(model?.parts.length).toBeGreaterThan(0);
    });
    it("regenerates nakagin 5d fixture when REGENERATE_NAKAGIN_5D=1", async () => {
      if (process.env.REGENERATE_NAKAGIN_5D !== "1") return;
      const fixture2d = parsePuzzle2dFixtureV1(nakagin2dJson as unknown);
      const fixture3d = parseFixtureV1(nakagin3dJson as unknown);
      expect(fixture2d).toBeTruthy();
      expect(fixture3d).toBeTruthy();
      const model = {
        ...compose5d(fixture2d!, fixture3d!),
        label: "Nakagin capsule tower",
        meta: {
          description: "Unified puzzle 5d source for Nakagin play; 2d and 3d views project from this model.",
        },
      };
      const { writeFile } = await import("node:fs/promises");
      const { join } = await import("node:path");
      const outPath = join(process.cwd(), "../fixture/nakagin-capsule-tower.5d.json");
      await writeFile(outPath, `${JSON.stringify(model, null, 2)}\n`, "utf8");
      expect(model.parts.length).toBeGreaterThan(0);
    });
    it("shared kinds merge metas like the play harness", () => {
      const sk = sharedKindsFromMetas({
        meta2d: undefined,
        meta3d: { kindCompatibility: [{ source: "u", target: "v" }] },
      });
      expect(sk.kindCompatibility?.length).toBeGreaterThan(0);
    });
    it("builds declarative 2d and 3d canvas-only bodies", () => {
      const wb = buildPuzzle5dPlayRuntime();
      const body2d = buildPuzzle5d2dDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_5D_PLAY_2D_WINDOW_ID,
        bodyKey: PUZZLE_5D_PLAY_2D_BODY_KEY,
        activeModeId: "main",
        generation: 0,
      });
      const body3d = buildPuzzle5d3dDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_5D_PLAY_3D_WINDOW_ID,
        bodyKey: PUZZLE_5D_PLAY_3D_BODY_KEY,
        activeModeId: "main",
        generation: 0,
      });
      expect(body2d).toEqual(buildPuzzle2dWindowBody(PUZZLE_5D_PLAY_2D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_2D_WINDOW_ID));
      expect(body3d).toEqual(buildPuzzle3dWindowBody(PUZZLE_5D_PLAY_3D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID));
    });
  });
}
//#endregion 🧪Tests

//#region 🔖Boot
if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "5d"
) {
  void (async () => {
    await import("./globals.css");
    const { boot5dPlay } = await import("@framework/playground/renderer/react/puzzle/5d");
    boot5dPlay(new Playground5d());
  })();
}
//#endregion 🔖Boot
