// #region 🧲Header
// 💻 elements/client/lib/system/renderer/react/scene/play/index.ts — Scene play harness: Nakagin fixture metadata, LOD controls, and localStorage keys (no React; mount via main.ts + scene-play-host).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	ProductRuntime,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildScene3dWindowBody,
	createStackLayout,
	type WindowBodyViewContext,
	Expertise,
	type AppTools,
	type ToolItem,
	type WindowMeasure,
	type UiNode,
} from "@elements/framework";

import nakaginSceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";
import {
	DEFAULT_MANUAL_LOD,
	SCENE_LOD_SLIDER_MAX,
	SCENE_LOD_SLIDER_MIN,
	formatSceneLod,
	lodFromSliderValue,
	parseFixtureV1,
	sceneLodCanvasProps,
	sliderValueFromLod,
	type FixtureV1,
	type KindCatalogBundle,
	type KindCompatEntry,
	type RelocateMode,
} from "../index.tsx";

//#region 🧾Meta
function parseKindCompatibility(meta: Record<string, unknown> | undefined): readonly KindCompatEntry[] {
	if (!meta || typeof meta !== "object") return [];
	const arr = (meta as { kindCompatibility?: unknown }).kindCompatibility;
	if (!Array.isArray(arr)) return [];
	const out: KindCompatEntry[] = [];
	for (const entry of arr) {
		if (!entry || typeof entry !== "object") continue;
		const e = entry as Record<string, unknown>;
		const source = typeof e.source === "string" ? e.source.trim() : "";
		const target = typeof e.target === "string" ? e.target.trim() : "";
		if (!source || !target) continue;
		const specificity =
			e.specificity === "general" ||
			e.specificity === "node" ||
			e.specificity === "edge" ||
			e.specificity === "handle" ||
			e.specificity === "wire" ||
			e.specificity === "object" ||
			e.specificity === "attraction"
				? e.specificity
				: undefined;
		out.push({
			source,
			target,
			...(e.bidirectional === true ? { bidirectional: true } : {}),
			...(e.important === true ? { important: true } : {}),
			...(specificity ? { specificity } : {}),
		});
	}
	return out;
}

function parseKindCatalogs(meta: Record<string, unknown> | undefined): KindCatalogBundle | undefined {
	const kc = meta?.kindCatalogs;
	if (!kc || typeof kc !== "object") return undefined;
	return kc as KindCatalogBundle;
}
//#endregion 🧾Meta

//#region 🖥️Surface
export const LS_THEME = "elements.board-play.surface.theme";
export const LS_DEVICE = "elements.board-play.surface.device";
export const LS_EXPERTISE = "elements.board-play.surface.expertise";

export function parseStoredTheme(raw: string | null) {
	if (raw === "light" || raw === "dark" || raw === "system") return raw;
	return "system";
}

export function parseStoredDevice(raw: string | null) {
	if (raw === "desktop" || raw === "tablet" || raw === "mobile") return raw;
	return "desktop";
}

export function parseStoredExpertise(raw: string | null) {
	if (raw === Expertise.BEGINNER || raw === Expertise.NORMAL || raw === Expertise.EXPERT) return raw;
	return Expertise.NORMAL;
}
//#endregion 🖥️Surface

//#region 🎬Play
export const PLAY_APP_ID = "elements-scene-play";
export const SCENE_PLAY_WINDOW_ID = "scene-main";
export const SCENE_PLAY_WINDOW_LABEL = "Scene";
export const SCENE_PLAY_BODY_KEY = "elements.scene.play.window";
export const SCENE_PLAY_CONTROLLER_ID = "scene-play";
export const SCENE_PLAY_SCENE_SURFACE_ID = "elements.scene.play.scene/v1";
//#endregion 🎬Play

export { parseKindCatalogs, parseKindCompatibility };

