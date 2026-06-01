// #region 🧲Header
/** @emoji 📽 React + reveal.js renderer for `@framework/presentation/core` declarative decks. */
// #endregion 🧲Header

// #region 🔌Adapters
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import { Document, Page, pdfjs } from "react-pdf";
import "./globals.css";

pdfjs.GlobalWorkerOptions.workerSrc = new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url).toString();
import {
	applyElementsSurfaceChrome,
	Expertise,
	type ElementsSurfaceChromeInput,
} from "@ui/react";
import {
	act,
	createContext,
	Fragment,
	type CSSProperties,
	useContext,
	useEffect,
	useRef,
	useState,
	type FC,
	type ReactNode,
	type RefObject,
} from "react";
import { createRoot, type Root } from "react-dom/client";
import type {
	AffiliationEntry,
	AffiliationsEmbodiment,
	Arrangement,
	AuthorPerson,
	AuthorsEmbodiment,
	BulletEmbodiment,
	DispositionPosition,
	DispositionSplit,
	DispositionStyle,
	Embodiment,
	FigureEmbodiment,
	ParticipantEmphasis,
	PdfEmbodiment,
	Presentation,
	ResolvedDisposition,
	SplitColumnGroup,
	SplitMorphTarget,
	SplitTile,
	TextEmbodiment,
	Thought,
	Transition,
	VideoEmbodiment,
} from "@framework/presentation/core";
import {
	abbreviateAuthorFirstName,
	affiliationLineName,
	analogy,
	formatPresentationUrlHash,
	intro,
	parsePresentationSlideHash,
	presentationLanguage,
	presentationEntityBookmarkName,
	presentationSlideAt,
	presentationSlideBookmarkParamKeys,
	resolveArrangement,
	resolveEmbodiment,
	resolveTextMorphRoot,
	columnMorphId,
	splitColumnBounds,
	splitColumnCrop,
	splitFigureGrid,
	tileMorphId,
	type TextMorphRoot,
} from "@framework/presentation/core";
// #endregion 🔌Adapters

export type {
	AffiliationEntry,
	AffiliationsEmbodiment,
	Arrangement,
	AuthorPerson,
	AuthorsEmbodiment,
	BulletEmbodiment,
	Disposition,
	DispositionPosition,
	DispositionSplit,
	DispositionStyle,
	Embodiment,
	FigureEmbodiment,
	Participant,
	ParticipantEmphasis,
	PdfEmbodiment,
	Chapter,
	Presentation,
	ResolvedDisposition,
	Sequence,
	SplitColumnGroup,
	SplitMorphTarget,
	SplitTile,
	TextEmbodiment,
	Thought,
	Transition,
	VideoEmbodiment,
} from "@framework/presentation/core";

export {
	analogy,
	columnMorphId,
	countArrangements,
	collectPresentationSlides,
	formatPresentationUrlHash,
	intro,
	morphId,
	parsePresentationSlideHash,
	PRESENTATION_CHAPTER_QUERY_PARAM,
	PRESENTATION_SEQUENCE_QUERY_PARAM,
	PRESENTATION_SLIDE_QUERY_PARAM,
	PRESENTATION_THOUGHT_QUERY_PARAM,
	presentationSequences,
	presentationLanguage,
	presentationEntityBookmarkName,
	presentationSlideAt,
	presentationSlideBookmarkParamKeys,
	resolveArrangement,
	resolveEmbodiment,
	resolveTextMorphRoot,
	splitColumnBounds,
	splitColumnCrop,
	splitFigureGrid,
	tileMorphId,
} from "@framework/presentation/core";
export type {
	PresentationLanguageKind,
	PresentationSlideBookmark,
	PresentationSlideBookmarkParamKeys,
	PresentationSlideRef,
	TextMorphRoot,
} from "@framework/presentation/core";
export { Expertise } from "@ui/react";

//#region 🔖MountOptions
/** @emoji ⚙️ Reveal.js and @ui/react surface chrome options for {@link mountPresentation}. */
export interface PresentationMountOptions {
	readonly surfaceChrome?: ElementsSurfaceChromeInput | false;
	readonly transition?: "fade" | "slide" | "convex" | "concave" | "zoom" | "none";
	/** @emoji 🔗 Sync slide position to the URL hash; defaults to true. */
	readonly hash?: boolean;
	readonly slideNumber?: boolean;
	readonly width?: number;
	readonly height?: number;
}

/** @emoji 🔗 Writes reveal.js hash with localized bookmark params after the path; bookmark params are ignored for navigation. */
export function syncPresentationSlideUrl(
	presentation: Presentation,
	indices: { readonly h: number; readonly v: number },
): void {
	if (typeof window === "undefined") {
		return;
	}
	const slide = presentationSlideAt(presentation, indices);
	if (!slide) {
		return;
	}
	const url = new URL(window.location.href);
	url.search = "";
	url.hash = formatPresentationUrlHash(indices, slide, presentationLanguage(presentation));
	history.replaceState(null, "", url);
}

/** @emoji 🔗 Reads reveal.js slide indices from the URL hash; trailing bookmark query params are ignored. */
export function readPresentationSlideIndicesFromUrl(
	hash: string = typeof window !== "undefined" ? window.location.hash : "",
): { readonly h: number; readonly v: number } | null {
	return parsePresentationSlideHash(hash);
}
//#endregion 🔖MountOptions

const DEFAULT_SURFACE_CHROME: ElementsSurfaceChromeInput = {
	theme: "system",
	device: "desktop",
	expertise: Expertise.NORMAL,
};

//#region 🔖RevealChrome
/** @emoji 📐 Writes reveal slide dimensions as CSS variables for positioned arrangement canvases. */
export function syncPresentationSlideSizeVars(deckEl: HTMLElement | null, deck: Reveal.Api | null): void {
	if (!deckEl || !deck) {
		return;
	}
	const size = deck.getComputedSlideSize();
	deckEl.style.setProperty("--presentation-slide-width", `${size.width}px`);
	deckEl.style.setProperty("--presentation-slide-height", `${size.height}px`);
}

/** @emoji 🌓 Align reveal `has-dark-background` with `html.dark` from system chrome. */
export function syncRevealBackgroundKind(deckEl: HTMLElement | null): void {
	if (!deckEl || typeof document === "undefined") {
		return;
	}
	const dark = document.documentElement.classList.contains("dark");
	deckEl.classList.toggle("has-dark-background", dark);
	deckEl.classList.toggle("has-light-background", !dark);
}
//#endregion 🔖RevealChrome

//#region 🔖HiddenPreflight
/**
 * @emoji 🩹 Lets reveal.js own slide visibility by relaxing Tailwind preflight's `[hidden]` reset.
 *
 * `@ui/react` surface chrome ships Tailwind v4 preflight, whose layered
 * `[hidden]{display:none!important}` outranks reveal.js's inline `display:block` on the off-screen
 * slides it briefly un-hides to measure auto-animate `from`/`to` rects. The collapsed measurement
 * makes morph elements fly in from the deck origin instead of morphing in place. Dropping only the
 * `display` declaration restores the standard, non-important UA `[hidden]{display:none}` (so ordinary
 * hidden elements stay hidden) while reveal's inline `display` again wins for slides — giving native
 * reveal.js auto-animate exactly like {@link https://revealjs.com/auto-animate/}.
 */
