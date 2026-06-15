// #region 🧲Header
/** @emoji 📽 33. Projektetage — declarative paper intro via `@framework/presentation`. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	buildResolutionScope,
	collectPresentationSlides,
	countArrangements,
	arrangementRestDispositions,
	expandThoughtSlides,
	loadPresentationFromSlideGlob,
	resolveArrangement,
	type Presentation,
	type Slide,
	type SlideFile,
	type Thought,
} from "@framework/presentation/core";
import { presentationMeta } from "./spec.ts";
import {
	CATALOGUE_COL1,
	CATALOGUE_COL2,
	CATALOGUE_COL3,
	CATALOGUE_EMBODIMENT_COL1_LABEL,
	CATALOGUE_SPLIT,
	ZUKUNFT_BAU_EMBODIMENT,
	ZUKUNFT_BAU_FRAME,
	ZUKUNFT_BAU_PARTICIPANT,
	columnLabelMorphFrom,
	inlineColumnLabelPosition,
	zukunftBauEmbodiment,
	zukunftBauParticipant,
} from "./spec.ts";
import "./globals.css";
// #endregion 🔌Adapters

//#region 🔖Deck
const slideModules = import.meta.glob<{ default: SlideFile }>("./slide/**/*.ts", { eager: true });
const sourceDeck: Presentation = loadPresentationFromSlideGlob(presentationMeta, slideModules);
const INTRO_TITLE_PARTICIPANT = "title";
const INTRO_TITLE_MORPH_FRAME = { x: 0.05, y: 0.36, width: 0.9, height: 0.28 };

function zukunftBauSlide(id: string, name: string): Slide {
	return {
		arrangement: {
			id,
			name,
			dispositions: [
				{
					participantId: ZUKUNFT_BAU_PARTICIPANT,
					embodimentId: ZUKUNFT_BAU_EMBODIMENT,
					emphasis: "active",
					position: ZUKUNFT_BAU_FRAME,
				},
			],
		},
	};
}

function addZukunftBauTitleMorph(slide: Slide | undefined): Slide | undefined {
	if (!slide) {
		return slide;
	}
	return {
		...slide,
		arrangement: {
			...slide.arrangement,
			dispositions: slide.arrangement.dispositions.map((disposition) =>
				disposition.participantId === INTRO_TITLE_PARTICIPANT
					? {
							...disposition,
							position: INTRO_TITLE_MORPH_FRAME,
							morphFrom: [
								...(disposition.morphFrom ?? []),
								{
									participantId: ZUKUNFT_BAU_PARTICIPANT,
									embodimentId: ZUKUNFT_BAU_EMBODIMENT,
									position: INTRO_TITLE_MORPH_FRAME,
								},
							],
						}
					: disposition,
			),
		},
	};
}

function addZukunftBauScope(thought: Thought): Thought {
	const participants = thought.participants?.some((participant) => participant.id === zukunftBauParticipant.id)
		? thought.participants
		: [...(thought.participants ?? []), zukunftBauParticipant];
	const embodiments = thought.embodiments?.some((embodiment) => embodiment.id === zukunftBauEmbodiment.id)
		? thought.embodiments
		: [...(thought.embodiments ?? []), zukunftBauEmbodiment];
	return {
		...thought,
		participants,
		embodiments,
	};
}

function addZukunftBauBookends(presentation: Presentation): Presentation {
	const firstSlide = {
		...zukunftBauSlide("zukunft-bau-auftakt", "Zukunft Bau Auftakt"),
		transition: { kind: "morph" as const },
	};
	const lastSlide = zukunftBauSlide("zukunft-bau-abschluss", "Zukunft Bau Abschluss");
	return {
		...presentation,
		chapters: presentation.chapters.map((chapter, chapterIndex) => ({
			...chapter,
			sequences: chapter.sequences.map((sequence, sequenceIndex) => ({
				...sequence,
				thoughts: sequence.thoughts.map((thought, thoughtIndex) => {
					const scoped = addZukunftBauScope(thought);
					if (chapterIndex === 0 && sequenceIndex === 0 && thoughtIndex === 0) {
						const [titleSlide, ...restSlides] = scoped.slides;
						return {
							...scoped,
							slides: [firstSlide, ...(titleSlide ? [addZukunftBauTitleMorph(titleSlide)] : []), ...restSlides],
						};
					}
					if (
						chapterIndex === presentation.chapters.length - 1 &&
						sequenceIndex === chapter.sequences.length - 1 &&
						thoughtIndex === sequence.thoughts.length - 1
					) {
						return { ...scoped, slides: [...scoped.slides, lastSlide] };
					}
					return scoped;
				}),
			})),
		})),
	};
}

export const deck: Presentation = addZukunftBauBookends(sourceDeck);

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
			expect(countArrangements(deck)).toBeGreaterThanOrEqual(13);
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
				slide: "Zukunft Bau Auftakt",
			});
			const titleSlide = collectPresentationSlides(deck)[1];
			expect(titleSlide).toEqual({
				h: 0,
				v: 1,
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

		it("bookends the deck with the Zukunft Bau Entwerfen mit Bestand image", () => {
			const slides = collectPresentationSlides(deck);
			expect(slides.at(0)?.slide).toBe("Zukunft Bau Auftakt");
			expect(slides.at(-1)?.slide).toBe("Zukunft Bau Abschluss");
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
			expect(expandThoughtSlides(firstThought!).slice(0, 2).map((slide) => slide.autoAnimateId)).toEqual([
				"einleitung--m0",
				"einleitung--m0",
			]);
			expect(lastThought?.slides.at(-1)?.arrangement.dispositions[0]).toMatchObject({
				participantId: ZUKUNFT_BAU_PARTICIPANT,
				embodimentId: ZUKUNFT_BAU_EMBODIMENT,
				position: ZUKUNFT_BAU_FRAME,
			});
		});

		it("assembles the catalogue with one full figure and dormant split tiles", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			const catalogue = media?.slides.find((slide) => slide.arrangement.id === "catalogue");
			expect(catalogue?.arrangement.dispositions).toHaveLength(16);
			expect(catalogue?.arrangement.settleBeforeMorphTo).toEqual(["catalogue-focus"]);
			const dormantTiles = catalogue?.arrangement.dispositions.filter(
				(disposition) => disposition.style?.opacity === 0,
			);
			expect(dormantTiles).toHaveLength(15);
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

		it("expands ten morph sources and three resting labels on Bauteilbeschriftungen", () => {
			const media = deck.chapters[0]?.sequences[0]?.thoughts.find((thought) => thought.name === "Medien");
			expect(media).toBeDefined();
			const expanded = expandThoughtSlides(media!);
			const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
			expect(labelSlide).toBeDefined();
			expect(arrangementRestDispositions(labelSlide!.arrangement)).toHaveLength(3);
			const morphFromSlots = labelSlide!.arrangement.dispositions.flatMap(
				(disposition) => disposition.morphFrom ?? [],
			);
			expect(morphFromSlots).toHaveLength(10);
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
			const labelDispositions = labelSlide ? arrangementRestDispositions(labelSlide.arrangement) : [];
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
				"Zukunft Bau Auftakt",
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
				"Zukunft Bau Abschluss",
			]);
		});
	});
}
//#endregion 🧪Tests
