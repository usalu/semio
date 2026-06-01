import type { SlideFile } from "@framework/presentation/core";
import {
	CATALOGUE_COL1,
	CATALOGUE_COL2,
	CATALOGUE_COL3,
	CATALOGUE_COLUMN_TILE_KEYS,
	CATALOGUE_EMBODIMENT_CROP,
	CATALOGUE_FOCUS_TILES,
	CATALOGUE_PARTICIPANT,
	unionTilePosition,
} from "../../../../spec.ts";

export default {
	order: 1,
	arrangement: {
		id: "catalogue-focus",
		name: "Bauteilarten",
		settleBeforeMorphTo: ["catalogue-labels"],
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
} satisfies SlideFile;