export function relaxHiddenPreflight(): void {
	if (typeof document === "undefined") {
		return;
	}
	const visit = (rules: CSSRuleList): void => {
		for (const rule of Array.from(rules)) {
			const styleRule = rule as CSSStyleRule;
			if (
				typeof styleRule.selectorText === "string" &&
				styleRule.selectorText.includes("[hidden]") &&
				styleRule.style?.getPropertyValue("display") === "none"
			) {
				styleRule.style.removeProperty("display");
			}
			const grouping = rule as CSSGroupingRule;
			if (grouping.cssRules) {
				visit(grouping.cssRules);
			}
		}
	};
	const adopted = (document as unknown as { adoptedStyleSheets?: CSSStyleSheet[] }).adoptedStyleSheets ?? [];
	for (const sheet of [...Array.from(document.styleSheets), ...adopted]) {
		try {
			visit(sheet.cssRules);
		} catch {
			// cross-origin stylesheet rules are not readable
		}
	}
}
//#endregion 🔖HiddenPreflight

//#region 🔖MorphView
const presentationMorphTextClass = "presentation-morph-text";

function morphTextSizeClass(morphId: string): string {
	return morphId === "title" ? "presentation-morph-text--title" : "presentation-morph-text--secondary";
}

function morphTextClass(morphId: string, extra?: string): string {
	return [presentationMorphTextClass, morphTextSizeClass(morphId), extra].filter(Boolean).join(" ");
}

function emphasisClass(emphasis: ParticipantEmphasis): string | undefined {
	return emphasis === "muted" ? "opacity-20" : undefined;
}

function morphAnchorClass(emphasis: ParticipantEmphasis): string {
	return ["presentation-morph-anchor", emphasisClass(emphasis)].filter(Boolean).join(" ");
}

//#region 🔖SlideEpoch
const PresentationSlideEpochContext = createContext(0);

function parsePresentationSlideCssSize(revealEl: HTMLElement | null): { readonly width: number; readonly height: number } {
	const width = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-width") ?? "960");
	const height = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-height") ?? "700");
	return {
		width: Number.isFinite(width) && width > 0 ? width : 960,
		height: Number.isFinite(height) && height > 0 ? height : 700,
	};
}

/** @emoji 📐 Measures the disposition frame for react-pdf; re-runs when the slide becomes visible. */
function usePdfPageSize(
	anchorRef: RefObject<HTMLDivElement | null>,
	position: DispositionPosition | undefined,
	slideEpoch: number,
): { readonly width?: number; readonly height?: number } {
	const [size, setSize] = useState<{ readonly width?: number; readonly height?: number }>({});
	useEffect(() => {
		const el = anchorRef.current;
		if (!el) {
			return;
		}
		const measure = (): void => {
			const frame = el.closest(".presentation-disposition-frame");
			const frameRect = frame?.getBoundingClientRect();
			if (frameRect && frameRect.width > 8 && frameRect.height > 8) {
				setSize({
					width: Math.floor(frameRect.width),
					height: Math.floor(frameRect.height),
				});
				return;
			}
			const slide = parsePresentationSlideCssSize(el.closest(".reveal"));
			const width = position ? Math.floor(slide.width * position.width) : Math.floor(slide.width * 0.8);
			const height = position ? Math.floor(slide.height * position.height) : Math.floor(slide.height * 0.4);
			if (width > 0 && height > 0) {
				setSize({ width, height });
			}
		};
		measure();
		const frame = el.closest(".presentation-disposition-frame");
		const observer =
			frame && typeof ResizeObserver !== "undefined" ? new ResizeObserver(measure) : null;
		observer?.observe(frame);
		const visibility =
			typeof IntersectionObserver !== "undefined"
				? new IntersectionObserver((entries) => {
						if (entries[0]?.isIntersecting) {
							measure();
						}
					})
				: null;
		visibility?.observe(el);
		return () => {
			observer?.disconnect();
			visibility?.disconnect();
		};
	}, [anchorRef, position?.height, position?.width, position?.x, position?.y, slideEpoch]);
	return size;
}
//#endregion 🔖SlideEpoch

