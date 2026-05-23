// #region 🧲Header
// 💻 elements/client/lib/geometry/play/index.tsx — Geometry play harness: Topologic all-kinds selector, single-window UI shell, and transform gumball editing for every entity kind.
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	WorkbenchApp,
	WorkbenchMode,
	WorkbenchWindowKind,
	createDefaultLayout,
	mountReactApp,
	type ShellToolItem,
} from "@elements/ui";

import topologyJson from "./fixtures/topology.json";
import type { TopologicTransformMode } from "../react/index.tsx";
import {
	TOPOLOGIC_KINDS,
	deriveAnalyzeTopologicFixtureV1,
	ensureTopologicWasmLoaded,
	loadTopologicFixtureV1,
	updateTopologicFixtureTransformKernelV1,
	type TopologicEntity,
	type TopologicFixtureV1,
	type TopologicKind,
	type TopologicTransform,
} from "../wasm/index.ts";
import "./globals.css";
import * as React from "react";

//#region 🔖Session
function childIds(entity: TopologicEntity): readonly string[] {
	if (entity.kind === "topology") return entity.members;
	if (entity.kind === "wire") return entity.edges;
	if (entity.kind === "face") return entity.wires;
	if (entity.kind === "shell") return entity.faces;
	if (entity.kind === "cell") return entity.shells;
	if (entity.kind === "cellComplex") return entity.cells;
	if (entity.kind === "cluster") return entity.topologies;
	if (entity.kind === "edge") return entity.vertices;
	return [];
}

class TopologicPlaySession {
	readonly entityById: ReadonlyMap<string, TopologicEntity>;

	constructor(readonly fixture: TopologicFixtureV1) {
		this.entityById = new Map(fixture.topologies.map((entity) => [entity.id, entity]));
	}

	getEntity(id: string): TopologicEntity | undefined {
		return this.entityById.get(id);
	}

	listByKind(kind: TopologicKind): readonly TopologicEntity[] {
		return this.fixture.topologies.filter((entity) => entity.kind === kind);
	}

	childrenOf(id: string): readonly TopologicEntity[] {
		const entity = this.entityById.get(id);
		if (!entity) return [];
		return childIds(entity)
			.map((childId) => this.entityById.get(childId))
			.filter((child): child is TopologicEntity => Boolean(child));
	}
}

function topologicEntityLabel(entity: TopologicEntity): string {
	return entity.label ?? entity.id;
}
//#endregion 🔖Session

//#region 🔖Ids
export const GEOMETRY_PLAY_APP_ID = "elements-geometry-play";
export const GEOMETRY_PLAY_WINDOW_ID = "geometry-topologic-window";
export const GEOMETRY_PLAY_WINDOW_LABEL = "Topologic Playground";
export const GEOMETRY_PLAY_DEFAULT_LAYOUT = createDefaultLayout([GEOMETRY_PLAY_WINDOW_ID], "row", [100], [GEOMETRY_PLAY_WINDOW_LABEL]);
export const GEOMETRY_PLAY_TRANSFORM_MODES = ["translate", "rotate", "scale"] as const satisfies readonly TopologicTransformMode[];
const GEOMETRY_PLAY_MODES = ["edit", "analyze"] as const;
//#endregion 🔖Ids

//#region 🔖AnalyzeKinds
type GeometryPlayMode = (typeof GEOMETRY_PLAY_MODES)[number];
type AnalyzeSurfaceExposure = "external" | "internal";
type AnalyzeSurfaceStance = "horizontal" | "vertical";
type AnalyzePartOverlap = "none" | "difference" | "intersection";
export type AnalyzeKind =
	| `surface.${AnalyzeSurfaceExposure}.${AnalyzeSurfaceStance}`
	| `part.${AnalyzePartOverlap}`
	| "solid";

