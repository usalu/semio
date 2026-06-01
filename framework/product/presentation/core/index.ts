// #region 🧱Header
/** 🧱 `@framework/presentation/core` — Render-independent declarative presentations: {@link Presentation}, {@link Chapter}, {@link Sequence}, {@link Thought}, {@link Slide}, {@link SlideFile}, {@link Participant}, {@link Embodiment}, {@link Disposition}, {@link Arrangement}, {@link Transition}, {@link expandThoughtSlides}, {@link loadPresentationFromSlideGlob}, {@link intro}, and {@link analogy}. */
// #endregion 🧱Header

//#region 🔖Emphasis
/** @emoji 🎚 Visual emphasis for a participant on one slide (maps to opacity layering in renderers). */
export type ParticipantEmphasis = "active" | "muted";
//#endregion 🔖Emphasis

//#region 🔖Transition
/** @emoji ↔️ Transition from one slide to the next within a thought. */
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
	readonly id: string;
	readonly lines: readonly string[];
	readonly level: "title" | "heading" | "subheading" | "body";
	readonly fit?: boolean;
	readonly morphRoot?: TextMorphRoot;
}

/** @emoji 🖼 Raster or vector figure on a slide; optional {@link FigureEmbodiment.crop} for a normalized source region. */
export interface FigureEmbodiment {
	readonly kind: "figure";
	readonly id: string;
	readonly src: string;
	readonly alt?: string;
	readonly crop?: DispositionPosition;
}

/** @emoji 🎬 Video clip on a slide. */
export interface VideoEmbodiment {
	readonly kind: "video";
	readonly id: string;
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
	readonly id: string;
	readonly src: string;
	readonly page?: number;
	readonly alt?: string;
}

