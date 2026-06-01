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
	useCallback,
	useContext,
	useEffect,
	useLayoutEffect,
	useMemo,
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
	RenderSlide,
	ResolvedDisposition,
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
	centerResolvedArrangement,
	collectPresentationSlides,
	expandThoughtSlides,
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
	splitFigureGrid,
	splitTilesBoundingFrame,
	splitTilesPackedFrame,
	splitTilesUnionSourceCrop,
	tileMorphId,
	unionDispositionPositions,
	type MorphFromSlot,
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
	RenderSlide,
	ResolvedDisposition,
	Sequence,
	Slide,
	SplitTile,
	TextEmbodiment,
	Thought,
	Transition,
	VideoEmbodiment,
} from "@framework/presentation/core";

export {
	analogy,
	countArrangements,
	collectPresentationSlides,
	expandThoughtSlides,
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
	splitFigureGrid,
	splitTilesBoundingFrame,
	splitTilesPackedFrame,
	splitTilesUnionSourceCrop,
	tileMorphId,
} from "@framework/presentation/core";
export type {
	MorphFromSlot,
	PresentationLanguageKind,
	PresentationSlideBookmark,
	PresentationSlideBookmarkParamKeys,
	PresentationSlideRef,
	RenderSlide,
	Slide,
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

//#region 🔖ArrangementSettled
/** @emoji ⏳ Clears settled state on arrival so split tiles stay visible at rest; drops settled on slides no longer adjacent. */
export function syncArrangementSettledState(
	deckEl: HTMLElement,
	currentSlide: HTMLElement | null,
	previousSlide: HTMLElement | null,
): void {
	const settleSections = deckEl.querySelectorAll<HTMLElement>("section[data-settle-before-morph-to]");
	for (const section of settleSections) {
		if (section !== currentSlide && section !== previousSlide) {
			section.classList.remove("presentation-arrangement--settled");
		}
	}
	if (currentSlide?.hasAttribute("data-settle-before-morph-to")) {
		currentSlide.classList.remove("presentation-arrangement--settled");
	}
}

/** @emoji ⏳ Swaps split tiles for dormant morph anchors on the outgoing slide when auto-animating to a listed target. */
export function prepareArrangementBeforeAutoAnimate(fromSlide: HTMLElement, toSlide: HTMLElement): void {
	const settleBefore = fromSlide.getAttribute("data-settle-before-morph-to");
	if (!settleBefore) {
		return;
	}
	const toIds = settleBefore.split(",").filter((id) => id.length > 0);
	const toId = toSlide.getAttribute("title");
	if (!toId || !toIds.includes(toId)) {
		return;
	}
	fromSlide.classList.add("presentation-arrangement--settled");
	void fromSlide.offsetHeight;
}

type AutoAnimateMatcherHost = {
	findAutoAnimateMatches: (
		pairs: { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[],
		fromScope: HTMLElement,
		toScope: HTMLElement,
		selector: string,
		serializer: (node: HTMLElement) => string,
		animationOptions?: Record<string, unknown>,
	) => void;
};

/** @emoji 🔗 reveal.js auto-animate matcher: only `data-id` pairs so figure crops never morph via placeholder label text. */
export function presentationAutoAnimateMatcher(
	this: AutoAnimateMatcherHost,
	fromSlide: HTMLElement,
	toSlide: HTMLElement,
): { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[] {
	const pairs: { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[] = [];
	this.findAutoAnimateMatches(pairs, fromSlide, toSlide, "[data-id]", (node) => {
		return `${node.nodeName}:::${node.getAttribute("data-id")}`;
	});
	const reserved: HTMLElement[] = [];
	return pairs.filter((pair) => {
		if (reserved.includes(pair.to)) {
			return false;
		}
		reserved.push(pair.to);
		return true;
	});
}

/** @emoji 📐 Rewrites reveal FLIP `scale(sx, sy)` to `scale(max(sx,sy))` so figure tiles zoom instead of squashing during auto-animate. */
export function patchAutoAnimateUniformScale(css: string): string {
	return css.replace(/scale\(([\d.]+),\s*([\d.]+)\)/g, (_match, scaleX: string, scaleY: string) => {
		const sx = Number.parseFloat(scaleX);
		const sy = Number.parseFloat(scaleY);
		if (!Number.isFinite(sx) || !Number.isFinite(sy)) {
			return `scale(${scaleX}, ${scaleY})`;
		}
		return `scale(${Math.max(sx, sy)})`;
	});
}
//#endregion 🔖ArrangementSettled

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
					className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center gap-x-[0.35em]"
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
					className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center gap-x-[0.35em]"
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

/** @emoji 🖼 CSS vars for crop tiles: stretch-fill at rest, uniform cover zoom while reveal auto-animates. */
export function figureCropBackgroundVars(
	embodiment: FigureEmbodiment,
	crop: DispositionPosition,
	frame?: DispositionPosition,
): CSSProperties {
	const stretchWidth = crop.width > 0 ? 100 / crop.width : 100;
	const stretchHeight = crop.height > 0 ? 100 / crop.height : 100;
	const stretchPosX = crop.width >= 1 ? 0 : (crop.x / (1 - crop.width)) * 100;
	const stretchPosY = crop.height >= 1 ? 0 : (crop.y / (1 - crop.height)) * 100;
	let morphWidth = stretchWidth;
	let morphHeight = stretchHeight;
	let morphPosX = stretchPosX;
	let morphPosY = stretchPosY;
	let restWidth = stretchWidth;
	let restHeight = stretchHeight;
	let restPosX = stretchPosX;
	let restPosY = stretchPosY;
	if (frame !== undefined && frame.width > 0 && frame.height > 0 && crop.width > 0 && crop.height > 0) {
		const cropAspect = crop.width / crop.height;
		const frameAspect = frame.width / frame.height;
		const coverScale = Math.max(frameAspect / cropAspect, cropAspect / frameAspect);
		morphWidth = stretchWidth * coverScale;
		morphHeight = stretchHeight * coverScale;
		morphPosX = crop.width >= 1 ? 50 : (crop.x + crop.width / 2) * 100;
		morphPosY = crop.height >= 1 ? 50 : (crop.y + crop.height / 2) * 100;
		if (coverScale > 1 + 1e-6) {
			restWidth = morphWidth;
			restHeight = morphHeight;
			restPosX = morphPosX;
			restPosY = morphPosY;
		}
	}
	return {
		backgroundImage: `url("${embodiment.src}")`,
		["--presentation-figure-bg-size" as string]: `${restWidth}% ${restHeight}%`,
		["--presentation-figure-bg-position" as string]: `${restPosX}% ${restPosY}%`,
		["--presentation-figure-bg-size-morph" as string]: `${morphWidth}% ${morphHeight}%`,
		["--presentation-figure-bg-position-morph" as string]: `${morphPosX}% ${morphPosY}%`,
	};
}

function FigureTileView({
	participantId,
	tile,
	embodiment,
	defaultEmphasis,
	tileDuplicateHidden,
}: {
	readonly participantId: string;
	readonly tile: SplitTile;
	readonly embodiment: FigureEmbodiment;
	readonly defaultEmphasis: ParticipantEmphasis;
	readonly tileDuplicateHidden?: boolean;
}): ReactNode {
	const emphasis = tile.emphasis ?? defaultEmphasis;
	const frameStyle = dispositionFrameStyle(tile.position, tile.style);
	return (
		<div
			data-id={tileMorphId(participantId, tile.key)}
			className={[
				"presentation-disposition-frame",
				"presentation-figure-tile-frame",
				tileDuplicateHidden ? "presentation-figure-tile-frame--morph-participant-duplicate" : undefined,
				emphasisClass(emphasis),
			]
				.filter(Boolean)
				.join(" ")}
			style={frameStyle}
			role="img"
			aria-label={embodiment.alt ?? ""}
		>
			<div
				className="presentation-figure-crop-fill"
				style={figureCropBackgroundVars(embodiment, tile.crop, tile.position)}
			/>
		</div>
	);
}

function FigureMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
	position,
	style,
}: {
	readonly morphId: string;
	readonly embodiment: FigureEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
}): ReactNode {
	if (embodiment.crop && position) {
		const dormant = style?.opacity === 0;
		const frameStyle = dispositionFrameStyle(position, dormant ? undefined : style);
		return (
			<div
				data-id={anchorId}
				className={[
					"presentation-disposition-frame",
					"presentation-morph-slot",
					"presentation-morph-slot--figure",
					dormant ? "presentation-morph-slot--dormant" : undefined,
					emphasisClass(emphasis),
				]
					.filter(Boolean)
					.join(" ")}
				style={frameStyle}
				role="img"
				aria-label={embodiment.alt ?? ""}
			>
				<div
					className="presentation-figure-crop-fill"
					style={figureCropBackgroundVars(embodiment, embodiment.crop, position)}
				/>
			</div>
		);
	}
	return (
		<div data-id={anchorId} className={morphAnchorClass(emphasis)}>
			<img className="presentation-media-figure" src={embodiment.src} alt={embodiment.alt ?? ""} />
		</div>
	);
}

/** @emoji 🏷 Positioned text morph slot: `data-id` on the frame so reveal.js can morph figure crops into labels. */
function PositionedTextMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
	position,
	style,
}: {
	readonly morphId: string;
	readonly embodiment: TextEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position: DispositionPosition;
	readonly style?: DispositionStyle;
}): ReactNode {
	const frameStyle = dispositionFrameStyle(position, style);
	if (resolveTextMorphRoot(embodiment) === "heading-block") {
		return (
			<div
				className={[
					"presentation-disposition-frame",
					"presentation-morph-slot",
					"presentation-morph-slot--label",
					emphasisClass(emphasis),
				]
					.filter(Boolean)
					.join(" ")}
				style={frameStyle}
			>
				<TextMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />
			</div>
		);
	}
	const headingClass = centeredLineClass(anchorId, embodiment, emphasis);
	return (
		<div
			data-id={anchorId}
			className={[
				"presentation-disposition-frame",
				"presentation-morph-slot",
				"presentation-morph-slot--label",
				emphasisClass(emphasis),
			]
				.filter(Boolean)
				.join(" ")}
			style={frameStyle}
		>
			<h2 className={headingClass}>{embodiment.lines[0]}</h2>
		</div>
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
	if (disposition.split?.morphParticipant) {
		const boundingFrame = splitTilesBoundingFrame(tiles);
		if (boundingFrame) {
			const sourceCrop = splitTilesUnionSourceCrop(tiles);
			const cropEmbodiment: FigureEmbodiment = embodiment.crop
				? embodiment
				: { ...embodiment, crop: sourceCrop };
			return (
				<>
					<FigureMorphView
						morphId={disposition.morphId}
						embodiment={cropEmbodiment}
						emphasis={disposition.emphasis}
						position={boundingFrame}
						style={{ opacity: 0 }}
					/>
					{tiles.map((tile) => (
						<FigureTileView
							key={tile.key}
							participantId={disposition.participant.id}
							tile={tile}
							embodiment={embodiment}
							defaultEmphasis={disposition.emphasis}
							tileDuplicateHidden
						/>
					))}
				</>
			);
		}
	}
	const packedFrame = splitTilesPackedFrame(tiles);
	if (packedFrame) {
		return (
			<div className="presentation-figure-split-assembled">
				<DispositionFrame
					disposition={{ ...disposition, position: packedFrame, split: undefined }}
					overlay
				>
					<div className="presentation-figure-split-assembled-full">
						<FigureMorphView
							morphId={disposition.morphId}
							embodiment={embodiment}
							emphasis={disposition.emphasis}
							position={packedFrame}
						/>
					</div>
				</DispositionFrame>
				<div className="presentation-figure-split-assembled-tiles" aria-hidden>
					{tiles.map((tile) => (
						<FigureTileView
							key={tile.key}
							participantId={disposition.participant.id}
							tile={tile}
							embodiment={embodiment}
							defaultEmphasis={disposition.emphasis}
						/>
					))}
				</div>
			</div>
		);
	}
	return (
		<>
			{tiles.map((tile) => (
				<FigureTileView
					key={tile.key}
					participantId={disposition.participant.id}
					tile={tile}
					embodiment={embodiment}
					defaultEmphasis={disposition.emphasis}
				/>
			))}
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
				return (
					<PositionedTextMorphView
						morphId={anchorId}
						embodiment={embodiment}
						emphasis={emphasis}
						position={disposition.position}
						style={disposition.style}
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
			if (embodiment.crop && disposition.position !== undefined) {
				return (
					<FigureMorphView
						morphId={anchorId}
						embodiment={embodiment}
						emphasis={emphasis}
						position={disposition.position}
						style={disposition.style}
					/>
				);
			}
			content = <FigureMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={disposition.position} />;
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
		disposition.split === undefined &&
		!disposition.embodiment.crop;
	return (
		<DispositionFrame disposition={disposition} overlay={overlay}>
			{content}
		</DispositionFrame>
	);
}
//#endregion 🔖MorphView

//#region 🔖Interaction
const DISPOSITION_MIN_FRACTION = 0.02;
const POINTER_DRAG_THRESHOLD_PX = 3;

/** @emoji 📐 Ephemeral slide-space rectangle for interactive dispositions (normalized 0..1). */
export type DispositionTransform = DispositionPosition;

/** @emoji ↔️ Marquee selection mode: crossing (L→R partial overlap) vs window (R→L full containment). */
export type MarqueeSelectionRule = "crossing" | "window";

/** @emoji ⊡ Eight resize handles on a disposition frame. */
export type DispositionResizeHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

const DISPOSITION_RESIZE_HANDLES: readonly DispositionResizeHandle[] = [
	"nw",
	"n",
	"ne",
	"e",
	"se",
	"s",
	"sw",
	"w",
];

/** @emoji 🔑 Stable id for one resolved disposition on a slide. */
export function dispositionInteractionId(
	renderSlideId: string,
	disposition: ResolvedDisposition,
	index: number,
): string {
	return `${renderSlideId}--${disposition.morphId}--${disposition.embodimentId ?? index}`;
}

/** @emoji 📐 True when two normalized rectangles overlap with positive area. */
export function rectsIntersect(a: DispositionPosition, b: DispositionPosition): boolean {
	return a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y;
}

/** @emoji 📐 True when outer fully contains inner. */
export function rectContains(outer: DispositionPosition, inner: DispositionPosition): boolean {
	return (
		inner.x >= outer.x &&
		inner.y >= outer.y &&
		inner.x + inner.width <= outer.x + outer.width &&
		inner.y + inner.height <= outer.y + outer.height
	);
}

/** @emoji ⊞ Normalized marquee rectangle from two pointer fractions. */
export function normalizeMarquee(
	start: { readonly x: number; readonly y: number },
	end: { readonly x: number; readonly y: number },
): DispositionPosition {
	const x = Math.min(start.x, end.x);
	const y = Math.min(start.y, end.y);
	return {
		x,
		y,
		width: Math.abs(end.x - start.x),
		height: Math.abs(end.y - start.y),
	};
}

/** @emoji ↔️ Crossing when dragged left-to-right (end.x >= start.x), else window. */
export function marqueeSelectionRule(
	start: { readonly x: number; readonly y: number },
	end: { readonly x: number; readonly y: number },
): MarqueeSelectionRule {
	return end.x >= start.x ? "crossing" : "window";
}

/** @emoji 🎯 Whether a marquee selects a target rect under crossing or window rules. */
export function marqueeSelects(
	marquee: DispositionPosition,
	target: DispositionPosition,
	rule: MarqueeSelectionRule,
): boolean {
	if (marquee.width <= 0 || marquee.height <= 0) {
		return false;
	}
	return rule === "crossing" ? rectsIntersect(marquee, target) : rectContains(marquee, target);
}

function clampFraction(value: number): number {
	return Math.max(0, Math.min(1, value));
}

/** @emoji ↔️ Moves a normalized rect by fractional deltas. */
export function translateDispositionRect(
	rect: DispositionPosition,
	dx: number,
	dy: number,
): DispositionPosition {
	const width = rect.width;
	const height = rect.height;
	const x = clampFraction(rect.x + dx);
	const y = clampFraction(rect.y + dy);
	const maxX = 1 - width;
	const maxY = 1 - height;
	return {
		x: Math.min(x, maxX),
		y: Math.min(y, maxY),
		width,
		height,
	};
}

/** @emoji ⊡ Resizes a normalized rect from one handle by fractional deltas. */
export function resizeDispositionRect(
	rect: DispositionPosition,
	handle: DispositionResizeHandle,
	dx: number,
	dy: number,
	minSize: number = DISPOSITION_MIN_FRACTION,
): DispositionPosition {
	let { x, y, width, height } = rect;
	if (handle.includes("e")) {
		width += dx;
	}
	if (handle.includes("w")) {
		x += dx;
		width -= dx;
	}
	if (handle.includes("s")) {
		height += dy;
	}
	if (handle.includes("n")) {
		y += dy;
		height -= dy;
	}
	width = Math.max(minSize, width);
	height = Math.max(minSize, height);
	x = clampFraction(x);
	y = clampFraction(y);
	if (x + width > 1) {
		width = 1 - x;
	}
	if (y + height > 1) {
		height = 1 - y;
	}
	if (width < minSize) {
		width = minSize;
	}
	if (height < minSize) {
		height = minSize;
	}
	return { x, y, width, height };
}

/** @emoji ⊞ Union bounding box of normalized rectangles. */
export function groupBoundingRect(rects: readonly DispositionPosition[]): DispositionPosition | null {
	if (rects.length === 0) {
		return null;
	}
	return unionDispositionPositions(rects);
}

/** @emoji ⊞ Scales one member rect when a group bounding box is resized. */
export function scaleRectWithinGroup(
	rect: DispositionPosition,
	oldGroup: DispositionPosition,
	newGroup: DispositionPosition,
): DispositionPosition {
	if (oldGroup.width <= 0 || oldGroup.height <= 0) {
		return rect;
	}
	const relX = (rect.x - oldGroup.x) / oldGroup.width;
	const relY = (rect.y - oldGroup.y) / oldGroup.height;
	const relW = rect.width / oldGroup.width;
	const relH = rect.height / oldGroup.height;
	return {
		x: newGroup.x + relX * newGroup.width,
		y: newGroup.y + relY * newGroup.height,
		width: relW * newGroup.width,
		height: relH * newGroup.height,
	};
}

/** @emoji ⛶ Toggles slide-fullscreen rect vs stashed pre-fullscreen rect. */
export function toggleFullscreenRect(
	current: DispositionPosition,
	stash: DispositionPosition | undefined,
): { readonly rect: DispositionPosition; readonly stash: DispositionPosition | undefined } {
	if (stash !== undefined) {
		return { rect: stash, stash: undefined };
	}
	return { rect: { x: 0, y: 0, width: 1, height: 1 }, stash: current };
}

/** @emoji 📍 Maps client coordinates to normalized fractions inside a section element. */
export function clientToSectionFraction(
	sectionEl: HTMLElement,
	clientX: number,
	clientY: number,
): { readonly x: number; readonly y: number } {
	const bounds = sectionEl.getBoundingClientRect();
	if (bounds.width <= 0 || bounds.height <= 0) {
		return { x: 0, y: 0 };
	}
	return {
		x: clampFraction((clientX - bounds.left) / bounds.width),
		y: clampFraction((clientY - bounds.top) / bounds.height),
	};
}

/** @emoji 📍 Maps an element's client rect to normalized fractions inside a section. */
export function measureElementRectInSection(
	element: HTMLElement,
	sectionEl: HTMLElement,
): DispositionPosition | null {
	const sectionBounds = sectionEl.getBoundingClientRect();
	if (sectionBounds.width <= 0 || sectionBounds.height <= 0) {
		return null;
	}
	const rect = element.getBoundingClientRect();
	return {
		x: (rect.left - sectionBounds.left) / sectionBounds.width,
		y: (rect.top - sectionBounds.top) / sectionBounds.height,
		width: rect.width / sectionBounds.width,
		height: rect.height / sectionBounds.height,
	};
}

/** @emoji 📐 Declared placement for one resolved disposition (tiles union when split). */
export function declaredDispositionRect(disposition: ResolvedDisposition): DispositionPosition | undefined {
	if (disposition.style?.opacity === 0) {
		return undefined;
	}
	if (disposition.split?.tiles?.length) {
		return splitTilesBoundingFrame(disposition.split.tiles) ?? undefined;
	}
	return disposition.position;
}

function transformFrameStyle(transform: DispositionPosition): CSSProperties {
	return {
		position: "absolute",
		left: `${transform.x * 100}%`,
		top: `${transform.y * 100}%`,
		width: `${transform.width * 100}%`,
		height: `${transform.height * 100}%`,
		boxSizing: "border-box",
	};
}

interface PresentationInteractionState {
	readonly selectedIds: ReadonlySet<string>;
	readonly transforms: ReadonlyMap<string, DispositionTransform>;
	readonly fullscreenIds: ReadonlySet<string>;
	readonly isSelected: (id: string) => boolean;
	readonly isFullscreen: (id: string) => boolean;
	readonly getTransform: (id: string) => DispositionTransform | undefined;
	readonly setTransform: (id: string, rect: DispositionTransform) => void;
	readonly setTransforms: (updates: ReadonlyMap<string, DispositionTransform>) => void;
	readonly selectIds: (ids: readonly string[], additive: boolean) => void;
	readonly clearSelection: () => void;
	readonly toggleFullscreen: (id: string) => void;
}

const PresentationInteractionContext = createContext<PresentationInteractionState | null>(null);

function usePresentationInteractionState(): PresentationInteractionState {
	const value = useContext(PresentationInteractionContext);
	if (!value) {
		throw new Error("Presentation interaction requires PresentationInteractionContext.");
	}
	return value;
}

/** @emoji 🖱 Ephemeral selection, transforms, and slide-fullscreen flags; resets when slideEpoch changes. */
export function usePresentationInteraction(slideEpoch: number): PresentationInteractionState {
	const [selectedIds, setSelectedIds] = useState<ReadonlySet<string>>(() => new Set());
	const [transforms, setTransforms] = useState<ReadonlyMap<string, DispositionTransform>>(() => new Map());
	const [fullscreenIds, setFullscreenIds] = useState<ReadonlySet<string>>(() => new Set());

	useEffect(() => {
		setSelectedIds(new Set());
		setTransforms(new Map());
		setFullscreenIds(new Set());
	}, [slideEpoch]);

	const isSelected = useCallback((id: string) => selectedIds.has(id), [selectedIds]);

	const isFullscreen = useCallback((id: string) => fullscreenIds.has(id), [fullscreenIds]);

	const getTransform = useCallback((id: string) => transforms.get(id), [transforms]);

	const setTransform = useCallback((id: string, rect: DispositionTransform) => {
		setTransforms((previous) => {
			const next = new Map(previous);
			next.set(id, rect);
			return next;
		});
	}, []);

	const setTransformsBatch = useCallback((updates: ReadonlyMap<string, DispositionTransform>) => {
		setTransforms((previous) => {
			const next = new Map(previous);
			for (const [id, rect] of updates) {
				next.set(id, rect);
			}
			return next;
		});
	}, []);

	const selectIds = useCallback((ids: readonly string[], additive: boolean) => {
		setSelectedIds((previous) => {
			if (!additive) {
				return new Set(ids);
			}
			const next = new Set(previous);
			for (const id of ids) {
				if (next.has(id)) {
					next.delete(id);
				} else {
					next.add(id);
				}
			}
			return next;
		});
	}, []);

	const clearSelection = useCallback(() => {
		setSelectedIds(new Set());
		setFullscreenIds(new Set());
	}, []);

	const toggleFullscreen = useCallback((id: string) => {
		setFullscreenIds((previous) => {
			const next = new Set(previous);
			if (next.has(id)) {
				next.delete(id);
			} else {
				next.add(id);
			}
			return next;
		});
	}, []);

	return useMemo(
		() => ({
			selectedIds,
			transforms,
			fullscreenIds,
			isSelected,
			isFullscreen,
			getTransform,
			setTransform,
			setTransforms: setTransformsBatch,
			selectIds,
			clearSelection,
			toggleFullscreen,
		}),
		[
			selectedIds,
			transforms,
			fullscreenIds,
			isSelected,
			isFullscreen,
			getTransform,
			setTransform,
			setTransformsBatch,
			selectIds,
			clearSelection,
			toggleFullscreen,
		],
	);
}

interface SlideDispositionRegistry {
	readonly registerRect: (id: string, rect: DispositionPosition | null) => void;
	readonly getRect: (id: string) => DispositionPosition | undefined;
	readonly listEntries: () => readonly { readonly id: string; readonly rect: DispositionPosition }[];
}

const SlideDispositionRegistryContext = createContext<SlideDispositionRegistry | null>(null);

function useSlideDispositionRegistry(): SlideDispositionRegistry {
	const value = useContext(SlideDispositionRegistryContext);
	if (!value) {
		throw new Error("Slide disposition registry requires SlideDispositionRegistryContext.");
	}
	return value;
}

function SlideDispositionRegistryProvider({
	children,
}: {
	readonly children: ReactNode;
}): ReactNode {
	const measuredRectsRef = useRef<Map<string, DispositionPosition>>(new Map());
	const [, bump] = useState(0);

	const registerRect = useCallback((id: string, rect: DispositionPosition | null) => {
		const map = measuredRectsRef.current;
		if (rect === null) {
			if (map.delete(id)) {
				bump((value) => value + 1);
			}
			return;
		}
		const existing = map.get(id);
		if (
			existing &&
			Math.abs(existing.x - rect.x) < 1e-5 &&
			Math.abs(existing.y - rect.y) < 1e-5 &&
			Math.abs(existing.width - rect.width) < 1e-5 &&
			Math.abs(existing.height - rect.height) < 1e-5
		) {
			return;
		}
		map.set(id, rect);
		bump((value) => value + 1);
	}, []);

	const getRect = useCallback((id: string) => measuredRectsRef.current.get(id), []);

	const listEntries = useCallback(() => {
		return [...measuredRectsRef.current.entries()].map(([id, rect]) => ({ id, rect }));
	}, []);

	const registry = useMemo(
		() => ({
			registerRect,
			getRect,
			listEntries,
		}),
		[registerRect, getRect, listEntries],
	);

	return (
		<SlideDispositionRegistryContext.Provider value={registry}>{children}</SlideDispositionRegistryContext.Provider>
	);
}

function resolveEffectiveDispositionRect(
	id: string,
	declared: DispositionPosition | undefined,
	interaction: PresentationInteractionState,
	registry: SlideDispositionRegistry,
): DispositionPosition | undefined {
	const transform = interaction.getTransform(id);
	if (transform) {
		return transform;
	}
	if (declared) {
		return declared;
	}
	return registry.getRect(id);
}

const InteractiveDisposition: FC<{
	readonly id: string;
	readonly disposition: ResolvedDisposition;
	readonly declaredRect: DispositionPosition | undefined;
	readonly allDeclaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
	readonly sectionRef: RefObject<HTMLElement | null>;
	readonly children: ReactNode;
}> = ({ id, disposition, declaredRect, allDeclaredRects, sectionRef, children }) => {
	const interaction = usePresentationInteractionState();
	const registry = useSlideDispositionRegistry();
	const rootRef = useRef<HTMLDivElement>(null);
	const selected = interaction.isSelected(id);
	const transform = interaction.getTransform(id);
	const pinned = transform !== undefined;
	const fullscreen = interaction.isFullscreen(id);

	const effectiveRect = resolveEffectiveDispositionRect(id, declaredRect, interaction, registry);

	useLayoutEffect(() => {
		if (declaredRect || transform) {
			registry.registerRect(id, null);
			return;
		}
		const root = rootRef.current;
		const section = sectionRef.current;
		if (!root || !section) {
			return;
		}
		const measured = measureElementRectInSection(root, section);
		registry.registerRect(id, measured);
	}, [id, declaredRect, transform, registry, sectionRef]);

	const ensureRectForManipulation = useCallback((): DispositionPosition | null => {
		const existing = resolveEffectiveDispositionRect(id, declaredRect, interaction, registry);
		if (existing) {
			return existing;
		}
		const root = rootRef.current;
		const section = sectionRef.current;
		if (!root || !section) {
			return null;
		}
		const measured = measureElementRectInSection(root, section);
		if (!measured) {
			return null;
		}
		interaction.setTransform(id, measured);
		return measured;
	}, [id, declaredRect, interaction, registry, sectionRef]);

	const runPointerGesture = useCallback(
		(
			event: React.PointerEvent,
			mode: "move" | DispositionResizeHandle,
			initialRect: DispositionPosition,
		) => {
			event.preventDefault();
			event.stopPropagation();
			const section = sectionRef.current;
			if (!section) {
				return;
			}
			const pointerId = event.pointerId;
			const startClient = { x: event.clientX, y: event.clientY };
			const startFraction = clientToSectionFraction(section, startClient.x, startClient.y);
			const selectedAtStart = [...interaction.selectedIds];
			const groupIds =
				selectedAtStart.includes(id) && selectedAtStart.length > 1 ? selectedAtStart : [id];
			const startRects = new Map<string, DispositionPosition>();
			for (const memberId of groupIds) {
				const rect =
					interaction.getTransform(memberId) ??
					allDeclaredRects.get(memberId) ??
					(memberId === id ? initialRect : registry.getRect(memberId));
				if (rect) {
					startRects.set(memberId, rect);
				}
			}
			const startGroup =
				mode !== "move" && groupIds.length > 1
					? groupBoundingRect([...startRects.values()])
					: null;

			const onMove = (moveEvent: PointerEvent): void => {
				const current = clientToSectionFraction(section, moveEvent.clientX, moveEvent.clientY);
				const dx = current.x - startFraction.x;
				const dy = current.y - startFraction.y;
				const updates = new Map<string, DispositionTransform>();
				if (mode === "move") {
					for (const [memberId, rect] of startRects) {
						updates.set(memberId, translateDispositionRect(rect, dx, dy));
					}
				} else if (startGroup && groupIds.length > 1) {
					const resizedGroup = resizeDispositionRect(startGroup, mode, dx, dy);
					for (const [memberId, rect] of startRects) {
						updates.set(memberId, scaleRectWithinGroup(rect, startGroup, resizedGroup));
					}
				} else {
					updates.set(id, resizeDispositionRect(initialRect, mode, dx, dy));
				}
				interaction.setTransforms(updates);
			};

			const onUp = (): void => {
				window.removeEventListener("pointermove", onMove);
				window.removeEventListener("pointerup", onUp);
				window.removeEventListener("pointercancel", onUp);
				try {
					(event.target as HTMLElement | null)?.releasePointerCapture?.(pointerId);
				} catch {
					// jsdom may not support pointer capture
				}
			};

			(event.target as HTMLElement).setPointerCapture?.(pointerId);
			window.addEventListener("pointermove", onMove);
			window.addEventListener("pointerup", onUp);
			window.addEventListener("pointercancel", onUp);
		},
		[id, declaredRect, allDeclaredRects, interaction, registry, sectionRef],
	);

	const onPointerDown = (event: React.PointerEvent): void => {
		if ((event.target as HTMLElement).closest(".presentation-interaction-handle")) {
			return;
		}
		if ((event.target as HTMLElement).closest(".presentation-interaction-fullscreen")) {
			return;
		}
		event.stopPropagation();
		const additive = event.shiftKey;
		if (!selected) {
			interaction.selectIds([id], additive);
		} else if (additive) {
			interaction.selectIds([id], true);
		}
		const rect = ensureRectForManipulation();
		if (!rect) {
			return;
		}
		const startClient = { x: event.clientX, y: event.clientY };
		let dragged = false;
		const onMove = (moveEvent: PointerEvent): void => {
			if (
				!dragged &&
				Math.hypot(moveEvent.clientX - startClient.x, moveEvent.clientY - startClient.y) >=
					POINTER_DRAG_THRESHOLD_PX
			) {
				dragged = true;
				runPointerGesture(event, "move", rect);
			}
		};
		const onUp = (): void => {
			window.removeEventListener("pointermove", onMove);
			window.removeEventListener("pointerup", onUp);
		};
		window.addEventListener("pointermove", onMove);
		window.addEventListener("pointerup", onUp);
	};

	const onHandlePointerDown = (handle: DispositionResizeHandle) => (event: React.PointerEvent) => {
		event.stopPropagation();
		if (!selected) {
			interaction.selectIds([id], false);
		}
		const rect = ensureRectForManipulation();
		if (!rect) {
			return;
		}
		runPointerGesture(event, handle, rect);
	};

	const onFullscreenClick = (event: React.MouseEvent): void => {
		event.preventDefault();
		event.stopPropagation();
		if (!selected) {
			interaction.selectIds([id], false);
		}
		interaction.toggleFullscreen(id);
	};

	const wrapperClass = [
		"presentation-interactive-disposition",
		`presentation-interactive-disposition--kind-${disposition.embodiment.kind}`,
		selected ? "presentation-interactive-disposition--selected" : undefined,
		pinned ? "presentation-interactive-disposition--pinned" : undefined,
		fullscreen ? "presentation-interactive-disposition--fullscreen" : undefined,
	]
		.filter(Boolean)
		.join(" ");

	const wrapperStyle: CSSProperties | undefined = pinned && transform ? transformFrameStyle(transform) : undefined;

	return (
		<div
			ref={rootRef}
			data-disposition-id={id}
			className={wrapperClass}
			style={wrapperStyle}
			onPointerDown={onPointerDown}
		>
			<div className="presentation-interactive-disposition__content">{children}</div>
			{selected && effectiveRect ? (
				<>
					<div className="presentation-interactive-disposition__chrome" aria-hidden>
						{DISPOSITION_RESIZE_HANDLES.map((handle) => (
							<div
								key={handle}
								className={`presentation-interaction-handle presentation-interaction-handle--${handle}`}
								onPointerDown={onHandlePointerDown(handle)}
							/>
						))}
					</div>
					<button
						type="button"
						className="presentation-interaction-fullscreen"
						title={fullscreen ? "Exit slide fullscreen" : "Slide fullscreen"}
						aria-pressed={fullscreen}
						onClick={onFullscreenClick}
					>
						{fullscreen ? "⤢" : "⤢"}
					</button>
				</>
			) : null}
		</div>
	);
};

const InteractionLayer: FC<{
	readonly sectionRef: RefObject<HTMLElement | null>;
	readonly dispositionIds: readonly string[];
	readonly declaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
}> = ({ sectionRef, dispositionIds, declaredRects }) => {
	const interaction = usePresentationInteractionState();
	const registry = useSlideDispositionRegistry();
	const [marquee, setMarquee] = useState<{
		readonly start: { readonly x: number; readonly y: number };
		readonly end: { readonly x: number; readonly y: number };
	} | null>(null);

	const resolveRectForId = useCallback(
		(targetId: string): DispositionPosition | undefined => {
			return resolveEffectiveDispositionRect(
				targetId,
				declaredRects.get(targetId),
				interaction,
				registry,
			);
		},
		[declaredRects, interaction, registry],
	);

	const onPointerDown = (event: React.PointerEvent): void => {
		if (event.button !== 0) {
			return;
		}
		const section = sectionRef.current;
		if (!section) {
			return;
		}
		const fraction = clientToSectionFraction(section, event.clientX, event.clientY);
		const start = { x: fraction.x, y: fraction.y };
		let moved = false;

		const onMove = (moveEvent: PointerEvent): void => {
			if (
				!moved &&
				Math.hypot(moveEvent.clientX - event.clientX, moveEvent.clientY - event.clientY) <
					POINTER_DRAG_THRESHOLD_PX
			) {
				return;
			}
			moved = true;
			const current = clientToSectionFraction(section, moveEvent.clientX, moveEvent.clientY);
			setMarquee({ start, end: { x: current.x, y: current.y } });
		};

		const onUp = (upEvent: PointerEvent): void => {
			window.removeEventListener("pointermove", onMove);
			window.removeEventListener("pointerup", onUp);
			setMarquee(null);
			if (!moved) {
				interaction.clearSelection();
				return;
			}
			const end = clientToSectionFraction(section, upEvent.clientX, upEvent.clientY);
			const box = normalizeMarquee(start, end);
			const rule = marqueeSelectionRule(start, end);
			const hits: string[] = [];
			for (const targetId of dispositionIds) {
				const rect = resolveRectForId(targetId);
				if (rect && marqueeSelects(box, rect, rule)) {
					hits.push(targetId);
				}
			}
			interaction.selectIds(hits, upEvent.shiftKey);
		};

		window.addEventListener("pointermove", onMove);
		window.addEventListener("pointerup", onUp);
	};

	const marqueeStyle: CSSProperties | undefined = marquee
		? (() => {
				const box = normalizeMarquee(marquee.start, marquee.end);
				const rule = marqueeSelectionRule(marquee.start, marquee.end);
				return {
					left: `${box.x * 100}%`,
					top: `${box.y * 100}%`,
					width: `${box.width * 100}%`,
					height: `${box.height * 100}%`,
					...(rule === "crossing"
						? { className: "presentation-interaction-marquee presentation-interaction-marquee--crossing" }
						: {}),
				};
			})()
		: undefined;

	const marqueeRule =
		marquee === null ? null : marqueeSelectionRule(marquee.start, marquee.end);

	return (
		<div className="presentation-interaction-layer" onPointerDown={onPointerDown}>
			{marquee && marqueeStyle ? (
				<div
					className={[
						"presentation-interaction-marquee",
						marqueeRule === "crossing"
							? "presentation-interaction-marquee--crossing"
							: "presentation-interaction-marquee--window",
					].join(" ")}
					style={{
						left: marqueeStyle.left,
						top: marqueeStyle.top,
						width: marqueeStyle.width,
						height: marqueeStyle.height,
					}}
				/>
			) : null}
		</div>
	);
};
//#endregion 🔖Interaction

//#region 🔖ArrangementSection
const ArrangementSection: FC<{
	readonly thought: Thought;
	readonly renderSlide: RenderSlide;
}> = ({ thought, renderSlide }) => {
	const sectionRef = useRef<HTMLElement>(null);
	const resolved = resolveArrangement(thought.participants, renderSlide.arrangement);
	const morph = renderSlide.autoAnimateId !== undefined;
	const positioned = resolved.some((disposition) => disposition.position !== undefined || disposition.split !== undefined);
	const layoutResolved = positioned ? centerResolvedArrangement(resolved) : resolved;
	const dispositionMeta = useMemo(
		() =>
			layoutResolved.map((disposition, index) => ({
				id: dispositionInteractionId(renderSlide.id, disposition, index),
				disposition,
				declaredRect: declaredDispositionRect(disposition),
			})),
		[layoutResolved, renderSlide.id],
	);
	const declaredRects = useMemo(() => {
		const map = new Map<string, DispositionPosition | undefined>();
		for (const entry of dispositionMeta) {
			map.set(entry.id, entry.declaredRect);
		}
		return map;
	}, [dispositionMeta]);
	const placements = dispositionMeta.map((entry) => (
		<InteractiveDisposition
			key={entry.id}
			id={entry.id}
			disposition={entry.disposition}
			declaredRect={entry.declaredRect}
			allDeclaredRects={declaredRects}
			sectionRef={sectionRef}
		>
			<MorphDispositionView disposition={entry.disposition} />
		</InteractiveDisposition>
	));
	return (
		<SlideDispositionRegistryProvider>
			<section
				ref={sectionRef}
				{...(morph ? { "data-auto-animate": "", "data-auto-animate-id": renderSlide.autoAnimateId } : {})}
				{...(renderSlide.arrangement.settleBeforeMorphTo?.length
					? { "data-settle-before-morph-to": renderSlide.arrangement.settleBeforeMorphTo.join(",") }
					: {})}
				title={renderSlide.id}
				className={[
					"presentation-arrangement--interactive",
					positioned ? "presentation-arrangement--positioned" : undefined,
				]
					.filter(Boolean)
					.join(" ")}
			>
				<InteractionLayer
					sectionRef={sectionRef}
					dispositionIds={dispositionMeta.map((entry) => entry.id)}
					declaredRects={declaredRects}
				/>
				{positioned ? <div className="presentation-arrangement-canvas">{placements}</div> : placements}
			</section>
		</SlideDispositionRegistryProvider>
	);
};
//#endregion 🔖ArrangementSection

//#region 🔖PresentationInteractionProvider
const PresentationInteractionProvider: FC<{
	readonly slideEpoch: number;
	readonly children: ReactNode;
}> = ({ slideEpoch, children }) => {
	const interaction = usePresentationInteraction(slideEpoch);
	return (
		<PresentationInteractionContext.Provider value={interaction}>{children}</PresentationInteractionContext.Provider>
	);
};
//#endregion 🔖PresentationInteractionProvider

//#region 🔖PresentationDeck
/** @emoji 🎞 Maps a {@link Presentation} to reveal.js DOM. */
export const PresentationDeck: FC<{
	readonly presentation: Presentation;
	readonly options?: PresentationMountOptions;
}> = ({ presentation, options }) => {
	const deckDivRef = useRef<HTMLDivElement>(null);
	const deckRef = useRef<Reveal.Api | null>(null);
	const previousSlideRef = useRef<HTMLElement | null>(null);
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
			autoAnimateMatcher: presentationAutoAnimateMatcher,
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
		const onSlideChanged = (event: Event): void => {
			relaxHiddenPreflight();
			syncRevealBackgroundKind(deckEl);
			syncPresentationSlideSizeVars(deckEl, deck);
			syncPresentSlideMedia();
			syncSlideUrl();
			const slideEvent = event as Event & {
				previousSlide?: HTMLElement;
				currentSlide?: HTMLElement;
			};
			const currentSlide = slideEvent.currentSlide ?? deck.getCurrentSlide();
			const previousSlide = slideEvent.previousSlide ?? previousSlideRef.current;
			if (previousSlide && currentSlide) {
				prepareArrangementBeforeAutoAnimate(previousSlide, currentSlide);
			}
			syncArrangementSettledState(deckEl, currentSlide, previousSlide);
			previousSlideRef.current = currentSlide;
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
		const onAutoAnimate = (event: Event): void => {
			const sheet = (event as Event & { data?: { sheet?: { innerHTML: string } } }).data?.sheet;
			if (sheet && typeof sheet.innerHTML === "string") {
				sheet.innerHTML = patchAutoAnimateUniformScale(sheet.innerHTML);
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
					const currentSlide = deck.getCurrentSlide();
					syncArrangementSettledState(deckEl, currentSlide, previousSlideRef.current);
					previousSlideRef.current = currentSlide;
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
				const currentSlide = deck.getCurrentSlide();
				syncArrangementSettledState(deckEl, currentSlide, previousSlideRef.current);
				previousSlideRef.current = currentSlide;
				setSlideEpoch((epoch) => epoch + 1);
			}
			deck.on("slidechanged", onSlideChanged);
			deck.on("resize", onResize);
			deck.on("autoanimate", onAutoAnimate);
		});
		return () => {
			window.removeEventListener("hashchange", onWindowHashChange);
			deck.off("slidechanged", onSlideChanged);
			deck.off("resize", onResize);
			deck.off("autoanimate", onAutoAnimate);
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
			<PresentationInteractionProvider slideEpoch={slideEpoch}>
				<div className="reveal" ref={deckDivRef} style={{ width: "100vw", height: "100vh" }}>
					<div className="slides">
						{presentation.chapters.flatMap((chapter) =>
							chapter.sequences.map((sequence) => (
								<section key={`${chapter.id}-${sequence.id}`}>
									{sequence.thoughts.flatMap((thought) =>
										expandThoughtSlides(thought).map((renderSlide) => (
											<ArrangementSection
												key={`${chapter.id}-${sequence.id}-${thought.id}-${renderSlide.id}`}
												thought={thought}
												renderSlide={renderSlide}
											/>
										)),
									)}
								</section>
							)),
						)}
					</div>
				</div>
			</PresentationInteractionProvider>
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

		it("renders expanded intro slides", () => {
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
			expect(sections.length).toBe(7);
			expect(sections[0]?.hasAttribute("data-auto-animate")).toBe(true);
			const revealEl = container.querySelector(".reveal");
			expect(revealEl?.getAttribute("style")).toContain("100vw");
			expect(container.querySelector('[data-id^="title"]')).toBeTruthy();
			expect(container.querySelector('[data-id^="description"]')).toBeTruthy();
			expect(container.querySelector('[data-id="goal"]')).toBeTruthy();
			expect(container.querySelector('[data-id^="authors--"]')).toBeTruthy();
			expect(container.querySelector('[data-id^="institutions--"]')).toBeTruthy();
		});

		it("keeps intro flow slides centered while enabling interactive dispositions", () => {
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
			for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="introduction--m0"]')) {
				expect(slide.classList.contains("presentation-arrangement--interactive")).toBe(true);
				expect(slide.classList.contains("presentation-arrangement--positioned")).toBe(false);
				expect(slide.querySelector(".presentation-arrangement-canvas")).toBeNull();
				expect(slide.querySelectorAll("[data-disposition-id]").length).toBeGreaterThan(0);
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
			expect(slide("authors")?.getAttribute("data-auto-animate-id")).toMatch(/^introduction--/);
			expect(slide("authors")?.querySelector(".presentation-intro-line")?.className).toContain("gap-x-");
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
				'.slides > section > section[data-auto-animate][data-auto-animate-id="introduction--m0"]',
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
										participants: [
											{ id: "clip", embodiments: [{ kind: "video", src: "/demo.mp4" }] },
											{ id: "doc", embodiments: [{ kind: "pdf", src: "/paper.pdf", page: 1 }] },
										],
										slides: [
											{
												arrangement: {
													id: "slide",
													dispositions: [
														{ participantId: "clip", emphasis: "active" },
														{ participantId: "doc", emphasis: "active" },
													],
												},
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
										slides: [
											{
												arrangement: {
													id: "placed",
													dispositions: [
														{
															participantId: "box",
															emphasis: "active",
															position: { x: 0.1, y: 0.2, width: 0.5, height: 0.3 },
														},
													],
												},
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
			expect(frame?.style.left).toBe("25%");
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
										participants: [
											{
												id: "catalogue",
												embodiments: [{ kind: "figure", src: "/catalogue.png", alt: "Catalogue" }],
											},
										],
										slides: [
											{
												arrangement: {
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
			expect(first.querySelector(".presentation-figure-crop-fill")?.getAttribute("style")).toContain("/catalogue.png");
			const fullFigure = container.querySelector(
				".presentation-figure-split-assembled-full .presentation-media-figure",
			) as HTMLImageElement | null;
			expect(fullFigure?.getAttribute("src")).toBe("/catalogue.png");
			const presentSection = container.querySelector("section.present");
			expect(presentSection?.querySelector(".presentation-figure-split-assembled-full")).not.toBeNull();
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
										participants: [
											{
												id: "catalogue",
												embodiments: [{ kind: "figure", src: "/catalogue.png" }],
											},
										],
										slides: [
											{
												arrangement: {
													id: "focus",
													dispositions: [
														{
															participantId: "catalogue",
															emphasis: "active",
															split: { tiles: [allTiles[0]!, allTiles[1]!] },
														},
													],
												},
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

		it("renders cropped figure dispositions with matching morph slot DOM", () => {
			const deck: Presentation = {
				id: "crop-figure",
				name: "Crop",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "crop",
										participants: [
											{
												id: "catalogue-col1",
												embodiments: [
													{
														kind: "figure",
														id: "crop",
														src: "/catalogue.png",
														crop: { x: 0, y: 0, width: 0.5, height: 1 },
													},
												],
											},
										],
										slides: [
											{
												arrangement: {
													id: "focus",
													dispositions: [
														{
															participantId: "catalogue-col1",
															embodimentId: "crop",
															emphasis: "active",
															position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
														},
													],
												},
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
			const slot = container.querySelector('[data-id="catalogue-col1"].presentation-morph-slot') as HTMLElement | null;
			const fill = slot?.querySelector(".presentation-figure-crop-fill") as HTMLElement | null;
			expect(fill?.style.backgroundImage).toContain("/catalogue.png");
			expect(slot?.querySelector("h2")).toBeNull();
			expect(slot?.style.left).toBe("35%");
			expect(fill?.style.getPropertyValue("--presentation-figure-bg-size")).toBe("200% 100%");
			expect(fill?.style.getPropertyValue("--presentation-figure-bg-position")).toBe("0% 0%");
		});

		it("rewrites non-uniform auto-animate scale() to a uniform zoom", () => {
			expect(patchAutoAnimateUniformScale("transform: translate(1px, 2px) scale(1.5, 2) !important;")).toBe(
				"transform: translate(1px, 2px) scale(2) !important;",
			);
		});

		it("uses larger morph background vars when the frame is wider than the crop", () => {
			const crop = { x: 0, y: 0, width: 0.5, height: 1 };
			const square = figureCropBackgroundVars(
				{ kind: "figure", src: "/catalogue.png", crop },
				crop,
				{ x: 0, y: 0, width: 0.5, height: 0.5 },
			);
			const wide = figureCropBackgroundVars(
				{ kind: "figure", src: "/catalogue.png", crop },
				crop,
				{ x: 0, y: 0, width: 1, height: 0.25 },
			);
			expect(square["--presentation-figure-bg-size-morph" as keyof typeof square]).toBe("400% 200%");
			expect(wide["--presentation-figure-bg-size-morph" as keyof typeof wide]).toBe("1600% 800%");
			expect(wide["--presentation-figure-bg-size" as keyof typeof wide]).toBe("1600% 800%");
			expect(square["--presentation-figure-bg-size" as keyof typeof square]).toBe("400% 200%");
		});

		it("matches auto-animate targets only by data-id", () => {
			const fromSlide = document.createElement("section");
			fromSlide.innerHTML =
				'<div data-id="catalogue-col1" class="presentation-morph-slot--figure"><h2>Rippendecke</h2></div>';
			const toSlide = document.createElement("section");
			toSlide.innerHTML =
				'<div data-id="catalogue-col1" class="presentation-morph-slot--label"><h2>Rippendecke</h2></div>';
			const host: AutoAnimateMatcherHost = {
				findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
					for (const element of fromScope.querySelectorAll<HTMLElement>(selector)) {
						const key = serializer(element);
						const toElement = toScope.querySelector<HTMLElement>(
							`${selector}[data-id="${element.getAttribute("data-id")}"]`,
						);
						if (toElement) {
							pairs.push({ from: element, to: toElement });
						}
					}
				},
			};
			const pairs = presentationAutoAnimateMatcher.call(host, fromSlide, toSlide);
			expect(pairs).toHaveLength(1);
			expect(pairs[0]?.from.getAttribute("data-id")).toBe("catalogue-col1");
			expect(pairs[0]?.from.classList.contains("presentation-morph-slot--figure")).toBe(true);
		});

		it("clears settled state when arriving on a slide and prepares it before morph to listed targets", () => {
			const deckEl = document.createElement("div");
			const focus = document.createElement("section");
			focus.setAttribute("title", "catalogue-focus");
			focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
			focus.classList.add("presentation-arrangement--positioned");
			deckEl.appendChild(focus);
			focus.classList.add("presentation-arrangement--settled");
			syncArrangementSettledState(deckEl, focus, null);
			expect(focus.classList.contains("presentation-arrangement--settled")).toBe(false);
			const labels = document.createElement("section");
			labels.setAttribute("title", "catalogue-labels");
			prepareArrangementBeforeAutoAnimate(focus, labels);
			expect(focus.classList.contains("presentation-arrangement--settled")).toBe(true);
		});

		it("marks zero-opacity crop morph slots dormant until the arrangement settles", () => {
			const deck: Presentation = {
				id: "dormant-crop",
				name: "Dormant",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "crop",
										participants: [
											{
												id: "catalogue-col1",
												embodiments: [
													{
														kind: "figure",
														id: "crop",
														src: "/catalogue.png",
														crop: { x: 0, y: 0, width: 0.5, height: 1 },
													},
												],
											},
										],
										slides: [
											{
												arrangement: {
													id: "focus",
													dispositions: [
														{
															participantId: "catalogue-col1",
															embodimentId: "crop",
															emphasis: "active",
															position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
															style: { opacity: 0 },
														},
													],
												},
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
			const slot = container.querySelector('[data-id="catalogue-col1"].presentation-morph-slot--dormant') as HTMLElement | null;
			expect(slot).toBeTruthy();
			expect(slot?.classList.contains("presentation-morph-slot--dormant")).toBe(true);
		});

		it("renders positioned labels with data-id on the morph slot frame", () => {
			const deck: Presentation = {
				id: "label-slot",
				name: "Labels",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "labels",
										participants: [
											{
												id: "catalogue-col1",
												embodiments: [
													{
														kind: "text",
														id: "label",
														lines: ["Rippendecke"],
														level: "heading",
														morphRoot: "heading-line",
													},
												],
											},
										],
										slides: [
											{
												arrangement: {
													id: "labels",
													dispositions: [
														{
															participantId: "catalogue-col1",
															embodimentId: "label",
															emphasis: "active",
															position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
														},
													],
												},
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
			const slot = container.querySelector('[data-id="catalogue-col1"].presentation-morph-slot--label');
			expect(slot?.querySelector("h2")?.textContent).toBe("Rippendecke");
			expect(slot?.querySelector("h2[data-id]")).toBeNull();
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
			for (const slide of collectPresentationSlides(deck)) {
				if (slide.slide === "Ziel") {
					syncPresentationSlideUrl(deck, { h: slide.h, v: slide.v });
					break;
				}
			}
			const params = new URLSearchParams(new URL(window.location.href).hash.split("?")[1] ?? "");
			expect(params.get("kapitel")).toBe("Hauptteil");
			expect(params.get("sequenz")).toBe("Einführung");
			expect(params.get("gedanke")).toBe("Einleitung");
			expect(params.get("folie")).toBe("Ziel");
			history.replaceState(null, "", "/deck");
		});

		it("writes chapter, sequence, thought, and slide bookmark params after the hash path", () => {
			history.replaceState(null, "", "/deck");
			const goalSlide = collectPresentationSlides(sampleDeck).find((slide) => slide.slide === "Goal");
			syncPresentationSlideUrl(sampleDeck, { h: goalSlide!.h, v: goalSlide!.v });
			const url = new URL(window.location.href);
			expect(url.search).toBe("");
			expect(url.hash).toContain("slide=Goal");
			history.replaceState(null, "", "/deck");
		});

		it("readPresentationSlideIndicesFromUrl ignores bookmark query params", () => {
			expect(readPresentationSlideIndicesFromUrl("#/1/3")).toEqual({ h: 1, v: 3 });
			expect(readPresentationSlideIndicesFromUrl("")).toEqual({ h: 0, v: 0 });
		});
	});

	describe("presentation interaction geometry", () => {
		it("detects intersection and containment", () => {
			const a = { x: 0.1, y: 0.1, width: 0.3, height: 0.3 };
			const b = { x: 0.25, y: 0.25, width: 0.3, height: 0.3 };
			const outer = { x: 0, y: 0, width: 1, height: 1 };
			expect(rectsIntersect(a, b)).toBe(true);
			expect(rectContains(outer, a)).toBe(true);
			expect(rectContains(a, b)).toBe(false);
		});

		it("applies crossing vs window marquee rules", () => {
			const inside = { x: 0.2, y: 0.2, width: 0.1, height: 0.1 };
			const partial = { x: 0.55, y: 0.55, width: 0.3, height: 0.3 };
			const crossingMarquee = normalizeMarquee({ x: 0.1, y: 0.1 }, { x: 0.5, y: 0.5 });
			const windowMarquee = normalizeMarquee({ x: 0.7, y: 0.7 }, { x: 0.1, y: 0.1 });
			expect(marqueeSelectionRule({ x: 0.1, y: 0.1 }, { x: 0.5, y: 0.5 })).toBe("crossing");
			expect(marqueeSelectionRule({ x: 0.7, y: 0.7 }, { x: 0.1, y: 0.1 })).toBe("window");
			expect(marqueeSelects(crossingMarquee, inside, "crossing")).toBe(true);
			expect(marqueeSelects(windowMarquee, inside, "window")).toBe(true);
			expect(marqueeSelects(windowMarquee, partial, "window")).toBe(false);
			expect(marqueeSelects(windowMarquee, partial, "crossing")).toBe(true);
		});

		it("translates and resizes with minimum size", () => {
			const rect = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
			const moved = translateDispositionRect(rect, 0.1, -0.05);
			expect(moved.x).toBeCloseTo(0.3);
			expect(moved.y).toBeCloseTo(0.25);
			const resized = resizeDispositionRect(rect, "se", 0.2, 0.1);
			expect(resized.width).toBeCloseTo(0.6);
			expect(resized.height).toBeCloseTo(0.3);
		});

		it("scales group members and toggles fullscreen", () => {
			const a = { x: 0.1, y: 0.2, width: 0.2, height: 0.2 };
			const b = { x: 0.5, y: 0.2, width: 0.2, height: 0.2 };
			const group = groupBoundingRect([a, b]);
			expect(group?.width).toBeCloseTo(0.6);
			const grown = { x: 0, y: 0.1, width: 0.8, height: 0.3 };
			const scaledA = scaleRectWithinGroup(a, group!, grown);
			expect(scaledA.x).toBeCloseTo(0);
			const full = toggleFullscreenRect(a, undefined);
			expect(full.rect).toEqual({ x: 0, y: 0, width: 1, height: 1 });
			expect(full.stash).toEqual(a);
			const restored = toggleFullscreenRect(full.rect, full.stash);
			expect(restored.rect).toEqual(a);
		});
	});

	describe("presentation interaction dom", () => {
		let container: HTMLDivElement;

		const positionedDeck: Presentation = {
			id: "interactive-dom",
			name: "Interactive DOM",
			chapters: [
				{
					id: "main",
					sequences: [
						{
							id: "main",
							thoughts: [
								{
									id: "placed",
									participants: [
										{
											id: "box",
											embodiments: [{ kind: "text", lines: ["Hello"], level: "body" }],
										},
									],
									slides: [
										{
											arrangement: {
												id: "placed",
												dispositions: [
													{
														participantId: "box",
														emphasis: "active",
														position: { x: 0.2, y: 0.3, width: 0.4, height: 0.2 },
													},
												],
											},
										},
									],
								},
							],
						},
					],
				},
			],
		};

		const pointerClick = (target: Element, clientX = 20, clientY = 20): void => {
			target.dispatchEvent(
				new PointerEvent("pointerdown", { bubbles: true, cancelable: true, button: 0, clientX, clientY }),
			);
			target.dispatchEvent(
				new PointerEvent("pointerup", { bubbles: true, cancelable: true, button: 0, clientX, clientY }),
			);
		};

		beforeEach(() => {
			container = document.createElement("div");
			document.body.appendChild(container);
		});

		afterEach(() => {
			unmountPresentation();
			container.remove();
		});

		it("renders data-disposition-id on every disposition", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			expect(container.querySelectorAll("[data-disposition-id]").length).toBe(1);
		});

		it("selects on click and deselects on empty slide click", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			expect(disposition.querySelector(".presentation-interaction-fullscreen")).toBeTruthy();
			act(() => {
				pointerClick(layer);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
		});

		it("toggles slide fullscreen without pinned transforms or inline placement", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			act(() => {
				pointerClick(disposition);
			});
			const fullscreen = disposition.querySelector(".presentation-interaction-fullscreen") as HTMLButtonElement;
			expect(fullscreen.getAttribute("aria-pressed")).toBe("false");
			act(() => {
				fullscreen.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(false);
			expect(disposition.classList.contains("presentation-interactive-disposition--fullscreen")).toBe(true);
			expect(disposition.style.left).toBe("");
			expect(disposition.querySelector(".presentation-disposition-frame")?.style.position).toBe("absolute");
			expect(fullscreen.getAttribute("aria-pressed")).toBe("true");
			act(() => {
				fullscreen.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--fullscreen")).toBe(false);
			expect(fullscreen.getAttribute("aria-pressed")).toBe("false");
		});
	});
}
//#endregion 🧪Tests