function lineClass(morphId: string, embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string | undefined {
	return [morphTextClass(morphId), embodiment.fit ? "r-fit-text" : undefined, emphasisClass(emphasis)]
		.filter(Boolean)
		.join(" ") || undefined;
}

function centeredLineClass(morphId: string, embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string {
	return [lineClass(morphId, embodiment, emphasis), "text-center"].filter(Boolean).join(" ");
}

/** @emoji 🎯 Renders {@link TextEmbodiment}; `data-id` sits on leaf text nodes so reveal.js does not double-match wrappers and headings. */
function textMorphAnchorId(anchorId: string, lineIndex: number, lineCount: number): string {
	return lineCount === 1 ? anchorId : `${anchorId}--${lineIndex}`;
}

function TextMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
}: {
	readonly morphId: string;
	readonly embodiment: TextEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	const root = resolveTextMorphRoot(embodiment);
	const centeredHeadingClass = centeredLineClass(anchorId, embodiment, emphasis);
	const lineCount = embodiment.lines.length;

	switch (root) {
		case "title":
			return (
				<h1 data-id={anchorId} className={centeredLineClass(anchorId, embodiment, emphasis)}>
					{embodiment.lines[0]}
				</h1>
			);
		case "body":
			return (
				<div className="w-full text-center">
					{embodiment.lines.map((line, lineIndex) => (
						<p
							key={line}
							data-id={textMorphAnchorId(anchorId, lineIndex, lineCount)}
							className={[lineClass(anchorId, embodiment, emphasis), "text-center"].filter(Boolean).join(" ") || "text-center"}
						>
							{line}
						</p>
					))}
				</div>
			);
		case "heading-line":
		case "subheading-line":
			return (
				<h2 data-id={anchorId} className={centeredHeadingClass}>
					{embodiment.lines[0]}
				</h2>
			);
		case "heading-block":
			return (
				<div className="w-full text-center">
					{embodiment.lines.map((line, lineIndex) => (
						<h2 key={line} data-id={textMorphAnchorId(anchorId, lineIndex, lineCount)} className={centeredHeadingClass}>
							{line}
						</h2>
					))}
				</div>
			);
		default: {
			const _exhaustive: never = root;
			return _exhaustive;
		}
	}
}

function authorRows(embodiment: AuthorsEmbodiment): readonly (readonly AuthorPerson[])[] {
	if (embodiment.lines && embodiment.lines.length > 0) {
		return embodiment.lines;
	}
	if (embodiment.people && embodiment.people.length > 0) {
		return [embodiment.people];
	}
	return [];
}

function authorDisplayName(person: AuthorPerson, embodiment: AuthorsEmbodiment): string {
	return embodiment.abbreviateFirstName ? abbreviateAuthorFirstName(person.name) : person.name;
}

function AuthorsMorphView({
	morphId: anchorId,
	embodiment,
}: {
	readonly morphId: string;
	readonly embodiment: AuthorsEmbodiment;
}): ReactNode {
	const namesMuted =
		embodiment.id === "marked" ||
		embodiment.id === "marked-affiliations" ||
		embodiment.id?.startsWith("marked-affiliations-step");
	const rows = authorRows(embodiment);
	return (
		<div className="presentation-intro-rows presentation-intro-authors flex w-full max-w-full flex-col items-center text-center">
			{rows.map((line, lineIndex) => (
				<div
					key={`${anchorId}-line-${lineIndex}`}
					className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center"
				>
					{line.map((person) => {
						const displayName = authorDisplayName(person, embodiment);
						return (
							<h4
								key={person.name}
								data-id={`${anchorId}--${person.name}`}
								className={morphTextClass(anchorId, "m-0 shrink-0 text-center")}
							>
								{namesMuted ? <span className="opacity-20">{displayName}</span> : displayName}
								{person.markEntries && person.markEntries.length > 0 ? (
									<sup className="ms-[0.35em]">
										{person.markEntries.map((entry, markIndex) => (
											<Fragment key={entry.mark}>
												{markIndex > 0 ? <span className="opacity-20">,</span> : null}
												<span className={emphasisClass(entry.emphasis)}>{entry.mark}</span>
											</Fragment>
										))}
									</sup>
								) : person.marks && person.marks.length > 0 ? (
									<sup className="ms-[0.35em]">{person.marks.join(",")}</sup>
								) : null}
							</h4>
						);
					})}
				</div>
			))}
		</div>
	);
}

function affiliationLineLabel(
	entry: AffiliationEntry,
	part: "line" | "suffix",
): string {
	if (part === "suffix" && entry.suffix) {
		return entry.suffix.name;
	}
	return affiliationLineName(entry);
}

function affiliationLineMuted(partEmphasis: ParticipantEmphasis | undefined): boolean {
	return partEmphasis === "muted";
}

function AffiliationsMorphView({
	morphId: anchorId,
	embodiment,
}: {
	readonly morphId: string;
	readonly embodiment: AffiliationsEmbodiment;
}): ReactNode {
	return (
		<div className="presentation-intro-rows presentation-intro-affiliations flex w-full max-w-full flex-col items-center text-center">
			{embodiment.entries.map((entry) => (
				<div
					key={entry.mark}
					className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center"
				>
					<h4
						data-id={`${anchorId}--${entry.mark}`}
						className={morphTextClass(anchorId, "m-0 shrink-0 text-center")}
					>
						{affiliationLineMuted(entry.lineEmphasis) ? (
							<span className="opacity-20">
								<sup>{entry.mark}</sup>
								{affiliationLineLabel(entry, "line")}
							</span>
						) : (
							<>
								<sup>{entry.mark}</sup>
								{affiliationLineLabel(entry, "line")}
							</>
						)}
					</h4>
					{entry.suffix ? (
						<h4
							data-id={`${anchorId}--${entry.suffix.mark}`}
							className={morphTextClass(anchorId, "m-0 shrink-0 text-center")}
						>
							{affiliationLineMuted(entry.suffixEmphasis) ? (
								<span className="opacity-20">
									<sup>{entry.suffix.mark}</sup>
									{affiliationLineLabel(entry, "suffix")}
								</span>
							) : (
								<>
									<sup>{entry.suffix.mark}</sup>
									{affiliationLineLabel(entry, "suffix")}
								</>
							)}
						</h4>
					) : null}
				</div>
			))}
		</div>
	);
}

function BulletMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
}: {
	readonly morphId: string;
	readonly embodiment: BulletEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	return (
		<div data-id={anchorId} className={emphasisClass(emphasis)}>
			<ul className={morphTextClass(anchorId)}>
				{embodiment.items.map((item) => (
					<li key={item}>{item}</li>
				))}
			</ul>
		</div>
	);
}

function figureTileBackgroundStyle(embodiment: FigureEmbodiment, crop: DispositionPosition): CSSProperties {
	const bgWidth = crop.width > 0 ? 100 / crop.width : 100;
	const bgHeight = crop.height > 0 ? 100 / crop.height : 100;
	const posX = crop.width >= 1 ? 0 : (crop.x / (1 - crop.width)) * 100;
	const posY = crop.height >= 1 ? 0 : (crop.y / (1 - crop.height)) * 100;
	return {
		backgroundImage: `url("${embodiment.src}")`,
		backgroundRepeat: "no-repeat",
		backgroundSize: `${bgWidth}% ${bgHeight}%`,
		backgroundPosition: `${posX}% ${posY}%`,
	};
}

function FigureTileView({
	participantId,
	tile,
	embodiment,
	defaultEmphasis,
	labelStackTile,
}: {
	readonly participantId: string;
	readonly tile: SplitTile;
	readonly embodiment: FigureEmbodiment;
	readonly defaultEmphasis: ParticipantEmphasis;
	/** @emoji 🏷 Target-side tile for column→label morph; hidden after auto-animate so only the heading stays. */
	readonly labelStackTile?: boolean;
}): ReactNode {
	const emphasis = tile.emphasis ?? defaultEmphasis;
	const frameStyle = dispositionFrameStyle(tile.position, tile.style);
	return (
		<div
			data-id={tileMorphId(participantId, tile.key)}
			className={[
				"presentation-disposition-frame",
				"presentation-figure-tile-frame",
				labelStackTile ? "presentation-column-morph-label-tile" : undefined,
				emphasisClass(emphasis),
			]
				.filter(Boolean)
				.join(" ")}
			style={{ ...frameStyle, ...figureTileBackgroundStyle(embodiment, tile.crop) }}
			role="img"
			aria-label={embodiment.alt ?? ""}
		/>
	);
}

/** @emoji 👻 One reveal.js morph slot per column: ghost (figure crop) or label (heading); identical `div > h2` + `data-id`. */
function ColumnMorphSlotView({
	morphId: anchorId,
	position,
	variant,
	line,
	figureEmbodiment,
	crop,
	textEmbodiment,
	emphasis,
	ghostVisibility,
	labelCompanion,
}: {
	readonly morphId: string;
	readonly position: DispositionPosition;
	readonly variant: "ghost" | "label";
	readonly line: string;
	readonly figureEmbodiment?: FigureEmbodiment;
	readonly crop?: SplitTile["crop"];
	readonly textEmbodiment?: TextEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly ghostVisibility?: "shown" | "hidden";
	readonly labelCompanion?: boolean;
}): ReactNode {
	const frameStyle = dispositionFrameStyle(position, undefined);
	const headingClass =
		variant === "label" && labelCompanion
			? "presentation-column-morph-slot-placeholder"
			: variant === "label" && textEmbodiment
				? centeredLineClass(anchorId, textEmbodiment, emphasis)
				: "presentation-column-morph-slot-placeholder";
	return (
		<div
			data-id={anchorId}
			className={[
				"presentation-disposition-frame",
				"presentation-column-morph-slot",
				variant === "ghost" ? "presentation-column-morph-slot--ghost" : "presentation-column-morph-slot--label",
				variant === "ghost" && ghostVisibility === "shown" ? "presentation-column-morph-slot--shown" : undefined,
				variant === "ghost" && ghostVisibility === "hidden" ? "presentation-column-morph-slot--hidden" : undefined,
				variant === "label" && labelCompanion ? "presentation-column-morph-slot--label-companion" : undefined,
				emphasisClass(emphasis),
			]
				.filter(Boolean)
				.join(" ")}
			style={{
				...frameStyle,
				...(variant === "ghost" && figureEmbodiment && crop
					? figureTileBackgroundStyle(figureEmbodiment, crop)
					: {}),
			}}
		>
			{variant === "ghost" || (variant === "label" && !labelCompanion) ? (
				<h2 className={headingClass}>{line}</h2>
			) : null}
		</div>
	);
}

