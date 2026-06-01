// #region 🧲Header
/** @emoji 📽 Shared deck metadata and intro spec for 33. Projektetage slide modules. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
    splitFigureGrid,
    type DispositionPosition,
    type IntroSpec,
    type Participant,
    type PresentationMeta,
    type SplitTile,
} from "@framework/presentation/core";
// #endregion 🔌Adapters

//#region 🔖Meta
export const presentationMeta: PresentationMeta = {
	id: "projektetage",
	name: "33. Projektetage",
	language: "de",
};

export const introSpec: IntroSpec = {
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
};
//#endregion 🔖Meta

//#region 🔖Catalogue
export const ASSET_CATALOGUE = "./bauteilbo\u0308rse.png";
export const ASSET_VIDEO = "./bauen-mit-bestand.mp4";
export const ASSET_THESIS_PDF = "./bachelor-thesis-ueli-saluz.pdf";

export const CATALOGUE_PARTICIPANT = "catalogue";
export const CATALOGUE_COL1 = "catalogue-col1";
export const CATALOGUE_COL2 = "catalogue-col2";
export const CATALOGUE_COL3 = "catalogue-col3";
export const CATALOGUE_LABELS = "catalogue-labels";

export const CATALOGUE_EMBODIMENT_CROP = "crop";
export const CATALOGUE_EMBODIMENT_LABEL = "label";
export const CATALOGUE_EMBODIMENT_STACK = "stack";

export const CATALOGUE_FRAME = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
export const CATALOGUE_TILES_ASSEMBLED = splitFigureGrid({
	rows: 3,
	columns: 5,
	frame: CATALOGUE_FRAME,
	gap: 0,
});

const CATALOGUE_TILE_BY_KEY = new Map(CATALOGUE_TILES_ASSEMBLED.map((tile) => [tile.key, tile]));

/** @emoji 📐 Union of normalized figure crops for the given tile keys. */
export function unionTileCrop(tiles: readonly SplitTile[], tileKeys: readonly string[]): DispositionPosition {
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
export function unionTilePosition(tiles: readonly SplitTile[], tileKeys: readonly string[]): DispositionPosition {
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
export function catalogueFocusColumnTiles(): SplitTile[] {
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

export const CATALOGUE_FOCUS_TILES = catalogueFocusColumnTiles();

/** @emoji 📐 Focus-slide tiles for one catalogue column participant. */
export function catalogueFocusTilesForColumn(
	column: keyof typeof CATALOGUE_COLUMN_TILE_KEYS,
): SplitTile[] {
	const keys = new Set<string>(CATALOGUE_COLUMN_TILE_KEYS[column]);
	return CATALOGUE_FOCUS_TILES.filter((tile) => keys.has(tile.key));
}

export const CATALOGUE_COLUMN_TILE_KEYS = {
	col1: ["tile-r1-c0", "tile-r1-c1", "tile-r1-c2", "tile-r1-c3", "tile-r1-c4", "tile-r2-c0"],
	col2: ["tile-r2-c1", "tile-r2-c2", "tile-r2-c3"],
	col3: ["tile-r2-c4"],
} as const;

export const CATALOGUE_COLUMN_LABELS: Record<keyof typeof CATALOGUE_COLUMN_TILE_KEYS, string> = {
	col1: "Rippenplatte",
	col2: "Unterzug",
	col3: "Stütze",
};

export const CATALOGUE_LABEL_STACK_FRAME = { x: 0.38, y: 0.12, width: 0.24, height: 0.76 };
export const CATALOGUE_LABEL_ROW_GAP = 0.04;

export function stackedColumnLabelPosition(rowIndex: number): DispositionPosition {
	const rowHeight = (CATALOGUE_LABEL_STACK_FRAME.height - 2 * CATALOGUE_LABEL_ROW_GAP) / 3;
	return {
		x: CATALOGUE_LABEL_STACK_FRAME.x,
		y: CATALOGUE_LABEL_STACK_FRAME.y + rowIndex * (rowHeight + CATALOGUE_LABEL_ROW_GAP),
		width: CATALOGUE_LABEL_STACK_FRAME.width,
		height: rowHeight,
	};
}

export function catalogueColumnParticipant(
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

/** @emoji 🏷 Single participant receiving independent morphs from each catalogue column. */
export const catalogueLabelsParticipant: Participant = {
	id: CATALOGUE_LABELS,
	embodiments: [
		{
			kind: "text",
			id: CATALOGUE_EMBODIMENT_STACK,
			lines: [
				CATALOGUE_COLUMN_LABELS.col1,
				CATALOGUE_COLUMN_LABELS.col2,
				CATALOGUE_COLUMN_LABELS.col3,
			],
			level: "heading",
			morphRoot: "heading-block",
		},
	],
};

export const mediaParticipants: Participant[] = [
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
	catalogueLabelsParticipant,
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
];
//#endregion 🔖Catalogue