export const ANALYZE_KINDS = [
	"surface.external.horizontal",
	"surface.external.vertical",
	"surface.internal.horizontal",
	"surface.internal.vertical",
	"part.none",
	"part.difference",
	"part.intersection",
	"solid",
] as const satisfies readonly AnalyzeKind[];

const ANALYZE_SURFACE_KINDS = ANALYZE_KINDS.filter((kind) => kind.startsWith("surface.")) as readonly AnalyzeKind[];
const ANALYZE_PART_KINDS = ANALYZE_KINDS.filter((kind) => kind.startsWith("part.")) as readonly AnalyzeKind[];
const ANALYZE_KIND_SET = new Set<string>(ANALYZE_KINDS);
//#endregion 🔖AnalyzeKinds

//#region 🔖Controls
export function geometryKindLabel(kind: TopologicKind): string {
	if (kind === "cellComplex") return "CellComplex";
	return kind.charAt(0).toUpperCase() + kind.slice(1);
}

export function analyzeKindLabel(kind: AnalyzeKind): string {
	if (kind === "solid") return "Solid";
	if (kind.startsWith("surface.")) {
		const [, exposure, stance] = kind.split(".") as ["surface", AnalyzeSurfaceExposure, AnalyzeSurfaceStance];
		return `${exposure.charAt(0).toUpperCase() + exposure.slice(1)} ${stance.charAt(0).toUpperCase() + stance.slice(1)}`;
	}
	const [, overlap] = kind.split(".") as ["part", AnalyzePartOverlap];
	return overlap.charAt(0).toUpperCase() + overlap.slice(1);
}

export function createAllKindsEnabled<TKind extends string>(order: readonly TKind[]): Record<TKind, boolean> {
	return Object.fromEntries(order.map((kind) => [kind, true])) as Record<TKind, boolean>;
}

export function listEnabledKinds<TKind extends string>(order: readonly TKind[], kinds: Readonly<Record<TKind, boolean>>): TKind[] {
	return order.filter((kind) => kinds[kind]);
}

export function formatEnabledKindsLabel(enabledKinds: readonly string[], totalCount: number): string {
	return enabledKinds.length === totalCount ? "all" : enabledKinds.join(",") || "none";
}

function areAllKindsEnabled<TKind extends string>(order: readonly TKind[], kinds: Readonly<Record<TKind, boolean>>): boolean {
	return order.every((kind) => kinds[kind]);
}

export function setKindGroup<TKind extends string>(
	current: Readonly<Record<TKind, boolean>>,
	order: readonly TKind[],
	enabled: boolean,
): Record<TKind, boolean> {
	return Object.fromEntries(order.map((kind) => [kind, enabled])) as Record<TKind, boolean>;
}

export function entityAnalyzeKind(entity: TopologicEntity): AnalyzeKind | null {
	const kind = entity.metadata?.analyzeKind;
	return typeof kind === "string" && ANALYZE_KIND_SET.has(kind) ? (kind as AnalyzeKind) : null;
}

export function isAnalyzeEntitySelectable(entity: TopologicEntity, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>): boolean {
	const kind = entityAnalyzeKind(entity);
	if (!kind) return false;
	return entity.metadata?.analyzeSelectable === true && selectableKinds[kind];
}

export function isAnalyzeEntityVisible(entity: TopologicEntity, visibleKinds: Readonly<Record<AnalyzeKind, boolean>>): boolean {
	const kind = entityAnalyzeKind(entity);
	return kind ? visibleKinds[kind] : false;
}

export function listSelectableEntities(session: TopologicPlaySession, selectableKinds: Readonly<Record<TopologicKind, boolean>>) {
	return session.fixture.topologies.filter((entity) => selectableKinds[entity.kind]);
}

export function listAnalyzeSelectableEntities(session: TopologicPlaySession, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>) {
	return session.fixture.topologies.filter((entity) => isAnalyzeEntitySelectable(entity, selectableKinds));
}

