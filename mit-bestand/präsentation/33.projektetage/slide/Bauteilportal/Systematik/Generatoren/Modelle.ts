import type { SlideFile } from "@semio-tech/framework-presentation-core";

const PARTICIPANT = "cad-modelle";
const EMBODIMENT = "cad-modelle--iframe";
const FRAME = { x: 0, y: 0, width: 1, height: 1 };

export default {
  order: 1,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "iframe",
      id: EMBODIMENT,
      src: "https://v4.cad.semio-tech.com/",
      title: "CAD Modelle",
    },
  ],
  arrangement: {
    id: "modelle",
    name: "Modelle",
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
