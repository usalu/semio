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

/** @emoji 👤 Author row (names with optional superscript marks). */
export interface AuthorsEmbodiment {
	readonly kind: "authors";
	readonly id?: string;
	readonly people: readonly { readonly name: string; readonly marks?: readonly string[] }[];
}

/** @emoji 🏛 Affiliation footnotes keyed by mark. */
export interface AffiliationsEmbodiment {
	readonly kind: "affiliations";
	readonly id?: string;
	readonly entries: readonly { readonly mark: string; readonly name: string }[];
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
/** @emoji 🎬 Spec for the standard paper intro template (brand → title → subtitle → authors → affiliations). */
export interface IntroSpec {
	readonly id?: string;
	readonly name?: string;
	readonly brand: string;
	readonly title: {
		readonly full: readonly string[];
		readonly short: string;
	};
	readonly description: readonly string[];
	readonly authors: readonly { readonly name: string; readonly marks?: readonly string[] }[];
	readonly affiliations: readonly { readonly mark: string; readonly name: string }[];
}

const INTRO_PARTICIPANT_NAME = "name";
const INTRO_PARTICIPANT_TITLE = "title";
const INTRO_PARTICIPANT_SUBTITLE = "subtitle";
const INTRO_PARTICIPANT_AUTHORS = "authors";
const INTRO_PARTICIPANT_INSTITUTIONS = "institutions";

const INTRO_EMBODIMENT_TITLE_FULL = "full";
const INTRO_EMBODIMENT_TITLE_SHORT = "short";
const INTRO_EMBODIMENT_AUTHORS_PLAIN = "plain";
const INTRO_EMBODIMENT_AUTHORS_MARKED = "marked";

/** @emoji 🎬 Builds a five-slide intro thought (brand → title → subtitle → authors → affiliations). */
export function intro(spec: IntroSpec): Presentation {
	const thoughtId = "intro";
	const participants: Participant[] = [
		{
			id: INTRO_PARTICIPANT_NAME,
			embodiments: [{ kind: "text", lines: [spec.brand], level: "title" }],
		},
		{
			id: INTRO_PARTICIPANT_TITLE,
			embodiments: [
				{
					kind: "text",
					id: INTRO_EMBODIMENT_TITLE_FULL,
					lines: spec.title.full,
					level: "heading",
					fit: true,
					morphRoot: "heading-block",
				},
				{
					kind: "text",
					id: INTRO_EMBODIMENT_TITLE_SHORT,
					lines: [spec.title.short],
					level: "subheading",
					fit: true,
					morphRoot: "subheading-line",
				},
			],
		},
		{
			id: INTRO_PARTICIPANT_SUBTITLE,
			embodiments: [
				{
					kind: "text",
					lines: spec.description,
					level: "heading",
					fit: true,
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
					people: spec.authors.map((a) => ({ name: a.name })),
				},
				{
					kind: "authors",
					id: INTRO_EMBODIMENT_AUTHORS_MARKED,
					people: spec.authors,
				},
			],
		},
		{
			id: INTRO_PARTICIPANT_INSTITUTIONS,
			embodiments: [{ kind: "affiliations", entries: spec.affiliations }],
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

	const thought: Thought = {
		id: thoughtId,
		participants,
		transition: { kind: "morph" },
		arrangements: [
			{ id: "brand", placements: [active(INTRO_PARTICIPANT_NAME)] },
			{
				id: "title",
				placements: [muted(INTRO_PARTICIPANT_NAME), active(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_FULL)],
			},
			{
				id: "subtitle",
				placements: [
					muted(INTRO_PARTICIPANT_NAME),
					muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
					active(INTRO_PARTICIPANT_SUBTITLE),
				],
			},
			{
				id: "authors",
				placements: [
					muted(INTRO_PARTICIPANT_NAME),
					muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
					muted(INTRO_PARTICIPANT_SUBTITLE),
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_PLAIN),
				],
			},
			{
				id: "institutions",
				placements: [
					muted(INTRO_PARTICIPANT_NAME),
					muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
					muted(INTRO_PARTICIPANT_SUBTITLE),
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED),
					active(INTRO_PARTICIPANT_INSTITUTIONS),
				],
			},
		],
	};

	return {
		id: spec.id ?? "presentation",
		name: spec.name ?? spec.brand,
		sequences: [{ id: "main", thoughts: [thought] }],
	};
}
//#endregion 🔖Intro

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	const sampleIntro = intro({
		brand: "semio",
		title: {
			full: ["Line A", "Line B", "Line C"],
			short: "Short title",
		},
		description: ["D1", "D2", "D3"],
		authors: [
			{ name: "Alice" },
			{ name: "Bob", marks: ["1", "b"] },
		],
		affiliations: [
			{ mark: "1", name: "University" },
			{ mark: "b", name: "Faculty" },
		],
	});

	describe("intro", () => {
		it("builds five arrangements in one thought", () => {
			expect(countArrangements(sampleIntro)).toBe(5);
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			expect(thought.arrangements.map((a) => a.id)).toEqual(["brand", "title", "subtitle", "authors", "institutions"]);
		});

		it("layers muted participants on the institutions slide", () => {
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			const resolved = resolveArrangement(thought, "institutions");
			expect(resolved.map((r) => r.participant.id)).toEqual([
				INTRO_PARTICIPANT_NAME,
				INTRO_PARTICIPANT_TITLE,
				INTRO_PARTICIPANT_SUBTITLE,
				INTRO_PARTICIPANT_AUTHORS,
				INTRO_PARTICIPANT_INSTITUTIONS,
			]);
			expect(resolved[0]!.emphasis).toBe("muted");
			expect(resolved[4]!.emphasis).toBe("active");
			expect(resolved[3]!.embodiment.kind).toBe("authors");
			if (resolved[3]!.embodiment.kind === "authors") {
				expect(resolved[3]!.embodiment.id).toBe(INTRO_EMBODIMENT_AUTHORS_MARKED);
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
			const resolved = resolveArrangement(thought, "subtitle");
			expect(resolved.map((r) => r.morphId)).toEqual(["name", "title", "subtitle"]);
		});
	});
}
//#endregion 🧪Tests
