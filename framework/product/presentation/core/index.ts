// #region 🧱Header
/** 🧱 `@framework/presentation/core` — Render-independent declarative presentations: {@link Presentation}, {@link Sequence}, {@link Thought}, {@link Participant}, {@link Embodiment}, {@link Arrangement}, {@link Transition}, and {@link intro}. */
// #endregion 🧱Header

//#region 🔖Emphasis
/** @emoji 🎚 Visual emphasis for a participant on one slide (maps to opacity layering in renderers). */
export type ParticipantEmphasis = "active" | "muted";
//#endregion 🔖Emphasis

//#region 🔖Transition
/** @emoji ↔️ Transition between arrangements within a thought. */
export interface Transition {
	readonly kind: "morph" | "fade";
}
//#endregion 🔖Transition

//#region 🔖Morph
/** @emoji 📐 reveal.js auto-animate DOM root for {@link TextEmbodiment} (see temp/eg-ice-25 intro slides). */
export type TextMorphRoot = "title" | "heading-block" | "heading-line" | "subheading-line" | "body";

/** @emoji 🎯 Stable reveal.js `data-id` for a placed participant (participant-scoped across embodiments). */
export function morphId(participantId: string): string {
	return participantId;
}

/** @emoji 📐 Chooses the eg-ice-25 text DOM root for reveal.js `data-id` pairing. */
export function resolveTextMorphRoot(embodiment: TextEmbodiment): TextMorphRoot {
	if (embodiment.morphRoot) {
		return embodiment.morphRoot;
	}
	if (embodiment.level === "title") {
		return "title";
	}
	if (embodiment.level === "body") {
		return "body";
	}
	if (embodiment.lines.length === 1) {
		return embodiment.level === "subheading" ? "subheading-line" : "heading-line";
	}
	return "heading-block";
}
//#endregion 🔖Morph

//#region 🔖Embodiment
/** @emoji 📝 Text lines at a heading level; optional `fit` hints fit-text in reveal renderers. */
export interface TextEmbodiment {
	readonly kind: "text";
	readonly id?: string;
	readonly lines: readonly string[];
	readonly level: "title" | "heading" | "subheading" | "body";
	readonly fit?: boolean;
	readonly morphRoot?: TextMorphRoot;
}

/** @emoji 🖼 Raster or vector figure on a slide. */
export interface FigureEmbodiment {
	readonly kind: "figure";
	readonly id?: string;
	readonly src: string;
	readonly alt?: string;
}

/** @emoji • Bulleted list body. */
export interface BulletEmbodiment {
	readonly kind: "bullet";
	readonly id?: string;
	readonly items: readonly string[];
}

/** @emoji 🔢 One superscript affiliation mark on an author (optional fade for marks introduced earlier). */
export interface AuthorMark {
	readonly mark: string;
	readonly emphasis?: ParticipantEmphasis;
}

/** @emoji 👤 One author name with optional affiliation marks. */
export interface AuthorPerson {
	readonly name: string;
	readonly marks?: readonly string[];
	readonly markEntries?: readonly AuthorMark[];
}

/** @emoji 👤 Author rows (names with optional superscript marks); use `lines` for multiple rows. */
export interface AuthorsEmbodiment {
	readonly kind: "authors";
	readonly id?: string;
	readonly people?: readonly AuthorPerson[];
	readonly lines?: readonly (readonly AuthorPerson[])[];
	readonly abbreviateFirstName?: boolean;
}

/** @emoji 👤 Abbreviates the first given name (`Ueli Saluz` → `U. Saluz`) for compact author rows with affiliation marks. */
export function abbreviateAuthorFirstName(fullName: string): string {
	const parts = fullName.trim().split(/\s+/).filter(Boolean);
	if (parts.length < 2) {
		return fullName.trim();
	}
	const [first, ...rest] = parts;
	const initial = first.codePointAt(0);
	if (initial === undefined) {
		return fullName.trim();
	}
	return `${String.fromCodePoint(initial).toLocaleUpperCase("de-DE")}. ${rest.join(" ")}`;
}