export function isSelectableEntity(
	session: TopologicPlaySession,
	selectableKinds: Readonly<Record<TopologicKind, boolean>>,
	id: string | null,
): boolean {
	if (!id) return false;
	const entity = session.getEntity(id);
	return Boolean(entity && selectableKinds[entity.kind]);
}

export function isAnalyzeSelectableEntity(session: TopologicPlaySession, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>, id: string | null): boolean {
	if (!id) return false;
	const entity = session.getEntity(id);
	return Boolean(entity && isAnalyzeEntitySelectable(entity, selectableKinds));
}

export function geometryPlayModeFromApp(activeModeId: string | null): GeometryPlayMode {
	return activeModeId === "analyze" ? "analyze" : "edit";
}
//#endregion 🔖Controls

//#region 🔖GeometryPlayWorkbench
export const GEOMETRY_PLAY_BODY_KEY = "elements.geometry.play.window";
export const GEOMETRY_PLAY_CONTROLLER_ID = "geometry-play";

export const GEOMETRY_PLAY_ICON_BOX_SELECT = "elements.geometry.icon.box-select";
export const GEOMETRY_PLAY_ICON_MOVE_3D = "elements.geometry.icon.move-3d";
export const GEOMETRY_PLAY_ICON_ROTATE_3D = "elements.geometry.icon.rotate-3d";
export const GEOMETRY_PLAY_ICON_SCALE_3D = "elements.geometry.icon.scale-3d";

function geometryKindShellToggles<TKind extends string>(
	prefix: "selection" | "filter",
	order: readonly TKind[],
	labelForKind: (kind: TKind) => string,
	kinds: Readonly<Record<TKind, boolean>>,
	command: string,
): ShellToolItem[] {
	return order.map((kind, itemOrder) => ({
		id: `geometry.${prefix}.kind.${kind}`,
		kind: "toggle" as const,
		text: labelForKind(kind),
		order: itemOrder,
		pressed: kinds[kind],
		controllerId: GEOMETRY_PLAY_CONTROLLER_ID,
		command,
		args: { kind },
	}));
}

function geometryAnalyzeShellToggles(
	prefix: "selection" | "filter",
	kinds: Readonly<Record<AnalyzeKind, boolean>>,
	command: string,
	groupCommand: string,
): ShellToolItem[] {
	const surfaceKinds = ANALYZE_KINDS.filter((kind) => kind.startsWith("surface.")) as readonly AnalyzeKind[];
	const partKinds = ANALYZE_KINDS.filter((kind) => kind.startsWith("part.")) as readonly AnalyzeKind[];
	const surfacesEnabled = surfaceKinds.every((kind) => kinds[kind]);
	const partsEnabled = partKinds.every((kind) => kinds[kind]);
	return [
		{
			id: `geometry.${prefix}.group.surface`,
			kind: "toggle" as const,
			text: "Surfaces",
			order: 0,
			pressed: surfacesEnabled,
			controllerId: GEOMETRY_PLAY_CONTROLLER_ID,
			command: groupCommand,
			args: { kindIds: surfaceKinds, enabled: !surfacesEnabled },
		},
		...geometryKindShellToggles(prefix, surfaceKinds, analyzeKindLabel, kinds, command).map((item) => ({ ...item, order: (item.order ?? 0) + 1 })),
		{ id: `geometry.${prefix}.group.surface.separator`, kind: "separator" as const, order: surfaceKinds.length + 1 },
		{
			id: `geometry.${prefix}.group.part`,
			kind: "toggle" as const,
			text: "Parts",
			order: surfaceKinds.length + 2,
			pressed: partsEnabled,
			controllerId: GEOMETRY_PLAY_CONTROLLER_ID,
			command: groupCommand,
			args: { kindIds: partKinds, enabled: !partsEnabled },
		},
		...geometryKindShellToggles(prefix, partKinds, analyzeKindLabel, kinds, command).map((item) => ({ ...item, order: (item.order ?? 0) + surfaceKinds.length + 3 })),
		{ id: `geometry.${prefix}.group.part.separator`, kind: "separator" as const, order: surfaceKinds.length + partKinds.length + 3 },
		...geometryKindShellToggles(prefix, ["solid"], analyzeKindLabel, kinds, command).map((item) => ({ ...item, order: (item.order ?? 0) + surfaceKinds.length + partKinds.length + 4 })),
	];
}

