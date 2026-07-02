export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { presentationPlayAppDefinition } from "./playground.ts";

export { presentationPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for presentation. */
export function buildPresentationProgramDefinition(): PlatformDefinition {
	const app = presentationPlayAppDefinition;
	return {
		id: "presentation",
		name: "Presentation",
		apiVersion: "1",
		apps: [{ id: "presentation", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	function thoughtScope(thought: Thought): ResolutionScope {
		return buildResolutionScope([thought]);
	}

	const sampleIntro = intro({
		title: {
			full: ["Line A", "Line B", "Line C"],
			short: "Short title",
		},
		description: { full: ["D1", "D2"], short: "D short" },
		goal: ["G1"],
		authors: {
			lines: [[{ name: "Alice" }, { name: "Bob" }], [{ name: "Carol", marks: ["1", "b"] }]],
		},
		affiliations: {
			steps: [
				[{ mark: "a", name: "Faculty" }],
				[
					{ mark: "a", name: "Faculty" },
					{ mark: "1", name: "University" },
					{ mark: "2", name: "Other University" },
				],
				[
					{ mark: "a", name: "Faculty" },
					{
						mark: "1",
						name: "University",
						shortName: "LUH",
						suffix: { mark: "x", name: "Chair X" },
					},
					{
						mark: "2",
						name: "Other University",
						shortName: "UdK",
						suffix: { mark: "y", name: "Chair Y" },
					},
				],
			],
		},
	});

	describe("loadPresentationFromSlideGlob", () => {
		it("assembles chapters, sequences, thoughts, and ordered slides from slide paths", () => {
			const deck = loadPresentationFromSlideGlob(
				{ id: "deck", name: "Deck", language: "de" },
				{
					"./slide/Hauptteil/Einführung/Einleitung/Titel.ts": {
						default: {
							order: 0,
							participants: [{ id: "title" }],
							embodiments: [{ kind: "text", id: "title--main", lines: ["A"], level: "title" }],
							arrangement: {
								id: "title",
								name: "Titel",
								dispositions: [{ participantId: "title", embodimentId: "title--main", emphasis: "active" }],
							},
						},
					},
					"./slide/Hauptteil/Einführung/Einleitung/Ziel.ts": {
						default: {
							order: 1,
							arrangement: {
								id: "goal",
								name: "Ziel",
								dispositions: [{ participantId: "title", embodimentId: "title--main", emphasis: "active" }],
							},
						},
					},
					"./slide/Hauptteil/Einführung/Medien/Bauteilkatalog.ts": {
						default: {
							order: 0,
							arrangement: {
								id: "catalogue",
								name: "Bauteilkatalog",
								dispositions: [{ participantId: "catalogue", embodimentId: "catalogue--figure", emphasis: "active" }],
							},
						},
					},
				},
			);
			expect(deck.chapters[0]).toMatchObject({
				name: "Hauptteil",
				sequences: [
					{
						name: "Einführung",
						thoughts: [
							{
								name: "Einleitung",
								slides: [{ arrangement: { name: "Titel" } }, { arrangement: { name: "Ziel" } }],
							},
							{
								name: "Medien",
								slides: [{ arrangement: { name: "Bauteilkatalog" } }],
							},
						],
					},
				],
			});
		});
	});

	describe("parsePresentationSlideFilePath", () => {
		it("round-trips canonical slide module paths", () => {
			const path = presentationSlideFilePath("Hauptteil", "Einführung", "Einleitung", "Titel");
			expect(path).toBe("slide/Hauptteil/Einführung/Einleitung/Titel.ts");
			expect(parsePresentationSlideFilePath(`./${path}`)).toEqual({
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
				slide: "Titel",
			});
		});
	});

	describe("parsePresentationThoughtFilePath", () => {
		it("round-trips canonical thought template paths", () => {
			const path = presentationThoughtFilePath("Hauptteil", "Einführung", "Einleitung");
			expect(path).toBe("slide/Hauptteil/Einführung/Einleitung.ts");
			expect(parsePresentationThoughtFilePath(`./${path}`)).toEqual({
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
			});
			expect(parsePresentationSlideFilePath(`./${path}`)).toBeNull();
		});
	});

	describe("loadPresentationFromSlideGlob thought templates", () => {
		it("expands intro thought files into ordered slides", () => {
			const deck = loadPresentationFromSlideGlob(
				{ id: "deck", name: "Deck", language: "de" },
				{
					"./slide/Hauptteil/Einführung/Einleitung.ts": {
						default: introThoughtFile({
							language: "de",
							title: { full: ["T"], short: "T" },
							description: { full: ["D"], short: "D" },
							goal: ["G"],
							authors: { lines: [[{ name: "A", marks: ["a"] }]] },
							affiliations: { steps: [[{ mark: "a", name: "Faculty" }], [{ mark: "a", name: "Faculty" }], [{ mark: "a", name: "Faculty" }]] },
						}),
					},
				},
			);
			const thought = deck.chapters[0]!.sequences[0]!.thoughts[0]!;
			expect(thought.name).toBe("Einleitung");
			expect(thought.slides.map((slide) => slide.arrangement.name)).toEqual([
				"Titel",
				"Beschreibung",
				"Ziel",
				"Autoren",
				"Fakultät",
				"Universitäten",
				"Lehrstühle",
			]);
		});
	});

	describe("intro", () => {
		it("recognizes intro arrangement ids regardless of bookmark language", () => {
			expect(isIntroArrangementId("affiliations-3")).toBe(true);
			expect(isIntroArrangementId("Lehrstühle")).toBe(false);
		});

		it("builds seven slides in one thought", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			expect(thought.slides.map((slide) => slide.arrangement.id)).toEqual([
				"title",
				"description",
				"goal",
				"authors",
				"affiliations-1",
				"affiliations-2",
				"affiliations-3",
			]);
			expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual([
				"title",
				"description",
				"goal",
				"authors",
				"affiliations-1",
				"affiliations-2",
				"affiliations-3",
			]);
		});

		it("keeps a single universities bookmark at v=5", () => {
			const uniSlides = collectPresentationSlides(sampleIntro).filter((slide) => slide.slide === "Universities");
			expect(uniSlides).toHaveLength(1);
			expect(uniSlides[0]?.v).toBe(5);
		});

		it("uses fixed-size heading blocks without fit-text", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const textEmbodiments = (thought.embodiments ?? []).filter((e): e is TextEmbodiment => e.kind === "text");
			expect(textEmbodiments.every((e) => e.fit !== true)).toBe(true);
			expect(textEmbodiments.every((e) => resolveTextMorphRoot(e) === "heading-block")).toBe(true);
		});

		it("uses affiliation short names when chairs are introduced", () => {
			expect(
				affiliationLineName({
					mark: "1",
					name: "Leibniz Universität Hannover",
					shortName: "LUH",
				}),
			).toBe("LUH");
		});

		it("includes muted authors on goal so reveal can morph into the authors slide", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
			const goalAuthors = resolveArrangement(thoughtScope(thought), goal).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_AUTHORS,
			);
			expect(goalAuthors?.emphasis).toBe("muted");
			expect(goalAuthors?.embodiment.id).toBe(INTRO_EMBODIMENT_AUTHORS_PLAIN);
		});

		it("uses plain short description embodiment on goal and later intro slides", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
			const authors = thought.slides.find((slide) => slide.arrangement.id === "authors")!.arrangement;
			const goalDescription = resolveArrangement(thoughtScope(thought), goal).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_DESCRIPTION,
			)!;
			const authorsDescription = resolveArrangement(thoughtScope(thought), authors).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_DESCRIPTION,
			)!;
			if (goalDescription.embodiment.kind === "text" && authorsDescription.embodiment.kind === "text") {
				expect(goalDescription.embodiment.id).toBe(INTRO_EMBODIMENT_DESCRIPTION_SHORT);
				expect(goalDescription.embodiment.morphFromLines).toBeUndefined();
				expect(authorsDescription.embodiment.id).toBe(INTRO_EMBODIMENT_DESCRIPTION_SHORT);
			}
		});

		it("records prior affiliation labels for embodiment morph on chairs slide", () => {
			const previous = [
				{ mark: "a", name: "Faculty" },
				{ mark: "1", name: "Leibniz Universität Hannover" },
			] as const;
			const current = [
				{ mark: "a", name: "Faculty" },
				{
					mark: "1",
					name: "Leibniz Universität Hannover",
					shortName: "LUH",
					suffix: { mark: "x", name: "Chair X" },
				},
			] as const;
			expect(affiliationEmbodimentMorphLabels(previous, current)).toEqual({
				"1": "Leibniz Universität Hannover",
			});
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const affiliations3 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-3")!.arrangement;
			const step3 = resolveArrangement(thoughtScope(thought), affiliations3).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step3.embodiment.kind === "affiliations") {
				expect(step3.embodiment.morphLineLabels).toEqual({
					"1": "University",
					"2": "Other University",
				});
			}
		});

		it("abbreviates author first names on affiliation slides", () => {
			expect(abbreviateAuthorFirstName("Ueli Saluz")).toBe("U. Saluz");
			expect(abbreviateAuthorFirstName("Christoph Gengnagel")).toBe("C. Gengnagel");
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const affiliations1 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-1")!.arrangement;
			const authors = resolveArrangement(thoughtScope(thought), affiliations1).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_AUTHORS,
			)!;
			if (authors.embodiment.kind === "authors") {
				expect(authors.embodiment.abbreviateFirstName).toBe(true);
			}
		});

		it("introduces author marks with each affiliation step", () => {
			const lines = [[{ name: "Alice", marks: ["a", "1", "x"] }]] as const;
			const rawSteps = [
				[{ mark: "a", name: "Faculty" }],
				[
					{ mark: "a", name: "Faculty" },
					{ mark: "1", name: "University" },
				],
				[
					{ mark: "a", name: "Faculty" },
					{ mark: "1", name: "University", suffix: { mark: "x", name: "Chair X" } },
				],
			] as const;
			const aff1 = authorLinesForAffiliationStep(lines, rawSteps[0], [])[0]![0]!;
			expect(aff1.markEntries?.map((m) => m.mark)).toEqual(["a"]);
			const aff2 = authorLinesForAffiliationStep(lines, rawSteps[1], rawSteps[0])[0]![0]!;
			expect(aff2.markEntries?.map((m) => m.mark)).toEqual(["a", "1"]);
			expect(aff2.markEntries?.find((m) => m.mark === "a")?.emphasis).toBe("muted");
			expect(aff2.markEntries?.find((m) => m.mark === "1")?.emphasis).toBe("active");
			const aff3 = authorLinesForAffiliationStep(lines, rawSteps[2], rawSteps[1])[0]![0]!;
			expect(aff3.markEntries?.map((m) => m.mark)).toEqual(["a", "1", "x"]);
			expect(aff3.markEntries?.find((m) => m.mark === "x")?.emphasis).toBe("active");
		});

		it("highlights only new affiliation marks per slide", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const affiliations2 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-2")!.arrangement;
			const step2 = resolveArrangement(thoughtScope(thought), affiliations2).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step2.embodiment.kind === "affiliations") {
				expect(step2.embodiment.entries.find((e) => e.mark === "1")?.lineEmphasis).toBe("active");
				expect(step2.embodiment.entries.find((e) => e.mark === "a")?.lineEmphasis).toBe("muted");
			}
			const affiliations3 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-3")!.arrangement;
			const step3 = resolveArrangement(thoughtScope(thought), affiliations3).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step3.embodiment.kind === "affiliations") {
				const uni = step3.embodiment.entries.find((e) => e.mark === "1");
				expect(uni?.lineEmphasis).toBe("muted");
				expect(uni?.suffixEmphasis).toBe("active");
				expect(step3.embodiment.entries.find((e) => e.mark === "a")?.lineEmphasis).toBe("muted");
			}
		});
		it("assigns German bookmark names when language is de", () => {
			const deck = intro({
				language: "de",
				title: { full: ["A"], short: "Short" },
				description: { full: ["D"], short: "D short" },
				goal: ["G"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: {
					steps: [
						[{ mark: "a", name: "Faculty" }],
						[{ mark: "a", name: "Faculty" }, { mark: "1", name: "Uni" }],
						[{ mark: "a", name: "Faculty" }, { mark: "1", name: "Uni" }],
					],
				},
			});
			const chapter = deck.chapters[0]!;
			const sequence = chapter.sequences[0]!;
			const thought = sequence.thoughts[0]!;
			expect(chapter.name).toBe("Hauptteil");
			expect(sequence.name).toBe("Einführung");
			expect(thought.name).toBe("Einleitung");
			expect(thought.slides.map((slide) => slide.arrangement.name)).toEqual([
				"Titel",
				"Beschreibung",
				"Ziel",
				"Autoren",
				"Fakultät",
				"Universitäten",
				"Lehrstühle",
			]);
			const goalSlide = collectPresentationSlides(deck).find((slide) => slide.slide === "Ziel");
			expect(goalSlide).toMatchObject({
				h: 0,
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
				slide: "Ziel",
			});
			expect(goalSlide?.v).toBeGreaterThan(0);
		});
	});

	describe("resolveEmbodiment", () => {
		it("throws when embodiment id is missing from scope", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "x" }],
					embodiments: [{ kind: "text", id: "x--body", lines: ["a"], level: "body" }],
				},
			]);
			expect(() => resolveEmbodiment(scope, "missing")).toThrow(/Unknown embodiment/);
		});
	});

	describe("buildResolutionScope", () => {
		it("lets inner scopes override embodiment ids", () => {
			const scope = buildResolutionScope([
				{ embodiments: [{ kind: "text", id: "a", lines: ["outer"], level: "body" }] },
				{ embodiments: [{ kind: "text", id: "a", lines: ["inner"], level: "body" }] },
			]);
			expect((resolveEmbodiment(scope, "a") as TextEmbodiment).lines[0]).toBe("inner");
		});
	});

	describe("morphId", () => {
		it("uses participant id as reveal data-id", () => {
			expect(morphId("title")).toBe("title");
		});
	});

	describe("split", () => {
		it("produces one participant, embodiment, and disposition per grid cell", () => {
			const artifacts = split({
				source: "/catalogue.png",
				rows: 2,
				columns: 2,
				frame: { x: 0, y: 0, width: 1, height: 1 },
			});
			expect(artifacts.participants).toHaveLength(4);
			expect(artifacts.embodiments).toHaveLength(4);
			expect(artifacts.dispositions).toHaveLength(4);
			expect(artifacts.dispositions[0]?.participantId).toBe("tile-r0-c0");
			expect(artifacts.dispositions[0]?.embodimentId).toBe("tile-r0-c0-figure");
		});
	});

	describe("expandThoughtSlides", () => {
		it("assigns one auto-animate id per morph run", () => {
			const thought: Thought = {
				id: "morph",
				participants: [{ id: "label" }],
				embodiments: [
					{ kind: "text", id: "source", lines: ["Reuse"], level: "heading" },
					{ kind: "text", id: "target", lines: ["Remanufacture"], level: "heading" },
				],
				slides: [
					{
						arrangement: {
							id: "source",
							dispositions: [{ participantId: "label", embodimentId: "source", emphasis: "active" }],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "mapping",
							dispositions: [{ participantId: "label", embodimentId: "target", emphasis: "active" }],
						},
					},
				],
			};
			const expanded = expandThoughtSlides(thought);
			expect(expanded.map((slide) => slide.id)).toEqual(["source", "mapping"]);
			expect(expanded.every((slide) => slide.autoAnimateId === "morph--m0")).toBe(true);
		});

		it("keeps morphFrom on label slides without expanding arrangements", () => {
			const thought: Thought = {
				id: "merge",
				participants: [{ id: "col1" }, { id: "labels" }],
				embodiments: [
					{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
					{ kind: "text", id: "label", lines: ["A"], level: "heading" },
					{ kind: "text", id: "stack", lines: ["A"], level: "heading", morphRoot: "heading-block" },
				],
				slides: [
					{
						arrangement: {
							id: "focus",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "crop",
									emphasis: "active",
									position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "labels",
							dispositions: [
								{
									participantId: "labels",
									embodimentId: "stack",
									emphasis: "active",
									position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
									morphFrom: [
										{
											participantId: "col1",
											embodimentId: "crop",
											position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
											targetLineIndex: 0,
										},
									],
								},
							],
						},
					},
				],
			};
			const labels = thought.slides.find((slide) => slide.arrangement.id === "labels")!.arrangement;
			const morphFrom = labels.dispositions[0]?.morphFrom?.[0];
			expect(morphFrom?.targetLineIndex).toBe(0);
			expect(morphFrom?.position).toEqual({ x: 0.38, y: 0.12, width: 0.24, height: 0.24 });
			expect(arrangementRestDispositions(labels)).toHaveLength(1);
		});

		it("resolves morph-into targets with --label morph ids", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "catalogue-col1" }],
					embodiments: [
						{ kind: "text", id: "catalogue-col1--label", lines: ["Rippenplatte"], level: "heading" },
					],
				},
			]);
			const resolved = resolveArrangement(scope, {
				id: "labels",
				dispositions: [
					{
						participantId: "catalogue-col1",
						embodimentId: "catalogue-col1--label",
						emphasis: "active",
						position: { x: 0.1, y: 0.4, width: 0.2, height: 0.1 },
						morphFrom: [
							{
								participantId: "Rippenplatte 1",
								embodimentId: "Rippenplatte 1-figure",
								position: { x: 0.1, y: 0.4, width: 0.2, height: 0.1 },
							},
						],
					},
				],
			});
			expect(resolved[0]?.morphId).toBe("catalogue-col1--label");
		});

		it("preserves declarative settleBeforeMorphTo on arrangements", () => {
			const thought: Thought = {
				id: "media",
				participants: [{ id: "col1" }],
				embodiments: [
					{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
					{ kind: "text", id: "label", lines: ["A"], level: "heading" },
				],
				slides: [
					{
						arrangement: {
							id: "focus",
							settleBeforeMorphTo: ["labels"],
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "crop",
									emphasis: "active",
									position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
									style: { opacity: 0 },
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "labels",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "label",
									emphasis: "active",
									position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
								},
							],
						},
					},
				],
			};
			const focus = expandThoughtSlides(thought).find((slide) => slide.id === "focus");
			expect(focus?.arrangement.settleBeforeMorphTo).toEqual(["labels"]);
		});

		it("keeps consecutive morph slides without extra render slides", () => {
			const thought: Thought = {
				id: "move",
				participants: [{ id: "box" }],
				embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
				slides: [
					{
						arrangement: {
							id: "left",
							dispositions: [
								{
									participantId: "box",
									embodimentId: "box--main",
									emphasis: "active",
									position: { x: 0.1, y: 0.2, width: 0.3, height: 0.2 },
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "right",
							dispositions: [
								{
									participantId: "box",
									embodimentId: "box--main",
									emphasis: "active",
									position: { x: 0.6, y: 0.2, width: 0.3, height: 0.2 },
								},
							],
						},
					},
				],
			};
			expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual(["left", "right"]);
		});

		it("keeps morphTo on the source slide without expanding arrangements", () => {
			const thought: Thought = {
				id: "split",
				participants: [{ id: "whole" }, { id: "tile-a" }],
				embodiments: [
					{ kind: "figure", id: "whole--figure", src: "/a.png" },
					{ kind: "figure", id: "tile-a--figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
				],
				slides: [
					{
						arrangement: {
							id: "whole",
							dispositions: [
								{
									participantId: "whole",
									embodimentId: "whole--figure",
									emphasis: "active",
									morphTo: [
										{
											participantId: "tile-a",
											position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 },
										},
									],
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "tiles",
							dispositions: [
								{
									participantId: "tile-a",
									embodimentId: "tile-a--figure",
									emphasis: "active",
								},
							],
						},
					},
				],
			};
			const whole = thought.slides[0]!.arrangement;
			expect(whole.dispositions[0]?.morphTo).toHaveLength(1);
			expect(arrangementRestDispositions(whole)).toHaveLength(1);
		});

		it("starts a new morph run after a fade transition", () => {
			const thought: Thought = {
				id: "fade",
				participants: [{ id: "box" }],
				embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
				slides: [
					{
						arrangement: {
							id: "a",
							dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "b",
							dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
						},
						transition: { kind: "fade" },
					},
					{
						arrangement: {
							id: "c",
							dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
						},
					},
				],
			};
			const expanded = expandThoughtSlides(thought);
			expect(expanded[0]?.autoAnimateId).toBe("fade--m0");
			expect(expanded[1]?.autoAnimateId).toBe("fade--m0");
			expect(expanded[2]?.autoAnimateId).toBeUndefined();
		});

		it("starts a new morph run when consecutive slides share no participants", () => {
			const thought: Thought = {
				id: "media",
				participants: [{ id: "catalogue" }, { id: "col1" }],
				embodiments: [
					{ kind: "figure", id: "catalogue--figure", src: "/a.png" },
					{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
					{ kind: "text", id: "label", lines: ["A"], level: "heading" },
				],
				slides: [
					{
						arrangement: {
							id: "catalogue",
							dispositions: [
								{ participantId: "catalogue", embodimentId: "catalogue--figure", emphasis: "active" },
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "focus",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "crop",
									emphasis: "active",
									position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "labels",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "label",
									emphasis: "active",
									position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
								},
							],
						},
					},
				],
			};
			const expanded = expandThoughtSlides(thought);
			expect(expanded.map((slide) => slide.id)).toEqual(["catalogue", "focus", "labels"]);
			expect(expanded[0]?.autoAnimateId).toBeUndefined();
			expect(expanded[1]?.autoAnimateId).toBe("media--m1");
			expect(expanded[2]?.autoAnimateId).toBe("media--m1");
		});
	});

	describe("centerResolvedArrangement", () => {
		it("offsets placements so their bounding box is centered in the unit slide", () => {
			const resolved: ResolvedDisposition[] = [
				{
					participant: { id: "a" },
					embodiment: { kind: "figure", id: "a--figure", src: "/a.png" },
					embodimentId: "a--figure",
					emphasis: "active",
					morphId: "a",
					position: { x: 0.2, y: 0.1, width: 0.3, height: 0.5 },
				},
			];
			const centered = centerResolvedArrangement(resolved);
			expect(centered[0]?.position?.x).toBeCloseTo(0.35);
			expect(centered[0]?.position?.y).toBeCloseTo(0.25);
		});
	});

	describe("figureFrameForSourceAspect", () => {
		const slideAspect = PRESENTATION_DEFAULT_SLIDE_ASPECT;

		it("matches the source physical aspect inside default padding", () => {
			const sourceAspect = 1536 / 1024;
			const frame = figureFrameForSourceAspect(sourceAspect, slideAspect);
			expect((frame.width / frame.height) * slideAspect).toBeCloseTo(sourceAspect, 10);
			expect(frame.x).toBeCloseTo((1 - frame.width) / 2, 10);
			expect(frame.y).toBeCloseTo((1 - frame.height) / 2, 10);
		});

		it("prefers full width for wider-than-slide sources", () => {
			const frame = figureFrameForSourceAspect(2, slideAspect);
			expect(frame.width).toBeCloseTo(0.92, 10);
		});
	});

	describe("resolveMediaScrollOrigin", () => {
		it("defaults to center when scroll origin is omitted", () => {
			expect(resolveMediaScrollOrigin(undefined)).toEqual({ x: 50, y: 50 });
		});

		it("resolves partial scroll origins and axis percents", () => {
			expect(resolveMediaScrollOrigin({ x: 0 })).toEqual({ x: 0, y: 50 });
			expect(mediaScrollPercentForAxis("x", MEDIA_SCROLL_ORIGIN_TOP_LEFT)).toBe(0);
			expect(mediaScrollPercentForAxis("y", MEDIA_SCROLL_ORIGIN_TOP_LEFT)).toBe(0);
			expect(mediaScrollPercentForAxis("y", MEDIA_SCROLL_ORIGIN_CENTER)).toBe(50);
		});
	});

	describe("splitFigureGrid", () => {
		const frame = { x: 0.1, y: 0.2, width: 0.8, height: 0.6 };

		it("builds rows×columns tiles with frame-relative source crops", () => {
			const tiles = splitFigureGrid({ rows: 3, columns: 5, frame });
			expect(tiles).toHaveLength(15);
			expect(tiles[0]?.key).toBe("tile-r0-c0");
			expect(tiles[0]?.crop.x).toBeCloseTo(0.1, 10);
			expect(tiles[0]?.crop.y).toBeCloseTo(0.2, 10);
			expect(tiles[0]?.crop.width).toBeCloseTo(0.16, 10);
			expect(tiles[0]?.crop.height).toBeCloseTo(0.2, 10);
			expect(tiles[14]?.key).toBe("tile-r2-c4");
			expect(tiles[14]?.crop.x).toBeCloseTo(0.74, 10);
			expect(tiles[14]?.crop.y).toBeCloseTo(0.6, 10);
		});

		it("reconstructs the frame at gap zero", () => {
			const tiles = splitFigureGrid({ rows: 2, columns: 2, frame });
			expect(tiles[0]?.position).toEqual({ x: 0.1, y: 0.2, width: 0.4, height: 0.3 });
			expect(tiles[3]?.position).toEqual({ x: 0.5, y: 0.5, width: 0.4, height: 0.3 });
		});

		it("inserts gap between tile cells", () => {
			const tiles = splitFigureGrid({ rows: 2, columns: 2, frame, gap: 0.05 });
			expect(tiles[1]?.position.x).toBeCloseTo(0.1 + 0.375 + 0.05, 5);
			expect(tiles[1]?.position.width).toBeCloseTo(0.375, 5);
		});

		it("applies default emphasis to every tile when set", () => {
			const tiles = splitFigureGrid({ rows: 1, columns: 2, frame, emphasis: "muted" });
			expect(tiles.every((tile) => tile.emphasis === "muted")).toBe(true);
		});
	});

	describe("unionSourceCrops", () => {
		it("unions normalized crops from grid cells", () => {
			const cells = splitFigureGrid({ rows: 2, columns: 2, frame: { x: 0, y: 0, width: 1, height: 1 } });
			const union = unionSourceCrops(cells.map((cell) => cell.crop));
			expect(union).toEqual({ x: 0, y: 0, width: 1, height: 1 });
		});
	});

	describe("resolveTextMorphRoot", () => {
		it("maps intro title embodiments like eg-ice-25", () => {
			const full: TextEmbodiment = {
				kind: "text",
				id: "full",
				lines: ["A", "B"],
				level: "heading",
				morphRoot: "heading-block",
			};
			const short: TextEmbodiment = {
				kind: "text",
				id: "short",
				lines: ["Short"],
				level: "subheading",
				morphRoot: "subheading-line",
			};
			expect(resolveTextMorphRoot(full)).toBe("heading-block");
			expect(resolveTextMorphRoot(short)).toBe("subheading-line");
		});
	});

	describe("resolveArrangement morphId", () => {
		it("resolves morphId per disposition", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
			const resolved = resolveArrangement(thoughtScope(thought), goal);
			expect(resolved.map((resolvedDisposition) => resolvedDisposition.morphId)).toEqual([
				"title",
				"description",
				"goal",
				"authors",
			]);
		});
	});

	describe("presentationSequences", () => {
		it("flattens sequences in chapter order", () => {
			const deck: Presentation = {
				id: "flat",
				name: "Flat",
				chapters: [
					{ id: "c1", sequences: [{ id: "s1", thoughts: [] }, { id: "s2", thoughts: [] }] },
					{ id: "c2", sequences: [{ id: "s3", thoughts: [] }] },
				],
			};
			expect(presentationSequences(deck).map((sequence) => sequence.id)).toEqual(["s1", "s2", "s3"]);
		});
	});

	describe("collectPresentationSlides", () => {
		it("orders slides by sequence h then arrangement v", () => {
			const deck: Presentation = {
				id: "multi",
				name: "Multi",
				chapters: [
					{
						id: "chapter-a",
						sequences: [
							{
								id: "seq-a",
								thoughts: [
									{
										id: "thought-a",
										participants: [],
										slides: [
											{ arrangement: { id: "a1", dispositions: [] } },
											{ arrangement: { id: "a2", dispositions: [] } },
										],
									},
								],
							},
						],
					},
					{
						id: "chapter-b",
						sequences: [
							{
								id: "seq-b",
								thoughts: [
									{
										id: "thought-b",
										participants: [],
										slides: [{ arrangement: { id: "b1", dispositions: [] } }],
									},
								],
							},
						],
					},
				],
			};
			expect(collectPresentationSlides(deck)).toEqual([
				{ h: 0, v: 0, chapter: "chapter-a", sequence: "seq-a", thought: "thought-a", slide: "a1" },
				{ h: 0, v: 1, chapter: "chapter-a", sequence: "seq-a", thought: "thought-a", slide: "a2" },
				{ h: 1, v: 0, chapter: "chapter-b", sequence: "seq-b", thought: "thought-b", slide: "b1" },
			]);
			expect(presentationSlideAt(deck, { h: 1, v: 0 })?.slide).toBe("b1");
		});
	});

	describe("parsePresentationSlideHash", () => {
		it("round-trips reveal.js hash paths", () => {
			expect(parsePresentationSlideHash("#/")).toEqual({ h: 0, v: 0 });
			expect(parsePresentationSlideHash("#/2/3")).toEqual({ h: 2, v: 3 });
			expect(parsePresentationSlideHash("#/0/2?sequence=main&thought=intro&slide=goal")).toEqual({ h: 0, v: 2 });
			expect(formatPresentationSlideHash({ h: 2, v: 3 })).toBe("/2/3");
		});

		it("formats chapter, sequence, thought, and slide bookmark params after the hash path", () => {
			const bookmark = {
				chapter: "Main",
				sequence: "Introduction",
				thought: "Introduction",
				slide: "Title",
			};
			expect(formatPresentationUrlHash({ h: 0, v: 0 }, bookmark)).toBe(
				"#/?chapter=Main&sequence=Introduction&thought=Introduction&slide=Title",
			);
			expect(formatPresentationUrlHash({ h: 0, v: 2 }, { ...bookmark, slide: "Goal" })).toBe(
				"#/0/2?chapter=Main&sequence=Introduction&thought=Introduction&slide=Goal",
			);
		});

		it("uses German bookmark query keys and titleized bookmark names for de presentations", () => {
			const bookmark = {
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
				slide: "Universitäten",
			};
			const hash = formatPresentationUrlHash({ h: 0, v: 5 }, bookmark, "de");
			expect(hash.startsWith("#/0/5?")).toBe(true);
			const params = new URLSearchParams(hash.split("?")[1] ?? "");
			expect(params.get("kapitel")).toBe("Hauptteil");
			expect(params.get("sequenz")).toBe("Einführung");
			expect(params.get("gedanke")).toBe("Einleitung");
			expect(params.get("folie")).toBe("Universitäten");
			expect(presentationSlideBookmarkParamKeys("de")).toEqual({
				chapter: "kapitel",
				sequence: "sequenz",
				thought: "gedanke",
				slide: "folie",
			});
		});
	});

	describe("analogy", () => {
		const sampleAnalogy = analogy({
			source: { label: "Reuse", figure: "/reuse.png" },
			target: { label: "Remanufacture", figure: "/remanufacture.png" },
		});

		it("builds two morph slides", () => {
			const thought = sampleAnalogy.chapters[0]!.sequences[0]!.thoughts[0]!;
			expect(thought.slides.map((slide) => slide.arrangement.id)).toEqual(["source", "mapping"]);
			expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual(["source", "mapping"]);
		});

		it("resolves positioned visual dispositions", () => {
			const thought = sampleAnalogy.chapters[0]!.sequences[0]!.thoughts[0]!;
			const mapping = thought.slides.find((slide) => slide.arrangement.id === "mapping")!.arrangement;
			const resolved = resolveArrangement(thoughtScope(thought), mapping);
			const visual = resolved.find((resolvedDisposition) => resolvedDisposition.participant.id === ANALOGY_PARTICIPANT_VISUAL);
			expect(visual?.position).toEqual({ x: 0.1, y: 0.35, width: 0.8, height: 0.5 });
			expect(visual?.embodiment.kind).toBe("figure");
		});
	});

	describe("video and pdf embodiments", () => {
		it("resolves video and pdf kinds", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "clip" }, { id: "doc" }],
					embodiments: [
						{ kind: "video", id: "clip--video", src: "/demo.mp4", muted: true },
						{ kind: "pdf", id: "doc--pdf", src: "/paper.pdf", page: 2 },
					],
				},
			]);
			const arrangement: Arrangement = {
				id: "slide",
				dispositions: [
					{ participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
					{
						participantId: "doc",
						embodimentId: "doc--pdf",
						emphasis: "active",
						position: { x: 0.2, y: 0.2, width: 0.6, height: 0.6 },
					},
				],
			};
			const resolved = resolveArrangement(scope, arrangement);
			expect(resolved[0]?.embodiment.kind).toBe("video");
			expect(resolved[1]?.embodiment.kind).toBe("pdf");
			if (resolved[1]?.embodiment.kind === "pdf") {
				expect(resolved[1].embodiment.page).toBe(2);
				expect(resolved[1].embodiment.pages).toBeUndefined();
			}
			expect(resolved[1]?.position?.width).toBe(0.6);
		});

		it("resolves pdf embodiments with a page subset", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "doc" }],
					embodiments: [
						{
							kind: "pdf",
							id: "doc--pdf",
							src: "/thesis.pdf",
							page: 25,
							pages: [1, 12, 25, 35],
						},
					],
				},
			]);
			const resolved = resolveArrangement(scope, {
				id: "slide",
				dispositions: [{ participantId: "doc", embodimentId: "doc--pdf", emphasis: "active" }],
			});
			expect(resolved[0]?.embodiment.kind).toBe("pdf");
			if (resolved[0]?.embodiment.kind === "pdf") {
				expect(resolved[0].embodiment.pages).toEqual([1, 12, 25, 35]);
			}
		});
	});

	describe("tile play", () => {
		const source: FigureTileSource = {
			src: "/catalogue.png",
			sourceAspect: 1222 / 896,
			frame: { x: 0.127, y: 0.1, width: 0.746, height: 0.75 },
		};

		it("seeds grid drafts with frame-relative crops", () => {
			const drafts = populateTileDraftsFromGrid({ source, rows: 2, columns: 2 });
			expect(drafts).toHaveLength(4);
			expect(drafts[0]?.id).toBe("tile-r0-c0");
			expect(drafts[0]?.crop.x).toBeCloseTo(source.frame.x, 10);
		});

		it("parses grid engagement tokens", () => {
			expect(parseGridEngagement("3x5")).toEqual({ rows: 3, columns: 5 });
			expect(parseGridEngagement("2×4")).toEqual({ rows: 2, columns: 4 });
			expect(parseGridEngagement("add")).toBeNull();
		});

		it("clamps move and resize to the unit square", () => {
			const rect = { x: 0.8, y: 0.8, width: 0.15, height: 0.15 };
			const moved = moveNormalizedRect(rect, 0.2, 0.2);
			expect(moved.x + moved.width).toBeLessThanOrEqual(1.001);
			expect(moved.y + moved.height).toBeLessThanOrEqual(1.001);
			const resized = resizeNormalizedRect({ x: 0.1, y: 0.1, width: 0.4, height: 0.4 }, "se", 0.5, 0.5);
			expect(resized.width).toBeLessThanOrEqual(0.9);
			expect(resized.height).toBeLessThanOrEqual(0.9);
		});

		it("allows overlapping tile crops in the morph prompt", () => {
			const drafts: FigureTileDraft[] = [
				{ id: "a", name: "Tile A", crop: { x: 0.1, y: 0.1, width: 0.5, height: 0.5 } },
				{ id: "b", name: "Tile B", crop: { x: 0.3, y: 0.3, width: 0.5, height: 0.5 } },
			];
			const prompt = buildTileMorphPrompt(source, drafts);
			expect(prompt).toContain("Tile A");
			expect(prompt).toContain("Tile B");
			expect(prompt).toContain("mit-bestand/präsentation/33.projektetage/spec.ts");
			expect(prompt).toContain("morphTo");
		});

		it("detects supported tile media kinds from file metadata", () => {
			expect(figureTileMediaKindFromFile("image/png", "photo.png")).toBe("figure");
			expect(figureTileMediaKindFromFile("image/svg+xml", "icon.svg")).toBe("figure");
			expect(figureTileMediaKindFromFile("video/mp4", "clip.mp4")).toBe("video");
			expect(figureTileMediaKindFromFile("application/pdf", "doc.pdf")).toBe("pdf");
			expect(figureTileMediaKindFromFile("", "notes.pdf")).toBe("pdf");
			expect(isFigureTileMediaFile("text/plain", "readme.txt")).toBe(false);
		});

		it("embeds video and pdf kind in the morph prompt", () => {
			const prompt = buildTileMorphPrompt(
				{ src: "/clip.mp4", kind: "video", sourceAspect: 16 / 9, frame: { x: 0, y: 0, width: 1, height: 1 } },
				[{ id: "t1", name: "Intro", crop: { x: 0, y: 0, width: 0.5, height: 0.5 } }],
			);
			expect(prompt).toContain("kind: video");
			expect(prompt).toContain("video(...)");
			const pdfPrompt = buildTileMorphPrompt(
				{ src: "/paper.pdf", kind: "pdf", pdfPage: 2, sourceAspect: FIGURE_TILE_PDF_PAGE_ASPECT, frame: { x: 0, y: 0, width: 1, height: 1 } },
				[],
			);
			expect(pdfPrompt).toContain("kind: pdf");
			expect(pdfPrompt).toContain("pdfPage: 2");
		});
	});

	describe("createPresentationAppVcsHandler", () => {
		it("materializes deck projection from inline json", () => {
			const handler = createPresentationAppVcsHandler();
			const projection = handler.materializeProjection({
				inline: JSON.stringify({ schema: "presentation.deck", source: { src: "/a.png" }, tiles: [{ id: "t1", name: "A", crop: { x: 0, y: 0, width: 1, height: 1 } }] }),
			}) as { tiles: readonly unknown[] };
			expect(projection.tiles).toHaveLength(1);
		});
	});
}
//#endregion 🧪Tests
