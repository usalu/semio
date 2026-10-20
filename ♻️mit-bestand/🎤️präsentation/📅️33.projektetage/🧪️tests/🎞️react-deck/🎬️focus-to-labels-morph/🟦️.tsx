/** 🎬️ The catalogue focus to inline column labels auto animate test. */
import { it, expect } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { elementIsInteractiveFigureDisposition, elementIsSourceGhostAnchor, elementIsTargetGhostAnchor, mountPresentation, prepareArrangementBeforeAutoAnimate, presentationAutoAnimateMatcher } from "@semio-tech/presentation-react";
import type { Presentation, AutoAnimateMatcherHost } from "@semio-tech/presentation-react";

/** 🏷 Registers the catalogue focus to inline column labels auto animate test. */
export function registerFocusToLabelsMorphTests(): void {
it("auto-animates catalogue focus into inline column labels", async () => {
      const {
        CATALOGUE_COL1,
        CATALOGUE_COL2,
        CATALOGUE_COL3,
        CATALOGUE_EMBODIMENT_COL1_LABEL,
        CATALOGUE_EMBODIMENT_COL2_LABEL,
        CATALOGUE_EMBODIMENT_COL3_LABEL,
        CATALOGUE_COLUMN_TILE_KEYS,
        CATALOGUE_SPLIT,
        catalogueFocusDispositions,
        columnLabelMorphFrom,
        inlineColumnLabelPosition,
        mediaEmbodiments,
        mediaParticipants,
      } = await import("@semio-tech/mit-bestand-praesentation-projektetage");
      const deck: Presentation = {
        id: "projektetage-morph",
        name: "Morph",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "medien",
                    participants: mediaParticipants,
                    embodiments: mediaEmbodiments,
                    slides: [
                      {
                        arrangement: {
                          id: "catalogue",
                          dispositions: CATALOGUE_SPLIT.dispositions,
                        },
                        transition: { kind: "morph" },
                      },
                      {
                        arrangement: {
                          id: "catalogue-focus",
                          settleBeforeMorphTo: ["catalogue-labels"],
                          dispositions: catalogueFocusDispositions(),
                        },
                        transition: { kind: "morph" },
                      },
                      {
                        arrangement: {
                          id: "catalogue-labels",
                          dispositions: [
                            {
                              participantId: CATALOGUE_COL1,
                              embodimentId: CATALOGUE_EMBODIMENT_COL1_LABEL,
                              emphasis: "active",
                              position: inlineColumnLabelPosition(0),
                              morphFrom: columnLabelMorphFrom("col1", inlineColumnLabelPosition(0)),
                            },
                            {
                              participantId: CATALOGUE_COL2,
                              embodimentId: CATALOGUE_EMBODIMENT_COL2_LABEL,
                              emphasis: "active",
                              position: inlineColumnLabelPosition(1),
                              morphFrom: columnLabelMorphFrom("col2", inlineColumnLabelPosition(1)),
                            },
                            {
                              participantId: CATALOGUE_COL3,
                              embodimentId: CATALOGUE_EMBODIMENT_COL3_LABEL,
                              emphasis: "active",
                              position: inlineColumnLabelPosition(2),
                              morphFrom: columnLabelMorphFrom("col3", inlineColumnLabelPosition(2)),
                            },
                          ],
                        },
                        transition: { kind: "morph" },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const revealEl = mountRoot.querySelector(".reveal") as HTMLElement;
      const focusSlide = revealEl.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const labelSlide = revealEl.querySelector('section[title="catalogue-labels"]') as HTMLElement;
      expect(focusSlide).toBeTruthy();
      expect(labelSlide).toBeTruthy();
      expect(focusSlide.getAttribute("data-auto-animate-id")).toBe(labelSlide.getAttribute("data-auto-animate-id"));
      prepareArrangementBeforeAutoAnimate(focusSlide, labelSlide);
      expect(focusSlide.classList.contains("presentation-arrangement--settled")).toBe(true);
      const pairs = presentationAutoAnimateMatcher.call({ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost, focusSlide, labelSlide);
      expect(pairs.every((pair) => !elementIsSourceGhostAnchor(pair.from))).toBe(true);
      expect(pairs.every((pair) => !elementIsSourceGhostAnchor(pair.to))).toBe(true);
      expect(pairs.every((pair) => elementIsTargetGhostAnchor(pair.to))).toBe(true);
      expect(pairs.every((pair) => elementIsInteractiveFigureDisposition(pair.from))).toBe(true);
      const columnIds = [CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3];
      const tileIds = [...CATALOGUE_COLUMN_TILE_KEYS.col1, ...CATALOGUE_COLUMN_TILE_KEYS.col2, ...CATALOGUE_COLUMN_TILE_KEYS.col3];
      expect(pairs.length).toBeGreaterThanOrEqual(8);
      expect(tileIds.filter((id) => pairs.some((pair) => pair.from.getAttribute("data-id") === id && elementIsTargetGhostAnchor(pair.to))).length).toBeGreaterThanOrEqual(8);
      expect(labelSlide.querySelectorAll(".presentation-interactive-disposition.presentation-morph-target").length).toBe(3);
      expect(labelSlide.querySelectorAll(".presentation-interactive-disposition.presentation-morph-target[data-id]").length).toBe(0);
      expect(tileIds.every((id) => labelSlide.querySelector(`[data-id="${id}"]`)?.closest(".presentation-target-ghost") !== null)).toBe(true);
      expect(labelSlide.querySelectorAll(".presentation-target-ghost").length).toBeGreaterThanOrEqual(8);
      const labelSlot = inlineColumnLabelPosition(2);
      const focusStuetze = focusSlide.querySelector('[data-disposition-id^="catalogue-focus--Stütze"]') as HTMLElement | null;
      const labelGhost = labelSlide.querySelector('[data-disposition-id^="catalogue-labels--Stütze"].presentation-target-ghost') as HTMLElement | null;
      expect(focusStuetze?.style.left).not.toBe(`${labelSlot.x * 100}%`);
      expect(labelGhost?.style.left).toBe(`${labelSlot.x * 100}%`);
      expect(labelGhost?.style.top).toBe(`${labelSlot.y * 100}%`);
      expect(labelGhost?.style.width).toBe(`${labelSlot.width * 100}%`);
      expect(labelGhost?.style.height).toBe(`${labelSlot.height * 100}%`);
      expect(labelGhost?.classList.contains("presentation-target-ghost")).toBe(true);
      expect(labelGhost?.getAttribute("data-disposition-id")).toContain("catalogue-labels");
      mountRoot.remove();
    });
}
