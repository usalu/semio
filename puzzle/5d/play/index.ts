// #region 🧲Header
// 💻 puzzle/5d/play/index.ts — Topology play on `@framework/playground/core`: unified topology fixture, LOD measures, relocate tools (no React).
// #endregion 🧲Header

import {
  CommandBus,
  Controller,
  Platform,
  AppRuntime,
  ModeRuntime,
  WindowKindRuntime,
  buildBoardWindowBody,
  buildScene3dWindowBody,
  createDefaultLayout,
  type ToolItem,
  type WindowBodyViewContext,
  type WindowMeasure,
  type UiNode,
  Playground,
  playgroundTreePanelRootItems,
  type UiTreeItemNode,
  type UiTreeNode,
} from "@framework/playground/core";

import { buildBoardPlayHierarchySections } from "../../2d/play/index.ts";
import { NakaginCapsuleTowerBoardJson as nakaginBoardJson } from "@puzzle/assets";
import { BOARD_LOD_MODE_AUTOMATIC, boardLodAutomaticSelectLabel, boardLodCanvasProps, isBoardDrawLodKind, parseBoardFixtureV1, type BoardDrawLodKind, type BoardFixtureV1, type BoardLodModeKind, type CameraState } from "../../2d/react/index.tsx";
import { NakaginCapsuleTowerSceneJson as nakaginSceneJson } from "@puzzle/assets";
import { buildPuzzle3dPlayHierarchyTree, PUZZLE_3D_PLAY_EMPTY_SELECTION } from "../../3d/play/index.ts";
import {
  DEFAULT_MANUAL_LOD,
  SCENE_LOD_SLIDER_MAX,
  SCENE_LOD_SLIDER_MIN,
  formatSceneLod,
  lodFromSliderValue,
  parseFixtureV1,
  sceneLodCanvasProps,
  sliderValueFromLod,
  type FixtureV1 as VolumeFixtureV1,
  type RelocateMode as VolumeRelocateMode,
} from "../../3d/react/index.tsx";
import { createTopologyStore, parseTopologyV1, projectFlat, projectVolume, topologyCompose, topologySharedKindsFromMetas, type TopologyStore, type TopologyV1 } from "../react/index.tsx";
import { NakaginCapsuleTowerTopologyJson as nakaginTopologyJson } from "@puzzle/assets";

//#region 🔖Ids
export const PUZZLE_5D_PLAY_APP_ID = "puzzle-5d-play";
export const PUZZLE_5D_PLAY_CONTROLLER_ID = "puzzle-5d-play";
export const PUZZLE_5D_PLAY_BOARD_WINDOW_ID = "puzzle-5d-2d";
export const PUZZLE_5D_PLAY_VOLUME_WINDOW_ID = "puzzle-5d-3d";
export const PUZZLE_5D_PLAY_BOARD_WINDOW_LABEL = "Puzzle 2d";
export const PUZZLE_5D_PLAY_VOLUME_WINDOW_LABEL = "Puzzle 3d";
export const PUZZLE_5D_PLAY_BOARD_BODY_KEY = "puzzle.5d.play.board";
export const PUZZLE_5D_PLAY_VOLUME_BODY_KEY = "puzzle.5d.play.volume";
export const PUZZLE_5D_PLAY_BOARD_SURFACE_ID = "puzzle.5d.play.board/v1";
export const PUZZLE_5D_PLAY_VOLUME_SURFACE_ID = "puzzle.5d.play.volume/v1";
export const PUZZLE_5D_PLAY_HIERARCHY_TAB_ID = "puzzle-5d-play-hierarchy";

const PUZZLE_5D_PLAY_LOD_TIERS_BOARD: readonly BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];
//#endregion 🔖Ids

//#region 🔖TopologyPlayHierarchy
export interface TopologyPlayHierarchySelectHandlers {
  readonly onSelectBoard: (id: string) => void;
  readonly onSelectVolumeObject: (objectId: string) => void;
  readonly onSelectVolumeVortex: (vortexFullId: string) => void;
  readonly onSelectVolumeAttraction: (attractionId: string) => void;
}