function SplitColumnMorphGhostView({
	participantId,
	column,
	tiles,
	embodiment,
	shown,
}: {
	readonly participantId: string;
	readonly column: SplitColumnGroup;
	readonly tiles: readonly SplitTile[];
	readonly embodiment: FigureEmbodiment;
	readonly shown: boolean;
}): ReactNode {
	const position = splitColumnBounds(tiles, column.tileKeys);
	const crop = splitColumnCrop(tiles, column.tileKeys);
	return (
		<ColumnMorphSlotView
			morphId={columnMorphId(participantId, column.key)}
			position={position}
			variant="ghost"
			line={column.labelLine ?? "\u00a0"}
			figureEmbodiment={embodiment}
			crop={crop}
			emphasis="active"
			ghostVisibility={shown ? "shown" : "hidden"}
		/>
	);
}

function FigureMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
}: {
	readonly morphId: string;
	readonly embodiment: FigureEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	return (
		<div data-id={anchorId} className={morphAnchorClass(emphasis)}>
			<img className="presentation-media-figure" src={embodiment.src} alt={embodiment.alt ?? ""} />
		</div>
	);
}

/** @emoji 🏷 Per-tile figure morph into the stack; column heading stays fixed (no `data-id` on text). */
function ColumnLabelMorphView({
	participantId,
	morphId: tileMorphAnchorId,
	columnKey,
	embodiment,
	emphasis,
	position,
	figureEmbodiment,
	morphTile,
	columnMorphCompanion,
}: {
	readonly participantId: string;
	readonly morphId: string;
	readonly columnKey?: string;
	readonly embodiment: TextEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position: DispositionPosition;
	readonly figureEmbodiment?: FigureEmbodiment;
	readonly morphTile?: SplitTile;
	readonly columnMorphCompanion?: boolean;
}): ReactNode {
	const frameStyle = dispositionFrameStyle(position, undefined);
	const line = embodiment.lines[0] ?? "";
	const labelHeadingClass = columnKey
		? centeredLineClass(columnMorphId(participantId, columnKey), embodiment, emphasis)
		: centeredLineClass(tileMorphAnchorId, embodiment, emphasis);
	return (
		<>
			{morphTile && figureEmbodiment ? (
				<FigureTileView
					participantId={participantId}
					tile={morphTile}
					embodiment={figureEmbodiment}
					defaultEmphasis={emphasis}
					labelStackTile
				/>
			) : (
				<div
					data-id={tileMorphAnchorId}
					className={[
						"presentation-disposition-frame",
						"presentation-column-morph-tile-target",
						emphasisClass(emphasis),
					]
						.filter(Boolean)
						.join(" ")}
					style={frameStyle}
					aria-hidden
				/>
			)}
			{columnMorphCompanion !== true ? (
				<div
					className={[
						"presentation-disposition-frame",
						"presentation-column-morph-label",
						emphasisClass(emphasis),
					]
						.filter(Boolean)
						.join(" ")}
					style={frameStyle}
				>
					<h2 className={labelHeadingClass}>{line}</h2>
				</div>
			) : null}
		</>
	);
}

function FigureSplitMorphView({
	disposition,
	embodiment,
}: {
	readonly disposition: ResolvedDisposition;
	readonly embodiment: FigureEmbodiment;
}): ReactNode {
	const tiles = disposition.split?.tiles ?? [];
	const columns = disposition.split?.columns ?? [];
	const ghostsOnly = disposition.split?.columnGhostsOnly === true;
	const showColumnGhosts = ghostsOnly && columns.length > 0;
	return (
		<>
			{showColumnGhosts
				? columns.map((column) => (
						<SplitColumnMorphGhostView
							key={column.key}
							participantId={disposition.participant.id}
							column={column}
							tiles={tiles}
							embodiment={embodiment}
							shown={ghostsOnly}
						/>
					))
				: null}
			{!ghostsOnly
				? tiles.map((tile) => (
				<FigureTileView
					key={tile.key}
					participantId={disposition.participant.id}
					tile={tile}
					embodiment={embodiment}
					defaultEmphasis={disposition.emphasis}
				/>
				))
				: null}
		</>
	);
}

function VideoMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
}: {
	readonly morphId: string;
	readonly embodiment: VideoEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	return (
		<div data-id={anchorId} className={morphAnchorClass(emphasis)}>
			<video
				className="presentation-media-video"
				src={embodiment.src}
				poster={embodiment.poster}
				autoPlay={false}
				loop={embodiment.loop ?? false}
				muted={embodiment.muted ?? true}
				controls={embodiment.controls ?? true}
				playsInline
				preload="metadata"
			/>
		</div>
	);
}

function PdfMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
	position,
}: {
	readonly morphId: string;
	readonly embodiment: PdfEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
}): ReactNode {
	const anchorRef = useRef<HTMLDivElement>(null);
	const slideEpoch = useContext(PresentationSlideEpochContext);
	const pageSize = usePdfPageSize(anchorRef, position, slideEpoch);
	const pageHeight = pageSize.height;
	return (
		<div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
			<Document
				className="presentation-media-pdf-document"
				file={embodiment.src}
				loading={<span className="presentation-media-pdf-loading">…</span>}
				error={<span className="presentation-media-pdf-error">PDF</span>}
			>
				{pageHeight ? (
					<Page
						className="presentation-media-pdf"
						pageNumber={embodiment.page ?? 1}
						height={pageHeight}
						renderTextLayer={false}
						renderAnnotationLayer={false}
					/>
				) : null}
			</Document>
		</div>
	);
}

function dispositionFrameStyle(
	position: DispositionPosition | undefined,
	style: DispositionStyle | undefined,
): React.CSSProperties | undefined {
	if (!position && !style) {
		return undefined;
	}
	const frame: React.CSSProperties = position
		? {
				position: "absolute",
				left: `${position.x * 100}%`,
				top: `${position.y * 100}%`,
				width: `${position.width * 100}%`,
				height: `${position.height * 100}%`,
				boxSizing: "border-box",
			}
		: {};
	if (style?.opacity !== undefined) {
		frame.opacity = style.opacity;
	}
	const transforms: string[] = [];
	if (style?.rotate !== undefined) {
		transforms.push(`rotate(${style.rotate}deg)`);
	}
	if (style?.scale !== undefined) {
		transforms.push(`scale(${style.scale})`);
	}
	if (transforms.length > 0) {
		frame.transform = transforms.join(" ");
	}
	return frame;
}

