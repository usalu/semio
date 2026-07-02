// #region 🧲Header
/** @emoji 🛝 Puzzle 2D playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import {
	PUZZLE_2D_PLAY_APP_ID,
	PUZZLE_2D_PLAY_CONTROLLER_ID,
	Puzzle2dPlayShellController,
	buildPuzzle2dPlayAppRuntime,
	registerPuzzle2dPlayDeclarativeBodies,
} from "./index.ts";
import nakaginFixtureJson from "../fixture/nakagin-capsule-tower.2d.json";
import concreteForestFixtureJson from "../fixture/concrete-forest.2d.json";
import { wireLiteralFromDagFixtureJson } from "@semio-tech/graph-dsl-core";
import {
	parsePuzzle2dFixture,
	puzzle2dFixtureNodeDisplayLabel,
	PUZZLE_2D_CAMERA_ZOOM_MAX,
	PUZZLE_2D_CAMERA_ZOOM_MIN,
	type CameraState,
	type Puzzle2dFixture,
} from "@semio-tech/puzzle-2d-react";

type Puzzle2dPlayPaneId = "2d-overview" | "2d-detail" | "2d-selection";

export const PUZZLE_2D_PLAY_FIXTURE_NAKAGIN_ID = "nakagin";
export const PUZZLE_2D_PLAY_FIXTURE_CONCRETE_FOREST_ID = "concrete-forest";

export const PUZZLE_2D_PLAY_FIXTURE_OPTIONS = [
	{ id: PUZZLE_2D_PLAY_FIXTURE_CONCRETE_FOREST_ID, label: "Concrete Forest" },
	{ id: PUZZLE_2D_PLAY_FIXTURE_NAKAGIN_ID, label: "Nakagin capsule tower" },
] as const;

/** @emoji 🔒 Resolves a playground fixture slug (e.g. `concrete`) to a puzzle 2d fixture id. */
export function resolvePuzzle2dPlayFixtureSlug(slug: string): string | undefined {
	const aliases: Record<string, string> = { concrete: PUZZLE_2D_PLAY_FIXTURE_CONCRETE_FOREST_ID };
	const normalized = aliases[slug] ?? slug;
	return PUZZLE_2D_PLAY_FIXTURE_OPTIONS.some((row) => row.id === normalized) ? normalized : undefined;
}

const PUZZLE_2D_PLAY_FIXTURE_JSON_BY_ID: Record<string, unknown> = {
	[PUZZLE_2D_PLAY_FIXTURE_NAKAGIN_ID]: nakaginFixtureJson,
	[PUZZLE_2D_PLAY_FIXTURE_CONCRETE_FOREST_ID]: concreteForestFixtureJson,
};

/** @emoji 🧪 Resolves imported puzzle 2d fixture JSON by catalog id. */
export function puzzle2dPlayFixtureJson(fixtureId: string = PUZZLE_2D_PLAY_FIXTURE_CONCRETE_FOREST_ID): unknown {
	return PUZZLE_2D_PLAY_FIXTURE_JSON_BY_ID[fixtureId] ?? concreteForestFixtureJson;
}

/** @emoji 📋 Parses a puzzle 2d play fixture by catalog id. */
export function puzzle2dPlayFixtureForId(fixtureId: string): Puzzle2dFixture {
	const parsed = parsePuzzle2dFixture(puzzle2dPlayFixtureJson(fixtureId) as unknown);
	if (!parsed) throw new Error(`puzzle 2d fixture "${fixtureId}" is invalid`);
	return parsed;
}

/** @emoji 📄 Serializes a puzzle 2d fixture for Jack and VCS bridges. */
export function puzzle2dFixtureToJson(fixture: Puzzle2dFixture): string {
	return JSON.stringify(fixture);
}

/** @emoji 🃏 Normalizes a puzzle 2d fixture into board-shaped JSON for Jack queries. */
export function puzzle2dFixtureToJackBoardJson(fixtureOrJson: Puzzle2dFixture | string): string {
	const fixture =
		typeof fixtureOrJson === "string"
			? (parsePuzzle2dFixture(JSON.parse(fixtureOrJson) as unknown) ?? PUZZLE_2D_PLAY_EMPTY_FIXTURE)
			: fixtureOrJson;
	return JSON.stringify({
		schema: fixture.schema,
		nodes: fixture.nodes.map((node) => ({
			id: node.id,
			nodeKind: "node",
			text: puzzle2dFixtureNodeDisplayLabel(node),
		})),
		edges: fixture.edges,
	});
}

/** @emoji 🔌 Renders a puzzle fixture as wire-literal compiled DAG text. */
export function puzzle2dFixtureToCompiledDagWireLiteral(fixtureOrJson: Puzzle2dFixture | string): string {
	const fixture =
		typeof fixtureOrJson === "string"
			? (parsePuzzle2dFixture(JSON.parse(fixtureOrJson) as unknown) ?? PUZZLE_2D_PLAY_EMPTY_FIXTURE)
			: fixtureOrJson;
	return wireLiteralFromDagFixtureJson(
		JSON.stringify({
			nodes: fixture.nodes.map((node) => ({
				id: node.id,
				operatorKind: node.nodeKind ?? "node",
			})),
			edges: fixture.edges.map((edge) => ({
				id: edge.id,
				source: edge.source,
				target: edge.target,
			})),
		}),
	);
}

