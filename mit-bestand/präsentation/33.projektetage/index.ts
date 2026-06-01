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
const CATALOGUE_SPLIT_FRAME = { x: 0.18, y: 0.12, width: 0.64, height: 0.52 };
const CATALOGUE_TILES_ASSEMBLED = splitFigureGrid({
	rows: 3,
	columns: 5,
	frame: CATALOGUE_FRAME,
	gap: 0,
});
const CATALOGUE_TILES_SPREAD = splitFigureGrid({
	rows: 3,
	columns: 5,
	frame: CATALOGUE_SPLIT_FRAME,
	gap: 0.02,
});

const CATALOGUE_TILE_BY_KEY = new Map(CATALOGUE_TILES_ASSEMBLED.map((tile) => [tile.key, tile]));

/** @emoji 📐 Ten catalogue tiles (5–14) in three columns after dropping the top row (1–4). */
function catalogueFocusColumnTiles(): SplitTile[] {
	const layout: DispositionPosition = { x: 0.08, y: 0.12, width: 0.84, height: 0.76 };
	const gap = 0.015;
	const col1Width = layout.width * 0.48;
	const col2Width = layout.width * 0.24;
	const col3Width = layout.width * 0.24;
	const col2X = layout.x + col1Width + gap;
	const col3X = col2X + col2Width + gap;
	const rowHeight = (layout.height - gap * 2) / 3;
	const cellW1 = (col1Width - gap) / 2;
	const rowY = (row: number): number => layout.y + row * (rowHeight + gap);

	const placements: readonly { readonly key: string; readonly position: DispositionPosition }[] = [
		{ key: "tile-r1-c0", position: { x: layout.x, y: rowY(0), width: cellW1, height: rowHeight } },
		{ key: "tile-r1-c1", position: { x: layout.x + cellW1 + gap, y: rowY(0), width: cellW1, height: rowHeight } },
		{ key: "tile-r1-c2", position: { x: layout.x, y: rowY(1), width: cellW1, height: rowHeight } },
		{ key: "tile-r1-c3", position: { x: layout.x + cellW1 + gap, y: rowY(1), width: cellW1, height: rowHeight } },
		{ key: "tile-r1-c4", position: { x: layout.x, y: rowY(2), width: cellW1, height: rowHeight } },
		{ key: "tile-r2-c0", position: { x: layout.x + cellW1 + gap, y: rowY(2), width: cellW1, height: rowHeight } },
		{ key: "tile-r2-c1", position: { x: col2X, y: rowY(0), width: col2Width, height: rowHeight } },
		{ key: "tile-r2-c2", position: { x: col2X, y: rowY(1), width: col2Width, height: rowHeight } },
		{ key: "tile-r2-c3", position: { x: col2X, y: rowY(2), width: col2Width, height: rowHeight } },
		{ key: "tile-r2-c4", position: { x: col3X, y: rowY(1), width: col3Width, height: rowHeight } },
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
					split: { tiles: CATALOGUE_TILES_ASSEMBLED },
				},
			],
		},
		{
			id: "catalogue-split",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					split: { tiles: CATALOGUE_TILES_SPREAD },
				},
			],
		},
		{
			id: "catalogue-focus",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					split: { tiles: CATALOGUE_FOCUS_TILES },
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

		it("uses fifteen split tiles on the catalogue slide for reveal auto-animate", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const catalogue = media?.arrangements.find((a) => a.id === "catalogue");
			expect(catalogue?.dispositions[0]?.split?.tiles).toHaveLength(15);
		});

		it("arranges catalogue-focus as three columns without the top row", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const focus = media?.arrangements.find((a) => a.id === "catalogue-focus");
			const keys = focus?.dispositions[0]?.split?.tiles.map((tile) => tile.key) ?? [];
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
		});

		it("includes figure, video, and pdf participants in the media thought", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const kinds = media?.participants.flatMap((p) => p.embodiments.map((e) => e.kind)) ?? [];
			expect(kinds).toEqual(["figure", "video", "pdf"]);
		});
	});
}
//#endregion 🧪Tests
