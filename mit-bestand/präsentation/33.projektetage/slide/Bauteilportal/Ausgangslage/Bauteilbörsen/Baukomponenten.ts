import type { SlideFile } from "@semio-tech/framework-presentation-core";
import { baukomponentenGridArtifacts } from "@semio-tech/mit-bestand-praesentation-projektetage-spec";

const { participants, embodiments, dispositions } = baukomponentenGridArtifacts();

export default {
  order: 0,
  participants,
  embodiments,
  arrangement: {
    id: "baukomponenten",
    name: "Baukomponenten",
    dispositions,
  },
} satisfies SlideFile;
