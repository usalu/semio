import type { SlideFile } from "@framework/presentation/core";
import {
	CATALOGUE_COL1,
	CATALOGUE_COL2,
	CATALOGUE_COL3,
	CATALOGUE_EMBODIMENT_CROP,
	CATALOGUE_EMBODIMENT_STACK,
	CATALOGUE_LABELS,
	CATALOGUE_LABEL_STACK_FRAME,
	stackedColumnLabelPosition,
} from "../../../../spec.ts";

export default {
	order: 2,
	arrangement: {
		id: "catalogue-labels",
		name: "Bauteilbeschriftungen",
		dispositions: [
			{
				participantId: CATALOGUE_LABELS,
				embodimentId: CATALOGUE_EMBODIMENT_STACK,
				emphasis: "active",
				position: CATALOGUE_LABEL_STACK_FRAME,
				morphFrom: [
					{
						participantId: CATALOGUE_COL1,
						embodimentId: CATALOGUE_EMBODIMENT_CROP,
						position: stackedColumnLabelPosition(0),
						targetLineIndex: 0,
					},
					{
						participantId: CATALOGUE_COL2,
						embodimentId: CATALOGUE_EMBODIMENT_CROP,
						position: stackedColumnLabelPosition(1),
						targetLineIndex: 1,
					},
					{
						participantId: CATALOGUE_COL3,
						embodimentId: CATALOGUE_EMBODIMENT_CROP,
						position: stackedColumnLabelPosition(2),
						targetLineIndex: 2,
					},
				],
			},
		],
	},
	transition: { kind: "morph" },
} satisfies SlideFile;
