import type { SlideFile } from "@framework/presentation/core";
import {
	CATALOGUE_PARTICIPANT,
	CATALOGUE_TILES_ASSEMBLED,
	mediaParticipants,
} from "../../../../spec.ts";

export default {
	order: 0,
	participants: mediaParticipants,
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
} satisfies SlideFile;
