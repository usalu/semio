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
	UI_INSPECTOR_MIXED_PLACEHOLDER,
	uiDeclarativeSectionsToTree,
	uiInspectorGroupsToTree,
	uiInspectorMixedNumber,
	uiInspectorMixedSelect,
	uiInspectorMixedText,
	uiInspectorReadonlyField,
	type AppTools,
	type CommandDescriptor,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	type ToolLeaf,
	type UiInspectorFieldGroup,
	toolCollection,
	type UiNode,
	type UiTreeItemNode,
	type WindowEngagement,
	type WindowMeasure,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
	DocumentVcsStore,
	createDocumentVcsEnvelope,
	recordProjectionChange,
} from "@semio-tech/framework-core";
import {
	DEFAULT_SHOOTING_FIXTURE,
	parseShootingFixture,
	resolveActiveShot,
	resolveActiveAsset,
	shootingFixtureToJson,
	applyShootingFixtureEditOp,
	type ShootingCameraV1,
	type ShootingFixtureEditOp,
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

function shootingPlaySelectionFromTreeRowIds(rawIds: readonly string[]): { shotIds: string[]; assetIds: string[] } {
	const shotIds: string[] = [];
	const assetIds: string[] = [];
	for (const rawId of rawIds) {
		const shotPrefix = "shooting-play-hierarchy.shot.";
		const assetPrefix = "shooting-play-hierarchy.asset.";
		if (rawId.startsWith(shotPrefix)) {
			shotIds.push(rawId.slice(shotPrefix.length));
		} else if (rawId.startsWith(assetPrefix)) {
			assetIds.push(rawId.slice(assetPrefix.length));
		}
	}
	return { shotIds, assetIds };
}

// #region 🔖ShootingPlayPanels
export function buildShootingPlayHierarchyTree(
	fixture: ShootingFixtureV1,
	selectedShotIds: readonly string[],
	selectedAssetIds: readonly string[],
): UiNode {
	const shotItems: UiTreeItemNode[] = fixture.shots.map((shot) => ({
		id: `shooting-play-hierarchy.shot.${shot.id}`,
		label: shot.label || shot.id,
		description: `${shot.width}×${shot.height} ${shot.format.toUpperCase()}`,
		command: shootingPlayCmd("setSelection", { shotIds: [shot.id], assetIds: [] }),
	}));
	const assetItems: UiTreeItemNode[] = fixture.assets.map((asset) => ({
		id: `shooting-play-hierarchy.asset.${asset.id}`,
		label: asset.name || asset.id,
		description: asset.format,
		command: shootingPlayCmd("setSelection", { assetIds: [asset.id], shotIds: [] }),
	}));
	const selectedIds = [
		...selectedShotIds.map((id) => `shooting-play-hierarchy.shot.${id}`),
		...selectedAssetIds.map((id) => `shooting-play-hierarchy.asset.${id}`),
	];
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
		selectionChange: shootingPlayCmd("setSelection"),
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

function shootingPlayInspectorPatchShots(shotIds: readonly string[], field: string) {
	return shootingPlayCmd("patchShots", { shotIds, field });
}

function shootingPlayInspectorPatchAssets(assetIds: readonly string[], field: string) {
	return shootingPlayCmd("patchAssets", { assetIds, field });
}

function shootingPlayInspectorNumberField(
	shotIds: readonly string[],
	fieldId: string,
	label: string,
	values: readonly number[],
	field: string,
): UiNode {
	const mixed = uiInspectorMixedNumber(values);
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "number",
			value: mixed.uniform ? String(mixed.value) : "",
			placeholder: mixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER,
			onChange: shootingPlayInspectorPatchShots(shotIds, field),
		},
	};
}

function shootingPlayInspectorTextField(
	ids: readonly string[],
	fieldId: string,
	label: string,
	values: readonly string[],
	field: string,
	patch: (ids: readonly string[], field: string) => CommandDescriptor,
): UiNode {
	const mixed = uiInspectorMixedText(values);
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "text",
			value: mixed.value,
			placeholder: mixed.placeholder,
			onChange: patch(ids, field),
		},
	};
}

