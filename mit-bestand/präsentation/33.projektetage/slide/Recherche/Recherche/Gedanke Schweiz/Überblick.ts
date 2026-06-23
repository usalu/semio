import {
	figureFrameForSourceAspect,
	type SlideFile,
} from "@semio-tech/framework-presentation-core";

const PARTICIPANT = "recherche-schweiz-überblick";
const EMBODIMENT = "recherche-schweiz-überblick--figure";
const SOURCE_ASPECT = 1987 / 1015;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
	order: 0,
	participants: [{ id: PARTICIPANT }],
	embodiments: [
		{
			kind: "figure",
			id: EMBODIMENT,
			src: "/recherche-schweiz-überblick.png",
			alt: "Recherche Schweiz Überblick",
			sourceAspect: SOURCE_ASPECT,
		},
	],
	arrangement: {
		id: "recherche-schweiz-überblick",
		name: "Überblick",
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
