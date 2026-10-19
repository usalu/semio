/** 🧱️ The slide path, puzzle, catalog step, bauteillogik and modell slide tests. */
import type { ProjektetageDeckTestKit } from "../🧰️deck-test-kit/🟦️.ts";

/** 🛠 Registers the slide path loading, puzzle, entwurfswerkzeug catalog, bauteillogik and modell iframe tests. */
export function registerToolSlideTests(kit: ProjektetageDeckTestKit): void {
  const { deck, expect, it } = kit;
  it("loads every slide from slide/<chapter>/<sequence>/<thought>/<slide>.ts paths", () => {
    expect(deck.chapters).toHaveLength(4);
    expect(deck.chapters.map((chapter) => chapter.name)).toEqual(["Einführung", "Recherche", "Bauteilportal", "Entwurfswerkzeug"]);
    const einführung = deck.chapters.find((chapter) => chapter.name === "Einführung");
    expect(einführung?.sequences.map((sequence) => sequence.name)).toEqual(["Einleitung"]);
    expect(einführung?.sequences[0]?.thoughts.map((thought) => thought.name)).toEqual(["Einleitung"]);
    expect(einführung?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Zukunft Bau Auftakt", "Titel", "Beschreibung", "Ziel", "Autoren", "Fakultät", "Universitäten", "Lehrstühle"]);
    const recherche = deck.chapters.find((chapter) => chapter.name === "Recherche");
    expect(recherche?.sequences.map((sequence) => sequence.name)).toEqual(["Recherche"]);
    expect(recherche?.sequences[0]?.thoughts.map((thought) => thought.name)).toEqual(["Gedanke Schweiz"]);
    expect(recherche?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Überblick", "Zoom In 1", "Zoom In 2", "Zoom In 3"]);
    const bauteilportal = deck.chapters.find((chapter) => chapter.name === "Bauteilportal");
    expect(bauteilportal?.sequences.map((sequence) => sequence.name)).toEqual(["Ausgangslage", "Systematik"]);
    expect(bauteilportal?.sequences[0]?.thoughts.map((thought) => thought.name)).toEqual(["Bauteilbörsen", "Typologien"]);
    expect(bauteilportal?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Baukomponenten", "Bauteilkatalog", "Bauteilarten", "Bauteilbeschriftungen"]);
    expect(bauteilportal?.sequences[0]?.thoughts[1]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Typologien", "Baum", "Katalog"]);
    expect(bauteilportal?.sequences[1]?.name).toBe("Systematik");
    expect(bauteilportal?.sequences[1]?.thoughts.map((thought) => thought.name)).toEqual(["Eingabeprozess", "Generatoren"]);
    expect(bauteilportal?.sequences[1]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual([
      "Eingabearten",
      "Eingabeoberfläche",
      "Eingabeoberfläche Annotiert",
      "Manuelles Prüfen",
      "Import Besipiel",
      "Import Verarbeitung",
      "Output",
    ]);
    expect(bauteilportal?.sequences[1]?.thoughts[1]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Bauteillogik", "Modelle", "Konnektivität"]);
    const entwurfswerkzeug = deck.chapters.find((chapter) => chapter.name === "Entwurfswerkzeug");
    expect(entwurfswerkzeug?.sequences.map((sequence) => sequence.name)).toEqual(["Benutzeroberfläche"]);
    expect(entwurfswerkzeug?.sequences[0]?.thoughts[0]?.name).toBe("Benutzeroberfläche");
    expect(entwurfswerkzeug?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual(["Katalog", "Filter", "Detail", "Puzzle", "Abschluss"]);
  });
  it("embeds the 3D puzzle iframe full-frame on Puzzle", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Entwurfswerkzeug")
      ?.sequences.find((sequence) => sequence.name === "Benutzeroberfläche")
      ?.thoughts.find((entry) => entry.name === "Benutzeroberfläche");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Puzzle");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]).toMatchObject({
      participantId: "puzzle-3d",
      embodimentId: "puzzle-3d--iframe",
      position: { x: 0, y: 0, width: 1, height: 1 },
    });
    expect(thought?.embodiments?.find((entry) => entry.id === "puzzle-3d--iframe")).toMatchObject({
      kind: "iframe",
      src: "https://v4.3d.puzzle.semio-tech.com/",
    });
  });
  it("lays out each entwurfswerkzeug catalog step as a single centered figure", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Entwurfswerkzeug")
      ?.sequences.find((sequence) => sequence.name === "Benutzeroberfläche")
      ?.thoughts.find((entry) => entry.name === "Benutzeroberfläche");
    for (const item of [
      {
        slide: "Katalog",
        participantId: "entwurfswerkzeug-katalog",
        src: "/📚️entwurfswerkzeug-katalog.png",
        sourceAspect: 688 / 1948,
      },
      {
        slide: "Filter",
        participantId: "entwurfswerkzeug-filter",
        src: "/🧮️entwurfswerkzeug-filter.png",
        sourceAspect: 674 / 1948,
      },
      {
        slide: "Detail",
        participantId: "entwurfswerkzeug-detail",
        src: "/🔍️entwurfswerkzeug-detail.png",
        sourceAspect: 674 / 1948,
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
    }
  });
  it("embeds the procedural bauteillogik iframe full-frame on Bauteillogik", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Generatoren");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Bauteillogik");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]).toMatchObject({
      participantId: "procedural-bauteillogik",
      embodimentId: "procedural-bauteillogik--iframe",
      position: { x: 0, y: 0, width: 1, height: 1 },
    });
    expect(thought?.embodiments?.find((entry) => entry.id === "procedural-bauteillogik--iframe")).toMatchObject({
      kind: "iframe",
      src: "https://v4.procedural.semio-tech.com/",
    });
  });
  it("embeds the CAD modelle iframe full-frame on Modelle", () => {
    const thought = deck.chapters
      .find((chapter) => chapter.name === "Bauteilportal")
      ?.sequences.find((sequence) => sequence.name === "Systematik")
      ?.thoughts.find((entry) => entry.name === "Generatoren");
    const slide = thought?.slides.find((entry) => entry.arrangement.name === "Modelle");
    expect(slide?.arrangement.dispositions).toHaveLength(1);
    expect(slide?.arrangement.dispositions[0]).toMatchObject({
      participantId: "cad-modelle",
      embodimentId: "cad-modelle--iframe",
      position: { x: 0, y: 0, width: 1, height: 1 },
    });
    expect(thought?.embodiments?.find((entry) => entry.id === "cad-modelle--iframe")).toMatchObject({
      kind: "iframe",
      src: "https://v4.cad.semio-tech.com/",
    });
  });
}
