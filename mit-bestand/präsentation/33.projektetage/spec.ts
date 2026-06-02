// #region 🧲Header
/** @emoji 📽 Shared deck metadata and intro spec for 33. Projektetage slide modules. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	remapSplitDispositions,
	split,
	splitFigureGrid,
	type Disposition,
	type DispositionPosition,
	type Embodiment,
	type MorphToSlot,
	type IntroSpec,
	type Participant,
	type PresentationMeta,
	type SplitArtifacts,
	unionSourceCrops,
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
export const ASSET_CATALOGUE = "/bauteilbörse.png";
export const ASSET_VIDEO = "./bauen-mit-bestand.mp4";
export const ASSET_THESIS_PDF = "./bachelor-thesis-ueli-saluz.pdf";

export const CATALOGUE_PARTICIPANT = "catalogue";
export const CATALOGUE_COL1 = "catalogue-col1";
export const CATALOGUE_COL2 = "catalogue-col2";
export const CATALOGUE_COL3 = "catalogue-col3";

export const CATALOGUE_EMBODIMENT_FULL = "catalogue--full";
export const CATALOGUE_EMBODIMENT_COL1_CROP = "catalogue-col1--crop";
export const CATALOGUE_EMBODIMENT_COL1_LABEL = "catalogue-col1--label";
export const CATALOGUE_EMBODIMENT_COL2_CROP = "catalogue-col2--crop";
export const CATALOGUE_EMBODIMENT_COL2_LABEL = "catalogue-col2--label";
export const CATALOGUE_EMBODIMENT_COL3_CROP = "catalogue-col3--crop";
export const CATALOGUE_EMBODIMENT_COL3_LABEL = "catalogue-col3--label";

export const CATALOGUE_FRAME = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };

/** @emoji 🏷 Grid keys of all 3×5 catalogue tiles → semantic participant ids. */
export const CATALOGUE_TILE_SEMANTIC_KEYS = {
	"tile-r0-c0": "Struktur 1",
	"tile-r0-c1": "Struktur 2",
	"tile-r0-c2": "Flächen",
	"tile-r0-c3": "Elemente 1",
	"tile-r0-c4": "Elemente 2",
	"tile-r1-c0": "Rippenplatte 1",
	"tile-r1-c1": "Rippenplatte 2",
	"tile-r1-c2": "Rippenplatte 3",
	"tile-r1-c3": "Rippenplatte 4",
	"tile-r1-c4": "Rippenplatte 5",
	"tile-r2-c0": "Rippenplatte 6",
	"tile-r2-c1": "Unterzug 1",
	"tile-r2-c2": "Unterzug 2",
	"tile-r2-c3": "Unterzug 3",
	"tile-r2-c4": "Stütze",
} as const;

/** @emoji 🧩 Applies semantic participant ids to split template artifacts. */
export function catalogueSplitWithSemanticKeys(artifacts: SplitArtifacts): SplitArtifacts {
	const keyMap = CATALOGUE_TILE_SEMANTIC_KEYS;
	const remapId = (gridKey: string): string =>
		keyMap[gridKey as keyof typeof keyMap] ?? gridKey;
	const participants = artifacts.participants.map((participant) => ({
		id: remapId(participant.id),
	}));
	const embodiments = artifacts.embodiments.map((embodiment) => {
		const gridKey = embodiment.id.replace(/-figure$/, "");
		const semantic = remapId(gridKey);
		return { ...embodiment, id: `${semantic}-figure` };
	});
	const dispositions = artifacts.dispositions.map((disposition) => {
		const semantic = remapId(disposition.participantId);
		return {
			...disposition,
			participantId: semantic,
			embodimentId: `${semantic}-figure`,
		};
	});
	return { participants, embodiments, dispositions };
}

const CATALOGUE_SPLIT_RAW = split({
	source: ASSET_CATALOGUE,
	rows: 3,
	columns: 5,
	frame: CATALOGUE_FRAME,
	gap: 0,
});

export const CATALOGUE_SPLIT = catalogueSplitWithSemanticKeys(CATALOGUE_SPLIT_RAW);

/** @emoji 📐 Union of normalized figure crops for participant ids. */
export function unionTileCropForParticipants(
	artifacts: SplitArtifacts,
	participantIds: readonly string[],
): DispositionPosition {
	const crops = artifacts.dispositions
		.filter((disposition) => participantIds.includes(disposition.participantId))
		.map((disposition) => {
			const embodiment = artifacts.embodiments.find((entry) => entry.id === disposition.embodimentId);
			return embodiment?.crop;
		})
		.filter((crop): crop is DispositionPosition => crop !== undefined);
	return unionSourceCrops(crops);
}

