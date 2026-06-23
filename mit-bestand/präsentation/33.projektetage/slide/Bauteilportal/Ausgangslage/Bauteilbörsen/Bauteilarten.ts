import type { SlideFile } from "@semio-tech/framework-presentation-core";
import { catalogueFocusDispositions, mediaEmbodiments, mediaParticipants } from "../../../../spec.ts";

export default {
	order: 2,
	arrangement: {
		id: "catalogue-focus",
		name: "Bauteilarten",
		settleBeforeMorphTo: ["catalogue-labels"],
		dispositions: catalogueFocusDispositions(),
	},
	transition: { kind: "morph" },
	participants: mediaParticipants,
	embodiments: mediaEmbodiments,
} satisfies SlideFile;
