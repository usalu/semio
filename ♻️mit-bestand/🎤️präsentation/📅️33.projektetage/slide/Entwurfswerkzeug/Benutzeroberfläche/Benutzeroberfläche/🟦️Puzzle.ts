import type { SlideFile } from "@semio-tech/animate-presentation-core";

const PARTICIPANT = "puzzle-3d";
const EMBODIMENT = "puzzle-3d--iframe";
const FRAME = { x: 0, y: 0, width: 1, height: 1 };

export default {
  order: 3,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "iframe",
      id: EMBODIMENT,
      src: "https://v4.3d.puzzle.semio-tech.com/",
      title: "3D Puzzle",
    },
  ],
  arrangement: {
    id: "puzzle-3d",
    name: "Puzzle",
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