//#region 🔖ScenePlayController
/** @emoji 🎬 Framework-free scene play controller: fixture, LOD, selection, and interaction counters. */
export class ScenePlayShellController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Scene", undefined);
	readonly fixture: FixtureV1 | null;
	private automaticLod: boolean;
	private depthVariableLod: boolean;
	private manualLod: number;
	private lodSlider: number;
	private lodTag: number;
	private relocateMode: RelocateMode;
	private selectedId: string | null;
	private proximityCount: number;
	private connectCount: number;
	private indirectCount: number;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SCENE_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixture = parseFixtureV1(nakaginSceneFixtureJson as unknown);
		this.automaticLod = true;
		this.depthVariableLod = false;
		this.manualLod = DEFAULT_MANUAL_LOD;
		this.lodSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
		this.lodTag = DEFAULT_MANUAL_LOD;
		this.relocateMode = "translate";
		this.selectedId = null;
		this.proximityCount = 0;
		this.connectCount = 0;
		this.indirectCount = 0;
		this.rebuildShellMode();
	}

	private lodMeasures(): readonly WindowMeasure[] {
		return [
			{
				kind: "toggle",
				id: `${SCENE_PLAY_WINDOW_ID}-auto`,
				label: "LOD",
				text: "Auto zoom",
				pressed: this.automaticLod,
				onChange: { controllerId: SCENE_PLAY_CONTROLLER_ID, command: "setAutoLod" },
			},
			{
				kind: "toggle",
				id: `${SCENE_PLAY_WINDOW_ID}-depth`,
				text: "Depth-variable",
				pressed: this.depthVariableLod,
				onChange: { controllerId: SCENE_PLAY_CONTROLLER_ID, command: "setDepthLod" },
			},
			{
				kind: "slider",
				id: `${SCENE_PLAY_WINDOW_ID}-lod`,
				label: formatSceneLod(this.lodTag),
				value: this.lodSlider,
				min: SCENE_LOD_SLIDER_MIN,
				max: SCENE_LOD_SLIDER_MAX,
				step: 1,
				onChange: { controllerId: SCENE_PLAY_CONTROLLER_ID, command: "setManualLod" },
			},
		];
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WindowKindRuntime(SCENE_PLAY_WINDOW_ID, SCENE_PLAY_WINDOW_LABEL, SCENE_PLAY_BODY_KEY, undefined, this.lodMeasures()),
		];
		const relocateTools: ToolItem[] = (["translate", "rotate", "scale"] as const).map((mode, order) => ({
			id: `scene.relocate.${mode}`,
			kind: "toggle" as const,
			text: mode.charAt(0).toUpperCase() + mode.slice(1),
			order,
			pressed: this.relocateMode === mode,
			controllerId: SCENE_PLAY_CONTROLLER_ID,
			command: "setRelocateMode",
			args: { mode },
		}));
		const tools: AppTools = { actions: relocateTools };
		this.mainMode.tools = tools;
	}

	override run(command: string, args?: unknown): void {
		let syncShell = true;
		switch (command) {
			case "setAutoLod": {
				const pressed = (args as { pressed?: boolean }).pressed;
				if (typeof pressed === "boolean") this.automaticLod = pressed;
				break;
			}
			case "setDepthLod": {
				const pressed = (args as { pressed?: boolean }).pressed;
				if (typeof pressed === "boolean") this.depthVariableLod = pressed;
				break;
			}
			case "setManualLod": {
				const value = (args as { value?: number }).value;
				if (typeof value === "number" && Number.isFinite(value)) {
					this.lodSlider = value;
					this.manualLod = lodFromSliderValue(value);
				}
				break;
			}
			case "setEffectiveLod": {
				const lod = (args as { lod: number }).lod;
				if (typeof lod === "number" && Number.isFinite(lod) && lod > 0) {
					this.lodTag = lod;
				}
				syncShell = false;
				break;
			}
			case "setRelocateMode": {
				const mode = (args as { mode: RelocateMode }).mode;
				if (mode === "translate" || mode === "rotate" || mode === "scale") this.relocateMode = mode;
				break;
			}
			case "setSelectedId": {
				this.selectedId = (args as { id: string | null }).id;
				break;
			}
			case "noteSelection": {
				this.selectedId = (args as { objectIds: readonly string[] }).objectIds[0] ?? null;
				break;
			}
			case "noteProximity":
				this.proximityCount += 1;
				break;
			case "noteConnect":
				this.connectCount += 1;
				break;
			case "noteIndirect":
				this.indirectCount += 1;
				break;
			default:
				break;
		}
		if (syncShell) {
			this.rebuildShellMode();
			this.emit();
		}
	}

	getSnapshot(): ScenePlaySnapshot {
		return {
			fixture: this.fixture,
			lodProps: sceneLodCanvasProps({
				automaticLod: this.automaticLod,
				depthVariableLod: this.depthVariableLod,
				manualLod: this.manualLod,
			}),
			lodTag: this.lodTag,
			lodSlider: this.lodSlider,
			automaticLod: this.automaticLod,
			depthVariableLod: this.depthVariableLod,
			relocateMode: this.relocateMode,
			selectedId: this.selectedId,
			proximityCount: this.proximityCount,
			connectCount: this.connectCount,
			indirectCount: this.indirectCount,
		};
	}
}

