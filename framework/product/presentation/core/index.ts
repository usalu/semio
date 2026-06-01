// #region 🧱Header
/** 🧱 `@framework/presentation/core` — Render-independent declarative presentations: {@link Presentation}, {@link Sequence}, {@link Thought}, {@link Participant}, {@link Embodiment}, {@link Disposition}, {@link Arrangement}, {@link Transition}, {@link intro}, and {@link analogy}. */
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

/** @emoji 🧩 Stable reveal.js `data-id` for one crop tile of a split figure disposition. */
export function tileMorphId(participantId: string, tileKey: string): string {
	return `${participantId}--tile--${tileKey}`;
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

/** @emoji 🎬 Video clip on a slide. */
export interface VideoEmbodiment {
	readonly kind: "video";
	readonly id?: string;
	readonly src: string;
	readonly poster?: string;
	readonly autoplay?: boolean;
	readonly loop?: boolean;
	readonly muted?: boolean;
	readonly controls?: boolean;
}

/** @emoji 📄 PDF document page on a slide. */
export interface PdfEmbodiment {
	readonly kind: "pdf";
	readonly id?: string;
	readonly src: string;
	readonly page?: number;
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
export type Embodiment =
	| TextEmbodiment
	| FigureEmbodiment
	| VideoEmbodiment
	| PdfEmbodiment
	| BulletEmbodiment
	| AuthorsEmbodiment
	| AffiliationsEmbodiment;
//#endregion 🔖Embodiment

//#region 🔖Participant
/** @emoji 🧑 Entity that may appear across arrangements (title, authors, …). */
export interface Participant {
	readonly id: string;
	readonly embodiments: readonly Embodiment[];
}
//#endregion 🔖Participant

//#region 🔖Disposition
/** @emoji 📐 Normalized slide rectangle (0..1 fractions) for a {@link Disposition}. */
export interface DispositionPosition {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
}

/** @emoji 🎨 Optional style overrides on a {@link Disposition}. */
export interface DispositionStyle {
	readonly opacity?: number;
	readonly rotate?: number;
	readonly scale?: number;
}

/** @emoji 🧩 One crop of a figure source with its own slide placement (see {@link DispositionSplit}). */
export interface SplitTile {
	readonly key: string;
	readonly crop: DispositionPosition;
	readonly position: DispositionPosition;
	readonly emphasis?: ParticipantEmphasis;
	readonly style?: DispositionStyle;
}

/** @emoji ✂️ Splits one figure disposition into independently placed crop tiles for auto-animate. */
export interface DispositionSplit {
	readonly tiles: readonly SplitTile[];
	/** @emoji 👻 Tiles stay in the DOM for reveal.js matching but are not painted until a later arrangement reveals them. */
	readonly concealed?: boolean;
}

/** @emoji 📍 Concrete positioned, styled embodiment of a participant on one arrangement. */
export interface Disposition {
	readonly participantId: string;
	readonly embodimentId?: string;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
	readonly split?: DispositionSplit;
}
//#endregion 🔖Disposition

//#region 🔖Split
/** @emoji 📐 Spec for {@link splitFigureGrid}: uniform rows×columns inside a slide frame. */
export interface SplitFigureGridSpec {
	readonly rows: number;
	readonly columns: number;
	readonly frame: DispositionPosition;
	readonly gap?: number;
	readonly emphasis?: ParticipantEmphasis;
	readonly keyPrefix?: string;
}

/** @emoji ✂️ Builds crop tiles that pack a figure grid into a normalized slide frame (gap=0 reconstructs the frame). */
export function splitFigureGrid(spec: SplitFigureGridSpec): SplitTile[] {
	const { rows, columns, frame, gap = 0, emphasis, keyPrefix = "tile" } = spec;
	if (rows < 1 || columns < 1) {
		throw new Error(`splitFigureGrid requires rows and columns >= 1 (got ${rows}×${columns}).`);
	}
	const cellWidth = (frame.width - gap * (columns - 1)) / columns;
	const cellHeight = (frame.height - gap * (rows - 1)) / rows;
	const tiles: SplitTile[] = [];
	for (let row = 0; row < rows; row += 1) {
		for (let column = 0; column < columns; column += 1) {
			tiles.push({
				key: `${keyPrefix}-r${row}-c${column}`,
				crop: {
					x: column / columns,
					y: row / rows,
					width: 1 / columns,
					height: 1 / rows,
				},
				position: {
					x: frame.x + column * (cellWidth + gap),
					y: frame.y + row * (cellHeight + gap),
					width: cellWidth,
					height: cellHeight,
				},
				...(emphasis ? { emphasis } : {}),
			});
		}
	}
	return tiles;
}
//#endregion 🔖Split

//#region 🔖Arrangement
/** @emoji 🖼 One slide: participants disposed with emphasis, position, and style. */
export interface Arrangement {
	readonly id: string;
	readonly dispositions: readonly Disposition[];
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
export interface ResolvedDisposition {
	readonly participant: Participant;
	readonly embodiment: Embodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly embodimentId?: string;
	readonly morphId: string;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
	readonly split?: DispositionSplit;
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

/** @emoji 🔍 Resolves all dispositions for one arrangement within a thought. */
export function resolveArrangement(thought: Thought, arrangementId: string): ResolvedDisposition[] {
	const arrangement = thought.arrangements.find((a) => a.id === arrangementId);
	if (!arrangement) {
		throw new Error(`Thought "${thought.id}" has no arrangement "${arrangementId}".`);
	}
	const byId = new Map(thought.participants.map((p) => [p.id, p]));
	return arrangement.dispositions.map((disposition) => {
		const participant = byId.get(disposition.participantId);
		if (!participant) {
			throw new Error(`Thought "${thought.id}" has no participant "${disposition.participantId}".`);
		}
		return {
			participant,
			embodiment: resolveEmbodiment(participant, disposition.embodimentId),
			emphasis: disposition.emphasis,
			embodimentId: disposition.embodimentId,
			morphId: morphId(participant.id),
			position: disposition.position,
			style: disposition.style,
			split: disposition.split,
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

	const muted = (participantId: string, embodimentId?: string): Disposition => ({
		participantId,
		...(embodimentId ? { embodimentId } : {}),
		emphasis: "muted",
	});

	const active = (participantId: string, embodimentId?: string): Disposition => ({
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
				dispositions: [active(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_FULL)],
			},
			{
				id: "description",
				dispositions: [
					muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
					active(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_FULL),
				],
			},
			{
				id: "goal",
				dispositions: [
					muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
					muted(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_SHORT),
					active(INTRO_PARTICIPANT_GOAL),
				],
			},
			{
				id: "authors",
				dispositions: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_PLAIN),
				],
			},
			{
				id: "affiliations-1",
				dispositions: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1),
					active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP1),
				],
			},
			{
				id: "affiliations-2",
				dispositions: [
					...introMutedAboveAuthors,
					active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2),
					active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP2),
				],
			},
			{
				id: "affiliations-3",
				dispositions: [
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

//#region 🔖Analogy
const ANALOGY_PARTICIPANT_LABEL = "label";
const ANALOGY_PARTICIPANT_VISUAL = "visual";
const ANALOGY_EMBODIMENT_LABEL_SOURCE = "source";
const ANALOGY_EMBODIMENT_LABEL_TARGET = "target";
const ANALOGY_EMBODIMENT_VISUAL_SOURCE = "source";
const ANALOGY_EMBODIMENT_VISUAL_TARGET = "target";

/** @emoji 🔀 Spec for a two-slide analogy (source concept morphs into target via shared participant ids). */
export interface AnalogySpec {
	readonly id?: string;
	readonly name?: string;
	readonly source: {
		readonly label: string;
		readonly figure?: string;
	};
	readonly target: {
		readonly label: string;
		readonly figure?: string;
	};
}

/** @emoji 🔀 Builds a morph thought: source arrangement then mapping arrangement (reveal.js auto-animate). */
export function analogy(spec: AnalogySpec): Presentation {
	const labelParticipant: Participant = {
		id: ANALOGY_PARTICIPANT_LABEL,
		embodiments: [
			{
				kind: "text",
				id: ANALOGY_EMBODIMENT_LABEL_SOURCE,
				lines: [spec.source.label],
				level: "heading",
				morphRoot: "heading-line",
			},
			{
				kind: "text",
				id: ANALOGY_EMBODIMENT_LABEL_TARGET,
				lines: [spec.target.label],
				level: "heading",
				morphRoot: "heading-line",
			},
		],
	};

	const visualEmbodiments: Embodiment[] = [];
	if (spec.source.figure) {
		visualEmbodiments.push({
			kind: "figure",
			id: ANALOGY_EMBODIMENT_VISUAL_SOURCE,
			src: spec.source.figure,
			alt: spec.source.label,
		});
	}
	if (spec.target.figure) {
		visualEmbodiments.push({
			kind: "figure",
			id: ANALOGY_EMBODIMENT_VISUAL_TARGET,
			src: spec.target.figure,
			alt: spec.target.label,
		});
	}

	const participants: Participant[] = [labelParticipant];
	if (visualEmbodiments.length > 0) {
		participants.push({ id: ANALOGY_PARTICIPANT_VISUAL, embodiments: visualEmbodiments });
	}

	const sourceDispositions: Disposition[] = [
		{
			participantId: ANALOGY_PARTICIPANT_LABEL,
			embodimentId: ANALOGY_EMBODIMENT_LABEL_SOURCE,
			emphasis: "active",
		},
	];
	const mappingDispositions: Disposition[] = [
		{
			participantId: ANALOGY_PARTICIPANT_LABEL,
			embodimentId: ANALOGY_EMBODIMENT_LABEL_TARGET,
			emphasis: "active",
		},
	];
	if (spec.source.figure) {
		sourceDispositions.push({
			participantId: ANALOGY_PARTICIPANT_VISUAL,
			embodimentId: ANALOGY_EMBODIMENT_VISUAL_SOURCE,
			emphasis: "active",
			position: { x: 0.1, y: 0.35, width: 0.8, height: 0.5 },
		});
	}
	if (spec.target.figure) {
		mappingDispositions.push({
			participantId: ANALOGY_PARTICIPANT_VISUAL,
			embodimentId: ANALOGY_EMBODIMENT_VISUAL_TARGET,
			emphasis: "active",
			position: { x: 0.1, y: 0.35, width: 0.8, height: 0.5 },
		});
	}

	const thought: Thought = {
		id: "analogy",
		participants,
		transition: { kind: "morph" },
		arrangements: [
			{ id: "source", dispositions: sourceDispositions },
			{ id: "mapping", dispositions: mappingDispositions },
		],
	};

	return {
		id: spec.id ?? "analogy",
		name: spec.name ?? `${spec.source.label} → ${spec.target.label}`,
		sequences: [{ id: "main", thoughts: [thought] }],
	};
}
//#endregion 🔖Analogy

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

	describe("tileMorphId", () => {
		it("scopes tile keys under the participant id", () => {
			expect(tileMorphId("catalogue", "tile-r0-c0")).toBe("catalogue--tile--tile-r0-c0");
		});
	});

	describe("splitFigureGrid", () => {
		const frame = { x: 0.1, y: 0.2, width: 0.8, height: 0.6 };

		it("builds rows×columns tiles with normalized crops", () => {
			const tiles = splitFigureGrid({ rows: 3, columns: 5, frame });
			expect(tiles).toHaveLength(15);
			expect(tiles[0]).toMatchObject({
				key: "tile-r0-c0",
				crop: { x: 0, y: 0, width: 0.2, height: 1 / 3 },
			});
			expect(tiles[14]).toMatchObject({
				key: "tile-r2-c4",
				crop: { x: 0.8, y: 2 / 3, width: 0.2, height: 1 / 3 },
			});
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

	describe("resolveArrangement split", () => {
		it("passes split through on resolved dispositions", () => {
			const split = { tiles: splitFigureGrid({ rows: 1, columns: 1, frame: { x: 0, y: 0, width: 1, height: 1 } }) };
			const thought: Thought = {
				id: "split",
				participants: [
					{
						id: "fig",
						embodiments: [{ kind: "figure", src: "/a.png" }],
					},
				],
				arrangements: [
					{
						id: "slide",
						dispositions: [{ participantId: "fig", emphasis: "active", split }],
					},
				],
			};
			const resolved = resolveArrangement(thought, "slide");
			expect(resolved[0]?.split?.tiles).toHaveLength(1);
		});

		it("passes concealed on split", () => {
			const split = {
				tiles: splitFigureGrid({ rows: 1, columns: 1, frame: { x: 0, y: 0, width: 1, height: 1 } }),
				concealed: true,
			};
			const thought: Thought = {
				id: "split",
				participants: [{ id: "fig", embodiments: [{ kind: "figure", src: "/a.png" }] }],
				arrangements: [{ id: "slide", dispositions: [{ participantId: "fig", emphasis: "active", split }] }],
			};
			expect(resolveArrangement(thought, "slide")[0]?.split?.concealed).toBe(true);
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
			const thought = sampleIntro.sequences[0]!.thoughts[0]!;
			const resolved = resolveArrangement(thought, "goal");
			expect(resolved.map((r) => r.morphId)).toEqual(["title", "description", "goal"]);
		});
	});

	describe("analogy", () => {
		const sampleAnalogy = analogy({
			source: { label: "Reuse", figure: "/reuse.png" },
			target: { label: "Remanufacture", figure: "/remanufacture.png" },
		});

		it("builds two morph arrangements", () => {
			expect(countArrangements(sampleAnalogy)).toBe(2);
			const thought = sampleAnalogy.sequences[0]!.thoughts[0]!;
			expect(thought.arrangements.map((a) => a.id)).toEqual(["source", "mapping"]);
		});

		it("resolves positioned visual dispositions", () => {
			const thought = sampleAnalogy.sequences[0]!.thoughts[0]!;
			const mapping = resolveArrangement(thought, "mapping");
			const visual = mapping.find((r) => r.participant.id === ANALOGY_PARTICIPANT_VISUAL);
			expect(visual?.position).toEqual({ x: 0.1, y: 0.35, width: 0.8, height: 0.5 });
			expect(visual?.embodiment.kind).toBe("figure");
		});
	});

	describe("video and pdf embodiments", () => {
		it("resolves video and pdf kinds", () => {
			const thought: Thought = {
				id: "media",
				participants: [
					{
						id: "clip",
						embodiments: [{ kind: "video", src: "/demo.mp4", muted: true }],
					},
					{
						id: "doc",
						embodiments: [{ kind: "pdf", src: "/paper.pdf", page: 2 }],
					},
				],
				arrangements: [
					{
						id: "slide",
						dispositions: [
							{ participantId: "clip", emphasis: "active" },
							{ participantId: "doc", emphasis: "active", position: { x: 0.2, y: 0.2, width: 0.6, height: 0.6 } },
						],
					},
				],
			};
			const resolved = resolveArrangement(thought, "slide");
			expect(resolved[0]?.embodiment.kind).toBe("video");
			expect(resolved[1]?.embodiment.kind).toBe("pdf");
			if (resolved[1]?.embodiment.kind === "pdf") {
				expect(resolved[1].embodiment.page).toBe(2);
			}
			expect(resolved[1]?.position?.width).toBe(0.6);
		});
	});
}
//#endregion 🧪Tests
