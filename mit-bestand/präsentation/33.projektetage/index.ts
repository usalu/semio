// #region 🧲Header
/** @emoji 📽 33. Projektetage — declarative paper intro via `@framework/presentation`. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	countArrangements,
	collectPresentationSlides,
	expandThoughtSlides,
	loadPresentationFromSlideGlob,
	type Presentation,
	type SlideFile,
} from "@framework/presentation/core";
import { presentationMeta } from "./spec.ts";
import {
	CATALOGUE_COL1,
	CATALOGUE_COL2,
	CATALOGUE_COL3,
	CATALOGUE_EMBODIMENT_CROP,
	CATALOGUE_EMBODIMENT_LABEL,
	CATALOGUE_PARTICIPANT,
} from "./spec.ts";
import "./globals.css";
// #endregion 🔌Adapters

//#region 🔖Deck
const slideModules = import.meta.glob<{ default: SlideFile }>("./slide/**/*.ts", { eager: true });
const deck: Presentation = loadPresentationFromSlideGlob(presentationMeta, slideModules);

function mount(): void {
	const el = document.getElementById("root");
	if (!el) {
		return;
	}
	void import("@framework/presentation/renderer/react").then(({ Expertise, mountPresentation }) => {
		mountPresentation(el, deck, {
			transition: "fade",
			slideNumber: false,
			surfaceChrome: { theme: "dark", device: "desktop", expertise: Expertise.NORMAL },
		});
	});
}

if (typeof document !== "undefined" && !import.meta.vitest) {
	mount();
}
//#endregion 🔖Deck

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("projektetage deck", () => {
		it("declares intro plus expanded media render slides", () => {
			expect(countArrangements(deck)).toBeGreaterThanOrEqual(11);
			expect(deck.language).toBe("de");
		});

		it("uses German bookmark names on intro and media slides", () => {
			const introSlide = collectPresentationSlides(deck)[0];
			expect(introSlide).toEqual({
				h: 0,
				v: 0,
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
				slide: "Titel",
			});
			const catalogueSlide = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
			expect(catalogueSlide).toMatchObject({
				h: 0,
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Medien",
				slide: "Bauteilkatalog",
			});
		});

		it("assembles the catalogue as a split figure grid", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const catalogue = media?.slides.find((slide) => slide.arrangement.id === "catalogue");
			expect(catalogue?.arrangement.dispositions[0]?.split?.tiles).toHaveLength(15);
		});

		it("focuses ten catalogue tiles plus per-column split morph participants", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const focus = media?.slides.find((slide) => slide.arrangement.id === "catalogue-focus");
			const dispositions = focus?.arrangement.dispositions ?? [];
			expect(dispositions[0]?.participantId).toBe(CATALOGUE_PARTICIPANT);
			expect(dispositions[0]?.split?.tiles).toHaveLength(10);
			expect(dispositions[0]?.split?.tiles.map((tile) => tile.key)).toEqual([
				"Rippenplatte 1",
				"Rippenplatte 2",
				"Rippenplatte 3",
				"Rippenplatte 4",
				"Rippenplatte 5",
				"Rippenplatte 6",
				"Unterzug 1",
				"Unterzug 2",
				"Unterzug 3",
				"Stütze",
			]);
			const col1 = dispositions.find((disposition) => disposition.participantId === CATALOGUE_COL1);
			expect(col1?.split?.morphParticipant).toBe(true);
			expect(col1?.split?.tiles).toHaveLength(6);
			expect(col1?.position).toBeUndefined();
		});

		it("morphs each column participant into inline label dispositions on one row", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			expect(media).toBeDefined();
			const expanded = expandThoughtSlides(media!);
			const focusSlide = expanded.find((slide) => slide.id === "catalogue-focus");
			const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
			expect(expanded.map((slide) => slide.id)).not.toContain("catalogue-labels--bridge");
			expect(focusSlide?.arrangement.settleBeforeMorphTo).toEqual(["catalogue-labels"]);
			const labelDispositions =
				labelSlide?.arrangement.dispositions.filter((disposition) => !disposition.morphGhost) ?? [];
			expect(labelDispositions).toHaveLength(3);
			expect(labelDispositions.map((disposition) => disposition.participantId)).toEqual([
				CATALOGUE_COL1,
				CATALOGUE_COL2,
				CATALOGUE_COL3,
			]);
			expect(labelDispositions.every((disposition) => disposition.embodimentId === "label")).toBe(true);
			const yPositions = labelDispositions.map((disposition) => disposition.position?.y);
			expect(new Set(yPositions).size).toBe(1);
			const ghosts = labelSlide?.arrangement.dispositions.filter((disposition) => disposition.morphGhost);
			expect(ghosts).toHaveLength(3);
			expect(ghosts?.map((disposition) => disposition.participantId)).toEqual([
				CATALOGUE_COL1,
				CATALOGUE_COL2,
				CATALOGUE_COL3,
			]);
			expect(ghosts?.every((disposition) => disposition.embodimentId === CATALOGUE_EMBODIMENT_CROP)).toBe(true);
		});

		it("includes figure, video, and pdf participants in the media thought", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const kinds = media?.participants.flatMap((participant) => participant.embodiments.map((embodiment) => embodiment.kind)) ?? [];
			expect(kinds).toContain("figure");
			expect(kinds).toContain("video");
			expect(kinds).toContain("pdf");
		});

		it("loads every slide from slide/<chapter>/<sequence>/<thought>/<slide>.ts paths", () => {
			expect(deck.chapters).toHaveLength(1);
			expect(deck.chapters[0]?.name).toBe("Hauptteil");
			expect(deck.chapters[0]?.sequences[0]?.name).toBe("Einführung");
			expect(deck.chapters[0]?.sequences[0]?.thoughts.map((thought) => thought.name)).toEqual(["Einleitung", "Medien"]);
			expect(deck.chapters[0]?.sequences[0]?.thoughts[0]?.slides.map((slide) => slide.arrangement.name)).toEqual([
				"Titel",
				"Beschreibung",
				"Ziel",
				"Autoren",
				"Fakultät",
				"Universitäten",
				"Lehrstühle",
			]);
			expect(deck.chapters[0]?.sequences[0]?.thoughts[1]?.slides.map((slide) => slide.arrangement.name)).toEqual([
				"Bauteilkatalog",
				"Bauteilarten",
				"Bauteilbeschriftungen",
				"Medienüberblick",
			]);
		});
	});
}
//#endregion 🧪Tests
