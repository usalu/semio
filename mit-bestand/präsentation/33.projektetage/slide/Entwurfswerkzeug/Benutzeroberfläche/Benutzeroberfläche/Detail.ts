import {
	figureFrameForSourceAspect,
	type SlideFile,
} from "@semio-tech/framework-presentation-core";

const PARTICIPANT = "entwurfswerkzeug-detail";
const EMBODIMENT = "entwurfswerkzeug-detail--figure";
const SOURCE_ASPECT = 674 / 1948;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
	order: 2,
	participants: [{ id: PARTICIPANT }],
	embodiments: [
		{
			kind: "figure",
			id: EMBODIMENT,
			src: "/entwurfswerkzeug-detail.png",
			alt: "Entwurfswerkzeug Detailansicht",
			sourceAspect: SOURCE_ASPECT,
		},
	],
	arrangement: {
		id: "entwurfswerkzeug-detail",
		name: "Detail",
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
