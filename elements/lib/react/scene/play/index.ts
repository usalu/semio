// #region 🧲Header
// 💻 elements/lib/react/scene/play/index.ts — Scene play on `@elements/playground`: Nakagin fixture, LOD measures, selection/filter tools (no React).
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
	buildPlaygroundBrowseFilterTools,
	buildPlaygroundBrowseSelectionTools,
	type AppTools,
	type ToolItem,
	type WindowMeasure,
	type UiNode,
} from "@elements/playground";

import nakaginSceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";
import {
	applyRelocateToSceneFixture,
	DEFAULT_MANUAL_LOD,
	SCENE_LOD_SLIDER_MAX,
	SCENE_LOD_SLIDER_MIN,
	formatSceneLod,
	lodFromSliderValue,
	parseFixtureV1,
	parseVortexFullId,
	fixturePoseFingerprint,
	fixtureStateFingerprint,
	sceneLodCanvasProps,
	sliderValueFromLod,
	type AttractionProps,
	type CameraState,
	type FixtureObjectV1,
	type FixtureV1,
	type KindCatalogBundle,
	type KindCompatEntry,
	type RelocateMode,
	type RelocatePayload,
	type SelectionMode,
	type SelectionSnapshot,
	type VortexProps,
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
export const SCENE_PLAY_INSPECTOR_TAB_ID = "scene-play-inspector";
export const SCENE_PLAY_SETTINGS_TAB_ID = "scene-play-settings";
//#endregion 🎬Play

export { parseKindCatalogs, parseKindCompatibility };

//#region 🔖ScenePlaySelection
/** @emoji 🎯 Play harness selection: objects, vortex full ids, and attractions. */
export interface ScenePlaySelection extends SelectionSnapshot {
	readonly attractionIds: readonly string[];
}

export const SCENE_PLAY_EMPTY_SELECTION: ScenePlaySelection = {
	objectIds: [],
	vortexIds: [],
	attractionIds: [],
};

/** @emoji 🔗 Canonical `objectId:vortexId` for fixture vortex rows. */
export function sceneVortexFullId(objectId: string, vortexId: string): string {
	return vortexId.includes(":") ? vortexId : `${objectId}:${vortexId}`;
}

/** @emoji 🗑️ Removes an object and any attractions touching it or its vortices. */
export function deleteSceneObjectFromFixture(fixture: FixtureV1, objectId: string): FixtureV1 {
	const removedVortexFullIds = new Set<string>();
	for (const object of fixture.objects) {
		if (object.id !== objectId) {
			continue;
		}
		for (const vortex of object.vortices) {
			removedVortexFullIds.add(sceneVortexFullId(objectId, vortex.id));
		}
	}
	return {
		...fixture,
		objects: fixture.objects.filter((object) => object.id !== objectId),
		attractions: fixture.attractions.filter((attraction) => {
			const sourceObjectId = parseVortexFullId(attraction.attracting).objectId;
			const targetObjectId = parseVortexFullId(attraction.attracted).objectId;
			if (sourceObjectId === objectId || targetObjectId === objectId) {
				return false;
			}
			return !removedVortexFullIds.has(attraction.attracting) && !removedVortexFullIds.has(attraction.attracted);
		}),
	};
}

/** @emoji 🗑️ Removes one vortex and stale attractions that referenced it. */
export function deleteSceneVortexFromFixture(fixture: FixtureV1, vortexFullId: string): FixtureV1 {
	const { objectId } = parseVortexFullId(vortexFullId);
	return {
		...fixture,
		objects: fixture.objects.map((object) =>
			object.id !== objectId
				? object
				: {
						...object,
						vortices: object.vortices.filter((vortex) => sceneVortexFullId(objectId, vortex.id) !== vortexFullId),
					},
		),
		attractions: fixture.attractions.filter(
			(attraction) => attraction.attracting !== vortexFullId && attraction.attracted !== vortexFullId,
		),
	};
}

/** @emoji 🗑️ Drops a single attraction row. */
export function deleteSceneAttractionFromFixture(fixture: FixtureV1, attractionId: string): FixtureV1 {
	return {
		...fixture,
		attractions: fixture.attractions.filter((attraction) => attraction.id !== attractionId),
	};
}

function patchSceneObject(
	objects: readonly FixtureObjectV1[],
	objectId: string,
	patch: (object: FixtureObjectV1) => FixtureObjectV1,
): FixtureObjectV1[] {
	return objects.map((object) => (object.id === objectId ? patch(object) : object));
}

/** @emoji ✏️ Updates fields on one fixture object. */
export function updateSceneObjectInFixture(
	fixture: FixtureV1,
	objectId: string,
	patch: Partial<Omit<FixtureObjectV1, "id" | "vortices">>,
): FixtureV1 {
	return {
		...fixture,
		objects: patchSceneObject(fixture.objects, objectId, (object) => ({ ...object, ...patch })),
	};
}

/** @emoji ✏️ Updates one vortex on an object. */
export function updateSceneVortexInFixture(
	fixture: FixtureV1,
	vortexFullId: string,
	patch: Partial<VortexProps>,
): FixtureV1 {
	const { objectId, vortexId } = parseVortexFullId(vortexFullId);
	return {
		...fixture,
		objects: patchSceneObject(fixture.objects, objectId, (object) => ({
			...object,
			vortices: object.vortices.map((vortex) => {
				const fullId = sceneVortexFullId(objectId, vortex.id);
				if (fullId !== vortexFullId && vortex.id !== vortexId) {
					return vortex;
				}
				return { ...vortex, ...patch, id: vortex.id };
			}),
		})),
	};
}

/** @emoji ✏️ Updates one attraction row. */
export function updateSceneAttractionInFixture(
	fixture: FixtureV1,
	attractionId: string,
	patch: Partial<AttractionProps>,
): FixtureV1 {
	return {
		...fixture,
		attractions: fixture.attractions.map((attraction) =>
			attraction.id === attractionId ? { ...attraction, ...patch } : attraction,
		),
	};
}

/** @emoji 📷 True when two camera states match within epsilon (avoids redundant fixture writes). */
export function cameraStateNearEqual(a: CameraState, b: CameraState, epsilon = 1e-3): boolean {
	for (let i = 0; i < 3; i += 1) {
		if (Math.abs(a.position[i]! - b.position[i]!) > epsilon) return false;
		if (Math.abs(a.target[i]! - b.target[i]!) > epsilon) return false;
	}
	return Math.abs(a.zoom - b.zoom) <= epsilon;
}

/** @emoji 📷 Writes camera fields on the fixture; returns the same reference when unchanged. */
export function updateSceneCameraInFixture(fixture: FixtureV1, camera: Partial<CameraState>): FixtureV1 {
	const nextCamera: CameraState = { ...fixture.camera, ...camera };
	if (cameraStateNearEqual(fixture.camera, nextCamera)) {
		return fixture;
	}
	return { ...fixture, camera: nextCamera };
}

/** @emoji 🎯 Maps {@link SelectionSnapshot} to play selection (attractions filled separately). */
export function selectionSnapshotToPlaySelection(
	snap: SelectionSnapshot,
	attractionIds: readonly string[] = [],
): ScenePlaySelection {
	return {
		objectIds: snap.objectIds,
		vortexIds: snap.vortexIds,
		attractionIds,
	};
}

/** @emoji 🎯 True when two selection snapshots match (skips redundant shell updates). */
export function scenePlaySelectionEqual(a: ScenePlaySelection, b: ScenePlaySelection): boolean {
	if (a.objectIds.length !== b.objectIds.length || a.vortexIds.length !== b.vortexIds.length) {
		return false;
	}
	if (a.attractionIds.length !== b.attractionIds.length) {
		return false;
	}
	for (let i = 0; i < a.objectIds.length; i += 1) {
		if (a.objectIds[i] !== b.objectIds[i]) {
			return false;
		}
	}
	for (let i = 0; i < a.vortexIds.length; i += 1) {
		if (a.vortexIds[i] !== b.vortexIds[i]) {
			return false;
		}
	}
	for (let i = 0; i < a.attractionIds.length; i += 1) {
		if (a.attractionIds[i] !== b.attractionIds[i]) {
			return false;
		}
	}
	return true;
}

/** @emoji 🎯 Primary object id for relocate / legacy e2e hooks. */
export function primaryScenePlayObjectId(selection: ScenePlaySelection): string | null {
	if (selection.objectIds[0]) {
		return selection.objectIds[0];
	}
	if (selection.vortexIds[0]) {
		return parseVortexFullId(selection.vortexIds[0]).objectId;
	}
	return null;
}
//#endregion 🔖ScenePlaySelection

//#region 🔖ScenePlayController
const SCENE_PLAY_KINDS = ["object", "vortex", "attraction"] as const;
type ScenePlayPickKind = (typeof SCENE_PLAY_KINDS)[number];

function scenePlayKindLabel(kind: ScenePlayPickKind): string {
	if (kind === "object") return "Objects";
	if (kind === "vortex") return "Vortices";
	return "Attractions";
}

/** @emoji 🎬 Playground scene play controller: fixture, LOD, selection/filter tools, and interaction counters. */
export class ScenePlayShellController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Scene", undefined);
	readonly selectableKinds: Record<ScenePlayPickKind, boolean> = { object: true, vortex: true, attraction: true };
	readonly visibleKinds: Record<ScenePlayPickKind, boolean> = { object: true, vortex: true, attraction: true };
	private fixture: FixtureV1 | null;
	private fixtureRevision: number;
	private automaticLod: boolean;
	private depthVariableLod: boolean;
	private manualLod: number;
	private lodSlider: number;
	private lodTag: number;
	private relocateMode: RelocateMode;
	private selection: ScenePlaySelection;
	private selectionMode: SelectionMode;
	private proximityRadius: number;
	private chunkSize: number;
	private gridFactor: number;
	private showLodGrid: boolean;
	private gridSnapEnabled: boolean;
	private proximityCount: number;
	private connectCount: number;
	private indirectCount: number;
	private compatibleObjectsCount: number;
	private targetRingCount: number;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SCENE_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixture = parseFixtureV1(nakaginSceneFixtureJson as unknown);
		this.fixtureRevision = 0;
		this.automaticLod = true;
		this.depthVariableLod = false;
		this.manualLod = DEFAULT_MANUAL_LOD;
		this.lodSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
		this.lodTag = DEFAULT_MANUAL_LOD;
		this.relocateMode = "translate";
		this.selection = SCENE_PLAY_EMPTY_SELECTION;
		this.selectionMode = "single";
		this.proximityRadius = 24;
		this.chunkSize = 256;
		this.gridFactor = 10;
		this.showLodGrid = false;
		this.gridSnapEnabled = true;
		this.proximityCount = 0;
		this.connectCount = 0;
		this.indirectCount = 0;
		this.compatibleObjectsCount = 0;
		this.targetRingCount = 0;
		this.rebuildShellMode();
	}

	getFixture(): FixtureV1 | null {
		return this.fixture;
	}

	getFixtureRevision(): number {
		return this.fixtureRevision;
	}

	patchFixture(updater: (prev: FixtureV1) => FixtureV1): void {
		if (!this.fixture) {
			return;
		}
		const prev = this.fixture;
		const next = updater(prev);
		if (next === prev) {
			return;
		}
		this.fixture = next;
		const structureChanged = fixtureStateFingerprint(next) !== fixtureStateFingerprint(prev);
		if (structureChanged) {
			this.fixtureRevision += 1;
		}
		const poseChanged = fixturePoseFingerprint(next) !== fixturePoseFingerprint(prev);
		if (structureChanged || poseChanged) {
			this.emit();
		}
	}

	/** @emoji ✋ Persists a gumball relocate on the fixture (pose-only; no structure revision). */
	patchRelocate(payload: RelocatePayload): void {
		if (!this.fixture) {
			return;
		}
		const next = applyRelocateToSceneFixture(this.fixture, payload);
		if (next === this.fixture) {
			return;
		}
		this.fixture = next;
		this.emit();
	}

	/** @emoji 📷 Persists orbit camera on the fixture without bumping structure revision or re-emitting React state. */
	setCamera(camera: Partial<CameraState>): void {
		if (!this.fixture) {
			return;
		}
		const next = updateSceneCameraInFixture(this.fixture, camera);
		if (next === this.fixture) {
			return;
		}
		this.fixture = next;
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
		this.mainMode.tools = {
			selection: buildPlaygroundBrowseSelectionTools(SCENE_PLAY_KINDS, scenePlayKindLabel, this.selectableKinds, SCENE_PLAY_CONTROLLER_ID),
			filter: buildPlaygroundBrowseFilterTools(SCENE_PLAY_KINDS, scenePlayKindLabel, this.visibleKinds, SCENE_PLAY_CONTROLLER_ID),
			actions: relocateTools,
		};
	}

	private filterSelectionByPlaygroundKinds(selection: ScenePlaySelection): ScenePlaySelection {
		return {
			objectIds: this.selectableKinds.object && this.visibleKinds.object ? [...selection.objectIds] : [],
			vortexIds: this.selectableKinds.vortex && this.visibleKinds.vortex ? [...selection.vortexIds] : [],
			attractionIds: this.selectableKinds.attraction && this.visibleKinds.attraction ? [...selection.attractionIds] : [],
		};
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
			case "toggleSelectableKind": {
				const { kind } = args as { kind: ScenePlayPickKind };
				if (kind === "object" || kind === "vortex" || kind === "attraction") {
					this.selectableKinds[kind] = !this.selectableKinds[kind];
					this.selection = this.filterSelectionByPlaygroundKinds(this.selection);
				}
				break;
			}
			case "toggleVisibleKind": {
				const { kind } = args as { kind: ScenePlayPickKind };
				if (kind === "object" || kind === "vortex" || kind === "attraction") {
					this.visibleKinds[kind] = !this.visibleKinds[kind];
					this.selection = this.filterSelectionByPlaygroundKinds(this.selection);
				}
				break;
			}
			case "setSelection": {
				const next = (args as { selection: ScenePlaySelection }).selection;
				if (next && typeof next === "object") {
					const resolved = this.filterSelectionByPlaygroundKinds({
						objectIds: [...(next.objectIds ?? [])],
						vortexIds: [...(next.vortexIds ?? [])],
						attractionIds: [...(next.attractionIds ?? [])],
					});
					if (scenePlaySelectionEqual(this.selection, resolved)) {
						return;
					}
					this.selection = resolved;
				}
				syncShell = false;
				break;
			}
			case "setSelectedId": {
				const id = (args as { id: string | null }).id;
				const resolved: ScenePlaySelection = id
					? { objectIds: [id], vortexIds: [], attractionIds: [] }
					: SCENE_PLAY_EMPTY_SELECTION;
				if (scenePlaySelectionEqual(this.selection, resolved)) {
					return;
				}
				this.selection = resolved;
				syncShell = false;
				break;
			}
			case "noteSelection": {
				const snap = args as SelectionSnapshot & { attractionIds?: readonly string[] };
				const resolved = this.filterSelectionByPlaygroundKinds({
					objectIds: [...(snap.objectIds ?? [])],
					vortexIds: [...(snap.vortexIds ?? [])],
					attractionIds:
						snap.attractionIds !== undefined
							? [...snap.attractionIds]
							: snap.objectIds.length === 0 && snap.vortexIds.length === 0
								? []
								: [...this.selection.attractionIds],
				});
				if (scenePlaySelectionEqual(this.selection, resolved)) {
					return;
				}
				this.selection = resolved;
				syncShell = false;
				break;
			}
			case "deleteSelection": {
				this.applyDeleteSelection();
				break;
			}
			case "setSelectionMode": {
				const mode = (args as { mode: SelectionMode }).mode;
				if (mode === "single" || mode === "additive" || mode === "subtractive" || mode === "toggle") {
					this.selectionMode = mode;
				}
				break;
			}
			case "setProximityRadius": {
				const value = (args as { value: number }).value;
				if (typeof value === "number" && Number.isFinite(value) && value > 0) {
					this.proximityRadius = value;
				}
				break;
			}
			case "setChunkSize": {
				const value = (args as { value: number }).value;
				if (typeof value === "number" && Number.isFinite(value) && value > 0) {
					this.chunkSize = value;
				}
				break;
			}
			case "setGridFactor": {
				const value = (args as { value: number }).value;
				if (typeof value === "number" && Number.isFinite(value) && value > 0) {
					this.gridFactor = value;
				}
				break;
			}
			case "setShowLodGrid": {
				const pressed = (args as { pressed?: boolean }).pressed;
				if (typeof pressed === "boolean") {
					this.showLodGrid = pressed;
				}
				break;
			}
			case "setGridSnapEnabled": {
				const pressed = (args as { pressed?: boolean }).pressed;
				if (typeof pressed === "boolean") {
					this.gridSnapEnabled = pressed;
				}
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
			case "noteCompatibleObjects":
				this.compatibleObjectsCount += 1;
				break;
			case "noteTargetRing":
				this.targetRingCount += 1;
				break;
			default:
				break;
		}
		if (syncShell) {
			this.rebuildShellMode();
			this.emit();
		}
	}

	private applyDeleteSelection(): void {
		if (!this.fixture) {
			return;
		}
		const objectIds = [...this.selection.objectIds];
		const vortexIds = [...this.selection.vortexIds];
		const attractionIds = [...this.selection.attractionIds];
		if (objectIds.length === 0 && vortexIds.length === 0 && attractionIds.length === 0) {
			return;
		}
		this.patchFixture((fixture) => {
			let next = fixture;
			for (const objectId of objectIds) {
				next = deleteSceneObjectFromFixture(next, objectId);
			}
			for (const vortexFullId of vortexIds) {
				next = deleteSceneVortexFromFixture(next, vortexFullId);
			}
			for (const attractionId of attractionIds) {
				next = deleteSceneAttractionFromFixture(next, attractionId);
			}
			return next;
		});
		this.selection = SCENE_PLAY_EMPTY_SELECTION;
	}

	getSnapshot(): ScenePlaySnapshot {
		return {
			fixture: this.fixture,
			fixtureRevision: this.fixtureRevision,
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
			selection: this.selection,
			selectedId: primaryScenePlayObjectId(this.selection),
			selectionMode: this.selectionMode,
			proximityRadius: this.proximityRadius,
			chunkSize: this.chunkSize,
			gridFactor: this.gridFactor,
			showLodGrid: this.showLodGrid,
			gridSnapEnabled: this.gridSnapEnabled,
			proximityCount: this.proximityCount,
			connectCount: this.connectCount,
			indirectCount: this.indirectCount,
			compatibleObjectsCount: this.compatibleObjectsCount,
			targetRingCount: this.targetRingCount,
		};
	}
}

