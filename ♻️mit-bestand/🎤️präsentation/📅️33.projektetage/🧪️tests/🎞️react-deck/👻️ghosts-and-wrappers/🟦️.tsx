/** 👻️ The catalogue rest figure, ghost anchor and tile wrapper tests. */
import { it, expect } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import { mountPresentation, presentationAutoAnimateMatcher } from "@semio-tech/presentation-react";
import type { AutoAnimateMatcherHost } from "@semio-tech/presentation-react";
import { tile } from "@semio-tech/presentation";

/** 🪞 Registers the catalogue rest figure, target ghost, reveal data id and focus layout tests. */
export function registerGhostAndWrapperTests(): void {
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
}
