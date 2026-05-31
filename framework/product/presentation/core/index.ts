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

/** @emoji 👤 One author name with optional affiliation marks. */
export interface AuthorPerson {
	readonly name: string;
	readonly marks?: readonly string[];
}

/** @emoji 👤 Author rows (names with optional superscript marks); use `lines` for multiple rows. */
export interface AuthorsEmbodiment {
	readonly kind: "authors";
	readonly id?: string;
	readonly people?: readonly AuthorPerson[];
	readonly lines?: readonly (readonly AuthorPerson[])[];
}

/** @emoji 🏛 One affiliation line with optional second mark+name on the same row (e.g. university + chair). */
export interface AffiliationEntry {
	readonly mark: string;
	readonly name: string;
	readonly suffix?: { readonly mark: string; readonly name: string };
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
					entries: spec.affiliations.steps[1],
				},
				{
					kind: "affiliations",
					id: INTRO_EMBODIMENT_INSTITUTIONS_STEP3,
					entries: spec.affiliations.steps[2],
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
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED),
					active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP1),
				],
			},
			{
				id: "affiliations-2",
				placements: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED),
					active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP2),
				],
			},
			{
				id: "affiliations-3",
				placements: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED),
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
				[
					{ mark: "1", name: "University" },
					{ mark: "2", name: "Other University" },
				],
				[
					{ mark: "1", name: "University" },
					{ mark: "2", name: "Other University" },
					{ mark: "a", name: "Faculty" },
				],
				[
					{ mark: "1", name: "University", suffix: { mark: "x", name: "Chair X" } },
					{ mark: "2", name: "Other University", suffix: { mark: "y", name: "Chair Y" } },
					{ mark: "a", name: "Faculty" },
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

		it("shows one institutions block per affiliations slide (chairs extend marks 1 and 2)", () => {
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			for (const arrangementId of ["affiliations-1", "affiliations-2", "affiliations-3"] as const) {
				const resolved = resolveArrangement(thought, arrangementId);
				const institutions = resolved.filter((r) => r.participant.id === INTRO_PARTICIPANT_INSTITUTIONS);
				expect(institutions).toHaveLength(1);
				expect(institutions[0]!.emphasis).toBe("active");
			}
			const step3 = resolveArrangement(thought, "affiliations-3").find(
				(r) => r.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step3.embodiment.kind === "affiliations") {
				expect(step3.embodiment.id).toBe(INTRO_EMBODIMENT_INSTITUTIONS_STEP3);
				expect(step3.embodiment.entries[0]?.suffix?.mark).toBe("x");
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
