import type { SlideFile } from "@semio-tech/animate-present-core";
import { CATALOGUE_EMBODIMENT_FULL, CATALOGUE_FRAME, CATALOGUE_PARTICIPANT, catalogueFocusMorphTo, mediaEmbodiments, mediaParticipants } from "@semio-tech/mit-bestand-praesentation-projektetage-spec";

export default {
  order: 1,
  participants: mediaParticipants,
  embodiments: mediaEmbodiments,
  arrangement: {
    id: "catalogue",
    name: "Bauteilkatalog",
    dispositions: [
      {
        participantId: CATALOGUE_PARTICIPANT,
        embodimentId: CATALOGUE_EMBODIMENT_FULL,
        emphasis: "active",
        position: CATALOGUE_FRAME,
        morphTo: catalogueFocusMorphTo(),
      },
    ],
  },
  transition: { kind: "morph" },
} satisfies SlideFile;
