/** 🧰️ The shared projektetage deck test kit: the destructured deck module exports, the vitest functions and the bauteilbörsen thought lookup. */

import type * as presentation from "@semio-tech/presentation";
import type { Presentation, Thought } from "@semio-tech/presentation";
import type * as spec from "../../../🔖️spec/🟦️.ts";

export type TestSource = { readonly directory: string; readonly url: string };

/** 💉 The exact members the deck module injects: the presentation engine's functions, the spec's constants and the assembled deck. */
export type ProjektetageDeckDependencies = Pick<typeof presentation, "PRESENTATION_DEFAULT_SLIDE_ASPECT" | "arrangementRestDispositions" | "buildResolutionScope" | "collectPresentationSlides" | "countArrangements" | "expandThoughtSlides" | "resolveArrangement"> & Pick<typeof spec, "BAUKOMPONENTEN_ITEMS" | "CATALOGUE_COL1" | "CATALOGUE_COL2" | "CATALOGUE_COL3" | "ZUKUNFT_BAU_EMBODIMENT" | "ZUKUNFT_BAU_FRAME" | "ZUKUNFT_BAU_PARTICIPANT" | "columnLabelMorphFrom" | "inlineColumnLabelPosition"> & Readonly<{ INTRO_TITLE_MORPH_FRAME: Readonly<{ x: number; y: number; width: number; height: number }>; deck: Presentation }>;

export function bauteilboersenThought(presentation: Presentation): Thought | undefined {
  return presentation.chapters
    .find((chapter) => chapter.name === "Bauteilportal")
    ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
    ?.thoughts.find((thought) => thought.name === "Bauteilbörsen");
}

/** 🏭 Creates the shared deck test kit from the injected dependencies and the vitest functions. */
export function createProjektetageDeckTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ProjektetageDeckDependencies, source: TestSource) {
  const { BAUKOMPONENTEN_ITEMS, CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3, INTRO_TITLE_MORPH_FRAME, PRESENTATION_DEFAULT_SLIDE_ASPECT, ZUKUNFT_BAU_EMBODIMENT, ZUKUNFT_BAU_FRAME, ZUKUNFT_BAU_PARTICIPANT, arrangementRestDispositions, buildResolutionScope, collectPresentationSlides, columnLabelMorphFrom, countArrangements, deck, expandThoughtSlides, inlineColumnLabelPosition, resolveArrangement } = dependencies;
  const { describe, expect, it } = vitest;
  return { BAUKOMPONENTEN_ITEMS, CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3, INTRO_TITLE_MORPH_FRAME, PRESENTATION_DEFAULT_SLIDE_ASPECT, ZUKUNFT_BAU_EMBODIMENT, ZUKUNFT_BAU_FRAME, ZUKUNFT_BAU_PARTICIPANT, arrangementRestDispositions, buildResolutionScope, collectPresentationSlides, columnLabelMorphFrom, countArrangements, deck, describe, expandThoughtSlides, expect, inlineColumnLabelPosition, it, resolveArrangement, source };
}
export type ProjektetageDeckTestKit = ReturnType<typeof createProjektetageDeckTestKit>;
