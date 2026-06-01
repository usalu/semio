// #region 🧲Header
/** @emoji 📽 33. Projektetage — declarative paper intro via `@framework/presentation`. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	countArrangements,
	intro,
	splitFigureGrid,
	type DispositionPosition,
	type Presentation,
	type SplitColumnGroup,
	type SplitMorphTarget,
	type SplitTile,
	type Thought,
} from "@framework/presentation/core";
import { Expertise, mountPresentation } from "@framework/presentation/renderer/react";
import "./globals.css";
// #endregion 🔌Adapters

//#region 🔖Deck
const ASSET_CATALOGUE = "./bauteilbo\u0308rse.png";
const ASSET_VIDEO = "./bauen-mit-bestand.mp4";
const ASSET_THESIS_PDF = "./bachelor-thesis-ueli-saluz.pdf";

const CATALOGUE_FRAME = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
const CATALOGUE_TILES_ASSEMBLED = splitFigureGrid({
	rows: 3,
	columns: 5,
	frame: CATALOGUE_FRAME,
	gap: 0,
});

const CATALOGUE_TILE_BY_KEY = new Map(CATALOGUE_TILES_ASSEMBLED.map((tile) => [tile.key, tile]));

/** @emoji 📐 Ten catalogue tiles (5–14) as three separated columns (2×3 | 1×3 | 1×1). */
function catalogueFocusColumnTiles(): SplitTile[] {
	const rowGap = 0.014;
	const innerGap = 0.01;
	const columnGap = 0.05;
	const col1Width = 0.44;
	const col2Width = 0.2;
	const col3Width = 0.2;
	const blockWidth = col1Width + columnGap + col2Width + columnGap + col3Width;
	const layout: DispositionPosition = {
		x: (1 - blockWidth) / 2,
		y: 0.11,
		width: blockWidth,
		height: 0.78,
	};
	const col1X = layout.x;
	const col2X = col1X + col1Width + columnGap;
	const col3X = col2X + col2Width + columnGap;
	const rowHeight = (layout.height - rowGap * 2) / 3;
	const cellW1 = (col1Width - innerGap) / 2;
	const col3Height = layout.height;
	const rowY = (row: number): number => layout.y + row * (rowHeight + rowGap);

	const placements: readonly { readonly key: string; readonly position: DispositionPosition }[] = [
		{ key: "tile-r1-c0", position: { x: col1X, y: rowY(0), width: cellW1, height: rowHeight } },
		{ key: "tile-r1-c1", position: { x: col1X + cellW1 + innerGap, y: rowY(0), width: cellW1, height: rowHeight } },
		{ key: "tile-r1-c2", position: { x: col1X, y: rowY(1), width: cellW1, height: rowHeight } },
		{ key: "tile-r1-c3", position: { x: col1X + cellW1 + innerGap, y: rowY(1), width: cellW1, height: rowHeight } },
		{ key: "tile-r1-c4", position: { x: col1X, y: rowY(2), width: cellW1, height: rowHeight } },
		{ key: "tile-r2-c0", position: { x: col1X + cellW1 + innerGap, y: rowY(2), width: cellW1, height: rowHeight } },
		{ key: "tile-r2-c1", position: { x: col2X, y: rowY(0), width: col2Width, height: rowHeight } },
		{ key: "tile-r2-c2", position: { x: col2X, y: rowY(1), width: col2Width, height: rowHeight } },
		{ key: "tile-r2-c3", position: { x: col2X, y: rowY(2), width: col2Width, height: rowHeight } },
		{ key: "tile-r2-c4", position: { x: col3X, y: layout.y, width: col3Width, height: col3Height } },
	];

	return placements.map(({ key, position }) => {
		const tile = CATALOGUE_TILE_BY_KEY.get(key);
		if (!tile) {
			throw new Error(`Missing catalogue tile "${key}".`);
		}
		return { ...tile, position, emphasis: "active" as const };
	});
}

const CATALOGUE_FOCUS_TILES = catalogueFocusColumnTiles();

const CATALOGUE_COLUMN_LABEL_LINES: Record<string, string> = {
	col1: "Rippendecke",
	col2: "Unterzug",
	col3: "Stütze",
};

const CATALOGUE_FOCUS_COLUMN_GROUPS: readonly SplitColumnGroup[] = [
	{
		key: "col1",
		tileKeys: ["tile-r1-c0", "tile-r1-c1", "tile-r1-c2", "tile-r1-c3", "tile-r1-c4", "tile-r2-c0"],
		labelLine: CATALOGUE_COLUMN_LABEL_LINES.col1,
	},
	{
		key: "col2",
		tileKeys: ["tile-r2-c1", "tile-r2-c2", "tile-r2-c3"],
		labelLine: CATALOGUE_COLUMN_LABEL_LINES.col2,
	},
	{ key: "col3", tileKeys: ["tile-r2-c4"], labelLine: CATALOGUE_COLUMN_LABEL_LINES.col3 },
];