/** @emoji 📸 Host-consumed scene play state (no React/DOM). */
export interface ScenePlaySnapshot {
	readonly fixture: FixtureV1 | null;
	readonly fixtureRevision: number;
	readonly lodProps: ReturnType<typeof sceneLodCanvasProps>;
	readonly lodTag: number;
	readonly lodSlider: number;
	readonly automaticLod: boolean;
	readonly depthVariableLod: boolean;
	readonly relocateMode: RelocateMode;
	readonly selection: ScenePlaySelection;
	readonly selectedId: string | null;
	readonly selectionMode: SelectionMode;
	readonly proximityRadius: number;
	readonly chunkSize: number;
	readonly gridFactor: number;
	readonly showLodGrid: boolean;
	readonly gridSnapEnabled: boolean;
	readonly proximityCount: number;
	readonly connectCount: number;
	readonly indirectCount: number;
	readonly compatibleObjectsCount: number;
	readonly targetRingCount: number;
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
	app.leftTabs = [];
	app.rightTabs = [];
	return app;
}

/** @emoji 🚀 Creates a {@link ProductRuntime} with scene play app registered. */
export function buildScenePlayRuntime(): ProductRuntime {
	const runtime = new ProductRuntime();
	const controller = new ScenePlayShellController(runtime.commandBus, () => runtime.notify());
	runtime.addApp(buildScenePlayAppRuntime(controller));
	return runtime;
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

		it("stores nakagin vortex positions in type-local CAD space", () => {
			const f = parseFixtureV1(nakaginSceneFixtureJson as unknown);
			const o = f?.objects.find((obj) => obj.id === "01890804-66f2-4544-98f0-b6f0c0615492");
			const v = o?.vortices.find((vx) => vx.id.endsWith(":link"));
			expect(v?.position[0]).toBeCloseTo(-1.3, 5);
			expect(v?.position[1]).toBeCloseTo(-1.25, 5);
			expect(v?.position[2]).toBeCloseTo(0, 5);
		});

		it("patchFixture bumps revision only when structure changes", () => {
			const bus = new CommandBus();
			const wb = new ProductRuntime();
			const ctrl = new ScenePlayShellController(bus, () => wb.notify());
			const base = ctrl.getFixture();
			expect(base).not.toBeNull();
			const revisionBefore = ctrl.getFixtureRevision();
			ctrl.patchFixture((fixture) => ({
				...fixture,
				objects: fixture.objects.map((object, index) =>
					index === 0 ? { ...object, origin: [object.origin[0]! + 1, object.origin[1]!, object.origin[2]!] as const } : object,
				),
			}));
			expect(ctrl.getFixtureRevision()).toBe(revisionBefore);
			ctrl.patchFixture((fixture) => ({
				...fixture,
				objects: fixture.objects.slice(0, -1),
			}));
			expect(ctrl.getFixtureRevision()).toBe(revisionBefore + 1);
		});

		it("noteSelection skips emit when selection is unchanged", () => {
			const bus = new CommandBus();
			const wb = new ProductRuntime();
			const ctrl = new ScenePlayShellController(bus, () => wb.notify());
			let notifyCount = 0;
			const trackingBus = new CommandBus();
			const trackingWb = new ProductRuntime();
			const trackingCtrl = new ScenePlayShellController(trackingBus, () => {
				notifyCount += 1;
			});
			trackingCtrl.run("noteSelection", { objectIds: ["a"], vortexIds: [] });
			const afterFirst = notifyCount;
			trackingCtrl.run("noteSelection", { objectIds: ["a"], vortexIds: [] });
			expect(notifyCount).toBe(afterFirst);
			void bus;
			void wb;
			void ctrl;
		});

		it("deleteSelection removes selected fixture rows and clears selection", () => {
			const bus = new CommandBus();
			const wb = new ProductRuntime();
			const ctrl = new ScenePlayShellController(bus, () => wb.notify());
			const before = ctrl.getSnapshot().fixture;
			expect(before).not.toBeNull();
			const target = before!.objects[0]!;
			const countBefore = before!.objects.length;
			ctrl.run("setSelection", {
				selection: { objectIds: [target.id], vortexIds: [], attractionIds: [] },
			});
			ctrl.run("deleteSelection");
			const snap = ctrl.getSnapshot();
			expect(snap.fixture?.objects.some((object) => object.id === target.id)).toBe(false);
			expect(snap.fixture?.objects.length).toBe(countBefore - 1);
			expect(snap.selection).toEqual(SCENE_PLAY_EMPTY_SELECTION);
		});

		it("deleteSceneObjectFromFixture removes child vortices and stale attractions", () => {
			const base = parseFixtureV1({
				schema: "elements.scene.fixture/v1",
				camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
				attractions: [
					{ id: "t1", attracting: "a:v1", attracted: "b:v2" },
					{ id: "t2", attracting: "b:v2", attracted: "c:v3" },
				],
				objects: [
					{ id: "a", meshUrl: "/m.glb", origin: [0, 0, 0], vortices: [{ id: "v1", position: [0, 0, 0] }] },
					{ id: "b", meshUrl: "/m.glb", origin: [1, 0, 0], vortices: [{ id: "v2", position: [0, 0, 0] }] },
					{ id: "c", meshUrl: "/m.glb", origin: [2, 0, 0], vortices: [{ id: "v3", position: [0, 0, 0] }] },
				],
			});
			expect(base).not.toBeNull();
			const next = deleteSceneObjectFromFixture(base!, "b");
			expect(next.objects.map((object) => object.id)).toEqual(["a", "c"]);
			expect(next.attractions).toEqual([]);
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
