import { figureFrameForSourceAspect, type SlideFile } from "@semio-tech/animate-presentation-core";

const PARTICIPANT = "eingabeprozess-eingabeoberfläche";
const EMBODIMENT = "eingabeprozess-eingabeoberfläche--figure";
const SOURCE_ASPECT = 2130 / 1670;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
  order: 1,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: EMBODIMENT,
      src: "/🖼️eingabeprozess-eingabeoberfläche.png",
      alt: "Eingabeoberfläche im Eingabeprozess",
      sourceAspect: SOURCE_ASPECT,
    },
  ],
  arrangement: {
    id: "eingabeprozess-eingabeoberfläche",
    name: "Eingabeoberfläche",
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
