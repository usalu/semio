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
					<DispositionFrame
						disposition={{ ...disposition, position: boundingFrame, split: undefined }}
						overlay
					>
						<FigureMorphView
							morphId={disposition.morphId}
							embodiment={cropEmbodiment}
							emphasis={disposition.emphasis}
							position={boundingFrame}
							style={{ opacity: 0 }}
						/>
					</DispositionFrame>
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

//#region 🔖ArrangementSection
const ArrangementSection: FC<{
	readonly thought: Thought;
	readonly renderSlide: RenderSlide;
}> = ({ thought, renderSlide }) => {
	const resolved = resolveArrangement(thought.participants, renderSlide.arrangement);
	const morph = renderSlide.autoAnimateId !== undefined;
	const positioned = resolved.some((disposition) => disposition.position !== undefined || disposition.split !== undefined);
	const layoutResolved = positioned ? centerResolvedArrangement(resolved) : resolved;
	const placements = layoutResolved.map((disposition, index) => (
		<MorphDispositionView
			key={`${renderSlide.id}-${disposition.morphId}-${disposition.embodimentId ?? index}`}
			disposition={disposition}
		/>
	));
	return (
		<section
			{...(morph ? { "data-auto-animate": "", "data-auto-animate-id": renderSlide.autoAnimateId } : {})}
			{...(renderSlide.arrangement.settleBeforeMorphTo?.length
				? { "data-settle-before-morph-to": renderSlide.arrangement.settleBeforeMorphTo.join(",") }
				: {})}
			title={renderSlide.id}
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

		it("renders expanded intro slides including morph bridges", () => {
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
			expect(sections.length).toBeGreaterThan(7);
			expect(sections[0]?.hasAttribute("data-auto-animate")).toBe(true);
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
			for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="introduction--m0"]')) {
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
			expect(morphSections.length).toBeGreaterThan(7);
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
}
//#endregion 🧪Tests
