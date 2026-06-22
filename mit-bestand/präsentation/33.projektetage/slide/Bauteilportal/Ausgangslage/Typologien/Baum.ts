import {
	figureFrameForSourceAspect,
	type SlideFile,
} from "@framework/presentation/core";

const PARTICIPANT = "typologien-baum";
const EMBODIMENT = "typologien-baum--figure";
const SOURCE_ASPECT = 1536 / 1024;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
	order: 1,
	participants: [{ id: PARTICIPANT }],
	embodiments: [
		{
			kind: "figure",
			id: EMBODIMENT,
			src: "/typologienbaum.png",
			alt: "Generator-Typologiebaum",
			sourceAspect: SOURCE_ASPECT,
		},
	],
	arrangement: {
		id: "typologien-baum",
		name: "Baum",
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
