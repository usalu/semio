import {
	figureFrameForSourceAspect,
	type SlideFile,
} from "@framework/presentation/core";

const PARTICIPANT = "recherche-schweiz-zoom-in-1";
const EMBODIMENT = "recherche-schweiz-zoom-in-1--figure";
const SOURCE_ASPECT = 1984 / 1014;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
	order: 1,
	participants: [{ id: PARTICIPANT }],
	embodiments: [
		{
			kind: "figure",
			id: EMBODIMENT,
			src: "/recherche-schweiz-zoom-in-1.png",
			alt: "Recherche Schweiz Zoom In 1",
			sourceAspect: SOURCE_ASPECT,
		},
	],
	arrangement: {
		id: "recherche-schweiz-zoom-in-1",
		name: "Zoom In 1",
		dispositions: [
			{
				participantId: PARTICIPANT,
				embodimentId: EMBODIMENT,
				emphasis: "active",
				position: FRAME,
			},
		],
	},
} satisfies SlideFile;
