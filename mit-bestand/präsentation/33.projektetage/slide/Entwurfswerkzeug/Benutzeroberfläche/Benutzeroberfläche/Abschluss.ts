import type { SlideFile } from "@semio-tech/framework-presentation-core";

const HEADING_PARTICIPANT = "abschluss-heading";
const HEADING_EMBODIMENT = "abschluss-heading--text";
const SPONSORSHIP_PARTICIPANT = "abschluss-sponsorship";
const SPONSORSHIP_EMBODIMENT = "abschluss-sponsorship--text";

export default {
	order: 4,
	participants: [{ id: HEADING_PARTICIPANT }, { id: SPONSORSHIP_PARTICIPANT }],
	embodiments: [
		{
			kind: "text",
			id: HEADING_EMBODIMENT,
			lines: ["Vielen Dank für Ihre Aufmerksamkeit!"],
			level: "heading",
		},
		{
			kind: "text",
			id: SPONSORSHIP_EMBODIMENT,
			lines: [
				"Dieses Projekt wird gefördert vom Bundesinstitut für Bau-, Stadt- und Raumforschung im Auftrag des Bundesministeriums für Wohnen, Stadtentwicklung und Bauwesen aus Mitteln der Zukunft Bau Forschungsförderung.",
			],
			level: "body",
		},
	],
	arrangement: {
		id: "abschluss",
		name: "Abschluss",
		dispositions: [
			{
				participantId: HEADING_PARTICIPANT,
				embodimentId: HEADING_EMBODIMENT,
				emphasis: "active",
			},
			{
				participantId: SPONSORSHIP_PARTICIPANT,
				embodimentId: SPONSORSHIP_EMBODIMENT,
				emphasis: "active",
			},
		],
	},
} satisfies SlideFile;
