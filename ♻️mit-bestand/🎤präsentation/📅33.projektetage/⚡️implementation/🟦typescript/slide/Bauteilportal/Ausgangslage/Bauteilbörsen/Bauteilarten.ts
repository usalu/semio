import type { SlideFile } from "@semio-tech/animate-present-core";
import { catalogueFocusDispositions, mediaEmbodiments, mediaParticipants } from "@semio-tech/mit-bestand-praesentation-projektetage-spec";

export default {
  order: 2,
  arrangement: {
    id: "catalogue-focus",
    name: "Bauteilarten",
    settleBeforeMorphTo: ["catalogue-labels"],
    dispositions: catalogueFocusDispositions(),
  },
  transition: { kind: "morph" },
  participants: mediaParticipants,
  embodiments: mediaEmbodiments,
} satisfies SlideFile;
