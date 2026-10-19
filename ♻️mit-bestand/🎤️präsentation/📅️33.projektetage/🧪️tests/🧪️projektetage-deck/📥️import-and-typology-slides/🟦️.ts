/** 📥️ The import, output and typology slide tests. */
import type { ProjektetageDeckTestKit } from "../🧰️deck-test-kit/🟦️.ts";

/** 🌳 Registers the import example, import processing, output json and typology figure tests. */
export function registerImportAndTypologySlideTests(kit: ProjektetageDeckTestKit): void {
  const { PRESENTATION_DEFAULT_SLIDE_ASPECT, deck, expect, it } = kit;
  it("lays out three bauteilbörse figures in a 1×3 grid on Import Besipiel", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Import Besipiel");
    expect(slide?.arrangement.dispositions).toHaveLength(3);
    expect(slide?.arrangement.dispositions.map((disposition) => disposition.participantId)).toEqual(["import-besipiel-holzbalken", "import-besipiel-rippenplatte", "import-besipiel-träger-heb"]);
    for (const [index, item] of [
      { id: "import-besipiel-holzbalken", src: "/🌲️bauteilbörse-holzbalken.png", alt: "Holzbalken" },
      { id: "import-besipiel-rippenplatte", src: "/🦴️bauteilbörse-rippenplatte.png", alt: "Rippenplatte" },
      { id: "import-besipiel-träger-heb", src: "/🅱️bauteilbörse-träger-heb.png", alt: "Träger HEB" },
    ].entries()) {
      const disposition = slide?.arrangement.dispositions[index];
      expect(disposition?.embodimentId).toBe(`${item.id}--figure`);
      const embodiment = thought?.embodiments?.find((entry) => entry.id === `${item.id}--figure`);
      expect(embodiment).toMatchObject({
        kind: "figure",
        src: item.src,
        alt: item.alt,
        scrollOrigin: { x: 0, y: 0 },
      });
    }
    const [left, middle, right] = slide?.arrangement.dispositions.map((disposition) => disposition.position) ?? [];
    expect(left?.x ?? 0).toBeLessThan(middle?.x ?? 0);
    expect((middle?.x ?? 0) + (middle?.width ?? 0)).toBeLessThanOrEqual((right?.x ?? 0) + 0.001);
  });
  it("shows the import verarbeitung figure on Import Verarbeitung", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Import Verarbeitung");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-import-verarbeitung");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-import-verarbeitung--figure");
    expect(embodiment).toMatchObject({
      kind: "figure",
      src: "/📥️import-verarbeitung.png",
      alt: "Import Verarbeitung im Eingabeprozess",
      sourceAspect: 1278 / 1288,
    });
    const position = slide?.arrangement.dispositions[0]?.position;
    expect(position).toBeDefined();
    expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(1278 / 1288, 10);
  });
  it("shows the eingabeprozess output json on Output", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Eingabeprozess");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Output");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-output");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "eingabeprozess-output--json");
    expect(embodiment).toMatchObject({
      kind: "json",
      src: "/🔣️eingabeprozess-output.json",
      title: "Eingabeprozess Output",
    });
    expect(slide?.arrangement.dispositions[0]?.position).toEqual({
      x: 0.04,
      y: 0.06,
      width: 0.92,
      height: 0.88,
    });
  });
  it("shows the typologien figure on Typologien", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
      ?.thoughts.find((entry) => entry.name === "Typologien");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Typologien");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("typologien");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "typologien--figure");
    expect(embodiment).toMatchObject({
      kind: "figure",
      src: "/🧬️typologien.png",
      alt: "Typologien-Katalog",
      sourceAspect: 984 / 1448,
    });
    const position = slide?.arrangement.dispositions[0]?.position;
    expect(position).toBeDefined();
    expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(984 / 1448, 10);
  });
  it("shows the typology tree figure on Baum", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
      ?.thoughts.find((entry) => entry.name === "Typologien");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Baum");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("typologien-baum");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "typologien-baum--figure");
    expect(embodiment).toMatchObject({
      kind: "figure",
      src: "/🌳️typologienbaum.png",
      alt: "Generator-Typologiebaum",
      sourceAspect: 1536 / 1024,
    });
    const position = slide?.arrangement.dispositions[0]?.position;
    expect(position).toBeDefined();
    expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(1536 / 1024, 10);
  });
  it("shows the typologien katalog figure on Katalog", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Ausgangslage")
      ?.thoughts.find((entry) => entry.name === "Typologien");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Katalog");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]?.participantId).toBe("typologien-katalog");
    const embodiment = thought?.embodiments?.find((entry) => entry.id === "typologien-katalog--figure");
    expect(embodiment).toMatchObject({
      kind: "figure",
      src: "/🗂️katalog.png",
      alt: "Typologien-Katalog",
      sourceAspect: 1264 / 713,
    });
    const position = slide?.arrangement.dispositions[0]?.position;
    expect(position).toBeDefined();
    expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(1264 / 713, 10);
  });
}
