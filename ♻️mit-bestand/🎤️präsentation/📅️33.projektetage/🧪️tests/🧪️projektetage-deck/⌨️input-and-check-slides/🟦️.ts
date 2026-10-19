/** ⌨️ The connectivity, input type, input surface and manual check slide tests. */
import type { ProjektetageDeckTestKit } from "../🧰️deck-test-kit/🟦️.ts";

/** ✋ Registers the Generatoren Konnektivität, Eingabearten, Eingabeoberfläche and Manuelles Prüfen slide tests. */
export function registerInputAndCheckSlideTests(kit: ProjektetageDeckTestKit): void {
  const { PRESENTATION_DEFAULT_SLIDE_ASPECT, deck, expect, it } = kit;
  it("lays out Generatoren Konnektivität as 50/50 figure and connector table", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Generatoren");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Konnektivität");
    expect(slide?.arrangement.dispositions).toHaveLength(2);
    const [figure, table] = slide?.arrangement.dispositions ?? [];
    expect(figure?.participantId).toBe("konnektivität-beispiel-3d");
    expect(table?.participantId).toBe("konnektivität-beispiel-tabelle");
    expect(figure?.position?.x ?? 0).toBeLessThan(0.5);
    expect((figure?.position?.x ?? 0) + (figure?.position?.width ?? 0)).toBeLessThanOrEqual(0.5);
    expect(table?.position?.x ?? 0).toBeGreaterThanOrEqual(0.5);
    expect(table?.position?.y).toBe(0.06);
    expect(table?.position?.height).toBeCloseTo(0.88, 10);
    const figureEmbodiment = thought?.embodiments?.find((entry) => entry.id === "konnektivität-beispiel-3d--figure");
    expect(figureEmbodiment).toMatchObject({
      kind: "figure",
      src: "/🔗️konnektivität-beispiel-3d.png",
    });
    const tableEmbodiment = thought?.embodiments?.find((entry) => entry.id === "konnektivität-beispiel-tabelle--markdown");
    expect(tableEmbodiment).toMatchObject({
      kind: "markdown",
      src: "/📊️konnektivität-beispiel-tabelle.md",
    });
  });
  it("shows the eingabearten figure on Eingabearten", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Eingabearten");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-eingabearten");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-eingabearten--figure");
    expect(embodiment).toMatchObject({
      kind: "figure",
      src: "/🔢️eingabeprozess-eingabearten.png",
      alt: "Eingabearten im Eingabeprozess",
      sourceAspect: 3586 / 1346,
    });
    const position = slide?.arrangement.dispositions[0]?.position;
    expect(position).toBeDefined();
    expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(3586 / 1346, 10);
  });
  it("shows the eingabeoberfläche figure on Eingabeoberfläche", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Eingabeoberfläche");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-eingabeoberfläche");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-eingabeoberfläche--figure");
    expect(embodiment).toMatchObject({
      kind: "figure",
      src: "/🖥️eingabeprozess-eingabeoberfläche.png",
      alt: "Eingabeoberfläche im Eingabeprozess",
      sourceAspect: 2130 / 1670,
    });
    const position = slide?.arrangement.dispositions[0]?.position;
    expect(position).toBeDefined();
    expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(2130 / 1670, 10);
  });
  it("shows the eingabeoberfläche annotiert figure on Eingabeoberfläche Annotiert", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Eingabeoberfläche Annotiert");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-eingabeoberfläche-annotiert");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-eingabeoberfläche-annotiert--figure");
    expect(embodiment).toMatchObject({
      kind: "figure",
      src: "/🏷️eingabeprozess-eingabeoberfläche-annotiert.png",
      alt: "Annotierte Eingabeoberfläche im Eingabeprozess",
      sourceAspect: 746 / 659,
    });
    const position = slide?.arrangement.dispositions[0]?.position;
    expect(position).toBeDefined();
    expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(746 / 659, 10);
  });
  it("shows the manuelles prüfen figure on Manuelles Prüfen", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Manuelles Prüfen");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-manuelles-prüfen");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-manuelles-prüfen--figure");
    expect(embodiment).toMatchObject({
      kind: "figure",
      src: "/📝️eingabeprozess-formular.png",
      alt: "Manuelles Prüfen im Eingabeprozess",
      sourceAspect: 860 / 1183,
    });
    const position = slide?.arrangement.dispositions[0]?.position;
    expect(position).toBeDefined();
    expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(860 / 1183, 10);
  });
}