/** @emoji 🌳 Paired topology tree: manifest → Board + Volume composition subtrees. */
export function buildTopologyPlayHierarchySections(snapshot: TopologyPlaySnapshot, handlers: TopologyPlayHierarchySelectHandlers): UiTreeNode {
  const branches: UiTreeItemNode[] = [];
  if (snapshot.boardFixture) {
    const boardRoot = buildBoardPlayHierarchySections(snapshot.boardFixture, [...snapshot.boardSelected], handlers.onSelectBoard).sections[0]?.items?.[0];
    branches.push({
      id: "puzzle-5d-play-hierarchy.board",
      label: "Board",
      defaultOpen: true,
      items: boardRoot?.items ?? [{ id: "puzzle-5d-play-hierarchy.board.empty", label: "(empty)" }],
    });
  }
  if (snapshot.volumeFixture) {
    const volumeSelection = snapshot.volumeSelected ? { ...PUZZLE_3D_PLAY_EMPTY_SELECTION, objectIds: [snapshot.volumeSelected] } : PUZZLE_3D_PLAY_EMPTY_SELECTION;
    const volumeRoot = buildPuzzle3dPlayHierarchyTree(snapshot.volumeFixture, volumeSelection).sections[0]?.items?.[0];
    branches.push({
      id: "puzzle-5d-play-hierarchy.volume",
      label: "Volume",
      defaultOpen: true,
      items: volumeRoot?.items ?? [{ id: "puzzle-5d-play-hierarchy.volume.empty", label: "(empty)" }],
    });
  }
  const topologyRoot: UiTreeItemNode = {
    id: "puzzle-5d-play-hierarchy.topology",
    label: snapshot.manifestLabel ?? "Topology",
    defaultOpen: true,
    items: branches.length ? branches : [{ id: "puzzle-5d-play-hierarchy.topology.empty", label: "(no fixtures)" }],
  };
  return playgroundTreePanelRootItems("puzzle-5d-play-hierarchy.root", [topologyRoot]);
}
//#endregion 🔖TopologyPlayHierarchy

//#region 🔖Helpers
function topologyPlayLodTierMenuLabel(tier: string): string {
  return tier.charAt(0).toUpperCase() + tier.slice(1);
}

function topologyControllerFromContext(ctx: WindowBodyViewContext): TopologyPlayShellController | undefined {
  return ctx.runtime.getActiveApp()?.controller as TopologyPlayShellController | undefined;
}

function sameCamera(a: CameraState | null, b: CameraState): boolean {
  return Boolean(a && a.x === b.x && a.y === b.y && a.zoom === b.zoom);
}
//#endregion 🔖Helpers

//#region 🔖Controller
export interface TopologyPlaySnapshot {
  readonly manifestLabel: string | undefined;
  readonly boardFixture: BoardFixtureV1 | null;
  readonly volumeFixture: VolumeFixtureV1 | null;
  readonly boardSelected: ReadonlySet<string>;
  readonly boardCamera: CameraState | null;
  readonly volumeCamera: CameraState | null;
  readonly volumeSelected: string | null;
  readonly relocateMode: VolumeRelocateMode;
  readonly volumeLodTag: number;
  readonly boardLodTag: BoardDrawLodKind;
  readonly boardLodProps: ReturnType<typeof boardLodCanvasProps>;
  readonly volumeLodProps: ReturnType<typeof sceneLodCanvasProps>;
  readonly volumeAutomaticLod: boolean;
  readonly volumeDepthVariableLod: boolean;
  readonly volumeLodSlider: number;
  readonly sharedKinds: ReturnType<typeof topologySharedKindsFromMetas>;
  readonly connectBoard: number;
  readonly connectVolume: number;
  readonly proximityBoard: number;
  readonly proximityVolume: number;
}

function loadNakaginTopologyModel(): TopologyV1 {
  const model = parseTopologyV1(nakaginTopologyJson as unknown);
  if (!model) throw new Error("nakagin-capsule-tower.topology.json must use schema puzzle.5d.topology/v1");
  return model;
}

/** @emoji 🎛 Topology play shell controller shared by declarative board and volume windows. */
export class TopologyPlayShellController extends Controller {
  readonly mainMode = new ModeRuntime("main", "Topology", undefined);
  readonly topologyStore: TopologyStore = createTopologyStore(loadNakaginTopologyModel());
  private relocateMode: VolumeRelocateMode = "translate";
  private boardSelected: ReadonlySet<string> = new Set();
  private volumeSelected: string | null = null;
  private boardCamera: CameraState | null = { ...this.topologyStore.getModel().flatCamera };
  private volumeCamera: CameraState | null = { ...this.topologyStore.getModel().volumeCamera };
  private volumeLodTag = DEFAULT_MANUAL_LOD;
  private volumeAutomaticLod = true;
  private volumeDepthVariableLod = false;
  private volumeManualLod = DEFAULT_MANUAL_LOD;
  private volumeLodSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
  private boardLodTag: BoardDrawLodKind = "normal";
  private boardLodMode: BoardLodModeKind = BOARD_LOD_MODE_AUTOMATIC;
  private connectBoard = 0;
  private connectVolume = 0;
  private proximityBoard = 0;
  private proximityVolume = 0;

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(PUZZLE_5D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.topologyStore.subscribe(() => this.emit());
    this.rebuildShellMode();
  }