/** @emoji 📸 Host-consumed scene play state (no React/DOM). */
export interface ScenePlaySnapshot {
	readonly fixture: FixtureV1 | null;
	readonly lodProps: ReturnType<typeof sceneLodCanvasProps>;
	readonly lodTag: number;
	readonly lodSlider: number;
	readonly automaticLod: boolean;
	readonly depthVariableLod: boolean;
	readonly relocateMode: RelocateMode;
	readonly selectedId: string | null;
	readonly proximityCount: number;
	readonly connectCount: number;
	readonly indirectCount: number;
}

export function buildScenePlayAppRuntime(controller: ScenePlayShellController): AppRuntime {
	const app = new AppRuntime(
		PLAY_APP_ID,
		"Scene play",
		undefined,
		controller,
		createStackLayout([SCENE_PLAY_WINDOW_ID], [SCENE_PLAY_WINDOW_LABEL]) as never,
		[new WindowKindRuntime(SCENE_PLAY_WINDOW_ID, SCENE_PLAY_WINDOW_LABEL, SCENE_PLAY_BODY_KEY)],
	);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

function sceneControllerFromContext(ctx: WindowBodyViewContext): ScenePlayShellController | undefined {
	return ctx.runtime.getActiveApp()?.controller as ScenePlayShellController | undefined;
}

/** @emoji 🧩 Declarative scene window: fullscreen scene3d only (relocate tools live on {@link ModeRuntime.tools}). */
export function buildScenePlayDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
	const ctrl = sceneControllerFromContext(ctx);
	if (!ctrl) {
		return { type: "text", value: "Missing scene controller" };
	}
	const snap = ctrl.getSnapshot();
	if (!snap.fixture) {
		return { type: "text", value: "Invalid scene fixture" };
	}
	return buildScene3dWindowBody(SCENE_PLAY_SCENE_SURFACE_ID, SCENE_PLAY_CONTROLLER_ID);
}
//#endregion 🔖ScenePlayController

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("scene play fixture", () => {
		it("parses nakagin fixture", () => {
			const f = parseFixtureV1(nakaginSceneFixtureJson as unknown);
			expect(f?.domain).toBe("architecture");
			expect(f?.attractions).toEqual([]);
			expect(f?.objects.length).toBeGreaterThan(0);
		});

		it("declarative window body is a lone scene3d surface", () => {
			const bus = new CommandBus();
			const wb = new ProductRuntime();
			const ctrl = new ScenePlayShellController(bus, () => wb.notify());
			wb.addApp(buildScenePlayAppRuntime(ctrl));
			const tree = buildScenePlayDeclarativeBody({
				runtime: wb,
				windowKindId: SCENE_PLAY_WINDOW_ID,
				bodyKey: SCENE_PLAY_BODY_KEY,
				activeModeId: "main",
				generation: wb.generation,
			});
			expect(tree).toEqual(buildScene3dWindowBody(SCENE_PLAY_SCENE_SURFACE_ID, SCENE_PLAY_CONTROLLER_ID));
		});
	});
}
//#endregion 🧪Tests
