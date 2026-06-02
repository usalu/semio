import type { SlideFile } from "@framework/presentation/core";
import { catalogueFocusDispositions, mediaEmbodiments, mediaParticipants } from "../../../../spec.ts";

export default {
	order: 1,
	arrangement: {
		id: "catalogue-focus",
		name: "Bauteilarten",
		dispositions: catalogueFocusDispositions(),
	},
	transition: { kind: "morph" },
	participants: mediaParticipants,
	embodiments: mediaEmbodiments,
} satisfies SlideFile;