/** @emoji 📐 Bounding box of slide positions for participant ids. */
export function unionTilePositionForParticipants(
	artifacts: SplitArtifacts,
	participantIds: readonly string[],
): DispositionPosition {
	const positions = artifacts.dispositions
		.filter((disposition) => participantIds.includes(disposition.participantId))
		.map((disposition) => disposition.position)
		.filter((position): position is DispositionPosition => position !== undefined);
	if (positions.length === 0) {
		throw new Error("unionTilePositionForParticipants: no positions matched.");
	}
	let minX = 1;
	let minY = 1;
	let maxX = 0;
	let maxY = 0;
	for (const position of positions) {
		minX = Math.min(minX, position.x);
		minY = Math.min(minY, position.y);
		maxX = Math.max(maxX, position.x + position.width);
		maxY = Math.max(maxY, position.y + position.height);
	}
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

/** @emoji 📐 Ten catalogue tiles (5–14) as three separated columns (2×3 | 1×3 | 1×1). */
export function catalogueFocusColumnTiles(): readonly { readonly participantId: string; readonly position: DispositionPosition }[] {
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

	const placements: readonly { readonly gridKey: string; readonly position: DispositionPosition }[] = [
		{ gridKey: "tile-r1-c0", position: { x: col1X, y: rowY(0), width: cellW1, height: rowHeight } },
		{ gridKey: "tile-r1-c1", position: { x: col1X + cellW1 + innerGap, y: rowY(0), width: cellW1, height: rowHeight } },
		{ gridKey: "tile-r1-c2", position: { x: col1X, y: rowY(1), width: cellW1, height: rowHeight } },
		{ gridKey: "tile-r1-c3", position: { x: col1X + cellW1 + innerGap, y: rowY(1), width: cellW1, height: rowHeight } },
		{ gridKey: "tile-r1-c4", position: { x: col1X, y: rowY(2), width: cellW1, height: rowHeight } },
		{ gridKey: "tile-r2-c0", position: { x: col1X + cellW1 + innerGap, y: rowY(2), width: cellW1, height: rowHeight } },
		{ gridKey: "tile-r2-c1", position: { x: col2X, y: rowY(0), width: col2Width, height: rowHeight } },
		{ gridKey: "tile-r2-c2", position: { x: col2X, y: rowY(1), width: col2Width, height: rowHeight } },
		{ gridKey: "tile-r2-c3", position: { x: col2X, y: rowY(2), width: col2Width, height: rowHeight } },
		{ gridKey: "tile-r2-c4", position: { x: col3X, y: layout.y, width: col3Width, height: col3Height } },
	];

	return placements.map(({ gridKey, position }) => {
		const semantic =
			CATALOGUE_TILE_SEMANTIC_KEYS[gridKey as keyof typeof CATALOGUE_TILE_SEMANTIC_KEYS] ?? gridKey;
		return { participantId: semantic, position };
	});
}

export const CATALOGUE_FOCUS_TILES = catalogueFocusColumnTiles();

export const CATALOGUE_COLUMN_TILE_KEYS = {
	col1: [
		"Rippenplatte 1",
		"Rippenplatte 2",
		"Rippenplatte 3",
		"Rippenplatte 4",
		"Rippenplatte 5",
		"Rippenplatte 6",
	],
	col2: ["Unterzug 1", "Unterzug 2", "Unterzug 3"],
	col3: ["Stütze"],
} as const;

export const CATALOGUE_COLUMN_LABELS: Record<keyof typeof CATALOGUE_COLUMN_TILE_KEYS, string> = {
	col1: "Rippenplatte",
	col2: "Unterzug",
	col3: "Stütze",
};

export const CATALOGUE_LABEL_INLINE_FRAME = { x: 0.1, y: 0.44, width: 0.8, height: 0.12 };
export const CATALOGUE_LABEL_INLINE_GAP = 0.03;

/** @emoji 📐 One of three equal inline label slots on the Bauteilbeschriftungen row. */
export function inlineColumnLabelPosition(columnIndex: 0 | 1 | 2): DispositionPosition {
	const gap = CATALOGUE_LABEL_INLINE_GAP;
	const colWidth = (CATALOGUE_LABEL_INLINE_FRAME.width - gap * 2) / 3;
	return {
		x: CATALOGUE_LABEL_INLINE_FRAME.x + columnIndex * (colWidth + gap),
		y: CATALOGUE_LABEL_INLINE_FRAME.y,
		width: colWidth,
		height: CATALOGUE_LABEL_INLINE_FRAME.height,
	};
}

/** @emoji 📐 Focus-slide dispositions for catalogue tile participants. */
export function catalogueFocusDispositions(): readonly Disposition[] {
	const positions = Object.fromEntries(CATALOGUE_FOCUS_TILES.map((tile) => [tile.participantId, tile.position]));
	return remapSplitDispositions(
		CATALOGUE_SPLIT.dispositions.filter((disposition) =>
			CATALOGUE_FOCUS_TILES.some((tile) => tile.participantId === disposition.participantId),
		),
		positions,
	);
}

/** @emoji 🔀 One-to-many morphTo slots: catalogue figure into focus tiles at grid positions on the catalogue slide. */
export function catalogueFocusMorphTo(): readonly MorphToSlot[] {
	return CATALOGUE_FOCUS_TILES.map((tile) => {
		const splitDisposition = CATALOGUE_SPLIT.dispositions.find(
			(disposition) => disposition.participantId === tile.participantId,
		);
		const position = splitDisposition?.position;
		if (!position) {
			throw new Error(`catalogueFocusMorphTo: no grid position for "${tile.participantId}".`);
		}
		return {
			participantId: tile.participantId,
			embodimentId: `${tile.participantId}-figure`,
			position,
		};
	});
}

/** @emoji 🔀 Many-to-one morphFrom slots: focus tiles (source figure) into one column label disposition. */
export function columnLabelMorphFrom(
	column: keyof typeof CATALOGUE_COLUMN_TILE_KEYS,
	labelPosition: DispositionPosition,
): Disposition["morphFrom"] {
	return CATALOGUE_COLUMN_TILE_KEYS[column].map((participantId) => ({
		participantId,
		embodimentId: `${participantId}-figure`,
		position: labelPosition,
	}));
}

export const mediaParticipants: Participant[] = [
	{ id: CATALOGUE_PARTICIPANT },
	{ id: CATALOGUE_COL1 },
	{ id: CATALOGUE_COL2 },
	{ id: CATALOGUE_COL3 },
	{ id: "demo-video" },
	{ id: "thesis" },
	...CATALOGUE_SPLIT.participants,
];

export const mediaEmbodiments: Embodiment[] = [
	{
		kind: "figure",
		id: CATALOGUE_EMBODIMENT_FULL,
		src: ASSET_CATALOGUE,
		alt: "Komponentenkatalog",
	},
	{
		kind: "figure",
		id: CATALOGUE_EMBODIMENT_COL1_CROP,
		src: ASSET_CATALOGUE,
		alt: CATALOGUE_COLUMN_LABELS.col1,
		crop: unionTileCropForParticipants(CATALOGUE_SPLIT, CATALOGUE_COLUMN_TILE_KEYS.col1),
	},
	{
		kind: "text",
		id: CATALOGUE_EMBODIMENT_COL1_LABEL,
		lines: [CATALOGUE_COLUMN_LABELS.col1],
		level: "heading",
		morphRoot: "heading-line",
	},
	{
		kind: "figure",
		id: CATALOGUE_EMBODIMENT_COL2_CROP,
		src: ASSET_CATALOGUE,
		alt: CATALOGUE_COLUMN_LABELS.col2,
		crop: unionTileCropForParticipants(CATALOGUE_SPLIT, CATALOGUE_COLUMN_TILE_KEYS.col2),
	},
	{
		kind: "text",
		id: CATALOGUE_EMBODIMENT_COL2_LABEL,
		lines: [CATALOGUE_COLUMN_LABELS.col2],
		level: "heading",
		morphRoot: "heading-line",
	},
	{
		kind: "figure",
		id: CATALOGUE_EMBODIMENT_COL3_CROP,
		src: ASSET_CATALOGUE,
		alt: CATALOGUE_COLUMN_LABELS.col3,
		crop: unionTileCropForParticipants(CATALOGUE_SPLIT, CATALOGUE_COLUMN_TILE_KEYS.col3),
	},
	{
		kind: "text",
		id: CATALOGUE_EMBODIMENT_COL3_LABEL,
		lines: [CATALOGUE_COLUMN_LABELS.col3],
		level: "heading",
		morphRoot: "heading-line",
	},
	{
		kind: "video",
		id: "demo-video--clip",
		src: ASSET_VIDEO,
		muted: true,
		controls: true,
	},
	{
		kind: "pdf",
		id: "thesis--doc",
		src: ASSET_THESIS_PDF,
		page: 1,
		alt: "Bachelorarbeit Ueli Saluz",
	},
	...CATALOGUE_SPLIT.embodiments,
];
//#endregion 🔖Catalogue
