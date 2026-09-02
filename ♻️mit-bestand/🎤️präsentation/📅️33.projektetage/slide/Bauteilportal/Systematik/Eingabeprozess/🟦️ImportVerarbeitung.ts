import { figureFrameForSourceAspect, type SlideFile } from "@semio-tech/animate-presentation-core";

const PARTICIPANT = "eingabeprozess-import-verarbeitung";
const EMBODIMENT = "eingabeprozess-import-verarbeitung--figure";
const SOURCE_ASPECT = 1278 / 1288;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
  order: 5,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: EMBODIMENT,
      src: "/🖼️import-verarbeitung.png",
      alt: "Import Verarbeitung im Eingabeprozess",
      sourceAspect: SOURCE_ASPECT,
    },
  ],
  arrangement: {
    id: "eingabeprozess-import-verarbeitung",
    name: "Import Verarbeitung",
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
