import type { SlideFile } from "@framework/presentation/core";

const PARTICIPANT = "typologien-baum";
const EMBODIMENT = "typologien-baum--figure";
const FRAME = { x: 0.04, y: 0.06, width: 0.92, height: 0.88 };

export default {
	order: 0,
	participants: [{ id: PARTICIPANT }],
	embodiments: [
		{
			kind: "figure",
			id: EMBODIMENT,
			src: "/typologienbaum.png",
			alt: "Generator-Typologiebaum",
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