function shootingPlayInspectorShotGroup(shots: readonly ShootingShotV1[]): UiInspectorFieldGroup {
	const shotIds = shots.map((entry) => entry.id);
	const formatMixed = uiInspectorMixedSelect(shots.map((entry) => entry.format));
	const shapeMixed = uiInspectorMixedSelect(shots.map((entry) => entry.shape ?? "rectangle"));
	return {
		id: "shooting-play-inspector.shots",
		label: shots.length === 1 ? "Shot" : "Shots",
		fields: [
			shootingPlayInspectorTextField(shotIds, "shooting-play-inspector.shot.label", "Label", shots.map((entry) => entry.label), "label", shootingPlayInspectorPatchShots),
			{
				type: "field",
				id: "shooting-play-inspector.shot.format",
				label: "Format",
				child: {
					type: "select",
					id: "shooting-play-inspector.shot.format.select",
					value: formatMixed.value,
					placeholder: formatMixed.placeholder,
					items: [
						{ id: "svg", value: "svg", label: "SVG" },
						{ id: "png", value: "png", label: "PNG" },
					],
					onChange: shootingPlayInspectorPatchShots(shotIds, "format"),
				},
			},
			{
				type: "field",
				id: "shooting-play-inspector.shot.shape",
				label: "Shape",
				child: {
					type: "select",
					id: "shooting-play-inspector.shot.shape.select",
					value: shapeMixed.value,
					placeholder: shapeMixed.placeholder,
					items: [
						{ id: "rectangle", value: "rectangle", label: "Rectangle" },
						{ id: "ellipse", value: "ellipse", label: "Ellipse" },
					],
					onChange: shootingPlayInspectorPatchShots(shotIds, "shape"),
				},
			},
			shootingPlayInspectorNumberField(shotIds, "shooting-play-inspector.shot.width", "Width", shots.map((entry) => entry.width), "width"),
			shootingPlayInspectorNumberField(shotIds, "shooting-play-inspector.shot.height", "Height", shots.map((entry) => entry.height), "height"),
			...(shotIds.length === 1
				? [
						{
							type: "button" as const,
							id: "shooting-play-inspector.shot.activate",
							label: "Set active shot",
							command: shootingPlayCmd("setActiveShot", { value: shotIds[0] }),
						},
					]
				: []),
		],
	};
}

function shootingPlayInspectorAssetGroup(
	assets: readonly { readonly id: string; readonly name: string; readonly url: string; readonly format: string }[],
): UiInspectorFieldGroup {
	const assetIds = assets.map((entry) => entry.id);
	return {
		id: "shooting-play-inspector.assets",
		label: assets.length === 1 ? "Asset" : "Assets",
		fields: [
			shootingPlayInspectorTextField(assetIds, "shooting-play-inspector.asset.name", "Name", assets.map((entry) => entry.name), "name", shootingPlayInspectorPatchAssets),
			uiInspectorReadonlyField(
				"shooting-play-inspector.asset.url",
				"URL",
				assets.length === 1 ? (assets[0]?.url ?? "") : UI_INSPECTOR_MIXED_PLACEHOLDER,
			),
			...(assetIds.length === 1
				? [
						{
							type: "button" as const,
							id: "shooting-play-inspector.asset.activate",
							label: "Set active asset",
							command: shootingPlayCmd("setActiveAsset", { value: assetIds[0] }),
						},
					]
				: []),
		],
	};
}