  private rebuildShellMode(): void {
    const relocateTools: ToolItem[] = (["translate", "rotate", "scale"] as const).map((mode, order) => ({
      id: `topology.relocate.${mode}`,
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

  private boardLodMeasure(): WindowMeasure {
    return {
      kind: "select",
      id: `${PUZZLE_5D_PLAY_BOARD_WINDOW_ID}-lod`,
      label: "LOD",
      value: this.boardLodMode,
      items: [{ id: "automatic", label: boardLodAutomaticSelectLabel(this.boardLodTag), value: BOARD_LOD_MODE_AUTOMATIC }, ...PUZZLE_5D_PLAY_LOD_TIERS_BOARD.map((tier) => ({ id: tier, label: topologyPlayLodTierMenuLabel(tier), value: tier }))],
      onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "setBoardLodMode" },
    };
  }

  private volumeLodMeasures(): readonly WindowMeasure[] {
    return [
      {
        kind: "toggle",
        id: `${PUZZLE_5D_PLAY_VOLUME_WINDOW_ID}-auto`,
        label: "LOD",
        text: "Auto zoom",
        pressed: this.volumeAutomaticLod,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "setVolumeAutoLod" },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_5D_PLAY_VOLUME_WINDOW_ID}-depth`,
        text: "Depth-variable",
        pressed: this.volumeDepthVariableLod,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "setVolumeDepthLod" },
      },
      {
        kind: "slider",
        id: `${PUZZLE_5D_PLAY_VOLUME_WINDOW_ID}-lod`,
        label: formatSceneLod(this.volumeLodTag),
        value: this.volumeLodSlider,
        min: SCENE_LOD_SLIDER_MIN,
        max: SCENE_LOD_SLIDER_MAX,
        step: 1,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "setVolumeManualLod" },
      },
    ];
  }

  getWindowKinds(): readonly WindowKindRuntime[] {
    return [
      new WindowKindRuntime(PUZZLE_5D_PLAY_BOARD_WINDOW_ID, PUZZLE_5D_PLAY_BOARD_WINDOW_LABEL, PUZZLE_5D_PLAY_BOARD_BODY_KEY, undefined, [this.boardLodMeasure()]),
      new WindowKindRuntime(PUZZLE_5D_PLAY_VOLUME_WINDOW_ID, PUZZLE_5D_PLAY_VOLUME_WINDOW_LABEL, PUZZLE_5D_PLAY_VOLUME_BODY_KEY, undefined, [...this.volumeLodMeasures()]),
    ];
  }

  override run(command: string, args?: unknown): void {
    let changed = true;
    switch (command) {
      case "setBoardLodMode": {
        const value = (args as { value?: string }).value;
        if ((value === BOARD_LOD_MODE_AUTOMATIC || (typeof value === "string" && isBoardDrawLodKind(value))) && this.boardLodMode !== value) this.boardLodMode = value as BoardLodModeKind;
        else changed = false;
        break;
      }
      case "setVolumeAutoLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean" && this.volumeAutomaticLod !== pressed) this.volumeAutomaticLod = pressed;
        else changed = false;
        break;
      }
      case "setVolumeDepthLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean" && this.volumeDepthVariableLod !== pressed) this.volumeDepthVariableLod = pressed;
        else changed = false;
        break;
      }
      case "setVolumeManualLod": {
        const value = (args as { value?: number }).value;
        if (typeof value === "number" && Number.isFinite(value)) {
          this.volumeLodSlider = value;
          this.volumeManualLod = lodFromSliderValue(value);
        } else changed = false;
        break;
      }
      case "setBoardLodTag": {
        const lod = (args as { lod: BoardDrawLodKind }).lod;
        if (this.boardLodTag !== lod) this.boardLodTag = lod;
        else changed = false;
        break;
      }
      case "setVolumeLodTag": {
        const lod = (args as { lod: number }).lod;
        if (typeof lod === "number" && Number.isFinite(lod) && lod > 0) {
          this.volumeLodTag = lod;
        }
        changed = false;
        break;
      }
      case "setBoardSelection": {
        const ids = (args as { ids: readonly string[] }).ids;
        if (ids.length !== this.boardSelected.size || ids.some((id) => !this.boardSelected.has(id))) this.boardSelected = new Set(ids);
        else changed = false;
        break;
      }
      case "setVolumeSelection": {
        const selected = (args as { objectIds: readonly string[] }).objectIds[0] ?? null;
        if (this.volumeSelected !== selected) this.volumeSelected = selected;
        else changed = false;
        break;
      }
      case "setBoardCamera": {
        const camera = (args as { camera: CameraState }).camera;
        if (!sameCamera(this.boardCamera, camera)) this.boardCamera = { ...camera };
        else changed = false;
        break;
      }
      case "setVolumeCamera": {
        const camera = (args as { camera: CameraState }).camera;
        if (!sameCamera(this.volumeCamera, camera)) this.volumeCamera = { ...camera };
        else changed = false;
        break;
      }
      case "setRelocateMode": {
        const mode = (args as { mode: VolumeRelocateMode }).mode;
        if (this.relocateMode !== mode) this.relocateMode = mode;
        else changed = false;
        break;
      }
      case "noteBoardConnect":
        this.connectBoard += 1;
        break;
      case "noteVolumeConnect":
        this.connectVolume += 1;
        break;
      case "noteBoardProximity":
        this.proximityBoard += 1;
        break;
      case "noteVolumeProximity":
        this.proximityVolume += 1;
        break;
      default:
        changed = false;
        break;
    }
    if (changed) {
      this.rebuildShellMode();
      this.emit();
    }
  }

  getSnapshot(): TopologyPlaySnapshot {
    const model = this.topologyStore.getModel();
    const boardFixture = projectFlat(model);
    const volumeFixture = projectVolume(model);
    return {
      manifestLabel: model.label,
      boardFixture,
      volumeFixture,
      boardSelected: this.boardSelected,
      boardCamera: this.boardCamera,
      volumeCamera: this.volumeCamera,
      volumeSelected: this.volumeSelected,
      relocateMode: this.relocateMode,
      volumeLodTag: this.volumeLodTag,
      boardLodTag: this.boardLodTag,
      boardLodProps: boardLodCanvasProps(this.boardLodMode),
      volumeLodProps: sceneLodCanvasProps({
        automaticLod: this.volumeAutomaticLod,
        depthVariableLod: this.volumeDepthVariableLod,
        manualLod: this.volumeManualLod,
      }),
      volumeAutomaticLod: this.volumeAutomaticLod,
      volumeDepthVariableLod: this.volumeDepthVariableLod,
      volumeLodSlider: this.volumeLodSlider,
      sharedKinds: topologySharedKindsFromMetas({ flatMeta: boardFixture.meta, volumeMeta: volumeFixture.meta }),
      connectBoard: this.connectBoard,
      connectVolume: this.connectVolume,
      proximityBoard: this.proximityBoard,
      proximityVolume: this.proximityVolume,
    };
  }
}
//#endregion 🔖Controller

//#region 🔖TopologyPlayRuntime
export function buildTopologyPlayAppRuntime(controller: TopologyPlayShellController): AppRuntime {
  const app = new AppRuntime(
    PUZZLE_5D_PLAY_APP_ID,
    "Topology play",
    undefined,
    controller,
    createDefaultLayout([PUZZLE_5D_PLAY_BOARD_WINDOW_ID, PUZZLE_5D_PLAY_VOLUME_WINDOW_ID], "row", [50, 50], [PUZZLE_5D_PLAY_BOARD_WINDOW_LABEL, PUZZLE_5D_PLAY_VOLUME_WINDOW_LABEL]) as never,
    controller.getWindowKinds(),
  );
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  app.leftTabs = [];
  app.rightTabs = [];
  return app;
}

export function buildTopologyPlayRuntime(): Platform {
  const runtime = new Platform();
  const controller = new TopologyPlayShellController(runtime.commandBus, () => runtime.notify());
  runtime.addApp(buildTopologyPlayAppRuntime(controller));
  return runtime;
}

/** @emoji 🛝 Topology play harness as a single {@link Playground} instance. */
export class Playground5d extends Playground {
  readonly id = PUZZLE_5D_PLAY_APP_ID;
  readonly initialPanelVisibility = { leftSidePanel: true, rightSidePanel: true };

  createRuntime(): Platform {
    return buildTopologyPlayRuntime();
  }

  registerBodies(): void {
    /* window bodies registered with surface hosts in {@link registerTopologyPlaySurfaceHosts} */
  }
}
//#endregion 🔖TopologyPlayRuntime

//#region 🔖DeclarativeBodies
export function buildTopologyFlatDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = topologyControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap?.boardFixture) return { type: "text", value: "Invalid board fixture" };
  return buildBoardWindowBody(PUZZLE_5D_PLAY_BOARD_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_BOARD_WINDOW_ID);
}

export function buildTopologyVolumeDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = topologyControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap?.volumeFixture) return { type: "text", value: "Invalid volume fixture" };
  return buildScene3dWindowBody(PUZZLE_5D_PLAY_VOLUME_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID);
}
//#endregion 🔖DeclarativeBodies

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("topology play hierarchy", () => {
    it("buildTopologyPlayHierarchySections includes Board and Volume branches", () => {
      const runtime = buildTopologyPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as TopologyPlayShellController;
      expect(controller).toBeTruthy();
      const tree = buildTopologyPlayHierarchySections(controller!.getSnapshot(), {
        onSelectBoard: () => {},
        onSelectVolumeObject: () => {},
        onSelectVolumeVortex: () => {},
        onSelectVolumeAttraction: () => {},
      });
      const topologyRoot = tree.sections[0]?.items?.[0];
      expect(topologyRoot?.label).toBeTruthy();
      const labels = topologyRoot?.items?.map((row) => row.label);
      expect(labels).toContain("Board");
      expect(labels).toContain("Volume");
    });
  });

  describe("topology play fixtures", () => {
    it("parses nakagin board and volume fixture", () => {
      const b = parseBoardFixtureV1(nakaginBoardJson as unknown);
      const s = parseFixtureV1(nakaginSceneJson as unknown);
      expect(b?.nodes.length).toBeGreaterThan(0);
      expect(s?.objects.length).toBeGreaterThan(0);
    });
    it("parses nakagin unified topology v1", () => {
      const model = parseTopologyV1(nakaginTopologyJson as unknown);
      expect(model?.schema).toBe("puzzle.5d.topology/v1");
      expect(model?.parts.length).toBeGreaterThan(0);
    });
    it("regenerates nakagin topology fixture when REGENERATE_NAKAGIN_TOPOLOGY=1", async () => {
      if (process.env.REGENERATE_NAKAGIN_TOPOLOGY !== "1") return;
      const board = parseBoardFixtureV1(nakaginBoardJson as unknown);
      const scene = parseFixtureV1(nakaginSceneJson as unknown);
      expect(board).toBeTruthy();
      expect(scene).toBeTruthy();
      const model = {
        ...topologyCompose(board!, scene!),
        label: "Nakagin capsule tower",
        meta: {
          description: "Unified topology source for Nakagin play; flat and volume views project from this model.",
        },
      };
      const { writeFile } = await import("node:fs/promises");
      const { join } = await import("node:path");
      const outPath = join(process.cwd(), "../assets/nakagin-capsule-tower.topology.json");
      await writeFile(outPath, `${JSON.stringify(model, null, 2)}\n`, "utf8");
      expect(model.parts.length).toBeGreaterThan(0);
    });
    it("shared kinds merge metas like the play harness", () => {
      const sk = topologySharedKindsFromMetas({
        flatMeta: undefined,
        volumeMeta: { kindCompatibility: [{ source: "u", target: "v" }] },
      });
      expect(sk.kindCompatibility?.length).toBeGreaterThan(0);
    });
    it("builds declarative board and volume canvas-only bodies", () => {
      const wb = buildTopologyPlayRuntime();
      const board = buildTopologyFlatDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_5D_PLAY_BOARD_WINDOW_ID,
        bodyKey: PUZZLE_5D_PLAY_BOARD_BODY_KEY,
        activeModeId: "main",
        generation: 0,
      });
      const scene = buildTopologyVolumeDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_5D_PLAY_VOLUME_WINDOW_ID,
        bodyKey: PUZZLE_5D_PLAY_VOLUME_BODY_KEY,
        activeModeId: "main",
        generation: 0,
      });
      expect(board).toEqual(buildBoardWindowBody(PUZZLE_5D_PLAY_BOARD_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_BOARD_WINDOW_ID));
      expect(scene).toEqual(buildScene3dWindowBody(PUZZLE_5D_PLAY_VOLUME_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID));
    });
  });
}
//#endregion 🧪Tests