/** @emoji 🏛 One affiliation line with optional second mark+name on the same row (e.g. university + chair). */
export interface AffiliationEntry {
	readonly mark: string;
	readonly name: string;
	readonly shortName?: string;
	readonly suffix?: { readonly mark: string; readonly name: string };
	readonly lineEmphasis?: ParticipantEmphasis;
	readonly suffixEmphasis?: ParticipantEmphasis;
}

/** @emoji 🏛 Affiliation line label (`shortName` when set, else full `name`). */
export function affiliationLineName(entry: AffiliationEntry): string {
	return entry.shortName ?? entry.name;
}

/** @emoji 🏛 Collects `mark` and `suffix.mark` values present in one affiliation step. */
export function affiliationMarksInStep(step: readonly AffiliationEntry[]): ReadonlySet<string> {
	const marks = new Set<string>();
	for (const entry of step) {
		marks.add(entry.mark);
		if (entry.suffix) {
			marks.add(entry.suffix.mark);
		}
	}
	return marks;
}

/** @emoji 🏛 Footnote mark order as listed in one affiliation step (line marks, then suffix marks). */
export function affiliationMarkOrderInStep(step: readonly AffiliationEntry[]): readonly string[] {
	const order: string[] = [];
	for (const entry of step) {
		if (!order.includes(entry.mark)) {
			order.push(entry.mark);
		}
		if (entry.suffix && !order.includes(entry.suffix.mark)) {
			order.push(entry.suffix.mark);
		}
	}
	return order;
}

/** @emoji 👤 Author rows with only marks defined in `currentStep`; newer marks vs `previousStep` stay active. */
export function authorLinesForAffiliationStep(
	lines: readonly (readonly AuthorPerson[])[],
	currentStep: readonly AffiliationEntry[],
	previousStep: readonly AffiliationEntry[],
): readonly (readonly AuthorPerson[])[] {
	const allowed = affiliationMarksInStep(currentStep);
	const previous = affiliationMarksInStep(previousStep);
	const markOrder = affiliationMarkOrderInStep(currentStep);
	const markRank = new Map(markOrder.map((mark, index) => [mark, index]));
	return lines.map((line) =>
		line.map((author) => {
			const markEntries = (author.marks ?? [])
				.filter((mark) => allowed.has(mark))
				.map((mark) => ({
					mark,
					emphasis: previous.has(mark) ? ("muted" as const) : ("active" as const),
				}))
				.sort((left, right) => (markRank.get(left.mark) ?? 0) - (markRank.get(right.mark) ?? 0));
			return { name: author.name, markEntries };
		}),
	);
}

/** @emoji 🏛 Mutes prior affiliation lines; only marks or suffixes new vs `previousStep` stay active. */
export function highlightAffiliationDelta(
	currentStep: readonly AffiliationEntry[],
	previousStep: readonly AffiliationEntry[],
): AffiliationEntry[] {
	const previousByMark = new Map(previousStep.map((entry) => [entry.mark, entry]));
	return currentStep.map((entry) => {
		const previous = previousByMark.get(entry.mark);
		if (!previous) {
			return { ...entry, lineEmphasis: "active" as const };
		}
		if (entry.suffix && !previous.suffix) {
			return {
				...entry,
				lineEmphasis: "muted" as const,
				suffixEmphasis: "active" as const,
			};
		}
		return { ...entry, lineEmphasis: "muted" as const, suffixEmphasis: "muted" as const };
	});
}

/** @emoji 🏛 Affiliation footnotes keyed by mark. */
export interface AffiliationsEmbodiment {
	readonly kind: "affiliations";
	readonly id?: string;
	readonly entries: readonly AffiliationEntry[];
}

/** @emoji 🎭 One visual form a {@link Participant} may take on a slide. */
export type Embodiment = TextEmbodiment | FigureEmbodiment | BulletEmbodiment | AuthorsEmbodiment | AffiliationsEmbodiment;
//#endregion 🔖Embodiment

//#region 🔖Participant
/** @emoji 🧑 Entity that may appear across arrangements (title, authors, …). */
export interface Participant {
	readonly id: string;
	readonly embodiments: readonly Embodiment[];
}
//#endregion 🔖Participant

//#region 🔖Placement
/** @emoji 📍 Which embodiment of which participant appears on one arrangement and how strongly. */
export interface ParticipantPlacement {
	readonly participantId: string;
	readonly embodimentId?: string;
	readonly emphasis: ParticipantEmphasis;
}
//#endregion 🔖Placement