export function buildShootingPlayInspectorTree(
	fixture: ShootingFixtureV1,
	selectedShotIds: readonly string[],
	selectedAssetIds: readonly string[],
): UiNode {
	if (selectedShotIds.length === 0 && selectedAssetIds.length === 0) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "shooting-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a shot or asset in the hierarchy." }],
			},
		]);
	}
	const groups: UiInspectorFieldGroup[] = [];
	if (selectedShotIds.length > 0) {
		const shots = selectedShotIds
			.map((id) => fixture.shots.find((entry) => entry.id === id))
			.filter((entry): entry is ShootingShotV1 => Boolean(entry));
		if (shots.length > 0) {
			groups.push(shootingPlayInspectorShotGroup(shots));
		}
	}
	if (selectedAssetIds.length > 0) {
		const assets = selectedAssetIds
			.map((id) => fixture.assets.find((entry) => entry.id === id))
			.filter((entry): entry is ShootingFixtureV1["assets"][number] => Boolean(entry));
		if (assets.length > 0) {
			groups.push(shootingPlayInspectorAssetGroup(assets));
		}
	}
	if (!groups.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "shooting-play-inspector.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Selection not found" }] },
		]);
	}
	return uiInspectorGroupsToTree(groups);
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
	private readonly docStore = new DocumentVcsStore<ShootingFixtureV1, ShootingFixtureEditOp>({
		envelope: createDocumentVcsEnvelope(
			"shooting.fixture/v1",
			"shooting-play",
			parseShootingFixture(shootingFixtureJsonForId(playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID))) ?? {
				...DEFAULT_SHOOTING_FIXTURE,
				assets: [],
				shots: [],
			},
		),
		applyOp: applyShootingFixtureEditOp,
	});
	private readonly fixtureStore: ShootingPlayFixtureStore;
	private hostBridge: ShootingPlayHostBridge | null = null;
	private renderRevision = 0;
	private fitRevision = 0;
	private centerModel = true;
	private cameraDraftLabel = "Camera 1";
	private readonly snapshotListeners = new Set<() => void>();
	private selectedShotIds: string[] = [];
	private selectedAssetIds: string[] = [];
	private interactionRevision = 0;

	constructor(commandBus: CommandBus, hostNotify: () => void, fixtureStore: ShootingPlayFixtureStore = createShootingPlayFixtureStore()) {
		super(SHOOTING_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixtureStore = fixtureStore;
		this.rebuildShellMode();
	}

	getFixture(): ShootingFixtureV1 {
		return this.projection();
	}

	getDocumentVcsStore(): DocumentVcsStore<ShootingFixtureV1, ShootingFixtureEditOp> {
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

	getSelectedShotIds(): readonly string[] {
		return this.selectedShotIds;
	}

	getSelectedAssetIds(): readonly string[] {
		return this.selectedAssetIds;
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
		this.mainMode.tools = buildShootingPlayToolbarTools(this.toolbarState(), this.id);
	}

	private applyFixtureEdit(op: ShootingFixtureEditOp): void {
		const previous = this.projection();
		const previousAssetUrl = resolveActiveAsset(previous)?.url;
		recordProjectionChange(this.docStore, [op]);
		const fixture = this.projection();
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
		this.applyFixtureEdit({ op: "setDocument", document: parsed });
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
		this.applyFixtureEdit({ op: "patchScene", patch });
	}

	private patchShots(shotIds: readonly string[], field: string, value: unknown): void {
		if (!shotIds.length) return;
		this.applyFixtureEdit({ op: "patchShots", shotIds, field, value });
	}

	private patchShot(shotId: string, field: string, value: unknown): void {
		this.patchShots([shotId], field, value);
	}

	private patchAssets(assetIds: readonly string[], field: string, value: unknown): void {
		if (!assetIds.length) return;
		this.applyFixtureEdit({ op: "patchAssets", assetIds, field, value });
	}

	private patchAsset(assetId: string, field: string, value: unknown): void {
		this.patchAssets([assetId], field, value);
	}

	override run(command: string, args?: unknown): void {
		if (command === "setSelection") {
			const payload = args as {
				shotIds?: readonly string[];
				assetIds?: readonly string[];
				ids?: readonly string[];
				mode?: "default" | "additive";
				selectedId?: string | null;
				selectedKind?: ShootingPlaySelectionKind | null;
			};
			let nextShotIds = this.selectedShotIds;
			let nextAssetIds = this.selectedAssetIds;
			if (Array.isArray(payload.ids)) {
				const resolved = shootingPlaySelectionFromTreeRowIds(payload.ids.map(String));
				nextShotIds = resolved.shotIds;
				nextAssetIds = resolved.assetIds;
			} else {
				const mode = payload.mode ?? "default";
				const incomingShots =
					payload.shotIds ??
					(payload.selectedKind === "shot" && typeof payload.selectedId === "string" ? [payload.selectedId] : []);
				const incomingAssets =
					payload.assetIds ??
					(payload.selectedKind === "asset" && typeof payload.selectedId === "string" ? [payload.selectedId] : []);
				if (mode === "additive") {
					nextShotIds = [...new Set([...this.selectedShotIds, ...incomingShots])];
					nextAssetIds = [...new Set([...this.selectedAssetIds, ...incomingAssets])];
				} else {
					nextShotIds = [...incomingShots];
					nextAssetIds = [...incomingAssets];
				}
			}
			if (
				nextShotIds.length === this.selectedShotIds.length &&
				nextAssetIds.length === this.selectedAssetIds.length &&
				nextShotIds.every((id, index) => id === this.selectedShotIds[index]) &&
				nextAssetIds.every((id, index) => id === this.selectedAssetIds[index])
			) {
				return;
			}
			this.selectedShotIds = nextShotIds;
			this.selectedAssetIds = nextAssetIds;
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "patchShots") {
			const shotIds = (args as { shotIds?: readonly string[] }).shotIds ?? [];
			const field = (args as { field?: string }).field;
			const value = (args as { value?: unknown }).value;
			if (shotIds.length > 0 && typeof field === "string") {
				this.patchShots(shotIds, field, value);
			}
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
		if (command === "patchAssets") {
			const assetIds = (args as { assetIds?: readonly string[] }).assetIds ?? [];
			const field = (args as { field?: string }).field;
			const value = (args as { value?: unknown }).value;
			if (assetIds.length > 0 && typeof field === "string") {
				this.patchAssets(assetIds, field, value);
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
			this.applyFixtureEdit({ op: "setActiveAsset", assetId: value });
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
			this.applyFixtureEdit({ op: "setCamera", camera });
			return;
		}
		if (command === "setShotCamera") {
			const camera = (args as { camera?: ShootingCameraV1 }).camera;
			const shotId = (args as { shotId?: string }).shotId;
			if (!camera) return;
			const shot = shotId ? this.projection().shots.find((entry) => entry.id === shotId) : resolveActiveShot(this.projection());
			if (!shot) return;
			this.applyFixtureEdit({ op: "setShotCamera", shotId: shot.id, camera });
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
			this.applyFixtureEdit({
				op: "addSavedCamera",
				entry: { id, label, camera: this.projection().camera },
			});
			console.log(`[DEBUG] shooting saved camera ${id} ${label}`);
			return;
		}
		if (command === "loadSavedCamera") {
			const id = (args as { id?: string }).id;
			if (typeof id !== "string") return;
			this.applyFixtureEdit({ op: "loadSavedCamera", cameraId: id });
			console.log(`[DEBUG] shooting loaded camera ${id}`);
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
			this.applyFixtureEdit({ op: "setActiveShot", shotId: value });
			return;
		}
		if (command === "setActiveShotFormat") {
			const value = (args as { value?: string }).value;
			if (value !== "svg" && value !== "png") return;
			const active = resolveActiveShot(this.projection());
			if (!active) return;
			this.patchShot(active.id, "format", value);
			return;
		}
		if (command === "setActiveShotShape") {
			const value = (args as { value?: string }).value;
			if (value !== "rectangle" && value !== "ellipse") return;
			const active = resolveActiveShot(this.projection());
			if (!active) return;
			this.patchShot(active.id, "shape", value);
			return;
		}
		if (command === "setActiveShotLabel") {
			const value = (args as { value?: string }).value;
			if (typeof value !== "string") return;
			const active = resolveActiveShot(this.projection());
			if (!active) return;
			this.patchShot(active.id, "label", value);
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
			this.applyFixtureEdit({ op: "importAsset", asset, setActive: true });
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

		it("setSelection resolves hierarchy row ids for multi-select", () => {
			const bus = new CommandBus();
			const ctrl = new ShootingPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE) });
			const shotIds = ctrl.getFixture().shots.slice(0, 2).map((shot) => shot.id);
			ctrl.run("setSelection", {
				ids: shotIds.map((id) => `shooting-play-hierarchy.shot.${id}`),
			});
			expect(ctrl.getSelectedShotIds()).toEqual(shotIds);
			expect(ctrl.getSelectedAssetIds()).toEqual([]);
		});

		it("buildShootingPlayInspectorTree batches shot label edits", () => {
			const fixture = DEFAULT_SHOOTING_FIXTURE;
			const shotIds = fixture.shots.slice(0, 2).map((shot) => shot.id);
			const tree = buildShootingPlayInspectorTree(fixture, shotIds, []);
			const shotSection = tree.sections.find((section) => section.id === "shooting-play-inspector.shots");
			const labelField = shotSection?.items.find((item) => item.id === "shooting-play-inspector.shot.label");
			expect(labelField?.control?.onChange?.command).toBe("patchShots");
			expect(labelField?.control?.onChange?.args).toMatchObject({ shotIds, field: "label" });
		});

		it("patchShots updates every selected shot", () => {
			const bus = new CommandBus();
			const ctrl = new ShootingPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE) });
			const shotIds = ctrl.getFixture().shots.map((shot) => shot.id);
			ctrl.run("patchShots", { shotIds, field: "label", value: "batch-shot" });
			for (const shot of ctrl.getFixture().shots) {
				expect(shot.label).toBe("batch-shot");
			}
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
