// #region 🧲Header
/** @emoji 📽 33. Projektetage — declarative paper intro via `@framework/presentation`. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	countArrangements,
	collectPresentationSlides,
	intro,
	splitFigureGrid,
	type DispositionPosition,
	type Participant,
	type Presentation,
	type SplitTile,
	type Thought,
} from "@framework/presentation/core";
import "./globals.css";
// #endregion 🔌Adapters

//#region 🔖Deck
const ASSET_CATALOGUE = "./bauteilbo\u0308rse.png";
const ASSET_VIDEO = "./bauen-mit-bestand.mp4";
const ASSET_THESIS_PDF = "./bachelor-thesis-ueli-saluz.pdf";

const CATALOGUE_PARTICIPANT = "catalogue";
const CATALOGUE_COL1 = "catalogue-col1";
const CATALOGUE_COL2 = "catalogue-col2";
const CATALOGUE_COL3 = "catalogue-col3";

const CATALOGUE_EMBODIMENT_CROP = "crop";
const CATALOGUE_EMBODIMENT_LABEL = "label";

const CATALOGUE_FRAME = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
const CATALOGUE_TILES_ASSEMBLED = splitFigureGrid({
	rows: 3,
	columns: 5,
	frame: CATALOGUE_FRAME,
	gap: 0,
});

const CATALOGUE_TILE_BY_KEY = new Map(CATALOGUE_TILES_ASSEMBLED.map((tile) => [tile.key, tile]));

/** @emoji 📐 Union of normalized figure crops for the given tile keys. */
function unionTileCrop(tiles: readonly SplitTile[], tileKeys: readonly string[]): DispositionPosition {
	const selected = tiles.filter((tile) => tileKeys.includes(tile.key));
	if (selected.length === 0) {
		throw new Error("unionTileCrop: no tiles matched the given keys.");
	}
	let minX = 1;
	let minY = 1;
	let maxX = 0;
	let maxY = 0;
	for (const tile of selected) {
		const crop = tile.crop;
		minX = Math.min(minX, crop.x);
		minY = Math.min(minY, crop.y);
		maxX = Math.max(maxX, crop.x + crop.width);
		maxY = Math.max(maxY, crop.y + crop.height);
	}
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

/** @emoji 📐 Bounding box of slide positions for the given tile keys. */
function unionTilePosition(tiles: readonly SplitTile[], tileKeys: readonly string[]): DispositionPosition {
	const selected = tiles.filter((tile) => tileKeys.includes(tile.key));
	if (selected.length === 0) {
		throw new Error("unionTilePosition: no tiles matched the given keys.");
	}
	let minX = 1;
	let minY = 1;
	let maxX = 0;
	let maxY = 0;
	for (const tile of selected) {
		const position = tile.position;
		minX = Math.min(minX, position.x);
		minY = Math.min(minY, position.y);
		maxX = Math.max(maxX, position.x + position.width);
		maxY = Math.max(maxY, position.y + position.height);
	}
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

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

const CATALOGUE_COLUMN_TILE_KEYS = {
	col1: ["tile-r1-c0", "tile-r1-c1", "tile-r1-c2", "tile-r1-c3", "tile-r1-c4", "tile-r2-c0"],
	col2: ["tile-r2-c1", "tile-r2-c2", "tile-r2-c3"],
	col3: ["tile-r2-c4"],
} as const;

const CATALOGUE_COLUMN_LABELS: Record<keyof typeof CATALOGUE_COLUMN_TILE_KEYS, string> = {
	col1: "Rippendecke",
	col2: "Unterzug",
	col3: "Stütze",
};

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

function catalogueColumnParticipant(
	id: string,
	tileKeys: readonly string[],
	label: string,
): Participant {
	return {
		id,
		embodiments: [
			{
				kind: "figure",
				id: CATALOGUE_EMBODIMENT_CROP,
				src: ASSET_CATALOGUE,
				alt: label,
				crop: unionTileCrop(CATALOGUE_TILES_ASSEMBLED, tileKeys),
			},
			{
				kind: "text",
				id: CATALOGUE_EMBODIMENT_LABEL,
				lines: [label],
				level: "heading",
				morphRoot: "heading-line",
			},
		],
	};
}

const introDeck = intro({
	id: "projektetage",
	name: "33. Projektetage",
	language: "de",
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
	name: "Medien",
	participants: [
		{
			id: CATALOGUE_PARTICIPANT,
			embodiments: [
				{
					kind: "figure",
					src: ASSET_CATALOGUE,
					alt: "Komponentenkatalog",
				},
			],
		},
		catalogueColumnParticipant(CATALOGUE_COL1, CATALOGUE_COLUMN_TILE_KEYS.col1, CATALOGUE_COLUMN_LABELS.col1),
		catalogueColumnParticipant(CATALOGUE_COL2, CATALOGUE_COLUMN_TILE_KEYS.col2, CATALOGUE_COLUMN_LABELS.col2),
		catalogueColumnParticipant(CATALOGUE_COL3, CATALOGUE_COLUMN_TILE_KEYS.col3, CATALOGUE_COLUMN_LABELS.col3),
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
	slides: [
		{
			arrangement: {
				id: "catalogue",
				name: "Bauteilkatalog",
				dispositions: [
					{
						participantId: CATALOGUE_PARTICIPANT,
						emphasis: "active",
						split: { tiles: CATALOGUE_TILES_ASSEMBLED },
					},
				],
			},
			transition: { kind: "morph" },
		},
		{
			arrangement: {
				id: "catalogue-focus",
				name: "Bauteilarten",
				settleAfterMorphFrom: ["catalogue"],
				dispositions: [
					{
						participantId: CATALOGUE_PARTICIPANT,
						emphasis: "active",
						split: { tiles: CATALOGUE_FOCUS_TILES },
					},
					{
						participantId: CATALOGUE_COL1,
						embodimentId: CATALOGUE_EMBODIMENT_CROP,
						emphasis: "active",
						position: unionTilePosition(CATALOGUE_FOCUS_TILES, CATALOGUE_COLUMN_TILE_KEYS.col1),
						style: { opacity: 0 },
					},
					{
						participantId: CATALOGUE_COL2,
						embodimentId: CATALOGUE_EMBODIMENT_CROP,
						emphasis: "active",
						position: unionTilePosition(CATALOGUE_FOCUS_TILES, CATALOGUE_COLUMN_TILE_KEYS.col2),
						style: { opacity: 0 },
					},
					{
						participantId: CATALOGUE_COL3,
						embodimentId: CATALOGUE_EMBODIMENT_CROP,
						emphasis: "active",
						position: unionTilePosition(CATALOGUE_FOCUS_TILES, CATALOGUE_COLUMN_TILE_KEYS.col3),
						style: { opacity: 0 },
					},
				],
			},
			transition: { kind: "morph" },
		},
		{
			arrangement: {
				id: "catalogue-labels",
				name: "Bauteilbeschriftungen",
				dispositions: [
					{
						participantId: CATALOGUE_COL1,
						embodimentId: CATALOGUE_EMBODIMENT_LABEL,
						emphasis: "active",
						position: stackedColumnLabelPosition(0),
					},
					{
						participantId: CATALOGUE_COL2,
						embodimentId: CATALOGUE_EMBODIMENT_LABEL,
						emphasis: "active",
						position: stackedColumnLabelPosition(1),
					},
					{
						participantId: CATALOGUE_COL3,
						embodimentId: CATALOGUE_EMBODIMENT_LABEL,
						emphasis: "active",
						position: stackedColumnLabelPosition(2),
					},
				],
			},
			transition: { kind: "morph" },
		},
		{
			arrangement: {
				id: "media-suite",
				name: "Medienüberblick",
				dispositions: [
					{
						participantId: CATALOGUE_PARTICIPANT,
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
		},
	],
};

const deck: Presentation = {
	...introDeck,
	chapters: [
		{
			...introDeck.chapters[0]!,
			sequences: [
				{
					...introDeck.chapters[0]!.sequences[0]!,
					thoughts: [...introDeck.chapters[0]!.sequences[0]!.thoughts, mediaThought],
				},
			],
		},
	],
};

function mount(): void {
	const el = document.getElementById("root");
	if (!el) {
		return;
	}
	void import("@framework/presentation/renderer/react").then(({ Expertise, mountPresentation }) => {
		mountPresentation(el, deck, {
			transition: "fade",
			slideNumber: false,
			surfaceChrome: { theme: "dark", device: "desktop", expertise: Expertise.NORMAL },
		});
	});
}

if (typeof document !== "undefined" && !import.meta.vitest) {
	mount();
}
//#endregion 🔖Deck

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	const { expandThoughtSlides } = await import("@framework/presentation/core");

	describe("projektetage deck", () => {
		it("declares intro plus expanded media render slides", () => {
			expect(countArrangements(deck)).toBeGreaterThan(11);
			expect(deck.language).toBe("de");
		});

		it("uses German bookmark names on intro and media slides", () => {
			const introSlide = collectPresentationSlides(deck)[0];
			expect(introSlide).toEqual({
				h: 0,
				v: 0,
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
				slide: "Titel",
			});
			const catalogueSlide = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
			expect(catalogueSlide).toMatchObject({
				h: 0,
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Medien",
				slide: "Bauteilkatalog",
			});
		});

		it("assembles the catalogue as a split figure grid", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.id === "media");
			const catalogue = media?.slides.find((slide) => slide.arrangement.id === "catalogue");
			expect(catalogue?.arrangement.dispositions[0]?.split?.tiles).toHaveLength(15);
		});

		it("focuses ten catalogue tiles plus hidden column morph anchors", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.id === "media");
			const focus = media?.slides.find((slide) => slide.arrangement.id === "catalogue-focus");
			const dispositions = focus?.arrangement.dispositions ?? [];
			expect(focus?.arrangement.settleAfterMorphFrom).toEqual(["catalogue"]);
			expect(dispositions[0]?.participantId).toBe(CATALOGUE_PARTICIPANT);
			expect(dispositions[0]?.split?.tiles).toHaveLength(10);
			expect(dispositions.slice(1).map((disposition) => disposition.participantId)).toEqual([
				CATALOGUE_COL1,
				CATALOGUE_COL2,
				CATALOGUE_COL3,
			]);
			expect(dispositions.slice(1).every((disposition) => disposition.style?.opacity === 0)).toBe(true);
			const col3 = dispositions.find((disposition) => disposition.participantId === CATALOGUE_COL3);
			expect(col3?.position?.height).toBeGreaterThan(0.7);
		});

		it("morphs catalogue tiles into focus and column crops directly into labels in one run", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.id === "media");
			expect(media).toBeDefined();
			const expanded = expandThoughtSlides(media!);
			const catalogueSlide = expanded.find((slide) => slide.id === "catalogue");
			const focusSlide = expanded.find((slide) => slide.id === "catalogue-focus");
			const bridgeSlide = expanded.find((slide) => slide.id === "catalogue-labels--bridge");
			const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
			expect(catalogueSlide?.autoAnimateId).toBeDefined();
			expect(focusSlide?.autoAnimateId).toBe(catalogueSlide?.autoAnimateId);
			expect(bridgeSlide).toBeUndefined();
			expect(labelSlide?.autoAnimateId).toBe(focusSlide?.autoAnimateId);
			expect(
				labelSlide?.arrangement.dispositions.every((disposition) => disposition.embodimentId === CATALOGUE_EMBODIMENT_LABEL),
			).toBe(true);
		});

		it("includes figure, video, and pdf participants in the media thought", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.id === "media");
			const kinds = media?.participants.flatMap((participant) => participant.embodiments.map((embodiment) => embodiment.kind)) ?? [];
			expect(kinds).toContain("figure");
			expect(kinds).toContain("video");
			expect(kinds).toContain("pdf");
		});
	});
}
//#endregion 🧪Tests