/** @emoji 🎮 Framework-free geometry play state and toolbar wiring for {@link Workbench}. */
export class GeometryPlayShellController extends Controller {
	readonly editMode = new WorkbenchMode("edit", "Edit", undefined);
	readonly analyzeMode = new WorkbenchMode("analyze", "Analyze", undefined);
	fixture: TopologicFixtureV1 | null;
	loadError: Error | null = null;
	selectableKinds: Record<TopologicKind, boolean>;
	visibleKinds: Record<TopologicKind, boolean>;
	analyzeSelectableKinds: Record<AnalyzeKind, boolean>;
	analyzeVisibleKinds: Record<AnalyzeKind, boolean>;
	selectedId: string | null;
	transformMode: TopologicTransformMode;

	constructor(commandBus: CommandBus, hostNotify: () => void, initialFixture: TopologicFixtureV1 | null = null) {
		super(GEOMETRY_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixture = initialFixture;
		this.selectableKinds = createAllKindsEnabled(TOPOLOGIC_KINDS);
		this.visibleKinds = createAllKindsEnabled(TOPOLOGIC_KINDS);
		this.analyzeSelectableKinds = createAllKindsEnabled(ANALYZE_KINDS);
		this.analyzeVisibleKinds = createAllKindsEnabled(ANALYZE_KINDS);
		this.selectedId = null;
		this.transformMode = "translate";
		if (initialFixture) this.rebuildShellModes();
		else void this.bootstrapFixture();
	}

	private async bootstrapFixture(): Promise<void> {
		try {
			await ensureTopologicWasmLoaded();
			const parsedFixture = await loadTopologicFixtureV1(topologyJson as unknown);
			if (!parsedFixture) throw new Error("geometry topology fixture failed to parse");
			this.fixture = parsedFixture;
			this.loadError = null;
		} catch (error) {
			this.loadError = error instanceof Error ? error : new Error(String(error));
		}
		this.rebuildShellModes();
		this.emit();
	}

	rebuildShellModes(): void {
		const selectionKindOrderBase = TOPOLOGIC_KINDS.length;
		this.editMode.tools = {
			selection: [
				...geometryKindShellToggles("selection", TOPOLOGIC_KINDS, geometryKindLabel, this.selectableKinds, "toggleSelectableKind"),
				{ id: "geometry.selection.separator.clear", kind: "separator", order: selectionKindOrderBase },
				{
					id: "geometry.selection.clear",
					kind: "button",
					iconId: GEOMETRY_PLAY_ICON_BOX_SELECT,
					label: "Clear",
					order: selectionKindOrderBase + 1,
					controllerId: GEOMETRY_PLAY_CONTROLLER_ID,
					command: "setSelectedId",
					args: { id: null },
				},
			],
			filter: geometryKindShellToggles("filter", TOPOLOGIC_KINDS, geometryKindLabel, this.visibleKinds, "toggleVisibleKind"),
			actions: GEOMETRY_PLAY_TRANSFORM_MODES.map((mode, order) => ({
				id: `geometry.transform.${mode}`,
				kind: "toggle" as const,
				iconId:
					mode === "translate"
						? GEOMETRY_PLAY_ICON_MOVE_3D
						: mode === "rotate"
							? GEOMETRY_PLAY_ICON_ROTATE_3D
							: GEOMETRY_PLAY_ICON_SCALE_3D,
				label: mode.charAt(0).toUpperCase() + mode.slice(1),
				order,
				pressed: this.transformMode === mode,
				controllerId: GEOMETRY_PLAY_CONTROLLER_ID,
				command: "setTransformMode",
				args: { mode },
			})),
		};
		this.analyzeMode.tools = {
			selection: [
				...geometryAnalyzeShellToggles("selection", this.analyzeSelectableKinds, "toggleAnalyzeSelectableKind", "setAnalyzeSelectableGroup"),
				{ id: "geometry.analyze.selection.separator.clear", kind: "separator", order: ANALYZE_KINDS.length + 4 },
				{
					id: "geometry.analyze.selection.clear",
					kind: "button",
					iconId: GEOMETRY_PLAY_ICON_BOX_SELECT,
					label: "Clear",
					order: ANALYZE_KINDS.length + 5,
					controllerId: GEOMETRY_PLAY_CONTROLLER_ID,
					command: "setSelectedId",
					args: { id: null },
				},
			],
			filter: geometryAnalyzeShellToggles("filter", this.analyzeVisibleKinds, "toggleAnalyzeVisibleKind", "setAnalyzeVisibleGroup"),
		};
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "toggleSelectableKind": {
				const kind = (args as { kind: TopologicKind }).kind;
				this.selectableKinds = { ...this.selectableKinds, [kind]: !this.selectableKinds[kind] };
				break;
			}
			case "toggleVisibleKind": {
				const kind = (args as { kind: TopologicKind }).kind;
				this.visibleKinds = { ...this.visibleKinds, [kind]: !this.visibleKinds[kind] };
				break;
			}
			case "toggleAnalyzeSelectableKind": {
				const kind = (args as { kind: AnalyzeKind }).kind;
				this.analyzeSelectableKinds = { ...this.analyzeSelectableKinds, [kind]: !this.analyzeSelectableKinds[kind] };
				break;
			}
			case "toggleAnalyzeVisibleKind": {
				const kind = (args as { kind: AnalyzeKind }).kind;
				this.analyzeVisibleKinds = { ...this.analyzeVisibleKinds, [kind]: !this.analyzeVisibleKinds[kind] };
				break;
			}
			case "setAnalyzeSelectableGroup": {
				const { kindIds, enabled } = args as { kindIds: readonly AnalyzeKind[]; enabled: boolean };
				this.analyzeSelectableKinds = { ...this.analyzeSelectableKinds, ...setKindGroup(this.analyzeSelectableKinds, [...kindIds], enabled) };
				break;
			}
			case "setAnalyzeVisibleGroup": {
				const { kindIds, enabled } = args as { kindIds: readonly AnalyzeKind[]; enabled: boolean };
				this.analyzeVisibleKinds = { ...this.analyzeVisibleKinds, ...setKindGroup(this.analyzeVisibleKinds, [...kindIds], enabled) };
				break;
			}
			case "setSelectedId": {
				const id = (args as { id: string | null }).id;
				if (!this.fixture) break;
				const session = new TopologicPlaySession(this.fixture);
				const analyzeFixture = deriveAnalyzeTopologicFixtureV1(this.fixture);
				const analyzeSession = new TopologicPlaySession(analyzeFixture);
				if (!id || isSelectableEntity(session, this.selectableKinds, id) || isAnalyzeSelectableEntity(analyzeSession, this.analyzeSelectableKinds, id)) {
					this.selectedId = id;
				}
				break;
			}
			case "setTransformMode": {
				const { pressed, mode } = args as { pressed?: boolean; mode?: TopologicTransformMode };
				if (pressed && mode) this.transformMode = mode;
				break;
			}
			default:
				break;
		}
		this.ensureSelectionValidity();
		this.rebuildShellModes();
		this.emit();
	}

