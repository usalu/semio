import { figureFrameForSourceAspect, type SlideFile } from "@semio-tech/animate-presentation-core";

const PARTICIPANT = "eingabeprozess-eingabearten";
const EMBODIMENT = "eingabeprozess-eingabearten--figure";
const SOURCE_ASPECT = 3586 / 1346;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
  order: 0,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: EMBODIMENT,
      src: "/🔢️eingabeprozess-eingabearten.png",
      alt: "Eingabearten im Eingabeprozess",
      sourceAspect: SOURCE_ASPECT,
    },
  ],
  arrangement: {
    id: "eingabeprozess-eingabearten",
    name: "Eingabearten",
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