function DispositionFrame({
	disposition,
	children,
	overlay,
}: {
	readonly disposition: ResolvedDisposition;
	readonly children: ReactNode;
	readonly overlay?: boolean;
}): ReactNode {
	const frameStyle = dispositionFrameStyle(disposition.position, disposition.style);
	if (!frameStyle) {
		return children;
	}
	return (
		<div
			className={[
				"presentation-disposition-frame",
				overlay ? "presentation-disposition-frame--overlay" : undefined,
			]
				.filter(Boolean)
				.join(" ")}
			style={frameStyle}
		>
			{children}
		</div>
	);
}

function MorphDispositionView({ disposition }: { readonly disposition: ResolvedDisposition }): ReactNode {
	const { embodiment, emphasis, morphId: anchorId } = disposition;
	let content: ReactNode;
	switch (embodiment.kind) {
		case "text":
			if (disposition.position !== undefined) {
				const figureEmbodiment = disposition.participant.embodiments.find(
					(candidate): candidate is FigureEmbodiment => candidate.kind === "figure",
				);
				if (disposition.morphTile && !figureEmbodiment) {
					throw new Error(
						`Participant "${disposition.participant.id}" needs a figure embodiment for column label morphs.`,
					);
				}
				return (
					<ColumnLabelMorphView
						participantId={disposition.participant.id}
						morphId={anchorId}
						columnKey={disposition.columnKey}
						embodiment={embodiment}
						emphasis={emphasis}
						position={disposition.position}
						figureEmbodiment={figureEmbodiment}
						morphTile={disposition.morphTile}
						columnMorphCompanion={disposition.columnMorphCompanion}
					/>
				);
			}
			content = <TextMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
			break;
		case "authors":
			content = <AuthorsMorphView morphId={anchorId} embodiment={embodiment} />;
			break;
		case "affiliations":
			content = <AffiliationsMorphView morphId={anchorId} embodiment={embodiment} />;
			break;
		case "bullet":
			content = <BulletMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
			break;
		case "figure":
			if (disposition.split) {
				return <FigureSplitMorphView disposition={disposition} embodiment={embodiment} />;
			}
			content = <FigureMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
			break;
		case "video":
			content = <VideoMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
			break;
		case "pdf":
			content = (
				<PdfMorphView
					morphId={anchorId}
					embodiment={embodiment}
					emphasis={emphasis}
					position={disposition.position}
				/>
			);
			break;
		default: {
			const _exhaustive: never = embodiment;
			content = _exhaustive;
		}
	}
	const overlay =
		disposition.embodiment.kind === "figure" &&
		disposition.position !== undefined &&
		disposition.split === undefined;
	return (
		<DispositionFrame disposition={disposition} overlay={overlay}>
			{content}
		</DispositionFrame>
	);
}
//#endregion 🔖MorphView

//#region 🔖ArrangementSection
function arrangementUsesMorph(transition: Transition | undefined): boolean {
	return (transition?.kind ?? "morph") === "morph";
}

const ArrangementSection: FC<{
	readonly thought: Thought;
	readonly arrangement: Arrangement;
	readonly transition?: Transition;
}> = ({ thought, arrangement, transition }) => {
	const resolved = resolveArrangement(thought, arrangement.id);
	const morph = arrangementUsesMorph(transition);
	const positioned = resolved.some((d) => d.position !== undefined || d.split !== undefined);
	const placements = resolved.map((disposition, index) => (
		<MorphDispositionView
			key={`${arrangement.id}-${disposition.morphId}-${disposition.embodimentId ?? index}`}
			disposition={disposition}
		/>
	));
	return (
		<section
			{...(morph ? { "data-auto-animate": "", "data-auto-animate-id": thought.id } : {})}
			title={arrangement.id}
			className={positioned ? "presentation-arrangement--positioned" : undefined}
		>
			{positioned ? <div className="presentation-arrangement-canvas">{placements}</div> : placements}
		</section>
	);
};
//#endregion 🔖ArrangementSection

//#region 🔖PresentationDeck
/** @emoji 🎞 Maps a {@link Presentation} to reveal.js DOM. */
export const PresentationDeck: FC<{
	readonly presentation: Presentation;
	readonly options?: PresentationMountOptions;
}> = ({ presentation, options }) => {
	const deckDivRef = useRef<HTMLDivElement>(null);
	const deckRef = useRef<Reveal.Api | null>(null);
	const [slideEpoch, setSlideEpoch] = useState(0);

	useEffect(() => {
		const deckEl = deckDivRef.current;
		if (!deckEl || deckRef.current) {
			return;
		}
		const slideUrlEnabled = options?.hash !== false;
		const revealOptions: Reveal.Options = {
			transition: options?.transition ?? "fade",
			autoAnimate: true,
			autoAnimateUnmatched: true,
			center: true,
		};
		// Custom hash format appends `?slide=` after the path; reveal.js hash sync stays off.
		if (options?.slideNumber === true) {
			revealOptions.slideNumber = true;
		}
		if (options?.width ?? presentation.width) {
			revealOptions.width = options?.width ?? presentation.width;
		}
		if (options?.height ?? presentation.height) {
			revealOptions.height = options?.height ?? presentation.height;
		}
		relaxHiddenPreflight();
		const deck = new Reveal(deckEl, revealOptions);
		deckRef.current = deck;
		const syncSlideUrl = (): void => {
			if (!slideUrlEnabled) {
				return;
			}
			syncPresentationSlideUrl(presentation, deck.getIndices());
		};
		const tryPlayVideo = (video: HTMLVideoElement): void => {
			try {
				const playResult = video.play();
				if (playResult !== undefined) {
					void playResult.catch(() => undefined);
				}
			} catch {
				// jsdom and browser autoplay policies
			}
		};
		const syncPresentSlideMedia = (): void => {
			if (import.meta.vitest) {
				return;
			}
			for (const video of deckEl.querySelectorAll<HTMLVideoElement>("video.presentation-media-video")) {
				const section = video.closest("section");
				const isPresent = section?.classList.contains("present") === true;
				if (isPresent) {
					tryPlayVideo(video);
				} else {
					video.pause();
				}
			}
		};
		const onResize = (): void => {
			syncPresentationSlideSizeVars(deckEl, deck);
		};
		const onSlideChanged = (): void => {
			relaxHiddenPreflight();
			syncRevealBackgroundKind(deckEl);
			syncPresentationSlideSizeVars(deckEl, deck);
			syncPresentSlideMedia();
			syncSlideUrl();
			setSlideEpoch((epoch) => epoch + 1);
		};
		const onWindowHashChange = (): void => {
			if (!slideUrlEnabled) {
				return;
			}
			const indices = readPresentationSlideIndicesFromUrl();
			if (!indices) {
				return;
			}
			const current = deck.getIndices();
			if (current.h !== indices.h || current.v !== indices.v) {
				void deck.slide(indices.h, indices.v);
			}
		};
		void deck.initialize().then(() => {
			relaxHiddenPreflight();
			syncRevealBackgroundKind(deckEl);
			syncPresentationSlideSizeVars(deckEl, deck);
			syncPresentSlideMedia();
			if (slideUrlEnabled) {
				const indices = readPresentationSlideIndicesFromUrl();
				const afterSlideSync = (): void => {
					syncSlideUrl();
					setSlideEpoch((epoch) => epoch + 1);
				};
				if (indices) {
					const current = deck.getIndices();
					if (current.h !== indices.h || current.v !== indices.v) {
						const slideResult = deck.slide(indices.h, indices.v) as Promise<void> | undefined;
						if (slideResult && typeof slideResult.then === "function") {
							void slideResult.then(afterSlideSync);
						} else {
							afterSlideSync();
						}
					} else {
						afterSlideSync();
					}
				} else {
					afterSlideSync();
				}
				window.addEventListener("hashchange", onWindowHashChange);
			} else {
				setSlideEpoch((epoch) => epoch + 1);
			}
			deck.on("slidechanged", onSlideChanged);
			deck.on("resize", onResize);
		});
		return () => {
			window.removeEventListener("hashchange", onWindowHashChange);
			deck.off("slidechanged", onSlideChanged);
			deck.off("resize", onResize);
			try {
				deck.destroy();
			} catch {
				// reveal destroy may throw if already torn down
			}
			deckRef.current = null;
		};
	}, []);

	return (
		<PresentationSlideEpochContext.Provider value={slideEpoch}>
			<div className="reveal" ref={deckDivRef} style={{ width: "100vw", height: "100vh" }}>
				<div className="slides">
				{presentation.chapters.flatMap((chapter) =>
					chapter.sequences.map((sequence) => (
						<section key={`${chapter.id}-${sequence.id}`}>
							{sequence.thoughts.flatMap((thought) =>
								thought.arrangements.map((arrangement) => (
									<ArrangementSection
										key={`${chapter.id}-${sequence.id}-${thought.id}-${arrangement.id}`}
										thought={thought}
										arrangement={arrangement}
										transition={thought.transition}
									/>
								)),
							)}
						</section>
					)),
				)}
				</div>
			</div>
		</PresentationSlideEpochContext.Provider>
	);
};
//#endregion 🔖PresentationDeck

