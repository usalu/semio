import type { SlideFile } from "@semio-tech/animate-presentation-core";
import {
  CATALOGUE_COL1,
  CATALOGUE_COL2,
  CATALOGUE_COL3,
  CATALOGUE_EMBODIMENT_COL1_LABEL,
  CATALOGUE_EMBODIMENT_COL2_LABEL,
  CATALOGUE_EMBODIMENT_COL3_LABEL,
  columnLabelMorphFrom,
  inlineColumnLabelPosition,
  mediaEmbodiments,
  mediaParticipants,
} from "@semio-tech/mit-bestand-praesentation-projektetage-spec";

export default {
  order: 3,
  participants: mediaParticipants,
  embodiments: mediaEmbodiments,
  arrangement: {
    id: "catalogue-labels",
    name: "Bauteilbeschriftungen",
    dispositions: [
      {
        participantId: CATALOGUE_COL1,
        embodimentId: CATALOGUE_EMBODIMENT_COL1_LABEL,
        emphasis: "active",
        position: inlineColumnLabelPosition(0),
        morphFrom: columnLabelMorphFrom("col1", inlineColumnLabelPosition(0)),
      },
      {
        participantId: CATALOGUE_COL2,
        embodimentId: CATALOGUE_EMBODIMENT_COL2_LABEL,
        emphasis: "active",
        position: inlineColumnLabelPosition(1),
        morphFrom: columnLabelMorphFrom("col2", inlineColumnLabelPosition(1)),
      },
      {
        participantId: CATALOGUE_COL3,
        embodimentId: CATALOGUE_EMBODIMENT_COL3_LABEL,
        emphasis: "active",
        position: inlineColumnLabelPosition(2),
        morphFrom: columnLabelMorphFrom("col3", inlineColumnLabelPosition(2)),
      },
    ],
  },
  transition: { kind: "morph" },
} satisfies SlideFile;