export const PUZZLE_2D_PLAY_DEFAULT_FIXTURE: Puzzle2dFixture = puzzle2dPlayFixtureForId(PUZZLE_2D_PLAY_FIXTURE_CONCRETE_FOREST_ID);

const PUZZLE_2D_PLAY_VIEWPORT_REF_SHORT_PX = 640;
const PUZZLE_2D_PLAY_VIEWPORT_MARGIN = 0.18;
const PUZZLE_2D_PLAY_VIEWPORT_FRAMING_HALF_SPAN_SCALE = 2.25;
const PUZZLE_2D_PLAY_VIEWPORT_ZOOM_BOOST = 2.5;
const PUZZLE_2D_PLAY_VIEWPORT_PANE_ZOOM_SCALE: Record<Puzzle2dPlayPaneId, number> = {
	"2d-overview": 0.68,
	"2d-detail": 2.15,
	"2d-selection": 0.36,
};

function clampPuzzle2dPlayViewportZoom(value: number): number {
	return Math.min(PUZZLE_2D_CAMERA_ZOOM_MAX, Math.max(PUZZLE_2D_CAMERA_ZOOM_MIN, value));
}

function puzzle2dPlayNodeWorldExtents(node: Record<string, unknown>): { minX: number; minY: number; maxX: number; maxY: number } | null {
	const x = Number(node.x);
	const y = Number(node.y);
	if (!Number.isFinite(x) || !Number.isFinite(y)) {
		return null;
	}
	if (node.shape === "rectangle") {
		const width = Number(node.width);
		const height = Number(node.height);
		if (!Number.isFinite(width) || !Number.isFinite(height) || width <= 0 || height <= 0) {
			return null;
		}
		const hw = width / 2;
		const hh = height / 2;
		return { minX: x - hw, maxX: x + hw, minY: y - hh, maxY: y + hh };
	}
	const radius = Number(node.radius);
	if (!Number.isFinite(radius) || radius <= 0) {
		return null;
	}
	return { minX: x - radius, maxX: x + radius, minY: y - radius, maxY: y + radius };
}

function puzzle2dPlayFixtureWorldBoundsFromNodeRecords(nodes: readonly Record<string, unknown>[]): { cx: number; cy: number; halfSpan: number } {
	let minX = Number.POSITIVE_INFINITY;
	let minY = Number.POSITIVE_INFINITY;
	let maxX = Number.NEGATIVE_INFINITY;
	let maxY = Number.NEGATIVE_INFINITY;
	for (const node of nodes) {
		const extents = puzzle2dPlayNodeWorldExtents(node);
		if (!extents) continue;
		minX = Math.min(minX, extents.minX);
		maxX = Math.max(maxX, extents.maxX);
		minY = Math.min(minY, extents.minY);
		maxY = Math.max(maxY, extents.maxY);
	}
	if (!Number.isFinite(minX)) {
		return { cx: 0, cy: 0, halfSpan: 400 };
	}
	const cx = (minX + maxX) / 2;
	const cy = (minY + maxY) / 2;
	const halfSpan = Math.max(maxX - minX, maxY - minY, 1) / 2;
	return { cx, cy, halfSpan };
}

function puzzle2dPlayFixtureWorldBounds(fixture: Puzzle2dFixture): { cx: number; cy: number; halfSpan: number } {
	return puzzle2dPlayFixtureWorldBoundsFromNodeRecords(fixture.nodes as unknown as Record<string, unknown>[]);
}

function puzzle2dPlayFixtureWorldBoundsFromJson(raw: unknown): { cx: number; cy: number; halfSpan: number } | null {
	if (!raw || typeof raw !== "object") return null;
	const nodes = (raw as Record<string, unknown>).nodes;
	if (!Array.isArray(nodes)) return null;
	const records = nodes.filter((node): node is Record<string, unknown> => Boolean(node) && typeof node === "object");
	if (!records.length) return null;
	return puzzle2dPlayFixtureWorldBoundsFromNodeRecords(records);
}

function puzzle2dPlayViewportCameraFromBounds(
	fixture: Puzzle2dFixture,
	bounds: { cx: number; cy: number; halfSpan: number },
): CameraState {
	const usable = PUZZLE_2D_PLAY_VIEWPORT_REF_SHORT_PX * (1 - 2 * PUZZLE_2D_PLAY_VIEWPORT_MARGIN);
	const worldSpan = Math.max(2 * bounds.halfSpan * PUZZLE_2D_PLAY_VIEWPORT_FRAMING_HALF_SPAN_SCALE, 1);
	const zoom = clampPuzzle2dPlayViewportZoom((usable / worldSpan) * PUZZLE_2D_PLAY_VIEWPORT_ZOOM_BOOST);
	return {
		x: bounds.cx,
		y: bounds.cy,
		zoom,
	};
}