//#region 🔖Mount
let mountedRoot: Root | null = null;
let surfaceChromeCleanup: (() => void) | null = null;

/** @emoji 🚀 Mounts a declarative presentation into a DOM root via React + reveal.js (eg-ice-25 reveal wiring). */
export function mountPresentation(
	rootEl: HTMLElement,
	presentation: Presentation,
	options?: PresentationMountOptions,
): void {
	surfaceChromeCleanup?.();
	surfaceChromeCleanup = null;
	const chrome = options?.surfaceChrome;
	if (chrome !== false) {
		surfaceChromeCleanup = applyElementsSurfaceChrome(chrome ?? DEFAULT_SURFACE_CHROME);
	}
	mountedRoot?.unmount();
	mountedRoot = createRoot(rootEl);
	mountedRoot.render(<PresentationDeck presentation={presentation} options={options} />);
}

/** @emoji 🧹 Unmounts a presentation previously mounted with {@link mountPresentation}. */
export function unmountPresentation(): void {
	mountedRoot?.unmount();
	mountedRoot = null;
	surfaceChromeCleanup?.();
	surfaceChromeCleanup = null;
}
//#endregion 🔖Mount

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it, beforeEach, afterEach } = import.meta.vitest;

	describe("PresentationDeck", () => {
		let container: HTMLDivElement;

		const testAffiliationSteps = {
			steps: [
				[{ mark: "a", name: "Faculty" }],
				[
					{ mark: "a", name: "Faculty" },
					{ mark: "1", name: "Uni" },
				],
				[
					{ mark: "a", name: "Faculty" },
					{ mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
				],
			],
		} as const;

		beforeEach(() => {
			container = document.createElement("div");
			document.body.appendChild(container);
		});

		afterEach(() => {
			unmountPresentation();
			container.remove();
		});

		it("renders seven vertical sections for the intro template", () => {
			const deck = intro({
				title: { full: ["A", "B", "C"], short: "Short" },
				description: { full: ["D1", "D2"], short: "D short" },
				goal: ["G1"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false });
			});
			const sections = container.querySelectorAll(".slides > section > section");
			expect(sections[0]?.hasAttribute("data-auto-animate")).toBe(true);
			expect(sections.length).toBe(7);
			const revealEl = container.querySelector(".reveal");
			expect(revealEl?.getAttribute("style")).toContain("100vw");
			expect(container.querySelector('[data-id^="title"]')).toBeTruthy();
			expect(container.querySelector('[data-id^="description"]')).toBeTruthy();
			expect(container.querySelector('[data-id="goal"]')).toBeTruthy();
			expect(container.querySelector('[data-id^="authors--"]')).toBeTruthy();
			expect(container.querySelector('[data-id^="institutions--"]')).toBeTruthy();
		});

		it("centers intro flow slides without a positioned arrangement canvas", () => {
			const deck = intro({
				title: { full: ["A", "B", "C"], short: "Short" },
				description: { full: ["D1", "D2"], short: "D short" },
				goal: ["G1"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="intro"]')) {
				expect(slide.classList.contains("presentation-arrangement--positioned")).toBe(false);
				expect(slide.querySelector(".presentation-arrangement-canvas")).toBeNull();
			}
		});

		it("applies muted opacity on layered description slide", () => {
			const deck = intro({
				title: { full: ["A"], short: "Short" },
				description: { full: ["D"], short: "D short" },
				goal: ["G"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const descriptionSlide = container.querySelector('.slides > section > section[title="description"]');
			expect(descriptionSlide?.querySelector(".opacity-20")).toBeTruthy();
		});

		it("matches eg-ice-25 intro morph DOM per arrangement", () => {
			const deck = intro({
				title: { full: ["A", "B", "C"], short: "Short" },
				description: { full: ["D1", "D2"], short: "D short" },
				goal: ["G1"],
				authors: {
					lines: [
						[{ name: "Alice Example", marks: ["a", "1", "x"] }, { name: "Bob Beta", marks: ["a", "1", "x"] }],
						[{ name: "Carol Creator" }],
					],
				},
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
			expect(slide("title")?.querySelectorAll('h2[data-id^="title"]').length).toBe(3);
			expect(slide("description")?.querySelector('h2[data-id="title"]')).toBeTruthy();
			expect(slide("description")?.querySelector('div[data-id="title"]')).toBeNull();
			expect(slide("description")?.querySelectorAll('h2[data-id^="description"]').length).toBe(2);
			expect(slide("goal")?.querySelector('h2[data-id="description"]')?.textContent).toBe("D short");
			expect(slide("goal")?.querySelector('h2[data-id="goal"]')).toBeTruthy();
			const authorLines = slide("authors")?.querySelectorAll('h4[data-id^="authors--"]');
			expect(authorLines?.length).toBe(3);
			expect(slide("affiliations-1")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(1);
			expect(slide("affiliations-2")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
			expect(slide("affiliations-3")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(3);
			expect(slide("affiliations-2")?.querySelector('h5[data-id="institutions"]')).toBeNull();
			expect(slide("affiliations-2")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("Uni");
			expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
			expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--x"]')?.textContent).toContain("Chair X");
			expect(slide("affiliations-3")?.textContent).toContain("Chair X");
			expect(slide("affiliations-1")?.querySelector('h4[data-id="authors--Alice Example"] sup')?.textContent).toBe("a");
			const marked2 = slide("affiliations-2")?.querySelector('h4[data-id="authors--Alice Example"] sup');
			expect(marked2?.textContent).toBe("a,1");
			expect(marked2?.querySelector('h4[data-id="authors--Alice Example"] sup span:not(.opacity-20)')?.textContent).toBe(
				"1",
			);
			const marked3 = slide("affiliations-3")?.querySelector('h4[data-id="authors--Alice Example"] sup');
			expect(marked3?.textContent).toBe("a,1,x");
			expect(marked3?.querySelector('h4[data-id="authors--Alice Example"] sup span:not(.opacity-20)')?.textContent).toBe(
				"x",
			);
			expect(slide("affiliations-1")?.querySelector('[data-id="authors--Alice Example"]')?.textContent).toContain("A. Example");
			expect(slide("affiliations-1")?.querySelector('[data-id="authors--Alice Example"] .opacity-20')).toBeTruthy();
			expect(slide("authors")?.querySelector('[data-id="authors--Alice Example"] .opacity-20')).toBeNull();
			expect(slide("authors")?.querySelector('[data-id="authors--Alice Example"]')?.textContent).toContain("Alice Example");
			const aff2 = slide("affiliations-2");
			expect(aff2?.querySelector('h4[data-id="institutions--a"] .opacity-20')).toBeTruthy();
			expect(aff2?.querySelector('h4[data-id="institutions--1"] .opacity-20')).toBeNull();
			const aff3 = slide("affiliations-3");
			expect(aff3?.querySelector('h4[data-id="institutions--a"] .opacity-20')).toBeTruthy();
			expect(aff3?.querySelector('h4[data-id="institutions--1"] .opacity-20')?.textContent).toContain("LUH");
			expect(aff3?.querySelector('h4[data-id="institutions--x"] .opacity-20')).toBeNull();
		});

		it("applies title and secondary morph text sizes on intro slides", () => {
			const deck = intro({
				title: { full: ["A", "B", "C"], short: "Short" },
				description: { full: ["D1", "D2"], short: "D short" },
				goal: ["G1"],
				authors: {
					lines: [
						[{ name: "Alice", marks: ["1"] }, { name: "Bob" }],
						[{ name: "Carol" }],
					],
				},
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const expectMorphClass = (selector: string, sizeClass: string) => {
				for (const node of container.querySelectorAll(selector)) {
					expect(node.classList.contains("presentation-morph-text")).toBe(true);
					expect(node.classList.contains(sizeClass)).toBe(true);
				}
			};
			expectMorphClass('[data-id^="title"]', "presentation-morph-text--title");
			expectMorphClass('[data-id^="description"], [data-id="goal"]', "presentation-morph-text--secondary");
			expect(container.querySelector('[data-id^="title"].presentation-morph-text--secondary')).toBeNull();
			expect(container.querySelector('[data-id="goal"].presentation-morph-text--title')).toBeNull();
		});

		it("does not use reveal fit-text on intro headings", () => {
			const deck = intro({
				title: { full: ["A", "B"], short: "Short" },
				description: { full: ["D1"], short: "D short" },
				goal: ["G1"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			expect(container.querySelector(".r-fit-text")).toBeNull();
		});

		it("enables reveal auto-animate and tags every morph arrangement with data-auto-animate", () => {
			const deck = intro({
				title: { full: ["A", "B", "C"], short: "Short" },
				description: { full: ["D1", "D2"], short: "D short" },
				goal: ["G1"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const morphSections = container.querySelectorAll(
				'.slides > section > section[data-auto-animate][data-auto-animate-id="intro"]',
			);
			expect(morphSections.length).toBe(7);
			const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
			expect(slide("title")?.querySelector('[data-id^="title"]')).toBeTruthy();
			expect(slide("title")?.querySelector('[data-id^="description"]')).toBeNull();
			expect(slide("description")?.querySelector('[data-id^="description"]')).toBeTruthy();
			expect(slide("goal")?.querySelector('[data-id="goal"]')).toBeTruthy();
		});

		it("renders video and pdf embodiments with data-id for auto-animate", () => {
			const deck: Presentation = {
				id: "media-test",
				name: "Media",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "media",
								thoughts: [
									{
										id: "media",
										transition: { kind: "morph" },
										participants: [
											{ id: "clip", embodiments: [{ kind: "video", src: "/demo.mp4" }] },
											{ id: "doc", embodiments: [{ kind: "pdf", src: "/paper.pdf", page: 1 }] },
										],
										arrangements: [
											{
												id: "slide",
												dispositions: [
													{ participantId: "clip", emphasis: "active" },
													{ participantId: "doc", emphasis: "active" },
												],
											},
										],
									},
								],
							},
						],
					},
				],
			};
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			expect(container.querySelector('[data-id="clip"] video[src="/demo.mp4"]')).toBeTruthy();
			expect(container.querySelector('[data-id="doc"] .react-pdf__Document')).toBeTruthy();
		});

		it("applies absolute positioning for dispositions with position", () => {
			const deck: Presentation = {
				id: "position-test",
				name: "Position",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "pos",
								thoughts: [
									{
										id: "pos",
										participants: [
											{
												id: "box",
												embodiments: [{ kind: "text", lines: ["A"], level: "body" }],
											},
										],
										arrangements: [
											{
												id: "placed",
												dispositions: [
													{
														participantId: "box",
														emphasis: "active",
														position: { x: 0.1, y: 0.2, width: 0.5, height: 0.3 },
													},
												],
											},
										],
									},
								],
							},
						],
					},
				],
			};
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const frame = container.querySelector(".presentation-disposition-frame") as HTMLElement | null;
			expect(frame?.style.position).toBe("absolute");
			expect(frame?.style.left).toBe("10%");
			expect(frame?.style.width).toBe("50%");
		});

		it("renders split figure tiles with per-tile data-id and background crops", () => {
			const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
			const deck: Presentation = {
				id: "split-figure",
				name: "Split",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "split",
										transition: { kind: "morph" },
										participants: [
											{
												id: "catalogue",
												embodiments: [{ kind: "figure", src: "/catalogue.png", alt: "Catalogue" }],
											},
										],
										arrangements: [
											{
												id: "tiles",
												dispositions: [
													{
														participantId: "catalogue",
														emphasis: "active",
														split: {
															tiles: splitFigureGrid({ rows: 2, columns: 2, frame }),
														},
													},
												],
											},
										],
									},
								],
							},
						],
					},
				],
			};
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const tiles = container.querySelectorAll('[data-id^="catalogue--tile--"]');
			expect(tiles.length).toBe(4);
			const first = tiles[0] as HTMLElement;
			expect(first.classList.contains("presentation-figure-tile-frame")).toBe(true);
			expect(first.style.position).toBe("absolute");
			expect(first.style.backgroundImage).toContain("/catalogue.png");
		});

		it("omits tiles not listed in a split disposition", () => {
			const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
			const allTiles = splitFigureGrid({ rows: 2, columns: 2, frame });
			const deck: Presentation = {
				id: "partial-split",
				name: "Partial",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "partial",
										transition: { kind: "morph" },
										participants: [
											{
												id: "catalogue",
												embodiments: [{ kind: "figure", src: "/catalogue.png" }],
											},
										],
										arrangements: [
											{
												id: "focus",
												dispositions: [
													{
														participantId: "catalogue",
														emphasis: "active",
														split: { tiles: [allTiles[0]!, allTiles[1]!] },
													},
												],
											},
										],
									},
								],
							},
						],
					},
				],
			};
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			expect(container.querySelectorAll('[data-id^="catalogue--tile--"]').length).toBe(2);
		});

		it("does not render column ghosts on focus when only columnMorphTiles is set", () => {
			const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
			const tiles = splitFigureGrid({ rows: 2, columns: 2, frame, gap: 0.05 });
			const columns = [
				{ key: "col1", tileKeys: ["tile-r0-c0", "tile-r1-c0"], labelLine: "A" },
				{ key: "col2", tileKeys: ["tile-r0-c1", "tile-r1-c1"], labelLine: "B" },
			];
			const deck: Presentation = {
				id: "column-tile-focus",
				name: "Column tiles",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "morph",
										transition: { kind: "morph" },
										participants: [
											{
												id: "catalogue",
												embodiments: [{ kind: "figure", src: "/catalogue.png" }],
											},
										],
										arrangements: [
											{
												id: "focus",
												dispositions: [
													{
														participantId: "catalogue",
														emphasis: "active",
														split: { tiles, columns, columnMorphTiles: true },
													},
												],
											},
										],
									},
								],
							},
						],
					},
				],
			};
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const focus = container.querySelector('section[title="focus"]');
			expect(focus?.querySelectorAll(".presentation-figure-tile-frame").length).toBe(4);
			expect(focus?.querySelectorAll(".presentation-column-morph-slot--ghost").length).toBe(0);
		});

		it("morphs each tile into a fixed label via tileMorphId sinks", () => {
			const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
			const tiles = splitFigureGrid({ rows: 2, columns: 2, frame, gap: 0.05 });
			const columns = [
				{ key: "col1", tileKeys: ["tile-r0-c0", "tile-r1-c0"], labelLine: "A" },
				{ key: "col2", tileKeys: ["tile-r0-c1", "tile-r1-c1"], labelLine: "B" },
			];
			const deck: Presentation = {
				id: "stacked-label-morph",
				name: "Stacked labels",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "morph",
										transition: { kind: "morph" },
										participants: [
											{
												id: "catalogue",
												embodiments: [{ kind: "figure", src: "/catalogue.png" }],
											},
										],
										arrangements: [
											{
												id: "focus",
												dispositions: [
													{
														participantId: "catalogue",
														emphasis: "active",
														split: { tiles, columns, columnMorphTiles: true },
													},
												],
											},
											{
												id: "labels",
												dispositions: [
													{
														participantId: "catalogue",
														emphasis: "active",
														morphSourceTiles: tiles,
														morphColumnGroups: columns,
														morphTargets: [
															{
																columnKey: "col1",
																position: { x: 0.4, y: 0.1, width: 0.2, height: 0.2 },
																lines: ["A"],
															},
															{
																columnKey: "col2",
																position: { x: 0.4, y: 0.35, width: 0.2, height: 0.2 },
																lines: ["B"],
															},
														],
													},
												],
											},
										],
									},
								],
							},
						],
					},
				],
			};
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const labels = container.querySelector('section[title="labels"]');
			const labelTiles = [...labels!.querySelectorAll(".presentation-column-morph-label-tile")].map((element) =>
				element.getAttribute("data-id"),
			);
			expect(new Set(labelTiles).size).toBe(4);
			expect(labels?.querySelectorAll(".presentation-column-morph-label h2").length).toBe(2);
			expect(labels?.querySelector(".presentation-column-morph-label h2[data-id]")).toBeNull();
			expect(
				labels?.querySelector('.presentation-column-morph-label-tile[data-id="catalogue--tile--tile-r0-c0"]'),
			).toBeTruthy();
		});

		it("relaxes Tailwind preflight [hidden] so reveal's inline display drives slide visibility", () => {
			const style = document.createElement("style");
			style.textContent = '[hidden]:where(:not([hidden="until-found"])) { display: none; color: red; }';
			document.head.appendChild(style);
			const hiddenRule = (style.sheet as CSSStyleSheet).cssRules[0] as CSSStyleRule;
			expect(hiddenRule.style.getPropertyValue("display")).toBe("none");
			relaxHiddenPreflight();
			expect(hiddenRule.style.getPropertyValue("display")).toBe("");
			expect(hiddenRule.style.getPropertyValue("color")).toBe("red");
			style.remove();
		});

		it("does not navigate from bookmark query params", () => {
			expect(readPresentationSlideIndicesFromUrl("#/?sequence=main&thought=intro&slide=goal")).toEqual({
				h: 0,
				v: 0,
			});
			expect(readPresentationSlideIndicesFromUrl("#/0/2?sequence=main&thought=intro&slide=goal")).toEqual({
				h: 0,
				v: 2,
			});
			history.replaceState(null, "", "/presentation");
		});
	});

	describe("syncPresentationSlideUrl", () => {
		const sampleDeck = intro({
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

		it("uses German bookmark labels for de intro decks", () => {
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
			history.replaceState(null, "", "/deck");
			syncPresentationSlideUrl(deck, { h: 0, v: 2 });
			const params = new URLSearchParams(new URL(window.location.href).hash.split("?")[1] ?? "");
			expect(params.get("kapitel")).toBe("Hauptteil");
			expect(params.get("sequenz")).toBe("Einführung");
			expect(params.get("gedanke")).toBe("Einleitung");
			expect(params.get("folie")).toBe("Ziel");
			history.replaceState(null, "", "/deck");
		});

		it("writes chapter, sequence, thought, and slide bookmark params after the hash path", () => {
			history.replaceState(null, "", "/deck");
			syncPresentationSlideUrl(sampleDeck, { h: 0, v: 2 });
			const url = new URL(window.location.href);
			expect(url.search).toBe("");
			expect(url.hash).toBe(
				"#/0/2?chapter=Main&sequence=Introduction&thought=Introduction&slide=Goal",
			);
			history.replaceState(null, "", "/deck");
		});

		it("readPresentationSlideIndicesFromUrl ignores bookmark query params", () => {
			expect(readPresentationSlideIndicesFromUrl("#/1/3")).toEqual({ h: 1, v: 3 });
			expect(readPresentationSlideIndicesFromUrl("")).toEqual({ h: 0, v: 0 });
		});
	});
}
//#endregion 🧪Tests