//#region 🔖Arrangement
/** @emoji 🖼 One slide: participants placed with emphasis. */
export interface Arrangement {
	readonly id: string;
	readonly placements: readonly ParticipantPlacement[];
}
//#endregion 🔖Arrangement

//#region 🔖Thought
/** @emoji 💭 Idea developed across one or more slides with shared participants. */
export interface Thought {
	readonly id: string;
	readonly participants: readonly Participant[];
	readonly arrangements: readonly Arrangement[];
	readonly transition?: Transition;
}
//#endregion 🔖Thought

//#region 🔖Sequence
/** @emoji 📚 Horizontal chapter: a sequence of thoughts. */
export interface Sequence {
	readonly id: string;
	readonly thoughts: readonly Thought[];
}
//#endregion 🔖Sequence

//#region 🔖Presentation
/** @emoji 📽 Root deck: ordered sequences of thoughts. */
export interface Presentation {
	readonly id: string;
	readonly name: string;
	readonly sequences: readonly Sequence[];
	readonly width?: number;
	readonly height?: number;
}
//#endregion 🔖Presentation

//#region 🔖Resolved
/** @emoji ✅ One participant embodiment resolved for rendering a single arrangement. */
export interface ResolvedPlacement {
	readonly participant: Participant;
	readonly embodiment: Embodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly embodimentId?: string;
	readonly morphId: string;
}
//#endregion 🔖Resolved

//#region 🔖Resolve
/** @emoji 🔍 Picks the embodiment for a participant (by id or first). */
export function resolveEmbodiment(participant: Participant, embodimentId?: string): Embodiment {
	if (embodimentId) {
		const match = participant.embodiments.find((e) => e.id === embodimentId);
		if (match) {
			return match;
		}
		throw new Error(`Participant "${participant.id}" has no embodiment "${embodimentId}".`);
	}
	const first = participant.embodiments[0];
	if (!first) {
		throw new Error(`Participant "${participant.id}" has no embodiments.`);
	}
	return first;
}

/** @emoji 🔍 Resolves all placements for one arrangement within a thought. */
export function resolveArrangement(thought: Thought, arrangementId: string): ResolvedPlacement[] {
	const arrangement = thought.arrangements.find((a) => a.id === arrangementId);
	if (!arrangement) {
		throw new Error(`Thought "${thought.id}" has no arrangement "${arrangementId}".`);
	}
	const byId = new Map(thought.participants.map((p) => [p.id, p]));
	return arrangement.placements.map((placement) => {
		const participant = byId.get(placement.participantId);
		if (!participant) {
			throw new Error(`Thought "${thought.id}" has no participant "${placement.participantId}".`);
		}
		return {
			participant,
			embodiment: resolveEmbodiment(participant, placement.embodimentId),
			emphasis: placement.emphasis,
			embodimentId: placement.embodimentId,
			morphId: morphId(participant.id),
		};
	});
}

/** @emoji 🔢 Counts slides (arrangements) across all sequences. */
export function countArrangements(presentation: Presentation): number {
	return presentation.sequences.reduce(
		(sum, seq) => sum + seq.thoughts.reduce((tSum, thought) => tSum + thought.arrangements.length, 0),
		0,
	);
}
//#endregion 🔖Resolve

//#region 🔖Intro
/** @emoji 🎬 Spec for the standard paper intro template (title → description → goal → authors → affiliations ×3). */
export interface IntroSpec {
	readonly id?: string;
	readonly name?: string;
	readonly title: {
		readonly full: readonly string[];
		readonly short: string;
	};
	readonly description: {
		readonly full: readonly string[];
		readonly short: string;
	};
	readonly goal: readonly string[];
	readonly authors: {
		readonly lines: readonly (readonly AuthorPerson[])[];
	};
	readonly affiliations: {
		readonly steps: readonly [
			readonly AffiliationEntry[],
			readonly AffiliationEntry[],
			readonly AffiliationEntry[],
		];
	};
}

const INTRO_PARTICIPANT_TITLE = "title";
const INTRO_PARTICIPANT_DESCRIPTION = "description";
const INTRO_PARTICIPANT_GOAL = "goal";
const INTRO_PARTICIPANT_AUTHORS = "authors";
const INTRO_PARTICIPANT_INSTITUTIONS = "institutions";

