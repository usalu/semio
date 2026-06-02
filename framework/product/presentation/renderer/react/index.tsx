// #region 🧲Header
/** @emoji 📽 React + reveal.js renderer for `@framework/presentation/core` declarative decks. */
// #endregion 🧲Header

// #region 🔌Adapters
import type {
    AffiliationEntry,
    AffiliationsEmbodiment,
    AuthorPerson,
    AuthorsEmbodiment,
    BulletEmbodiment,
    DispositionPosition,
    DispositionStyle,
    FigureEmbodiment,
    GhostKind,
    ParticipantEmphasis,
    PdfEmbodiment,
    Presentation,
    RenderSlide,
    Chapter,
    ResolvedDisposition,
    Sequence,
    TextEmbodiment,
    Thought,
    VideoEmbodiment
} from "@framework/presentation/core";
import {
    abbreviateAuthorFirstName,
    affiliationLineName,
    buildResolutionScope,
    centerResolvedArrangement,
    collectPresentationSlides,
    expandThoughtSlides,
    formatPresentationUrlHash,
    intro,
    parsePresentationSlideHash,
    presentationLanguage,
    presentationSlideAt,
    resolutionScopeForArrangement,
    resolveArrangement,
    resolveTextMorphRoot,
    remapSplitDispositions,
    split,
    splitFigureGrid,
    unionDispositionPositions
} from "@framework/presentation/core";
import {
    applyElementsSurfaceChrome,
    Expertise,
    type ElementsSurfaceChromeInput,
} from "@ui/react";
import {
    act,
    createContext,
    Fragment,
    useCallback,
    useContext,
    useEffect,
    useLayoutEffect,
    useMemo,
    useRef,
    useState,
    type CSSProperties,
    type FC,
    type ReactNode,
    type RefObject,
} from "react";
import { flushSync } from "react-dom";
import { createRoot, type Root } from "react-dom/client";
import { Document, Page, pdfjs } from "react-pdf";
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import "./globals.css";

pdfjs.GlobalWorkerOptions.workerSrc = new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url).toString();
// #endregion 🔌Adapters

export type {
    AffiliationEntry,
    AffiliationsEmbodiment,
    Arrangement,
    AuthorPerson,
    AuthorsEmbodiment,
    BulletEmbodiment, Chapter, Disposition,
    DispositionPosition,
    DispositionStyle,
    Embodiment,
    FigureEmbodiment,
    Participant,
    ParticipantEmphasis,
    PdfEmbodiment, Presentation,
    RenderSlide,
    ResolvedDisposition,
    Sequence,
    Slide,
    TextEmbodiment,
    Thought,
    Transition,
    VideoEmbodiment
} from "@framework/presentation/core";

export {
    analogy,
    buildResolutionScope,
    collectPresentationSlides,
    countArrangements,
    expandThoughtSlides,
    formatPresentationUrlHash,
    intro,
    morphId,
    parsePresentationSlideHash,
    PRESENTATION_CHAPTER_QUERY_PARAM,
    PRESENTATION_SEQUENCE_QUERY_PARAM,
    PRESENTATION_SLIDE_QUERY_PARAM,
    PRESENTATION_THOUGHT_QUERY_PARAM,
    presentationEntityBookmarkName,
    presentationLanguage,
    presentationSequences,
    presentationSlideAt,
    presentationSlideBookmarkParamKeys,
    resolutionScopeForArrangement,
    resolveArrangement,
    resolveEmbodiment,
    resolveTextMorphRoot,
    split,
    splitFigureGrid,
    tile,
    unionSourceCrops,
} from "@framework/presentation/core";
export type {
    MorphFromSlot,
    PresentationLanguageKind,
    PresentationSlideBookmark,
    PresentationSlideBookmarkParamKeys,
    PresentationSlideRef,
    RenderSlide,
    Slide,
    TextMorphRoot
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
	/** @emoji 🎞 Called once reveal.js finished initializing (tests and tooling). */
	readonly onRevealReady?: (api: Reveal.Api) => void;
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
	syncPresentationAutoAnimateDurationVar(deckEl, deck);
}

/** @emoji ⏱️ Syncs reveal auto-animate duration for morph ghost/target opacity fades. */
export function syncPresentationAutoAnimateDurationVar(deckEl: HTMLElement, deck: Reveal.Api | null): void {
	const durationSeconds =
		typeof deck?.getConfig().autoAnimateDuration === "number" ? deck.getConfig().autoAnimateDuration : 1;
	deckEl.style.setProperty("--presentation-auto-animate-duration", `${durationSeconds}s`);
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
	if (
		currentSlide?.classList.contains("presentation-arrangement--settled") &&
		currentSlide.getAttribute("data-auto-animate") !== "pending" &&
		currentSlide.getAttribute("data-auto-animate") !== "running"
	) {
		currentSlide.classList.remove("presentation-arrangement--settled");
	}
}

/** @emoji 🔗 Resolves the reveal.js slide section at stack indices `h` / `v`. */
export function resolveRevealSlideAt(
	deckEl: HTMLElement,
	indices: { readonly h: number; readonly v: number },
): HTMLElement | null {
	const horizontal = deckEl.querySelectorAll<HTMLElement>(".slides > section")[indices.h];
	if (!horizontal) {
		return null;
	}
	const vertical = horizontal.querySelectorAll<HTMLElement>("section");
	return vertical[indices.v] ?? horizontal;
}

/** @emoji 📖 Reads reveal.js `slidechanged` slide elements extended onto the event object. */
export function slideChangedEventSlides(event: Event): {
	readonly previousSlide: HTMLElement | undefined;
	readonly currentSlide: HTMLElement | undefined;
} {
	const slideEvent = event as Event & {
		readonly previousSlide?: HTMLElement;
		readonly currentSlide?: HTMLElement;
	};
	return {
		previousSlide: slideEvent.previousSlide,
		currentSlide: slideEvent.currentSlide,
	};
}

/** @emoji 🧹 Strips reveal.js FLIP `transform`/`transition` only; never `left`/`top`/`width`/`height` (React owns those on morph frames). */
export function clearRevealAutoAnimateInlineLayout(deckEl: HTMLElement): void {
	const flipProps = ["transform", "transition"] as const;
	for (const element of deckEl.querySelectorAll<HTMLElement>("[data-auto-animate-target]")) {
		for (const prop of flipProps) {
			element.style.removeProperty(prop);
		}
	}
}

