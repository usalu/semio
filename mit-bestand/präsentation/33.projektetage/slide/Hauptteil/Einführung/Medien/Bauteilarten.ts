import type { SlideFile } from "@framework/presentation/core";
import {
	CATALOGUE_COL1,
	CATALOGUE_COL2,
	CATALOGUE_COL3,
	catalogueFocusTilesForColumn,
	CATALOGUE_PARTICIPANT,
	CATALOGUE_FOCUS_TILES,
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
				emphasis: "active",
				split: { tiles: catalogueFocusTilesForColumn("col1"), morphParticipant: true },
			},
			{
				participantId: CATALOGUE_COL2,
				emphasis: "active",
				split: { tiles: catalogueFocusTilesForColumn("col2"), morphParticipant: true },
			},
			{
				participantId: CATALOGUE_COL3,
				emphasis: "active",
				split: { tiles: catalogueFocusTilesForColumn("col3"), morphParticipant: true },
			},
		],
	},
	transition: { kind: "morph" },
} satisfies SlideFile;
