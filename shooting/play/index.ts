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
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	uiDeclarativeSectionsToTree,
	type AppTools,
	type CommandDescriptor,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	type ToolLeaf,
	toolCollection,
	type UiNode,
	type UiSectionNode,
	type UiTreeItemNode,
	type WindowEngagement,
	type WindowMeasure,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
	DocumentVcsStore,
	applyJsonReplaceOp,
	createDocumentVcsEnvelope,
	recordJsonProjectionChange,
	type JsonReplaceOp,
} from "@semio-tech/framework-core";
import {
	DEFAULT_SHOOTING_FIXTURE,
	parseShootingFixture,
	resolveActiveShot,
	resolveActiveAsset,
	applyShootingCameraToFixture,
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
export const SHOOTING_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const SHOOTING_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const SHOOTING_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

export type ShootingPlaySelectionKind = "shot" | "asset";

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

// #region 🔖ShootingPlayPanels
export function buildShootingPlayHierarchyTree(fixture: ShootingFixtureV1, selectedId: string | null, selectedKind: ShootingPlaySelectionKind | null): UiNode {
	const shotItems: UiTreeItemNode[] = fixture.shots.map((shot) => ({
		id: `shooting-play-hierarchy.shot.${shot.id}`,
		label: shot.label || shot.id,
		description: `${shot.width}×${shot.height} ${shot.format.toUpperCase()}`,
		command: shootingPlayCmd("setSelection", { selectedId: shot.id, selectedKind: "shot" }),
	}));
	const assetItems: UiTreeItemNode[] = fixture.assets.map((asset) => ({
		id: `shooting-play-hierarchy.asset.${asset.id}`,
		label: asset.name || asset.id,
		description: asset.format,
		command: shootingPlayCmd("setSelection", { selectedId: asset.id, selectedKind: "asset" }),
	}));
	const selectedIds = selectedId && selectedKind ? [`shooting-play-hierarchy.${selectedKind}.${selectedId}`] : [];
	return {
		type: "tree",
		sections: [
			{
				id: "shooting-play-hierarchy.shots",
				label: "Shots",
				defaultOpen: true,
				items: shotItems.length ? shotItems : [{ id: "shooting-play-hierarchy.shots.empty", label: "(none)" }],
			},
			{
				id: "shooting-play-hierarchy.assets",
				label: "Assets",
				defaultOpen: false,
				items: assetItems.length ? assetItems : [{ id: "shooting-play-hierarchy.assets.empty", label: "(none)" }],
			},
		],
		selectedIds,
	};
}

export function buildShootingPlayCatalogueTree(): UiNode {
	return {
		type: "tree",
		sections: [
			{
				id: "shooting-play-catalogue.shots",
				label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
				defaultOpen: true,
				items: [
					{ id: "shooting-play-catalogue.svg", label: "SVG shot", description: "svg" },
					{ id: "shooting-play-catalogue.png", label: "PNG shot", description: "png" },
				],
			},
			{
				id: "shooting-play-catalogue.assets",
				label: "Assets",
				defaultOpen: false,
				items: [{ id: "shooting-play-catalogue.glb", label: "GLB asset", description: "glb" }],
			},
		],
	};
}

export function buildShootingPlayInspectorTree(fixture: ShootingFixtureV1, selectedId: string | null, selectedKind: ShootingPlaySelectionKind | null): UiNode {
	if (!selectedId || !selectedKind) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "shooting-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a shot or asset in the hierarchy." }],
			},
		]);
	}
	if (selectedKind === "shot") {
		const shot = fixture.shots.find((entry) => entry.id === selectedId);
		if (!shot) {
			return uiDeclarativeSectionsToTree([
				{ type: "section", id: "shooting-play-inspector.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Shot not found" }] },
			]);
		}
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "shooting-play-inspector.shot",
				label: shot.label || shot.id,
				children: [
					{
						type: "field",
						id: "shooting-play-inspector.shot.label",
						label: "Label",
						child: {
							type: "input",
							id: "shooting-play-inspector.shot.label.input",
							inputKind: "text",
							value: shot.label,
							onChange: shootingPlayCmd("patchShot", { shotId: shot.id, field: "label" }),
						},
					},
					{
						type: "field",
						id: "shooting-play-inspector.shot.format",
						label: "Format",
						child: {
							type: "select",
							id: "shooting-play-inspector.shot.format.select",
							value: shot.format,
							items: [
								{ id: "svg", value: "svg", label: "SVG" },
								{ id: "png", value: "png", label: "PNG" },
							],
							onChange: shootingPlayCmd("patchShot", { shotId: shot.id, field: "format" }),
						},
					},
					{
						type: "field",
						id: "shooting-play-inspector.shot.shape",
						label: "Shape",
						child: {
							type: "select",
							id: "shooting-play-inspector.shot.shape.select",
							value: shot.shape ?? "rectangle",
							items: [
								{ id: "rectangle", value: "rectangle", label: "Rectangle" },
								{ id: "ellipse", value: "ellipse", label: "Ellipse" },
							],
							onChange: shootingPlayCmd("patchShot", { shotId: shot.id, field: "shape" }),
						},
					},
					{
						type: "field",
						id: "shooting-play-inspector.shot.width",
						label: "Width",
						child: {
							type: "input",
							id: "shooting-play-inspector.shot.width.input",
							inputKind: "number",
							value: String(shot.width),
							onChange: shootingPlayCmd("patchShot", { shotId: shot.id, field: "width" }),
						},
					},
					{
						type: "field",
						id: "shooting-play-inspector.shot.height",
						label: "Height",
						child: {
							type: "input",
							id: "shooting-play-inspector.shot.height.input",
							inputKind: "number",
							value: String(shot.height),
							onChange: shootingPlayCmd("patchShot", { shotId: shot.id, field: "height" }),
						},
					},
					{
						type: "button",
						id: "shooting-play-inspector.shot.activate",
						label: "Set active shot",
						command: shootingPlayCmd("setActiveShot", { value: shot.id }),
					},
				],
			},
		] as readonly UiSectionNode[]);
	}
	const asset = fixture.assets.find((entry) => entry.id === selectedId);
	if (!asset) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "shooting-play-inspector.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Asset not found" }] },
		]);
	}
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "shooting-play-inspector.asset",
			label: asset.name || asset.id,
			children: [
				{
					type: "field",
					id: "shooting-play-inspector.asset.name",
					label: "Name",
					child: {
						type: "input",
						id: "shooting-play-inspector.asset.name.input",
						inputKind: "text",
						value: asset.name,
						onChange: shootingPlayCmd("patchAsset", { assetId: asset.id, field: "name" }),
					},
				},
				{ type: "field", id: "shooting-play-inspector.asset.url", label: "URL", child: { type: "text", value: asset.url } },
				{
					type: "button",
					id: "shooting-play-inspector.asset.activate",
					label: "Set active asset",
					command: shootingPlayCmd("setActiveAsset", { value: asset.id }),
				},
			],
		},
	] as readonly UiSectionNode[]);
}
// #endregion 🔖ShootingPlayPanels

