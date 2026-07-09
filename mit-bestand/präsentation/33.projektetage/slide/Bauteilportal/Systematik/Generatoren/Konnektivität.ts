import { PRESENTATION_DEFAULT_SLIDE_ASPECT, type DispositionPosition, type SlideFile } from "@semio-tech/framework-presentation-core";

const FIGURE_PARTICIPANT = "konnektivität-beispiel-3d";
const FIGURE_EMBODIMENT = "konnektivität-beispiel-3d--figure";
const TABLE_PARTICIPANT = "konnektivität-beispiel-tabelle";
const TABLE_EMBODIMENT = "konnektivität-beispiel-tabelle--markdown";
const SOURCE_ASPECT = 1760 / 1320;

/** @emoji 📐 Largest centered frame in the left half matching {@link SOURCE_ASPECT}. */
function leftHalfFigureFrame(sourceAspect: number): DispositionPosition {
  const paddingX = 0.02;
  const paddingY = 0.06;
  const halfMaxWidth = 0.5 - paddingX * 2;
  const maxHeight = 1 - paddingY * 2;
  let height = maxHeight;
  let width = (height * sourceAspect) / PRESENTATION_DEFAULT_SLIDE_ASPECT;
  if (width > halfMaxWidth) {
    width = halfMaxWidth;
    height = (width * PRESENTATION_DEFAULT_SLIDE_ASPECT) / sourceAspect;
  }
  return {
    x: paddingX + (halfMaxWidth - width) / 2,
    y: (1 - height) / 2,
    width,
    height,
  };
}

/** @emoji 📐 Right-half frame with the same padding as {@link leftHalfFigureFrame}. */
function rightHalfFrame(): DispositionPosition {
  const paddingX = 0.02;
  const paddingY = 0.06;
  return {
    x: 0.5 + paddingX,
    y: paddingY,
    width: 0.5 - paddingX * 2,
    height: 1 - paddingY * 2,
  };
}

const TABLE_FRAME = rightHalfFrame();

export default {
  order: 2,
  participants: [{ id: FIGURE_PARTICIPANT }, { id: TABLE_PARTICIPANT }],
  embodiments: [
    {
      kind: "figure",
      id: FIGURE_EMBODIMENT,
      src: "/konnektivität-beispiel-3d.png",
      alt: "Konnektivitätsbeispiel in 3D mit Port- und Connector-Annotationen",
      sourceAspect: SOURCE_ASPECT,
    },
    {
      kind: "markdown",
      id: TABLE_EMBODIMENT,
      src: "/konnektivität-beispiel-tabelle.md",
      title: "Konnektivitätstabelle",
    },
  ],
  arrangement: {
    id: "konnektivität",
    name: "Konnektivität",
    dispositions: [
      {
        participantId: FIGURE_PARTICIPANT,
        embodimentId: FIGURE_EMBODIMENT,
        emphasis: "active",
        position: leftHalfFigureFrame(SOURCE_ASPECT),
      },
      {
        participantId: TABLE_PARTICIPANT,
        embodimentId: TABLE_EMBODIMENT,
        emphasis: "active",
        position: TABLE_FRAME,
      },
    ],
  },
} satisfies SlideFile;
