/** 🧭️ The deck structure, German bookmark and bookend slide tests. */
import type { ProjektetageDeckTestKit } from "../🧰️deck-test-kit/🟦️.ts";

/** 📑 Registers the intro and render slide, German bookmark and Zukunft Bau bookend tests. */
export function registerStructureAndBookendTests(kit: ProjektetageDeckTestKit): void {
  const { INTRO_TITLE_MORPH_FRAME, ZUKUNFT_BAU_EMBODIMENT, ZUKUNFT_BAU_FRAME, ZUKUNFT_BAU_PARTICIPANT, collectPresentationSlides, countArrangements, deck, expandThoughtSlides, expect, it } = kit;
  it("declares intro plus expanded render slides", () => {
    expect(countArrangements(deck)).toBeGreaterThanOrEqual(15);
    expect(deck.language).toBe("de");
  });
  it("uses German bookmark names on intro and catalogue slides", () => {
    const introSlide = collectPresentationSlides(deck)[0];
    expect(introSlide).toEqual({
      h: 0,
      v: 0,
      chapter: "Einführung",
      sequence: "Einleitung",
      thought: "Einleitung",
      slide: "Zukunft Bau Auftakt",
    });
    const titleSlide = collectPresentationSlides(deck)[1];
    expect(titleSlide).toEqual({
      h: 0,
      v: 1,
      chapter: "Einführung",
      sequence: "Einleitung",
      thought: "Einleitung",
      slide: "Titel",
    });
    const catalogueSlide = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
    expect(catalogueSlide).toMatchObject({
      h: 2,
      v: 1,
      chapter: "Bauteilportal",
      sequence: "Ausgangslage",
      thought: "Bauteilbörsen",
      slide: "Bauteilkatalog",
    });
  });
  it("opens with the Zukunft Bau Entwerfen mit Bestand image and closes with Abschluss", () => {
    const slides = collectPresentationSlides(deck);
    expect(slides.at(0)?.slide).toBe("Zukunft Bau Auftakt");
    expect(slides.at(-1)?.slide).toBe("Abschluss");
    const firstThought = deck.chapters[0]?.sequences[0]?.thoughts[0];
    const lastThought = deck.chapters.at(-1)?.sequences.at(-1)?.thoughts.at(-1);
    expect(firstThought?.slides[0]?.arrangement.dispositions[0]).toMatchObject({
      participantId: ZUKUNFT_BAU_PARTICIPANT,
      embodimentId: ZUKUNFT_BAU_EMBODIMENT,
      position: ZUKUNFT_BAU_FRAME,
    });
    expect(firstThought?.slides[1]?.arrangement.dispositions[0]?.morphFrom).toEqual([
      {
        participantId: ZUKUNFT_BAU_PARTICIPANT,
        embodimentId: ZUKUNFT_BAU_EMBODIMENT,
        position: INTRO_TITLE_MORPH_FRAME,
      },
    ]);
    expect(firstThought?.slides[1]?.arrangement.dispositions[0]?.position).toEqual(INTRO_TITLE_MORPH_FRAME);
    expect(firstThought?.slides[0]?.transition).toEqual({ kind: "morph" });
    expect(
      expandThoughtSlides(firstThought!)
        .slice(0, 2)
        .map((slide) => slide.autoAnimateId),
    ).toEqual(["einleitung--m0", "einleitung--m0"]);
    const abschluss = lastThought?.slides.at(-1);
    expect(abschluss?.arrangement.name).toBe("Abschluss");
    expect(abschluss?.arrangement.dispositions).toHaveLength(2);
    expect(lastThought?.embodiments?.find((entry) => entry.id === "abschluss-heading--text")).toMatchObject({
      kind: "text",
      lines: ["Vielen Dank für Ihre Aufmerksamkeit!"],
      level: "heading",
    });
    expect(lastThought?.embodiments?.find((entry) => entry.id === "abschluss-sponsorship--text")).toMatchObject({
      kind: "text",
      level: "body",
    });
  });
}
