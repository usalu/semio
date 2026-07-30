import { figureFrameForSourceAspect, type SlideFile } from "@semio-tech/animate-present-core";

const PARTICIPANT = "eingabeprozess-eingabeoberfläche-annotiert";
const EMBODIMENT = "eingabeprozess-eingabeoberfläche-annotiert--figure";
const SOURCE_ASPECT = 746 / 659;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
  order: 2,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: EMBODIMENT,
      src: "/eingabeprozess-eingabeoberfläche-annotiert.png",
      alt: "Annotierte Eingabeoberfläche im Eingabeprozess",
      sourceAspect: SOURCE_ASPECT,
    },
  ],
  arrangement: {
    id: "eingabeprozess-eingabeoberfläche-annotiert",
    name: "Eingabeoberfläche Annotiert",
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
