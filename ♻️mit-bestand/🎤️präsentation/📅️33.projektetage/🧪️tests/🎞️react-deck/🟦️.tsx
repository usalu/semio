// @vitest-environment jsdom
import { describe, it, expect, afterEach } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import Reveal from "reveal.js";
import { elementIsInteractiveFigureDisposition, elementIsSourceGhostAnchor, elementIsTargetGhostAnchor, mountPresentation, prepareArrangementBeforeAutoAnimate, presentationAutoAnimateMatcher, unmountPresentation } from "@semio-tech/presentation-react";
import type { Presentation, AutoAnimateMatcherHost } from "@semio-tech/presentation-react";
import { intro, splitFigureGrid, tile } from "@semio-tech/presentation";
afterEach(() => { unmountPresentation(); document.body.innerHTML = ""; });
describe("Projektetage presentation interaction", () => {
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

it("shows catalogue full figure at rest with source ghosts and focus tiles visible", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/presentation");
      const { deck } = await import("@semio-tech/mit-bestand-praesentation-projektetage");
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
      expect(catalogueSlide.classList.contains("presentation-arrangement--settled")).toBe(false);
      expect(catalogueSlide.hasAttribute("data-settle-before-morph-to")).toBe(false);
      const catalogueFigure = catalogueSlide.querySelector('.presentation-morph-one .presentation-morph-slot--figure[role="img"]') as HTMLElement | null;
      expect(catalogueFigure?.style.backgroundImage).toContain("bauteilb");
      expect(catalogueSlide.querySelectorAll(".presentation-morph-one").length).toBe(1);
      const sourceGhosts = catalogueSlide.querySelectorAll(".presentation-interactive-disposition.presentation-source-ghost");
      expect(sourceGhosts.length).toBe(10);
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const focusSlots = focusSlide.querySelectorAll(".presentation-morph-slot--figure");
      expect(focusSlots.length).toBe(10);
      for (const slot of focusSlots) {
        expect(slot.classList.contains("presentation-target-ghost")).toBe(false);
        expect(slot.classList.contains("presentation-source-ghost")).toBe(false);
        const backgroundImage = (slot as HTMLElement).style.backgroundImage;
        expect(backgroundImage.length).toBeGreaterThan(0);
        expect(backgroundImage).toContain("bauteilb");
      }
      mountRoot.remove();
    });

it("renders target ghosts but no source ghosts on focus and labels slides", async () => {
      const { deck } = await import("@semio-tech/mit-bestand-praesentation-projektetage");
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const labelSlide = mountRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
      expect(focusSlide.querySelectorAll(".presentation-source-ghost").length).toBe(0);
      expect(labelSlide.querySelectorAll(".presentation-source-ghost").length).toBe(0);
      expect(labelSlide.querySelectorAll(".presentation-target-ghost").length).toBeGreaterThanOrEqual(8);
      for (const ghost of labelSlide.querySelectorAll<HTMLElement>(".presentation-interactive-disposition.presentation-target-ghost")) {
        expect(ghost.style.pointerEvents).toBe("none");
        expect(ghost.getAttribute("aria-hidden")).toBe("true");
      }
      mountRoot.remove();
    });

it("puts reveal data-id on catalogue tile wrappers for catalogue-to-focus morph", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/presentation");
      const { deck, CATALOGUE_FOCUS_TILES } = await import("@semio-tech/mit-bestand-praesentation-projektetage");
      const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
      const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
      expect(catalogueRef).toBeDefined();
      expect(focusRef).toBeDefined();
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const tileId = CATALOGUE_FOCUS_TILES[0]!.participantId;
      const catalogueTile = catalogueSlide.querySelector(`.presentation-interactive-disposition[data-id="${tileId}"]`) as HTMLElement;
      const focusTile = focusSlide.querySelector(`.presentation-interactive-disposition[data-id="${tileId}"]`) as HTMLElement;
      expect(catalogueTile).toBeTruthy();
      expect(focusTile).toBeTruthy();
      expect(catalogueTile.querySelector(`[data-id="${tileId}"]`)).toBeNull();
      expect(catalogueSlide.getAttribute("data-auto-animate-id")).toBe(focusSlide.getAttribute("data-auto-animate-id"));
      mountRoot.remove();
    });