	commitEntityTransform(id: string, transform: TopologicTransform): void {
		if (!this.fixture) return;
		this.fixture = updateTopologicFixtureTransformKernelV1(this.fixture, id, transform) ?? this.fixture;
		this.rebuildShellModes();
		this.emit();
	}

	private ensureSelectionValidity(): void {
		if (!this.fixture) return;
		const session = new TopologicPlaySession(this.fixture);
		const analyzeFixture = deriveAnalyzeTopologicFixtureV1(this.fixture);
		const analyzeSession = new TopologicPlaySession(analyzeFixture);
		if (!isSelectableEntity(session, this.selectableKinds, this.selectedId) && !isAnalyzeSelectableEntity(analyzeSession, this.analyzeSelectableKinds, this.selectedId)) {
			this.selectedId = null;
		}
	}

	getSnapshot(): GeometryPlaySnapshot | null {
		if (this.loadError) throw this.loadError;
		if (!this.fixture) return null;
		const session = new TopologicPlaySession(this.fixture);
		const analyzeFixture = deriveAnalyzeTopologicFixtureV1(this.fixture);
		const analyzeSession = new TopologicPlaySession(analyzeFixture);
		return {
			fixture: this.fixture,
			session,
			analyzeFixture,
			analyzeSession,
			selectableKinds: this.selectableKinds,
			visibleKinds: this.visibleKinds,
			analyzeSelectableKinds: this.analyzeSelectableKinds,
			analyzeVisibleKinds: this.analyzeVisibleKinds,
			selectedId: this.selectedId,
			transformMode: this.transformMode,
			setSelectedId: (id) => this.commandBus.dispatch(this.id, "setSelectedId", { id }),
			setTransformMode: (mode) => this.commandBus.dispatch(this.id, "setTransformMode", { mode, pressed: true }),
			onTransformCommit: (id, transform) => this.commitEntityTransform(id, transform),
		};
	}
}

