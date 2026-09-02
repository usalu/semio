import { figureFrameForSourceAspect, type SlideFile } from "@semio-tech/animate-presentation-core";

const PARTICIPANT = "eingabeprozess-manuelles-prüfen";
const EMBODIMENT = "eingabeprozess-manuelles-prüfen--figure";
const SOURCE_ASPECT = 860 / 1183;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
  order: 3,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: EMBODIMENT,
      src: "/🖼️eingabeprozess-formular.png",
      alt: "Manuelles Prüfen im Eingabeprozess",
      sourceAspect: SOURCE_ASPECT,
    },
  ],
  arrangement: {
    id: "eingabeprozess-manuelles-prüfen",
    name: "Manuelles Prüfen",
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
