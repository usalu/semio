/** 🧭️ The catalogue label frame placement and focus advance tests. */
import { it, expect } from "vitest";
import { act } from "@semio-tech/ui-react/test";
import Reveal from "reveal.js";
import { mountPresentation } from "@semio-tech/presentation-react";

/** 📍 Registers the catalogue label ghost frame and the focus to labels advance tests. */
export function registerLabelFramesAndAdvanceTests(): void {
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
}
