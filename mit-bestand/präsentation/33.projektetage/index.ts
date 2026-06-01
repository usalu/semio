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
			expect(countArrangements(deck)).toBeGreaterThan(11);
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

		it("focuses ten catalogue tiles plus hidden column morph anchors", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const focus = media?.slides.find((slide) => slide.arrangement.id === "catalogue-focus");
			const dispositions = focus?.arrangement.dispositions ?? [];
			expect(dispositions[0]?.participantId).toBe(CATALOGUE_PARTICIPANT);
			expect(dispositions[0]?.split?.tiles).toHaveLength(10);
			expect(dispositions.slice(1).map((disposition) => disposition.participantId)).toEqual([
				CATALOGUE_COL1,
				CATALOGUE_COL2,
				CATALOGUE_COL3,
			]);
			expect(dispositions.slice(1).every((disposition) => disposition.style?.opacity === 0)).toBe(true);
			const col3 = dispositions.find((disposition) => disposition.participantId === CATALOGUE_COL3);
			expect(col3?.position?.height).toBeGreaterThan(0.7);
		});

		it("morphs focus column crops to label positions on a bridge then switches to text", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			expect(media).toBeDefined();
			const expanded = expandThoughtSlides(media!);
			const catalogueSlide = expanded.find((slide) => slide.id === "catalogue");
			const focusSlide = expanded.find((slide) => slide.id === "catalogue-focus");
			const bridgeSlide = expanded.find((slide) => slide.id === "catalogue-labels--bridge");
			const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
			expect(catalogueSlide?.autoAnimateId).toBeDefined();
			expect(focusSlide?.autoAnimateId).toBe(catalogueSlide?.autoAnimateId);
			expect(bridgeSlide?.derived).toBe(true);
			expect(
				bridgeSlide?.arrangement.dispositions.every(
					(disposition) => disposition.embodimentId === CATALOGUE_EMBODIMENT_CROP,
				),
			).toBe(true);
			expect(focusSlide?.arrangement.settleBeforeMorphTo).toEqual(["catalogue-labels--bridge"]);
			expect(labelSlide?.autoAnimateId).toBe(focusSlide?.autoAnimateId);
			expect(
				labelSlide?.arrangement.dispositions.every((disposition) => disposition.embodimentId === CATALOGUE_EMBODIMENT_LABEL),
			).toBe(true);
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
