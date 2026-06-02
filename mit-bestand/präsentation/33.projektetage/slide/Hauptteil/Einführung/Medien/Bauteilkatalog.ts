import type { SlideFile } from "@framework/presentation/core";
import {
	CATALOGUE_EMBODIMENT_FULL,
	CATALOGUE_FRAME,
	CATALOGUE_PARTICIPANT,
	CATALOGUE_SPLIT,
	mediaEmbodiments,
	mediaParticipants,
} from "../../../../spec.ts";

export default {
	order: 0,
	participants: mediaParticipants,
	embodiments: mediaEmbodiments,
	arrangement: {
		id: "catalogue",
		name: "Bauteilkatalog",
		settleBeforeMorphTo: ["catalogue-focus"],
		dispositions: [
			{
				participantId: CATALOGUE_PARTICIPANT,
				embodimentId: CATALOGUE_EMBODIMENT_FULL,
				emphasis: "active",
				position: CATALOGUE_FRAME,
			},
			...CATALOGUE_SPLIT.dispositions.map((disposition) => ({
				...disposition,
				style: { opacity: 0 },
			})),
		],
	},
	transition: { kind: "morph" },
} satisfies SlideFile;