/** @emoji 📷 Viewport camera centered on fixture node bounds with zoom fitted for growth. */
export function puzzle2dPlayViewportCameraFromFixture(fixture: Puzzle2dFixture, rawFixture?: unknown): CameraState {
	const bounds = (rawFixture ? puzzle2dPlayFixtureWorldBoundsFromJson(rawFixture) : null) ?? puzzle2dPlayFixtureWorldBounds(fixture);
	return puzzle2dPlayViewportCameraFromBounds(fixture, bounds);
}

/** @emoji 📷 Viewport camera for a play fixture catalog id (uses raw JSON bounds before circle normalization). */
export function puzzle2dPlayViewportCameraForFixtureId(fixtureId: string): CameraState {
	const raw = puzzle2dPlayFixtureJson(fixtureId);
	return puzzle2dPlayViewportCameraFromFixture(puzzle2dPlayFixtureForId(fixtureId), raw);
}

function puzzle2dPlayTriptychCameraForPane(
	pane: Puzzle2dPlayPaneId,
	fixture: Puzzle2dFixture,
	bounds: { cx: number; cy: number; halfSpan: number },
	baseZoom: number,
): CameraState {
	const camOffset = fixture.camera;
	const detailNode = fixture.nodes[Math.min(42, Math.max(0, fixture.nodes.length - 1))];
	const zoom = clampPuzzle2dPlayViewportZoom(baseZoom * PUZZLE_2D_PLAY_VIEWPORT_PANE_ZOOM_SCALE[pane]);
	switch (pane) {
		case "2d-overview":
			return { x: bounds.cx + camOffset.x * 0.04, y: bounds.cy + camOffset.y * 0.03, zoom };
		case "2d-detail":
			return {
				x: (detailNode?.x ?? bounds.cx) + camOffset.x * 0.02,
				y: (detailNode?.y ?? bounds.cy) + camOffset.y * 0.02,
				zoom,
			};
		case "2d-selection":
			return {
				x: bounds.cx - bounds.halfSpan * 0.28 + camOffset.x * 0.06,
				y: bounds.cy + bounds.halfSpan * 0.22 + camOffset.y * 0.05,
				zoom,
			};
	}
}

/** @emoji 📷 Default cameras for all puzzle 2d play panes (wide overview, tight detail, regional selection). */
export function puzzle2dPlayTriptychCamerasFromFixture(fixture: Puzzle2dFixture, rawFixture?: unknown): Record<Puzzle2dPlayPaneId, CameraState> {
	const bounds = (rawFixture ? puzzle2dPlayFixtureWorldBoundsFromJson(rawFixture) : null) ?? puzzle2dPlayFixtureWorldBounds(fixture);
	const base = puzzle2dPlayViewportCameraFromBounds(fixture, bounds);
	return {
		"2d-overview": puzzle2dPlayTriptychCameraForPane("2d-overview", fixture, bounds, base.zoom),
		"2d-detail": puzzle2dPlayTriptychCameraForPane("2d-detail", fixture, bounds, base.zoom),
		"2d-selection": puzzle2dPlayTriptychCameraForPane("2d-selection", fixture, bounds, base.zoom),
	};
}

export const PUZZLE_2D_PLAY_EMPTY_FIXTURE: Puzzle2dFixture = {
	schema: "puzzle.2d.fixture/v1",
	camera: { x: 0, y: 0, zoom: 1 },
	nodes: [],
	edges: [],
};



/** @emoji 🛝 Puzzle 2d play harness as a single {@link Playground} instance. */
export class Playground2d extends Playground {
	readonly id = PUZZLE_2D_PLAY_APP_ID;
	readonly keybindings = [
		{ key: "ctrl+a,meta+a", controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID, command: "selectAllSelection" },
	];

	createRuntime(): Platform {
		const runtime = new Platform({ id: this.id });
		const ctrl = new Puzzle2dPlayShellController(runtime.commandBus, () => runtime.notify(), () => runtime.notifyChrome());
		runtime.addApp(buildPuzzle2dPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerPuzzle2dPlayDeclarativeBodies();
	}
}


//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Puzzle 2D playground app definition. */
export const puzzle2dPlayAppDefinition: PlaygroundAppDefinition = {
	id: PUZZLE_2D_PLAY_APP_ID,
	label: "Puzzle 2D",
	controllerId: PUZZLE_2D_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new Playground2d(),
	bootRenderer: async (pg) => {
		const { boot2dPlay } = await import("@semio-tech/framework-playground-renderer-react/puzzle/2d");
		boot2dPlay(pg);
	},
	devHost: {
		playEntryKind: "2d",
		resolveDedupe: ["react", "react-dom", "three"],
		watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/Cargo.toml", "../rs/Cargo.lock", "../rs/script.ts"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "three", "@react-three/fiber", "@react-three/drei", "lucide-react", "@semio-tech/infinite-cavas-react-renderer"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
