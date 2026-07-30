import { figureFrameForSourceAspect, type SlideFile } from "@semio-tech/animate-present-core";

const PARTICIPANT = "typologien-katalog";
const EMBODIMENT = "typologien-katalog--figure";
const SOURCE_ASPECT = 1264 / 713;
const FRAME = figureFrameForSourceAspect(SOURCE_ASPECT);

export default {
  order: 2,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: EMBODIMENT,
      src: "/🖼️katalog.png",
      alt: "Typologien-Katalog",
      sourceAspect: SOURCE_ASPECT,
    },
  ],
  arrangement: {
    id: "typologien-katalog",
    name: "Katalog",
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