const CATALOGUE_LABEL_STACK_FRAME = { x: 0.38, y: 0.12, width: 0.24, height: 0.76 };
const CATALOGUE_LABEL_ROW_GAP = 0.04;

function stackedColumnLabelPosition(rowIndex: number): DispositionPosition {
	const rowHeight = (CATALOGUE_LABEL_STACK_FRAME.height - 2 * CATALOGUE_LABEL_ROW_GAP) / 3;
	return {
		x: CATALOGUE_LABEL_STACK_FRAME.x,
		y: CATALOGUE_LABEL_STACK_FRAME.y + rowIndex * (rowHeight + CATALOGUE_LABEL_ROW_GAP),
		width: CATALOGUE_LABEL_STACK_FRAME.width,
		height: rowHeight,
	};
}

/** @emoji 🏷 One stacked label per column; tiles morph via shared {@link columnMorphId} on hidden per-tile ghosts. */
const CATALOGUE_COLUMN_LABELS: readonly SplitMorphTarget[] = CATALOGUE_FOCUS_COLUMN_GROUPS.map((column, rowIndex) => ({
	columnKey: column.key,
	position: stackedColumnLabelPosition(rowIndex),
	lines: [column.labelLine ?? column.key],
	level: "heading",
	morphRoot: "heading-line",
}));

const introDeck = intro({
	id: "projektetage",
	name: "33. Projektetage",
	title: {
		full: ["Entwerfen mit Bestand"],
		short: "Entwerfen mit Bestand",
	},
	description: {
		full: [
			"Eine offene Plattform für einen KI-unterstützten, performance-optimierten und integrativen Entwurfsprozess mit wiederverwendeten Baukomponenten",
		],
		short: "Plattform zum Entwerfen mit wiederverwendete Bauteilen",
	},
	goal: ["Mehr Zeit zum manuellen Entwerfen", "dank Automatisierung!"],
	authors: {
		lines: [
			[
				{ name: "Ueli Saluz", marks: ["a", "1", "x"] },
				{ name: "Phillipp Geyer", marks: ["a", "1", "x"] },
			],
			[
				{ name: "Kinan Sarakbi", marks: ["a", "2", "y"] },
				{ name: "Christoph Gengnagel", marks: ["a", "2", "y"] },
			],
		],
	},
	affiliations: {
		steps: [
			[{ mark: "a", name: "Fakultät für Architektur" }],
			[
				{ mark: "a", name: "Fakultät für Architektur" },
				{ mark: "1", name: "Leibniz Universität Hannover" },
				{ mark: "2", name: "Universität der Künste" },
			],
			[
				{ mark: "a", name: "Fakultät für Architektur" },
				{
					mark: "1",
					name: "Leibniz Universität Hannover",
					shortName: "LUH",
					suffix: { mark: "x", name: "Nachhaltige Gebäudesysteme" },
				},
				{
					mark: "2",
					name: "Universität der Künste",
					shortName: "UdK",
					suffix: { mark: "y", name: "Konstruktives Entwerfen" },
				},
			],
		],
	},
});

const mediaThought: Thought = {
	id: "media",
	transition: { kind: "morph" },
	participants: [
		{
			id: "catalogue",
			embodiments: [
				{
					kind: "figure",
					src: ASSET_CATALOGUE,
					alt: "Komponentenkatalog",
				},
			],
		},
		{
			id: "demo-video",
			embodiments: [
				{
					kind: "video",
					src: ASSET_VIDEO,
					muted: true,
					controls: true,
				},
			],
		},
		{
			id: "thesis",
			embodiments: [
				{
					kind: "pdf",
					src: ASSET_THESIS_PDF,
					page: 1,
					alt: "Bachelorarbeit Ueli Saluz",
				},
			],
		},
	],
	arrangements: [
		{
			id: "catalogue",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					split: { tiles: CATALOGUE_TILES_ASSEMBLED, concealed: true },
				},
				{
					participantId: "catalogue",
					emphasis: "active",
					position: CATALOGUE_FRAME,
				},
			],
		},
		{
			id: "catalogue-focus",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					split: {
						tiles: CATALOGUE_FOCUS_TILES,
						columns: CATALOGUE_FOCUS_COLUMN_GROUPS,
						columnMorphTileGhosts: true,
					},
				},
			],
		},
		{
			id: "catalogue-labels",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					morphTargets: CATALOGUE_COLUMN_LABELS,
				},
			],
		},
		{
			id: "media-suite",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "muted",
					position: { x: 0.02, y: 0.05, width: 0.3, height: 0.35 },
				},
				{
					participantId: "demo-video",
					emphasis: "active",
					position: { x: 0.35, y: 0.1, width: 0.6, height: 0.5 },
				},
				{
					participantId: "thesis",
					emphasis: "active",
					position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
				},
			],
		},
	],
};

