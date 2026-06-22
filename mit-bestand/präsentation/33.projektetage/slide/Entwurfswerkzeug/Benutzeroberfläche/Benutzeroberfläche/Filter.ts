import {
	figureFrameForSourceAspect,
	type SlideFile,
} from "@framework/presentation/core";

const PARTICIPANT = "entwurfswerkzeug-filter";
const EMBODIMENT = "entwurfswerkzeug-filter--figure";
const SOURCE_ASPECT = 674 / 1948;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
	order: 1,
	participants: [{ id: PARTICIPANT }],
	embodiments: [
		{
			kind: "figure",
			id: EMBODIMENT,
			src: "/entwurfswerkzeug-filter.png",
			alt: "Entwurfswerkzeug Filter",
			sourceAspect: SOURCE_ASPECT,
		},
	],
	arrangement: {
		id: "entwurfswerkzeug-filter",
		name: "Filter",
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
