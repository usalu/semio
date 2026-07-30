import { type SlideFile } from "@semio-tech/animate-present-core";

const PARTICIPANT = "eingabeprozess-output";
const EMBODIMENT = "eingabeprozess-output--json";
const FRAME = { x: 0.04, y: 0.06, width: 0.92, height: 0.88 };

export default {
  order: 6,
  participants: [{ id: PARTICIPANT }],
  embodiments: [
    {
      kind: "json",
      id: EMBODIMENT,
      src: "/🔣eingabeprozess-output.json",
      title: "Eingabeprozess Output",
    },
  ],
  arrangement: {
    id: "eingabeprozess-output",
    name: "Output",
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
