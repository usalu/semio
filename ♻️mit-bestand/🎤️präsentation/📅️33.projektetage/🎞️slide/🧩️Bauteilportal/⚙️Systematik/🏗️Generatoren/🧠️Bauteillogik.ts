import type { SlideFile } from "@semio-tech/animate-presentation-core";

const PARTICIPANT = "procedural-bauteillogik";
const EMBODIMENT = "procedural-bauteillogik--iframe";
const FRAME = { x: 0, y: 0, width: 1, height: 1 };

export default {
  order: 0,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "iframe",
      id: EMBODIMENT,
      src: "https://v4.procedural.semio-tech.com/",
      title: "Procedural Bauteillogik",
    },
  ],
  arrangement: {
    id: "bauteillogik",
    name: "Bauteillogik",
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
