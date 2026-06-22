import {
	figureFrameForSourceAspect,
	type SlideFile,
} from "@framework/presentation/core";

const PARTICIPANT = "typologien";
const EMBODIMENT = "typologien--figure";
const SOURCE_ASPECT = 984 / 1448;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
	order: 0,
	participants: [{ id: PARTICIPANT }],
	embodiments: [
		{
			kind: "figure",
			id: EMBODIMENT,
			src: "/typologien.png",
			alt: "Typologien-Katalog",
			sourceAspect: SOURCE_ASPECT,
		},
	],
	arrangement: {
		id: "typologien",
		name: "Typologien",
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
