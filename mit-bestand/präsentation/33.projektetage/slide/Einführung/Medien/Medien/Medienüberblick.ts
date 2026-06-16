import type { SlideFile } from "@framework/presentation/core";
import { CATALOGUE_EMBODIMENT_FULL, CATALOGUE_PARTICIPANT } from "../../../../spec.ts";

export default {
	order: 3,
	arrangement: {
		id: "media-suite",
		name: "Medienüberblick",
		dispositions: [
			{
				participantId: CATALOGUE_PARTICIPANT,
				embodimentId: CATALOGUE_EMBODIMENT_FULL,
				emphasis: "muted",
				position: { x: 0.02, y: 0.05, width: 0.3, height: 0.35 },
			},
			{
				participantId: "demo-video",
				embodimentId: "demo-video--clip",
				emphasis: "active",
				position: { x: 0.35, y: 0.1, width: 0.6, height: 0.5 },
			},
			{
				participantId: "thesis",
				embodimentId: "thesis--doc",
				emphasis: "active",
				position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
			},
		],
	},
} satisfies SlideFile;