/** @emoji 🧭 Values consumed by the geometry play window body (React adapter). */
export interface GeometryPlaySnapshot {
	readonly fixture: TopologicFixtureV1;
	readonly session: TopologicPlaySession;
	readonly analyzeFixture: TopologicFixtureV1;
	readonly analyzeSession: TopologicPlaySession;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly analyzeSelectableKinds: Readonly<Record<AnalyzeKind, boolean>>;
	readonly analyzeVisibleKinds: Readonly<Record<AnalyzeKind, boolean>>;
	readonly selectedId: string | null;
	readonly transformMode: TopologicTransformMode;
	readonly setSelectedId: (id: string | null) => void;
	readonly setTransformMode: (mode: TopologicTransformMode) => void;
	readonly onTransformCommit: (id: string, transform: TopologicTransform) => void;
}

/** @emoji 🧩 Builds the single-app geometry play registration for a {@link Workbench}. */
export function buildGeometryPlayWorkbenchApp(controller: GeometryPlayShellController): WorkbenchApp {
	const app = new WorkbenchApp(
		GEOMETRY_PLAY_APP_ID,
		"Geometry play",
		undefined,
		controller,
		GEOMETRY_PLAY_DEFAULT_LAYOUT as never,
		new WorkbenchWindowKind(GEOMETRY_PLAY_WINDOW_ID, GEOMETRY_PLAY_WINDOW_LABEL, GEOMETRY_PLAY_BODY_KEY),
	);
	app.defaultModeId = "edit";
	app.addMode(controller.editMode);
	app.addMode(controller.analyzeMode);
	controller.rebuildShellModes();
	return app;
}
//#endregion 🔖GeometryPlayWorkbench

