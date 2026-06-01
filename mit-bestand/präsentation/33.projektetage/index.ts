// #region 🧲Header
/** @emoji 📽 33. Projektetage — declarative paper intro via `@framework/presentation`. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	buildResolutionScope,
	collectPresentationSlides,
	countArrangements,
	expandThoughtSlides,
	loadPresentationFromSlideGlob,
	resolveArrangement,
	type Presentation,
	type SlideFile,
} from "@framework/presentation/core";
import { presentationMeta } from "./spec.ts";
import {
	CATALOGUE_COL1,
	CATALOGUE_COL2,
	CATALOGUE_COL3,
	CATALOGUE_EMBODIMENT_COL1_LABEL,
	CATALOGUE_SPLIT,
	columnLabelMorphFrom,
	inlineColumnLabelPosition,
} from "./spec.ts";
import "./globals.css";
// #endregion 🔌Adapters

//#region 🔖Deck
const slideModules = import.meta.glob<{ default: SlideFile }>("./slide/**/*.ts", { eager: true });
export const deck: Presentation = loadPresentationFromSlideGlob(presentationMeta, slideModules);

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

		it("assembles the catalogue as split tile dispositions", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const catalogue = media?.slides.find((slide) => slide.arrangement.id === "catalogue");
			expect(catalogue?.arrangement.dispositions).toHaveLength(15);
		});

		it("names all fifteen catalogue tiles semantically", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const catalogue = media?.slides.find((slide) => slide.arrangement.id === "catalogue");
			expect(catalogue?.arrangement.dispositions.map((disposition) => disposition.participantId)).toEqual([
				"Struktur 1",
				"Struktur 2",
				"Flächen",
				"Elemente 1",
				"Elemente 2",
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
		});

		it("focuses ten catalogue tile participants for column morph", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const focus = media?.slides.find((slide) => slide.arrangement.id === "catalogue-focus");
			const dispositions = focus?.arrangement.dispositions ?? [];
			expect(dispositions).toHaveLength(10);
			expect(dispositions.map((disposition) => disposition.participantId)).toEqual([
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
		});

		it("assigns one auto-animate run across catalogue, focus, and labels", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			expect(media).toBeDefined();
			const expanded = expandThoughtSlides(media!);
			const morphIds = expanded
				.filter((slide) => ["catalogue", "catalogue-focus", "catalogue-labels"].includes(slide.id))
				.map((slide) => slide.autoAnimateId);
			expect(morphIds).toHaveLength(3);
			expect(new Set(morphIds).size).toBe(1);
			expect(morphIds[0]).toBeTruthy();
		});

		it("morphs tile figures into label positions before column text appears", () => {
			const labelPosition = inlineColumnLabelPosition(0);
			const slots = columnLabelMorphFrom("col1", labelPosition);
			expect(slots?.every((slot) => slot.position === labelPosition)).toBe(true);
			expect(slots?.every((slot) => slot.embodimentId.endsWith("-figure"))).toBe(true);
		});

		it("morphs each column participant into inline label dispositions on one row", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			expect(media).toBeDefined();
			const expanded = expandThoughtSlides(media!);
			const focusSlide = expanded.find((slide) => slide.id === "catalogue-focus");
			const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
			expect(focusSlide?.arrangement.settleBeforeMorphTo).toEqual(["catalogue-labels"]);
			const labelDispositions =
				labelSlide?.arrangement.dispositions.filter((disposition) => !disposition.morphGhost) ?? [];
			expect(labelDispositions).toHaveLength(3);
			expect(labelDispositions.map((disposition) => disposition.participantId)).toEqual([
				CATALOGUE_COL1,
				CATALOGUE_COL2,
				CATALOGUE_COL3,
			]);
			expect(labelDispositions.every((disposition) => disposition.embodimentId.endsWith("--label"))).toBe(
				true,
			);
			const yPositions = labelDispositions.map((disposition) => disposition.position?.y);
			expect(new Set(yPositions).size).toBe(1);
			const scope = buildResolutionScope([media!]);
			const resolved = resolveArrangement(scope, labelSlide!.arrangement);
			expect(resolved.filter((entry) => entry.embodiment.kind === "text")).toHaveLength(3);
		});

		it("includes figure, video, and pdf embodiments in the media thought", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const kinds = media?.embodiments?.map((embodiment) => embodiment.kind) ?? [];
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