it("auto-animates catalogue tiles into focus layout", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/presentation");
      const { deck, CATALOGUE_FOCUS_TILES } = await import("@semio-tech/mit-bestand-praesentation-projektetage");
      const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
      const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
      expect(catalogueRef).toBeDefined();
      expect(focusRef).toBeDefined();
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      expect(catalogueSlide.getAttribute("data-auto-animate-id")).toBe(focusSlide.getAttribute("data-auto-animate-id"));
      expect(catalogueSlide.querySelectorAll(".presentation-interactive-disposition.presentation-source-ghost").length).toBe(10);
      catalogueSlide.setAttribute("data-auto-animate", "pending");
      const pairs = presentationAutoAnimateMatcher.call({ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost, catalogueSlide, focusSlide);
      const componentTileIds = CATALOGUE_FOCUS_TILES.map((tile) => tile.participantId);
      expect(componentTileIds.every((id) => pairs.some((pair) => pair.from.getAttribute("data-id") === id))).toBe(true);
      mountRoot.remove();
    });

it("places catalogue-labels target ghosts at inline label frames", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/presentation");
      const { deck, inlineColumnLabelPosition } = await import("@semio-tech/mit-bestand-praesentation-projektetage");
      const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
      const labelRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilbeschriftungen");
      expect(focusRef).toBeDefined();
      expect(labelRef).toBeDefined();
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const labelSlide = mountRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
      const labelSlot = inlineColumnLabelPosition(2);
      const focusStuetze = focusSlide.querySelector('[data-disposition-id^="catalogue-focus--Stütze"]') as HTMLElement | null;
      const labelGhost = labelSlide.querySelector('[data-disposition-id^="catalogue-labels--Stütze"].presentation-target-ghost') as HTMLElement | null;
      expect(focusStuetze?.style.left).not.toBe(`${labelSlot.x * 100}%`);
      expect(labelGhost?.style.left).toBe(`${labelSlot.x * 100}%`);
      expect(labelGhost?.style.top).toBe(`${labelSlot.y * 100}%`);
      expect(labelGhost?.style.width).toBe(`${labelSlot.width * 100}%`);
      expect(labelGhost?.style.height).toBe(`${labelSlot.height * 100}%`);
      mountRoot.remove();
    });

it("fires reveal auto-animate when advancing projektetage focus to labels", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/presentation");
      const { deck } = await import("@semio-tech/mit-bestand-praesentation-projektetage");
      const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
      const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
      const labelRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilbeschriftungen");
      expect(catalogueRef).toBeDefined();
      expect(focusRef).toBeDefined();
      expect(labelRef).toBeDefined();
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      let revealApi: Reveal.Api | undefined;
      let autoAnimateCount = 0;
      act(() => {
        mountPresentation(mountRoot, deck, {
          hash: false,
          slideNumber: false,
          surfaceChrome: false,
          onRevealReady: (api) => {
            revealApi = api;
          },
        });
      });
      const revealRoot = mountRoot.querySelector(".reveal") as HTMLElement;
      revealRoot.addEventListener("autoanimate", () => {
        autoAnimateCount += 1;
      });
      await new Promise<void>((resolve) => {
        const start = Date.now();
        const wait = (): void => {
          if (revealApi) {
            resolve();
            return;
          }
          if (Date.now() - start > 5000) {
            throw new Error("reveal.js did not become ready.");
          }
          setTimeout(wait, 50);
        };
        wait();
      });
      await revealApi!.slide(catalogueRef!.h, catalogueRef!.v);
      await new Promise((resolve) => setTimeout(resolve, 50));
      await revealApi!.slide(focusRef!.h, focusRef!.v);
      await new Promise((resolve) => setTimeout(resolve, 50));
      const afterCatalogueToFocus = autoAnimateCount;
      await revealApi!.slide(labelRef!.h, labelRef!.v);
      const configuredAutoAnimateDuration = revealApi!.getConfig().autoAnimateDuration;
      const autoAnimateDurationMs = (typeof configuredAutoAnimateDuration === "number" ? configuredAutoAnimateDuration : 1) * 1000 + 120;
      await new Promise((resolve) => setTimeout(resolve, autoAnimateDurationMs));
      expect(afterCatalogueToFocus).toBeGreaterThan(0);
      expect(autoAnimateCount).toBeGreaterThan(afterCatalogueToFocus);
      const focusSlide = revealRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const labelSlide = revealRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
      expect(focusSlide.getAttribute("data-auto-animate-id")).toBe(labelSlide.getAttribute("data-auto-animate-id"));
      expect(labelSlide.getAttribute("data-auto-animate")).not.toBe("running");
      expect(labelSlide.querySelector("[data-auto-animate-target]")).toBeNull();
      expect(labelSlide.querySelectorAll(".presentation-target-ghost[data-auto-animate-target]").length).toBe(0);
      mountRoot.remove();
    });
});
