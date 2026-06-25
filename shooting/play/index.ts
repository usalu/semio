// #region 🧲Header
/** @emoji 📸 Shooting play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildShootingWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	enforcePlaygroundWindowEngagementInput,
	isPlaygroundFixtureLocked,
	isPlaygroundNoFixtureId,
	PLAYGROUND_NO_FIXTURE_ID,
	playgroundResolvedFixtureId,
	registerWindowBody,
	type AppTools,
	type CommandDescriptor,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	type ToolItem,
	type WindowEngagement,
	type WindowMeasure,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
	DEFAULT_SHOOTING_FIXTURE,
	parseShootingFixture,
	resolveActiveShot,
	shootingFixtureToJson,
	type ShootingCameraV1,
	type ShootingFixtureV1,
	type ShootingSceneV1,
	type ShootingShotV1,
} from "@semio-tech/shooting-react";
import { SHOOTING_PLAY_FIXTURE_DEFAULT_ID, resolveShootingPlayFixtureSlug } from "./fixture-slugs.ts";

export const SHOOTING_PLAY_APP_ID = "shooting-play";
export const SHOOTING_PLAY_CONTROLLER_ID = "shooting-play";
export const SHOOTING_PLAY_SURFACE_ID_MODEL = "shooting.play.model/v1";
export const SHOOTING_PLAY_SURFACE_ID_ICON = "shooting.play.icon/v1";
export const SHOOTING_PLAY_BODY_KEY_MODEL = "shooting.play.model";
export const SHOOTING_PLAY_BODY_KEY_ICON = "shooting.play.icon";
export const SHOOTING_PLAY_WINDOW_KIND_MODEL = "shooting-model";
export const SHOOTING_PLAY_WINDOW_KIND_ICON = "shooting-icon";

export const SHOOTING_PLAY_LAYOUT = createDefaultLayout(
	[SHOOTING_PLAY_WINDOW_KIND_MODEL, SHOOTING_PLAY_WINDOW_KIND_ICON],
	"row",
	[55, 45],
	["Model", "Icon"],
);

export { SHOOTING_PLAY_FIXTURE_DEFAULT_ID, resolveShootingPlayFixtureSlug };

const shootingFixtureModules = import.meta.glob("../fixture/*.shooting.json", { eager: true }) as Record<string, { default: unknown }>;

function shootingFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.shooting\.json$/, "");
}

function shootingFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const SHOOTING_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(shootingFixtureModules).map(([path, mod]) => {
		const id = shootingFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const SHOOTING_PLAY_EMPTY_FIXTURE_JSON = shootingFixtureToJson({
	...DEFAULT_SHOOTING_FIXTURE,
	assets: [],
	shots: [],
});

export const SHOOTING_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = [
	{ id: SHOOTING_PLAY_FIXTURE_DEFAULT_ID, label: "Default Base Icon" },
	...Object.keys(SHOOTING_PLAY_FILE_FIXTURE_JSON_BY_ID)
		.sort()
		.map((id) => ({ id, label: shootingFixtureLabelFromId(id) })),
];

const SHOOTING_PLAY_STORE_KEY = "shooting.fixture/v1";

export interface ShootingPlayFixtureStore {
	load(): string | null;
	save(fixtureJson: string): void;
	clear(): void;
}

export function createShootingPlayFixtureStore(storage?: Pick<Storage, "getItem" | "setItem" | "removeItem">): ShootingPlayFixtureStore {
	const resolved =
		storage ??
		(typeof globalThis.localStorage !== "undefined"
			? globalThis.localStorage
			: (() => {
					const backing = new Map<string, string>();
					return {
						getItem: (key: string) => backing.get(key) ?? null,
						setItem: (key: string, value: string) => {
							backing.set(key, value);
						},
						removeItem: (key: string) => {
							backing.delete(key);
						},
					};
				})());
	return {
		load(): string | null {
			return resolved.getItem(SHOOTING_PLAY_STORE_KEY);
		},
		save(fixtureJson: string): void {
			resolved.setItem(SHOOTING_PLAY_STORE_KEY, fixtureJson);
		},
		clear(): void {
			resolved.removeItem(SHOOTING_PLAY_STORE_KEY);
		},
	};
}

export interface ShootingPlayToolbarState {
	readonly hasStoredFixture: boolean;
	readonly activeShotId: string | null;
}

export interface ShootingPlayHostBridge {
	getToolbarState(): ShootingPlayToolbarState;
	runHostCommand(command: string, args?: unknown): void;
}

function shootingPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: SHOOTING_PLAY_CONTROLLER_ID, command, args };
}

function shootingFixtureJsonForId(fixtureId: string): string {
	if (isPlaygroundNoFixtureId(fixtureId)) return SHOOTING_PLAY_EMPTY_FIXTURE_JSON;
	if (fixtureId === SHOOTING_PLAY_FIXTURE_DEFAULT_ID) return shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE);
	return SHOOTING_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId] ?? SHOOTING_PLAY_EMPTY_FIXTURE_JSON;
}

export function buildShootingPlayToolbarTools(state: ShootingPlayToolbarState, controllerId: string): AppTools {
	return {
		open: [
			{
				id: "shooting.open.fixture",
				kind: "button",
				iconId: "folder-open",
				label: "Import Shooting",
				order: 0,
				controllerId,
				command: "loadRequest",
			},
			{
				id: "shooting.open.asset",
				kind: "button",
				iconId: "box",
				label: "Import Glb",
				order: 1,
				controllerId,
				command: "importAssetRequest",
			},
		],
		save: [
			{
				id: "shooting.save.stored",
				kind: "button",
				iconId: "hard-drive",
				label: "Store",
				order: 0,
				controllerId,
				command: "saveStored",
			},
			{
				id: "shooting.save.download",
				kind: "button",
				iconId: "save",
				label: "Download Shooting",
				order: 1,
				controllerId,
				command: "saveDownload",
			},
			{
				id: "shooting.save.shot",
				kind: "button",
				iconId: "image",
				label: "Export Shot",
				order: 2,
				disabled: !state.activeShotId,
				controllerId,
				command: "exportActiveShot",
			},
			{
				id: "shooting.save.allShots",
				kind: "button",
				iconId: "images",
				label: "Export All Shots",
				order: 3,
				controllerId,
				command: "exportAllShots",
			},
			{
				id: "shooting.save.loadStored",
				kind: "button",
				iconId: "rotate-ccw",
				label: "Restore",
				order: 4,
				disabled: !state.hasStoredFixture,
				controllerId,
				command: "loadStored",
			},
			{
				id: "shooting.save.reset",
				kind: "button",
				iconId: "refresh-cw",
				label: "Reset",
				order: 5,
				controllerId,
				command: "resetFixture",
			},
		],
		actions: [
			{
				id: "shooting.camera.save",
				kind: "button",
				iconId: "camera",
				label: "Save Camera",
				order: 0,
				controllerId,
				command: "saveCamera",
			},
			{
				id: "shooting.camera.load",
				kind: "button",
				iconId: "video",
				label: "Load Camera",
				order: 1,
				controllerId,
				command: "loadCameraMenu",
			},
		],
	};
}

export class ShootingPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Shooting", undefined);
	private activeFixtureId = playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID);
	private fixture: ShootingFixtureV1 = parseShootingFixture(shootingFixtureJsonForId(playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID))) ?? {
		...DEFAULT_SHOOTING_FIXTURE,
		assets: [],
		shots: [],
	};
	private readonly fixtureStore: ShootingPlayFixtureStore;
	private hostBridge: ShootingPlayHostBridge | null = null;
	private renderRevision = 0;
	private cameraDraftLabel = "Camera 1";

	constructor(commandBus: CommandBus, hostNotify: () => void, fixtureStore: ShootingPlayFixtureStore = createShootingPlayFixtureStore()) {
		super(SHOOTING_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixtureStore = fixtureStore;
		this.rebuildShellMode();
	}

	getFixture(): ShootingFixtureV1 {
		return this.fixture;
	}

	getFixtureJson(): string {
		return shootingFixtureToJson(this.fixture);
	}

	getRenderRevision(): number {
		return this.renderRevision;
	}

	hasStoredFixture(): boolean {
		return this.fixtureStore.load() != null;
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog | null {
		if (isPlaygroundFixtureLocked()) return null;
		return { activeFixtureId: this.activeFixtureId, options: [...SHOOTING_PLAY_FIXTURE_OPTIONS] };
	}

	setHostBridge(bridge: ShootingPlayHostBridge | null): void {
		this.hostBridge = bridge;
		this.rebuildToolbarTools();
	}

	private toolbarState(): ShootingPlayToolbarState {
		return (
			this.hostBridge?.getToolbarState() ?? {
				hasStoredFixture: this.hasStoredFixture(),
				activeShotId: this.fixture.activeShotId ?? resolveActiveShot(this.fixture)?.id ?? null,
			}
		);
	}

	rebuildToolbarTools(): void {
		if (!this.hostBridge) {
			this.mainMode.tools = undefined;
			return;
		}
		this.mainMode.tools = buildShootingPlayToolbarTools(this.toolbarState(), this.id);
	}

	private applyFixture(fixture: ShootingFixtureV1): void {
		this.fixture = fixture;
		this.renderRevision += 1;
		this.rebuildShellMode();
		this.emit();
	}

	private applyFixtureJson(json: string): void {
		const parsed = parseShootingFixture(json);
		if (!parsed) return;
		this.applyFixture(parsed);
	}

	private modelMeasures(): readonly WindowMeasure[] {
		const scene = this.fixture.scene;
		return [
			{
				kind: "slider",
				id: "shooting-sun-azimuth",
				label: "Sun Azimuth",
				value: scene.sun.azimuth,
				min: 0,
				max: 360,
				step: 1,
				onChange: shootingPlayCmd("setSunAzimuth"),
			},
			{
				kind: "slider",
				id: "shooting-sun-elevation",
				label: "Sun Elevation",
				value: scene.sun.elevation,
				min: -10,
				max: 90,
				step: 1,
				onChange: shootingPlayCmd("setSunElevation"),
			},
			{
				kind: "slider",
				id: "shooting-sun-intensity",
				label: "Sun Intensity",
				value: scene.sun.intensity,
				min: 0,
				max: 5,
				step: 0.1,
				onChange: shootingPlayCmd("setSunIntensity"),
			},
			{
				kind: "slider",
				id: "shooting-ambient-intensity",
				label: "Ambient",
				value: scene.ambient.intensity,
				min: 0,
				max: 3,
				step: 0.05,
				onChange: shootingPlayCmd("setAmbientIntensity"),
			},
			{
				kind: "toggle",
				id: "shooting-shadow-enabled",
				label: "Shadow",
				value: scene.shadow.enabled,
				onChange: shootingPlayCmd("setShadowEnabled"),
			},
			{
				kind: "slider",
				id: "shooting-material-roughness",
				label: "Roughness",
				value: scene.material.roughness,
				min: 0,
				max: 1,
				step: 0.05,
				onChange: shootingPlayCmd("setMaterialRoughness"),
			},
		];
	}

	private iconMeasures(): readonly WindowMeasure[] {
		const activeShot = resolveActiveShot(this.fixture);
		return [
			{
				kind: "select",
				id: "shooting-active-shot",
				label: "Shot",
				value: activeShot?.id ?? "",
				items: this.fixture.shots.map((shot) => ({ id: shot.id, value: shot.id, label: shot.label })),
				onChange: shootingPlayCmd("setActiveShot"),
			},
			{
				kind: "select",
				id: "shooting-shot-format",
				label: "Format",
				value: activeShot?.format ?? "svg",
				items: [
					{ id: "svg", value: "svg", label: "SVG" },
					{ id: "png", value: "png", label: "PNG" },
				],
				onChange: shootingPlayCmd("setActiveShotFormat"),
			},
		];
	}

	private modelEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "shooting-camera-label",
				value: this.cameraDraftLabel,
				placeholder: "Camera label",
				onChange: shootingPlayCmd("setCameraDraftLabel"),
				onSubmit: shootingPlayCmd("saveCamera"),
			},
			possibleEngagements: this.fixture.savedCameras.map((entry) => ({
				id: `shooting.camera.${entry.id}`,
				label: entry.label,
				command: shootingPlayCmd("loadSavedCamera", { id: entry.id }),
			})),
			status: [{ id: "shooting-asset-count", text: `${this.fixture.assets.length} assets · ${this.fixture.shots.length} shots` }],
		};
	}

	private iconEngagement(): WindowEngagement {
		const shot = resolveActiveShot(this.fixture);
		return {
			sessionActive: false,
			input: {
				id: "shooting-shot-label",
				value: shot?.label ?? "",
				placeholder: "Shot label",
				onChange: shootingPlayCmd("setActiveShotLabel"),
				onSubmit: shootingPlayCmd("commitActiveShotLabel"),
			},
			status: shot ? [{ id: "shooting-shot-size", text: `${shot.width}×${shot.height} ${shot.format.toUpperCase()}` }] : [],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WindowKindRuntime(
				SHOOTING_PLAY_WINDOW_KIND_MODEL,
				"Model",
				SHOOTING_PLAY_BODY_KEY_MODEL,
				undefined,
				this.modelMeasures(),
				this.modelEngagement(),
			),
			new WindowKindRuntime(
				SHOOTING_PLAY_WINDOW_KIND_ICON,
				"Icon",
				SHOOTING_PLAY_BODY_KEY_ICON,
				undefined,
				this.iconMeasures(),
				this.iconEngagement(),
			),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Shooting play window "${windowKind.id}"`);
		}
		this.rebuildToolbarTools();
	}

	private patchScene(patch: Partial<ShootingSceneV1>): void {
		this.applyFixture({ ...this.fixture, scene: { ...this.fixture.scene, ...patch } });
	}

	override run(command: string, args?: unknown): void {
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") this.applyFixtureJson(json);
			return;
		}
		if (command === "setActiveFixture") {
			if (isPlaygroundFixtureLocked()) return;
			const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
			this.activeFixtureId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
			this.applyFixtureJson(shootingFixtureJsonForId(this.activeFixtureId));
			return;
		}
		if (command === "setCamera") {
			const camera = (args as { camera?: ShootingCameraV1 }).camera;
			if (!camera) return;
			this.applyFixture({ ...this.fixture, camera });
			return;
		}
		if (command === "setCameraDraftLabel") {
			const value = (args as { value?: string }).value;
			if (typeof value === "string") {
				this.cameraDraftLabel = value;
				this.rebuildShellMode();
				this.emit();
			}
			return;
		}
		if (command === "saveCamera") {
			const label = ((args as { value?: string }).value ?? this.cameraDraftLabel).trim() || "Camera";
			const id = `camera_${Date.now()}`;
			this.applyFixture({
				...this.fixture,
				savedCameras: [...this.fixture.savedCameras, { id, label, camera: this.fixture.camera }],
			});
			console.log(`[DEBUG] shooting saved camera ${id} ${label}`);
			return;
		}
		if (command === "loadSavedCamera") {
			const id = (args as { id?: string }).id;
			const saved = this.fixture.savedCameras.find((entry) => entry.id === id);
			if (!saved) return;
			this.applyFixture({ ...this.fixture, camera: saved.camera });
			console.log(`[DEBUG] shooting loaded camera ${saved.id}`);
			return;
		}
		if (command === "loadCameraMenu") {
			const first = this.fixture.savedCameras[0];
			if (first) this.run("loadSavedCamera", { id: first.id });
			return;
		}
		if (command === "setActiveShot") {
			const value = (args as { value?: string }).value ?? (args as { id?: string }).id;
			if (typeof value !== "string" || !value) return;
			this.applyFixture({ ...this.fixture, activeShotId: value });
			return;
		}
		if (command === "setActiveShotFormat") {
			const value = (args as { value?: string }).value;
			if (value !== "svg" && value !== "png") return;
			const active = resolveActiveShot(this.fixture);
			if (!active) return;
			const shots = this.fixture.shots.map((shot) => (shot.id === active.id ? { ...shot, format: value } : shot));
			this.applyFixture({ ...this.fixture, shots });
			return;
		}
		if (command === "setActiveShotLabel") {
			const value = (args as { value?: string }).value;
			if (typeof value !== "string") return;
			const active = resolveActiveShot(this.fixture);
			if (!active) return;
			const shots = this.fixture.shots.map((shot) => (shot.id === active.id ? { ...shot, label: value } : shot));
			this.applyFixture({ ...this.fixture, shots });
			return;
		}
		if (command === "commitActiveShotLabel") {
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "setSunAzimuth") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ sun: { ...this.fixture.scene.sun, azimuth: value } });
			return;
		}
		if (command === "setSunElevation") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ sun: { ...this.fixture.scene.sun, elevation: value } });
			return;
		}
		if (command === "setSunIntensity") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ sun: { ...this.fixture.scene.sun, intensity: value } });
			return;
		}
		if (command === "setAmbientIntensity") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ ambient: { ...this.fixture.scene.ambient, intensity: value } });
			return;
		}
		if (command === "setShadowEnabled") {
			const value = (args as { value?: boolean }).value;
			if (typeof value !== "boolean") return;
			this.patchScene({ shadow: { ...this.fixture.scene.shadow, enabled: value } });
			return;
		}
		if (command === "setMaterialRoughness") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ material: { ...this.fixture.scene.material, roughness: value } });
			return;
		}
		if (command === "saveStored") {
			this.fixtureStore.save(this.getFixtureJson());
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "saveDownload" || command === "loadRequest" || command === "importAssetRequest" || command === "exportActiveShot" || command === "exportAllShots") {
			this.hostBridge?.runHostCommand(command, args);
			return;
		}
		if (command === "loadStored") {
			const json = this.fixtureStore.load();
			if (json) this.applyFixtureJson(json);
			return;
		}
		if (command === "resetFixture") {
			this.fixtureStore.clear();
			this.activeFixtureId = PLAYGROUND_NO_FIXTURE_ID;
			this.applyFixtureJson(SHOOTING_PLAY_EMPTY_FIXTURE_JSON);
			return;
		}
		if (command === "importAsset") {
			const asset = (args as { asset?: ShootingFixtureV1["assets"][number] }).asset;
			if (!asset) return;
			this.applyFixture({
				...this.fixture,
				assets: [...this.fixture.assets, asset],
				activeAssetId: asset.id,
			});
			console.log(`[DEBUG] shooting imported asset ${asset.id}`);
			return;
		}
	}
}

export function registerShootingPlayDeclarativeBodies(): void {
	registerWindowBody(SHOOTING_PLAY_BODY_KEY_MODEL, () =>
		buildShootingWindowBody(SHOOTING_PLAY_SURFACE_ID_MODEL, SHOOTING_PLAY_CONTROLLER_ID, "model"));
	registerWindowBody(SHOOTING_PLAY_BODY_KEY_ICON, () =>
		buildShootingWindowBody(SHOOTING_PLAY_SURFACE_ID_ICON, SHOOTING_PLAY_CONTROLLER_ID, "icon"));
}

export function buildShootingPlayAppRuntime(controller: ShootingPlayController): AppRuntime {
	return createPlayAppRuntime(SHOOTING_PLAY_APP_ID, "semio · shooting", controller, SHOOTING_PLAY_LAYOUT, controller.mainMode);
}

export class PlaygroundShooting extends Playground {
	readonly id = SHOOTING_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new ShootingPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildShootingPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerShootingPlayDeclarativeBodies();
	}
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/shooting-play", () => {
		it("exports default layout with model and icon windows", () => {
			expect(SHOOTING_PLAY_LAYOUT.root.kind).toBe("row");
		});

		it("controller stores fixture json", () => {
			const bus = new CommandBus();
			const ctrl = new ShootingPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE) });
			expect(ctrl.getFixtureJson()).toContain("shooting.fixture/v1");
		});

		it("fixture catalog includes shooting/fixture files", () => {
			expect(SHOOTING_PLAY_FIXTURE_OPTIONS.some((option) => option.id === "base-icon")).toBe(true);
		});

		it("toolbar includes import and export actions", () => {
			const tools = buildShootingPlayToolbarTools({ hasStoredFixture: false, activeShotId: "overview-svg" }, SHOOTING_PLAY_CONTROLLER_ID);
			expect(tools.open?.some((row) => row.id === "shooting.open.fixture")).toBe(true);
			expect(tools.save?.some((row) => row.id === "shooting.save.shot")).toBe(true);
		});
	});
}
// #endregion 🧪Tests

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "shooting") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootShootingPlay } = await import("@semio-tech/framework-playground-renderer-react/shooting");
		bootShootingPlay(new PlaygroundShooting());
	})();
}
// #endregion 🔖Boot
