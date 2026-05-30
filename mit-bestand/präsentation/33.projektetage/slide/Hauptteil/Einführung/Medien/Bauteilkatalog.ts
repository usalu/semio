import type { SlideFile } from "@framework/presentation/core";
import { CATALOGUE_SPLIT, mediaEmbodiments, mediaParticipants } from "../../../../spec.ts";

export default {
	order: 0,
	participants: mediaParticipants,
	embodiments: mediaEmbodiments,
	arrangement: {
		id: "catalogue",
		name: "Bauteilkatalog",
		dispositions: CATALOGUE_SPLIT.dispositions,
	},
	transition: { kind: "morph" },
} satisfies SlideFile;
