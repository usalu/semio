// #region ­ƒº▓Header
// ­ƒÆ╗ elements/client/lib/system/renderer/react/scene/play/index.ts ÔÇö Scene play harness: Nakagin fixture metadata, LOD tiers, and localStorage keys (no React; mount via main.ts + scene-play-host).
// #endregion ­ƒº▓Header

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
	LOD_MODE_AUTOMATIC,
	isLodKind,
	lodAutomaticSelectLabel,
	lodCanvasProps,
	parseFixtureV1,
	type FixtureV1,
	type KindCatalogBundle,
	type KindCompatEntry,
	type LodKind,
	type LodModeKind,
	type RelocateMode,
} from "../index.tsx";

//#region ­ƒº¥Meta
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
//#endregion ­ƒº¥Meta

//#region ­ƒûÑ´©ÅSurface
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
//#endregion ­ƒûÑ´©ÅSurface

//#region ­ƒÄ¼Play
export const PLAY_LOD_TIERS: LodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

export function playLodTierMenuLabel(tier: LodKind): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}
export const PLAY_APP_ID = "elements-scene-play";
export const SCENE_PLAY_WINDOW_ID = "scene-main";
export const SCENE_PLAY_WINDOW_LABEL = "Scene";
export const SCENE_PLAY_BODY_KEY = "elements.scene.play.window";
export const SCENE_PLAY_CONTROLLER_ID = "scene-play";
export const SCENE_PLAY_SCENE_SURFACE_ID = "elements.scene.play.scene/v1";
//#endregion ­ƒÄ¼Play

export { parseKindCatalogs, parseKindCompatibility };

//#region ­ƒöûScenePlayController
/** @emoji ­ƒÄ¼ Framework-free scene play controller: fixture, LOD, selection, and interaction counters. */
export class ScenePlayShellController extends Controller {
	readonly mainMode = new WorkbenchMode("main", "Scene", undefined);
	readonly fixture: FixtureV1 | null;
	private lodMode: LodModeKind;
	private lodTag: LodKind;
	private relocateMode: RelocateMode;
	private selectedId: string | null;
	private proximityCount: number;
	private connectCount: number;
	private indirectCount: number;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SCENE_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixture = parseFixtureV1(nakaginSceneFixtureJson as unknown);
		this.lodMode = LOD_MODE_AUTOMATIC;
		this.lodTag = "normal";
		this.relocateMode = "translate";
		this.selectedId = null;
		this.proximityCount = 0;
		this.connectCount = 0;
		this.indirectCount = 0;
		this.rebuildShellMode();
	}

	private rebuildShellMode(): void {
		const lodMeasure: ShellWindowMeasure = {
			kind: "select",
			id: `${SCENE_PLAY_WINDOW_ID}-lod`,
			label: "LOD",
			value: this.lodMode,
			items: [
				{ id: "automatic", value: LOD_MODE_AUTOMATIC, label: lodAutomaticSelectLabel(this.lodTag) },
				...PLAY_LOD_TIERS.map((tier) => ({ id: tier, value: tier, label: playLodTierMenuLabel(tier) })),
			],
			onChange: { controllerId: SCENE_PLAY_CONTROLLER_ID, command: "setLodMode" },
		};
		this.mainMode.windowKinds = [
			new WorkbenchWindowKind(SCENE_PLAY_WINDOW_ID, SCENE_PLAY_WINDOW_LABEL, SCENE_PLAY_BODY_KEY, undefined, [lodMeasure]),
		];
		const relocateTools: ShellToolItem[] = (["translate", "rotate", "scale"] as const).map((mode, order) => ({
			id: `scene.relocate.${mode}`,
			kind: "toggle" as const,
			text: mode.charAt(0).toUpperCase() + mode.slice(1),
			order,
			pressed: this.relocateMode === mode,
			controllerId: SCENE_PLAY_CONTROLLER_ID,
			command: "setRelocateMode",
			args: { mode },
		}));
		const tools: ShellAppTools = { actions: relocateTools };
		this.mainMode.tools = tools;
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setLodMode": {
				const value = (args as { value?: string }).value;
				if (value === LOD_MODE_AUTOMATIC || (typeof value === "string" && isLodKind(value))) {
					this.lodMode = value as LodModeKind;
				}
				break;
			}
			case "setEffectiveLod": {
				const lod = (args as { lod: LodKind }).lod;
				if (isLodKind(lod)) this.lodTag = lod;
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
		this.rebuildShellMode();
		this.emit();
	}

	getSnapshot(): ScenePlaySnapshot {
		return {
			fixture: this.fixture,
			lodProps: lodCanvasProps(this.lodMode),
			lodTag: this.lodTag,
			relocateMode: this.relocateMode,
			selectedId: this.selectedId,
			proximityCount: this.proximityCount,
			connectCount: this.connectCount,
			indirectCount: this.indirectCount,
		};
	}
}

/** @emoji ­ƒô© Host-consumed scene play state (no React/DOM). */
export interface ScenePlaySnapshot {
	readonly fixture: FixtureV1 | null;
	readonly lodProps: ReturnType<typeof lodCanvasProps>;
	readonly lodTag: LodKind;
	readonly relocateMode: RelocateMode;
	readonly selectedId: string | null;
	readonly proximityCount: number;
	readonly connectCount: number;
	readonly indirectCount: number;
}

export function buildScenePlayWorkbenchApp(controller: ScenePlayShellController): WorkbenchApp {
	const app = new WorkbenchApp(
		PLAY_APP_ID,
		"Scene play",
		undefined,
		controller,
		createStackLayout([SCENE_PLAY_WINDOW_ID], [SCENE_PLAY_WINDOW_LABEL]) as never,
		[new WorkbenchWindowKind(SCENE_PLAY_WINDOW_ID, SCENE_PLAY_WINDOW_LABEL, SCENE_PLAY_BODY_KEY)],
	);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

function sceneControllerFromContext(ctx: ShellWindowBodyViewContext): ScenePlayShellController | undefined {
	return ctx.workbench.getActiveApp()?.controller as ScenePlayShellController | undefined;
}

/** @emoji ­ƒº® Declarative scene window: fullscreen scene3d only (relocate tools live on {@link WorkbenchMode.tools}). */
export function buildScenePlayDeclarativeBody(ctx: ShellWindowBodyViewContext): UiNode {
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
//#endregion ­ƒöûScenePlayController

//#region ­ƒº¬Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("scene play fixture", () => {
		it("parses nakagin fixture", () => {
			const f = parseFixtureV1(nakaginSceneFixtureJson as unknown);
			expect(f?.domain).toBe("architecture");
			expect(f?.attractions.length).toBeGreaterThan(0);
			expect(f?.objects.length).toBeGreaterThan(0);
		});

		it("declarative window body is a lone scene3d surface", () => {
			const bus = new CommandBus();
			const wb = new Workbench();
			const ctrl = new ScenePlayShellController(bus, () => wb.notify());
			wb.addApp(buildScenePlayWorkbenchApp(ctrl));
			const tree = buildScenePlayDeclarativeBody({
				workbench: wb,
				windowKindId: SCENE_PLAY_WINDOW_ID,
				bodyKey: SCENE_PLAY_BODY_KEY,
				activeModeId: "main",
				generation: wb.generation,
			});
			expect(tree).toEqual(buildScene3dWindowBody(SCENE_PLAY_SCENE_SURFACE_ID, SCENE_PLAY_CONTROLLER_ID));
		});
	});
}
//#endregion ­ƒº¬Tests