/** @emoji ✅ Clears reveal `pending`/`running` on slides so morph-source/into rest CSS applies after FLIP completes. */
export function finalizeRevealAutoAnimateRestState(deckEl: HTMLElement): void {
	for (const slide of deckEl.querySelectorAll<HTMLElement>(
		'section[data-auto-animate="running"], section[data-auto-animate="pending"]',
	)) {
		slide.setAttribute("data-auto-animate", "");
	}
	clearRevealAutoAnimateInlineLayout(deckEl);
	for (const element of deckEl.querySelectorAll<HTMLElement>("[data-auto-animate-target]")) {
		delete element.dataset.autoAnimateTarget;
	}
	const presentSlide = deckEl.querySelector<HTMLElement>("section.present");
	if (
		presentSlide?.classList.contains("presentation-arrangement--settled") &&
		presentSlide.getAttribute("data-auto-animate") !== "pending" &&
		presentSlide.getAttribute("data-auto-animate") !== "running"
	) {
		presentSlide.classList.remove("presentation-arrangement--settled");
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
	void toSlide.offsetHeight;
	for (const ghost of toSlide.querySelectorAll<HTMLElement>(".presentation-target-ghost")) {
		void ghost.offsetHeight;
	}
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

function elementIsTargetGhostAnchor(element: HTMLElement): boolean {
	return (
		element.classList.contains("presentation-target-ghost") ||
		element.closest(".presentation-target-ghost") !== null
	);
}

function elementIsSourceGhostAnchor(element: HTMLElement): boolean {
	return (
		element.classList.contains("presentation-source-ghost") ||
		element.closest(".presentation-source-ghost") !== null
	);
}

function elementIsMorphOneAnchor(element: HTMLElement): boolean {
	return (
		element.classList.contains("presentation-morph-one") ||
		element.closest(".presentation-morph-one") !== null
	);
}

function elementIsFigureMorphSlot(element: HTMLElement): boolean {
	return (
		element.classList.contains("presentation-morph-slot--figure") ||
		element.closest(".presentation-morph-slot--figure") !== null ||
		element.querySelector(".presentation-morph-slot--figure") !== null
	);
}

/** @emoji 🔗 reveal.js auto-animate matcher: tiles pair with target/source ghosts, not morph targets or the morph-one. */
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
		if (
			elementIsFigureMorphSlot(pair.from) &&
			pair.to.closest(".presentation-morph-target") !== null &&
			!elementIsTargetGhostAnchor(pair.to)
		) {
			return false;
		}
		if (elementIsTargetGhostAnchor(pair.to) && !elementIsFigureMorphSlot(pair.from)) {
			return false;
		}
		if (elementIsMorphOneAnchor(pair.from)) {
			return false;
		}
		if (elementIsSourceGhostAnchor(pair.from) && !elementIsFigureMorphSlot(pair.to)) {
			return false;
		}
		if (elementIsFigureMorphSlot(pair.from) && elementIsMorphOneAnchor(pair.to)) {
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

/** @emoji 👻 Opacity overrides so target ghosts fade 1→0 and morph targets 0→1 during reveal `running`. */
export function presentationMorphGhostAutoAnimateCss(durationSeconds: number): string {
	const duration = `${durationSeconds}s`;
	return `
.reveal .slides section[data-auto-animate="pending"] .presentation-target-ghost[data-auto-animate-target],
.reveal .slides section[data-auto-animate="pending"] .presentation-affiliation-morph-source[data-auto-animate-target] {
	opacity: 1 !important;
	visibility: visible !important;
	animation: none !important;
}
.reveal .slides section[data-auto-animate="running"] .presentation-target-ghost[data-auto-animate-target],
.reveal .slides section[data-auto-animate="running"] .presentation-affiliation-morph-source[data-auto-animate-target] {
	visibility: visible !important;
	animation: presentation-target-ghost-fade-out ${duration} ease forwards !important;
}
.reveal .slides section[data-auto-animate="pending"] .presentation-morph-target[data-auto-animate-target] {
	opacity: 0 !important;
	visibility: hidden !important;
	animation: none !important;
}
.reveal .slides section[data-auto-animate="running"] .presentation-morph-target[data-auto-animate-target] {
	visibility: visible !important;
	animation: presentation-morph-target-fade-in ${duration} ease forwards !important;
}
.reveal .slides section[data-auto-animate="pending"] .presentation-morph-one[data-auto-animate-target],
.reveal .slides section[data-auto-animate="running"] .presentation-morph-one[data-auto-animate-target] {
	animation: presentation-morph-one-fade-out ${duration} ease forwards !important;
}
`;
}

/** @emoji 🩹 Patches reveal auto-animate sheet: uniform scale, morph ghost opacity 1→0. */
export function patchPresentationAutoAnimateStyleSheet(
	sheet: { innerHTML: string },
	durationSeconds: number,
): void {
	sheet.innerHTML =
		patchAutoAnimateUniformScale(sheet.innerHTML) + presentationMorphGhostAutoAnimateCss(durationSeconds);
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

/** @emoji 🔗 When true, the interactive canvas wrapper owns `data-id` for reveal.js auto-animate. */
const MorphAnchorOnWrapperContext = createContext(false);

/** @emoji ⛶ When true, {@link PdfMorphView} measures against the slide-fullscreen content box, not the declared catalogue frame. */
const PresentationDispositionFullscreenContext = createContext(false);

export function parsePresentationSlideCssSize(revealEl: HTMLElement | null): { readonly width: number; readonly height: number } {
	const width = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-width") ?? "960");
	const height = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-height") ?? "700");
	return {
		width: Number.isFinite(width) && width > 0 ? width : 960,
		height: Number.isFinite(height) && height > 0 ? height : 700,
	};
}

/** @emoji 📐 Measures the react-pdf viewport from the disposition frame or slide-fullscreen content box. */
function usePdfPageSize(
	anchorRef: RefObject<HTMLDivElement | null>,
	position: DispositionPosition | undefined,
	slideEpoch: number,
	fullscreen: boolean,
): { readonly width?: number; readonly height?: number } {
	const [size, setSize] = useState<{ readonly width?: number; readonly height?: number }>({});
	useEffect(() => {
		const el = anchorRef.current;
		if (!el) {
			return;
		}
		const measureTarget = (): HTMLElement | null => {
			if (fullscreen) {
				return (
					el.closest(".presentation-interactive-disposition--fullscreen")?.querySelector(
						".presentation-interactive-disposition__content",
					) ?? null
				);
			}
			return el.closest(".presentation-disposition-frame");
		};
		const measure = (): void => {
			const target = measureTarget();
			const targetRect = target?.getBoundingClientRect();
			if (targetRect && targetRect.width > 8 && targetRect.height > 8) {
				setSize({
					width: Math.floor(targetRect.width),
					height: Math.floor(targetRect.height),
				});
				return;
			}
			const slide = parsePresentationSlideCssSize(el.closest(".reveal"));
			const frame = fullscreen ? SLIDE_INTERACTIVE_FULLSCREEN_FRAME : position;
			const width = frame
				? Math.floor(slide.width * frame.width)
				: Math.floor(slide.width * 0.8);
			const height = frame
				? Math.floor(slide.height * frame.height)
				: Math.floor(slide.height * 0.4);
			if (width > 0 && height > 0) {
				setSize({ width, height });
			}
		};
		measure();
		const observed = measureTarget();
		const observer =
			observed && typeof ResizeObserver !== "undefined" ? new ResizeObserver(measure) : null;
		observer?.observe(observed);
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
	}, [
		anchorRef,
		fullscreen,
		position?.height,
		position?.width,
		position?.x,
		position?.y,
		slideEpoch,
	]);
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
	const namesMuted = embodiment.abbreviateFirstName === true;
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

function affiliationLineContent(
	entry: AffiliationEntry,
	part: "line" | "suffix",
	label: string,
): ReactNode {
	const muted =
		part === "suffix"
			? affiliationLineMuted(entry.suffixEmphasis)
			: affiliationLineMuted(entry.lineEmphasis);
	const mark = part === "suffix" && entry.suffix ? entry.suffix.mark : entry.mark;
	if (muted) {
		return (
			<span className="opacity-20">
				<sup>{mark}</sup>
				{label}
			</span>
		);
	}
	return (
		<>
			<sup>{mark}</sup>
			{label}
		</>
	);
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
			{embodiment.entries.map((entry) => {
				const displayLabel = affiliationLineLabel(entry, "line");
				return (
					<div
						key={entry.mark}
						className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center gap-x-[0.35em]"
					>
						<h4
							data-id={`${anchorId}--${entry.mark}`}
							className={morphTextClass(
								anchorId,
								"presentation-affiliation-row m-0 inline-flex max-w-full shrink-0 flex-row flex-nowrap items-center justify-center gap-x-[0.35em] text-center",
							)}
						>
							{affiliationLineContent(entry, "line", displayLabel)}
							{entry.suffix ? (
								<span
									data-id={`${anchorId}--${entry.suffix.mark}`}
									className="inline-flex shrink-0 items-center justify-center text-center"
								>
									{affiliationLineContent(entry, "suffix", affiliationLineLabel(entry, "suffix"))}
								</span>
							) : null}
						</h4>
					</div>
				);
			})}
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

/** @emoji 🔗 Resolves deck-relative figure paths against the Vite base URL. */
export function resolvePresentationAssetUrl(src: string): string {
	if (/^(?:[a-z]+:)?\/\//i.test(src) || src.startsWith("data:") || src.startsWith("blob:")) {
		return src;
	}
	const trimmed = src.replace(/^\.\//, "");
	const base = import.meta.env.BASE_URL ?? "/";
	return `${base.endsWith("/") ? base : `${base}/`}${trimmed}`.replace(/\/{2,}/g, "/");
}

/** @emoji 🖼 CSS vars for crop tiles: uniform cover (shorter-side scale, centered) in frame and during reveal auto-animate. */
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
		const coverWidth = stretchWidth * coverScale;
		const coverHeight = stretchHeight * coverScale;
		const coverPosX = crop.width >= 1 ? 50 : (crop.x + crop.width / 2) * 100;
		const coverPosY = crop.height >= 1 ? 50 : (crop.y + crop.height / 2) * 100;
		morphWidth = coverWidth;
		morphHeight = coverHeight;
		morphPosX = coverPosX;
		morphPosY = coverPosY;
		restWidth = coverWidth;
		restHeight = coverHeight;
		restPosX = coverPosX;
		restPosY = coverPosY;
	} else {
		const uniform = Math.max(stretchWidth, stretchHeight);
		morphWidth = uniform;
		morphHeight = uniform;
		restWidth = uniform;
		restHeight = uniform;
		const centerPosX = crop.width >= 1 ? 50 : (crop.x + crop.width / 2) * 100;
		const centerPosY = crop.height >= 1 ? 50 : (crop.y + crop.height / 2) * 100;
		morphPosX = centerPosX;
		morphPosY = centerPosY;
		restPosX = centerPosX;
		restPosY = centerPosY;
	}
	return {
		backgroundImage: `url("${resolvePresentationAssetUrl(embodiment.src)}")`,
		["--presentation-figure-bg-size" as string]: `${restWidth}% ${restHeight}%`,
		["--presentation-figure-bg-position" as string]: `${restPosX}% ${restPosY}%`,
		["--presentation-figure-bg-size-morph" as string]: `${morphWidth}% ${morphHeight}%`,
		["--presentation-figure-bg-position-morph" as string]: `${morphPosX}% ${morphPosY}%`,
	};
}

function FigureMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
	position,
	style,
	dormantAnchor,
	anchorOnWrapper = false,
	ghost,
}: {
	readonly morphId: string;
	readonly embodiment: FigureEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
	readonly dormantAnchor?: boolean;
	readonly anchorOnWrapper?: boolean;
	readonly ghost?: GhostKind;
}): ReactNode {
	if (embodiment.crop && position) {
		const dormant = dormantAnchor === true;
		const frameStyle = anchorOnWrapper
			? {
					position: "relative" as const,
					width: "100%",
					height: "100%",
					boxSizing: "border-box" as const,
					...(style?.opacity !== undefined ? { opacity: style.opacity } : {}),
				}
			: dispositionFrameStyle(position, dormant ? undefined : style);
		return (
			<div
				{...(anchorOnWrapper ? {} : { "data-id": anchorId })}
				className={[
					"presentation-disposition-frame",
					"presentation-morph-slot",
					"presentation-morph-slot--figure",
					ghost === "target" ? "presentation-target-ghost" : undefined,
					ghost === "source" ? "presentation-source-ghost" : undefined,
					dormant ? "presentation-morph-slot--dormant" : undefined,
					emphasisClass(emphasis),
				]
					.filter(Boolean)
					.join(" ")}
				style={{
					...frameStyle,
					...figureCropBackgroundVars(embodiment, embodiment.crop, position),
				}}
				role="img"
				aria-label={embodiment.alt ?? ""}
			/>
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
	anchorOnWrapper = false,
	receivesMorphFrom = false,
}: {
	readonly morphId: string;
	readonly embodiment: TextEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position: DispositionPosition;
	readonly style?: DispositionStyle;
	readonly anchorOnWrapper?: boolean;
	readonly receivesMorphFrom?: boolean;
}): ReactNode {
	const frameStyle = anchorOnWrapper
		? {
				position: "relative" as const,
				width: "100%",
				height: "100%",
				boxSizing: "border-box" as const,
				...(style?.opacity !== undefined ? { opacity: style.opacity } : {}),
			}
		: dispositionFrameStyle(position, style);
	if (receivesMorphFrom) {
		const headingClass = centeredLineClass(anchorId, embodiment, emphasis);
		return (
			<div
				className={[
					"presentation-disposition-frame",
					"presentation-morph-slot",
					"presentation-morph-slot--label",
					"presentation-morph-target",
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
			{...(anchorOnWrapper ? {} : { "data-id": anchorId })}
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
	const fullscreen = useContext(PresentationDispositionFullscreenContext);
	const pageSize = usePdfPageSize(anchorRef, position, slideEpoch, fullscreen);
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
	const anchorOnWrapper = useContext(MorphAnchorOnWrapperContext);
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
						anchorOnWrapper={anchorOnWrapper}
						receivesMorphFrom={(disposition.morphFrom?.length ?? 0) > 0}
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
			if (embodiment.crop && disposition.position !== undefined) {
				return (
					<FigureMorphView
						morphId={anchorId}
						embodiment={embodiment}
						emphasis={emphasis}
						position={disposition.position}
						style={disposition.style}
						anchorOnWrapper={anchorOnWrapper}
						ghost={disposition.ghost}
					/>
				);
			}
			content = (
				<FigureMorphView
					morphId={anchorId}
					embodiment={embodiment}
					emphasis={emphasis}
					position={disposition.position}
					anchorOnWrapper={anchorOnWrapper}
					ghost={disposition.ghost}
				/>
			);
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

/** @emoji 🖱 Disposition ids to move/resize for this gesture (before async `selectIds` commits). */
function resolveDispositionDragGroupIds(
	id: string,
	wasSelected: boolean,
	additive: boolean,
	selectedIds: ReadonlySet<string>,
): readonly string[] {
	if (!wasSelected) {
		if (additive) {
			const group = new Set(selectedIds);
			group.add(id);
			return group.size > 1 ? [...group] : [id];
		}
		return [id];
	}
	if (additive) {
		return [id];
	}
	return selectedIds.size > 1 ? [...selectedIds] : [id];
}

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

/** @emoji 🔑 Stable id for one split tile on an interactive slide. */
export function tileDispositionInteractionId(
	renderSlideId: string,
	disposition: ResolvedDisposition,
	dispositionIndex: number,
	tileKey: string,
): string {
	return `${dispositionInteractionId(renderSlideId, disposition, dispositionIndex)}--tile--${tileKey}`;
}

/** @emoji 🔑 Stable id for a visual row band grouping split tiles on one disposition. */
export function rowBandInteractionId(
	renderSlideId: string,
	disposition: ResolvedDisposition,
	dispositionIndex: number,
	rowIndex: number,
): string {
	return `${dispositionInteractionId(renderSlideId, disposition, dispositionIndex)}--row--${rowIndex}`;
}

/** @emoji 🖱 One interactive placement (whole disposition or a single split tile). */
export interface InteractiveDispositionPlacement {
	readonly id: string;
	readonly disposition: ResolvedDisposition;
	/** @emoji 📐 Wrapper frame (row-local for tiles inside a visual row). */
	readonly declaredRect: DispositionPosition | undefined;
	/** @emoji 📐 Slide-space frame for marquee, drag, and resize. */
	readonly sectionRect: DispositionPosition | undefined;
	/** @emoji 🔗 reveal.js `data-id` on the canvas wrapper when the tile frame must not own it. */
	readonly revealMorphId?: string;
	readonly rowBandId?: string;
}

/** @emoji 📏 Row-level hit target spanning all tiles in one visual row of a split disposition. */
export interface InteractiveRowBandPlacement {
	readonly id: string;
	readonly frame: DispositionPosition;
	readonly tileIds: readonly string[];
}

/** @emoji 🖱 Interactive placements and row bands for one slide arrangement. */
export interface InteractiveSlideLayout {
	readonly placements: readonly InteractiveDispositionPlacement[];
	readonly rowBands: readonly InteractiveRowBandPlacement[];
}

/** @emoji 🧩 Builds one interactive placement per resolved disposition. */
export function buildInteractiveSlideLayout(
	renderSlideId: string,
	resolved: readonly ResolvedDisposition[],
	morph = false,
): InteractiveSlideLayout {
	const placements: InteractiveDispositionPlacement[] = [];
	resolved.forEach((disposition, dispositionIndex) => {
		const declaredRect = declaredDispositionRect(disposition);
		placements.push({
			id: dispositionInteractionId(renderSlideId, disposition, dispositionIndex),
			disposition,
			declaredRect,
			sectionRect: declaredRect,
			revealMorphId:
				morph && declaredRect !== undefined
					? disposition.ghost !== undefined
						? disposition.morphId
						: disposition.morphFrom?.length
							? undefined
							: disposition.morphId
					: undefined,
		});
	});
	return { placements, rowBands: [] };
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

/** @emoji ↔️ Moves a normalized rect by fractional deltas (unbounded; follows pointer across the slide). */
export function translateDispositionRect(
	rect: DispositionPosition,
	dx: number,
	dy: number,
): DispositionPosition {
	return {
		x: rect.x + dx,
		y: rect.y + dy,
		width: rect.width,
		height: rect.height,
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

/** @emoji ⛶ Centered near-slide frame for interactive fullscreen (uniform across figure, video, pdf, tiles). */
export const SLIDE_INTERACTIVE_FULLSCREEN_FRAME: DispositionPosition = {
	x: 0.05,
	y: 0.075,
	width: 0.9,
	height: 0.85,
};

/** @emoji ⛶ Toggles uniform slide-fullscreen frame vs stashed pre-fullscreen rect. */
export function toggleFullscreenRect(
	current: DispositionPosition,
	stash: DispositionPosition | undefined,
): { readonly rect: DispositionPosition; readonly stash: DispositionPosition | undefined } {
	if (stash !== undefined) {
		return { rect: stash, stash: undefined };
	}
	return { rect: SLIDE_INTERACTIVE_FULLSCREEN_FRAME, stash: current };
}

/** @emoji 📐 reveal.js nested slide section with usable layout height for pointer math. */
export function slideCoordinateRoot(sectionEl: HTMLElement): HTMLElement {
	const arrangement = sectionEl.closest("section.presentation-arrangement--interactive");
	if (arrangement instanceof HTMLElement) {
		return arrangement;
	}
	let current: HTMLElement | null = sectionEl;
	while (current && !current.classList.contains("slides")) {
		if (current.offsetWidth > 0 && current.offsetHeight > 0) {
			return current;
		}
		current = current.parentElement;
	}
	const stack = sectionEl.closest(".reveal .slides > section.present");
	if (stack instanceof HTMLElement && stack.offsetWidth > 0 && stack.offsetHeight > 0) {
		return stack;
	}
	const parent = sectionEl.parentElement;
	if (parent instanceof HTMLElement && parent.offsetWidth > 0 && parent.offsetHeight > 0) {
		return parent;
	}
	const reveal = sectionEl.closest(".reveal");
	return reveal instanceof HTMLElement ? reveal : sectionEl;
}

/** @emoji 📐 Client/layout bounds for slide-space fractions when inner reveal sections report zero height. */
export function slideLayoutBounds(sectionEl: HTMLElement): DOMRect {
	const root = slideCoordinateRoot(sectionEl);
	const rect = root.getBoundingClientRect();
	if (rect.width > 0 && rect.height > 0) {
		return rect;
	}
	const reveal = sectionEl.closest(".reveal");
	const { width, height } = parsePresentationSlideCssSize(reveal instanceof HTMLElement ? reveal : null);
	const anchor = sectionEl.getBoundingClientRect();
	return new DOMRect(anchor.left, anchor.top, width, height);
}

/** @emoji 🖼 Arrangement canvas when dispositions use declared slide-space frames; otherwise the slide coordinate root. */
export function dispositionPlacementContainer(
	sectionEl: HTMLElement,
	canvasPlacement: boolean,
): HTMLElement {
	if (!canvasPlacement) {
		return slideCoordinateRoot(sectionEl);
	}
	const canvas = sectionEl.querySelector(".presentation-arrangement-canvas");
	return canvas instanceof HTMLElement ? canvas : sectionEl;
}

/** @emoji 🖼 Ink-bearing node used to measure a positioned disposition on the arrangement canvas. */
export function dispositionFrameElement(root: HTMLElement): HTMLElement {
	return (
		(root.querySelector(
			".presentation-disposition-frame, .presentation-morph-slot--figure, .presentation-morph-anchor",
		) as HTMLElement | null) ?? root
	);
}

/** @emoji ⊡ Selection chrome style: flow slides use measured ink frame; canvas-framed slides use wrapper inset via CSS. */
export function interactiveDispositionChromeStyle(options: {
	readonly selected: boolean;
	readonly effectiveRect: DispositionPosition | undefined;
	readonly canvasFramed: boolean;
	readonly fullscreen: boolean;
}): CSSProperties | undefined {
	const { selected, effectiveRect, canvasFramed, fullscreen } = options;
	if (!selected || !effectiveRect || fullscreen || canvasFramed) {
		return undefined;
	}
	return transformFrameStyle(effectiveRect);
}

/** @emoji 📍 Maps client coordinates to normalized fractions inside a section element. */
export function clientToSectionFraction(
	sectionEl: HTMLElement,
	clientX: number,
	clientY: number,
	options?: { readonly clamp?: boolean },
): { readonly x: number; readonly y: number } {
	const bounds = slideLayoutBounds(sectionEl);
	if (bounds.width <= 0 || bounds.height <= 0) {
		return { x: 0, y: 0 };
	}
	const x = (clientX - bounds.left) / bounds.width;
	const y = (clientY - bounds.top) / bounds.height;
	if (options?.clamp === false) {
		return { x, y };
	}
	return {
		x: clampFraction(x),
		y: clampFraction(y),
	};
}

/** @emoji 📍 Maps an element's client rect to normalized fractions inside a section. */
export function measureElementRectInSection(
	element: HTMLElement,
	sectionEl: HTMLElement,
): DispositionPosition | null {
	const sectionBounds = slideLayoutBounds(sectionEl);
	if (sectionBounds.width <= 0 || sectionBounds.height <= 0) {
		return null;
	}
	const rect = element.getBoundingClientRect();
	if (rect.width <= 0 || rect.height <= 0) {
		return null;
	}
	return clientRectToSectionFraction(rect, sectionBounds);
}

const DISPOSITION_BOUNDS_SELECTORS =
	"[data-id], .presentation-disposition-frame, .presentation-morph-anchor, .presentation-intro-line, .presentation-morph-text, h1, h2, h3, h4, p, li, img, video, .presentation-media-figure, .presentation-figure-crop-fill, .presentation-morph-slot--figure";

function unionDomRects(a: DOMRect, b: DOMRect): DOMRect {
	const left = Math.min(a.left, b.left);
	const top = Math.min(a.top, b.top);
	const right = Math.max(a.right, b.right);
	const bottom = Math.max(a.bottom, b.bottom);
	return new DOMRect(left, top, right - left, bottom - top);
}

function tightRangeBoundsRect(element: HTMLElement): DOMRect | null {
	if (typeof document.createRange !== "function") {
		return null;
	}
	try {
		const range = document.createRange();
		range.selectNodeContents(element);
		if (typeof range.getClientRects === "function") {
			const clientRects = range.getClientRects();
			let union: DOMRect | null = null;
			for (const rect of clientRects) {
				if (rect.width <= 0 || rect.height <= 0) {
					continue;
				}
				union = union === null ? rect : unionDomRects(union, rect);
			}
			if (union !== null) {
				return union;
			}
		}
		if (typeof range.getBoundingClientRect === "function") {
			const rangeBox = range.getBoundingClientRect();
			if (rangeBox.width > 0 && rangeBox.height > 0) {
				return rangeBox;
			}
		}
	} catch {
		return null;
	}
	return null;
}

function alignTightBoxWithin(element: HTMLElement, container: DOMRect, tight: DOMRect): DOMRect {
	const align = typeof getComputedStyle === "function" ? getComputedStyle(element).textAlign : "center";
	let left = tight.left;
	if (align === "center") {
		left = container.left + (container.width - tight.width) / 2;
	} else if (align === "right") {
		left = container.right - tight.width;
	}
	return new DOMRect(left, tight.top, tight.width, tight.height);
}

function tightProbeBoundsRect(element: HTMLElement, container: DOMRect): DOMRect | null {
	const probe = document.createElement("div");
	probe.style.cssText =
		"position:fixed;left:0;top:0;visibility:hidden;pointer-events:none;width:max-content;max-width:none;";
	const clone = element.cloneNode(true) as HTMLElement;
	clone.style.display = "inline-block";
	clone.style.width = "auto";
	clone.style.maxWidth = "none";
	probe.appendChild(clone);
	document.body.appendChild(probe);
	const probeBox = clone.getBoundingClientRect();
	document.body.removeChild(probe);
	if (probeBox.width <= 0 || probeBox.height <= 0) {
		return null;
	}
	return alignTightBoxWithin(element, container, probeBox);
}

/** @emoji 📍 Ink bounds for text nodes (block headings otherwise span the full slide width). */
export function tightElementBoundsRect(element: HTMLElement): DOMRect | null {
	const box = element.getBoundingClientRect();
	if (box.width <= 0 || box.height <= 0) {
		return null;
	}
	const rangeBox = tightRangeBoundsRect(element);
	if (rangeBox !== null && rangeBox.width < box.width * 0.95) {
		return alignTightBoxWithin(element, box, rangeBox);
	}
	const probeBox = tightProbeBoundsRect(element, box);
	if (probeBox !== null && probeBox.width < box.width * 0.95) {
		return probeBox;
	}
	return box;
}

function dispositionNodeBoundsRect(node: HTMLElement, sectionWidth: number): DOMRect | null {
	const box = node.getBoundingClientRect();
	if (box.width <= 0 || box.height <= 0) {
		return null;
	}
	const useTight =
		node.matches(
			"[data-id], h1, h2, h3, h4, p, .presentation-morph-text, li, .presentation-intro-line",
		) && box.width >= sectionWidth * 0.85;
	return useTight ? (tightElementBoundsRect(node) ?? box) : box;
}

function clientRectToSectionFraction(
	rect: DOMRect,
	sectionBounds: DOMRect,
): DispositionPosition {
	return {
		x: (rect.left - sectionBounds.left) / sectionBounds.width,
		y: (rect.top - sectionBounds.top) / sectionBounds.height,
		width: rect.width / sectionBounds.width,
		height: rect.height / sectionBounds.height,
	};
}

/** @emoji 📍 Union of morph content bounds in section space (avoids full-width flow wrappers). */
export function measureDispositionBoundsInSection(
	root: HTMLElement,
	sectionEl: HTMLElement,
): DispositionPosition | null {
	root.classList.add("presentation-interactive-disposition--measuring");
	try {
		const sectionBounds = slideLayoutBounds(sectionEl);
		if (sectionBounds.width <= 0 || sectionBounds.height <= 0) {
			return null;
		}
		let union: DOMRect | null = null;
		for (const node of root.querySelectorAll(DISPOSITION_BOUNDS_SELECTORS)) {
			if (!(node instanceof HTMLElement)) {
				continue;
			}
			const rect = dispositionNodeBoundsRect(node, sectionBounds.width);
			if (!rect) {
				continue;
			}
			union = union === null ? rect : unionDomRects(union, rect);
		}
		if (union === null) {
			const fallback = root.getBoundingClientRect();
			if (fallback.width <= 0 || fallback.height <= 0) {
				return null;
			}
			return clientRectToSectionFraction(fallback, sectionBounds);
		}
		return clientRectToSectionFraction(union, sectionBounds);
	} finally {
		root.classList.remove("presentation-interactive-disposition--measuring");
	}
}

/** @emoji 📍 Ink bounds as fractions inside a container (selection chrome and fill use this space). */
export function measureDispositionBoundsInContainer(
	root: HTMLElement,
	containerEl: HTMLElement,
): DispositionPosition | null {
	root.classList.add("presentation-interactive-disposition--measuring");
	try {
		const containerBounds = containerEl.getBoundingClientRect();
		if (containerBounds.width <= 0 || containerBounds.height <= 0) {
			return null;
		}
		let union: DOMRect | null = null;
		for (const node of root.querySelectorAll(DISPOSITION_BOUNDS_SELECTORS)) {
			if (!(node instanceof HTMLElement)) {
				continue;
			}
			const rect = dispositionNodeBoundsRect(node, containerBounds.width);
			if (!rect) {
				continue;
			}
			union = union === null ? rect : unionDomRects(union, rect);
		}
		if (union === null) {
			const fallback = root.getBoundingClientRect();
			if (fallback.width <= 0 || fallback.height <= 0) {
				return null;
			}
			return clientRectToSectionFraction(fallback, containerBounds);
		}
		return clientRectToSectionFraction(union, containerBounds);
	} finally {
		root.classList.remove("presentation-interactive-disposition--measuring");
	}
}

/** @emoji 📏 True when measured fractions are large enough to drag or resize reliably. */
export function isUsableMeasuredRect(rect: DispositionPosition): boolean {
	return rect.width >= DISPOSITION_MIN_FRACTION && rect.height >= DISPOSITION_MIN_FRACTION;
}

/** @emoji 📐 Declared placement for one resolved disposition (includes dormant morph anchors at opacity 0). */
export function declaredDispositionRect(disposition: ResolvedDisposition): DispositionPosition | undefined {
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

/** @emoji 📐 Cumulative visual scale for one element (client rect vs layout box, includes ancestor transforms). */
export function elementVisualScale(element: HTMLElement): number {
	const rect = element.getBoundingClientRect();
	const layoutW = element.offsetWidth;
	const layoutH = element.offsetHeight;
	if (layoutW <= 0 || layoutH <= 0 || rect.width <= 0 || rect.height <= 0) {
		return 1;
	}
	const scaleX = rect.width / layoutW;
	const scaleY = rect.height / layoutH;
	const scale = Math.min(scaleX, scaleY);
	if (!Number.isFinite(scale) || scale <= 0) {
		return 1;
	}
	return Math.min(4, Math.max(0.05, scale));
}

/** @emoji 📐 reveal.js scales slides visually; map screen-pointer deltas to local translate pixels. */
export function sectionVisualScale(sectionEl: HTMLElement): number {
	return elementVisualScale(slideCoordinateRoot(sectionEl));
}

/** @emoji ↔️ Node that receives flow `translate3d` during drag (content when laid out, else disposition root). */
export function flowDragTransformElement(
	sectionEl: HTMLElement,
	root: HTMLElement | null,
	content: HTMLElement | null,
): HTMLElement {
	if (content && content.offsetWidth > 0 && content.offsetHeight > 0) {
		return content;
	}
	if (root && root.offsetWidth > 0 && root.offsetHeight > 0) {
		return root;
	}
	return slideCoordinateRoot(sectionEl);
}

/** @emoji ↔️ Pointer travel in screen px → local px for CSS translate on the flow drag target (1:1 with cursor). */
export function flowPointerDeltaToLocal(
	_transformEl: HTMLElement,
	startClientX: number,
	startClientY: number,
	currentClientX: number,
	currentClientY: number,
): { readonly dx: number; readonly dy: number } {
	return {
		dx: currentClientX - startClientX,
		dy: currentClientY - startClientY,
	};
}

/** @emoji ↔️ Flow-layout drag: local-pixel translate (x/y are px, not normalized). */
export function flowDispositionOffsetStyle(transform: DispositionPosition): CSSProperties {
	return {
		transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
	};
}

/** @emoji 📐 True when two normalized disposition frames differ. */
export function dispositionPositionChanged(
	a: DispositionPosition,
	b: DispositionPosition,
): boolean {
	return (
		Math.abs(a.x - b.x) > 1e-6 ||
		Math.abs(a.y - b.y) > 1e-6 ||
		Math.abs(a.width - b.width) > 1e-6 ||
		Math.abs(a.height - b.height) > 1e-6
	);
}

/** @emoji 📐 Flow drag: preserve the wrapper footprint while ink is absolutely positioned inside. */
export function flowDispositionReserveStyle(reservePx: {
	readonly width: number;
	readonly height: number;
}): CSSProperties {
	return {
		minWidth: reservePx.width,
		minHeight: reservePx.height,
	};
}

/** @emoji 📐 Flow transforms store pointer deltas; positioned transforms store slide-space frames. */
export function flowDispositionManipulationRect(
	measured: DispositionPosition,
	existing: DispositionPosition | undefined,
): DispositionPosition {
	if (existing) {
		return existing;
	}
	return { x: 0, y: 0, width: measured.width, height: measured.height };
}

/** @emoji 📐 True when x/y/width/height are a normalized slide-space frame (not pixel drag storage). */
export function isNormalizedSlideFrame(transform: DispositionPosition): boolean {
	return (
		transform.x >= 0 &&
		transform.x <= 1 &&
		transform.y >= 0 &&
		transform.y <= 1 &&
		transform.width > 0 &&
		transform.width <= 1 &&
		transform.height > 0 &&
		transform.height <= 1 &&
		transform.x + transform.width <= 1 + 1e-6 &&
		transform.y + transform.height <= 1 + 1e-6
	);
}

/** @emoji ↔️ True when a flow transform only stores pixel translate with unchanged measured size. */
export function isFlowPixelOffsetTransform(
	transform: DispositionPosition,
	measured: DispositionPosition | undefined,
): boolean {
	const sizeMatches =
		measured === undefined
			? transform.width > 0 &&
				transform.width <= 1 &&
				transform.height > 0 &&
				transform.height <= 1
			: transform.width === measured.width && transform.height === measured.height;
	if (!sizeMatches) {
		return false;
	}
	if (measured !== undefined) {
		return true;
	}
	if (Math.abs(transform.x) > 1 || Math.abs(transform.y) > 1) {
		return true;
	}
	if (transform.x === 0 && transform.y === 0) {
		return true;
	}
	return !isNormalizedSlideFrame(transform);
}

/** @emoji 🔍 Uniform scale for pinned resize so ink zooms inside the frame without reflow overflow. */
export function interactiveDispositionContentScale(
	transform: DispositionPosition,
	baseline: DispositionPosition | undefined,
): number | null {
	if (!baseline || baseline.width <= 0 || baseline.height <= 0) {
		return null;
	}
	if (transform.width <= 0 || transform.height <= 0) {
		return null;
	}
	const scaleX = transform.width / baseline.width;
	const scaleY = transform.height / baseline.height;
	const widthChanged = Math.abs(scaleX - 1) >= 1e-4;
	const heightChanged = Math.abs(scaleY - 1) >= 1e-4;
	if (!widthChanged && !heightChanged) {
		return null;
	}
	if (!widthChanged) {
		return Number.isFinite(scaleY) ? scaleY : null;
	}
	if (!heightChanged) {
		return Number.isFinite(scaleX) ? scaleX : null;
	}
	const uniform = Math.min(scaleX, scaleY);
	return Number.isFinite(uniform) ? uniform : null;
}

/** @emoji 🔍 CSS transform that scales disposition content from its center during interactive resize. */
export function interactiveDispositionContentScaleStyle(scale: number): CSSProperties {
	return {
		transform: `scale(${scale})`,
		transformOrigin: "center center",
		willChange: "transform",
	};
}

/** @emoji 📍 Converts a flow pixel-offset transform into a slide-space frame using measured ink bounds. */
export function flowPixelOffsetToSectionRect(
	measured: DispositionPosition,
	transform: DispositionPosition,
	sectionEl: HTMLElement,
	transformEl?: HTMLElement,
): DispositionPosition {
	const el = transformEl ?? flowDragTransformElement(sectionEl, null, null);
	const layout = slideLayoutBounds(sectionEl);
	const scale = elementVisualScale(el);
	const w = layout.width / scale;
	const h = layout.height / scale;
	return {
		x: measured.x + transform.x / w,
		y: measured.y + transform.y / h,
		width: transform.width,
		height: transform.height,
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

const InteractiveRowBand: FC<{
	readonly id: string;
	readonly frame: DispositionPosition;
	readonly tileIds: readonly string[];
}> = ({ id, frame, tileIds }) => {
	const interaction = usePresentationInteractionState();
	const rowSelected = tileIds.length > 0 && tileIds.every((tileId) => interaction.isSelected(tileId));
	const onPointerDown = (event: React.PointerEvent): void => {
		if (event.button !== 0) {
			return;
		}
		event.preventDefault();
		event.stopPropagation();
		interaction.selectIds(tileIds, event.shiftKey);
	};
	return (
		<div
			className={[
				"presentation-interactive-row-band",
				rowSelected ? "presentation-interactive-row-band--selected" : undefined,
			]
				.filter(Boolean)
				.join(" ")}
			data-row-band={id}
			style={transformFrameStyle(frame)}
			onPointerDown={onPointerDown}
			aria-hidden
		/>
	);
};

const InteractiveDisposition: FC<{
	readonly id: string;
	readonly disposition: ResolvedDisposition;
	readonly declaredRect: DispositionPosition | undefined;
	readonly sectionDeclaredRect: DispositionPosition | undefined;
	readonly allDeclaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
	readonly sectionRef: RefObject<HTMLElement | null>;
	readonly revealMorphId?: string;
	readonly rowBandId?: string;
}> = ({
	id,
	disposition,
	declaredRect,
	sectionDeclaredRect,
	allDeclaredRects,
	sectionRef,
	revealMorphId,
	rowBandId,
}) => {
	const slideEpoch = useContext(PresentationSlideEpochContext);
	const interaction = usePresentationInteractionState();
	const registry = useSlideDispositionRegistry();
	const rootRef = useRef<HTMLDivElement>(null);
	const contentRef = useRef<HTMLDivElement>(null);
	const selected = interaction.isSelected(id);
	const interactionRect = sectionDeclaredRect ?? declaredRect;
	const flowLayout = interactionRect === undefined;
	const transform = interaction.getTransform(id);
	const transformed = transform !== undefined;
	const measuredNatural = registry.getRect(id);
	const flowPixelOffset =
		transformed &&
		flowLayout &&
		transform !== undefined &&
		isFlowPixelOffsetTransform(transform, measuredNatural);
	const flowSectionFrame = transformed && flowLayout && !flowPixelOffset;
	const fullscreen = interaction.isFullscreen(id);
	const canvasFramed = declaredRect !== undefined && !fullscreen;
	const canvasPlacement = interactionRect !== undefined;
	const effectiveRect = resolveEffectiveDispositionRect(id, interactionRect, interaction, registry);
	const canvasAnchorRect = interactionRect;
	const canvasLiveTransform =
		canvasFramed && transform && canvasAnchorRect ? transform : undefined;
	const canvasDragActive = Boolean(
		canvasLiveTransform &&
			canvasAnchorRect &&
			!fullscreen &&
			dispositionPositionChanged(canvasAnchorRect, canvasLiveTransform),
	);
	const pinned =
		!fullscreen &&
		((transformed && !flowLayout && !canvasFramed) ||
			flowSectionFrame ||
			(canvasFramed && canvasDragActive && !flowPixelOffset));
	const [gesturing, setGesturing] = useState(false);
	const [flowReservePx, setFlowReservePx] = useState<{ readonly width: number; readonly height: number } | null>(
		null,
	);
	const useFlowInkFrame = flowLayout && !flowSectionFrame;
	const [inkInWrapper, setInkInWrapper] = useState<DispositionPosition | null>(null);
	const displayDisposition =
		fullscreen && disposition.position !== undefined
			? { ...disposition, position: undefined }
			: disposition;

	const measureDispositionRect = useCallback((): DispositionPosition | null => {
		const section = sectionRef.current;
		const root = rootRef.current;
		if (!root || !section) {
			return null;
		}
		const container = dispositionPlacementContainer(section, canvasPlacement);
		const measured = canvasPlacement
			? measureElementRectInSection(dispositionFrameElement(root), container)
			: measureDispositionBoundsInSection(root, section);
		if (!measured || !isUsableMeasuredRect(measured)) {
			return null;
		}
		return measured;
	}, [canvasPlacement, sectionRef]);

	useLayoutEffect(() => {
		if (!transform) {
			setFlowReservePx(null);
		}
	}, [transform]);

	useLayoutEffect(() => {
		if (interactionRect) {
			registry.registerRect(id, null);
			return;
		}
		if (transform) {
			return;
		}
		const section = sectionRef.current;
		if (!section || !section.classList.contains("present")) {
			registry.registerRect(id, null);
			return;
		}
		const measured = measureDispositionRect();
		registry.registerRect(id, measured);
	}, [id, interactionRect, transform, registry, sectionRef, slideEpoch, measureDispositionRect]);

	useLayoutEffect(() => {
		if (!useFlowInkFrame || !(selected || gesturing || flowPixelOffset)) {
			setInkInWrapper(null);
			return;
		}
		const root = rootRef.current;
		if (!root) {
			setInkInWrapper(null);
			return;
		}
		setInkInWrapper(measureDispositionBoundsInContainer(root, root));
	}, [useFlowInkFrame, selected, gesturing, flowPixelOffset, transform, slideEpoch, disposition]);

	const ensureRectForManipulation = useCallback(
		(kind: "move" | "resize"): DispositionPosition | null => {
			const section = sectionRef.current;
			const existing = interaction.getTransform(id);
			const measuredNatural = registry.getRect(id) ?? measureDispositionRect();
			if (interactionRect) {
				return existing ?? interactionRect;
			}
			if (kind === "resize") {
				if (!measuredNatural) {
					return null;
				}
				registry.registerRect(id, measuredNatural);
				if (existing && section && isFlowPixelOffsetTransform(existing, measuredNatural)) {
					return flowPixelOffsetToSectionRect(
						measuredNatural,
						existing,
						section,
						flowDragTransformElement(section, rootRef.current, contentRef.current),
					);
				}
				return existing ?? measuredNatural;
			}
			if (existing) {
				return existing;
			}
			if (!measuredNatural) {
				return null;
			}
			registry.registerRect(id, measuredNatural);
			return flowDispositionManipulationRect(measuredNatural, undefined);
		},
		[id, interactionRect, interaction, registry, measureDispositionRect, sectionRef],
	);

	const attachPointerGesture = useCallback(
		(
			origin: {
				readonly pointerId: number;
				readonly clientX: number;
				readonly clientY: number;
				readonly captureEl: HTMLElement | null;
			},
			mode: "move" | DispositionResizeHandle,
			initialRect: DispositionPosition,
			requireDragThreshold: boolean,
			dragMemberIds: readonly string[],
		): void => {
			const section = sectionRef.current;
			if (!section) {
				return;
			}
			const fractionContainer = dispositionPlacementContainer(section, canvasPlacement);
			const gesturePlacementBounds = fractionContainer.getBoundingClientRect();
			const pointerId = origin.pointerId;
			const startClient = { x: origin.clientX, y: origin.clientY };
			const groupIds = dragMemberIds.length > 1 ? [...dragMemberIds] : [id];
			const startRects = new Map<string, DispositionPosition>();
			for (const memberId of groupIds) {
				const memberDeclared = allDeclaredRects.get(memberId);
				let rect: DispositionPosition | undefined =
					interaction.getTransform(memberId) ?? memberDeclared;
				if (!rect) {
					const natural = memberId === id ? initialRect : registry.getRect(memberId);
					if (natural) {
						rect =
							memberDeclared !== undefined
								? natural
								: flowDispositionManipulationRect(natural, undefined);
					}
				}
				if (rect && (memberDeclared !== undefined || isUsableMeasuredRect(rect))) {
					startRects.set(memberId, rect);
				}
			}
			if (startRects.size === 0) {
				startRects.set(id, initialRect);
			}
			const startGroup =
				mode !== "move" && groupIds.length > 1
					? groupBoundingRect([...startRects.values()])
					: null;
			let dragging = !requireDragThreshold;
			setGesturing(true);
			const flowTransformEl = flowDragTransformElement(
				section,
				rootRef.current,
				contentRef.current,
			);
			if (allDeclaredRects.get(id) === undefined) {
				const root = rootRef.current;
				if (root && root.offsetWidth > 0 && root.offsetHeight > 0) {
					setFlowReservePx({ width: root.offsetWidth, height: root.offsetHeight });
				}
			}

			const onMove = (moveEvent: PointerEvent): void => {
				if (moveEvent.pointerId !== pointerId) {
					return;
				}
				if (!dragging) {
					if (
						Math.hypot(moveEvent.clientX - startClient.x, moveEvent.clientY - startClient.y) <
						POINTER_DRAG_THRESHOLD_PX
					) {
						return;
					}
					dragging = true;
				}
				moveEvent.preventDefault();
				const updates = new Map<string, DispositionTransform>();
				if (mode === "move") {
					for (const [memberId, rect] of startRects) {
						if (allDeclaredRects.get(memberId) === undefined) {
							const { dx, dy } = flowPointerDeltaToLocal(
								flowTransformEl,
								startClient.x,
								startClient.y,
								moveEvent.clientX,
								moveEvent.clientY,
							);
							updates.set(memberId, {
								x: rect.x + dx,
								y: rect.y + dy,
								width: rect.width,
								height: rect.height,
							});
						} else {
							const container = dispositionPlacementContainer(
								section,
								allDeclaredRects.get(memberId) !== undefined,
							);
							const bounds =
								container === fractionContainer && gesturePlacementBounds.width > 0
									? gesturePlacementBounds
									: container.getBoundingClientRect();
							const dx =
								bounds.width > 0 ? (moveEvent.clientX - startClient.x) / bounds.width : 0;
							const dy =
								bounds.height > 0 ? (moveEvent.clientY - startClient.y) / bounds.height : 0;
							updates.set(memberId, translateDispositionRect(rect, dx, dy));
						}
					}
				} else {
					const bounds =
						gesturePlacementBounds.width > 0
							? gesturePlacementBounds
							: fractionContainer.getBoundingClientRect();
					const dx = bounds.width > 0 ? (moveEvent.clientX - startClient.x) / bounds.width : 0;
					const dy = bounds.height > 0 ? (moveEvent.clientY - startClient.y) / bounds.height : 0;
					if (startGroup && groupIds.length > 1) {
						const resizedGroup = resizeDispositionRect(startGroup, mode, dx, dy);
						for (const [memberId, rect] of startRects) {
							updates.set(memberId, scaleRectWithinGroup(rect, startGroup, resizedGroup));
						}
					} else {
						const startRect = startRects.get(id) ?? initialRect;
						updates.set(id, resizeDispositionRect(startRect, mode, dx, dy));
					}
				}
				interaction.setTransforms(updates);
			};

			const onUp = (upEvent: PointerEvent): void => {
				if (upEvent.pointerId !== pointerId) {
					return;
				}
				setGesturing(false);
				window.removeEventListener("pointermove", onMove);
				window.removeEventListener("pointerup", onUp);
				window.removeEventListener("pointercancel", onUp);
				try {
					origin.captureEl?.releasePointerCapture?.(pointerId);
				} catch {
					// jsdom may not support pointer capture
				}
			};

			try {
				origin.captureEl?.setPointerCapture?.(pointerId);
			} catch {
				// jsdom may not support pointer capture
			}
			window.addEventListener("pointermove", onMove);
			window.addEventListener("pointerup", onUp);
			window.addEventListener("pointercancel", onUp);
		},
		[id, canvasPlacement, allDeclaredRects, interaction, registry, sectionRef],
	);

	const seedCanvasTransform = useCallback((): DispositionPosition | null => {
		if (!interactionRect) {
			return null;
		}
		const existing = interaction.getTransform(id);
		if (existing) {
			return existing;
		}
		interaction.setTransform(id, interactionRect);
		return interactionRect;
	}, [id, interactionRect, interaction]);

	const onPointerDown = (event: React.PointerEvent): void => {
		if (event.button !== 0) {
			return;
		}
		if ((event.target as HTMLElement).closest(".presentation-interaction-handle")) {
			return;
		}
		if ((event.target as HTMLElement).closest(".presentation-interaction-fullscreen")) {
			return;
		}
		event.preventDefault();
		event.stopPropagation();
		const additive = event.shiftKey;
		const dragMemberIds = resolveDispositionDragGroupIds(
			id,
			selected,
			additive,
			interaction.selectedIds,
		);
		if (!selected) {
			interaction.selectIds([id], additive);
		} else if (additive) {
			interaction.selectIds([id], true);
		}
		seedCanvasTransform();
		const rect = ensureRectForManipulation("move");
		if (!rect) {
			return;
		}
		attachPointerGesture(
			{
				pointerId: event.pointerId,
				clientX: event.clientX,
				clientY: event.clientY,
				captureEl: rootRef.current,
			},
			"move",
			rect,
			true,
			dragMemberIds,
		);
	};

	const onHandlePointerDown = (handle: DispositionResizeHandle) => (event: React.PointerEvent) => {
		if (event.button !== 0) {
			return;
		}
		event.preventDefault();
		event.stopPropagation();
		const dragMemberIds = resolveDispositionDragGroupIds(
			id,
			selected,
			false,
			interaction.selectedIds,
		);
		if (!selected) {
			interaction.selectIds([id], false);
		}
		seedCanvasTransform();
		const rect = ensureRectForManipulation("resize");
		if (!rect) {
			return;
		}
		attachPointerGesture(
			{
				pointerId: event.pointerId,
				clientX: event.clientX,
				clientY: event.clientY,
				captureEl: event.currentTarget as HTMLElement,
			},
			handle,
			rect,
			false,
			dragMemberIds,
		);
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
		flowPixelOffset ? "presentation-interactive-disposition--offset" : undefined,
		pinned ? "presentation-interactive-disposition--pinned" : undefined,
		canvasFramed ? "presentation-interactive-disposition--canvas-framed" : undefined,
		gesturing ? "presentation-interactive-disposition--gesturing" : undefined,
		fullscreen ? "presentation-interactive-disposition--fullscreen" : undefined,
		disposition.ghost === "target" ? "presentation-target-ghost" : undefined,
		disposition.ghost === "source" ? "presentation-source-ghost" : undefined,
		(disposition.morphFrom?.length ?? 0) > 0 ? "presentation-morph-target" : undefined,
		(disposition.morphTo?.length ?? 0) > 0 ? "presentation-morph-one" : undefined,
	]
		.filter(Boolean)
		.join(" ");

	const flowInkActive = useFlowInkFrame && (selected || gesturing || flowPixelOffset);
	const wrapperFrame: CSSProperties = flowInkActive ? { position: "relative" } : {};
	if (flowReservePx) {
		Object.assign(wrapperFrame, flowDispositionReserveStyle(flowReservePx));
	}
	if (canvasFramed && canvasAnchorRect) {
		// 🔀 The wrapper owns the reveal `data-id` morph anchor; placing it on the live ephemeral
		// rect (drag/resize) makes reveal.js auto-animate capture the modified frame as the morph
		// `from`, so morphs start from the current disposition including ephemeral modifications.
		// 👻 Morph-source ghosts must stay on morphFrom label frames, never ephemeral focus tiles.
		const morphAnchorRect =
			disposition.ghost !== undefined ? canvasAnchorRect : (canvasLiveTransform ?? canvasAnchorRect);
		Object.assign(wrapperFrame, transformFrameStyle(morphAnchorRect));
	} else if (transformed && transform && !flowPixelOffset) {
		Object.assign(wrapperFrame, transformFrameStyle(transform));
	}
	const wrapperStyle: CSSProperties | undefined = fullscreen
		? transformFrameStyle(SLIDE_INTERACTIVE_FULLSCREEN_FRAME)
		: Object.keys(wrapperFrame).length > 0
			? wrapperFrame
			: undefined;
	const contentInkStyle: CSSProperties | undefined =
		flowInkActive && inkInWrapper ? transformFrameStyle(inkInWrapper) : undefined;
	const flowDragOffsetStyle =
		flowPixelOffset && transform ? flowDispositionOffsetStyle(transform) : undefined;
	const resizeContentBaseline = measuredNatural ?? interactionRect;
	const resizeContentScale =
		!canvasFramed &&
		!flowPixelOffset &&
		transform &&
		resizeContentBaseline
			? interactiveDispositionContentScale(transform, resizeContentBaseline)
			: null;
	const contentStyle: CSSProperties | undefined = {
		...(contentInkStyle ?? {}),
		...(flowDragOffsetStyle ?? {}),
		...(resizeContentScale !== null ? interactiveDispositionContentScaleStyle(resizeContentScale) : {}),
	};
	const hasContentStyle = Object.keys(contentStyle).length > 0;
	const chromeLayoutRect =
		flowSectionFrame && transform
			? transform
			: canvasLiveTransform ?? effectiveRect;
	const chromeStyle: CSSProperties | undefined = useFlowInkFrame
		? undefined
		: interactiveDispositionChromeStyle({
				selected: selected || gesturing,
				effectiveRect: chromeLayoutRect,
				canvasFramed,
				fullscreen,
			});
	const showControls =
		(selected || gesturing) &&
		Boolean(useFlowInkFrame ? inkInWrapper ?? measuredNatural : chromeLayoutRect ?? effectiveRect);
	const showHandles = showControls && !fullscreen;

	return (
		<div
			ref={rootRef}
			data-disposition-id={id}
			{...(revealMorphId ? { "data-id": revealMorphId } : {})}
			{...(rowBandId ? { "data-row-band": rowBandId } : {})}
			className={wrapperClass}
			style={wrapperStyle}
			onPointerDown={onPointerDown}
		>
			<div
				ref={contentRef}
				className="presentation-interactive-disposition__content"
				style={hasContentStyle ? contentStyle : undefined}
			>
				<PresentationDispositionFullscreenContext.Provider value={fullscreen}>
					<MorphAnchorOnWrapperContext.Provider
						value={Boolean(revealMorphId && declaredRect !== undefined)}
					>
						<MorphDispositionView disposition={displayDisposition} />
					</MorphAnchorOnWrapperContext.Provider>
				</PresentationDispositionFullscreenContext.Provider>
				{showControls ? (
					<>
						{showHandles ? (
							<div
								className="presentation-interactive-disposition__chrome"
								style={chromeStyle}
								aria-hidden
							>
								{DISPOSITION_RESIZE_HANDLES.map((handle) => (
									<div
										key={handle}
										className={`presentation-interaction-handle presentation-interaction-handle--${handle}`}
										onPointerDown={onHandlePointerDown(handle)}
									/>
								))}
							</div>
						) : null}
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
		</div>
	);
};

function isDispositionPointerTarget(target: EventTarget | null): boolean {
	if (!(target instanceof HTMLElement)) {
		return false;
	}
	return Boolean(
		target.closest(".presentation-interactive-disposition") ||
			target.closest(".presentation-interactive-row-band") ||
			target.closest(".presentation-interactive-visual-row") ||
			target.closest(".presentation-interaction-handle") ||
			target.closest(".presentation-interaction-fullscreen"),
	);
}

/** @emoji 🖱 Marquee and deselect on slide background; mounted on the arrangement section capture phase. */
function useSlideBackgroundInteraction({
	sectionRef,
	canvasPlacement,
	dispositionIds,
	declaredRects,
}: {
	readonly sectionRef: RefObject<HTMLElement | null>;
	readonly canvasPlacement: boolean;
	readonly dispositionIds: readonly string[];
	readonly declaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
}): {
	readonly onPointerDownCapture: (event: React.PointerEvent) => void;
	readonly marquee: {
		readonly start: { readonly x: number; readonly y: number };
		readonly end: { readonly x: number; readonly y: number };
	} | null;
} {
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

	const onPointerDownCapture = useCallback(
		(event: React.PointerEvent): void => {
			if (event.button !== 0 || isDispositionPointerTarget(event.target)) {
				return;
			}
			const section = sectionRef.current;
			if (!section) {
				return;
			}
			const fractionContainer = dispositionPlacementContainer(section, canvasPlacement);
			const fraction = clientToSectionFraction(fractionContainer, event.clientX, event.clientY);
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
				const current = clientToSectionFraction(fractionContainer, moveEvent.clientX, moveEvent.clientY);
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
				const end = clientToSectionFraction(fractionContainer, upEvent.clientX, upEvent.clientY);
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
		},
		[canvasPlacement, declaredRects, dispositionIds, interaction, resolveRectForId, sectionRef],
	);

	return { onPointerDownCapture, marquee };
}

const InteractionLayer: FC<{
	readonly marquee: {
		readonly start: { readonly x: number; readonly y: number };
		readonly end: { readonly x: number; readonly y: number };
	} | null;
}> = ({ marquee }) => {
	const marqueeStyle: CSSProperties | undefined = marquee
		? (() => {
				const box = normalizeMarquee(marquee.start, marquee.end);
				return {
					left: `${box.x * 100}%`,
					top: `${box.y * 100}%`,
					width: `${box.width * 100}%`,
					height: `${box.height * 100}%`,
				};
			})()
		: undefined;

	const marqueeRule =
		marquee === null ? null : marqueeSelectionRule(marquee.start, marquee.end);

	return (
		<div className="presentation-interaction-layer" aria-hidden>
			{marquee && marqueeStyle ? (
				<div
					className={[
						"presentation-interaction-marquee",
						marqueeRule === "crossing"
							? "presentation-interaction-marquee--crossing"
							: "presentation-interaction-marquee--window",
					].join(" ")}
					style={marqueeStyle}
				/>
			) : null}
		</div>
	);
};
//#endregion 🔖Interaction

//#region 🔖ArrangementSection
const ArrangementSection: FC<{
	readonly presentation: Presentation;
	readonly chapter: Chapter;
	readonly sequence: Sequence;
	readonly thought: Thought;
	readonly renderSlide: RenderSlide;
}> = ({ presentation, chapter, sequence, thought, renderSlide }) => {
	const sectionRef = useRef<HTMLElement>(null);
	const scope = useMemo(
		() => resolutionScopeForArrangement(presentation, chapter, sequence, thought, renderSlide.arrangement),
		[presentation, chapter, sequence, thought, renderSlide.arrangement],
	);
	const resolved = resolveArrangement(scope, renderSlide.arrangement);
	const morph = renderSlide.autoAnimateId !== undefined;
	const positioned = resolved.some((disposition) => disposition.position !== undefined);
	const layoutResolved =
		positioned && !morph ? centerResolvedArrangement(resolved) : resolved;
	const interactiveLayout = useMemo(
		() => buildInteractiveSlideLayout(renderSlide.id, layoutResolved, morph),
		[layoutResolved, morph, renderSlide.id],
	);
	const declaredRects = useMemo(() => {
		const map = new Map<string, DispositionPosition | undefined>();
		for (const entry of interactiveLayout.placements) {
			map.set(entry.id, entry.sectionRect ?? entry.declaredRect);
		}
		return map;
	}, [interactiveLayout.placements]);
	const placements = (
		<>
			{interactiveLayout.rowBands.map((band) => (
				<InteractiveRowBand key={band.id} id={band.id} frame={band.frame} tileIds={band.tileIds} />
			))}
			{interactiveLayout.placements.map((entry) => (
				<InteractiveDisposition
					key={entry.id}
					id={entry.id}
					disposition={entry.disposition}
					declaredRect={entry.declaredRect}
					sectionDeclaredRect={entry.sectionRect}
					allDeclaredRects={declaredRects}
					sectionRef={sectionRef}
					revealMorphId={entry.revealMorphId}
					rowBandId={entry.rowBandId}
				/>
			))}
		</>
	);
	return (
		<SlideDispositionRegistryProvider>
			<ArrangementSectionSurface
				sectionRef={sectionRef}
				morph={morph}
				autoAnimateId={renderSlide.autoAnimateId}
				settleBeforeMorphTo={renderSlide.arrangement.settleBeforeMorphTo}
				slideId={renderSlide.id}
				positioned={positioned}
				dispositionIds={interactiveLayout.placements.map((entry) => entry.id)}
				declaredRects={declaredRects}
				placements={placements}
			/>
		</SlideDispositionRegistryProvider>
	);
};

const ArrangementSectionSurface: FC<{
	readonly sectionRef: RefObject<HTMLElement | null>;
	readonly morph: boolean;
	readonly autoAnimateId: string | undefined;
	readonly settleBeforeMorphTo: readonly string[] | undefined;
	readonly slideId: string;
	readonly positioned: boolean;
	readonly dispositionIds: readonly string[];
	readonly declaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
	readonly placements: ReactNode;
}> = ({
	sectionRef,
	morph,
	autoAnimateId,
	settleBeforeMorphTo,
	slideId,
	positioned,
	dispositionIds,
	declaredRects,
	placements,
}) => {
	const backgroundInteraction = useSlideBackgroundInteraction({
		sectionRef,
		canvasPlacement: positioned,
		dispositionIds,
		declaredRects,
	});
	return (
		<section
			ref={sectionRef}
			onPointerDownCapture={backgroundInteraction.onPointerDownCapture}
			{...(morph ? { "data-auto-animate": "", "data-auto-animate-id": autoAnimateId } : {})}
			{...(settleBeforeMorphTo?.length
				? { "data-settle-before-morph-to": settleBeforeMorphTo.join(",") }
				: {})}
			title={slideId}
			className={[
				"presentation-arrangement--interactive",
				positioned ? "presentation-arrangement--positioned" : undefined,
			]
				.filter(Boolean)
				.join(" ")}
		>
			<InteractionLayer marquee={backgroundInteraction.marquee} />
			{positioned ? (
				<div className="presentation-arrangement-canvas">
					{placements}
				</div>
			) : (
				placements
			)}
		</section>
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
		let autoAnimateFinalizeTimer: ReturnType<typeof setTimeout> | undefined;
		const scheduleFinalizeAutoAnimateRest = (): void => {
			const durationSeconds =
				typeof deck.getConfig().autoAnimateDuration === "number" ? deck.getConfig().autoAnimateDuration : 1;
			if (autoAnimateFinalizeTimer !== undefined) {
				clearTimeout(autoAnimateFinalizeTimer);
			}
			autoAnimateFinalizeTimer = setTimeout(() => {
				finalizeRevealAutoAnimateRestState(deckEl);
				autoAnimateFinalizeTimer = undefined;
			}, durationSeconds * 1000 + 80);
		};
		const onBeforeSlideChange = (event: Event): void => {
			if (autoAnimateFinalizeTimer !== undefined) {
				clearTimeout(autoAnimateFinalizeTimer);
				autoAnimateFinalizeTimer = undefined;
			}
			const slideEvent = event as Event & { readonly indexh?: number; readonly indexv?: number };
			const fromSlide = deck.getCurrentSlide() as HTMLElement | null;
			if (!fromSlide || slideEvent.indexh === undefined || slideEvent.indexv === undefined) {
				return;
			}
			const toSlide = resolveRevealSlideAt(deckEl, { h: slideEvent.indexh, v: slideEvent.indexv });
			if (!toSlide) {
				return;
			}
			const fromAnimateId = fromSlide.getAttribute("data-auto-animate-id");
			const toAnimateId = toSlide.getAttribute("data-auto-animate-id");
			if (!fromAnimateId || fromAnimateId !== toAnimateId) {
				finalizeRevealAutoAnimateRestState(deckEl);
			}
			prepareArrangementBeforeAutoAnimate(fromSlide, toSlide);
			if (toSlide.getAttribute("title") === "catalogue-labels") {
				flushSync(() => {
					setSlideEpoch((epoch) => epoch + 1);
				});
				for (const ghost of toSlide.querySelectorAll<HTMLElement>(".presentation-target-ghost")) {
					void ghost.offsetHeight;
				}
			}
		};
		const onSlideChanged = (event: Event): void => {
			relaxHiddenPreflight();
			syncRevealBackgroundKind(deckEl);
			syncPresentationSlideSizeVars(deckEl, deck);
			syncPresentSlideMedia();
			syncSlideUrl();
			const { previousSlide: eventPrevious, currentSlide: eventCurrent } = slideChangedEventSlides(event);
			const currentSlide = eventCurrent ?? (deck.getCurrentSlide() as HTMLElement | null);
			const previousSlide = eventPrevious ?? previousSlideRef.current;
			if (previousSlide && currentSlide) {
				prepareArrangementBeforeAutoAnimate(previousSlide, currentSlide);
			}
			syncArrangementSettledState(deckEl, currentSlide, previousSlide);
			previousSlideRef.current = currentSlide;
			setSlideEpoch((epoch) => epoch + 1);
			scheduleFinalizeAutoAnimateRest();
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
			const animateEvent = event as Event & {
				readonly data?: { readonly fromSlide?: HTMLElement; readonly toSlide?: HTMLElement };
				readonly sheet?: { innerHTML: string };
			};
			const fromSlide = animateEvent.data?.fromSlide;
			const toSlide = animateEvent.data?.toSlide;
			if (fromSlide && toSlide) {
				prepareArrangementBeforeAutoAnimate(fromSlide, toSlide);
			}
			const sheet = animateEvent.sheet;
			if (sheet && typeof sheet.innerHTML === "string") {
				const durationSeconds =
					typeof deck.getConfig().autoAnimateDuration === "number" ? deck.getConfig().autoAnimateDuration : 1;
				patchPresentationAutoAnimateStyleSheet(sheet, durationSeconds);
			}
			scheduleFinalizeAutoAnimateRest();
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
			options?.onRevealReady?.(deck);
			deck.on("beforeslidechange", onBeforeSlideChange);
			deck.on("slidechanged", onSlideChanged);
			deck.on("resize", onResize);
			deck.on("autoanimate", onAutoAnimate);
		});
		return () => {
			if (autoAnimateFinalizeTimer !== undefined) {
				clearTimeout(autoAnimateFinalizeTimer);
			}
			window.removeEventListener("hashchange", onWindowHashChange);
			deck.off("beforeslidechange", onBeforeSlideChange);
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
												presentation={presentation}
												chapter={chapter}
												sequence={sequence}
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
	const { readFileSync } = await import("node:fs");
	const { dirname, join } = await import("node:path");
	const { fileURLToPath } = await import("node:url");
	const globalsCssSource = readFileSync(
		join(dirname(fileURLToPath(import.meta.url)), "globals.css"),
		"utf8",
	);

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
			const deck = intro({ language: "de",
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
			const deck = intro({ language: "de",
				title: { full: ["A", "B", "C"], short: "Short" },
				description: { full: ["D1", "D2"], short: "D short" },
				goal: ["G1"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="einleitung--m0"]')) {
				expect(slide.classList.contains("presentation-arrangement--interactive")).toBe(true);
				expect(slide.classList.contains("presentation-arrangement--positioned")).toBe(false);
				expect(slide.querySelector(".presentation-arrangement-canvas")).toBeNull();
				expect(slide.querySelectorAll("[data-disposition-id]").length).toBeGreaterThan(0);
			}
			expect(globalsCssSource).toMatch(
				/\.presentation-arrangement--interactive\s*\{[^}]*overflow\s*:\s*visible/s,
			);
			expect(globalsCssSource).toMatch(
				/\.presentation-arrangement-canvas\s*>\s*\.presentation-interactive-disposition\[data-id\][\s\S]*:not\(\s*\.presentation-interactive-disposition--gesturing\s*\)[\s\S]*overflow\s*:\s*hidden/s,
			);
			expect(globalsCssSource).not.toMatch(
				/\.presentation-arrangement--interactive\s*\{[^}]*position\s*:\s*relative/s,
			);
		});

		it("applies muted opacity on layered description slide", () => {
			const deck = intro({ language: "de",
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
			const deck = intro({ language: "de",
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
			expect(slide("description")?.querySelector('div[data-id="title"].presentation-disposition-frame')).toBeNull();
			expect(slide("description")?.querySelectorAll('h2[data-id^="description"]').length).toBe(2);
			expect(slide("goal")?.querySelector('h2[data-id="description"]')?.textContent).toBe("D short");
			expect(slide("goal")?.querySelector('.presentation-interactive-disposition[data-id="description"]')).toBeNull();
			expect(slide("description")?.querySelector('.presentation-interactive-disposition[data-id="description"]')).toBeNull();
			expect(slide("goal")?.querySelector('h2[data-id^="goal"]')).toBeTruthy();
			const authorLines = slide("authors")?.querySelectorAll('h4[data-id^="authors--"]');
			expect(authorLines?.length).toBe(3);
			expect(slide("authors")?.getAttribute("data-auto-animate-id")).toMatch(/^einleitung--/);
			expect(slide("authors")?.querySelector(".presentation-intro-line")?.className).toContain("gap-x-");
			expect(slide("affiliations-1")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(1);
			expect(slide("affiliations-2")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
			expect(slide("affiliations-3")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
			expect(slide("affiliations-3")?.querySelectorAll('[data-id^="institutions--"]').length).toBe(3);
			expect(slide("affiliations-2")?.querySelector('h5[data-id="institutions"]')).toBeNull();
			expect(slide("affiliations-2")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("Uni");
			expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
			expect(slide("affiliations-3")?.querySelector(".presentation-affiliation-morph-source")).toBeNull();
			expect(
				slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.classList.contains("presentation-affiliation-row"),
			).toBe(true);
			expect(slide("affiliations-3")?.querySelector('[data-id="institutions--x"]')?.textContent).toContain("Chair X");
			expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"] [data-id="institutions--x"]')).toBeTruthy();
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
			expect(aff3?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
			expect(aff3?.querySelector('[data-id="institutions--x"] .opacity-20')).toBeNull();
		});

		it("applies title and secondary morph text sizes on intro slides", () => {
			const deck = intro({ language: "de",
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
			expectMorphClass('h1[data-id^="title"], h2[data-id^="title"]', "presentation-morph-text--title");
			expectMorphClass(
				'h2[data-id^="description"], p[data-id^="description"], h2[data-id="goal"], p[data-id="goal"]',
				"presentation-morph-text--secondary",
			);
			expect(container.querySelector('h2[data-id^="title"].presentation-morph-text--secondary')).toBeNull();
			expect(container.querySelector('h2[data-id="goal"].presentation-morph-text--title')).toBeNull();
		});

		it("does not use reveal fit-text on intro headings", () => {
			const deck = intro({ language: "de",
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
			const deck = intro({ language: "de",
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
				'.slides > section > section[data-auto-animate][data-auto-animate-id="einleitung--m0"]',
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
										participants: [{ id: "clip" }, { id: "doc" }],
										embodiments: [
											{ kind: "video", id: "clip--video", src: "/demo.mp4" },
											{ kind: "pdf", id: "doc--pdf", src: "/paper.pdf", page: 1 },
										],
										slides: [
											{
												arrangement: {
													id: "slide",
													dispositions: [
														{ participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
														{ participantId: "doc", embodimentId: "doc--pdf", emphasis: "active" },
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
										participants: [{ id: "box" }],
										embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
										slides: [
											{
												arrangement: {
													id: "placed",
													dispositions: [
														{
															participantId: "box",
															embodimentId: "box--main",
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

		it("renders split figure tiles with per-participant data-id and background crops", () => {
			const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
			const artifacts = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
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
										participants: artifacts.participants,
										embodiments: artifacts.embodiments,
										slides: [
											{
												arrangement: {
													id: "tiles",
													dispositions: artifacts.dispositions,
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
			const wrappers = container.querySelectorAll("[data-disposition-id]");
			expect(wrappers.length).toBe(4);
			const first = wrappers[0] as HTMLElement;
			expect(first.classList.contains("presentation-interactive-disposition")).toBe(true);
			expect(first.style.position).toBe("absolute");
			const tileFrame = first.querySelector(
				'.presentation-morph-slot--figure[data-id^="tile-r"]',
			) as HTMLElement;
			expect(tileFrame).toBeTruthy();
			expect(tileFrame.style.backgroundImage).toContain("/catalogue.png");
			expect(tileFrame.style.getPropertyValue("--presentation-figure-bg-size")).toBe("240% 240%");
			const tiles = [...container.querySelectorAll(".presentation-morph-slot--figure")] as HTMLElement[];
			for (const node of tiles) {
				const size = node.style.getPropertyValue("--presentation-figure-bg-size");
				const [width, height] = size.split(/\s+/);
				expect(width).toBe(height);
			}
			const positions = tiles.map((node) => node.style.getPropertyValue("--presentation-figure-bg-position"));
			expect(new Set(positions).size).toBe(4);
			expect(first.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
		});

		it("pairs catalogue tile morph anchors on interactive wrappers across slides", () => {
			const frameA = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
			const gridA = split({ source: "/catalogue.png", rows: 2, columns: 2, frame: frameA });
			const frameB = { x: 0.1, y: 0.15, width: 0.35, height: 0.3 };
			const positionsB = Object.fromEntries(
				gridA.dispositions.map((disposition, index) => [
					disposition.participantId,
					{
						x: frameB.x + (index % 2) * (frameB.width / 2),
						y: frameB.y + Math.floor(index / 2) * (frameB.height / 2),
						width: frameB.width / 2,
						height: frameB.height / 2,
					},
				]),
			);
			const gridB = {
				...gridA,
				dispositions: remapSplitDispositions(gridA.dispositions, positionsB),
			};
			const deck: Presentation = {
				id: "tile-morph",
				name: "Tile morph",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "media",
								thoughts: [
									{
										id: "media",
										participants: gridA.participants,
										embodiments: gridA.embodiments,
										slides: [
											{
												arrangement: {
													id: "catalogue",
													dispositions: gridA.dispositions,
												},
												transition: { kind: "morph" },
											},
											{
												arrangement: {
													id: "catalogue-focus",
													dispositions: gridB.dispositions,
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
			const sections = [...container.querySelectorAll("section[data-auto-animate-id]")] as HTMLElement[];
			expect(sections.length).toBeGreaterThanOrEqual(2);
			const fromSlide = sections.find((section) => section.title === "catalogue");
			const toSlide = sections.find((section) => section.title === "catalogue-focus");
			expect(fromSlide).toBeDefined();
			expect(toSlide).toBeDefined();
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
			const pairs = presentationAutoAnimateMatcher.call(host, fromSlide!, toSlide!);
			expect(pairs).toHaveLength(4);
			expect(pairs.every((pair) => pair.from.getAttribute("data-id") === pair.to.getAttribute("data-id"))).toBe(
				true,
			);
			expect(pairs[0]?.from.classList.contains("presentation-interactive-disposition")).toBe(true);
			expect(pairs[0]?.to.classList.contains("presentation-interactive-disposition")).toBe(true);
		});

		it("renders only the dispositions listed on a slide", () => {
			const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
			const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
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
										participants: grid.participants,
										embodiments: grid.embodiments,
										slides: [
											{
												arrangement: {
													id: "focus",
													dispositions: grid.dispositions.slice(0, 2),
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
			expect(container.querySelectorAll('[data-id^="tile-r"]').length).toBe(2);
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
										participants: [{ id: "catalogue-col1" }],
										embodiments: [
											{
												kind: "figure",
												id: "catalogue-col1--crop",
												src: "/catalogue.png",
												crop: { x: 0, y: 0, width: 0.5, height: 1 },
											},
										],
										slides: [
											{
												arrangement: {
													id: "focus",
													dispositions: [
														{
															participantId: "catalogue-col1",
															embodimentId: "catalogue-col1--crop",
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
			const slot = container.querySelector(
				'[data-id="catalogue-col1"].presentation-morph-slot--figure',
			) as HTMLElement | null;
			expect(slot?.style.backgroundImage).toContain("/catalogue.png");
			expect(slot?.querySelector("h2")).toBeNull();
			expect(slot?.style.left).toBe("35%");
			expect(slot?.style.getPropertyValue("--presentation-figure-bg-size")).toBe("200% 100%");
			expect(slot?.style.getPropertyValue("--presentation-figure-bg-position")).toBe("25% 50%");
		});

		it("rewrites non-uniform auto-animate scale() to a uniform zoom", () => {
			expect(patchAutoAnimateUniformScale("transform: translate(1px, 2px) scale(1.5, 2) !important;")).toBe(
				"transform: translate(1px, 2px) scale(2) !important;",
			);
		});

		it("appends morph ghost opacity 1→0 and morph-into 0→1 rules to the auto-animate sheet", () => {
			const sheet = { innerHTML: "transform: scale(1, 2);" };
			patchPresentationAutoAnimateStyleSheet(sheet, 0.8);
			expect(sheet.innerHTML).toContain("scale(2)");
			expect(sheet.innerHTML).toContain(
				'[data-auto-animate="running"] .presentation-target-ghost[data-auto-animate-target]',
			);
			expect(sheet.innerHTML).toContain("opacity: 1 !important");
			expect(sheet.innerHTML).toContain("presentation-target-ghost-fade-out 0.8s ease forwards !important");
			expect(sheet.innerHTML).toContain("presentation-morph-target-fade-in 0.8s ease forwards !important");
		});

		it("rests morph-source ghosts with opacity only so reveal can measure FLIP targets", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, resolve } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const cssPath = resolve(dirname(fileURLToPath(import.meta.url)), "globals.css");
			const css = readFileSync(cssPath, "utf8");
			const restRule = css.match(/\.reveal \.presentation-target-ghost \{[\s\S]*?\}/)?.[0] ?? "";
			expect(restRule).toContain("opacity: 0");
			expect(restRule).not.toContain("opacity: 0 !important");
			expect(restRule).not.toContain("visibility: hidden");
			const runningRule =
				css.match(
					/\.reveal \.slides section\[data-auto-animate="running"\] \.presentation-target-ghost \{[\s\S]*?\}/,
				)?.[0] ?? "";
			expect(runningRule).toContain("presentation-target-ghost-fade-out");
			expect(runningRule).not.toMatch(/opacity:\s*1\s*!important/);
		});

		it("resolvePresentationAssetUrl maps deck-relative paths through the Vite base", () => {
			expect(resolvePresentationAssetUrl("/bauteilbörse.png")).toBe("/bauteilbörse.png");
			expect(resolvePresentationAssetUrl("./bauteilbörse.png")).toBe("/bauteilbörse.png");
		});

		it("keeps figure selection chrome as a translucent overlay, not a solid fill", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, resolve } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "globals.css"), "utf8");
			expect(css).toContain(
				".presentation-interactive-disposition--kind-figure.presentation-interactive-disposition--selected",
			);
			expect(css).toContain(".presentation-morph-slot--figure::after");
			expect(css).not.toMatch(
				/\.presentation-interactive-disposition--kind-figure\.presentation-interactive-disposition--selected[\s\S]*?\.presentation-morph-slot--figure\s*\{[^}]*background-color:\s*var\(--color-primary\)/,
			);
		});

		it("uses uniform cover at rest and during auto-animate morph vars", () => {
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
			expect(wide["--presentation-figure-bg-position" as keyof typeof wide]).toBe("25% 50%");
			expect(square["--presentation-figure-bg-position" as keyof typeof square]).toBe("25% 50%");
		});

		it("assigns distinct crop background positions per split tile", () => {
			const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
			const tiles = splitFigureGrid({ rows: 2, columns: 2, frame });
			const embodiment = { kind: "figure" as const, src: "/catalogue.png" };
			const positions = tiles.map(
				(tile) =>
					figureCropBackgroundVars(embodiment, tile.crop, tile.position)[
						"--presentation-figure-bg-position" as keyof ReturnType<typeof figureCropBackgroundVars>
					],
			);
			expect(new Set(positions).size).toBe(4);
		});

		it("clearRevealAutoAnimateInlineLayout removes only FLIP transform from auto-animate targets", () => {
			const deckEl = document.createElement("div");
			const target = document.createElement("div");
			target.dataset.autoAnimateTarget = "0";
			target.style.left = "65%";
			target.style.transform = "translate(10px, 20px) scale(2)";
			deckEl.appendChild(target);
			clearRevealAutoAnimateInlineLayout(deckEl);
			expect(target.style.left).toBe("65%");
			expect(target.style.transform).toBe("");
		});

		it("finalizeRevealAutoAnimateRestState clears running so morph sources rest hidden", () => {
			const deckEl = document.createElement("div");
			deckEl.className = "reveal";
			const slide = document.createElement("section");
			slide.classList.add("present");
			slide.setAttribute("data-auto-animate", "running");
			const morphSource = document.createElement("div");
			morphSource.className = "presentation-target-ghost";
			morphSource.dataset.autoAnimateTarget = "0";
			slide.appendChild(morphSource);
			deckEl.appendChild(slide);
			document.body.appendChild(deckEl);
			finalizeRevealAutoAnimateRestState(deckEl);
			expect(slide.getAttribute("data-auto-animate")).toBe("");
			expect(morphSource.hasAttribute("data-auto-animate-target")).toBe(false);
			deckEl.remove();
		});

		it("matches auto-animate targets only by data-id", () => {
			const fromSlide = document.createElement("section");
			fromSlide.innerHTML =
				'<div data-id="catalogue-col1" class="presentation-morph-slot--figure"><h2>Rippenplatte</h2></div>';
			const toSlide = document.createElement("section");
			toSlide.innerHTML =
				'<div data-id="catalogue-col1" class="presentation-morph-slot--label"><h2>Rippenplatte</h2></div>';
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
										participants: [{ id: "catalogue-col1" }],
										embodiments: [
											{
												kind: "figure",
												id: "catalogue-col1--crop",
												src: "/catalogue.png",
												crop: { x: 0, y: 0, width: 0.5, height: 1 },
											},
										],
										slides: [
											{
												arrangement: {
													id: "focus",
													dispositions: [
														{
															participantId: "catalogue-col1",
															embodimentId: "catalogue-col1--crop",
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
										participants: [{ id: "catalogue-col1" }],
										embodiments: [
											{
												kind: "text",
												id: "catalogue-col1--label",
												lines: ["Rippenplatte"],
												level: "heading",
												morphRoot: "heading-line",
											},
										],
										slides: [
											{
												arrangement: {
													id: "labels",
													dispositions: [
														{
															participantId: "catalogue-col1",
															embodimentId: "catalogue-col1--label",
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
			expect(slot?.querySelector("h2")?.textContent).toBe("Rippenplatte");
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
		const sampleDeck = intro({ language: "de",
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
			const goalSlide = collectPresentationSlides(sampleDeck).find((slide) => slide.slide === "Ziel");
			syncPresentationSlideUrl(sampleDeck, { h: goalSlide!.h, v: goalSlide!.v });
			const url = new URL(window.location.href);
			expect(url.search).toBe("");
			expect(url.hash).toContain("folie=Ziel");
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
			const narrow = { x: 0.35, y: 0.4, width: 0.3, height: 0.1 };
			const movedX = translateDispositionRect(narrow, 0.12, 0);
			expect(movedX.x).toBeCloseTo(0.47);
			expect(movedX.y).toBeCloseTo(0.4);
			const unbounded = translateDispositionRect({ x: 0.8, y: 0.5, width: 0.2, height: 0.1 }, 0.5, -0.3);
			expect(unbounded.x).toBeCloseTo(1.3);
			expect(unbounded.y).toBeCloseTo(0.2);
			const resized = resizeDispositionRect(rect, "se", 0.2, 0.1);
			expect(resized.width).toBeCloseTo(0.6);
			expect(resized.height).toBeCloseTo(0.3);
		});

		it("starts flow manipulation at zero offset with measured size", () => {
			const measured = { x: 0.35, y: 0.4, width: 0.3, height: 0.08 };
			expect(flowDispositionManipulationRect(measured, undefined)).toEqual({
				x: 0,
				y: 0,
				width: 0.3,
				height: 0.08,
			});
		});

		it("detects flow pixel-offset transforms and maps them to section frames", () => {
			const measured = { x: 0.2, y: 0.3, width: 0.25, height: 0.1 };
			const offset = { x: 96, y: 40, width: 0.25, height: 0.1 };
			expect(isFlowPixelOffsetTransform(offset, measured)).toBe(true);
			expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.25, height: 0.1 }, measured)).toBe(true);
			expect(isFlowPixelOffsetTransform(offset, undefined)).toBe(true);
			expect(isFlowPixelOffsetTransform({ ...offset, width: 0.3 }, measured)).toBe(false);
			const section = document.createElement("section");
			section.style.width = "960px";
			section.style.height = "700px";
			document.body.appendChild(section);
			section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
			const sectionRect = flowPixelOffsetToSectionRect(measured, offset, section);
			document.body.removeChild(section);
			expect(sectionRect.x).toBeCloseTo(0.3);
			expect(sectionRect.y).toBeCloseTo(0.357, 2);
		});

		it("maps flow offsets to local translate pixels", () => {
			expect(flowDispositionOffsetStyle({ x: 96, y: 140, width: 0.3, height: 0.08 }).transform).toBe(
				"translate3d(96px, 140px, 0)",
			);
		});

		it("resolves arrangement canvas as placement container for positioned slides", () => {
			const section = document.createElement("section");
			const canvas = document.createElement("div");
			canvas.className = "presentation-arrangement-canvas";
			section.append(canvas);
			Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
			expect(dispositionPlacementContainer(section, false)).toBe(section);
			expect(dispositionPlacementContainer(section, true)).toBe(canvas);
		});

		it("maps pointer fractions via parent stack when nested reveal section has zero height", () => {
			const outer = document.createElement("section");
			const inner = document.createElement("section");
			outer.append(inner);
			Object.defineProperty(outer, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(outer, "offsetHeight", { value: 700, configurable: true });
			Object.defineProperty(inner, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(inner, "offsetHeight", { value: 0, configurable: true });
			outer.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
			inner.getBoundingClientRect = () => new DOMRect(0, 0, 960, 0);
			expect(dispositionPlacementContainer(inner, false)).toBe(outer);
			const fraction = clientToSectionFraction(inner, 480, 350, { clamp: false });
			expect(fraction.x).toBeCloseTo(0.5);
			expect(fraction.y).toBeCloseTo(0.5);
		});

		it("uses explicit chrome frame only for flow slides, not canvas-framed slides", () => {
			const rect = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
			expect(
				interactiveDispositionChromeStyle({
					selected: true,
					effectiveRect: rect,
					canvasFramed: false,
					fullscreen: false,
				}),
			).toEqual(transformFrameStyle(rect));
			expect(
				interactiveDispositionChromeStyle({
					selected: true,
					effectiveRect: rect,
					canvasFramed: true,
					fullscreen: false,
				}),
			).toBeUndefined();
		});

		it("detects flow pixel-offset transforms without measured rect", () => {
			const measured = { x: 0.2, y: 0.3, width: 0.25, height: 0.1 };
			expect(isFlowPixelOffsetTransform({ x: 12, y: -4, width: 0.25, height: 0.1 }, undefined)).toBe(
				true,
			);
			expect(isFlowPixelOffsetTransform({ x: 0, y: 0, width: 0.25, height: 0.1 }, undefined)).toBe(true);
			expect(isFlowPixelOffsetTransform({ x: 0.2, y: 0.3, width: 0.25, height: 0.1 }, undefined)).toBe(
				false,
			);
			expect(isFlowPixelOffsetTransform({ x: 0, y: 0, width: 0.25, height: 0.1 }, measured)).toBe(true);
		});

		it("maps flow pointer delta 1:1 with screen travel", () => {
			const section = document.createElement("section");
			section.className = "presentation-arrangement--interactive";
			document.body.appendChild(section);
			const delta = flowPointerDeltaToLocal(section, 100, 200, 150, 260);
			document.body.removeChild(section);
			expect(delta.dx).toBe(50);
			expect(delta.dy).toBe(60);
		});

		it("maps flow drag 1:1 when reveal stack layout is tall but drag target is slide-sized", () => {
			const reveal = document.createElement("div");
			reveal.className = "reveal";
			reveal.style.setProperty("--presentation-slide-width", "960");
			reveal.style.setProperty("--presentation-slide-height", "700");
			const stack = document.createElement("section");
			const inner = document.createElement("section");
			inner.className = "presentation-arrangement--interactive";
			const content = document.createElement("div");
			content.className = "presentation-interactive-disposition__content";
			inner.append(content);
			stack.append(inner);
			reveal.append(stack);
			document.body.append(reveal);
			Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(stack, "offsetHeight", { value: 4900, configurable: true });
			Object.defineProperty(inner, "offsetWidth", { value: 0, configurable: true });
			Object.defineProperty(inner, "offsetHeight", { value: 0, configurable: true });
			Object.defineProperty(content, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(content, "offsetHeight", { value: 120, configurable: true });
			stack.getBoundingClientRect = () => new DOMRect(0, 0, 480, 350);
			content.getBoundingClientRect = () => new DOMRect(0, 280, 480, 60);
			expect(slideCoordinateRoot(inner)).toBe(inner);
			const delta = flowPointerDeltaToLocal(content, 100, 200, 120, 220);
			document.body.removeChild(reveal);
			expect(delta.dx).toBe(20);
			expect(delta.dy).toBe(20);
		});

		it("does not treat sub-unit flow drag offsets as normalized slide frames", () => {
			const measured = { x: 0.35, y: 0.4, width: 0.3, height: 0.08 };
			expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 }, measured)).toBe(true);
			expect(isNormalizedSlideFrame({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 })).toBe(true);
			expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 }, undefined)).toBe(
				false,
			);
		});

		it("rejects unusable measured fractions", () => {
			expect(isUsableMeasuredRect({ x: 0.2, y: 0.3, width: 0.1, height: 0.1 })).toBe(true);
			expect(isUsableMeasuredRect({ x: 0.2, y: 0.3, width: 0.005, height: 0.1 })).toBe(false);
		});

		it("measures flow disposition bounds from morph nodes not full-width wrapper", () => {
			const section = document.createElement("section");
			const root = document.createElement("div");
			const heading = document.createElement("h2");
			heading.textContent = "Entwerfen mit Bestand";
			root.appendChild(heading);
			section.appendChild(root);
			document.body.appendChild(section);
			section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
			root.getBoundingClientRect = () => new DOMRect(0, 280, 960, 120);
			heading.getBoundingClientRect = () => new DOMRect(0, 300, 960, 80);
			const measured = measureDispositionBoundsInSection(root, section);
			document.body.removeChild(section);
			expect(measured?.width).toBeLessThan(0.5);
			expect(measured?.x).toBeGreaterThan(0.1);
		});

		it("measures ink bounds relative to the disposition wrapper for selection chrome", () => {
			const root = document.createElement("div");
			const heading = document.createElement("h2");
			heading.textContent = "Title";
			root.appendChild(heading);
			document.body.appendChild(root);
			root.getBoundingClientRect = () => new DOMRect(0, 280, 960, 120);
			heading.getBoundingClientRect = () => new DOMRect(330, 300, 300, 80);
			const inWrapper = measureDispositionBoundsInContainer(root, root);
			document.body.removeChild(root);
			expect(inWrapper?.x).toBeCloseTo(330 / 960);
			expect(inWrapper?.width).toBeCloseTo(300 / 960);
			expect(inWrapper?.y).toBeCloseTo(20 / 120);
		});

		it("uses tight ink bounds for block headings", () => {
			const heading = document.createElement("h2");
			heading.textContent = "Goal line";
			document.body.appendChild(heading);
			heading.getBoundingClientRect = () => new DOMRect(0, 200, 960, 48);
			const tight = tightElementBoundsRect(heading);
			document.body.removeChild(heading);
			expect(tight).not.toBeNull();
			expect(tight!.width).toBeLessThan(960);
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
			expect(full.rect).toEqual(SLIDE_INTERACTIVE_FULLSCREEN_FRAME);
			expect(full.stash).toEqual(a);
			const restored = toggleFullscreenRect(full.rect, full.stash);
			expect(restored.rect).toEqual(a);
		});

		it("uses uniform min-axis scale for interactive resize content", () => {
			const baseline = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
			const grown = { x: 0.2, y: 0.3, width: 0.6, height: 0.2 };
			expect(interactiveDispositionContentScale(grown, baseline)).toBeCloseTo(1.5);
			const stretched = { x: 0.2, y: 0.3, width: 0.6, height: 0.35 };
			expect(interactiveDispositionContentScale(stretched, baseline)).toBeCloseTo(1.5);
			expect(interactiveDispositionContentScale(baseline, baseline)).toBeNull();
			expect(
				interactiveDispositionContentScaleStyle(1.25).transform,
			).toBe("scale(1.25)");
		});

		it("builds one interactive placement per tile disposition", () => {
			const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
			const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
			const scope = buildResolutionScope([
				{ participants: grid.participants, embodiments: grid.embodiments },
			]);
			const resolved = resolveArrangement(scope, {
				id: "tiles",
				dispositions: grid.dispositions,
			});
			const layout = buildInteractiveSlideLayout("slide-1", resolved, true);
			expect(layout.placements).toHaveLength(4);
			expect(layout.rowBands).toHaveLength(0);
			expect(layout.placements.every((entry) => entry.sectionRect !== undefined)).toBe(true);
			expect(layout.placements.every((entry) => entry.revealMorphId !== undefined)).toBe(true);
		});

		it("keeps reveal morph data-id on canvas placements only", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "title" }, { id: "description" }],
					embodiments: [
						{ kind: "text", id: "title--full", lines: ["A"], level: "heading", morphRoot: "heading-line" },
						{
							kind: "text",
							id: "description--full",
							lines: ["Long description"],
							level: "heading",
							morphRoot: "heading-block",
						},
					],
				},
			]);
			const flow = buildInteractiveSlideLayout(
				"goal",
				resolveArrangement(scope, {
					id: "goal",
					dispositions: [
						{ participantId: "title", embodimentId: "title--full", emphasis: "muted" },
						{ participantId: "description", embodimentId: "description--full", emphasis: "muted" },
					],
				}),
				true,
			);
			expect(flow.placements.every((entry) => entry.revealMorphId === undefined)).toBe(true);
			const canvas = buildInteractiveSlideLayout(
				"tiles",
				resolveArrangement(scope, {
					id: "tiles",
					dispositions: [
						{
							participantId: "description",
							embodimentId: "description--full",
							emphasis: "active",
							position: { x: 0.1, y: 0.2, width: 0.8, height: 0.3 },
						},
					],
				}),
				true,
			);
			expect(canvas.placements[0]?.revealMorphId).toBe("description");
		});

		it("auto-animates catalogue focus into inline column labels", async () => {
			const {
				CATALOGUE_COL1,
				CATALOGUE_COL2,
				CATALOGUE_COL3,
				CATALOGUE_EMBODIMENT_COL1_LABEL,
				CATALOGUE_EMBODIMENT_COL2_LABEL,
				CATALOGUE_EMBODIMENT_COL3_LABEL,
				CATALOGUE_COLUMN_TILE_KEYS,
				CATALOGUE_SPLIT,
				catalogueFocusDispositions,
				columnLabelMorphFrom,
				inlineColumnLabelPosition,
				mediaEmbodiments,
				mediaParticipants,
			} = await import("@mit-bestand/praesentation/projektetage-spec");
			const deck: Presentation = {
				id: "projektetage-morph",
				name: "Morph",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "medien",
										participants: mediaParticipants,
										embodiments: mediaEmbodiments,
										slides: [
											{
												arrangement: {
													id: "catalogue",
													dispositions: CATALOGUE_SPLIT.dispositions,
												},
												transition: { kind: "morph" },
											},
											{
												arrangement: {
													id: "catalogue-focus",
													settleBeforeMorphTo: ["catalogue-labels"],
													dispositions: catalogueFocusDispositions(),
												},
												transition: { kind: "morph" },
											},
											{
												arrangement: {
													id: "catalogue-labels",
													dispositions: [
														{
															participantId: CATALOGUE_COL1,
															embodimentId: CATALOGUE_EMBODIMENT_COL1_LABEL,
															emphasis: "active",
															position: inlineColumnLabelPosition(0),
															morphFrom: columnLabelMorphFrom("col1", inlineColumnLabelPosition(0)),
														},
														{
															participantId: CATALOGUE_COL2,
															embodimentId: CATALOGUE_EMBODIMENT_COL2_LABEL,
															emphasis: "active",
															position: inlineColumnLabelPosition(1),
															morphFrom: columnLabelMorphFrom("col2", inlineColumnLabelPosition(1)),
														},
														{
															participantId: CATALOGUE_COL3,
															embodimentId: CATALOGUE_EMBODIMENT_COL3_LABEL,
															emphasis: "active",
															position: inlineColumnLabelPosition(2),
															morphFrom: columnLabelMorphFrom("col3", inlineColumnLabelPosition(2)),
														},
													],
												},
												transition: { kind: "morph" },
											},
										],
									},
								],
							},
						],
					},
				],
			};
			const mountRoot = document.createElement("div");
			document.body.appendChild(mountRoot);
			act(() => {
				mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			await new Promise((resolve) => setTimeout(resolve, 100));
			const revealEl = mountRoot.querySelector(".reveal") as HTMLElement;
			const focusSlide = revealEl.querySelector('section[title="catalogue-focus"]') as HTMLElement;
			const labelSlide = revealEl.querySelector('section[title="catalogue-labels"]') as HTMLElement;
			expect(focusSlide).toBeTruthy();
			expect(labelSlide).toBeTruthy();
			expect(focusSlide.getAttribute("data-auto-animate-id")).toBe(labelSlide.getAttribute("data-auto-animate-id"));
			prepareArrangementBeforeAutoAnimate(focusSlide, labelSlide);
			expect(focusSlide.classList.contains("presentation-arrangement--settled")).toBe(true);
			const host: AutoAnimateMatcherHost = {
				findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
					for (const element of fromScope.querySelectorAll<HTMLElement>(selector)) {
						const toElement = toScope.querySelector<HTMLElement>(
							`${selector}[data-id="${element.getAttribute("data-id")}"]`,
						);
						if (toElement) {
							pairs.push({ from: element, to: toElement });
						}
					}
				},
			};
			const pairs = presentationAutoAnimateMatcher.call(host, focusSlide, labelSlide);
			const columnIds = [CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3];
			const tileIds = [
				...CATALOGUE_COLUMN_TILE_KEYS.col1,
				...CATALOGUE_COLUMN_TILE_KEYS.col2,
				...CATALOGUE_COLUMN_TILE_KEYS.col3,
			];
			expect(pairs.length).toBeGreaterThanOrEqual(8);
			expect(
				tileIds.filter((id) =>
					pairs.some(
						(pair) =>
							pair.from.getAttribute("data-id") === id &&
							elementIsTargetGhostAnchor(pair.to),
					),
				).length,
			).toBeGreaterThanOrEqual(8);
			expect(labelSlide.querySelectorAll(".presentation-interactive-disposition.presentation-morph-target").length).toBe(3);
			expect(labelSlide.querySelectorAll(".presentation-interactive-disposition.presentation-morph-target[data-id]").length).toBe(0);
			expect(
				tileIds.every(
					(id) =>
						labelSlide.querySelector(`[data-id="${id}"]`)?.closest(".presentation-target-ghost") !== null,
				),
			).toBe(true);
			expect(labelSlide.querySelectorAll(".presentation-target-ghost").length).toBeGreaterThanOrEqual(8);
			const labelSlot = inlineColumnLabelPosition(2);
			const focusStuetze = focusSlide.querySelector(
				'[data-disposition-id^="catalogue-focus--Stütze"]',
			) as HTMLElement | null;
			const labelGhost = labelSlide.querySelector(
				'[data-disposition-id^="catalogue-labels--Stütze"].presentation-target-ghost',
			) as HTMLElement | null;
			expect(focusStuetze?.style.left).not.toBe(`${labelSlot.x * 100}%`);
			expect(labelGhost?.style.left).toBe(`${labelSlot.x * 100}%`);
			expect(labelGhost?.style.top).toBe(`${labelSlot.y * 100}%`);
			expect(labelGhost?.style.width).toBe(`${labelSlot.width * 100}%`);
			expect(labelGhost?.style.height).toBe(`${labelSlot.height * 100}%`);
			expect(labelGhost?.classList.contains("presentation-target-ghost")).toBe(true);
			expect(labelGhost?.getAttribute("data-disposition-id")).toContain("catalogue-labels");
			mountRoot.remove();
		});

		it("styles source ghosts hidden at rest and visible during auto-animate", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, resolve } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const cssPath = resolve(dirname(fileURLToPath(import.meta.url)), "globals.css");
			const css = readFileSync(cssPath, "utf8");
			expect(css).toContain(".reveal .presentation-source-ghost");
			expect(css).toContain(
				'section[data-auto-animate="pending"] .presentation-source-ghost',
			);
			expect(css).not.toContain('section[title="catalogue-focus"]');
		});

		it("shows catalogue full figure at rest with source ghosts and focus tiles visible", async () => {
			const { collectPresentationSlides, loadPresentationFromSlideGlob } =
				await import("@framework/presentation/core");
			type SlideFile = import("@framework/presentation/core").SlideFile;
			const { presentationMeta } = await import("@mit-bestand/praesentation/projektetage-spec");
			const slideModules = import.meta.glob<{ default: SlideFile }>(
				"../../../../../mit-bestand/präsentation/33.projektetage/slide/**/*.ts",
				{ eager: true },
			);
			const deck = loadPresentationFromSlideGlob(presentationMeta, slideModules);
			const mountRoot = document.createElement("div");
			document.body.appendChild(mountRoot);
			act(() => {
				mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			await new Promise((resolve) => setTimeout(resolve, 100));
			const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
			expect(catalogueSlide.classList.contains("presentation-arrangement--settled")).toBe(false);
			expect(catalogueSlide.hasAttribute("data-settle-before-morph-to")).toBe(false);
			const catalogueFigure = catalogueSlide.querySelector(
				".presentation-media-figure",
			) as HTMLImageElement | null;
			expect(catalogueFigure?.src).toContain("bauteilb");
			expect(catalogueSlide.querySelectorAll(".presentation-morph-one").length).toBe(1);
			const sourceGhosts = catalogueSlide.querySelectorAll(".presentation-source-ghost");
			expect(sourceGhosts.length).toBe(10);
			const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
			const focusSlots = focusSlide.querySelectorAll(".presentation-morph-slot--figure");
			expect(focusSlots.length).toBe(10);
			for (const slot of focusSlots) {
				expect(slot.classList.contains("presentation-target-ghost")).toBe(false);
				expect(slot.classList.contains("presentation-source-ghost")).toBe(false);
				const backgroundImage = (slot as HTMLElement).style.backgroundImage;
				expect(backgroundImage.length).toBeGreaterThan(0);
				expect(backgroundImage).toContain("bauteilb");
			}
			mountRoot.remove();
		});

		it("puts reveal data-id on catalogue tile wrappers for catalogue-to-focus morph", async () => {
			const { collectPresentationSlides, loadPresentationFromSlideGlob } =
				await import("@framework/presentation/core");
			type SlideFile = import("@framework/presentation/core").SlideFile;
			const { presentationMeta, CATALOGUE_FOCUS_TILES } = await import("@mit-bestand/praesentation/projektetage-spec");
			const slideModules = import.meta.glob<{ default: SlideFile }>(
				"../../../../../mit-bestand/präsentation/33.projektetage/slide/**/*.ts",
				{ eager: true },
			);
			const deck = loadPresentationFromSlideGlob(presentationMeta, slideModules);
			const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
			const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
			expect(catalogueRef).toBeDefined();
			expect(focusRef).toBeDefined();
			const mountRoot = document.createElement("div");
			document.body.appendChild(mountRoot);
			act(() => {
				mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			await new Promise((resolve) => setTimeout(resolve, 100));
			const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
			const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
			const tileId = CATALOGUE_FOCUS_TILES[0]!.participantId;
			const catalogueTile = catalogueSlide.querySelector(
				`.presentation-interactive-disposition[data-id="${tileId}"]`,
			) as HTMLElement;
			const focusTile = focusSlide.querySelector(
				`.presentation-interactive-disposition[data-id="${tileId}"]`,
			) as HTMLElement;
			expect(catalogueTile).toBeTruthy();
			expect(focusTile).toBeTruthy();
			expect(catalogueTile.querySelector(`[data-id="${tileId}"]`)).toBeNull();
			expect(catalogueSlide.getAttribute("data-auto-animate-id")).toBe(
				focusSlide.getAttribute("data-auto-animate-id"),
			);
			mountRoot.remove();
		});

		it("auto-animates catalogue tiles into focus layout", async () => {
			const { collectPresentationSlides, loadPresentationFromSlideGlob } =
				await import("@framework/presentation/core");
			type SlideFile = import("@framework/presentation/core").SlideFile;
			const { presentationMeta, CATALOGUE_FOCUS_TILES } = await import("@mit-bestand/praesentation/projektetage-spec");
			const slideModules = import.meta.glob<{ default: SlideFile }>(
				"../../../../../mit-bestand/präsentation/33.projektetage/slide/**/*.ts",
				{ eager: true },
			);
			const deck = loadPresentationFromSlideGlob(presentationMeta, slideModules);
			const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
			const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
			expect(catalogueRef).toBeDefined();
			expect(focusRef).toBeDefined();
			const mountRoot = document.createElement("div");
			document.body.appendChild(mountRoot);
			act(() => {
				mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			await new Promise((resolve) => setTimeout(resolve, 100));
			const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
			const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
			expect(catalogueSlide.getAttribute("data-auto-animate-id")).toBe(focusSlide.getAttribute("data-auto-animate-id"));
			expect(catalogueSlide.querySelectorAll(".presentation-source-ghost").length).toBe(10);
			catalogueSlide.setAttribute("data-auto-animate", "pending");
			const host: AutoAnimateMatcherHost = {
				findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
					for (const element of fromScope.querySelectorAll<HTMLElement>(selector)) {
						const toElement = toScope.querySelector<HTMLElement>(
							`${selector}[data-id="${element.getAttribute("data-id")}"]`,
						);
						if (toElement) {
							pairs.push({ from: element, to: toElement });
						}
					}
				},
			};
			const pairs = presentationAutoAnimateMatcher.call(host, catalogueSlide, focusSlide);
			const componentTileIds = CATALOGUE_FOCUS_TILES.map((tile) => tile.participantId);
			expect(componentTileIds.every((id) => pairs.some((pair) => pair.from.getAttribute("data-id") === id))).toBe(
				true,
			);
			mountRoot.remove();
		});

		it("places catalogue-labels target ghosts at inline label frames", async () => {
			const { collectPresentationSlides, loadPresentationFromSlideGlob } =
				await import("@framework/presentation/core");
			type SlideFile = import("@framework/presentation/core").SlideFile;
			const { presentationMeta, inlineColumnLabelPosition } =
				await import("@mit-bestand/praesentation/projektetage-spec");
			const slideModules = import.meta.glob<{ default: SlideFile }>(
				"../../../../../mit-bestand/präsentation/33.projektetage/slide/**/*.ts",
				{ eager: true },
			);
			const deck = loadPresentationFromSlideGlob(presentationMeta, slideModules);
			const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
			const labelRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilbeschriftungen");
			expect(focusRef).toBeDefined();
			expect(labelRef).toBeDefined();
			const mountRoot = document.createElement("div");
			document.body.appendChild(mountRoot);
			act(() => {
				mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			await new Promise((resolve) => setTimeout(resolve, 100));
			const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
			const labelSlide = mountRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
			const labelSlot = inlineColumnLabelPosition(2);
			const focusStuetze = focusSlide.querySelector(
				'[data-disposition-id^="catalogue-focus--Stütze"]',
			) as HTMLElement | null;
			const labelGhost = labelSlide.querySelector(
				'[data-disposition-id^="catalogue-labels--Stütze"].presentation-target-ghost',
			) as HTMLElement | null;
			expect(focusStuetze?.style.left).not.toBe(`${labelSlot.x * 100}%`);
			expect(labelGhost?.style.left).toBe(`${labelSlot.x * 100}%`);
			expect(labelGhost?.style.top).toBe(`${labelSlot.y * 100}%`);
			expect(labelGhost?.style.width).toBe(`${labelSlot.width * 100}%`);
			expect(labelGhost?.style.height).toBe(`${labelSlot.height * 100}%`);
			mountRoot.remove();
		});

		it("fires reveal auto-animate when advancing projektetage focus to labels", async () => {
			const { collectPresentationSlides, loadPresentationFromSlideGlob } =
				await import("@framework/presentation/core");
			type SlideFile = import("@framework/presentation/core").SlideFile;
			const { presentationMeta } = await import("@mit-bestand/praesentation/projektetage-spec");
			const slideModules = import.meta.glob<{ default: SlideFile }>(
				"../../../../../mit-bestand/präsentation/33.projektetage/slide/**/*.ts",
				{ eager: true },
			);
			const deck = loadPresentationFromSlideGlob(presentationMeta, slideModules);
			const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
			const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
			const labelRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilbeschriftungen");
			expect(catalogueRef).toBeDefined();
			expect(focusRef).toBeDefined();
			expect(labelRef).toBeDefined();
			const mountRoot = document.createElement("div");
			document.body.appendChild(mountRoot);
			let revealApi: Reveal.Api | undefined;
			let autoAnimateCount = 0;
			act(() => {
				mountPresentation(mountRoot, deck, {
					hash: false,
					slideNumber: false,
					surfaceChrome: false,
					onRevealReady: (api) => {
						revealApi = api;
					},
				});
			});
			const revealRoot = mountRoot.querySelector(".reveal") as HTMLElement;
			revealRoot.addEventListener("autoanimate", () => {
				autoAnimateCount += 1;
			});
			await new Promise<void>((resolve) => {
				const start = Date.now();
				const wait = (): void => {
					if (revealApi) {
						resolve();
						return;
					}
					if (Date.now() - start > 5000) {
						throw new Error("reveal.js did not become ready.");
					}
					setTimeout(wait, 50);
				};
				wait();
			});
			await revealApi!.slide(catalogueRef!.h, catalogueRef!.v);
			await new Promise((resolve) => setTimeout(resolve, 50));
			await revealApi!.slide(focusRef!.h, focusRef!.v);
			await new Promise((resolve) => setTimeout(resolve, 50));
			const afterCatalogueToFocus = autoAnimateCount;
			await revealApi!.slide(labelRef!.h, labelRef!.v);
			const autoAnimateDurationMs =
				(typeof revealApi!.getConfig().autoAnimateDuration === "number"
					? revealApi!.getConfig().autoAnimateDuration
					: 1) *
					1000 +
				120;
			await new Promise((resolve) => setTimeout(resolve, autoAnimateDurationMs));
			expect(afterCatalogueToFocus).toBeGreaterThan(0);
			expect(autoAnimateCount).toBeGreaterThan(afterCatalogueToFocus);
			const focusSlide = revealRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
			const labelSlide = revealRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
			expect(focusSlide.getAttribute("data-auto-animate-id")).toBe(labelSlide.getAttribute("data-auto-animate-id"));
			expect(labelSlide.getAttribute("data-auto-animate")).not.toBe("running");
			expect(labelSlide.querySelector("[data-auto-animate-target]")).toBeNull();
			expect(labelSlide.querySelectorAll(".presentation-target-ghost[data-auto-animate-target]").length).toBe(0);
			mountRoot.remove();
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
									participants: [{ id: "box" }],
									embodiments: [{ kind: "text", id: "box--main", lines: ["Hello"], level: "body" }],
									slides: [
										{
											arrangement: {
												id: "placed",
												dispositions: [
													{
														participantId: "box",
														embodimentId: "box--main",
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

		const mockClientRect = (
			element: Element,
			left: number,
			top: number,
			width: number,
			height: number,
		): void => {
			const rect = {
				left,
				top,
				width,
				height,
				right: left + width,
				bottom: top + height,
				x: left,
				y: top,
				toJSON: () => ({}),
			};
			element.getBoundingClientRect = () => rect as DOMRect;
		};

		const pointerDrag = (
			target: Element,
			fromX: number,
			fromY: number,
			toX: number,
			toY: number,
			pointerId = 1,
		): void => {
			target.dispatchEvent(
				new PointerEvent("pointerdown", {
					bubbles: true,
					cancelable: true,
					button: 0,
					clientX: fromX,
					clientY: fromY,
					pointerId,
				}),
			);
			window.dispatchEvent(
				new PointerEvent("pointermove", {
					bubbles: true,
					cancelable: true,
					clientX: toX,
					clientY: toY,
					pointerId,
				}),
			);
			window.dispatchEvent(
				new PointerEvent("pointerup", {
					bubbles: true,
					cancelable: true,
					clientX: toX,
					clientY: toY,
					pointerId,
				}),
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

		it("renders one interactive wrapper per tile disposition", () => {
			const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
			const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
			const deck: Presentation = {
				id: "split-interactive",
				name: "Split Interactive",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "split",
										participants: grid.participants,
										embodiments: grid.embodiments,
										slides: [
											{
												arrangement: {
													id: "tiles",
													dispositions: grid.dispositions,
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
			expect(container.querySelectorAll("[data-disposition-id]").length).toBe(4);
			expect(container.querySelectorAll(".presentation-interactive-row-band").length).toBe(0);
		});

		it("selects on click and deselects on empty slide click", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			expect(disposition.querySelector(".presentation-interaction-fullscreen")).toBeTruthy();
			act(() => {
				pointerClick(layer);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			act(() => {
				pointerClick(canvas, 8, 8);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
		});

		it("drags flow intro title disposition", () => {
			const deck = intro({ language: "de",
				title: { full: ["A", "B", "C"], short: "Short" },
				description: { full: ["D1"], short: "D short" },
				goal: ["G1"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: {
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
				},
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const section = container.querySelector(
				'.slides > section > section[title="title"]',
			) as HTMLElement;
			section.classList.add("present");
			const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
			const content = disposition.querySelector(
				".presentation-interactive-disposition__content",
			) as HTMLElement;
			const heading = disposition.querySelector("h2") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(disposition, 0, 280, 960, 120);
			mockClientRect(content, 0, 280, 960, 120);
			mockClientRect(heading, 330, 300, 300, 80);
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.querySelectorAll(".presentation-interaction-handle").length).toBe(8);
			act(() => {
				pointerDrag(disposition, 480, 320, 560, 360);
			});
			const dragMatch = content.style.transform.match(
				/translate3d\(([-\d.]+)px,\s*([-\d.]+)px/,
			);
			expect(dragMatch).not.toBeNull();
			expect(Number.parseFloat(dragMatch![1]!)).toBeCloseTo(80, 0);
			expect(Number.parseFloat(dragMatch![2]!)).toBeCloseTo(40, 0);
			expect(disposition.classList.contains("presentation-interactive-disposition--offset")).toBe(true);
			expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(false);
			expect(content.style.transform).toContain("translate");
			expect(disposition.style.transform).toBe("");
			expect(disposition.style.left).toBe("");
			expect(disposition.querySelectorAll(".presentation-interaction-handle").length).toBe(8);
			const chrome = disposition.querySelector(
				".presentation-interactive-disposition__chrome",
			) as HTMLElement;
			expect(chrome.isConnected).toBe(true);
			expect(chrome.style.left).toBe("");
			expect(content.style.left).not.toBe("");
			expect(content.style.width).not.toBe("");
			expect(chrome.querySelectorAll(".presentation-interaction-handle").length).toBe(8);
		});

		it("resizes flow disposition from se handle when nested reveal section has zero height", () => {
			const deck = intro({ language: "de",
				title: { full: ["A", "B", "C"], short: "Short" },
				description: { full: ["D1"], short: "D short" },
				goal: ["G1"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: {
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
				},
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const section = container.querySelector(
				'.slides > section > section[title="title"]',
			) as HTMLElement;
			const stack = section.parentElement as HTMLElement;
			section.classList.add("present");
			stack.classList.add("present");
			Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(stack, "offsetHeight", { value: 700, configurable: true });
			Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(section, "offsetHeight", { value: 0, configurable: true });
			const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
			const heading = disposition.querySelector("h2") as HTMLElement;
			mockClientRect(stack, 0, 0, 960, 700);
			mockClientRect(section, 0, 0, 960, 0);
			mockClientRect(disposition, 330, 300, 300, 80);
			mockClientRect(heading, 330, 300, 300, 80);
			act(() => {
				pointerClick(disposition);
			});
			const handle = disposition.querySelector(
				".presentation-interaction-handle--se",
			) as HTMLElement;
			act(() => {
				pointerDrag(handle, 620, 370, 720, 420);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
			expect(parseFloat(disposition.style.width)).toBeGreaterThan(15);
			expect(parseFloat(disposition.style.height)).toBeGreaterThan(5);
			expect(disposition.querySelector(".presentation-interactive-disposition__content")?.style.transform).toContain(
				"scale(",
			);
		});

		it("drags positioned disposition", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const content = disposition.querySelector(
				".presentation-interactive-disposition__content",
			) as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(disposition, 192, 210, 384, 140);
			act(() => {
				pointerDrag(disposition, 300, 280, 380, 320);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
			// 🔀 The wrapper owns the reveal `data-id` morph anchor, so the ephemeral drag must move the
			// wrapper frame itself (not just translate inner content). Auto-animate then morphs from the
			// dragged frame, including the ephemeral modification. The declared frame is centered first
			// (single box shifts +0.1/+0.1), then the 80px/40px drag adds +8.333%/+5.714%.
			expect(parseFloat(disposition.style.left)).toBeCloseTo(38.333, 1);
			expect(parseFloat(disposition.style.top)).toBeCloseTo(45.714, 1);
			expect(content.style.transform).toBe("");
		});

		it("keeps other canvas dispositions on their declared frames while one is dragged", () => {
			const twoBoxDeck: Presentation = {
				id: "interactive-dom-two",
				name: "Interactive DOM Two",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "placed",
										participants: [{ id: "left" }, { id: "right" }],
										embodiments: [
											{ kind: "text", id: "left--main", lines: ["Left"], level: "body" },
											{ kind: "text", id: "right--main", lines: ["Right"], level: "body" },
										],
										slides: [
											{
												arrangement: {
													id: "placed",
													dispositions: [
														{
															participantId: "left",
															embodimentId: "left--main",
															emphasis: "active",
															position: { x: 0.1, y: 0.3, width: 0.3, height: 0.2 },
														},
														{
															participantId: "right",
															embodimentId: "right--main",
															emphasis: "active",
															position: { x: 0.6, y: 0.3, width: 0.3, height: 0.2 },
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
				mountPresentation(container, twoBoxDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const dispositions = [...container.querySelectorAll("[data-disposition-id]")] as HTMLElement[];
			expect(dispositions).toHaveLength(2);
			const section = dispositions[0]!.closest(
				"section.presentation-arrangement--interactive",
			) as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(dispositions[0]!, 96, 210, 288, 140);
			mockClientRect(dispositions[1]!, 576, 210, 288, 140);
			const peerBefore = dispositions[1]!.style.left;
			act(() => {
				pointerDrag(dispositions[0]!, 240, 280, 400, 320);
			});
			// 🔀 Dragged disposition's wrapper (its morph anchor) follows the ephemeral frame...
			expect(parseFloat(dispositions[0]!.style.left)).toBeCloseTo(26.667, 1);
			// ...while peers keep their declared frames.
			expect(dispositions[1]!.style.left).toBe(peerBefore);
			expect(
				dispositions[0]!.querySelector(".presentation-interactive-disposition__content")?.style.transform,
			).toBe("");
		});

		it("click-drags only the newly targeted disposition when another stays selected", () => {
			const twoBoxDeck: Presentation = {
				id: "interactive-dom-click-drag",
				name: "Interactive DOM Click Drag",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "placed",
										participants: [{ id: "left" }, { id: "right" }],
										embodiments: [
											{ kind: "text", id: "left--main", lines: ["Left"], level: "body" },
											{ kind: "text", id: "right--main", lines: ["Right"], level: "body" },
										],
										slides: [
											{
												arrangement: {
													id: "placed",
													dispositions: [
														{
															participantId: "left",
															embodimentId: "left--main",
															emphasis: "active",
															position: { x: 0.1, y: 0.3, width: 0.3, height: 0.2 },
														},
														{
															participantId: "right",
															embodimentId: "right--main",
															emphasis: "active",
															position: { x: 0.6, y: 0.3, width: 0.3, height: 0.2 },
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
				mountPresentation(container, twoBoxDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const dispositions = [...container.querySelectorAll("[data-disposition-id]")] as HTMLElement[];
			const section = dispositions[0]!.closest(
				"section.presentation-arrangement--interactive",
			) as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(dispositions[0]!, 96, 210, 288, 140);
			mockClientRect(dispositions[1]!, 576, 210, 288, 140);
			act(() => {
				pointerClick(dispositions[0]!, 240, 280);
			});
			const leftBefore = dispositions[0]!.style.left;
			const rightBefore = dispositions[1]!.style.left;
			act(() => {
				pointerDrag(dispositions[1]!, 720, 280, 880, 320);
			});
			expect(dispositions[0]!.style.left).toBe(leftBefore);
			expect(dispositions[1]!.style.left).not.toBe(rightBefore);
		});

		it("shows tile disposition drag preview outside the declared frame without clipping", () => {
			const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
			const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
			const splitDeck: Presentation = {
				id: "interactive-dom-split-tiles",
				name: "Interactive DOM Split Tiles",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "tiles",
										participants: grid.participants,
										embodiments: grid.embodiments,
										slides: [
											{
												arrangement: {
													id: "tiles",
													dispositions: grid.dispositions,
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
				mountPresentation(container, splitDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const tile = container.querySelector(
				".presentation-arrangement-canvas > .presentation-interactive-disposition[data-disposition-id]",
			) as HTMLElement;
			expect(tile).toBeTruthy();
			const section = tile.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(tile, 96, 70, 384, 210);
			act(() => {
				pointerDrag(tile, 200, 140, 360, 260);
			});
			// 🔀 The tile wrapper (morph anchor) moves to the ephemeral frame so the morph starts there.
			expect(parseFloat(tile.style.left)).toBeCloseTo(26.667, 1);
			expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
			expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
			// Pinned keeps the dragged tile unclipped while it sits outside the declared frame.
			expect(getComputedStyle(tile).overflow).not.toBe("hidden");
			expect(
				tile.querySelector(".presentation-interactive-disposition__content")?.style.transform,
			).toBe("");
		});

		it("resizes positioned disposition from se handle", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(disposition, 192, 210, 384, 140);
			act(() => {
				pointerClick(disposition);
			});
			const handle = disposition.querySelector(
				".presentation-interaction-handle--se",
			) as HTMLElement;
			act(() => {
				pointerDrag(handle, 560, 340, 640, 400);
			});
			const content = disposition.querySelector(
				".presentation-interactive-disposition__content",
			) as HTMLElement;
			expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
			// 🔀 Resize grows the wrapper frame (the morph anchor) itself; the content fills it at 100%
			// rather than carrying a scaled inline size, so auto-animate morphs from the resized frame.
			expect(parseFloat(disposition.style.width)).toBeCloseTo(48.333, 1);
			expect(parseFloat(disposition.style.height)).toBeCloseTo(28.571, 1);
			expect(content.style.width).toBe("");
			expect(content.style.height).toBe("");
			const chrome = disposition.querySelector(
				".presentation-interactive-disposition__chrome",
			) as HTMLElement;
			expect(chrome.isConnected).toBe(true);
			expect(chrome.querySelectorAll(".presentation-interaction-handle").length).toBe(8);
		});

		it("aligns canvas-framed chrome with the disposition wrapper on the arrangement canvas", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			const frame = disposition.querySelector(".presentation-disposition-frame") as HTMLElement;
			mockClientRect(section, 0, 0, 1200, 900);
			mockClientRect(canvas, 120, 100, 960, 700);
			const frameBox = { left: 408, top: 380, width: 384, height: 140 };
			mockClientRect(disposition, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
			mockClientRect(frame, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(
				true,
			);
			const chrome = disposition.querySelector(
				".presentation-interactive-disposition__chrome",
			) as HTMLElement;
			mockClientRect(chrome, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
			const chromeRect = chrome.getBoundingClientRect();
			const wrapperRect = disposition.getBoundingClientRect();
			expect(chromeRect.left).toBeCloseTo(wrapperRect.left, 0);
			expect(chromeRect.top).toBeCloseTo(wrapperRect.top, 0);
			expect(chromeRect.width).toBeCloseTo(wrapperRect.width, 0);
			expect(chromeRect.height).toBeCloseTo(wrapperRect.height, 0);
		});

		it("toggles slide fullscreen on a canvas-framed disposition", () => {
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
			expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
			expect(disposition.classList.contains("presentation-interactive-disposition--fullscreen")).toBe(true);
			expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_FULLSCREEN_FRAME.width * 100}%`);
			expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_FULLSCREEN_FRAME.height * 100}%`);
			const fullscreenOn = disposition.querySelector(
				".presentation-interaction-fullscreen",
			) as HTMLButtonElement;
			expect(fullscreenOn.getAttribute("aria-pressed")).toBe("true");
			act(() => {
				fullscreenOn.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--fullscreen")).toBe(false);
			const fullscreenOff = disposition.querySelector(
				".presentation-interaction-fullscreen",
			) as HTMLButtonElement;
			expect(fullscreenOff.getAttribute("aria-pressed")).toBe("false");
		});

		it("drops nested pdf frame and uses slide-fullscreen sizing when toggling fullscreen", () => {
			const deck: Presentation = {
				id: "pdf-fullscreen",
				name: "Pdf Fullscreen",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "media",
										participants: [{ id: "thesis" }],
										embodiments: [
											{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 },
										],
										slides: [
											{
												arrangement: {
													id: "media",
													dispositions: [
														{
															participantId: "thesis",
															embodimentId: "thesis--doc",
															emphasis: "active",
															position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
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
			const disposition = container.querySelector(
				".presentation-interactive-disposition--kind-pdf",
			) as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(disposition, 96, 385, 768, 280);
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.querySelector(".presentation-disposition-frame")).toBeTruthy();
			const fullscreen = disposition.querySelector(
				".presentation-interaction-fullscreen",
			) as HTMLButtonElement;
			const content = disposition.querySelector(
				".presentation-interactive-disposition__content",
			) as HTMLElement;
			mockClientRect(content, 48, 52, 864, 595);
			act(() => {
				fullscreen.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--fullscreen")).toBe(true);
			expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_FULLSCREEN_FRAME.width * 100}%`);
			expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_FULLSCREEN_FRAME.height * 100}%`);
			expect(disposition.querySelector(".presentation-disposition-frame")).toBeNull();
			const pageCanvas = disposition.querySelector(
				".presentation-media-pdf canvas",
			) as HTMLCanvasElement | null;
			if (pageCanvas) {
				expect(pageCanvas.height).toBeGreaterThan(400);
			}
			expect(globalsCssSource).toMatch(
				/\.presentation-interactive-disposition--kind-pdf\.presentation-interactive-disposition--fullscreen[\s\S]*\.presentation-media-pdf-document[\s\S]*height\s*:\s*100%/s,
			);
		});

		it("toggles slide fullscreen on a cropped figure tile disposition", () => {
			const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
			const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
			const deck: Presentation = {
				id: "split-fullscreen",
				name: "Split Fullscreen",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "split",
										participants: grid.participants,
										embodiments: grid.embodiments,
										slides: [
											{
												arrangement: {
													id: "tiles",
													dispositions: grid.dispositions,
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
			const tile = container.querySelector("[data-disposition-id]") as HTMLElement;
			const canvas = tile.closest(".presentation-arrangement-canvas") as HTMLElement;
			act(() => {
				pointerClick(tile);
			});
			const fullscreen = tile.querySelector(".presentation-interaction-fullscreen") as HTMLButtonElement;
			act(() => {
				fullscreen.click();
			});
			expect(tile.classList.contains("presentation-interactive-disposition--fullscreen")).toBe(true);
			expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
			expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(false);
			expect(tile.style.position).toBe("absolute");
			expect(tile.style.width).toBe(`${SLIDE_INTERACTIVE_FULLSCREEN_FRAME.width * 100}%`);
			expect(tile.style.height).toBe(`${SLIDE_INTERACTIVE_FULLSCREEN_FRAME.height * 100}%`);
			expect(canvas.contains(tile)).toBe(true);
			expect(globalsCssSource).toMatch(
				/\.presentation-interactive-disposition--fullscreen:not\(\.presentation-interactive-disposition--offset\)[\s\S]*\.presentation-figure-crop-fill[\s\S]*width\s*:\s*100%\s*!important/s,
			);
			act(() => {
				fullscreen.click();
			});
			expect(tile.classList.contains("presentation-interactive-disposition--fullscreen")).toBe(false);
			expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
		});
	});
}
//#endregion 🧪Tests
