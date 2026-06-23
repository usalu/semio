import {
	figureFrameForSourceAspect,
	type SlideFile,
} from "@semio-tech/framework-presentation-core";

const PARTICIPANT = "entwurfswerkzeug-katalog";
const EMBODIMENT = "entwurfswerkzeug-katalog--figure";
const SOURCE_ASPECT = 688 / 1948;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
	order: 0,
	participants: [{ id: PARTICIPANT }],
	embodiments: [
		{
			kind: "figure",
			id: EMBODIMENT,
			src: "/entwurfswerkzeug-katalog.png",
			alt: "Entwurfswerkzeug Katalog",
			sourceAspect: SOURCE_ASPECT,
		},
	],
	arrangement: {
		id: "entwurfswerkzeug-katalog",
		name: "Katalog",
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
