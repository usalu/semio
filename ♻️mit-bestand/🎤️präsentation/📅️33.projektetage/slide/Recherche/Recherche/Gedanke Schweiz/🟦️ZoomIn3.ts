import { figureFrameForSourceAspect, type SlideFile } from "@semio-tech/animate-presentation-core";

const PARTICIPANT = "recherche-schweiz-zoom-in-3";
const EMBODIMENT = "recherche-schweiz-zoom-in-3--figure";
const SOURCE_ASPECT = 1981 / 1017;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
  order: 3,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: EMBODIMENT,
      src: "/🖼️recherche-schweiz-zoom-in-3.png",
      alt: "Recherche Schweiz Zoom In 3",
      sourceAspect: SOURCE_ASPECT,
    },
  ],
  arrangement: {
    id: "recherche-schweiz-zoom-in-3",
    name: "Zoom In 3",
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