/** @emoji • Bulleted list body. */
export interface BulletEmbodiment {
	readonly kind: "bullet";
	readonly id: string;
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
	readonly id: string;
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

/** @emoji 🔀 Maps affiliation marks whose line label changes between steps to the prior label for reveal.js morph pairing. */
export function affiliationEmbodimentMorphLabels(
	previousStep: readonly AffiliationEntry[],
	currentStep: readonly AffiliationEntry[],
): Readonly<Record<string, string>> {
	const previousByMark = new Map(previousStep.map((entry) => [entry.mark, entry]));
	const labels: Record<string, string> = {};
	for (const entry of currentStep) {
		const previous = previousByMark.get(entry.mark);
		if (!previous) {
			continue;
		}
		const fromLabel = affiliationLineName(previous);
		const toLabel = affiliationLineName(entry);
		if (fromLabel !== toLabel) {
			labels[entry.mark] = fromLabel;
		}
	}
	return labels;
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
	readonly id: string;
	readonly entries: readonly AffiliationEntry[];
	/** @emoji 🔀 Prior line labels keyed by mark when the visible label changes after position morph (e.g. full name → shortName). */
	readonly morphLineLabels?: Readonly<Record<string, string>>;
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
/** @emoji 🧑 Entity that may appear on one or many slides (identity only; embodiments live in a scope registry). */
export interface Participant {
	readonly id: string;
	readonly name?: string;
}
//#endregion 🔖Participant

//#region 🔖Scope
/** @emoji 🗂 Optional participant and embodiment registries on a presentation artifact and its children. */
export interface ArtifactScope {
	readonly participants?: readonly Participant[];
	readonly embodiments?: readonly Embodiment[];
}

/** @emoji 🔍 Merged participant and embodiment maps for resolving dispositions. */
export interface ResolutionScope {
	readonly participants: ReadonlyMap<string, Participant>;
	readonly embodiments: ReadonlyMap<string, Embodiment>;
}

/** @emoji 🔍 Merges registries from outer to inner scope layers (later layers override earlier ids). */
export function buildResolutionScope(ancestors: readonly ArtifactScope[]): ResolutionScope {
	const participants = new Map<string, Participant>();
	const embodiments = new Map<string, Embodiment>();
	for (const layer of ancestors) {
		for (const participant of layer.participants ?? []) {
			participants.set(participant.id, participant);
		}
		for (const embodiment of layer.embodiments ?? []) {
			embodiments.set(embodiment.id, embodiment);
		}
	}
	return { participants, embodiments };
}

/** @emoji 🔍 Builds the resolution scope for one arrangement inside a deck hierarchy. */
export function resolutionScopeForArrangement(
	presentation: Presentation,
	chapter: Chapter,
	sequence: Sequence,
	thought: Thought,
	arrangement: Arrangement,
): ResolutionScope {
	return buildResolutionScope([presentation, chapter, sequence, thought, arrangement]);
}
//#endregion 🔖Scope

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

/** @emoji 👻 One source participant that morphs independently into a target participant slot. */
export interface MorphFromSlot {
	readonly participantId: string;
	readonly position: DispositionPosition;
	readonly embodimentId: string;
	/** @emoji 🎯 Line index when the target uses a multi-line text morph root (`participantId--index`). */
	readonly targetLineIndex?: number;
}

/** @emoji 📍 Concrete positioned, styled embodiment of a participant on one arrangement. */
export interface Disposition {
	readonly participantId: string;
	readonly embodimentId: string;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
	/** @emoji 👻 Source participants that each morph into this disposition (expanded as ghost dispositions for reveal.js). */
	readonly morphFrom?: readonly MorphFromSlot[];
	/** @emoji 👻 True when auto-expanded from {@link Disposition.morphFrom} (not authored directly). */
	readonly morphGhost?: boolean;
	/** @emoji 🎯 Overrides reveal.js `data-id` for this disposition (ghost morph into a target line). */
	readonly morphTargetId?: string;
}
//#endregion 🔖Disposition

//#region 🔖Tile
/** @emoji 🧩 Spec for {@link tile}: one cropped figure embodiment from a source image. */
export interface TileSpec {
	readonly id: string;
	readonly source: string;
	readonly crop: DispositionPosition;
	readonly alt?: string;
}

/** @emoji 🧩 Produces one cropped {@link FigureEmbodiment} from a source figure. */
export function tile(spec: TileSpec): FigureEmbodiment {
	return {
		kind: "figure",
		id: spec.id,
		src: spec.source,
		alt: spec.alt,
		crop: spec.crop,
	};
}
//#endregion 🔖Tile

//#region 🔖Split
/** @emoji 📐 One grid cell placement and source crop (internal to {@link splitFigureGrid}). */
export interface SplitGridCell {
	readonly key: string;
	readonly crop: DispositionPosition;
	readonly position: DispositionPosition;
	readonly emphasis?: ParticipantEmphasis;
	readonly style?: DispositionStyle;
}

/** @emoji 📐 Spec for {@link splitFigureGrid}: uniform rows×columns inside a slide frame. */
export interface SplitFigureGridSpec {
	readonly rows: number;
	readonly columns: number;
	readonly frame: DispositionPosition;
	readonly gap?: number;
	readonly emphasis?: ParticipantEmphasis;
	readonly keyPrefix?: string;
}

/** @emoji ✂️ Builds grid cells that pack a figure into a normalized slide frame (gap=0 reconstructs the frame). */
export function splitFigureGrid(spec: SplitFigureGridSpec): SplitGridCell[] {
	const { rows, columns, frame, gap = 0, emphasis, keyPrefix = "tile" } = spec;
	if (rows < 1 || columns < 1) {
		throw new Error(`splitFigureGrid requires rows and columns >= 1 (got ${rows}×${columns}).`);
	}
	const cellWidth = (frame.width - gap * (columns - 1)) / columns;
	const cellHeight = (frame.height - gap * (rows - 1)) / rows;
	const cells: SplitGridCell[] = [];
	for (let row = 0; row < rows; row += 1) {
		for (let column = 0; column < columns; column += 1) {
			cells.push({
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
	return cells;
}

/** @emoji 📐 Union of normalized source-image crops. */
export function unionSourceCrops(crops: readonly DispositionPosition[]): DispositionPosition {
	if (crops.length === 0) {
		throw new Error("unionSourceCrops: no crops.");
	}
	let minX = 1;
	let minY = 1;
	let maxX = 0;
	let maxY = 0;
	for (const crop of crops) {
		minX = Math.min(minX, crop.x);
		minY = Math.min(minY, crop.y);
		maxX = Math.max(maxX, crop.x + crop.width);
		maxY = Math.max(maxY, crop.y + crop.height);
	}
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

/** @emoji ✂️ Spec for {@link split}: grid of tile participants, embodiments, and dispositions. */
export interface SplitSpec {
	readonly source: string;
	readonly rows: number;
	readonly columns: number;
	readonly frame: DispositionPosition;
	readonly gap?: number;
	readonly emphasis?: ParticipantEmphasis;
	readonly keyPrefix?: string;
	readonly alt?: string;
	readonly embodimentIdSuffix?: string;
}

/** @emoji ✂️ Artifacts produced by the split template (one participant and disposition per grid cell). */
export interface SplitArtifacts {
	readonly participants: readonly Participant[];
	readonly embodiments: readonly FigureEmbodiment[];
	readonly dispositions: readonly Disposition[];
}

/** @emoji ✂️ Produces a grid of tile figure embodiments with one disposition per cell. */
export function split(spec: SplitSpec): SplitArtifacts {
	const suffix = spec.embodimentIdSuffix ?? "figure";
	const cells = splitFigureGrid({
		rows: spec.rows,
		columns: spec.columns,
		frame: spec.frame,
		gap: spec.gap,
		emphasis: spec.emphasis,
		keyPrefix: spec.keyPrefix,
	});
	const participants: Participant[] = [];
	const embodiments: FigureEmbodiment[] = [];
	const dispositions: Disposition[] = [];
	for (const cell of cells) {
		const participantId = cell.key;
		const embodimentId = `${participantId}-${suffix}`;
		participants.push({ id: participantId });
		embodiments.push(
			tile({
				id: embodimentId,
				source: spec.source,
				crop: cell.crop,
				alt: spec.alt,
			}),
		);
		dispositions.push({
			participantId,
			embodimentId,
			emphasis: cell.emphasis ?? spec.emphasis ?? "active",
			position: cell.position,
			...(cell.style ? { style: cell.style } : {}),
		});
	}
	return { participants, embodiments, dispositions };
}

/** @emoji 📐 Replaces slide positions on split dispositions matched by participant id. */
export function remapSplitDispositions(
	dispositions: readonly Disposition[],
	positionsByParticipantId: Readonly<Record<string, DispositionPosition>>,
): Disposition[] {
	return dispositions.map((disposition) => {
		const position = positionsByParticipantId[disposition.participantId];
		return position ? { ...disposition, position } : disposition;
	});
}
//#endregion 🔖Split

//#region 🔖Arrangement
/** @emoji 🖼 One slide: participants disposed with emphasis, position, and style. */
export interface Arrangement extends ArtifactScope {
	readonly id: string;
	/** @emoji 🔖 URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly dispositions: readonly Disposition[];
	/** @emoji ⏳ Arrangement ids that settle ephemeral layout before auto-animating to them. */
	readonly settleBeforeMorphTo?: readonly string[];
}
//#endregion 🔖Arrangement

//#region 🔖Slide
/** @emoji 🖼 One slide: an arrangement with an optional transition to the next slide. */
export interface Slide {
	readonly arrangement: Arrangement;
	readonly transition?: Transition;
}
//#endregion 🔖Slide

//#region 🔖Thought
/** @emoji 💭 Idea developed across one or more slides with scoped participants and embodiments. */
export interface Thought extends ArtifactScope {
	readonly id: string;
	/** @emoji 🔖 URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly slides: readonly Slide[];
}
//#endregion 🔖Thought

//#region 🔖Sequence
/** @emoji 📚 Reveal.js horizontal stack: ordered thoughts rendered as one vertical slide column. */
export interface Sequence extends ArtifactScope {
	readonly id: string;
	/** @emoji 🔖 URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly thoughts: readonly Thought[];
}
//#endregion 🔖Sequence

//#region 🔖Chapter
/** @emoji 📖 Groups related sequences in a deck (bookmarks and authoring; sequences stay top-level in reveal.js). */
export interface Chapter extends ArtifactScope {
	readonly id: string;
	/** @emoji 🔖 URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly sequences: readonly Sequence[];
}
//#endregion 🔖Chapter

//#region 🔖Presentation
/** @emoji 🌐 Main language of a deck; drives localized URL bookmark query keys. */
export type PresentationLanguageKind = "de" | "en";

/** @emoji 📽 Root deck: ordered chapters of sequences of thoughts. */
export interface Presentation extends ArtifactScope {
	readonly id: string;
	readonly name: string;
	readonly chapters: readonly Chapter[];
	readonly language?: PresentationLanguageKind;
	readonly width?: number;
	readonly height?: number;
}
//#endregion 🔖Presentation

//#region 🔖Traverse
/** @emoji 📚 Flattens all sequences in chapter order (one reveal.js horizontal stack per sequence). */
export function presentationSequences(presentation: Presentation): readonly Sequence[] {
	return presentation.chapters.flatMap((chapter) => chapter.sequences);
}
//#endregion 🔖Traverse

//#region 🔖Resolved
/** @emoji ✅ One participant embodiment resolved for rendering a single arrangement. */
export interface ResolvedDisposition {
	readonly participant: Participant;
	readonly embodiment: Embodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly embodimentId: string;
	readonly morphId: string;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
}

/** @emoji 📐 Union of normalized placement rectangles. */
export function unionDispositionPositions(positions: readonly DispositionPosition[]): DispositionPosition {
	if (positions.length === 0) {
		throw new Error("unionDispositionPositions: no positions.");
	}
	let minX = 1;
	let minY = 1;
	let maxX = 0;
	let maxY = 0;
	for (const position of positions) {
		minX = Math.min(minX, position.x);
		minY = Math.min(minY, position.y);
		maxX = Math.max(maxX, position.x + position.width);
		maxY = Math.max(maxY, position.y + position.height);
	}
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

/** @emoji ⊕ Translation that centers {@link bounds} in the unit slide. */
export function centerDispositionBoundsOffset(bounds: DispositionPosition): DispositionPosition {
	return {
		x: (1 - bounds.width) / 2 - bounds.x,
		y: (1 - bounds.height) / 2 - bounds.y,
		width: 0,
		height: 0,
	};
}

/** @emoji ↔️ Moves a placement by a normalized offset. */
export function shiftDispositionPosition(
	position: DispositionPosition,
	offset: DispositionPosition,
): DispositionPosition {
	return {
		x: position.x + offset.x,
		y: position.y + offset.y,
		width: position.width,
		height: position.height,
	};
}

const CENTER_ARRANGEMENT_EPSILON = 1e-6;

function isDispositionVisibleForLayout(style: DispositionStyle | undefined): boolean {
	return style?.opacity !== 0;
}

/** @emoji 📍 Slide positions for layout centering (visible tiles and frames; omits opacity-0 dispositions). */
export function visibleArrangementPositions(resolved: readonly ResolvedDisposition[]): DispositionPosition[] {
	const positions: DispositionPosition[] = [];
	for (const disposition of resolved) {
		if (!isDispositionVisibleForLayout(disposition.style)) {
			continue;
		}
		if (disposition.position) {
			positions.push(disposition.position);
		}
	}
	return positions;
}

function shiftResolvedDisposition(
	disposition: ResolvedDisposition,
	offset: DispositionPosition,
): ResolvedDisposition {
	if (!disposition.position) {
		return disposition;
	}
	return {
		...disposition,
		position: shiftDispositionPosition(disposition.position, offset),
	};
}

/** @emoji ⊕ Centers visible placements in the unit slide frame. */
export function centerResolvedArrangement(resolved: readonly ResolvedDisposition[]): ResolvedDisposition[] {
	const positions = visibleArrangementPositions(resolved);
	if (positions.length === 0) {
		return [...resolved];
	}
	const bounds = unionDispositionPositions(positions);
	const offset = centerDispositionBoundsOffset(bounds);
	if (
		Math.abs(offset.x) < CENTER_ARRANGEMENT_EPSILON &&
		Math.abs(offset.y) < CENTER_ARRANGEMENT_EPSILON
	) {
		return [...resolved];
	}
	return resolved.map((disposition) => shiftResolvedDisposition(disposition, offset));
}
//#endregion 🔖Resolved

//#region 🔖Expand
/** @emoji 🎞 One renderable slide after morph expansion for reveal.js. */
export interface RenderSlide {
	readonly id: string;
	/** @emoji 🔖 URL bookmark label only; falls back to {@link id}. Never rendered on the slide. */
	readonly name?: string;
	readonly arrangement: Arrangement;
	/** @emoji ↔️ reveal.js `data-auto-animate-id` shared by consecutive morph-linked slides in one run. */
	readonly autoAnimateId?: string;
}

function transitionUsesMorph(transition: Transition | undefined): boolean {
	return (transition?.kind ?? "morph") === "morph";
}

function primaryDispositionByParticipant(arrangement: Arrangement): Map<string, Disposition> {
	const map = new Map<string, Disposition>();
	for (const disposition of arrangement.dispositions) {
		if (!map.has(disposition.participantId)) {
			map.set(disposition.participantId, disposition);
		}
	}
	return map;
}

function slideParticipantIds(slide: Slide): ReadonlySet<string> {
	return new Set(slide.arrangement.dispositions.map((disposition) => disposition.participantId));
}

function slideMorphParticipantIds(slide: Slide): ReadonlySet<string> {
	const ids = slideParticipantIds(slide);
	for (const disposition of slide.arrangement.dispositions) {
		for (const slot of disposition.morphFrom ?? []) {
			ids.add(slot.participantId);
		}
	}
	return ids;
}

/** @emoji 👻 Expands {@link Disposition.morphFrom} into per-source ghost dispositions for reveal.js pairing. */
export function expandArrangementMorphFrom(
	sourceSlide: Slide,
	arrangement: Arrangement,
	options?: { readonly morphLineTargets?: boolean },
): Arrangement {
	const sourceByParticipant = primaryDispositionByParticipant(sourceSlide.arrangement);
	const morphLineTargets = options?.morphLineTargets ?? true;
	const ghosts: Disposition[] = [];
	for (const disposition of arrangement.dispositions) {
		for (const slot of disposition.morphFrom ?? []) {
			const sourceDisposition = sourceByParticipant.get(slot.participantId);
			const morphTargetId =
				morphLineTargets && slot.targetLineIndex !== undefined
					? `${disposition.participantId}--${slot.targetLineIndex}`
					: undefined;
			const embodimentId = slot.embodimentId ?? sourceDisposition?.embodimentId;
			if (!embodimentId) {
				throw new Error(
					`morphFrom slot for "${slot.participantId}" needs embodimentId (or a source disposition with embodimentId).`,
				);
			}
			ghosts.push({
				participantId: slot.participantId,
				embodimentId,
				emphasis: sourceDisposition?.emphasis ?? "active",
				position: slot.position,
				style: { opacity: 0 },
				morphGhost: true,
				morphTargetId,
			});
		}
	}
	if (ghosts.length === 0) {
		return arrangement;
	}
	return { ...arrangement, dispositions: [...arrangement.dispositions, ...ghosts] };
}

/** @emoji 🔗 True when two consecutive slides share at least one participant for reveal.js auto-animate pairing. */
export function slidesShareMorphParticipants(source: Slide, target: Slide): boolean {
	const sourceIds = slideMorphParticipantIds(source);
	for (const participantId of slideMorphParticipantIds(target)) {
		if (sourceIds.has(participantId)) {
			return true;
		}
	}
	return false;
}

/** @emoji 🎞 Expands {@link Thought.slides} with morph ghosts and morph-run auto-animate ids. */
export function expandThoughtSlides(thought: Thought): readonly RenderSlide[] {
	const slides = thought.slides;
	if (slides.length === 0) {
		return [];
	}
	const expanded: RenderSlide[] = [];
	let runIndex = 0;
	let index = 0;
	while (index < slides.length) {
		let runEnd = index;
		while (runEnd < slides.length - 1 && transitionUsesMorph(slides[runEnd]?.transition)) {
			const current = slides[runEnd]!;
			const next = slides[runEnd + 1]!;
			if (!slidesShareMorphParticipants(current, next)) {
				break;
			}
			runEnd += 1;
		}
		const autoAnimateId = runEnd > index ? `${thought.id}--m${runIndex}` : undefined;
		runIndex += 1;
		for (let slideIndex = index; slideIndex <= runEnd; slideIndex += 1) {
			const slide = slides[slideIndex]!;
			const sourceSlide = slideIndex > index ? slides[slideIndex - 1]! : undefined;
			const arrangement =
				sourceSlide === undefined
					? slide.arrangement
					: expandArrangementMorphFrom(sourceSlide, slide.arrangement, { morphLineTargets: true });
			expanded.push({
				id: slide.arrangement.id,
				name: slide.arrangement.name,
				arrangement,
				autoAnimateId,
			});
		}
		index = runEnd + 1;
	}
	return expanded;
}
//#endregion 🔖Expand

//#region 🔖Resolve
/** @emoji 🔍 Looks up one embodiment in a {@link ResolutionScope}. */
export function resolveEmbodiment(scope: ResolutionScope, embodimentId: string): Embodiment {
	const match = scope.embodiments.get(embodimentId);
	if (!match) {
		throw new Error(`Unknown embodiment "${embodimentId}".`);
	}
	return match;
}

/** @emoji 🔍 Resolves all dispositions for one arrangement against a scope. */
export function resolveArrangement(scope: ResolutionScope, arrangement: Arrangement): ResolvedDisposition[] {
	return arrangement.dispositions.flatMap((disposition) => {
		const participant = scope.participants.get(disposition.participantId);
		if (!participant) {
			throw new Error(`Arrangement "${arrangement.id}" references unknown participant "${disposition.participantId}".`);
		}
		return [
			{
				participant,
				embodiment: resolveEmbodiment(scope, disposition.embodimentId),
				emphasis: disposition.emphasis,
				embodimentId: disposition.embodimentId,
				morphId: disposition.morphTargetId ?? morphId(participant.id),
				position: disposition.position,
				style: disposition.style,
			},
		];
	});
}

/** @emoji 🔢 Counts render slides across all chapters and sequences. */
export function countArrangements(presentation: Presentation): number {
	return presentationSequences(presentation).reduce(
		(sum, sequence) =>
			sum + sequence.thoughts.reduce((thoughtSum, thought) => thoughtSum + expandThoughtSlides(thought).length, 0),
		0,
	);
}

/** @emoji 🔖 English bookmark query keys after the reveal.js hash path. */
export const PRESENTATION_CHAPTER_QUERY_PARAM = "chapter";
export const PRESENTATION_SEQUENCE_QUERY_PARAM = "sequence";
export const PRESENTATION_THOUGHT_QUERY_PARAM = "thought";
export const PRESENTATION_SLIDE_QUERY_PARAM = "slide";

/** @emoji 🔖 Localized bookmark query keys for chapter, sequence, thought, and slide. */
export interface PresentationSlideBookmarkParamKeys {
	readonly chapter: string;
	readonly sequence: string;
	readonly thought: string;
	readonly slide: string;
}

/** @emoji 🌐 Resolves a deck's main language (`en` when unset). */
export function presentationLanguage(presentation: Presentation): PresentationLanguageKind {
	return presentation.language ?? "en";
}

/** @emoji 🌐 Bookmark query param names for a presentation language (`sequenz`, `gedanke`, `folie` in German). */
export function presentationSlideBookmarkParamKeys(
	language: PresentationLanguageKind = "en",
): PresentationSlideBookmarkParamKeys {
	if (language === "de") {
		return {
			chapter: "kapitel",
			sequence: "sequenz",
			thought: "gedanke",
			slide: "folie",
		};
	}
	return {
		chapter: PRESENTATION_CHAPTER_QUERY_PARAM,
		sequence: PRESENTATION_SEQUENCE_QUERY_PARAM,
		thought: PRESENTATION_THOUGHT_QUERY_PARAM,
		slide: PRESENTATION_SLIDE_QUERY_PARAM,
	};
}

/** @emoji 🔗 Bookmark ids carried in the URL hash query; navigation uses only the hash path. */
export interface PresentationSlideBookmark {
	readonly chapter: string;
	readonly sequence: string;
	readonly thought: string;
	readonly slide: string;
}

/** @emoji 🔖 Resolves the URL bookmark label for a sequence, thought, or arrangement. */
export function presentationEntityBookmarkName(entity: { readonly id: string; readonly name?: string }): string {
	return entity.name ?? entity.id;
}

/** @emoji 🔗 One reveal.js slide location with localized bookmark labels for the URL. */
export interface PresentationSlideRef extends PresentationSlideBookmark {
	readonly h: number;
	readonly v: number;
}

/** @emoji 🔗 Lists every slide in reveal.js h/v order (one horizontal stack per sequence, chapters flattened). */
export function collectPresentationSlides(presentation: Presentation): readonly PresentationSlideRef[] {
	const slides: PresentationSlideRef[] = [];
	let h = 0;
	for (const chapter of presentation.chapters) {
		for (const sequence of chapter.sequences) {
			let v = 0;
			for (const thought of sequence.thoughts) {
				for (const renderSlide of expandThoughtSlides(thought)) {
					slides.push({
						h,
						v,
						chapter: presentationEntityBookmarkName(chapter),
						sequence: presentationEntityBookmarkName(sequence),
						thought: presentationEntityBookmarkName(thought),
						slide: presentationEntityBookmarkName(renderSlide),
					});
					v += 1;
				}
			}
			h += 1;
		}
	}
	return slides;
}

/** @emoji 🔗 Resolves the slide at reveal.js indices within a deck. */
export function presentationSlideAt(
	presentation: Presentation,
	indices: { readonly h: number; readonly v: number },
): PresentationSlideRef | undefined {
	return collectPresentationSlides(presentation).find(
		(slide) => slide.h === indices.h && slide.v === indices.v,
	);
}

/** @emoji 🔗 Formats reveal.js hash path (`/` = first slide, `/2/1` = h=2 v=1). */
export function formatPresentationSlideHash(indices: { readonly h: number; readonly v: number }): string {
	if (indices.h <= 0 && indices.v <= 0) {
		return "/";
	}
	let hash = `/${indices.h}`;
	if (indices.v > 0) {
		hash += `/${indices.v}`;
	}
	return hash;
}

/** @emoji 🔗 Parses reveal.js slide hash into zero-based h/v indices; ignores trailing bookmark query params. */
export function parsePresentationSlideHash(hash: string): { readonly h: number; readonly v: number } | null {
	const pathPart = hash.replace(/^#/, "").trim().split("?")[0]?.replace(/^\/?/, "").trim() ?? "";
	if (!pathPart) {
		return { h: 0, v: 0 };
	}
	const bits = pathPart.split("/");
	const h = Number.parseInt(bits[0] ?? "0", 10);
	const v = Number.parseInt(bits[1] ?? "0", 10);
	if (!Number.isFinite(h) || !Number.isFinite(v) || Number.isNaN(h) || Number.isNaN(v)) {
		return null;
	}
	return { h, v };
}

/** @emoji 🔗 Formats reveal.js hash with localized sequence, thought, and slide bookmark params after the path. */
export function formatPresentationUrlHash(
	indices: { readonly h: number; readonly v: number },
	bookmark: PresentationSlideBookmark,
	language: PresentationLanguageKind = "en",
): string {
	const path = formatPresentationSlideHash(indices);
	const keys = presentationSlideBookmarkParamKeys(language);
	const params = new URLSearchParams([
		[keys.chapter, bookmark.chapter],
		[keys.sequence, bookmark.sequence],
		[keys.thought, bookmark.thought],
		[keys.slide, bookmark.slide],
	]);
	const query = params.toString();
	if (path === "/") {
		return `#/?${query}`;
	}
	return `#${path}?${query}`;
}
//#endregion 🔖Resolve

//#region 🔖SlideFile
/** @emoji 📄 One slide module under `slide/<chapter>/<sequence>/<thought>/<slide>.ts`. */
export interface SlideFile extends ArtifactScope {
	readonly order: number;
	readonly arrangement: Arrangement;
	readonly transition?: Transition;
}

/** @emoji 📁 Parsed path segments for a slide module file. */
export interface ParsedSlideFilePath {
	readonly chapter: string;
	readonly sequence: string;
	readonly thought: string;
	readonly slide: string;
}

/** @emoji 📁 Slide module path plus its parsed segments. */
export interface SlideFileModule {
	readonly path: ParsedSlideFilePath;
	readonly file: SlideFile;
}

/** @emoji 📁 Parsed path segments for a thought template module (`slide/<chapter>/<sequence>/<thought>.ts`). */
export interface ParsedThoughtFilePath {
	readonly chapter: string;
	readonly sequence: string;
	readonly thought: string;
}

/** @emoji 📄 Thought module expanded into slides via a named template (e.g. intro). */
export type ThoughtFile = {
	readonly template: "intro";
	readonly spec: IntroSpec;
};

/** @emoji 📽 Deck metadata paired with slide modules discovered from a glob import. */
export interface PresentationMeta {
	readonly id: string;
	readonly name: string;
	readonly language?: PresentationLanguageKind;
	readonly width?: number;
	readonly height?: number;
}

/** @emoji 🔤 Stable id derived from a titleized presentation entity name. */
export function presentationNameToId(name: string): string {
	return name
		.normalize("NFD")
		.replace(/\p{M}/gu, "")
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "");
}

/** @emoji 📁 Builds the canonical slide module path for one arrangement bookmark name. */
export function presentationSlideFilePath(
	chapter: string,
	sequence: string,
	thought: string,
	slide: string,
): string {
	return `slide/${chapter}/${sequence}/${thought}/${slide}.ts`;
}

/** @emoji 📁 Builds the canonical thought template path for one Gedanke bookmark name. */
export function presentationThoughtFilePath(chapter: string, sequence: string, thought: string): string {
	return `slide/${chapter}/${sequence}/${thought}.ts`;
}

/** @emoji 📁 Parses `slide/<chapter>/<sequence>/<thought>/<slide>.ts` from an import path. */
export function parsePresentationSlideFilePath(path: string): ParsedSlideFilePath | null {
	const normalized = path.replace(/\\/g, "/");
	const match = normalized.match(/(?:^|\/)slide\/([^/]+)\/([^/]+)\/([^/]+)\/([^/]+)\.ts$/u);
	if (!match) {
		return null;
	}
	return {
		chapter: match[1]!,
		sequence: match[2]!,
		thought: match[3]!,
		slide: match[4]!,
	};
}

/** @emoji 📁 Parses `slide/<chapter>/<sequence>/<thought>.ts` from an import path. */
export function parsePresentationThoughtFilePath(path: string): ParsedThoughtFilePath | null {
	const normalized = path.replace(/\\/g, "/");
	const match = normalized.match(/(?:^|\/)slide\/([^/]+)\/([^/]+)\/([^/]+)\.ts$/u);
	if (!match) {
		return null;
	}
	return {
		chapter: match[1]!,
		sequence: match[2]!,
		thought: match[3]!,
	};
}

function mergeArtifactScopeFromSlideFiles(modules: readonly SlideFile[]): ArtifactScope {
	const participants = new Map<string, Participant>();
	const embodiments = new Map<string, Embodiment>();
	for (const module of modules) {
		for (const participant of module.participants ?? []) {
			participants.set(participant.id, participant);
		}
		for (const embodiment of module.embodiments ?? []) {
			embodiments.set(embodiment.id, embodiment);
		}
	}
	return {
		participants: [...participants.values()],
		embodiments: [...embodiments.values()],
	};
}

/** @emoji 🧩 Assembles one thought from slide modules sharing the same folder path. */
export function assembleThoughtFromSlideFiles(thoughtName: string, modules: readonly SlideFileModule[]): Thought {
	const sorted = [...modules].sort((left, right) => left.file.order - right.file.order);
	const slideFiles = sorted.map((module) => module.file);
	const scope = mergeArtifactScopeFromSlideFiles(slideFiles);
	return {
		id: presentationNameToId(thoughtName),
		name: thoughtName,
		participants: scope.participants,
		embodiments: scope.embodiments,
		slides: sorted.map((module) => ({
			arrangement: {
				...module.file.arrangement,
				name: module.file.arrangement.name ?? module.path.slide,
			},
			transition: module.file.transition,
		})),
	};
}

/** @emoji 📚 Assembles one chapter from nested slide modules. */
export function assembleChapterFromSlideFiles(
	chapterName: string,
	thoughtModules: ReadonlyMap<string, readonly SlideFileModule[]>,
): Chapter {
	const sequences = new Map<string, Map<string, SlideFileModule[]>>();
	for (const [key, modules] of thoughtModules) {
		const [sequenceName, thoughtName] = key.split("\0");
		if (!sequenceName || !thoughtName) {
			throw new Error(`Invalid thought key "${key}" while assembling chapter "${chapterName}".`);
		}
		const byThought = sequences.get(sequenceName) ?? new Map<string, SlideFileModule[]>();
		byThought.set(thoughtName, [...modules]);
		sequences.set(sequenceName, byThought);
	}
	return {
		id: presentationNameToId(chapterName),
		name: chapterName,
		sequences: [...sequences.entries()]
			.sort(([left], [right]) => left.localeCompare(right, "de"))
			.map(([sequenceName, byThought]) => ({
				id: presentationNameToId(sequenceName),
				name: sequenceName,
				thoughts: [...byThought.entries()]
					.sort(([left], [right]) => left.localeCompare(right, "de"))
					.map(([thoughtName, modules]) => assembleThoughtFromSlideFiles(thoughtName, modules)),
			})),
	};
}

function isThoughtFile(module: SlideFile | ThoughtFile): module is ThoughtFile {
	return "template" in module;
}

/** @emoji 📽 Assembles a deck from eager import.meta.glob slide and thought template modules. */
export function loadPresentationFromSlideGlob(
	meta: PresentationMeta,
	globModules: Readonly<Record<string, { readonly default: SlideFile | ThoughtFile }>>,
): Presentation {
	const byChapter = new Map<string, Map<string, readonly SlideFileModule[]>>();
	const thoughtTemplateKeys = new Set<string>();
	for (const [importPath, module] of Object.entries(globModules)) {
		const slidePath = parsePresentationSlideFilePath(importPath);
		const thoughtPath = slidePath ? null : parsePresentationThoughtFilePath(importPath);
		if (!slidePath && !thoughtPath) {
			continue;
		}
		const byThought =
			byChapter.get((slidePath ?? thoughtPath)!.chapter) ?? new Map<string, readonly SlideFileModule[]>();
		const keyPath = slidePath ?? thoughtPath!;
		const thoughtKey = `${keyPath.sequence}\0${keyPath.thought}`;
		const existing = byThought.get(thoughtKey) ?? [];
		if (thoughtPath) {
			if (existing.length > 0) {
				throw new Error(
					`Thought "${thoughtPath.thought}" is defined both as ${presentationThoughtFilePath(thoughtPath.chapter, thoughtPath.sequence, thoughtPath.thought)} and as slide files under that folder.`,
				);
			}
			if (!isThoughtFile(module.default)) {
				throw new Error(
					`Expected a thought template export from ${presentationThoughtFilePath(thoughtPath.chapter, thoughtPath.sequence, thoughtPath.thought)}.`,
				);
			}
			thoughtTemplateKeys.add(thoughtKey);
			byThought.set(thoughtKey, expandThoughtFileToSlideModules(thoughtPath, module.default));
		} else {
			if (thoughtTemplateKeys.has(thoughtKey)) {
				throw new Error(
					`Thought "${slidePath!.thought}" already uses template file ${presentationThoughtFilePath(slidePath!.chapter, slidePath!.sequence, slidePath!.thought)}.`,
				);
			}
			byThought.set(thoughtKey, [...existing, { path: slidePath!, file: module.default as SlideFile }]);
		}
		byChapter.set(keyPath.chapter, byThought);
	}
	return {
		id: meta.id,
		name: meta.name,
		...(meta.language ? { language: meta.language } : {}),
		...(meta.width ? { width: meta.width } : {}),
		...(meta.height ? { height: meta.height } : {}),
		chapters: [...byChapter.entries()]
			.sort(([left], [right]) => left.localeCompare(right, "de"))
			.map(([chapterName, thoughtModules]) => assembleChapterFromSlideFiles(chapterName, thoughtModules)),
	};
}
//#endregion 🔖SlideFile

//#region 🔖Intro
/** @emoji 🎬 Spec for the standard paper intro template (title → description → goal → authors → affiliations ×3). */
export interface IntroSpec {
	readonly id?: string;
	readonly name?: string;
	readonly language?: PresentationLanguageKind;
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

const INTRO_EMBODIMENT_TITLE_FULL = "title--full";
const INTRO_EMBODIMENT_TITLE_SHORT = "title--short";
const INTRO_EMBODIMENT_DESCRIPTION_FULL = "description--full";
const INTRO_EMBODIMENT_DESCRIPTION_SHORT = "description--short";
const INTRO_EMBODIMENT_GOAL = "goal--main";
const INTRO_EMBODIMENT_AUTHORS_PLAIN = "authors--plain";
const INTRO_EMBODIMENT_AUTHORS_MARKED = "authors--marked";
const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1 = "authors--marked-affiliations-step1";
const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2 = "authors--marked-affiliations-step2";
const INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP3 = "authors--marked-affiliations-step3";
const INTRO_EMBODIMENT_INSTITUTIONS_STEP1 = "institutions--step1";
const INTRO_EMBODIMENT_INSTITUTIONS_STEP2 = "institutions--step2";
const INTRO_EMBODIMENT_INSTITUTIONS_STEP3 = "institutions--step3";

const INTRO_CHAPTER_BOOKMARK: Record<PresentationLanguageKind, string> = {
	en: "Main",
	de: "Hauptteil",
};

const INTRO_SEQUENCE_BOOKMARK: Record<PresentationLanguageKind, string> = {
	en: "Introduction",
	de: "Einführung",
};

const INTRO_THOUGHT_BOOKMARK: Record<PresentationLanguageKind, string> = {
	en: "Introduction",
	de: "Einleitung",
};

const INTRO_ARRANGEMENT_BOOKMARK: Record<PresentationLanguageKind, Record<string, string>> = {
	en: {
		title: "Title",
		description: "Description",
		goal: "Goal",
		authors: "Authors",
		"affiliations-1": "Faculty",
		"affiliations-2": "Universities",
		"affiliations-3": "Chairs",
	},
	de: {
		title: "Titel",
		description: "Beschreibung",
		goal: "Ziel",
		authors: "Autoren",
		"affiliations-1": "Fakultät",
		"affiliations-2": "Universitäten",
		"affiliations-3": "Lehrstühle",
	},
};

function introBookmarkLanguage(language: PresentationLanguageKind | undefined): PresentationLanguageKind {
	return language ?? "en";
}

function introArrangementBookmarkName(
	arrangementId: string,
	language: PresentationLanguageKind | undefined,
): string {
	return INTRO_ARRANGEMENT_BOOKMARK[introBookmarkLanguage(language)][arrangementId] ?? arrangementId;
}

function introParticipants(): Participant[] {
	return [
		{ id: INTRO_PARTICIPANT_TITLE },
		{ id: INTRO_PARTICIPANT_DESCRIPTION },
		{ id: INTRO_PARTICIPANT_GOAL },
		{ id: INTRO_PARTICIPANT_AUTHORS },
		{ id: INTRO_PARTICIPANT_INSTITUTIONS },
	];
}

function introEmbodiments(spec: IntroSpec): Embodiment[] {
	return [
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
		{
			kind: "text",
			id: INTRO_EMBODIMENT_GOAL,
			lines: spec.goal,
			level: "heading",
			morphRoot: "heading-block",
		},
		{
			kind: "authors",
			id: INTRO_EMBODIMENT_AUTHORS_PLAIN,
			lines: spec.authors.lines.map((line) => line.map((author) => ({ name: author.name }))),
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
			morphLineLabels: affiliationEmbodimentMorphLabels(spec.affiliations.steps[1], spec.affiliations.steps[2]),
		},
	];
}

/** @emoji 🎬 Slide modules for the standard intro thought (`slide/<chapter>/<sequence>/<thought>/<slide>.ts`). */
export function introSlideFiles(spec: IntroSpec): readonly SlideFile[] {
	const language = introBookmarkLanguage(spec.language);
	const participants = introParticipants();
	const embodiments = introEmbodiments(spec);
	const muted = (participantId: string, embodimentId: string): Disposition => ({
		participantId,
		embodimentId,
		emphasis: "muted",
	});
	const active = (participantId: string, embodimentId: string): Disposition => ({
		participantId,
		embodimentId,
		emphasis: "active",
	});
	const introMutedAboveAuthors = [
		muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
		muted(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_SHORT),
		muted(INTRO_PARTICIPANT_GOAL, INTRO_EMBODIMENT_GOAL),
	];
	const introArrangement = (
		arrangementId: string,
		dispositions: Arrangement["dispositions"],
	): Arrangement => ({
		id: arrangementId,
		name: introArrangementBookmarkName(arrangementId, language),
		dispositions,
	});
	return [
		{
			order: 0,
			participants,
			embodiments,
			arrangement: introArrangement("title", [active(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_FULL)]),
			transition: { kind: "morph" },
		},
		{
			order: 1,
			arrangement: introArrangement("description", [
				muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
				active(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_FULL),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 2,
			arrangement: introArrangement("goal", [
				muted(INTRO_PARTICIPANT_TITLE, INTRO_EMBODIMENT_TITLE_SHORT),
				muted(INTRO_PARTICIPANT_DESCRIPTION, INTRO_EMBODIMENT_DESCRIPTION_SHORT),
				active(INTRO_PARTICIPANT_GOAL, INTRO_EMBODIMENT_GOAL),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 3,
			arrangement: introArrangement("authors", [
				...introMutedAboveAuthors,
				active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_PLAIN),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 4,
			arrangement: introArrangement("affiliations-1", [
				...introMutedAboveAuthors,
				active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP1),
				active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP1),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 5,
			arrangement: introArrangement("affiliations-2", [
				...introMutedAboveAuthors,
				active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP2),
				active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP2),
			]),
			transition: { kind: "morph" },
		},
		{
			order: 6,
			arrangement: introArrangement("affiliations-3", [
				...introMutedAboveAuthors,
				active(INTRO_PARTICIPANT_AUTHORS, INTRO_EMBODIMENT_AUTHORS_MARKED_AFFILIATIONS_STEP3),
				active(INTRO_PARTICIPANT_INSTITUTIONS, INTRO_EMBODIMENT_INSTITUTIONS_STEP3),
			]),
		},
	];
}

/** @emoji 🎬 Intro thought module referencing {@link IntroSpec}. */
export function introThoughtFile(spec: IntroSpec): ThoughtFile {
	return { template: "intro", spec };
}

/** @emoji 🧩 Expands a thought template module into slide modules under its folder path. */
export function expandThoughtFileToSlideModules(path: ParsedThoughtFilePath, thought: ThoughtFile): SlideFileModule[] {
	if (thought.template !== "intro") {
		throw new Error(`Unknown thought template "${thought.template}" at ${presentationThoughtFilePath(path.chapter, path.sequence, path.thought)}.`);
	}
	const language = introBookmarkLanguage(thought.spec.language);
	return introSlideFiles(thought.spec).map((file) => ({
		path: {
			chapter: path.chapter,
			sequence: path.sequence,
			thought: path.thought,
			slide:
				file.arrangement.name ??
				INTRO_ARRANGEMENT_BOOKMARK[language][file.arrangement.id] ??
				file.arrangement.id,
		},
		file,
	}));
}

/** @emoji 🎬 Builds a seven-slide intro; each arrangement is that slide's target content for reveal.js auto-animate. */
export function intro(spec: IntroSpec): Presentation {
	const language = introBookmarkLanguage(spec.language);
	const chapterName = INTRO_CHAPTER_BOOKMARK[language];
	const sequenceName = INTRO_SEQUENCE_BOOKMARK[language];
	const thoughtName = INTRO_THOUGHT_BOOKMARK[language];
	const modules: SlideFileModule[] = introSlideFiles(spec).map((file, index) => ({
		path: {
			chapter: chapterName,
			sequence: sequenceName,
			thought: thoughtName,
			slide: file.arrangement.name ?? INTRO_ARRANGEMENT_BOOKMARK[language][file.arrangement.id] ?? `${index}`,
		},
		file,
	}));
	const chapter = assembleChapterFromSlideFiles(chapterName, new Map([[`${sequenceName}\0${thoughtName}`, modules]]));
	return {
		id: spec.id ?? "presentation",
		name: spec.name ?? spec.title.short,
		...(spec.language ? { language: spec.language } : {}),
		chapters: [chapter],
	};
}
//#endregion 🔖Intro

//#region 🔖Analogy
const ANALOGY_PARTICIPANT_LABEL = "label";
const ANALOGY_PARTICIPANT_VISUAL = "visual";
const ANALOGY_EMBODIMENT_LABEL_SOURCE = "label--source";
const ANALOGY_EMBODIMENT_LABEL_TARGET = "label--target";
const ANALOGY_EMBODIMENT_VISUAL_SOURCE = "visual--source";
const ANALOGY_EMBODIMENT_VISUAL_TARGET = "visual--target";

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
	const participants: Participant[] = [{ id: ANALOGY_PARTICIPANT_LABEL }];
	const embodiments: Embodiment[] = [
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
	];
	if (spec.source.figure) {
		embodiments.push({
			kind: "figure",
			id: ANALOGY_EMBODIMENT_VISUAL_SOURCE,
			src: spec.source.figure,
			alt: spec.source.label,
		});
	}
	if (spec.target.figure) {
		embodiments.push({
			kind: "figure",
			id: ANALOGY_EMBODIMENT_VISUAL_TARGET,
			src: spec.target.figure,
			alt: spec.target.label,
		});
	}
	if (spec.source.figure || spec.target.figure) {
		participants.push({ id: ANALOGY_PARTICIPANT_VISUAL });
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
		embodiments,
		slides: [
			{ arrangement: { id: "source", dispositions: sourceDispositions }, transition: { kind: "morph" } },
			{ arrangement: { id: "mapping", dispositions: mappingDispositions } },
		],
	};

	return {
		id: spec.id ?? "analogy",
		name: spec.name ?? `${spec.source.label} → ${spec.target.label}`,
		chapters: [
			{
				id: "main",
				sequences: [{ id: "main", thoughts: [thought] }],
			},
		],
	};
}
//#endregion 🔖Analogy

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

		it("expands morphFrom ghosts with line targets on morph target slides", () => {
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
			const expanded = expandThoughtSlides(thought);
			const labels = expanded.find((slide) => slide.id === "labels");
			const labelGhost = labels?.arrangement.dispositions.find((disposition) => disposition.morphGhost);
			expect(labelGhost?.morphTargetId).toBe("labels--0");
			expect(labelGhost?.style?.opacity).toBe(0);
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
			}
			expect(resolved[1]?.position?.width).toBe(0.6);
		});
	});
}
//#endregion 🧪Tests