const INTRO_EMBODIMENT_TITLE_FULL = "full";
const INTRO_EMBODIMENT_TITLE_SHORT = "short";
const INTRO_EMBODIMENT_DESCRIPTION_FULL = "full";
const INTRO_EMBODIMENT_DESCRIPTION_SHORT = "short";
const INTRO_EMBODIMENT_AUTHORS_PLAIN = "plain";
const INTRO_EMBODIMENT_AUTHORS_MARKED = "marked";
const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1 = "marked-affiliations-step1";
const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2 = "marked-affiliations-step2";
const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP3 = "marked-affiliations-step3";
const INTRO_EMBODIMENT_INSTITUTIONS_STEP1 = "step1";
const INTRO_EMBODIMENT_INSTITUTIONS_STEP2 = "step2";
const INTRO_EMBODIMENT_INSTITUTIONS_STEP3 = "step3";

/** @emoji 🎬 Builds a seven-slide intro; each arrangement is that slide's target content for reveal.js auto-animate. */
export function intro(spec: IntroSpec): Presentation {
	const thoughtId = "intro";
	const participants: Participant[] = [
		{
			id: INTRO_PARTICIPANT_TITLE,
			embodiments: [
				{
					kind: "text",
					id: INTRO_EMBODIMENT_TITLE_FULL,
					lines: spec.title.full,
					level: "heading",
					morphRoot: "heading-block",
				},
				{
					kind: "text",
					id: INTRO_EMBODIMENT_TITLE_SHORT,
					lines: [spec.title.short],
					level: "heading",
					morphRoot: "heading-block",
				},
			],
		},
		{
			id: INTRO_PARTICIPANT_DESCRIPTION,
			embodiments: [
				{
					kind: "text",
					id: INTRO_EMBODIMENT_DESCRIPTION_FULL,
					lines: spec.description.full,
					level: "heading",
					morphRoot: "heading-block",
				},
				{
					kind: "text",
					id: INTRO_EMBODIMENT_DESCRIPTION_SHORT,
					lines: [spec.description.short],
					level: "heading",
					morphRoot: "heading-block",
				},
			],
		},
		{
			id: INTRO_PARTICIPANT_GOAL,
			embodiments: [
				{
					kind: "text",
					lines: spec.goal,
					level: "heading",
					morphRoot: "heading-block",
				},
			],
		},
		{
			id: INTRO_PARTICIPANT_AUTHORS,
			embodiments: [
				{
					kind: "authors",
					id: INTRO_EMBODIMENT_AUTHORS_PLAIN,
					lines: spec.authors.lines.map((line) => line.map((a) => ({ name: a.name }))),
				},
				{
					kind: "authors",
					id: INTRO_EMBODIMENT_AUTHORS_MARKED,
					lines: spec.authors.lines,
				},
				{
					kind: "authors",
					id: INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1,
					lines: authorLinesForAffiliationStep(spec.authors.lines, spec.affiliations.steps[0], []),
					abbreviateFirstName: true,
				},
				{
					kind: "authors",
					id: INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2,
					lines: authorLinesForAffiliationStep(
						spec.authors.lines,
						spec.affiliations.steps[1],
						spec.affiliations.steps[0],
					),
					abbreviateFirstName: true,
				},
				{
					kind: "authors",
					id: INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP3,
					lines: authorLinesForAffiliationStep(
						spec.authors.lines,
						spec.affiliations.steps[2],
						spec.affiliations.steps[1],
					),
					abbreviateFirstName: true,
				},
			],
		},
		{
			id: INTRO_PARTICIPANT_INSTITUTIONS,
			embodiments: [
				{
					kind: "affiliations",
					id: INTRO_EMBODIMENT_INSTITUTIONS_STEP1,
					entries: spec.affiliations.steps[0],
				},
				{
					kind: "affiliations",
					id: INTRO_EMBODIMENT_INSTITUTIONS_STEP2,
					entries: highlightAffiliationDelta(spec.affiliations.steps[1], spec.affiliations.steps[0]),
				},
				{
					kind: "affiliations",
					id: INTRO_EMBODIMENT_INSTITUTIONS_STEP3,
					entries: highlightAffiliationDelta(spec.affiliations.steps[2], spec.affiliations.steps[1]),
				},
			],
		},
	];

	const muted = (participantId: string, embodimentId?: string): ParticipantPlacement => ({
		participantId,
		...(embodimentId ? { embodimentId } : {}),
		emphasis: "muted",
	});

	const active = (participantId: string, embodimentId?: string): ParticipantPlacement => ({
		participantId,
		...(embodimentId ? { embodimentId } : {}),
		emphasis: "active",
	});

	const introMutedAboveAuthors = [
		muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
		muted(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_SHORT),
		muted(INTRO_PARTICIPANT_GOAL),
	];

	const thought: Thought = {
		id: thoughtId,
		participants,
		transition: { kind: "morph" },
		arrangements: [
			{
				id: "title",
				placements: [active(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_FULL)],
			},
			{
				id: "description",
				placements: [
					muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
					active(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_FULL),
				],
			},
			{
				id: "goal",
				placements: [
					muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
					muted(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_SHORT),
					active(INTRO_PARTICIPANT_GOAL),
				],
			},
			{
				id: "authors",
				placements: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_PLAIN),
				],
			},
			{
				id: "affiliations-1",
				placements: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1),
					active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP1),
				],
			},
			{
				id: "affiliations-2",
				placements: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2),
					active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP2),
				],
			},
			{
				id: "affiliations-3",
				placements: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP3),
					active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP3),
				],
			},
		],
	};

	return {
		id: spec.id ?? "presentation",
		name: spec.name ?? spec.title.short,
		sequences: [{ id: "main", thoughts: [thought] }],
	};
}
//#endregion 🔖Intro

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

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

	describe("intro", () => {
		it("builds seven arrangements in one thought", () => {
			expect(countArrangements(sampleIntro)).toBe(7);
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			expect(thought.arrangements.map((a) => a.id)).toEqual([
				"title",
				"description",
				"goal",
				"authors",
				"affiliations-1",
				"affiliations-2",
				"affiliations-3",
			]);
		});

		it("uses fixed-size heading blocks without fit-text", () => {
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			const textEmbodiments = thought.participants.flatMap((p) =>
				p.embodiments.filter((e): e is TextEmbodiment => e.kind === "text"),
			);
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

		it("abbreviates author first names on affiliation slides", () => {
			expect(abbreviateAuthorFirstName("Ueli Saluz")).toBe("U. Saluz");
			expect(abbreviateAuthorFirstName("Christoph Gengnagel")).toBe("C. Gengnagel");
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			const authors = resolveArrangement(thought, "affiliations-1").find(
				(r) => r.participant.id === INTRO_PARTICIPANT_AUTHORS,
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
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			const step2 = resolveArrangement(thought, "affiliations-2").find(
				(r) => r.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step2.embodiment.kind === "affiliations") {
				expect(step2.embodiment.entries.find((e) => e.mark === "1")?.lineEmphasis).toBe("active");
				expect(step2.embodiment.entries.find((e) => e.mark === "a")?.lineEmphasis).toBe("muted");
			}
			const step3 = resolveArrangement(thought, "affiliations-3").find(
				(r) => r.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step3.embodiment.kind === "affiliations") {
				const uni = step3.embodiment.entries.find((e) => e.mark === "1");
				expect(uni?.lineEmphasis).toBe("muted");
				expect(uni?.suffixEmphasis).toBe("active");
				expect(step3.embodiment.entries.find((e) => e.mark === "a")?.lineEmphasis).toBe("muted");
			}
		});
	});

	describe("resolveEmbodiment", () => {
		it("throws when embodiment id is missing", () => {
			const participant: Participant = {
				id: "x",
				embodiments: [{ kind: "text", lines: ["a"], level: "body" }],
			};
			expect(() => resolveEmbodiment(participant, "missing")).toThrow(/no embodiment/);
		});
	});

	describe("morphId", () => {
		it("uses participant id as reveal data-id", () => {
			expect(morphId("title")).toBe("title");
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
		it("resolves morphId per placement", () => {
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			const resolved = resolveArrangement(thought, "goal");
			expect(resolved.map((r) => r.morphId)).toEqual(["title", "description", "goal"]);
		});
	});
}
//#endregion 🧪Tests
