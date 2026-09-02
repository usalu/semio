import { figureFrameForSourceAspect, type SlideFile } from "@semio-tech/animate-presentation-core";

const PARTICIPANT = "recherche-schweiz-zoom-in-2";
const EMBODIMENT = "recherche-schweiz-zoom-in-2--figure";
const SOURCE_ASPECT = 1988 / 1018;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
  order: 2,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: EMBODIMENT,
      src: "/🖼️recherche-schweiz-zoom-in-2.png",
      alt: "Recherche Schweiz Zoom In 2",
      sourceAspect: SOURCE_ASPECT,
    },
  ],
  arrangement: {
    id: "recherche-schweiz-zoom-in-2",
    name: "Zoom In 2",
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
