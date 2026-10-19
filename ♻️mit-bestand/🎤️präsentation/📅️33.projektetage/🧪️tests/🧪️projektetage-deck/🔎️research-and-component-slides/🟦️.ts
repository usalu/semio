/** 🔎️ The research and component grid slide tests. */
import type { ProjektetageDeckTestKit } from "../🧰️deck-test-kit/🟦️.ts";

/** 🧩 Registers the Schweiz research figure and the Baukomponenten grid tests. */
export function registerResearchAndComponentSlideTests(kit: ProjektetageDeckTestKit): void {
  const { BAUKOMPONENTEN_ITEMS, PRESENTATION_DEFAULT_SLIDE_ASPECT, deck, expect, it } = kit;
  it("shows the recherche schweiz figures on Gedanke Schweiz", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Recherche")
      ?.sequences.find((sequence) => sequence.name === "Recherche")
      ?.thoughts.find((entry) => entry.name === "Gedanke Schweiz");
    expect(thought?.slides.map((slide) => slide.arrangement.name)).toEqual(["Überblick", "Zoom In 1", "Zoom In 2", "Zoom In 3"]);
    for (const item of [
      {
        slide: "Überblick",
        participantId: "recherche-schweiz-überblick",
        src: "/🗺️recherche-schweiz-überblick.png",
        sourceAspect: 1987 / 1015,
      },
      {
        slide: "Zoom In 1",
        participantId: "recherche-schweiz-zoom-in-1",
        src: "/🔎️recherche-schweiz-zoom-in-1.png",
        sourceAspect: 1984 / 1014,
      },
      {
        slide: "Zoom In 2",
        participantId: "recherche-schweiz-zoom-in-2",
        src: "/🔬️recherche-schweiz-zoom-in-2.png",
        sourceAspect: 1988 / 1018,
      },
      {
        slide: "Zoom In 3",
        participantId: "recherche-schweiz-zoom-in-3",
        src: "/🎯️recherche-schweiz-zoom-in-3.png",
        sourceAspect: 1981 / 1017,
      },
    ]) {
      const slide = thought?.slides.find((entry) => entry.arrangement.name === item.slide);
      expect(slide?.arrangement.dispositions).toHaveLength(1);
      expect(slide?.arrangement.dispositions[0]).toMatchObject({
        participantId: item.participantId,
        embodimentId: `${item.participantId}--figure`,
      });
      expect(thought?.embodiments?.find((entry) => entry.id === `${item.participantId}--figure`)).toMatchObject({
        kind: "figure",
        src: item.src,
        sourceAspect: item.sourceAspect,
      });
      const position = slide?.arrangement.dispositions[0]?.position;
      expect(position).toBeDefined();
      expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(item.sourceAspect, 10);
    }
  });
  it("lays out nine bauteilbörse figures in a 3×3 grid on Baukomponenten", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
      ?.thoughts.find((entry) => entry.name === "Bauteilbörsen");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Baukomponenten");
    expect(slide?.arrangement.dispositions).toHaveLength(9);
    expect(slide?.arrangement.dispositions.map((disposition) => disposition.participantId)).toEqual([
      "betondeckenplatten",
      "gipsplatten",
      "holzbalken-2",
      "holzbalken",
      "metallprofile",
      "stahltragwerk",
      "träger-hea",
      "träger-ipe",
      "trennwand-glas",
    ]);
    const gridEmbodimentIds = BAUKOMPONENTEN_ITEMS.map((item) => (item.kind === "figure" ? `${item.id}--figure` : `${item.id}--doc`));
    const gridEmbodiments = thought?.embodiments?.filter((embodiment) => gridEmbodimentIds.includes(embodiment.id)) ?? [];
    const kinds = gridEmbodiments.map((embodiment) => embodiment.kind);
    expect(kinds.filter((kind) => kind === "figure")).toHaveLength(8);
    expect(kinds).toContain("pdf");
    for (const embodiment of gridEmbodiments) {
      if (embodiment.kind === "figure" || embodiment.kind === "pdf") {
        expect(embodiment.scrollOrigin).toEqual({ x: 0, y: 0 });
      }
    }
  });
}