void (async () => {
	const [{ WorkbenchView, LevelProvider, getLevelBgClass, mountReactApp }, mod] = await Promise.all([import("@elements/ui"), import("./react.tsx")]);
	const wb = await mod.bootstrapGeometryPlayWorkbench();
	mountReactApp(
		<LevelProvider>
			<WorkbenchView workbench={wb} className={getLevelBgClass(0)} />
		</LevelProvider>,
	);
})();

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("geometry play fixture", () => {
		it("enables selection and visibility for every kind by default", () => {
			expect(createAllKindsEnabled(TOPOLOGIC_KINDS)).toEqual({
				topology: true,
				vertex: true,
				edge: true,
				wire: true,
				face: true,
				shell: true,
				cell: true,
				cellComplex: true,
				cluster: true,
			});
			expect(createAllKindsEnabled(ANALYZE_KINDS)).toEqual({
				"surface.external.horizontal": true,
				"surface.external.vertical": true,
				"surface.internal.horizontal": true,
				"surface.internal.vertical": true,
				"part.none": true,
				"part.difference": true,
				"part.intersection": true,
				solid: true,
			});
		});

		it("ships at least one selectable entity for every topologic kind", async () => {
			const fixture = (await loadTopologicFixtureV1(topologyJson as unknown)) as TopologicFixtureV1;
			const session = new TopologicPlaySession(fixture);
			for (const kind of TOPOLOGIC_KINDS) {
				expect(session.listByKind(kind).length).toBeGreaterThan(0);
			}
		});

		it("registers translate, rotate, and scale as toolbar transform tools", () => {
			expect([...GEOMETRY_PLAY_TRANSFORM_MODES]).toEqual(["translate", "rotate", "scale"]);
		});

		it("labels enabled kind sets for the status strip", () => {
			const all = createAllKindsEnabled(TOPOLOGIC_KINDS);
			expect(formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, all), TOPOLOGIC_KINDS.length)).toBe("all");
			expect(formatEnabledKindsLabel(["vertex", "edge"], TOPOLOGIC_KINDS.length)).toBe("vertex,edge");
			expect(formatEnabledKindsLabel([], TOPOLOGIC_KINDS.length)).toBe("none");
		});

		it("adds analyze group toggles for surfaces and parts", () => {
			expect(ANALYZE_KINDS.filter((kind) => kind.startsWith("surface."))).not.toHaveLength(0);
			expect(ANALYZE_KINDS.filter((kind) => kind.startsWith("part."))).not.toHaveLength(0);
		});

		it("toggles topology vertex selection via shell commands", async () => {
			const bus = new CommandBus();
			const fixture = (await loadTopologicFixtureV1(topologyJson as unknown)) as TopologicFixtureV1;
			const ctrl = new GeometryPlayShellController(bus, () => undefined, fixture);
			expect(ctrl.selectableKinds.vertex).toBe(true);
			bus.dispatch(GEOMETRY_PLAY_CONTROLLER_ID, "toggleSelectableKind", { kind: "vertex" });
			expect(ctrl.selectableKinds.vertex).toBe(false);
			bus.dispatch(GEOMETRY_PLAY_CONTROLLER_ID, "setTransformMode", { mode: "rotate", pressed: true });
			expect(ctrl.transformMode).toBe("rotate");
		});

		it("derives analyze solids, parts, and semantic surfaces from the shipped fixture", async () => {
			const fixture = (await loadTopologicFixtureV1(topologyJson as unknown)) as TopologicFixtureV1;
			const analyzeFixture = deriveAnalyzeTopologicFixtureV1(fixture);
			const analyzeSession = new TopologicPlaySession(analyzeFixture);
			const selectable = analyzeFixture.topologies.filter((entity) => entity.metadata?.analyzeSelectable === true);
			expect(selectable.filter((entity) => entity.metadata?.analyzeGroup === "solid")).toHaveLength(3);
			expect(selectable.filter((entity) => entity.metadata?.analyzeKind === "part.difference")).toHaveLength(3);
			expect(selectable.filter((entity) => entity.metadata?.analyzeKind === "part.intersection")).toHaveLength(2);
			expect(selectable.filter((entity) => String(entity.metadata?.analyzeKind).startsWith("surface."))).not.toHaveLength(0);
			expect(isAnalyzeSelectableEntity(analyzeSession, createAllKindsEnabled(ANALYZE_KINDS), "analyze.part.1")).toBe(true);
			expect(isAnalyzeSelectableEntity(analyzeSession, createAllKindsEnabled(ANALYZE_KINDS), "analyze.part.1.face.1")).toBe(false);
		});
	});
}
//#endregion 🧪Tests