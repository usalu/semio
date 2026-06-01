import type { SlideFile } from "@framework/presentation/core";
import {
	CATALOGUE_COL1,
	CATALOGUE_COL2,
	CATALOGUE_COL3,
	CATALOGUE_EMBODIMENT_LABEL,
	stackedColumnLabelPosition,
} from "../../../../spec.ts";

export default {
	order: 2,
	arrangement: {
		id: "catalogue-labels",
		name: "Bauteilbeschriftungen",
		dispositions: [
			{
				participantId: CATALOGUE_COL1,
				embodimentId: CATALOGUE_EMBODIMENT_LABEL,
				emphasis: "active",
				position: stackedColumnLabelPosition(0),
			},
			{
				participantId: CATALOGUE_COL2,
				embodimentId: CATALOGUE_EMBODIMENT_LABEL,
				emphasis: "active",
				position: stackedColumnLabelPosition(1),
			},
			{
				participantId: CATALOGUE_COL3,
				embodimentId: CATALOGUE_EMBODIMENT_LABEL,
				emphasis: "active",
				position: stackedColumnLabelPosition(2),
			},
		],
	},
	transition: { kind: "morph" },
} satisfies SlideFile;