const deck: Presentation = {
	...introDeck,
	sequences: [
		{
			id: "main",
			thoughts: [...introDeck.sequences[0]!.thoughts, mediaThought],
		},
	],
};

function mount(): void {
	const el = document.getElementById("root");
	if (!el) {
		return;
	}
	mountPresentation(el, deck, {
		transition: "fade",
		hash: false,
		slideNumber: false,
		surfaceChrome: { theme: "dark", device: "desktop", expertise: Expertise.NORMAL },
	});
}

if (typeof document !== "undefined" && !import.meta.vitest) {
	mount();
}
//#endregion 🔖Deck

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("projektetage deck", () => {
		it("declares intro plus media arrangement slides", () => {
			expect(countArrangements(deck)).toBe(11);
		});

		it("conceals catalogue tiles under one figure for auto-animate into focus", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const catalogue = media?.arrangements.find((a) => a.id === "catalogue");
			expect(catalogue?.dispositions[0]?.split?.concealed).toBe(true);
			expect(catalogue?.dispositions[0]?.split?.tiles).toHaveLength(15);
			expect(catalogue?.dispositions[0]?.split?.columns).toBeUndefined();
			expect(catalogue?.dispositions[1]?.position).toEqual(CATALOGUE_FRAME);
			expect(catalogue?.dispositions[1]?.split).toBeUndefined();
		});

		it("arranges catalogue-focus as three columns without the top row", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const focus = media?.arrangements.find((a) => a.id === "catalogue-focus");
			const tiles = focus?.dispositions[0]?.split?.tiles ?? [];
			const keys = tiles.map((tile) => tile.key);
			expect(keys).toHaveLength(10);
			expect(keys.every((key) => !key.startsWith("tile-r0-"))).toBe(true);
			expect(keys).toEqual([
				"tile-r1-c0",
				"tile-r1-c1",
				"tile-r1-c2",
				"tile-r1-c3",
				"tile-r1-c4",
				"tile-r2-c0",
				"tile-r2-c1",
				"tile-r2-c2",
				"tile-r2-c3",
				"tile-r2-c4",
			]);
			const col1 = tiles.filter((tile) => tile.key.startsWith("tile-r1-") || tile.key === "tile-r2-c0");
			const col2 = tiles.filter((tile) => ["tile-r2-c1", "tile-r2-c2", "tile-r2-c3"].includes(tile.key));
			const col3 = tiles.filter((tile) => tile.key === "tile-r2-c4");
			expect(col1.every((tile) => (tile.position?.x ?? 0) < 0.5)).toBe(true);
			expect(col2.every((tile) => (tile.position?.x ?? 0) > 0.48 && (tile.position?.x ?? 0) < 0.72)).toBe(true);
			expect(col3[0]?.position?.x).toBeGreaterThan(0.7);
			expect(col3[0]?.position?.height).toBeGreaterThan(0.7);
		});

		it("stacks three column labels and uses per-tile column morph ghosts on focus", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const labels = media?.arrangements.find((a) => a.id === "catalogue-labels");
			const targets = labels?.dispositions[0]?.morphTargets ?? [];
			expect(targets).toHaveLength(3);
			expect(targets.map((target) => target.lines[0])).toEqual(["Rippendecke", "Unterzug", "Stütze"]);
			const ys = targets.map((target) => target.position.y);
			expect(ys[0]).toBeLessThan(ys[1] ?? 0);
			expect(ys[1]).toBeLessThan(ys[2] ?? 0);
			expect(media?.arrangements.find((a) => a.id === "catalogue-focus")?.dispositions[0]?.split?.columnMorphTileGhosts).toBe(
				true,
			);
			expect(media?.arrangements.some((a) => a.id === "catalogue-merge")).toBe(false);
		});

		it("includes figure, video, and pdf participants in the media thought", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const kinds = media?.participants.flatMap((p) => p.embodiments.map((e) => e.kind)) ?? [];
			expect(kinds).toEqual(["figure", "video", "pdf"]);
		});
	});
}
//#endregion 🧪Tests
