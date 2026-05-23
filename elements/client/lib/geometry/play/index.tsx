// #region 🧲Header
// 💻 elements/client/lib/geometry/play/index.tsx — Geometry play harness: Topologic all-kinds selector, single-window UI shell, and transform gumball editing for every entity kind.
// #endregion 🧲Header

import { createDefaultLayout, mountAsyncReactApp, type UIToolbarItem } from "@elements/ui";

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
void mountAsyncReactApp(async () => (await import("./react.tsx")).createGeometryPlayElement());

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