function shootingFixtureJsonForId(fixtureId: string): string {
	if (isPlaygroundNoFixtureId(fixtureId)) return SHOOTING_PLAY_EMPTY_FIXTURE_JSON;
	if (fixtureId === SHOOTING_PLAY_FIXTURE_DEFAULT_ID) return shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE);
	return SHOOTING_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId] ?? SHOOTING_PLAY_EMPTY_FIXTURE_JSON;
}

export function buildShootingPlayToolbarTools(state: ShootingPlayToolbarState, controllerId: string): AppTools {
	return [
		toolCollection("open", "folder-open", [
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
		]),
		toolCollection("save", "save", [
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
		]),
		toolCollection("actions", "more-horizontal", [
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
		]),
	];
}

export class ShootingPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private activeFixtureId = playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID);
	private readonly docStore = new DocumentVcsStore<ShootingFixtureV1, JsonReplaceOp<ShootingFixtureV1>>({
		envelope: createDocumentVcsEnvelope(
			"shooting.fixture/v1",
			"shooting-play",
			parseShootingFixture(shootingFixtureJsonForId(playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID))) ?? {
				...DEFAULT_SHOOTING_FIXTURE,
				assets: [],
				shots: [],
			},
		),
		applyOp: applyJsonReplaceOp,
	});
	private readonly fixtureStore: ShootingPlayFixtureStore;
	private hostBridge: ShootingPlayHostBridge | null = null;
	private renderRevision = 0;
	private fitRevision = 0;
	private centerModel = true;
	private cameraDraftLabel = "Camera 1";
	private readonly snapshotListeners = new Set<() => void>();
	private selectedId: string | null = null;
	private selectedKind: ShootingPlaySelectionKind | null = null;
	private interactionRevision = 0;

	constructor(commandBus: CommandBus, hostNotify: () => void, fixtureStore: ShootingPlayFixtureStore = createShootingPlayFixtureStore()) {
		super(SHOOTING_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixtureStore = fixtureStore;
		this.rebuildShellMode();
	}

	getFixture(): ShootingFixtureV1 {
		return this.projection();
	}

	getDocumentVcsStore(): DocumentVcsStore<ShootingFixtureV1, JsonReplaceOp<ShootingFixtureV1>> {
		return this.docStore;
	}

	private projection(): ShootingFixtureV1 {
		return this.docStore.projection();
	}

	getFixtureJson(): string {
		return shootingFixtureToJson(this.projection());
	}

	getRenderRevision(): number {
		return this.renderRevision;
	}

	getCenterModel(): boolean {
		return this.centerModel;
	}

	getFitRevision(): number {
		return this.fitRevision;
	}

	getSelectedId(): string | null {
		return this.selectedId;
	}

	getSelectedKind(): ShootingPlaySelectionKind | null {
		return this.selectedKind;
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
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

	/** @emoji 🔔 Subscribes to fixture catalog updates for navbar fixture select refresh. */
	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => this.snapshotListeners.delete(listener);
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	private toolbarState(): ShootingPlayToolbarState {
		return (
			this.hostBridge?.getToolbarState() ?? {
				hasStoredFixture: this.hasStoredFixture(),
				activeShotId: this.projection().activeShotId ?? resolveActiveShot(this.projection())?.id ?? null,
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
		const previousAssetUrl = resolveActiveAsset(this.projection())?.url;
		recordJsonProjectionChange(this.docStore, fixture);
		this.renderRevision += 1;
		this.interactionRevision += 1;
		const nextAssetUrl = resolveActiveAsset(fixture)?.url;
		if (this.centerModel && previousAssetUrl !== nextAssetUrl) {
			this.fitRevision += 1;
		}
		this.notifySnapshot();
		this.rebuildShellMode();
		this.emit();
	}

	private applyFixtureJson(json: string): void {
		const parsed = parseShootingFixture(json);
		if (!parsed) return;
		this.applyFixture(parsed);
	}

	private loadFixtureById(fixtureId: string): void {
		const nextId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
		const nextJson = shootingFixtureJsonForId(nextId);
		if (nextId === this.activeFixtureId && nextJson === this.getFixtureJson()) return;
		this.activeFixtureId = nextId;
		this.applyFixtureJson(nextJson);
	}

	private viewportMeasures(): readonly WindowMeasure[] {
		return [
			{
				kind: "group",
				id: "shooting-viewport",
				label: "Viewport",
				defaultOpen: true,
				children: [
					{
						kind: "toggle",
						id: "shooting-center-model",
						iconId: "focus",
						label: "Center Model",
						pressed: this.centerModel,
						onChange: shootingPlayCmd("setCenterModel"),
					},
				],
			},
		];
	}

	private modelMeasures(): readonly WindowMeasure[] {
		const scene = this.projection().scene;
		return [
			...this.viewportMeasures(),
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
				iconId: "sun",
				label: "Shadow",
				pressed: scene.shadow.enabled,
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
		const activeShot = resolveActiveShot(this.projection());
		return [
			{
				kind: "select",
				id: "shooting-active-shot",
				label: "Shot",
				value: activeShot?.id ?? "",
				items: this.projection().shots.map((shot) => ({ id: shot.id, value: shot.id, label: shot.label })),
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
			{
				kind: "select",
				id: "shooting-shot-shape",
				label: "Shape",
				value: activeShot ? (activeShot.shape ?? "rectangle") : "rectangle",
				items: [
					{ id: "rectangle", value: "rectangle", label: "Rectangle" },
					{ id: "ellipse", value: "ellipse", label: "Ellipse" },
				],
				onChange: shootingPlayCmd("setActiveShotShape"),
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
			possibleEngagements: this.projection().savedCameras.map((entry) => ({
				id: `shooting.camera.${entry.id}`,
				label: entry.label,
				command: shootingPlayCmd("loadSavedCamera", { id: entry.id }),
			})),
			status: [{ id: "shooting-asset-count", text: `${this.projection().assets.length} assets · ${this.projection().shots.length} shots` }],
		};
	}

	private iconEngagement(): WindowEngagement {
		const shot = resolveActiveShot(this.projection());
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
		this.applyFixture({ ...this.projection(), scene: { ...this.projection().scene, ...patch } });
	}

	private patchShot(shotId: string, field: string, value: unknown): void {
		const shots = this.projection().shots.map((shot) => {
			if (shot.id !== shotId) return shot;
			if (field === "width" || field === "height") {
				const numeric = typeof value === "number" ? value : Number(value);
				if (!Number.isFinite(numeric)) return shot;
				return { ...shot, [field]: Math.round(numeric) };
			}
			if (field === "format" && (value === "svg" || value === "png")) {
				return { ...shot, format: value };
			}
			if (field === "shape" && (value === "rectangle" || value === "ellipse")) {
				return { ...shot, shape: value };
			}
			if (typeof value !== "string") return shot;
			return { ...shot, [field]: value };
		});
		this.applyFixture({ ...this.projection(), shots });
	}

	private patchAsset(assetId: string, field: string, value: unknown): void {
		const assets = this.projection().assets.map((asset) => {
			if (asset.id !== assetId) return asset;
			if (typeof value !== "string") return asset;
			return { ...asset, [field]: value };
		});
		this.applyFixture({ ...this.projection(), assets });
	}

	override run(command: string, args?: unknown): void {
		if (command === "setSelection") {
			const selectedId = (args as { selectedId?: string | null }).selectedId ?? null;
			const selectedKind = (args as { selectedKind?: ShootingPlaySelectionKind | null }).selectedKind ?? null;
			if (this.selectedId === selectedId && this.selectedKind === selectedKind) return;
			this.selectedId = selectedId;
			this.selectedKind = selectedKind;
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "patchShot") {
			const shotId = (args as { shotId?: string }).shotId;
			const field = (args as { field?: string }).field;
			const value = (args as { value?: unknown }).value;
			if (typeof shotId === "string" && typeof field === "string") {
				this.patchShot(shotId, field, value);
			}
			return;
		}
		if (command === "patchAsset") {
			const assetId = (args as { assetId?: string }).assetId;
			const field = (args as { field?: string }).field;
			const value = (args as { value?: unknown }).value;
			if (typeof assetId === "string" && typeof field === "string") {
				this.patchAsset(assetId, field, value);
			}
			return;
		}
		if (command === "setActiveAsset") {
			const value = (args as { value?: string }).value ?? (args as { id?: string }).id;
			if (typeof value !== "string" || !value) return;
			this.applyFixture({ ...this.projection(), activeAssetId: value });
			return;
		}
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") this.applyFixtureJson(json);
			return;
		}
		if (command === "setActiveFixture") {
			if (isPlaygroundFixtureLocked()) return;
			const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
			this.loadFixtureById(fixtureId);
			return;
		}
		if (command === "setCamera") {
			const camera = (args as { camera?: ShootingCameraV1 }).camera;
			if (!camera) return;
			this.applyFixture({ ...this.projection(), camera });
			return;
		}
		if (command === "setShotCamera") {
			const camera = (args as { camera?: ShootingCameraV1 }).camera;
			const shotId = (args as { shotId?: string }).shotId;
			if (!camera) return;
			const shot = shotId ? this.projection().shots.find((entry) => entry.id === shotId) : resolveActiveShot(this.projection());
			if (!shot) return;
			this.applyFixture(applyShootingCameraToFixture(this.projection(), shot, camera));
			return;
		}
		if (command === "setCenterModel") {
			const pressed = (args as { pressed?: boolean }).pressed;
			if (typeof pressed !== "boolean") return;
			this.centerModel = pressed;
			if (pressed) this.fitRevision += 1;
			this.rebuildShellMode();
			this.emit();
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
				...this.projection(),
				savedCameras: [...this.projection().savedCameras, { id, label, camera: this.projection().camera }],
			});
			console.log(`[DEBUG] shooting saved camera ${id} ${label}`);
			return;
		}
		if (command === "loadSavedCamera") {
			const id = (args as { id?: string }).id;
			const saved = this.projection().savedCameras.find((entry) => entry.id === id);
			if (!saved) return;
			this.applyFixture({ ...this.projection(), camera: saved.camera });
			console.log(`[DEBUG] shooting loaded camera ${saved.id}`);
			return;
		}
		if (command === "loadCameraMenu") {
			const first = this.projection().savedCameras[0];
			if (first) this.run("loadSavedCamera", { id: first.id });
			return;
		}
		if (command === "setActiveShot") {
			const value = (args as { value?: string }).value ?? (args as { id?: string }).id;
			if (typeof value !== "string" || !value) return;
			this.applyFixture({ ...this.projection(), activeShotId: value });
			return;
		}
		if (command === "setActiveShotFormat") {
			const value = (args as { value?: string }).value;
			if (value !== "svg" && value !== "png") return;
			const active = resolveActiveShot(this.projection());
			if (!active) return;
			const shots = this.projection().shots.map((shot) => (shot.id === active.id ? { ...shot, format: value } : shot));
			this.applyFixture({ ...this.projection(), shots });
			return;
		}
		if (command === "setActiveShotShape") {
			const value = (args as { value?: string }).value;
			if (value !== "rectangle" && value !== "ellipse") return;
			const active = resolveActiveShot(this.projection());
			if (!active) return;
			const shots = this.projection().shots.map((shot) => (shot.id === active.id ? { ...shot, shape: value } : shot));
			this.applyFixture({ ...this.projection(), shots });
			return;
		}
		if (command === "setActiveShotLabel") {
			const value = (args as { value?: string }).value;
			if (typeof value !== "string") return;
			const active = resolveActiveShot(this.projection());
			if (!active) return;
			const shots = this.projection().shots.map((shot) => (shot.id === active.id ? { ...shot, label: value } : shot));
			this.applyFixture({ ...this.projection(), shots });
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
			this.patchScene({ sun: { ...this.projection().scene.sun, azimuth: value } });
			return;
		}
		if (command === "setSunElevation") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ sun: { ...this.projection().scene.sun, elevation: value } });
			return;
		}
		if (command === "setSunIntensity") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ sun: { ...this.projection().scene.sun, intensity: value } });
			return;
		}
		if (command === "setAmbientIntensity") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ ambient: { ...this.projection().scene.ambient, intensity: value } });
			return;
		}
		if (command === "setShadowEnabled") {
			const value = (args as { value?: boolean; pressed?: boolean }).value ?? (args as { pressed?: boolean }).pressed;
			if (typeof value !== "boolean") return;
			this.patchScene({ shadow: { ...this.projection().scene.shadow, enabled: value } });
			return;
		}
		if (command === "setMaterialRoughness") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			this.patchScene({ material: { ...this.projection().scene.material, roughness: value } });
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
				...this.projection(),
				assets: [...this.projection().assets, asset],
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
	return createPlayAppRuntime(SHOOTING_PLAY_APP_ID, "Shooting", controller, SHOOTING_PLAY_LAYOUT, controller.mainMode);
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

		it("setActiveFixture loads file fixtures and updates catalog", () => {
			const bus = new CommandBus();
			const ctrl = new ShootingPlayController(bus, () => {});
			ctrl.run("setActiveFixture", { fixtureId: "base-icon" });
			expect(ctrl.getFixture().assets).toHaveLength(1);
			expect(ctrl.getFixture().shots).toHaveLength(2);
			expect(ctrl.getFixtureCatalog()?.activeFixtureId).toBe("base-icon");
		});

		it("setShotCamera updates active shot camera", () => {
			const bus = new CommandBus();
			const ctrl = new ShootingPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE) });
			ctrl.run("setShotCamera", { shotId: "overview-svg", camera: { ...DEFAULT_SHOOTING_FIXTURE.camera, zoom: 3 } });
			expect(ctrl.getFixture().camera.zoom).toBe(3);
		});

		it("setCenterModel toggles centering and bumps fit revision when enabled", () => {
			const bus = new CommandBus();
			const ctrl = new ShootingPlayController(bus, () => {});
			expect(ctrl.getCenterModel()).toBe(true);
			const initialFit = ctrl.getFitRevision();
			ctrl.run("setCenterModel", { pressed: false });
			expect(ctrl.getCenterModel()).toBe(false);
			expect(ctrl.getFitRevision()).toBe(initialFit);
			ctrl.run("setCenterModel", { pressed: true });
			expect(ctrl.getCenterModel()).toBe(true);
			expect(ctrl.getFitRevision()).toBe(initialFit + 1);
		});

		it("setActiveShotShape updates active shot shape", () => {
			const bus = new CommandBus();
			const ctrl = new ShootingPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE) });
			ctrl.run("setActiveShotShape", { value: "ellipse" });
			expect(ctrl.getFixture().shots[0]?.shape).toBe("ellipse");
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
