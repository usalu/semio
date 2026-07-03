// #region 🧲Header
/** @emoji 📽 React + reveal.js renderer for `@semio-tech/framework-presentation-core` declarative decks. */
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
    Arrangement,
    FigureEmbodiment,
    FigureMosaicGrid,
    IframeEmbodiment,
    JsonEmbodiment,
    MarkdownEmbodiment,
    MediaTeaser,
    ParticipantEmphasis,
    Slide,
    PdfEmbodiment,
    Presentation,
    RenderSlide,
    Chapter,
    ResolvedDisposition,
    Sequence,
    TextEmbodiment,
    Thought,
    VideoEmbodiment
} from "@semio-tech/framework-presentation-core";
import {
    abbreviateAuthorFirstName,
    affiliationLineName,
    buildResolutionScope,
    centerResolvedArrangement,
    collectPresentationSlides,
    expandThoughtSlides,
    formatPresentationUrlHash,
    intro,
    isIntroArrangementId,
    morphId,
    parsePresentationSlideHash,
    presentationLanguage,
    presentationSlideAt,
    resolveMediaScrollOrigin,
    resolutionScopeForArrangement,
    resolveArrangement,
    resolveEmbodiment,
    resolveTextMorphRoot,
    remapSplitDispositions,
    split,
    splitFigureGrid,
    unionDispositionPositions
} from "@semio-tech/framework-presentation-core";
import {
    applyElementsSurfaceChrome,
    Expertise,
    Icon,
    Scrollable,
    SelectionMarquee,
    type ElementsSurfaceChromeInput,
} from "@semio-tech/ui-react";
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
import { createRoot, type Root } from "react-dom/client";
import { Document, Page, pdfjs } from "react-pdf";
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import "./globals.css";
import { compileMarkdownToHtml } from "./markdown.ts";
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
    IframeEmbodiment,
    JsonEmbodiment,
    MarkdownEmbodiment,
    MediaTeaser,
    MediaScrollOrigin,
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
} from "@semio-tech/framework-presentation-core";

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
    resolveMediaScrollOrigin,
    resolveTextMorphRoot,
    split,
    splitFigureGrid,
    tile,
    unionSourceCrops,
} from "@semio-tech/framework-presentation-core";
export type {
    MorphFromSlot,
    PresentationLanguageKind,
    PresentationSlideBookmark,
    PresentationSlideBookmarkParamKeys,
    PresentationSlideRef,
    RenderSlide,
    Slide,
    TextMorphRoot
} from "@semio-tech/framework-presentation-core";
export { Expertise } from "@semio-tech/ui-react";

//#region 🔖MountOptions
/** @emoji ⚙️ Reveal.js and @semio-tech/ui-react surface chrome options for {@link mountPresentation}. */
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

//#region 🔖RevealMorph
/** @emoji 👻 reveal.js-only morph companion role (not part of presentation core). */
export type RevealMorphCompanionKind = "source" | "target";

/** @emoji ✅ Resolved disposition plus optional reveal.js morph companion metadata. */
export interface RevealResolvedDisposition extends ResolvedDisposition {
	readonly revealMorphCompanion?: RevealMorphCompanionKind;
	/** @emoji 📐 Source slide frame for target ghosts: paired with {@link ResolvedDisposition.position} for frame and crop morph during auto-animate. */
	readonly revealMorphFromFrame?: DispositionPosition;
	/** @emoji 📐 Target slide frame for source tiles: paired with {@link ResolvedDisposition.position} when the next slide morphFrom references this participant. */
	readonly revealMorphToFrame?: DispositionPosition;
	/** @emoji 📐 Previous-slide morphTo slot frame (catalogue grid) for crop morph into {@link ResolvedDisposition.position}. */
	readonly revealMorphFromMorphToFrame?: DispositionPosition;
}

function primaryDispositionByParticipant(arrangement: Arrangement): Map<string, { readonly participantId: string; readonly embodimentId: string; readonly emphasis: ParticipantEmphasis }> {
	const map = new Map<string, { readonly participantId: string; readonly embodimentId: string; readonly emphasis: ParticipantEmphasis }>();
	for (const disposition of arrangement.dispositions) {
		if (!map.has(disposition.participantId)) {
			map.set(disposition.participantId, disposition);
		}
	}
	return map;
}

function dispositionByParticipant(arrangement: Arrangement): Map<string, Disposition> {
	const map = new Map<string, Disposition>();
	for (const disposition of arrangement.dispositions) {
		if (!map.has(disposition.participantId)) {
			map.set(disposition.participantId, disposition);
		}
	}
	return map;
}

function revealMorphCompanionFromMorphFrom(
	scope: ReturnType<typeof buildResolutionScope>,
	sourceSlide: Slide,
	arrangement: Arrangement,
	options?: { readonly morphLineTargets?: boolean },
): RevealResolvedDisposition[] {
	const sourceByParticipant = primaryDispositionByParticipant(sourceSlide.arrangement);
	const sourceDeclByParticipant = dispositionByParticipant(sourceSlide.arrangement);
	const morphLineTargets = options?.morphLineTargets ?? true;
	const companions: RevealResolvedDisposition[] = [];
	for (const disposition of arrangement.dispositions) {
		for (const slot of disposition.morphFrom ?? []) {
			const sourceDisposition = sourceByParticipant.get(slot.participantId);
			const sourceDecl = sourceDeclByParticipant.get(slot.participantId);
			const participant = scope.participants.get(slot.participantId);
			if (!participant) {
				throw new Error(`morphFrom slot references unknown participant "${slot.participantId}".`);
			}
			const embodimentId = slot.embodimentId ?? sourceDisposition?.embodimentId;
			if (!embodimentId) {
				throw new Error(
					`morphFrom slot for "${slot.participantId}" needs embodimentId (or a source disposition with embodimentId).`,
				);
			}
			companions.push({
				participant,
				embodiment: resolveEmbodiment(scope, embodimentId),
				emphasis: sourceDisposition?.emphasis ?? "active",
				embodimentId,
				morphId: morphId(participant.id),
				position: slot.position,
				revealMorphFromFrame: sourceDecl?.position,
				revealMorphCompanion: "target",
			});
		}
	}
	return companions;
}

function revealMorphCompanionFromMorphTo(
	scope: ReturnType<typeof buildResolutionScope>,
	targetSlide: Slide,
	arrangement: Arrangement,
): RevealResolvedDisposition[] {
	const targetByParticipant = primaryDispositionByParticipant(targetSlide.arrangement);
	const companions: RevealResolvedDisposition[] = [];
	for (const disposition of arrangement.dispositions) {
		for (const slot of disposition.morphTo ?? []) {
			const targetDisposition = targetByParticipant.get(slot.participantId);
			const participant = scope.participants.get(slot.participantId);
			if (!participant) {
				throw new Error(`morphTo slot references unknown participant "${slot.participantId}".`);
			}
			const embodimentId = slot.embodimentId ?? targetDisposition?.embodimentId;
			if (!embodimentId) {
				throw new Error(
					`morphTo slot for "${slot.participantId}" needs embodimentId (or a target disposition with embodimentId).`,
				);
			}
			companions.push({
				participant,
				embodiment: resolveEmbodiment(scope, embodimentId),
				emphasis: targetDisposition?.emphasis ?? disposition.emphasis,
				embodimentId,
				morphId: morphId(participant.id),
				position: slot.position,
				revealMorphCompanion: "source",
			});
		}
	}
	return companions;
}

function revealMorphToFrameByParticipant(nextSlide: Slide): Map<string, DispositionPosition> {
	const map = new Map<string, DispositionPosition>();
	for (const disposition of nextSlide.arrangement.dispositions) {
		for (const slot of disposition.morphFrom ?? []) {
			if (!map.has(slot.participantId)) {
				map.set(slot.participantId, slot.position);
			}
		}
	}
	return map;
}

function revealMorphFromMorphToFrameByParticipant(previousSlide: Slide): Map<string, DispositionPosition> {
	const map = new Map<string, DispositionPosition>();
	for (const disposition of previousSlide.arrangement.dispositions) {
		for (const slot of disposition.morphTo ?? []) {
			if (!map.has(slot.participantId)) {
				map.set(slot.participantId, slot.position);
			}
		}
	}
	return map;
}

/** @emoji 🔀 Resolves an arrangement and appends reveal.js morph companions for one-to-many / many-to-one. */
export function resolveRevealArrangement(
	scope: ReturnType<typeof buildResolutionScope>,
	arrangement: Arrangement,
	context: { readonly previousSlide?: Slide; readonly nextSlide?: Slide },
): RevealResolvedDisposition[] {
	const resolved = resolveArrangement(scope, arrangement) as RevealResolvedDisposition[];
	const companions: RevealResolvedDisposition[] = [];
	if (context.previousSlide !== undefined) {
		companions.push(...revealMorphCompanionFromMorphFrom(scope, context.previousSlide, arrangement));
	}
	const hasMorphTo = arrangement.dispositions.some((disposition) => (disposition.morphTo?.length ?? 0) > 0);
	if (context.nextSlide !== undefined && hasMorphTo) {
		companions.push(...revealMorphCompanionFromMorphTo(scope, context.nextSlide, arrangement));
	}
	const morphToFrames =
		context.nextSlide !== undefined ? revealMorphToFrameByParticipant(context.nextSlide) : undefined;
	const morphFromMorphToFrames =
		context.previousSlide !== undefined
			? revealMorphFromMorphToFrameByParticipant(context.previousSlide)
			: undefined;
	const withMorphFrames = resolved.map((disposition) => {
		if (disposition.revealMorphCompanion !== undefined || disposition.position === undefined) {
			return disposition;
		}
		let updated = disposition;
		const revealMorphToFrame = morphToFrames?.get(disposition.participant.id);
		if (revealMorphToFrame !== undefined) {
			updated = { ...updated, revealMorphToFrame };
		}
		const revealMorphFromMorphToFrame = morphFromMorphToFrames?.get(disposition.participant.id);
		if (revealMorphFromMorphToFrame !== undefined) {
			updated = { ...updated, revealMorphFromMorphToFrame };
		}
		return updated;
	});
	return [...withMorphFrames, ...companions];
}

function isRevealMorphCompanionOnly(disposition: RevealResolvedDisposition): boolean {
	return disposition.revealMorphCompanion !== undefined;
}

function visibleRevealArrangementPositions(resolved: readonly RevealResolvedDisposition[]): DispositionPosition[] {
	const positions: DispositionPosition[] = [];
	for (const disposition of resolved) {
		if (isRevealMorphCompanionOnly(disposition)) {
			continue;
		}
		if (disposition.style?.opacity === 0) {
			continue;
		}
		if (disposition.position) {
			positions.push(disposition.position);
		}
	}
	return positions;
}

/** @emoji ⊕ Centers visible placements; omits reveal-only morph companions. */
export function centerRevealResolvedArrangement(resolved: readonly RevealResolvedDisposition[]): RevealResolvedDisposition[] {
	const positions = visibleRevealArrangementPositions(resolved);
	if (positions.length === 0) {
		return [...resolved];
	}
	const bounds = unionDispositionPositions(positions);
	const offset = {
		x: (1 - bounds.width) / 2 - bounds.x,
		y: (1 - bounds.height) / 2 - bounds.y,
		width: 0,
		height: 0,
	};
	const epsilon = 1e-6;
	if (Math.abs(offset.x) < epsilon && Math.abs(offset.y) < epsilon) {
		return [...resolved];
	}
	return resolved.map((disposition) => {
		if (!disposition.position || isRevealMorphCompanionOnly(disposition)) {
			return disposition;
		}
		return {
			...disposition,
			position: {
				x: disposition.position.x + offset.x,
				y: disposition.position.y + offset.y,
				width: disposition.position.width,
				height: disposition.position.height,
			},
		};
	});
}
//#endregion 🔖RevealMorph

//#region 🔖ArrangementSettled
/** @emoji 🔗 True when two slide sections share the same reveal.js auto-animate run id. */
export function slidesShareAutoAnimateId(
	fromSlide: HTMLElement | null | undefined,
	toSlide: HTMLElement | null | undefined,
): boolean {
	const fromId = fromSlide?.getAttribute("data-auto-animate-id");
	const toId = toSlide?.getAttribute("data-auto-animate-id");
	return fromId !== null && fromId !== undefined && fromId === toId;
}

/** @emoji ⏳ True while reveal.js auto-animate is pending or running on one section. */
export function isSectionAutoAnimating(sectionEl: HTMLElement): boolean {
	const state = sectionEl.getAttribute("data-auto-animate");
	return state === "pending" || state === "running";
}

/** @emoji ⏳ True while reveal.js auto-animate is measuring or running on this slide (or any slide in the deck). */
export function isRevealSlideAutoAnimating(sectionEl: HTMLElement): boolean {
	if (isSectionAutoAnimating(sectionEl)) {
		return true;
	}
	const deck = sectionEl.closest(".reveal");
	if (!(deck instanceof HTMLElement)) {
		return false;
	}
	return (
		deck.querySelector('section[data-auto-animate="pending"], section[data-auto-animate="running"]') !==
		null
	);
}

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
	const selectors = ["[data-auto-animate-target]"];
	for (const selector of selectors) {
		for (const element of deckEl.querySelectorAll<HTMLElement>(selector)) {
			for (const prop of flipProps) {
				element.style.removeProperty(prop);
			}
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
	clearManyToOneMorphArrangementClass(deckEl);
	const presentSlide = deckEl.querySelector<HTMLElement>("section.present");
	const introFlowPresent = presentSlide !== null && isIntroFlowSlide(presentSlide);
	if (!introFlowPresent) {
		clearRevealAutoAnimateInlineLayout(deckEl);
		for (const element of deckEl.querySelectorAll<HTMLElement>("[data-auto-animate-target]")) {
			delete element.dataset.autoAnimateTarget;
		}
	}
	if (
		presentSlide?.classList.contains("presentation-arrangement--settled") &&
		presentSlide.getAttribute("data-auto-animate") !== "pending" &&
		presentSlide.getAttribute("data-auto-animate") !== "running"
	) {
		presentSlide.classList.remove("presentation-arrangement--settled");
	}
}

/** @emoji 🔀 Slide class while auto-animating a `data-settle-before-morph-to` many-to-one run (focus tiles → label ghosts). */
export const PRESENTATION_MANY_TO_ONE_MORPH_CLASS = "presentation-arrangement--many-to-one-morph";

/** @emoji 🔀 True when `fromSlide` settles into `toSlide` via `data-settle-before-morph-to` (arrangement id on `title`). */
export function isManyToOneMorphTransition(fromSlide: HTMLElement, toSlide: HTMLElement): boolean {
	const settleBefore = fromSlide.getAttribute("data-settle-before-morph-to");
	if (!settleBefore) {
		return false;
	}
	const toId = toSlide.getAttribute("title");
	if (!toId) {
		return false;
	}
	return settleBefore.split(",").some((entry) => entry.trim() === toId);
}

/** @emoji 🧹 Clears {@link PRESENTATION_MANY_TO_ONE_MORPH_CLASS} from all slides in the deck. */
export function clearManyToOneMorphArrangementClass(deckEl: HTMLElement): void {
	for (const slide of deckEl.querySelectorAll<HTMLElement>(
		`section.${PRESENTATION_MANY_TO_ONE_MORPH_CLASS}`,
	)) {
		slide.classList.remove(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
	}
}

/** @emoji 🔀 Marks the active many-to-one morph run on `fromSlide` and `toSlide` only. */
export function syncManyToOneMorphArrangementClass(
	deckEl: HTMLElement,
	fromSlide: HTMLElement | null,
	toSlide: HTMLElement | null,
): void {
	clearManyToOneMorphArrangementClass(deckEl);
	if (fromSlide && toSlide && isManyToOneMorphTransition(fromSlide, toSlide)) {
		fromSlide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
		toSlide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
	}
}

/** @emoji ⏳ Prepares settle + many-to-one frame/crop morph before reveal auto-animate measures FLIP. */
export function prepareArrangementBeforeAutoAnimate(fromSlide: HTMLElement, toSlide: HTMLElement): void {
	const deckEl = fromSlide.closest(".reveal");
	if (deckEl instanceof HTMLElement) {
		syncManyToOneMorphArrangementClass(deckEl, fromSlide, toSlide);
	}
	if (!isManyToOneMorphTransition(fromSlide, toSlide)) {
		return;
	}
	fromSlide.classList.add("presentation-arrangement--settled");
	void fromSlide.offsetHeight;
	void toSlide.offsetHeight;
	syncManyToOneGhostMorphFramesFromDom(fromSlide, toSlide);
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

function elementIsLabelMorphSource(element: HTMLElement): boolean {
	return element.classList.contains("presentation-affiliation-morph-source");
}

/** @emoji 🎯 True when this node may be a reveal.js auto-animate pair endpoint (intro wrapper or canvas-framed tile). */
export function isRevealAutoAnimatePairSource(element: HTMLElement): boolean {
	if (!element.hasAttribute("data-id")) {
		return false;
	}
	if (element.classList.contains("presentation-interactive-disposition")) {
		return element.classList.contains("presentation-interactive-disposition--canvas-framed");
	}
	if (element.closest(".presentation-interactive-disposition") !== null) {
		return element.matches("h1, h2, h3, h4, h5, h6, p, img, video");
	}
	return true;
}

/** @emoji 📐 Slide-local ink box for reveal.js `measure` (avoids viewport `getBoundingClientRect` fly-in when `center: true`). */
export function revealInkMeasureForAutoAnimate(element: HTMLElement): {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
} {
	const slide = element.closest(
		"section.presentation-arrangement--interactive, section[data-auto-animate]",
	);
	if (!(slide instanceof HTMLElement)) {
		const rect = element.getBoundingClientRect();
		return { x: rect.left, y: rect.top, width: rect.width, height: rect.height };
	}
	const slideBounds = slideLayoutBounds(slide);
	const ink = tightElementBoundsRect(element) ?? element.getBoundingClientRect();
	return {
		x: ink.left - slideBounds.left,
		y: ink.top - slideBounds.top,
		width: ink.width,
		height: ink.height,
	};
}

function isIntroFlowSlide(slide: HTMLElement): boolean {
	return (
		slide.classList.contains("presentation-arrangement--intro") &&
		!slide.classList.contains("presentation-arrangement--positioned")
	);
}

/** @emoji 🎯 reveal.js auto-animate options for canvas morph text (translate only). */
export function revealTextAutoAnimatePairOptions(
	_from: HTMLElement,
	_fromSlide: HTMLElement,
	_toSlide: HTMLElement,
): Record<string, unknown> {
	return {
		scale: false,
		measure: revealInkMeasureForAutoAnimate,
	};
}

function elementIsFigureMorphSlot(element: HTMLElement): boolean {
	return (
		element.classList.contains("presentation-morph-slot--figure") ||
		element.closest(".presentation-morph-slot--figure") !== null ||
		element.querySelector(".presentation-morph-slot--figure") !== null
	);
}

function elementIsInteractiveFigureDisposition(element: HTMLElement): boolean {
	return (
		element.classList.contains("presentation-interactive-disposition") &&
		elementIsFigureMorphSlot(element) &&
		!elementIsSourceGhostAnchor(element) &&
		!elementIsTargetGhostAnchor(element)
	);
}

/** @emoji 🎯 Picks the `to` morph anchor for one `data-id` (focus tile → label target ghost, catalogue source → focus tile). */
export function resolveMorphAutoAnimateTo(
	fromElement: HTMLElement,
	toSlide: HTMLElement,
): HTMLElement | null {
	const id = fromElement.getAttribute("data-id");
	if (!id) {
		return null;
	}
	const escapedId = id.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
	let candidates = [...toSlide.querySelectorAll<HTMLElement>(`[data-id="${escapedId}"]`)];
	if (candidates.length === 0) {
		const baseId = id.replace(/--\d+$/, "");
		if (baseId !== id) {
			candidates = [...toSlide.querySelectorAll<HTMLElement>(`[data-id="${baseId.replace(/\\/g, "\\\\").replace(/"/g, '\\"')}"]`)];
		} else {
			candidates = [...toSlide.querySelectorAll<HTMLElement>(`[data-id="${escapedId}--0"]`)];
		}
	}
	if (candidates.length === 0) {
		return null;
	}
	if (elementIsInteractiveFigureDisposition(fromElement)) {
		const targetGhost = candidates.find(
			(candidate) =>
				candidate.classList.contains("presentation-interactive-disposition") &&
				elementIsTargetGhostAnchor(candidate),
		);
		if (targetGhost) {
			return targetGhost;
		}
	}
	if (elementIsSourceGhostAnchor(fromElement)) {
		const focusTile = candidates.find((candidate) => elementIsInteractiveFigureDisposition(candidate));
		if (focusTile) {
			return focusTile;
		}
	}
	const visible = candidates.find((candidate) => !elementIsLabelMorphSource(candidate));
	if (visible) {
		return visible;
	}
	return (
		candidates.find(
			(candidate) =>
				!elementIsSourceGhostAnchor(candidate) ||
				(elementIsTargetGhostAnchor(candidate) &&
					candidate.classList.contains("presentation-interactive-disposition")),
		) ?? null
	);
}

/** @emoji 🔗 reveal.js auto-animate matcher: intro uses stock `data-id` pairing; catalogue uses ghost-aware pairing. */
export function presentationAutoAnimateMatcher(
	this: AutoAnimateMatcherHost,
	fromSlide: HTMLElement,
	toSlide: HTMLElement,
): { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[] {
	if (isIntroFlowSlide(fromSlide) && isIntroFlowSlide(toSlide)) {
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

	const pairs: { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[] = [];
	for (const fromElement of fromSlide.querySelectorAll<HTMLElement>("[data-id]")) {
		if (!isRevealAutoAnimatePairSource(fromElement)) {
			continue;
		}
		const toElement = resolveMorphAutoAnimateTo(fromElement, toSlide);
		if (toElement && isRevealAutoAnimatePairSource(toElement)) {
			const isTextLeaf = fromElement.matches("h1, h2, h3, h4, h5, h6, p");
			const options = isTextLeaf
				? revealTextAutoAnimatePairOptions(fromElement, fromSlide, toSlide)
				: undefined;
			pairs.push({ from: fromElement, to: toElement, ...(options ? { options } : {}) });
		}
	}
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
		if (elementIsSourceGhostAnchor(pair.to)) {
			return false;
		}
		if (elementIsMorphOneAnchor(pair.from)) {
			return false;
		}
		if (elementIsSourceGhostAnchor(pair.from) && !elementIsInteractiveFigureDisposition(pair.to)) {
			return false;
		}
		if (elementIsFigureMorphSlot(pair.from) && elementIsMorphOneAnchor(pair.to)) {
			return false;
		}
		if (elementIsLabelMorphSource(pair.from) && !elementIsLabelMorphSource(pair.to)) {
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

/** @emoji 👻 Figure ghost visibility during reveal auto-animate (FLIP only; no intro text keyframes). */
export function presentationMorphGhostAutoAnimateCss(durationSeconds: number): string {
	const duration = `${durationSeconds}s`;
	return `
.reveal .slides section[data-auto-animate="pending"] .presentation-target-ghost[data-auto-animate-target] {
	opacity: 1 !important;
	visibility: visible !important;
	animation: none !important;
}
.reveal .slides section[data-auto-animate="running"] .presentation-target-ghost[data-auto-animate-target] {
	visibility: visible !important;
	animation: presentation-target-ghost-fade-out ${duration} ease forwards !important;
}
.reveal .slides section.presentation-arrangement--many-to-one-morph[data-auto-animate="pending"] .presentation-target-ghost.presentation-interactive-disposition--canvas-framed {
	left: var(--presentation-morph-frame-left) !important;
	top: var(--presentation-morph-frame-top) !important;
	width: var(--presentation-morph-frame-width) !important;
	height: var(--presentation-morph-frame-height) !important;
	transform: none !important;
	transition: none !important;
}
.reveal .slides section.presentation-arrangement--many-to-one-morph[data-auto-animate="running"] .presentation-target-ghost.presentation-interactive-disposition--canvas-framed[data-auto-animate-target] {
	visibility: visible !important;
	transform: none !important;
	transition: none !important;
	animation: presentation-target-ghost-frame ${duration} ease forwards,
		presentation-target-ghost-fade-out ${duration} ease forwards !important;
}
.reveal .slides section[data-auto-animate="pending"] .presentation-morph-one[data-auto-animate-target],
.reveal .slides section[data-auto-animate="running"] .presentation-morph-one[data-auto-animate-target] {
	animation: presentation-morph-one-fade-out ${duration} ease forwards !important;
}
`;
}

/** @emoji 🩹 Patches reveal auto-animate sheet: uniform scale for catalogue tiles only; intro text keeps native FLIP. */
export function patchPresentationAutoAnimateStyleSheet(
	sheet: { innerHTML: string },
	durationSeconds: number,
	options?: { readonly manyToOneMorph?: boolean; readonly introFlowMorph?: boolean },
): void {
	let css = sheet.innerHTML;
	if (options?.manyToOneMorph !== true && options?.introFlowMorph !== true) {
		css = patchAutoAnimateUniformScale(css);
	}
	sheet.innerHTML = css + presentationMorphGhostAutoAnimateCss(durationSeconds);
}

/** @emoji 🩹 Applies reveal auto-animate sheet fixes for the current slide pair. */
export function patchPresentationAutoAnimateRunStyleSheet(
	sheet: { innerHTML: string },
	durationSeconds: number,
	fromSlide: HTMLElement | undefined,
	toSlide: HTMLElement | undefined,
): void {
	if (fromSlide !== undefined && toSlide !== undefined && isIntroFlowSlide(fromSlide) && isIntroFlowSlide(toSlide)) {
		return;
	}
	patchPresentationAutoAnimateStyleSheet(sheet, durationSeconds, {
		manyToOneMorph:
			fromSlide !== undefined &&
			toSlide !== undefined &&
			isManyToOneMorphTransition(fromSlide, toSlide),
	});
}

export interface PresentationAutoAnimateRunSlides {
	readonly fromSlide?: HTMLElement;
	readonly toSlide?: HTMLElement;
}

/** @emoji 🔎 Resolves an auto-animate slide pair when reveal.js omits it from the `autoanimate` event. */
export function resolvePresentationAutoAnimateRunSlides(
	explicit: PresentationAutoAnimateRunSlides,
	pending: PresentationAutoAnimateRunSlides | undefined,
): PresentationAutoAnimateRunSlides {
	if (explicit.fromSlide !== undefined && explicit.toSlide !== undefined) {
		return explicit;
	}
	if (pending?.fromSlide !== undefined && pending.toSlide !== undefined) {
		return pending;
	}
	return {
		fromSlide: explicit.fromSlide ?? pending?.fromSlide,
		toSlide: explicit.toSlide ?? pending?.toSlide,
	};
}
//#endregion 🔖ArrangementSettled

//#region 🔖HiddenPreflight
/**
 * @emoji 🩹 Lets reveal.js own slide visibility by relaxing Tailwind preflight's `[hidden]` reset.
 *
 * `@semio-tech/ui-react` surface chrome ships Tailwind v4 preflight, whose layered
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

/** @emoji ⛶ When true, {@link PdfMorphView} measures against the enlarged slide content box, not the declared catalogue frame. */
const PresentationDispositionEnlargeContext = createContext(false);

/** @emoji 📐 Live slide frame for {@link FigureMorphView} cover math (drag/resize updates this). */
const PresentationFigureCropFrameContext = createContext<DispositionPosition | undefined>(undefined);

/** @emoji 📐 Slide width÷height for mosaic windowed-cover (`undefined` → use {@link FigureEmbodiment.sourceAspect}). */
const PresentationSlideAspectContext = createContext<number | undefined>(undefined);

/** @emoji 🆔 Interactive disposition id for ephemeral pdf page navigation in {@link PdfMorphView}. */
const PresentationInteractiveDispositionIdContext = createContext<string | undefined>(undefined);

export function parsePresentationSlideCssSize(revealEl: HTMLElement | null): { readonly width: number; readonly height: number } {
	const width = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-width") ?? "960");
	const height = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-height") ?? "700");
	return {
		width: Number.isFinite(width) && width > 0 ? width : 960,
		height: Number.isFinite(height) && height > 0 ? height : 700,
	};
}

/** @emoji 📐 Uniform react-pdf scale so the page covers the disposition frame without distortion. */
export function pdfCoverScale(
	containerWidth: number,
	containerHeight: number,
	pageWidth: number,
	pageHeight: number,
): number | null {
	if (containerWidth <= 0 || containerHeight <= 0 || pageWidth <= 0 || pageHeight <= 0) {
		return null;
	}
	return Math.max(containerWidth / pageWidth, containerHeight / pageHeight);
}

/** @emoji 📐 Uniform react-pdf scale for one-axis cover scroll (fit width or height, overflow the other). */
export function pdfScrollCoverScale(
	containerWidth: number,
	containerHeight: number,
	pageWidth: number,
	pageHeight: number,
): number | null {
	if (containerWidth <= 0 || containerHeight <= 0 || pageWidth <= 0 || pageHeight <= 0) {
		return null;
	}
	const pageAspect = pageWidth / pageHeight;
	const axis = figureCoverOverflowAxis(containerWidth, containerHeight, pageAspect);
	if (axis === "y") {
		return containerWidth / pageWidth;
	}
	if (axis === "x") {
		return containerHeight / pageHeight;
	}
	return Math.min(containerWidth / pageWidth, containerHeight / pageHeight);
}

/** @emoji 📑 Ordered PDF page numbers for navigation; empty means all document pages. */
export function pdfEmbodimentPageList(embodiment: PdfEmbodiment): readonly number[] {
	return embodiment.pages ?? [];
}

/** @emoji 📄 Starting page for a pdf embodiment (subset or single page). */
export function pdfEmbodimentInitialPage(embodiment: PdfEmbodiment): number {
	const pages = pdfEmbodimentPageList(embodiment);
	if (pages.length > 0) {
		const declared = embodiment.page ?? pages[0];
		return pages.includes(declared) ? declared : pages[0];
	}
	return embodiment.page ?? 1;
}

/** @emoji 🧭 Whether pdf prev/next controls apply for this embodiment. */
export function pdfPageNavEnabled(embodiment: PdfEmbodiment, numPages: number | null): boolean {
	const pages = pdfEmbodimentPageList(embodiment);
	if (pages.length > 1) {
		return true;
	}
	return pages.length === 0 && numPages !== null && numPages > 1;
}

/** @emoji ◀️ True when pdf page nav can move to an earlier page. */
export function pdfCanGoToPreviousPage(
	currentPage: number,
	embodiment: PdfEmbodiment,
	numPages: number | null,
): boolean {
	const pages = pdfEmbodimentPageList(embodiment);
	if (pages.length > 0) {
		return pages.indexOf(currentPage) > 0;
	}
	return currentPage > 1;
}

/** @emoji ▶️ True when pdf page nav can move to a later page. */
export function pdfCanGoToNextPage(
	currentPage: number,
	embodiment: PdfEmbodiment,
	numPages: number | null,
): boolean {
	const pages = pdfEmbodimentPageList(embodiment);
	if (pages.length > 0) {
		const index = pages.indexOf(currentPage);
		return index >= 0 && index < pages.length - 1;
	}
	return numPages !== null && currentPage < numPages;
}

/** @emoji 📄 Target page for prev/next within a subset or the full document. */
export function pdfAdjacentPage(
	currentPage: number,
	direction: "prev" | "next",
	embodiment: PdfEmbodiment,
	numPages: number | null,
): number {
	const pages = pdfEmbodimentPageList(embodiment);
	if (pages.length > 0) {
		let index = pages.indexOf(currentPage);
		if (index < 0) {
			index = direction === "next" ? -1 : pages.length;
		}
		const nextIndex = direction === "prev" ? index - 1 : index + 1;
		return pages[nextIndex] ?? currentPage;
	}
	const step = direction === "prev" ? -1 : 1;
	const next = currentPage + step;
	if (direction === "prev" && next < 1) {
		return currentPage;
	}
	if (direction === "next" && numPages !== null && next > numPages) {
		return currentPage;
	}
	return next;
}

/** @emoji 📐 Measures the react-pdf viewport from the disposition frame or enlarged slide content box. */
function usePdfContainerSize(
	anchorRef: RefObject<HTMLDivElement | null>,
	position: DispositionPosition | undefined,
	slideEpoch: number,
	enlarged: boolean,
): { readonly width?: number; readonly height?: number } {
	const [size, setSize] = useState<{ readonly width?: number; readonly height?: number }>({});
	useEffect(() => {
		const el = anchorRef.current;
		if (!el) {
			return;
		}
		const measureTarget = (): HTMLElement | null => {
			if (enlarged) {
				return (
					el.closest(".presentation-interactive-disposition--enlarged")?.querySelector(
						".presentation-interactive-disposition__content",
					) ?? null
				);
			}
			const frame = el.closest(".presentation-disposition-frame");
			const scrollViewport = frame?.querySelector<HTMLElement>(
				".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport",
			);
			return scrollViewport ?? frame;
		};
		const measure = (): void => {
			const target = measureTarget();
			if (target && target.clientWidth > 8 && target.clientHeight > 8) {
				setSize({
					width: target.clientWidth,
					height: target.clientHeight,
				});
				return;
			}
			const slide = parsePresentationSlideCssSize(el.closest(".reveal"));
			const frame = enlarged ? SLIDE_INTERACTIVE_ENLARGE_FRAME : position;
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
		enlarged,
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

/** @emoji 🎯 Renders {@link TextEmbodiment}; `data-id` on leaf text unless the intro wrapper owns the morph anchor. */
/** @emoji 🎯 Stable reveal.js `data-id` for one text line; single-line blocks use the morph id, multi-line use `--index`. */
function textMorphAnchorId(
	anchorId: string,
	lineIndex: number,
	lineCount: number,
	_root: ReturnType<typeof resolveTextMorphRoot>,
): string {
	return lineCount === 1 ? anchorId : `${anchorId}--${lineIndex}`;
}

function morphLeafDataId(anchorOnWrapper: boolean, id: string): { readonly "data-id"?: string } {
	return anchorOnWrapper ? {} : { "data-id": id };
}

function TextMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
	anchorOnWrapper = false,
}: {
	readonly morphId: string;
	readonly embodiment: TextEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly anchorOnWrapper?: boolean;
}): ReactNode {
	const root = resolveTextMorphRoot(embodiment);
	const centeredHeadingClass = centeredLineClass(anchorId, embodiment, emphasis);
	const lineCount = embodiment.lines.length;

	switch (root) {
		case "title":
			return (
				<h1 {...morphLeafDataId(anchorOnWrapper, anchorId)} className={centeredLineClass(anchorId, embodiment, emphasis)}>
					{embodiment.lines[0]}
				</h1>
			);
		case "body":
			return (
				<div className="w-full text-center">
					{embodiment.lines.map((line, lineIndex) => (
						<p
							key={`${anchorId}--${lineIndex}`}
							{...morphLeafDataId(anchorOnWrapper, textMorphAnchorId(anchorId, lineIndex, lineCount, root))}
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
				<h2 {...morphLeafDataId(anchorOnWrapper, anchorId)} className={centeredHeadingClass}>
					{embodiment.lines[0]}
				</h2>
			);
		case "heading-block":
			return (
				<div className="w-full text-center">
					{embodiment.lines.map((line, lineIndex) => (
						<h2
							key={`${anchorId}--${lineIndex}`}
							{...morphLeafDataId(anchorOnWrapper, textMorphAnchorId(anchorId, lineIndex, lineCount, root))}
							className={centeredHeadingClass}
						>
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
	emphasis,
	anchorOnWrapper = false,
}: {
	readonly morphId: string;
	readonly embodiment: AuthorsEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly anchorOnWrapper?: boolean;
}): ReactNode {
	const namesMuted = embodiment.abbreviateFirstName === true || emphasis === "muted";
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
								{...morphLeafDataId(anchorOnWrapper, `${anchorId}--${person.name}`)}
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
	anchorOnWrapper = false,
}: {
	readonly morphId: string;
	readonly embodiment: AffiliationsEmbodiment;
	readonly anchorOnWrapper?: boolean;
}): ReactNode {
	const rowClass = morphTextClass(
		anchorId,
		"presentation-affiliation-row m-0 inline-flex max-w-full shrink-0 flex-row flex-nowrap items-center justify-center gap-x-[0.35em] text-center",
	);
	return (
		<div className="presentation-intro-rows presentation-intro-affiliations flex w-full max-w-full flex-col items-center text-center">
			{embodiment.entries.map((entry) => {
				const displayLabel = affiliationLineLabel(entry, "line");
				return (
					<div
						key={entry.mark}
						className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center gap-x-[0.35em]"
					>
						<h4 {...morphLeafDataId(anchorOnWrapper, `${anchorId}--${entry.mark}`)} className={rowClass}>
							{affiliationLineContent(entry, "line", displayLabel)}
							{entry.suffix ? (
								<span
									{...morphLeafDataId(anchorOnWrapper, `${anchorId}--${entry.suffix.mark}`)}
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

/** @emoji 📐 Physical aspect of a normalized crop (width÷height in source pixels). */
export function figureCropPhysicalAspect(crop: DispositionPosition, sourceAspect = 1): number {
	return (crop.width / crop.height) * sourceAspect;
}

const FIGURE_MOSAIC_ALIGN_EPSILON = 1e-4;

/** @emoji 🧩 Grid column/row for a split crop, or null when the crop is not a mosaic cell. */
export function figureMosaicCellIndex(
	crop: DispositionPosition,
	mosaic: { readonly rows: number; readonly columns: number; readonly frame?: DispositionPosition },
): { readonly column: number; readonly row: number } | null {
	const { rows, columns } = mosaic;
	if (rows < 1 || columns < 1) {
		return null;
	}
	const mosaicFrame = mosaic.frame ?? { x: 0, y: 0, width: 1, height: 1 };
	const colWidth = mosaicFrame.width / columns;
	const rowHeight = mosaicFrame.height / rows;
	if (
		Math.abs(crop.width - colWidth) > FIGURE_MOSAIC_ALIGN_EPSILON ||
		Math.abs(crop.height - rowHeight) > FIGURE_MOSAIC_ALIGN_EPSILON
	) {
		return null;
	}
	const column = Math.round((crop.x - mosaicFrame.x) / colWidth);
	const row = Math.round((crop.y - mosaicFrame.y) / rowHeight);
	if (column < 0 || column >= columns || row < 0 || row >= rows) {
		return null;
	}
	if (
		Math.abs(crop.x - (mosaicFrame.x + column * colWidth)) > FIGURE_MOSAIC_ALIGN_EPSILON ||
		Math.abs(crop.y - (mosaicFrame.y + row * rowHeight)) > FIGURE_MOSAIC_ALIGN_EPSILON
	) {
		return null;
	}
	return { column, row };
}

/** @emoji 🧩 Edge-aligned background-position for one cell in a rows×columns sprite grid. */
export function figureMosaicBackgroundPosition(
	column: number,
	row: number,
	columns: number,
	rows: number,
): { readonly posX: number; readonly posY: number } {
	return {
		posX: columns <= 1 ? 50 : (column / (columns - 1)) * 100,
		posY: rows <= 1 ? 50 : (row / (rows - 1)) * 100,
	};
}

/** @emoji 📐 Background-position along one axis when cover overflows (k≥1; k=1 → edge-aligned i/(n−1)). */
export function overflowAxisPosition(index: number, count: number, coverOverflowK: number): number {
	if (count <= 1) {
		return 50;
	}
	if (Math.abs(coverOverflowK - 1) < FIGURE_MOSAIC_ALIGN_EPSILON) {
		return (index / (count - 1)) * 100;
	}
	const cellSpan = 1 / count;
	const numerator = (1 - coverOverflowK) / 2 - index / count;
	const denominator = cellSpan - coverOverflowK;
	return (numerator / denominator) * 100;
}

/** @emoji 🪟 One mosaic cell as a window onto a single cover render of `frame` (no per-crop sprite zoom). */
export function mosaicWindowedCoverVars(
	cell: { readonly column: number; readonly row: number },
	grid: { readonly rows: number; readonly columns: number },
	frame: DispositionPosition,
	sourceAspect: number,
	slideAspect?: number,
): { readonly size: string; readonly posX: number; readonly posY: number } {
	const { rows, columns } = grid;
	const { column, row } = cell;
	const slideAR = slideAspect ?? sourceAspect;
	const frameAspect = (frame.width / frame.height) * slideAR;
	if (frameAspect >= sourceAspect) {
		const k = frameAspect / sourceAspect;
		return {
			size: `${columns * 100}% auto`,
			posX: columns <= 1 ? 50 : (column / (columns - 1)) * 100,
			posY: overflowAxisPosition(row, rows, k),
		};
	}
	const k = sourceAspect / frameAspect;
	return {
		size: `auto ${rows * 100}%`,
		posY: rows <= 1 ? 50 : (row / (rows - 1)) * 100,
		posX: overflowAxisPosition(column, columns, k),
	};
}

/** @emoji 🖼 True when the crop is the full source bitmap. */
function figureCropIsFullImage(crop: DispositionPosition): boolean {
	return (
		crop.width >= 1 - FIGURE_MOSAIC_ALIGN_EPSILON && crop.height >= 1 - FIGURE_MOSAIC_ALIGN_EPSILON
	);
}

/** @emoji 🖼 Positions a normalized source crop when background width is `(100/crop.width)%` (uniform, no distortion). */
export function figureCropBackgroundPosition(crop: DispositionPosition): {
	readonly posX: number;
	readonly posY: number;
} {
	if (figureCropIsFullImage(crop)) {
		return { posX: 50, posY: 50 };
	}
	const spanX = 1 - crop.width;
	const spanY = 1 - crop.height;
	return {
		posX: spanX > FIGURE_MOSAIC_ALIGN_EPSILON ? (crop.x / spanX) * 100 : 50,
		posY: spanY > FIGURE_MOSAIC_ALIGN_EPSILON ? (crop.y / spanY) * 100 : 50,
	};
}

/** @emoji 🖼 Uniform background-size: `cover` for full image, else `N% auto` / `auto N%` zoomed to the crop (never dual-axis `%`). */
export function figureCropBackgroundSize(
	crop: DispositionPosition,
	frame: DispositionPosition,
	sourceAspect = 1,
): string {
	if (figureCropIsFullImage(crop)) {
		return "cover";
	}
	if (frame.width <= 0 || frame.height <= 0 || crop.width <= 0 || crop.height <= 0) {
		return "cover";
	}
	const zoomW = 100 / crop.width;
	const zoomH = 100 / crop.height;
	const cropSpanHAtZoomW = (crop.height * zoomW * frame.width) / sourceAspect;
	const cropAspect = figureCropPhysicalAspect(crop, sourceAspect);
	const frameAspect = frame.width / frame.height;
	const coverScale = Math.max(frameAspect / cropAspect, cropAspect / frameAspect);
	if (cropSpanHAtZoomW >= frame.height) {
		return `${zoomW * coverScale}% auto`;
	}
	return `auto ${zoomH * coverScale}%`;
}

/** @emoji 🖼 Centered crop cover in a slide frame (non-mosaic / focus morph). */
function figureCropCoverVars(
	crop: DispositionPosition,
	frame: DispositionPosition,
	sourceAspect = 1,
): { readonly size: string; readonly posX: number; readonly posY: number } {
	const { posX, posY } = figureCropBackgroundPosition(crop);
	return {
		size: figureCropBackgroundSize(crop, frame, sourceAspect),
		posX,
		posY,
	};
}

/** @emoji 🪟 Mosaic cell background vars (windowed cover of `mosaic.frame`). */
function figureMosaicCellCoverVars(
	crop: DispositionPosition,
	mosaic: FigureMosaicGrid,
	sourceAspect: number,
	slideAspect?: number,
): { readonly size: string; readonly posX: number; readonly posY: number } | null {
	const cell = figureMosaicCellIndex(crop, mosaic);
	if (!cell) {
		return null;
	}
	const mosaicFrame = mosaic.frame ?? { x: 0, y: 0, width: 1, height: 1 };
	return mosaicWindowedCoverVars(cell, mosaic, mosaicFrame, sourceAspect, slideAspect);
}

//#region 🔖FigureScroll
/** @emoji 📐 Default slide pixel size from {@link PresentationSlideAspectContext} (960×700 when unknown). */
function presentationSlidePixelSize(slideAspect?: number): { readonly width: number; readonly height: number } {
	const height = 700;
	return {
		width: slideAspect !== undefined ? slideAspect * height : 960,
		height,
	};
}
/** @emoji 📐 Estimated frame pixels from normalized disposition (before DOM measure on hidden slides). */
export function estimateDispositionFramePixels(
	frame: DispositionPosition,
	slideWidth = 960,
	slideHeight = 700,
): { readonly width: number; readonly height: number } {
	return {
		width: frame.width * slideWidth,
		height: frame.height * slideHeight,
	};
}

/** @emoji 📜 True when a figure should use one-axis cover scroll (mosaic tiles always clip). */
export function figureEmbodimentScrollEnabled(embodiment: FigureEmbodiment): boolean {
	if (embodiment.scroll === false || embodiment.mosaic !== undefined) {
		return false;
	}
	return true;
}

/** @emoji 📜 True when a video should use one-axis cover scroll. */
export function videoEmbodimentScrollEnabled(embodiment: VideoEmbodiment): boolean {
	return embodiment.scroll !== false;
}

/** @emoji 📜 True when a pdf page should use one-axis cover scroll. */
export function pdfEmbodimentScrollEnabled(embodiment: PdfEmbodiment): boolean {
	return embodiment.scroll !== false;
}

/** @emoji ✨ True when a media embodiment should show the glassy teaser veil. */
export function mediaTeaserActive(teaser: MediaTeaser | undefined): teaser is MediaTeaser {
	return teaser !== undefined;
}

function MediaTeaserWrap({
	teaser,
	children,
}: {
	readonly teaser?: MediaTeaser;
	readonly children: ReactNode;
}): ReactNode {
	if (!mediaTeaserActive(teaser)) {
		return children;
	}
	return (
		<div className="presentation-media-teaser">
			<div className="presentation-media-teaser__content">{children}</div>
			<div
				className="presentation-media-teaser__veil"
				aria-hidden={teaser.label === undefined}
				{...(teaser.label !== undefined ? { role: "img", "aria-label": teaser.label } : {})}
			>
				{teaser.label !== undefined ? (
					<span className="presentation-media-teaser__label">{teaser.label}</span>
				) : null}
			</div>
		</div>
	);
}

/** @emoji ↔️ Which axis overflows under uniform cover (frame vs source aspect). */
export function figureCoverOverflowAxis(
	frameWidth: number,
	frameHeight: number,
	sourceAspect: number,
): "x" | "y" | null {
	if (frameWidth <= 0 || frameHeight <= 0 || sourceAspect <= 0) {
		return null;
	}
	const frameAspect = frameWidth / frameHeight;
	if (frameAspect > sourceAspect + FIGURE_MOSAIC_ALIGN_EPSILON) {
		return "y";
	}
	if (frameAspect < sourceAspect - FIGURE_MOSAIC_ALIGN_EPSILON) {
		return "x";
	}
	return null;
}

/** @emoji ↔️ Scroll axis implied by a crop background-size (`N% auto` → x, `auto N%` → y). */
export function figureBackgroundSizeScrollAxis(bgSize: string): "x" | "y" | null {
	if (bgSize === "cover" || bgSize === "contain") {
		return null;
	}
	if (/^[\d.]+% auto$/.test(bgSize)) {
		return "x";
	}
	if (/^auto [\d.]+%$/.test(bgSize)) {
		return "y";
	}
	return null;
}

/** @emoji 📐 Inner scroll content size for a crop background-size string. */
export function figureCropScrollContentSize(bgSize: string): CSSProperties {
	const horizontal = bgSize.match(/^([\d.]+)% auto$/);
	if (horizontal) {
		return { width: bgSize, height: "100%", minHeight: "100%" };
	}
	const vertical = bgSize.match(/^auto ([\d.]+)%$/);
	if (vertical) {
		return { width: "100%", minWidth: "100%", height: `${vertical[1]}%` };
	}
	return { width: "100%", height: "100%", minWidth: "100%", minHeight: "100%" };
}

/** @emoji ↔️ Scroll offset matching CSS background-position along one axis. */
export function figureScrollOffsetForBackgroundPosition(
	axis: "x" | "y",
	positionPercent: number,
	scrollSize: number,
	clientSize: number,
): number {
	const max = Math.max(0, scrollSize - clientSize);
	return (positionPercent / 100) * max;
}

/** @emoji 🖼 Crop background-size on a scroll inner (span one scroll axis). */
export function figureCropScrollBackgroundSize(
	bgSize: string,
	scrollAxis?: "x" | "y" | null,
	zoom = 1,
): string {
	const axis = figureBackgroundSizeScrollAxis(bgSize) ?? scrollAxis ?? null;
	if (axis === "x") {
		return zoom > 1 ? `${zoom * 100}% auto` : "100% auto";
	}
	if (axis === "y") {
		return zoom > 1 ? `auto ${zoom * 100}%` : "auto 100%";
	}
	return figureBackgroundSizeZoomed(bgSize, zoom);
}

/** @emoji 🔍 Minimum ctrl+wheel zoom (cover baseline). */
export const FIGURE_WHEEL_ZOOM_MIN = 1;

/** @emoji 🔍 Maximum ctrl+wheel zoom multiplier. */
export const FIGURE_WHEEL_ZOOM_MAX = 8;

const FIGURE_WHEEL_ZOOM_FACTOR = 1.1;

/** @emoji 🔍 Next zoom level from a wheel delta and current multiplier. */
export function figureWheelZoomStep(deltaY: number, currentZoom: number): number {
	const factor = deltaY < 0 ? FIGURE_WHEEL_ZOOM_FACTOR : 1 / FIGURE_WHEEL_ZOOM_FACTOR;
	return Math.min(FIGURE_WHEEL_ZOOM_MAX, Math.max(FIGURE_WHEEL_ZOOM_MIN, currentZoom * factor));
}

/** @emoji 🔍 Keep the pointer anchor fixed while zooming scrollable figure content. */
export function figureWheelZoomAdjustScroll(
	scroller: HTMLElement,
	clientX: number,
	clientY: number,
	prevZoom: number,
	nextZoom: number,
): void {
	if (prevZoom <= 0 || nextZoom === prevZoom) {
		return;
	}
	const rect = scroller.getBoundingClientRect();
	const anchorX = clientX - rect.left;
	const anchorY = clientY - rect.top;
	const ratio = nextZoom / prevZoom;
	scroller.scrollLeft = Math.max(0, (scroller.scrollLeft + anchorX) * ratio - anchorX);
	scroller.scrollTop = Math.max(0, (scroller.scrollTop + anchorY) * ratio - anchorY);
}

/** @emoji 🔍 Scale a crop `background-size` string by a zoom multiplier. */
export function figureBackgroundSizeZoomed(bgSize: string, zoom: number): string {
	if (zoom <= 1) {
		return bgSize;
	}
	const horizontal = bgSize.match(/^([\d.]+)% auto$/);
	if (horizontal) {
		return `${Number.parseFloat(horizontal[1]) * zoom}% auto`;
	}
	const vertical = bgSize.match(/^auto ([\d.]+)%$/);
	if (vertical) {
		return `auto ${Number.parseFloat(vertical[1]) * zoom}%`;
	}
	return bgSize;
}

/** @emoji 🔍 Apply ctrl+wheel zoom to cover scroll content sizing. */
export function figureCoverScrollContentSize(
	portWidth: number,
	portHeight: number,
	sourceAspect: number,
	zoom = 1,
): { readonly axis: "x" | "y" | "both" | null; readonly style: CSSProperties } {
	const base = figureCoverScrollContentSizeAtZoom(portWidth, portHeight, sourceAspect, 1);
	if (zoom <= 1) {
		return base;
	}
	if (base.axis === null) {
		const widthPercent = ((sourceAspect * portHeight * zoom) / portWidth) * 100;
		const heightPercent = ((portWidth * zoom) / sourceAspect / portHeight) * 100;
		return {
			axis: "both",
			style: { width: `${widthPercent}%`, height: `${heightPercent}%` },
		};
	}
	if (base.axis === "y") {
		const heightPercent = Number.parseFloat(String(base.style.height)) * zoom;
		return {
			axis: zoom > 1 ? "both" : "y",
			style: {
				width: zoom > 1 ? `${zoom * 100}%` : "100%",
				height: `${heightPercent}%`,
			},
		};
	}
	const widthPercent = Number.parseFloat(String(base.style.width)) * zoom;
	return {
		axis: zoom > 1 ? "both" : "x",
		style: {
			width: `${widthPercent}%`,
			height: zoom > 1 ? `${zoom * 100}%` : "100%",
		},
	};
}

function figureCoverScrollContentSizeAtZoom(
	portWidth: number,
	portHeight: number,
	sourceAspect: number,
	_zoom: number,
): { readonly axis: "x" | "y" | null; readonly style: CSSProperties } {
	const axis = figureCoverOverflowAxis(portWidth, portHeight, sourceAspect);
	if (!axis) {
		return { axis: null, style: { width: "100%", height: "100%" } };
	}
	if (axis === "y") {
		const heightPercent = (portWidth / sourceAspect / portHeight) * 100;
		return { axis, style: { width: "100%", height: `${heightPercent}%` } };
	}
	const widthPercent = ((sourceAspect * portHeight) / portWidth) * 100;
	return { axis, style: { width: `${widthPercent}%`, height: "100%" } };
}

/** @emoji 🔍 Apply ctrl+wheel zoom to img/video cover element sizing. */
export function figureCoverScrollElementStyle(
	portWidth: number,
	portHeight: number,
	sourceAspect: number,
	zoom = 1,
): CSSProperties {
	if (zoom <= 1) {
		const { axis } = figureCoverScrollContentSizeAtZoom(portWidth, portHeight, sourceAspect, 1);
		if (!axis) {
			return { width: "100%", height: "100%" };
		}
		if (axis === "y") {
			return {
				width: "100%",
				height: "auto",
				aspectRatio: String(sourceAspect),
				maxWidth: "none",
				maxHeight: "none",
			};
		}
		return {
			width: "auto",
			height: "100%",
			aspectRatio: String(sourceAspect),
			maxWidth: "none",
			maxHeight: "none",
		};
	}
	const { axis } = figureCoverScrollContentSizeAtZoom(portWidth, portHeight, sourceAspect, 1);
	if (!axis) {
		return {
			width: `${zoom * 100}%`,
			height: "auto",
			aspectRatio: String(sourceAspect),
			maxWidth: "none",
			maxHeight: "none",
		};
	}
	if (axis === "y") {
		return {
			width: `${zoom * 100}%`,
			height: "auto",
			aspectRatio: String(sourceAspect),
			maxWidth: "none",
			maxHeight: "none",
		};
	}
	return {
		width: "auto",
		height: `${zoom * 100}%`,
		aspectRatio: String(sourceAspect),
		maxWidth: "none",
		maxHeight: "none",
	};
}

const FigureZoomContext = createContext(1);

/** @emoji 🔍 Ctrl+wheel zoom multiplier for the enclosing {@link FigureScrollViewport}. */
export function useFigureZoom(): number {
	return useContext(FigureZoomContext);
}

function figureScrollViewportClass(axis: "x" | "y" | "both" | null): string {
	if (axis === "x") {
		return "presentation-figure-scroll-viewport presentation-figure-scroll-viewport--axis-x";
	}
	if (axis === "y") {
		return "presentation-figure-scroll-viewport presentation-figure-scroll-viewport--axis-y";
	}
	if (axis === "both") {
		return "presentation-figure-scroll-viewport presentation-figure-scroll-viewport--axis-both";
	}
	return "presentation-figure-scroll-viewport";
}

function figureScrollScrollerClass(axis: "x" | "y" | "both" | null): string {
	if (axis === "x") {
		return "presentation-figure-scroll-scroller presentation-figure-scroll-scroller--axis-x";
	}
	if (axis === "y") {
		return "presentation-figure-scroll-scroller presentation-figure-scroll-scroller--axis-y";
	}
	if (axis === "both") {
		return "presentation-figure-scroll-scroller presentation-figure-scroll-scroller--axis-both";
	}
	return "presentation-figure-scroll-scroller";
}

/** @emoji 📏 Overlay scrollbar thumb size and offset from native scroll metrics. */
export function figureScrollOverlayThumbMetrics(
	clientSize: number,
	scrollSize: number,
	scrollPos: number,
	minThumb = 24,
): { readonly thumbSize: number; readonly thumbOffset: number; readonly visible: boolean } {
	if (clientSize <= 0 || scrollSize <= clientSize + 1) {
		return { thumbSize: 0, thumbOffset: 0, visible: false };
	}
	const thumbSize = Math.max(minThumb, (clientSize / scrollSize) * clientSize);
	const maxThumbOffset = Math.max(0, clientSize - thumbSize);
	const maxScroll = scrollSize - clientSize;
	const thumbOffset = maxThumbOffset <= 0 || maxScroll <= 0 ? 0 : (scrollPos / maxScroll) * maxThumbOffset;
	return { thumbSize, thumbOffset, visible: true };
}

function observeFigureScrollScrollerLayout(
	scroller: HTMLElement,
	onChange: () => void,
): () => void {
	const observed = new Set<Element>();
	const observeNode = (node: Element): void => {
		if (!(node instanceof HTMLElement) || observed.has(node)) {
			return;
		}
		observed.add(node);
		resizeObserver.observe(node);
	};
	const walk = (root: Element): void => {
		observeNode(root);
		for (const child of root.children) {
			walk(child);
		}
	};
	const resizeObserver = new ResizeObserver(onChange);
	walk(scroller);
	const mutationObserver = new MutationObserver(() => {
		walk(scroller);
		onChange();
	});
	mutationObserver.observe(scroller, { childList: true, subtree: true, attributes: true });
	return () => {
		mutationObserver.disconnect();
		resizeObserver.disconnect();
		observed.clear();
	};
}

function observeFigureScrollScroller(
	scroller: HTMLElement,
	onChange: () => void,
): () => void {
	const stopLayout = observeFigureScrollScrollerLayout(scroller, onChange);
	scroller.addEventListener("scroll", onChange, { passive: true });
	return () => {
		scroller.removeEventListener("scroll", onChange);
		stopLayout();
	};
}

function useFigureScrollOverlayBar(
	scrollerRef: RefObject<HTMLElement | null>,
	barRef: RefObject<HTMLDivElement | null>,
	thumbRef: RefObject<HTMLDivElement | null>,
	axis: "x" | "y" | null,
	enabled: boolean,
): void {
	const syncBar = useCallback((): void => {
		const scroller = scrollerRef.current;
		const bar = barRef.current;
		const thumb = thumbRef.current;
		if (!scroller || !bar || !thumb) {
			return;
		}
		if (!enabled || axis === null) {
			bar.style.display = "none";
			return;
		}
		const clientSize = axis === "y" ? scroller.clientHeight : scroller.clientWidth;
		const scrollSize = axis === "y" ? scroller.scrollHeight : scroller.scrollWidth;
		const scrollPos = axis === "y" ? scroller.scrollTop : scroller.scrollLeft;
		const metrics = figureScrollOverlayThumbMetrics(clientSize, scrollSize, scrollPos);
		if (!metrics.visible) {
			bar.style.display = "none";
			return;
		}
		bar.style.display = "";
		if (axis === "y") {
			thumb.style.width = "";
			thumb.style.height = `${metrics.thumbSize}px`;
			thumb.style.transform = `translateY(${metrics.thumbOffset}px)`;
		} else {
			thumb.style.height = "";
			thumb.style.width = `${metrics.thumbSize}px`;
			thumb.style.transform = `translateX(${metrics.thumbOffset}px)`;
		}
	}, [axis, barRef, enabled, scrollerRef, thumbRef]);
	useLayoutEffect(() => {
		syncBar();
		const scroller = scrollerRef.current;
		if (!scroller) {
			return;
		}
		return observeFigureScrollScroller(scroller, syncBar);
	}, [axis, enabled, scrollerRef, syncBar]);
}

function useFigureScrollViewportSync(
	viewportRef: RefObject<HTMLElement | null>,
	axis: "x" | "y" | null,
	scrollOrigin: { readonly x: number; readonly y: number },
	enabled: boolean,
): void {
	const syncedScrollSizeRef = useRef(0);
	const applyingRef = useRef(false);
	useLayoutEffect(() => {
		syncedScrollSizeRef.current = 0;
		if (!enabled || axis === null) {
			return;
		}
		const viewport = viewportRef.current;
		if (!viewport) {
			return;
		}
		const apply = (): void => {
			if (syncedScrollSizeRef.current === Number.POSITIVE_INFINITY) {
				return;
			}
			const clientSize = axis === "y" ? viewport.clientHeight : viewport.clientWidth;
			const scrollSize = axis === "y" ? viewport.scrollHeight : viewport.scrollWidth;
			if (scrollSize <= clientSize + 1 || scrollSize <= syncedScrollSizeRef.current) {
				return;
			}
			applyingRef.current = true;
			const positionPercent = axis === "x" ? scrollOrigin.x : scrollOrigin.y;
			if (axis === "x") {
				viewport.scrollLeft = figureScrollOffsetForBackgroundPosition(
					"x",
					positionPercent,
					scrollSize,
					clientSize,
				);
			} else {
				viewport.scrollTop = figureScrollOffsetForBackgroundPosition(
					"y",
					positionPercent,
					scrollSize,
					clientSize,
				);
			}
			syncedScrollSizeRef.current = scrollSize;
			applyingRef.current = false;
		};
		const onUserScroll = (): void => {
			if (applyingRef.current) {
				return;
			}
			syncedScrollSizeRef.current = Number.POSITIVE_INFINITY;
		};
		apply();
		viewport.addEventListener("scroll", onUserScroll, { passive: true });
		const stopLayout = observeFigureScrollScrollerLayout(viewport, apply);
		return () => {
			viewport.removeEventListener("scroll", onUserScroll);
			stopLayout();
		};
	}, [axis, enabled, scrollOrigin.x, scrollOrigin.y, viewportRef]);
}

function FigureScrollViewport({
	enabled,
	axis,
	scrollOrigin,
	slideEpoch,
	className,
	style,
	children,
}: {
	readonly enabled: boolean;
	readonly axis: "x" | "y" | null;
	readonly scrollOrigin: { readonly x: number; readonly y: number };
	readonly slideEpoch?: number;
	readonly className?: string;
	readonly style?: CSSProperties;
	readonly children: ReactNode;
}): ReactNode {
	const [zoom, setZoom] = useState(1);
	const scrollerRef = useRef<HTMLDivElement>(null);
	const pendingZoomAnchorRef = useRef<{
		readonly clientX: number;
		readonly clientY: number;
		readonly prevZoom: number;
		readonly nextZoom: number;
	} | null>(null);
	const barRef = useRef<HTMLDivElement>(null);
	const thumbRef = useRef<HTMLDivElement>(null);
	const thumbDragRef = useRef<{ readonly startPointer: number; readonly startScroll: number } | null>(null);
	const scrollActive = enabled || zoom > 1;
	const scrollAxis: "x" | "y" | "both" | null = zoom > 1 ? "both" : axis;
	const overlayAxis: "x" | "y" | null = scrollAxis === "both" ? axis : scrollAxis;
	useEffect(() => {
		setZoom(1);
	}, [slideEpoch]);
	useLayoutEffect(() => {
		const pending = pendingZoomAnchorRef.current;
		const scroller = scrollerRef.current;
		if (!pending || !scroller) {
			return;
		}
		figureWheelZoomAdjustScroll(
			scroller,
			pending.clientX,
			pending.clientY,
			pending.prevZoom,
			pending.nextZoom,
		);
		pendingZoomAnchorRef.current = null;
	}, [zoom]);
	useFigureScrollViewportSync(scrollerRef, overlayAxis, scrollOrigin, scrollActive && overlayAxis !== null);
	useFigureScrollOverlayBar(scrollerRef, barRef, thumbRef, overlayAxis, scrollActive && overlayAxis !== null);
	const onWheel = useCallback(
		(event: React.WheelEvent<HTMLDivElement>) => {
			if (event.ctrlKey) {
				event.preventDefault();
				event.stopPropagation();
				const nextZoom = figureWheelZoomStep(event.deltaY, zoom);
				if (nextZoom === zoom) {
					return;
				}
				pendingZoomAnchorRef.current = {
					clientX: event.clientX,
					clientY: event.clientY,
					prevZoom: zoom,
					nextZoom,
				};
				const scroller = scrollerRef.current;
				if (scroller) {
					figureWheelZoomAdjustScroll(scroller, event.clientX, event.clientY, zoom, nextZoom);
					pendingZoomAnchorRef.current = null;
				}
				setZoom(nextZoom);
				return;
			}
			if (!scrollActive || scrollAxis === null || scrollAxis === "both") {
				return;
			}
			const viewport = event.currentTarget;
			const delta =
				scrollAxis === "x"
					? Math.abs(event.deltaX) > Math.abs(event.deltaY)
						? event.deltaX
						: event.deltaY
					: event.deltaY;
			if (delta === 0) {
				return;
			}
			const max =
				scrollAxis === "x"
					? viewport.scrollWidth - viewport.clientWidth
					: viewport.scrollHeight - viewport.clientHeight;
			if (max <= 0) {
				return;
			}
			const next =
				scrollAxis === "x" ? viewport.scrollLeft + delta : viewport.scrollTop + delta;
			if (next > 0 && next < max) {
				event.stopPropagation();
			}
		},
		[scrollActive, scrollAxis, zoom],
	);
	const onThumbPointerDown = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			const scroller = scrollerRef.current;
			const bar = barRef.current;
			if (!scrollActive || overlayAxis === null || scroller === null) {
				return;
			}
			event.preventDefault();
			event.stopPropagation();
			event.currentTarget.setPointerCapture(event.pointerId);
			bar?.classList.add("presentation-figure-scroll-bar--dragging");
			thumbDragRef.current = {
				startPointer: overlayAxis === "y" ? event.clientY : event.clientX,
				startScroll: overlayAxis === "y" ? scroller.scrollTop : scroller.scrollLeft,
			};
		},
		[overlayAxis, barRef, scrollActive],
	);
	const onThumbPointerMove = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			const drag = thumbDragRef.current;
			const scroller = scrollerRef.current;
			if (!scrollActive || overlayAxis === null || drag === null || scroller === null) {
				return;
			}
			const clientSize = overlayAxis === "y" ? scroller.clientHeight : scroller.clientWidth;
			const scrollSize = overlayAxis === "y" ? scroller.scrollHeight : scroller.scrollWidth;
			const maxScroll = scrollSize - clientSize;
			const metrics = figureScrollOverlayThumbMetrics(
				clientSize,
				scrollSize,
				overlayAxis === "y" ? scroller.scrollTop : scroller.scrollLeft,
			);
			const maxThumbOffset = Math.max(0, clientSize - metrics.thumbSize);
			if (maxThumbOffset <= 0 || maxScroll <= 0) {
				return;
			}
			const pointer = overlayAxis === "y" ? event.clientY : event.clientX;
			const delta = pointer - drag.startPointer;
			const nextScroll = Math.max(
				0,
				Math.min(maxScroll, drag.startScroll + (delta / maxThumbOffset) * maxScroll),
			);
			if (overlayAxis === "y") {
				scroller.scrollTop = nextScroll;
			} else {
				scroller.scrollLeft = nextScroll;
			}
		},
		[overlayAxis, scrollActive],
	);
	const onThumbPointerUp = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			thumbDragRef.current = null;
			barRef.current?.classList.remove("presentation-figure-scroll-bar--dragging");
			if (event.currentTarget.hasPointerCapture(event.pointerId)) {
				event.currentTarget.releasePointerCapture(event.pointerId);
			}
		},
		[barRef],
	);
	if (!scrollActive) {
		return (
			<FigureZoomContext.Provider value={zoom}>
				<div className={className} style={style} onWheel={onWheel}>
					{children}
				</div>
			</FigureZoomContext.Provider>
		);
	}
	return (
		<FigureZoomContext.Provider value={zoom}>
			<div
				className={[
					figureScrollViewportClass(scrollAxis),
					"presentation-figure-scroll-viewport--overlay",
					className,
				]
					.filter(Boolean)
					.join(" ")}
				style={style}
			>
				<div
					ref={scrollerRef}
					className={figureScrollScrollerClass(scrollAxis)}
					onWheel={onWheel}
				>
					{children}
				</div>
				{overlayAxis !== null ? (
					<div
						ref={barRef}
						className={`presentation-figure-scroll-bar presentation-figure-scroll-bar--axis-${overlayAxis}`}
						style={{ display: "none" }}
						aria-hidden="true"
					>
						<div
							ref={thumbRef}
							className="presentation-figure-scroll-bar-thumb"
							onPointerDown={onThumbPointerDown}
							onPointerMove={onThumbPointerMove}
							onPointerUp={onThumbPointerUp}
							onPointerCancel={onThumbPointerUp}
						/>
					</div>
				) : null}
			</div>
		</FigureZoomContext.Provider>
	);
}
//#endregion 🔖FigureScroll

function FigureZoomedImage({
	className,
	portWidth,
	portHeight,
	sourceAspect,
	scrollEnabled,
	src,
	alt,
	onLoad,
}: {
	readonly className: string;
	readonly portWidth: number;
	readonly portHeight: number;
	readonly sourceAspect: number;
	readonly scrollEnabled: boolean;
	readonly src: string;
	readonly alt: string;
	readonly onLoad: (event: React.SyntheticEvent<HTMLImageElement>) => void;
}): ReactNode {
	const zoom = useFigureZoom();
	const elementStyle = scrollEnabled
		? figureCoverScrollElementStyle(portWidth, portHeight, sourceAspect, zoom)
		: zoom > 1
			? {
					width: `${zoom * 100}%`,
					height: "auto",
					aspectRatio: String(sourceAspect),
					maxWidth: "none",
					maxHeight: "none",
				}
			: undefined;
	const mergedClassName = [
		className,
		(scrollEnabled || zoom > 1) ? "presentation-figure-scroll-media" : undefined,
	]
		.filter(Boolean)
		.join(" ");
	return (
		<img
			className={mergedClassName}
			style={elementStyle}
			src={src}
			alt={alt}
			onLoad={onLoad}
		/>
	);
}

function FigureZoomedVideo({
	className,
	portWidth,
	portHeight,
	sourceAspect,
	scrollEnabled,
	src,
	poster,
	loop,
	muted,
	controls,
	onLoadedMetadata,
}: {
	readonly className: string;
	readonly portWidth: number;
	readonly portHeight: number;
	readonly sourceAspect: number;
	readonly scrollEnabled: boolean;
	readonly src: string;
	readonly poster?: string;
	readonly loop?: boolean;
	readonly muted?: boolean;
	readonly controls?: boolean;
	readonly onLoadedMetadata: (event: React.SyntheticEvent<HTMLVideoElement>) => void;
}): ReactNode {
	const zoom = useFigureZoom();
	const elementStyle = scrollEnabled
		? figureCoverScrollElementStyle(portWidth, portHeight, sourceAspect, zoom)
		: zoom > 1
			? {
					width: `${zoom * 100}%`,
					height: "auto",
					aspectRatio: String(sourceAspect),
					maxWidth: "none",
					maxHeight: "none",
				}
			: undefined;
	const mergedClassName = [
		className,
		(scrollEnabled || zoom > 1) ? "presentation-figure-scroll-media" : undefined,
	]
		.filter(Boolean)
		.join(" ");
	return (
		<video
			className={mergedClassName}
			style={elementStyle}
			src={src}
			poster={poster}
			autoPlay={false}
			loop={loop ?? false}
			muted={muted ?? true}
			controls={controls ?? true}
			playsInline
			preload="metadata"
			onLoadedMetadata={onLoadedMetadata}
		/>
	);
}

function FigureZoomedPdfPage({
	currentPage,
	coverScale,
	ready,
	onPageLoadSuccess,
}: {
	readonly currentPage: number;
	readonly coverScale: number | null;
	readonly ready: boolean;
	readonly onPageLoadSuccess: (page: {
		getViewport: (options: { readonly scale: number }) => { readonly width: number; readonly height: number };
	}) => void;
}): ReactNode {
	const zoom = useFigureZoom();
	if (!ready) {
		return null;
	}
	return (
		<Page
			key={currentPage}
			className="presentation-media-pdf"
			pageNumber={currentPage}
			scale={(coverScale ?? 1) * zoom}
			onLoadSuccess={onPageLoadSuccess}
			renderTextLayer={false}
			renderAnnotationLayer={false}
		/>
	);
}

function FigureZoomedCropScrollContent({
	className,
	portWidth,
	portHeight,
	sourceAspect,
	backgroundSize,
	backgroundPosition,
	backgroundVars,
	ariaLabel,
	morphCropData,
}: {
	readonly className: string;
	readonly portWidth: number;
	readonly portHeight: number;
	readonly sourceAspect: number;
	readonly backgroundSize: string;
	readonly backgroundPosition: string;
	readonly backgroundVars: CSSProperties;
	readonly ariaLabel: string;
	readonly morphCropData?: string;
}): ReactNode {
	const zoom = useFigureZoom();
	const scrollContentStyle: CSSProperties = {
		...(backgroundSize === "cover"
			? figureCoverScrollContentSize(portWidth, portHeight, sourceAspect, zoom).style
			: figureCropScrollContentSize(figureBackgroundSizeZoomed(backgroundSize, zoom))),
		backgroundImage: backgroundVars.backgroundImage,
		["--presentation-figure-bg-size" as string]: figureCropScrollBackgroundSize(
			backgroundSize,
			figureBackgroundSizeScrollAxis(backgroundSize) ??
				figureCoverOverflowAxis(portWidth, portHeight, sourceAspect),
			zoom,
		),
		["--presentation-figure-bg-position" as string]: backgroundPosition,
		["--presentation-figure-bg-size-morph" as string]: backgroundVars[
			"--presentation-figure-bg-size-morph" as keyof typeof backgroundVars
		],
		["--presentation-figure-bg-position-morph" as string]: backgroundVars[
			"--presentation-figure-bg-position-morph" as keyof typeof backgroundVars
		],
		["--presentation-figure-bg-grid-size" as string]: backgroundVars[
			"--presentation-figure-bg-grid-size" as keyof typeof backgroundVars
		],
		["--presentation-figure-bg-grid-position" as string]: backgroundVars[
			"--presentation-figure-bg-grid-position" as keyof typeof backgroundVars
		],
	};
	return (
		<div
			className={className}
			style={scrollContentStyle}
			{...(morphCropData !== undefined
				? { "data-presentation-morph-crop": morphCropData }
				: {})}
			role="img"
			aria-label={ariaLabel}
		/>
	);
}

/** @emoji 📐 Reads `left`/`top`/`width`/`height` percent inline styles as a normalized disposition frame. */
export function readPercentDispositionFrame(element: HTMLElement): DispositionPosition | null {
	const read = (property: "left" | "top" | "width" | "height"): number | null => {
		const raw = element.style.getPropertyValue(property);
		if (!raw.endsWith("%")) {
			return null;
		}
		const value = Number.parseFloat(raw);
		return Number.isFinite(value) ? value / 100 : null;
	};
	const x = read("left");
	const y = read("top");
	const width = read("width");
	const height = read("height");
	if (x === null || y === null || width === null || height === null) {
		return null;
	}
	return { x, y, width, height };
}

/** @emoji 🔀 Copies live focus-tile frames from slide 8 onto label-slide target ghosts before many-to-one FLIP. */
export function syncManyToOneGhostMorphFramesFromDom(fromSlide: HTMLElement, toSlide: HTMLElement): void {
	if (!isManyToOneMorphTransition(fromSlide, toSlide)) {
		return;
	}
	for (const ghost of toSlide.querySelectorAll<HTMLElement>(
		".presentation-target-ghost.presentation-interactive-disposition--canvas-framed",
	)) {
		const morphId = ghost.getAttribute("data-id");
		if (!morphId) {
			continue;
		}
		const escapedId = morphId.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
		const sourceEl = fromSlide.querySelector<HTMLElement>(
			`.presentation-interactive-disposition--canvas-framed[data-id="${escapedId}"]`,
		);
		if (!sourceEl) {
			continue;
		}
		const sourceFrame =
			measureElementRectInSection(sourceEl, fromSlide) ?? readPercentDispositionFrame(sourceEl);
		if (!sourceFrame || !isUsableMeasuredRect(sourceFrame)) {
			continue;
		}
		const labelFrame = readPercentDispositionFrame(ghost) ?? measureElementRectInSection(ghost, toSlide);
		if (!labelFrame || !isUsableMeasuredRect(labelFrame)) {
			continue;
		}
		applyMorphFrameCssVars(ghost, sourceFrame, labelFrame);
		const slot = ghost.querySelector<HTMLElement>(".presentation-morph-slot--figure");
		const cropRaw = slot?.dataset.presentationMorphCrop;
		if (slot && cropRaw) {
			try {
				const crop = JSON.parse(cropRaw) as DispositionPosition;
				applyFigureCropCssVars(
					slot,
					figureCropBackgroundVarsTargetGhost(
						{ kind: "figure", src: "", crop },
						crop,
						sourceFrame,
						labelFrame,
					),
				);
			} catch {
				// skip invalid crop payload
			}
		}
		void ghost.offsetHeight;
	}
}

/** @emoji 📐 Custom properties for morphing a canvas frame from `from` into `to` during reveal auto-animate. */
export function morphFrameCssVars(
	from: DispositionPosition,
	to: DispositionPosition,
): CSSProperties {
	return {
		["--presentation-morph-frame-left" as string]: `${from.x * 100}%`,
		["--presentation-morph-frame-top" as string]: `${from.y * 100}%`,
		["--presentation-morph-frame-width" as string]: `${from.width * 100}%`,
		["--presentation-morph-frame-height" as string]: `${from.height * 100}%`,
		["--presentation-frame-left" as string]: `${to.x * 100}%`,
		["--presentation-frame-top" as string]: `${to.y * 100}%`,
		["--presentation-frame-width" as string]: `${to.width * 100}%`,
		["--presentation-frame-height" as string]: `${to.height * 100}%`,
	};
}

/** @emoji 📐 Writes {@link morphFrameCssVars} onto an element via `style.setProperty` (required for CSS variables). */
export function applyMorphFrameCssVars(
	element: HTMLElement,
	from: DispositionPosition,
	to: DispositionPosition,
): void {
	for (const [key, value] of Object.entries(morphFrameCssVars(from, to))) {
		if (typeof value === "string") {
			element.style.setProperty(key, value);
		}
	}
}

/** @emoji 🖼 Applies figure crop CSS variables from a vars object via `setProperty`. */
export function applyFigureCropCssVars(element: HTMLElement, vars: CSSProperties): void {
	for (const [key, value] of Object.entries(vars)) {
		if (typeof value === "string" && key.startsWith("--presentation-figure-bg-")) {
			element.style.setProperty(key, value);
		}
	}
}

/** @emoji 🖼 CSS vars for crop tiles: mosaic windowed cover at rest/grid; centered crop cover for focus morph. */
export function figureCropBackgroundVars(
	embodiment: FigureEmbodiment,
	crop: DispositionPosition,
	frame?: DispositionPosition,
	morphToFrame?: DispositionPosition,
	fromMorphToFrame?: DispositionPosition,
	slideAspect?: number,
): CSSProperties {
	const mosaic = embodiment.mosaic;
	const sourceAspect = embodiment.sourceAspect ?? 1;
	const mosaicRest = mosaic ? figureMosaicCellCoverVars(crop, mosaic, sourceAspect, slideAspect) : null;
	const restBasis = frame ?? morphToFrame;
	const rest = mosaicRest
		? mosaicRest
		: restBasis
			? figureCropCoverVars(crop, restBasis, sourceAspect)
			: figureCropCoverVars(crop, { x: 0, y: 0, width: 1, height: 1 }, sourceAspect);
	const morphBasis = morphToFrame ?? frame;
	const morph = morphBasis ? figureCropCoverVars(crop, morphBasis, sourceAspect) : rest;
	const gridFrom = fromMorphToFrame
		? mosaic
			? (figureMosaicCellCoverVars(crop, mosaic, sourceAspect, slideAspect) ??
				figureCropCoverVars(crop, fromMorphToFrame, sourceAspect))
			: figureCropCoverVars(crop, fromMorphToFrame, sourceAspect)
		: undefined;
	const vars: CSSProperties = {
		backgroundImage: `url("${resolvePresentationAssetUrl(embodiment.src)}")`,
		["--presentation-figure-bg-size" as string]: rest.size,
		["--presentation-figure-bg-position" as string]: `${rest.posX}% ${rest.posY}%`,
		["--presentation-figure-bg-size-morph" as string]: morph.size,
		["--presentation-figure-bg-position-morph" as string]: `${morph.posX}% ${morph.posY}%`,
	};
	if (gridFrom) {
		vars["--presentation-figure-bg-grid-size" as string] = gridFrom.size;
		vars["--presentation-figure-bg-grid-position" as string] = `${gridFrom.posX}% ${gridFrom.posY}%`;
	}
	return vars;
}

/** @emoji 👻 Target ghost crop vars: `--presentation-figure-bg-size` = source tile frame, `-morph` = label slot (many-to-one 8→9). */
export function figureCropBackgroundVarsTargetGhost(
	embodiment: FigureEmbodiment,
	crop: DispositionPosition,
	sourceFrame: DispositionPosition,
	labelFrame: DispositionPosition,
): CSSProperties {
	return figureCropBackgroundVars(embodiment, crop, sourceFrame, labelFrame);
}

function FigureImageMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
	position,
}: {
	readonly morphId: string;
	readonly embodiment: FigureEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
}): ReactNode {
	const scrollEnabled = figureEmbodimentScrollEnabled(embodiment);
	const slideEpoch = useContext(PresentationSlideEpochContext);
	const slideAspect = useContext(PresentationSlideAspectContext);
	const anchorRef = useRef<HTMLDivElement>(null);
	const [portSize, setPortSize] = useState({ width: 0, height: 0 });
	const [sourceAspect, setSourceAspect] = useState(embodiment.sourceAspect ?? 1);
	useLayoutEffect(() => {
		const anchor = anchorRef.current;
		if (!anchor) {
			return;
		}
		const measure = (): void => {
			const viewport = anchor.querySelector<HTMLElement>(
				".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport",
			);
			const target = viewport ?? anchor;
			setPortSize({ width: target.clientWidth, height: target.clientHeight });
		};
		measure();
		const observer = new ResizeObserver(measure);
		observer.observe(anchor);
		return () => observer.disconnect();
	}, [slideEpoch]);
	const estimatedPortSize = useMemo(
		() => {
			const slide = presentationSlidePixelSize(slideAspect);
			return position
				? estimateDispositionFramePixels(position, slide.width, slide.height)
				: estimateDispositionFramePixels({ x: 0, y: 0, width: 1, height: 1 });
		},
		[position, slideAspect],
	);
	const effectivePortSize = portSize.width > 0 ? portSize : estimatedPortSize;
	const coverScroll = figureCoverScrollContentSize(
		effectivePortSize.width,
		effectivePortSize.height,
		sourceAspect,
	);
	const axis = coverScroll.axis;
	const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
	return (
		<div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
			<MediaTeaserWrap teaser={embodiment.teaser}>
				<FigureScrollViewport
					enabled={scrollEnabled}
					axis={axis}
					scrollOrigin={scrollOrigin}
					slideEpoch={slideEpoch}
					style={{ width: "100%", height: "100%" }}
				>
					<FigureZoomedImage
						className="presentation-media-figure"
						portWidth={effectivePortSize.width}
						portHeight={effectivePortSize.height}
						sourceAspect={sourceAspect}
						scrollEnabled={scrollEnabled}
						src={resolvePresentationAssetUrl(embodiment.src)}
						alt={embodiment.alt ?? ""}
						onLoad={(event) => {
							const image = event.currentTarget;
							if (
								embodiment.sourceAspect === undefined &&
								image.naturalWidth > 0 &&
								image.naturalHeight > 0
							) {
								setSourceAspect(image.naturalWidth / image.naturalHeight);
							}
						}}
					/>
				</FigureScrollViewport>
			</MediaTeaserWrap>
		</div>
	);
}

function FigureCropMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
	position,
	style,
	dormantAnchor,
	anchorOnWrapper = false,
	revealMorphCompanion,
	morphFrame,
	morphToFrame,
	fromMorphToFrame,
}: {
	readonly morphId: string;
	readonly embodiment: FigureEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position: DispositionPosition;
	readonly style?: DispositionStyle;
	readonly dormantAnchor?: boolean;
	readonly anchorOnWrapper?: boolean;
	readonly revealMorphCompanion?: RevealMorphCompanionKind;
	readonly morphFrame?: DispositionPosition;
	readonly morphToFrame?: DispositionPosition;
	readonly fromMorphToFrame?: DispositionPosition;
}): ReactNode {
	const cropFrame = useContext(PresentationFigureCropFrameContext) ?? position;
	const slideAspect = useContext(PresentationSlideAspectContext);
	const slideEpoch = useContext(PresentationSlideEpochContext);
	const scrollEnabled = figureEmbodimentScrollEnabled(embodiment);
	const frameRef = useRef<HTMLDivElement>(null);
	const [portSize, setPortSize] = useState({ width: 0, height: 0 });
	const estimatedPortSize = useMemo(() => {
		const slide = presentationSlidePixelSize(slideAspect);
		return estimateDispositionFramePixels(cropFrame, slide.width, slide.height);
	}, [cropFrame, slideAspect]);
	const effectivePortSize = portSize.width > 0 ? portSize : estimatedPortSize;
	const dormant = dormantAnchor === true;
	const morphCropFrom =
		revealMorphCompanion === "target" && morphFrame !== undefined && position !== undefined;
	const morphCropTo =
		position !== undefined && (fromMorphToFrame !== undefined || morphToFrame !== undefined);
	const backgroundVars =
		morphCropFrom && morphFrame
			? figureCropBackgroundVarsTargetGhost(embodiment, embodiment.crop!, morphFrame, position)
			: figureCropBackgroundVars(
					embodiment,
					embodiment.crop!,
					cropFrame,
					morphToFrame,
					fromMorphToFrame,
					slideAspect,
				);
	const backgroundSize = String(backgroundVars["--presentation-figure-bg-size" as keyof typeof backgroundVars] ?? "cover");
	const backgroundPosition = String(
		backgroundVars["--presentation-figure-bg-position" as keyof typeof backgroundVars] ?? "50% 50%",
	);
	const sourceAspect = embodiment.sourceAspect ?? 1;
	useLayoutEffect(() => {
		const frame = frameRef.current;
		if (!frame) {
			return;
		}
		const measure = (): void => {
			setPortSize({ width: frame.clientWidth, height: frame.clientHeight });
		};
		measure();
		const observer = new ResizeObserver(measure);
		observer.observe(frame);
		return () => observer.disconnect();
	}, [slideEpoch]);
	const scrollAxis =
		figureBackgroundSizeScrollAxis(backgroundSize) ??
		figureCoverOverflowAxis(effectivePortSize.width, effectivePortSize.height, sourceAspect);
	const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
	const frameStyle = anchorOnWrapper
		? {
				position: "relative" as const,
				width: "100%",
				height: "100%",
				boxSizing: "border-box" as const,
				...(style?.opacity !== undefined ? { opacity: style.opacity } : {}),
			}
		: dispositionFrameStyle(position, dormant ? undefined : style);
	const slotClassName = [
		"presentation-morph-slot",
		"presentation-morph-slot--figure",
		revealMorphCompanion === "target" ? "presentation-target-ghost" : undefined,
		revealMorphCompanion === "source" ? "presentation-source-ghost" : undefined,
		morphCropFrom ? "presentation-morph-crop-from" : undefined,
		morphCropTo ? "presentation-morph-crop-to" : undefined,
		dormant ? "presentation-morph-slot--dormant" : undefined,
		emphasisClass(emphasis),
	]
		.filter(Boolean)
		.join(" ");
	if (!scrollEnabled) {
		return (
			<MediaTeaserWrap teaser={embodiment.teaser}>
				<div
					{...(anchorOnWrapper ? {} : { "data-id": anchorId })}
					className={["presentation-disposition-frame", slotClassName].join(" ")}
					style={{ ...frameStyle, ...backgroundVars }}
					{...(morphCropFrom && embodiment.crop
						? { "data-presentation-morph-crop": JSON.stringify(embodiment.crop) }
						: {})}
					role="img"
					aria-label={embodiment.alt ?? ""}
				/>
			</MediaTeaserWrap>
		);
	}
	return (
		<MediaTeaserWrap teaser={embodiment.teaser}>
			<div
				ref={frameRef}
				{...(anchorOnWrapper ? {} : { "data-id": anchorId })}
				className={["presentation-disposition-frame", "presentation-disposition-frame--figure-scroll"].join(
					" ",
				)}
				style={frameStyle}
			>
				<FigureScrollViewport
					enabled
					axis={scrollAxis}
					scrollOrigin={scrollOrigin}
					slideEpoch={slideEpoch}
					style={{ width: "100%", height: "100%" }}
				>
					<FigureZoomedCropScrollContent
						className={[slotClassName, "presentation-figure-scroll-content"].join(" ")}
						portWidth={effectivePortSize.width}
						portHeight={effectivePortSize.height}
						sourceAspect={sourceAspect}
						backgroundSize={backgroundSize}
						backgroundPosition={
							scrollAxis === "x"
								? `0% ${scrollOrigin.y}%`
								: scrollAxis === "y"
									? `${scrollOrigin.x}% 0%`
									: backgroundPosition
						}
						backgroundVars={backgroundVars}
						ariaLabel={embodiment.alt ?? ""}
						morphCropData={
							morphCropFrom && embodiment.crop
								? JSON.stringify(embodiment.crop)
								: undefined
						}
					/>
				</FigureScrollViewport>
			</div>
		</MediaTeaserWrap>
	);
}

function FigureMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
	position,
	style,
	dormantAnchor,
	anchorOnWrapper = false,
	revealMorphCompanion,
	morphFrame,
	morphToFrame,
	fromMorphToFrame,
}: {
	readonly morphId: string;
	readonly embodiment: FigureEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
	readonly style?: DispositionStyle;
	readonly dormantAnchor?: boolean;
	readonly anchorOnWrapper?: boolean;
	readonly revealMorphCompanion?: RevealMorphCompanionKind;
	readonly morphFrame?: DispositionPosition;
	readonly morphToFrame?: DispositionPosition;
	readonly fromMorphToFrame?: DispositionPosition;
}): ReactNode {
	if (embodiment.crop && position) {
		return (
			<FigureCropMorphView
				morphId={anchorId}
				embodiment={embodiment}
				emphasis={emphasis}
				position={position}
				style={style}
				dormantAnchor={dormantAnchor}
				anchorOnWrapper={anchorOnWrapper}
				revealMorphCompanion={revealMorphCompanion}
				morphFrame={morphFrame}
				morphToFrame={morphToFrame}
				fromMorphToFrame={fromMorphToFrame}
			/>
		);
	}
	return <FigureImageMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={position} />;
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
	position,
}: {
	readonly morphId: string;
	readonly embodiment: VideoEmbodiment;
	readonly emphasis: ParticipantEmphasis;
	readonly position?: DispositionPosition;
}): ReactNode {
	const scrollEnabled = videoEmbodimentScrollEnabled(embodiment);
	const slideEpoch = useContext(PresentationSlideEpochContext);
	const slideAspect = useContext(PresentationSlideAspectContext);
	const anchorRef = useRef<HTMLDivElement>(null);
	const [portSize, setPortSize] = useState({ width: 0, height: 0 });
	const [sourceAspect, setSourceAspect] = useState(16 / 9);
	useLayoutEffect(() => {
		const anchor = anchorRef.current;
		if (!anchor) {
			return;
		}
		const measure = (): void => {
			const viewport = anchor.querySelector<HTMLElement>(
				".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport",
			);
			const target = viewport ?? anchor;
			setPortSize({ width: target.clientWidth, height: target.clientHeight });
		};
		measure();
		const observer = new ResizeObserver(measure);
		observer.observe(anchor);
		return () => observer.disconnect();
	}, [slideEpoch]);
	const estimatedPortSize = useMemo(
		() => {
			const slide = presentationSlidePixelSize(slideAspect);
			return position
				? estimateDispositionFramePixels(position, slide.width, slide.height)
				: estimateDispositionFramePixels({ x: 0, y: 0, width: 1, height: 1 });
		},
		[position, slideAspect],
	);
	const effectivePortSize = portSize.width > 0 ? portSize : estimatedPortSize;
	const coverScroll = figureCoverScrollContentSize(
		effectivePortSize.width,
		effectivePortSize.height,
		sourceAspect,
	);
	const axis = coverScroll.axis;
	const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
	return (
		<div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
			<MediaTeaserWrap teaser={embodiment.teaser}>
				<FigureScrollViewport
					enabled={scrollEnabled}
					axis={axis}
					scrollOrigin={scrollOrigin}
					slideEpoch={slideEpoch}
					style={{ width: "100%", height: "100%" }}
				>
					<FigureZoomedVideo
						className="presentation-media-video"
						portWidth={effectivePortSize.width}
						portHeight={effectivePortSize.height}
						sourceAspect={sourceAspect}
						scrollEnabled={scrollEnabled}
						src={embodiment.src}
						poster={embodiment.poster}
						loop={embodiment.loop}
						muted={embodiment.muted}
						controls={embodiment.controls}
						onLoadedMetadata={(event) => {
							const video = event.currentTarget;
							if (video.videoWidth > 0 && video.videoHeight > 0) {
								setSourceAspect(video.videoWidth / video.videoHeight);
							}
						}}
					/>
				</FigureScrollViewport>
			</MediaTeaserWrap>
		</div>
	);
}

function IframeMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
}: {
	readonly morphId: string;
	readonly embodiment: IframeEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	return (
		<div data-id={anchorId} className={morphAnchorClass(emphasis)}>
			<MediaTeaserWrap teaser={embodiment.teaser}>
				<iframe
					className="presentation-media-iframe"
					src={resolvePresentationAssetUrl(embodiment.src)}
					title={embodiment.title ?? ""}
					loading="eager"
				/>
			</MediaTeaserWrap>
		</div>
	);
}

function MarkdownMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
}: {
	readonly morphId: string;
	readonly embodiment: MarkdownEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	const [html, setHtml] = useState("");
	useEffect(() => {
		let cancelled = false;
		void (async () => {
			const markdown =
				embodiment.markdown ??
				(await fetch(resolvePresentationAssetUrl(embodiment.src)).then((response) => response.text()));
			const compiled = await compileMarkdownToHtml(markdown);
			if (!cancelled) {
				setHtml(compiled);
			}
		})();
		return () => {
			cancelled = true;
		};
	}, [embodiment.markdown, embodiment.src]);
	return (
		<div
			data-id={anchorId}
			className={[
				morphAnchorClass(emphasis),
				"presentation-markdown-morph",
				"h-full w-full min-h-0 min-w-0",
			]
				.filter(Boolean)
				.join(" ")}
			aria-label={embodiment.title}
		>
			<Scrollable orientation="both" className="h-full w-full p-small">
				<div
					className="prose prose-sm max-w-none dark:prose-invert presentation-markdown-prose presentation-markdown-prose--top-left"
					dangerouslySetInnerHTML={{ __html: html }}
				/>
			</Scrollable>
		</div>
	);
}

function JsonMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
}: {
	readonly morphId: string;
	readonly embodiment: JsonEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	const [data, setData] = useState<unknown>(embodiment.json ?? null);
	useEffect(() => {
		let cancelled = false;
		if (embodiment.json !== undefined) {
			setData(embodiment.json);
			return;
		}
		void (async () => {
			const text = await fetch(resolvePresentationAssetUrl(embodiment.src)).then((response) => response.text());
			const parsed = JSON.parse(text) as unknown;
			if (!cancelled) {
				setData(parsed);
			}
		})();
		return () => {
			cancelled = true;
		};
	}, [embodiment.json, embodiment.src]);
	return (
		<div
			data-id={anchorId}
			className={[
				morphAnchorClass(emphasis),
				"presentation-json-morph",
				"h-full w-full min-h-0 min-w-0",
			]
				.filter(Boolean)
				.join(" ")}
			aria-label={embodiment.title}
		>
			<Scrollable orientation="both" className="h-full w-full p-small">
				{renderJsonTree(data)}
			</Scrollable>
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
	const enlarged = useContext(PresentationDispositionEnlargeContext);
	const dispositionId = useContext(PresentationInteractiveDispositionIdContext);
	const interaction = useContext(PresentationInteractionContext);
	const declaredPage = pdfEmbodimentInitialPage(embodiment);
	const currentPage =
		dispositionId !== undefined && interaction !== null
			? interaction.getPdfPage(dispositionId, declaredPage)
			: declaredPage;
	const containerSize = usePdfContainerSize(anchorRef, position, slideEpoch, enlarged);
	const [pageViewport, setPageViewport] = useState<{
		readonly width: number;
		readonly height: number;
	} | null>(null);
	const [numPages, setNumPages] = useState<number | null>(null);
	useEffect(() => {
		setPageViewport(null);
	}, [currentPage, embodiment.src, slideEpoch]);
	const pdfSrcRef = useRef(embodiment.src);
	useEffect(() => {
		if (pdfSrcRef.current === embodiment.src) {
			return;
		}
		pdfSrcRef.current = embodiment.src;
		setNumPages(null);
	}, [embodiment.src]);
	const onDocumentLoadSuccess = useCallback(({ numPages: total }: { readonly numPages: number }) => {
		setNumPages(total);
	}, []);
	const onPageLoadSuccess = useCallback(
		(page: { getViewport: (options: { readonly scale: number }) => { readonly width: number; readonly height: number } }) => {
			const viewport = page.getViewport({ scale: 1 });
			setPageViewport({ width: viewport.width, height: viewport.height });
		},
		[],
	);
	const scrollEnabled = pdfEmbodimentScrollEnabled(embodiment);
	const slideAspect = useContext(PresentationSlideAspectContext);
	const estimatedContainer = useMemo(() => {
		const slide = presentationSlidePixelSize(slideAspect);
		return position
			? estimateDispositionFramePixels(position, slide.width, slide.height)
			: { width: 960, height: 700 };
	}, [position, slideAspect]);
	const effectiveContainer =
		containerSize.width !== undefined &&
		containerSize.height !== undefined &&
		containerSize.width > 0 &&
		containerSize.height > 0
			? { width: containerSize.width, height: containerSize.height }
			: estimatedContainer;
	const pageAspect =
		pageViewport !== null && pageViewport.height > 0 ? pageViewport.width / pageViewport.height : 1;
	const scrollAxis =
		scrollEnabled && pageViewport !== null
			? figureCoverOverflowAxis(effectiveContainer.width, effectiveContainer.height, pageAspect)
			: null;
	const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
	const coverScale = useMemo(() => {
		if (pageViewport === null) {
			return null;
		}
		if (effectiveContainer.width <= 0 || effectiveContainer.height <= 0) {
			return null;
		}
		if (scrollEnabled) {
			const port =
				position !== undefined
					? estimatedContainer
					: containerSize.width !== undefined &&
						  containerSize.height !== undefined &&
						  containerSize.width > 0 &&
						  containerSize.height > 0
						? effectiveContainer
						: estimatedContainer;
			return pdfScrollCoverScale(
				port.width,
				port.height,
				pageViewport.width,
				pageViewport.height,
			);
		}
		if (containerSize.width === undefined || containerSize.height === undefined) {
			return null;
		}
		return pdfCoverScale(
			containerSize.width,
			containerSize.height,
			pageViewport.width,
			pageViewport.height,
		);
	}, [
		containerSize.height,
		containerSize.width,
		effectiveContainer.height,
		effectiveContainer.width,
		estimatedContainer.height,
		estimatedContainer.width,
		pageViewport,
		position,
		scrollEnabled,
	]);
	const ready =
		containerSize.width !== undefined &&
		containerSize.height !== undefined &&
		containerSize.width > 0 &&
		containerSize.height > 0;
	const selected =
		dispositionId !== undefined && interaction !== null && interaction.isSelected(dispositionId);
	const showPageNav =
		(enlarged || selected) &&
		dispositionId !== undefined &&
		interaction !== null &&
		pdfPageNavEnabled(embodiment, numPages);
	const goToPage = useCallback(
		(nextPage: number) => {
			if (dispositionId === undefined || interaction === null) {
				return;
			}
			interaction.setPdfPage(dispositionId, nextPage, declaredPage);
		},
		[declaredPage, dispositionId, interaction],
	);
	const goToAdjacentPage = useCallback(
		(direction: "prev" | "next") => {
			goToPage(pdfAdjacentPage(currentPage, direction, embodiment, numPages));
		},
		[currentPage, embodiment, goToPage, numPages],
	);
	return (
		<div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
			<MediaTeaserWrap teaser={embodiment.teaser}>
				<FigureScrollViewport
					enabled={scrollEnabled}
					axis={scrollAxis}
					scrollOrigin={scrollOrigin}
					slideEpoch={slideEpoch}
					style={{ width: "100%", height: "100%" }}
				>
					<Document
					className={[
						"presentation-media-pdf-document",
						scrollEnabled ? "presentation-media-pdf-document--scroll" : undefined,
					]
						.filter(Boolean)
						.join(" ")}
					file={embodiment.src}
					loading={<span className="presentation-media-pdf-loading">…</span>}
					error={<span className="presentation-media-pdf-error">PDF</span>}
					onLoadSuccess={onDocumentLoadSuccess}
				>
					<FigureZoomedPdfPage
						currentPage={currentPage}
						coverScale={coverScale}
						ready={ready}
						onPageLoadSuccess={onPageLoadSuccess}
					/>
				</Document>
			</FigureScrollViewport>
			</MediaTeaserWrap>
			{showPageNav ? (
				<div className="presentation-pdf-page-nav" role="group" aria-label="PDF pages">
					<button
						type="button"
						className="presentation-pdf-page-nav__button presentation-pdf-page-nav__button--prev"
						title="Previous page"
						disabled={!pdfCanGoToPreviousPage(currentPage, embodiment, numPages)}
						onPointerDown={(event) => {
							event.stopPropagation();
						}}
						onClick={(event) => {
							event.stopPropagation();
							goToAdjacentPage("prev");
						}}
					>
						‹
					</button>
					<button
						type="button"
						className="presentation-pdf-page-nav__button presentation-pdf-page-nav__button--next"
						title="Next page"
						disabled={!pdfCanGoToNextPage(currentPage, embodiment, numPages)}
						onPointerDown={(event) => {
							event.stopPropagation();
						}}
						onClick={(event) => {
							event.stopPropagation();
							goToAdjacentPage("next");
						}}
					>
						›
					</button>
				</div>
			) : null}
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
	readonly disposition: RevealResolvedDisposition;
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

function MorphDispositionView({ disposition }: { readonly disposition: RevealResolvedDisposition }): ReactNode {
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
			content = (
				<TextMorphView
					morphId={anchorId}
					embodiment={embodiment}
					emphasis={emphasis}
					anchorOnWrapper={anchorOnWrapper}
				/>
			);
			break;
		case "authors":
			content = (
				<AuthorsMorphView
					morphId={anchorId}
					embodiment={embodiment}
					emphasis={emphasis}
					anchorOnWrapper={anchorOnWrapper}
				/>
			);
			break;
		case "affiliations":
			content = (
				<AffiliationsMorphView
					morphId={anchorId}
					embodiment={embodiment}
					anchorOnWrapper={anchorOnWrapper}
				/>
			);
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
						revealMorphCompanion={disposition.revealMorphCompanion}
						morphFrame={disposition.revealMorphFromFrame}
						morphToFrame={disposition.revealMorphToFrame}
						fromMorphToFrame={disposition.revealMorphFromMorphToFrame}
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
					revealMorphCompanion={disposition.revealMorphCompanion}
				/>
			);
			break;
		case "video":
			content = (
				<VideoMorphView
					morphId={anchorId}
					embodiment={embodiment}
					emphasis={emphasis}
					position={disposition.position}
				/>
			);
			break;
		case "iframe":
			content = <IframeMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
			break;
		case "markdown":
			content = <MarkdownMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
			break;
		case "json":
			content = <JsonMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
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

/** @emoji 🗺 Whether reveal.js is showing the slide grid (Escape overview). */
export function revealDeckInOverview(element: Element | null): boolean {
	return element?.closest(".reveal")?.classList.contains("overview") ?? false;
}

/** @emoji 🗺 Swallow the post-gesture click reveal.js uses to leave overview (capture on `.reveal`). */
export function suppressRevealOverviewSlideNavigation(event: Event): void {
	const origin = event.target;
	if (!(origin instanceof Element)) {
		return;
	}
	const reveal = origin.closest(".reveal");
	if (!reveal) {
		return;
	}
	const swallowClick = (clickEvent: MouseEvent): void => {
		if (!(clickEvent.target instanceof Element) || !clickEvent.target.closest("section")) {
			return;
		}
		clickEvent.preventDefault();
		clickEvent.stopImmediatePropagation();
		reveal.removeEventListener("click", swallowClick, true);
	};
	reveal.addEventListener("click", swallowClick, true);
}

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
	disposition: RevealResolvedDisposition,
	index: number,
): string {
	return `${renderSlideId}--${disposition.morphId}--${disposition.embodimentId ?? index}`;
}

/** @emoji 🔑 Stable id for one split tile on an interactive slide. */
export function tileDispositionInteractionId(
	renderSlideId: string,
	disposition: RevealResolvedDisposition,
	dispositionIndex: number,
	tileKey: string,
): string {
	return `${dispositionInteractionId(renderSlideId, disposition, dispositionIndex)}--tile--${tileKey}`;
}

/** @emoji 🔑 Stable id for a visual row band grouping split tiles on one disposition. */
export function rowBandInteractionId(
	renderSlideId: string,
	disposition: RevealResolvedDisposition,
	dispositionIndex: number,
	rowIndex: number,
): string {
	return `${dispositionInteractionId(renderSlideId, disposition, dispositionIndex)}--row--${rowIndex}`;
}

/** @emoji 🖱 One interactive placement (whole disposition or a single split tile). */
export interface InteractiveDispositionPlacement {
	readonly id: string;
	readonly disposition: RevealResolvedDisposition;
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
	resolved: readonly RevealResolvedDisposition[],
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
					? disposition.revealMorphCompanion !== undefined
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

/** @emoji ↔️ Crossing when dragged right-to-left (end.x < start.x), else window. */
export function marqueeSelectionRule(
	start: { readonly x: number; readonly y: number },
	end: { readonly x: number; readonly y: number },
): MarqueeSelectionRule {
	return end.x < start.x ? "crossing" : "window";
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

/** @emoji ⛶ Centered near-slide frame for interactive enlarge (uniform across figure, video, pdf, tiles). */
export const SLIDE_INTERACTIVE_ENLARGE_FRAME: DispositionPosition = {
	x: 0.05,
	y: 0.075,
	width: 0.9,
	height: 0.85,
};

/** @emoji ⛶ Toggles uniform enlarged slide frame vs stashed pre-enlarge rect. */
export function toggleEnlargeRect(
	current: DispositionPosition,
	stash: DispositionPosition | undefined,
): { readonly rect: DispositionPosition; readonly stash: DispositionPosition | undefined } {
	if (stash !== undefined) {
		return { rect: stash, stash: undefined };
	}
	return { rect: SLIDE_INTERACTIVE_ENLARGE_FRAME, stash: current };
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
	readonly enlarged: boolean;
}): CSSProperties | undefined {
	const { selected, effectiveRect, canvasFramed, enlarged } = options;
	if (!selected || !effectiveRect || enlarged || canvasFramed) {
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
	"[data-id], .presentation-disposition-frame, .presentation-morph-anchor, .presentation-intro-line, .presentation-morph-text, h1, h2, h3, h4, p, li, img, video, iframe, .presentation-media-figure, .presentation-media-iframe, .presentation-figure-crop-fill, .presentation-morph-slot--figure";

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
export function declaredDispositionRect(disposition: RevealResolvedDisposition): DispositionPosition | undefined {
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

/** @emoji ↔️ Node that receives flow `translate3d` during drag (offset wrapper, else content, else disposition root). */
export function flowDragTransformElement(
	sectionEl: HTMLElement,
	root: HTMLElement | null,
	content: HTMLElement | null,
): HTMLElement {
	if (root?.classList.contains("presentation-interactive-disposition--offset")) {
		return root;
	}
	if (content && content.offsetWidth > 0 && content.offsetHeight > 0) {
		return content;
	}
	if (root && root.offsetWidth > 0 && root.offsetHeight > 0) {
		return root;
	}
	return slideCoordinateRoot(sectionEl);
}

/** @emoji ↔️ Pointer travel in screen px → local px for CSS translate on the flow drag target. */
export function flowPointerDeltaToLocal(
	transformEl: HTMLElement,
	startClientX: number,
	startClientY: number,
	currentClientX: number,
	currentClientY: number,
): { readonly dx: number; readonly dy: number } {
	const scale = elementVisualScale(transformEl);
	const safe = Number.isFinite(scale) && scale > 0 ? Math.min(4, Math.max(0.05, scale)) : 1;
	return {
		dx: (currentClientX - startClientX) / safe,
		dy: (currentClientY - startClientY) / safe,
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

const SLIDE_INTERACTION_RESET_PROXIMITY_PX = 72;

/** @emoji 📐 True when any disposition on the slide has ephemeral layout (drag, resize, enlarge, or pdf page). */
export function slideHasEphemeralLayout(
	transforms: ReadonlyMap<string, DispositionTransform>,
	enlargedIds: ReadonlySet<string>,
	declaredRects: ReadonlyMap<string, DispositionPosition | undefined>,
	pdfPageById: ReadonlyMap<string, number> = new Map(),
): boolean {
	if (enlargedIds.size > 0 || pdfPageById.size > 0) {
		return true;
	}
	for (const [id, transform] of transforms) {
		const declared = declaredRects.get(id);
		if (declared === undefined) {
			return true;
		}
		if (dispositionPositionChanged(declared, transform)) {
			return true;
		}
	}
	return false;
}

/** @emoji 📐 True when a disposition transform differs from its declared anchor. */
export function dispositionHasEphemeralLayout(
	transform: DispositionTransform | undefined,
	anchorRect: DispositionPosition | undefined,
	measuredNatural: DispositionPosition | undefined,
	flowLayout: boolean,
): boolean {
	if (!transform) {
		return false;
	}
	if (!flowLayout) {
		return anchorRect !== undefined && dispositionPositionChanged(anchorRect, transform);
	}
	if (measuredNatural) {
		return dispositionPositionChanged(
			flowDispositionManipulationRect(measuredNatural, undefined),
			transform,
		);
	}
	return true;
}

/** @emoji 🎯 Pointer is within the top-right hotspot where the slide reset control lives. */
export function pointerNearSlideResetHotspot(
	section: HTMLElement,
	clientX: number,
	clientY: number,
	proximityPx = SLIDE_INTERACTION_RESET_PROXIMITY_PX,
): boolean {
	const bounds = slideLayoutBounds(section);
	const reachX = Math.max(proximityPx, bounds.width * 0.12);
	const reachY = Math.max(proximityPx, bounds.height * 0.12);
	return (
		clientX >= bounds.right - reachX &&
		clientY >= bounds.top &&
		clientY <= bounds.top + reachY
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
		if (transform.x === 0 && transform.y === 0) {
			return true;
		}
		return !isNormalizedSlideFrame(transform);
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
	readonly enlargedIds: ReadonlySet<string>;
	readonly enlargeStashById: ReadonlyMap<string, DispositionPosition>;
	readonly pdfPageById: ReadonlyMap<string, number>;
	readonly isSelected: (id: string) => boolean;
	readonly isEnlarged: (id: string) => boolean;
	readonly hasPdfPageOverride: (id: string) => boolean;
	readonly getTransform: (id: string) => DispositionTransform | undefined;
	readonly getEnlargeStash: (id: string) => DispositionPosition | undefined;
	readonly getPdfPage: (id: string, defaultPage: number) => number;
	readonly setTransform: (id: string, rect: DispositionTransform) => void;
	readonly setTransforms: (updates: ReadonlyMap<string, DispositionTransform>) => void;
	readonly setPdfPage: (id: string, page: number, defaultPage: number) => void;
	readonly stashEnlargeRect: (id: string, rect: DispositionPosition) => void;
	readonly clearEnlargeStash: (id: string) => void;
	readonly selectIds: (ids: readonly string[], additive: boolean) => void;
	readonly clearSelection: () => void;
	readonly toggleEnlarge: (id: string) => void;
	readonly clearTransform: (id: string) => void;
	readonly resetSlide: (dispositionIds: readonly string[]) => void;
	readonly clearEphemeralLayout: () => void;
}

const PresentationInteractionContext = createContext<PresentationInteractionState | null>(null);

function usePresentationInteractionState(): PresentationInteractionState {
	const value = useContext(PresentationInteractionContext);
	if (!value) {
		throw new Error("Presentation interaction requires PresentationInteractionContext.");
	}
	return value;
}

/** @emoji 🖱 Per-slide selection, transforms, and enlarge flags; cleared only via reset controls. */
export function usePresentationInteraction(): PresentationInteractionState {
	const [selectedIds, setSelectedIds] = useState<ReadonlySet<string>>(() => new Set());
	const [transforms, setTransforms] = useState<ReadonlyMap<string, DispositionTransform>>(() => new Map());
	const [enlargedIds, setEnlargedIds] = useState<ReadonlySet<string>>(() => new Set());
	const [enlargeStashById, setEnlargeStashById] = useState<ReadonlyMap<string, DispositionPosition>>(
		() => new Map(),
	);
	const [pdfPageById, setPdfPageById] = useState<ReadonlyMap<string, number>>(() => new Map());

	const isSelected = useCallback((id: string) => selectedIds.has(id), [selectedIds]);

	const isEnlarged = useCallback((id: string) => enlargedIds.has(id), [enlargedIds]);

	const hasPdfPageOverride = useCallback((id: string) => pdfPageById.has(id), [pdfPageById]);

	const getPdfPage = useCallback(
		(id: string, defaultPage: number) => pdfPageById.get(id) ?? defaultPage,
		[pdfPageById],
	);

	const setPdfPage = useCallback((id: string, page: number, defaultPage: number) => {
		setPdfPageById((previous) => {
			const had = previous.has(id);
			const same = page === defaultPage;
			if (!had && same) {
				return previous;
			}
			if (had && same) {
				const next = new Map(previous);
				next.delete(id);
				return next;
			}
			const next = new Map(previous);
			next.set(id, page);
			return next;
		});
	}, []);

	const getTransform = useCallback((id: string) => transforms.get(id), [transforms]);

	const getEnlargeStash = useCallback((id: string) => enlargeStashById.get(id), [enlargeStashById]);

	const stashEnlargeRect = useCallback((id: string, rect: DispositionPosition) => {
		setEnlargeStashById((previous) => {
			const existing = previous.get(id);
			if (
				existing &&
				!dispositionPositionChanged(existing, rect)
			) {
				return previous;
			}
			const next = new Map(previous);
			next.set(id, rect);
			return next;
		});
	}, []);

	const clearEnlargeStash = useCallback((id: string) => {
		setEnlargeStashById((previous) => {
			if (!previous.has(id)) {
				return previous;
			}
			const next = new Map(previous);
			next.delete(id);
			return next;
		});
	}, []);

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
	}, []);

	const toggleEnlarge = useCallback((id: string) => {
		setEnlargedIds((previous) => {
			const next = new Set(previous);
			if (next.has(id)) {
				next.delete(id);
			} else {
				next.add(id);
			}
			return next;
		});
	}, []);

	const clearTransform = useCallback((id: string) => {
		setTransforms((previous) => {
			if (!previous.has(id)) {
				return previous;
			}
			const next = new Map(previous);
			next.delete(id);
			return next;
		});
	}, []);

	const resetSlide = useCallback((dispositionIds: readonly string[]) => {
		const idSet = new Set(dispositionIds);
		setSelectedIds((previous) => {
			const next = new Set(previous);
			for (const id of idSet) {
				next.delete(id);
			}
			return next;
		});
		setTransforms((previous) => {
			let changed = false;
			for (const id of idSet) {
				if (previous.has(id)) {
					changed = true;
					break;
				}
			}
			if (!changed) {
				return previous;
			}
			const next = new Map(previous);
			for (const id of idSet) {
				next.delete(id);
			}
			return next;
		});
		setEnlargedIds((previous) => {
			const next = new Set(previous);
			for (const id of idSet) {
				next.delete(id);
			}
			return next;
		});
		setEnlargeStashById((previous) => {
			let changed = false;
			for (const id of idSet) {
				if (previous.has(id)) {
					changed = true;
					break;
				}
			}
			if (!changed) {
				return previous;
			}
			const next = new Map(previous);
			for (const id of idSet) {
				next.delete(id);
			}
			return next;
		});
		setPdfPageById((previous) => {
			let changed = false;
			for (const id of idSet) {
				if (previous.has(id)) {
					changed = true;
					break;
				}
			}
			if (!changed) {
				return previous;
			}
			const next = new Map(previous);
			for (const id of idSet) {
				next.delete(id);
			}
			return next;
		});
	}, []);

	const clearEphemeralLayout = useCallback(() => {
		setSelectedIds(new Set());
		setTransforms(new Map());
		setEnlargedIds(new Set());
		setEnlargeStashById(new Map());
		setPdfPageById(new Map());
	}, []);

	return useMemo(
		() => ({
			selectedIds,
			transforms,
			enlargedIds,
			enlargeStashById,
			pdfPageById,
			isSelected,
			isEnlarged,
			hasPdfPageOverride,
			getTransform,
			getEnlargeStash,
			getPdfPage,
			setTransform,
			setTransforms: setTransformsBatch,
			setPdfPage,
			stashEnlargeRect,
			clearEnlargeStash,
			selectIds,
			clearSelection,
			toggleEnlarge,
			clearTransform,
			resetSlide,
			clearEphemeralLayout,
		}),
		[
			selectedIds,
			transforms,
			enlargedIds,
			enlargeStashById,
			pdfPageById,
			isSelected,
			isEnlarged,
			hasPdfPageOverride,
			getTransform,
			getEnlargeStash,
			getPdfPage,
			setTransform,
			setTransformsBatch,
			setPdfPage,
			stashEnlargeRect,
			clearEnlargeStash,
			selectIds,
			clearSelection,
			toggleEnlarge,
			clearTransform,
			resetSlide,
			clearEphemeralLayout,
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
	readonly disposition: RevealResolvedDisposition;
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
	const enlarged = interaction.isEnlarged(id);
	const morphGhost = isRevealMorphCompanionOnly(disposition);
	const canvasFramed = declaredRect !== undefined && !enlarged;
	const canvasPlacement = interactionRect !== undefined;
	const effectiveRect = resolveEffectiveDispositionRect(id, interactionRect, interaction, registry);
	const canvasAnchorRect = interactionRect;
	const canvasLiveTransform =
		canvasFramed && transform && canvasAnchorRect ? transform : undefined;
	const canvasDragActive = Boolean(
		canvasLiveTransform &&
			canvasAnchorRect &&
			!enlarged &&
			dispositionPositionChanged(canvasAnchorRect, canvasLiveTransform),
	);
	const pinned =
		!enlarged &&
		((transformed && !flowLayout && !canvasFramed) ||
			flowSectionFrame ||
			(canvasFramed && canvasDragActive && !flowPixelOffset));
	const [gesturing, setGesturing] = useState(false);
	const [flowReservePx, setFlowReservePx] = useState<{ readonly width: number; readonly height: number } | null>(
		null,
	);
	const useFlowInkFrame = flowLayout && !flowSectionFrame;
	const [inkInWrapper, setInkInWrapper] = useState<DispositionPosition | null>(null);
	const [enlargedFlowContentScale, setEnlargedFlowContentScale] = useState<number | null>(null);
	const displayDisposition =
		enlarged && disposition.position !== undefined
			? disposition.embodiment.kind === "figure" && disposition.embodiment.crop !== undefined
				? { ...disposition, position: SLIDE_INTERACTIVE_ENLARGE_FRAME }
				: { ...disposition, position: undefined }
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
		if (!section || !section.classList.contains("present") || isRevealSlideAutoAnimating(section)) {
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
		if (gesturing) {
			return;
		}
		const section = sectionRef.current;
		if (section && isRevealSlideAutoAnimating(section)) {
			return;
		}
		const root = rootRef.current;
		if (!root) {
			setInkInWrapper(null);
			return;
		}
		setInkInWrapper(measureDispositionBoundsInContainer(root, root));
	}, [useFlowInkFrame, selected, gesturing, flowPixelOffset, transform, slideEpoch, disposition, sectionRef]);

	useLayoutEffect(() => {
		if (!enlarged || !flowLayout) {
			setEnlargedFlowContentScale(null);
			return;
		}
		const section = sectionRef.current;
		const root = rootRef.current;
		if (!section || !root || !section.classList.contains("present")) {
			setEnlargedFlowContentScale(null);
			return;
		}
		const baseline = measureDispositionBoundsInSection(root, section);
		if (!baseline) {
			setEnlargedFlowContentScale(null);
			return;
		}
		setEnlargedFlowContentScale(
			interactiveDispositionContentScale(SLIDE_INTERACTIVE_ENLARGE_FRAME, baseline),
		);
	}, [enlarged, flowLayout, slideEpoch, sectionRef]);

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
				const natural = memberId === id ? initialRect : registry.getRect(memberId);
				let rect: DispositionPosition | undefined = interaction.getTransform(memberId);
				if (!rect) {
					if (flowLayout && natural) {
						rect = flowDispositionManipulationRect(natural, undefined);
					} else if (natural) {
						rect =
							memberDeclared !== undefined
								? natural
								: flowDispositionManipulationRect(natural, undefined);
					} else {
						rect = memberDeclared;
					}
				} else if (flowLayout && natural) {
					rect = isFlowPixelOffsetTransform(rect, natural)
						? rect
						: flowDispositionManipulationRect(natural, undefined);
				}
				if (rect && (flowLayout || memberDeclared !== undefined || isUsableMeasuredRect(rect))) {
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
			if (flowLayout) {
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
						if (flowLayout) {
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
				if (dragging) {
					suppressRevealOverviewSlideNavigation(upEvent);
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
		[id, flowLayout, canvasPlacement, allDeclaredRects, interaction, registry, sectionRef],
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
		const target = event.target instanceof Element ? event.target : null;
		if (target?.closest(".presentation-interaction-handle")) {
			return;
		}
		if (target?.closest(".presentation-interaction-enlarge")) {
			return;
		}
		if (target?.closest(".presentation-interaction-reset")) {
			return;
		}
		if (target?.closest(".presentation-pdf-page-nav")) {
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

	const onEnlargeClick = (event: React.MouseEvent): void => {
		event.preventDefault();
		event.stopPropagation();
		if (!selected) {
			interaction.selectIds([id], false);
		}
		if (enlarged) {
			const stashed = interaction.getEnlargeStash(id);
			interaction.toggleEnlarge(id);
			interaction.clearEnlargeStash(id);
			if (stashed) {
				if (interactionRect && !dispositionPositionChanged(interactionRect, stashed)) {
					interaction.clearTransform(id);
				} else {
					interaction.setTransform(id, stashed);
				}
			}
			return;
		}
		const preEnlargeRect = effectiveRect ?? interactionRect;
		if (preEnlargeRect) {
			interaction.stashEnlargeRect(id, preEnlargeRect);
		}
		interaction.toggleEnlarge(id);
	};

	const onResetClick = (event: React.MouseEvent): void => {
		event.preventDefault();
		event.stopPropagation();
		if (!selected) {
			interaction.selectIds([id], false);
		}
		if (enlarged) {
			const stashed = interaction.getEnlargeStash(id);
			interaction.toggleEnlarge(id);
			interaction.clearEnlargeStash(id);
			if (stashed && interactionRect && !dispositionPositionChanged(interactionRect, stashed)) {
				interaction.clearTransform(id);
			} else if (stashed) {
				interaction.setTransform(id, stashed);
			} else {
				interaction.clearTransform(id);
			}
			return;
		}
		interaction.clearTransform(id);
	};

	const morphCropFrom =
		disposition.revealMorphCompanion === "target" &&
		disposition.revealMorphFromFrame !== undefined &&
		disposition.position !== undefined;
	const morphCropTo =
		disposition.position !== undefined &&
		(disposition.revealMorphFromMorphToFrame !== undefined || disposition.revealMorphToFrame !== undefined);
	const wrapperClass = [
		"presentation-interactive-disposition",
		`presentation-interactive-disposition--kind-${disposition.embodiment.kind}`,
		selected ? "presentation-interactive-disposition--selected" : undefined,
		flowPixelOffset ? "presentation-interactive-disposition--offset" : undefined,
		pinned ? "presentation-interactive-disposition--pinned" : undefined,
		canvasFramed ? "presentation-interactive-disposition--canvas-framed" : undefined,
		gesturing ? "presentation-interactive-disposition--gesturing" : undefined,
		enlarged ? "presentation-interactive-disposition--enlarged" : undefined,
		disposition.revealMorphCompanion === "target" ? "presentation-target-ghost" : undefined,
		disposition.revealMorphCompanion === "source" ? "presentation-source-ghost" : undefined,
		morphCropFrom ? "presentation-morph-crop-from" : undefined,
		morphCropTo ? "presentation-morph-crop-to" : undefined,
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
	const flowDragOffsetStyle =
		flowPixelOffset && transform ? flowDispositionOffsetStyle(transform) : undefined;
	if (flowDragOffsetStyle) {
		Object.assign(wrapperFrame, flowDragOffsetStyle);
	}
	if (canvasFramed && canvasAnchorRect) {
		// 🔀 The wrapper owns the reveal `data-id` morph anchor; placing it on the live ephemeral
		// rect (drag/resize) makes reveal.js auto-animate capture the modified frame as the morph
		// `from`, so morphs start from the current disposition including ephemeral modifications.
		// 👻 Target ghosts keep label layout; live source frames are synced from slide 8 DOM before morph.
		const morphAnchorRect =
			disposition.revealMorphCompanion === "target"
				? canvasAnchorRect
				: disposition.revealMorphCompanion === "source"
					? canvasAnchorRect
					: (canvasLiveTransform ?? canvasAnchorRect);
		Object.assign(wrapperFrame, transformFrameStyle(morphAnchorRect));
		if (
			morphCropFrom &&
			disposition.revealMorphFromFrame !== undefined &&
			disposition.position !== undefined
		) {
			for (const [key, value] of Object.entries(
				morphFrameCssVars(disposition.revealMorphFromFrame, disposition.position),
			)) {
				if (typeof value === "string") {
					(wrapperFrame as Record<string, string>)[key] = value;
				}
			}
		}
	} else if (transformed && transform && !flowPixelOffset) {
		Object.assign(wrapperFrame, transformFrameStyle(transform));
	}
	if (morphGhost) {
		Object.assign(wrapperFrame, { pointerEvents: "none", zIndex: 0 });
	}
	const wrapperStyle: CSSProperties | undefined = enlarged
		? transformFrameStyle(SLIDE_INTERACTIVE_ENLARGE_FRAME)
		: Object.keys(wrapperFrame).length > 0
			? wrapperFrame
			: undefined;
	const resizeContentBaseline = measuredNatural ?? interactionRect;
	const resizeContentScale =
		!enlarged &&
		!canvasFramed &&
		!flowPixelOffset &&
		transform &&
		resizeContentBaseline
			? interactiveDispositionContentScale(transform, resizeContentBaseline)
			: null;
	const contentScale = enlargedFlowContentScale ?? resizeContentScale;
	const contentStyle: CSSProperties | undefined =
		contentScale !== null ? interactiveDispositionContentScaleStyle(contentScale) : undefined;
	const hasContentStyle = contentStyle !== undefined;
	const chromeLayoutRect =
		flowSectionFrame && transform
			? transform
			: canvasLiveTransform ?? effectiveRect;
	const sectionAnimating =
		sectionRef.current !== null && isSectionAutoAnimating(sectionRef.current);
	const flowInkChromeRect = useFlowInkFrame ? inkInWrapper : undefined;
	const chromeStyle: CSSProperties | undefined = interactiveDispositionChromeStyle({
		selected: (selected || gesturing) && !sectionAnimating,
		effectiveRect: flowInkChromeRect ?? chromeLayoutRect,
		canvasFramed,
		enlarged,
	});
	const showControls =
		!morphGhost &&
		(selected || gesturing || enlarged) &&
		Boolean(
			enlarged ||
				selected ||
				gesturing ||
				(useFlowInkFrame ? inkInWrapper : chromeLayoutRect ?? effectiveRect),
		);
	const showHandles = showControls && !enlarged;
	const canResetPosition =
		enlarged ||
		dispositionHasEphemeralLayout(transform, interactionRect, measuredNatural, flowLayout);

	return (
		<div
			ref={rootRef}
			data-disposition-id={id}
			{...(revealMorphId ? { "data-id": revealMorphId } : {})}
			{...(rowBandId ? { "data-row-band": rowBandId } : {})}
			className={wrapperClass}
			style={wrapperStyle}
			onPointerDown={morphGhost ? undefined : onPointerDown}
			aria-hidden={morphGhost ? true : undefined}
		>
			<div
				ref={contentRef}
				className="presentation-interactive-disposition__content"
				style={hasContentStyle ? contentStyle : undefined}
			>
				<PresentationInteractiveDispositionIdContext.Provider value={id}>
					<PresentationDispositionEnlargeContext.Provider value={enlarged}>
						<MorphAnchorOnWrapperContext.Provider
							value={Boolean(revealMorphId && declaredRect !== undefined)}
						>
							<PresentationFigureCropFrameContext.Provider
								value={canvasLiveTransform ?? displayDisposition.position}
							>
								<MorphDispositionView disposition={displayDisposition} />
							</PresentationFigureCropFrameContext.Provider>
						</MorphAnchorOnWrapperContext.Provider>
					</PresentationDispositionEnlargeContext.Provider>
				</PresentationInteractiveDispositionIdContext.Provider>
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
			</div>
			{showControls ? (
				<div className="presentation-interaction-actions">
					{canResetPosition ? (
						<button
							type="button"
							className="presentation-interaction-reset"
							title="Reset position"
							onClick={onResetClick}
						>
							<Icon icon="rotate-ccw" size="small" />
						</button>
					) : null}
					<button
						type="button"
						className="presentation-interaction-enlarge"
						title={enlarged ? "Exit enlarge" : "Enlarge"}
						aria-pressed={enlarged}
						onClick={onEnlargeClick}
					>
						<Icon icon="maximize-2" size="small" />
					</button>
				</div>
			) : null}
		</div>
	);
};

function isDispositionPointerTarget(target: EventTarget | null): boolean {
	if (!(target instanceof Element)) {
		return false;
	}
	if (
		target.closest(".presentation-target-ghost") ||
		target.closest(".presentation-source-ghost")
	) {
		return false;
	}
	return Boolean(
		target.closest(".presentation-interactive-disposition") ||
			target.closest(".presentation-interactive-row-band") ||
			target.closest(".presentation-interactive-visual-row") ||
			target.closest(".presentation-interaction-handle") ||
			target.closest(".presentation-interaction-enlarge") ||
			target.closest(".presentation-pdf-page-nav__button") ||
			target.closest(".presentation-interaction-reset") ||
			target.closest(".presentation-interaction-slide-reset") ||
			target.closest(".presentation-interaction-slide-reset-host"),
	);
}

function isRevealSlideBackgroundPointerTarget(target: EventTarget | null): boolean {
	return (
		target instanceof Element &&
		Boolean(target.closest(".slide-background.present, .slide-background-content"))
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
			const startedInOverview = revealDeckInOverview(section);
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
					if (!startedInOverview) {
						interaction.clearSelection();
					}
					return;
				}
				suppressRevealOverviewSlideNavigation(upEvent);
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

const SlideInteractionReset: FC<{
	readonly sectionRef: RefObject<HTMLElement | null>;
	readonly dispositionIds: readonly string[];
	readonly declaredRects: ReadonlyMap<string, DispositionPosition | undefined>;
}> = ({ sectionRef, dispositionIds, declaredRects }) => {
	const interaction = usePresentationInteractionState();
	const hostRef = useRef<HTMLDivElement>(null);
	const modified = dispositionIds.some((id) => {
		if (interaction.isEnlarged(id)) {
			return true;
		}
		if (interaction.hasPdfPageOverride(id)) {
			return true;
		}
		const transform = interaction.getTransform(id);
		const declared = declaredRects.get(id);
		return dispositionHasEphemeralLayout(transform, declared, undefined, declared === undefined);
	});

	useEffect(() => {
		if (!modified) {
			return;
		}
		const section = sectionRef.current;
		const host = hostRef.current;
		if (!section || !host) {
			return;
		}
		const syncNear = (clientX: number, clientY: number): void => {
			host.classList.toggle(
				"presentation-interaction-slide-reset-host--near",
				pointerNearSlideResetHotspot(section, clientX, clientY),
			);
		};
		const onMove = (event: PointerEvent): void => {
			syncNear(event.clientX, event.clientY);
		};
		const onLeave = (): void => {
			host.classList.remove("presentation-interaction-slide-reset-host--near");
		};
		window.addEventListener("pointermove", onMove, { passive: true });
		section.addEventListener("pointerleave", onLeave);
		return () => {
			window.removeEventListener("pointermove", onMove);
			section.removeEventListener("pointerleave", onLeave);
			host.classList.remove("presentation-interaction-slide-reset-host--near");
		};
	}, [declaredRects, modified, sectionRef]);

	if (!modified) {
		return null;
	}

	return (
		<div ref={hostRef} className="presentation-interaction-slide-reset-host">
			<button
				type="button"
				className="presentation-interaction-slide-reset"
				title="Reset slide"
				onClick={(event) => {
					event.preventDefault();
					event.stopPropagation();
					interaction.resetSlide(dispositionIds);
				}}
			>
				<Icon icon="rotate-ccw" size="small" />
			</button>
		</div>
	);
};

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
				<SelectionMarquee
					coverage={marqueeRule === "crossing" ? "partial" : "full"}
					shape="rect"
					rect={{
						x: marqueeStyle.left,
						y: marqueeStyle.top,
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
	const slideContext = useMemo(() => {
		const index = thought.slides.findIndex((slide) => slide.arrangement.id === renderSlide.id);
		if (index < 0) {
			return {};
		}
		return {
			previousSlide: index > 0 ? thought.slides[index - 1] : undefined,
			nextSlide: index < thought.slides.length - 1 ? thought.slides[index + 1] : undefined,
		};
	}, [thought.slides, renderSlide.id]);
	const resolved = useMemo(
		() => resolveRevealArrangement(scope, renderSlide.arrangement, slideContext),
		[scope, renderSlide.arrangement, slideContext],
	);
	const morph = renderSlide.autoAnimateId !== undefined;
	const positioned = resolved.some((disposition) => disposition.position !== undefined);
	const layoutResolved =
		positioned && !morph ? centerRevealResolvedArrangement(resolved) : resolved;
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
	const interactiveDispositionIds = useMemo(
		() =>
			interactiveLayout.placements
				.filter((entry) => !isRevealMorphCompanionOnly(entry.disposition))
				.map((entry) => entry.id),
		[interactiveLayout.placements],
	);
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
				dispositionIds={interactiveDispositionIds}
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
				isIntroArrangementId(slideId) ? "presentation-arrangement--intro" : undefined,
			]
				.filter(Boolean)
				.join(" ")}
		>
			{positioned ? (
				<div className="presentation-arrangement-surface">
					<InteractionLayer marquee={backgroundInteraction.marquee} />
					<div className="presentation-arrangement-canvas">{placements}</div>
					<SlideInteractionReset
						sectionRef={sectionRef}
						dispositionIds={dispositionIds}
						declaredRects={declaredRects}
					/>
				</div>
			) : (
				<>
					<InteractionLayer marquee={backgroundInteraction.marquee} />
					{placements}
					<SlideInteractionReset
						sectionRef={sectionRef}
						dispositionIds={dispositionIds}
						declaredRects={declaredRects}
					/>
				</>
			)}
		</section>
	);
};
//#endregion 🔖ArrangementSection

//#region 🔖PresentationInteractionProvider
/** @emoji 🖱 Clears selection and ephemeral layout when reveal.js changes the active slide. */
const PresentationSlideInteractionBridge: FC<{
	readonly deckRef: RefObject<Reveal.Api | null>;
}> = ({ deckRef }) => {
	const interaction = usePresentationInteractionState();

	useEffect(() => {
		const attach = (deck: Reveal.Api): (() => void) => {
			const onSlideChanged = (): void => {
				interaction.clearEphemeralLayout();
			};
			const deckEl = deck.getRevealElement();
			const onPointerDown = (event: PointerEvent): void => {
				if (
					event.button === 0 &&
					deckEl &&
					!deckEl.classList.contains("overview") &&
					isRevealSlideBackgroundPointerTarget(event.target)
				) {
					interaction.clearSelection();
				}
			};
			deck.on("slidechanged", onSlideChanged);
			deckEl?.addEventListener("pointerdown", onPointerDown, { capture: true });
			return () => {
				deck.off("slidechanged", onSlideChanged);
				deckEl?.removeEventListener("pointerdown", onPointerDown, { capture: true });
			};
		};

		const deck = deckRef.current;
		if (deck) {
			return attach(deck);
		}

		let detach: (() => void) | undefined;
		const poll = window.setInterval(() => {
			const ready = deckRef.current;
			if (!ready) {
				return;
			}
			window.clearInterval(poll);
			detach = attach(ready);
		}, 50);
		return () => {
			window.clearInterval(poll);
			detach?.();
		};
	}, [deckRef, interaction]);

	return null;
};

const PresentationInteractionProvider: FC<{
	readonly children: ReactNode;
}> = ({ children }) => {
	const interaction = usePresentationInteraction();
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
	const [slideAspect, setSlideAspect] = useState<number | undefined>(undefined);
	const syncSlideAspect = useCallback((deckEl: HTMLElement): void => {
		const { width, height } = parsePresentationSlideCssSize(deckEl);
		setSlideAspect(width / height);
	}, []);

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
			syncSlideAspect(deckEl);
		};
		let autoAnimateFinalizeTimer: ReturnType<typeof setTimeout> | undefined;
		let pendingAutoAnimateRunSlides: PresentationAutoAnimateRunSlides | undefined;
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
			relaxHiddenPreflight();
			const slideEvent = event as Event & { readonly indexh?: number; readonly indexv?: number };
			const fromSlide = deck.getCurrentSlide() as HTMLElement | null;
			if (!fromSlide || slideEvent.indexh === undefined || slideEvent.indexv === undefined) {
				return;
			}
			const toSlide = resolveRevealSlideAt(deckEl, { h: slideEvent.indexh, v: slideEvent.indexv });
			if (!toSlide) {
				return;
			}
			pendingAutoAnimateRunSlides = { fromSlide, toSlide };
			const fromAnimateId = fromSlide.getAttribute("data-auto-animate-id");
			const toAnimateId = toSlide.getAttribute("data-auto-animate-id");
			if (!fromAnimateId || fromAnimateId !== toAnimateId) {
				finalizeRevealAutoAnimateRestState(deckEl);
			}
			prepareArrangementBeforeAutoAnimate(fromSlide, toSlide);
		};
		const onSlideChanged = (event: Event): void => {
			relaxHiddenPreflight();
			syncRevealBackgroundKind(deckEl);
			syncPresentationSlideSizeVars(deckEl, deck);
			syncSlideAspect(deckEl);
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
			if (!slidesShareAutoAnimateId(previousSlide, currentSlide)) {
				setSlideEpoch((epoch) => epoch + 1);
			}
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
			const runSlides = resolvePresentationAutoAnimateRunSlides(
				{ fromSlide, toSlide },
				pendingAutoAnimateRunSlides,
			);
			if (runSlides.fromSlide && runSlides.toSlide) {
				prepareArrangementBeforeAutoAnimate(runSlides.fromSlide, runSlides.toSlide);
			}
			const sheet = animateEvent.sheet;
			if (sheet && typeof sheet.innerHTML === "string") {
				const durationSeconds =
					typeof deck.getConfig().autoAnimateDuration === "number"
						? deck.getConfig().autoAnimateDuration
						: 1;
				patchPresentationAutoAnimateRunStyleSheet(
					sheet,
					durationSeconds,
					runSlides.fromSlide,
					runSlides.toSlide,
				);
			}
			scheduleFinalizeAutoAnimateRest();
		};
		void deck.initialize().then(() => {
			relaxHiddenPreflight();
			syncRevealBackgroundKind(deckEl);
			syncPresentationSlideSizeVars(deckEl, deck);
			syncSlideAspect(deckEl);
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
							void slideResult.then(() => {
								afterSlideSync();
								finalizeRevealAutoAnimateRestState(deckEl);
								scheduleFinalizeAutoAnimateRest();
							});
						} else {
							afterSlideSync();
							finalizeRevealAutoAnimateRestState(deckEl);
							scheduleFinalizeAutoAnimateRest();
						}
					} else {
						afterSlideSync();
						finalizeRevealAutoAnimateRestState(deckEl);
						scheduleFinalizeAutoAnimateRest();
					}
				} else {
					afterSlideSync();
					finalizeRevealAutoAnimateRestState(deckEl);
					scheduleFinalizeAutoAnimateRest();
				}
				window.addEventListener("hashchange", onWindowHashChange);
			} else {
				const currentSlide = deck.getCurrentSlide();
				syncArrangementSettledState(deckEl, currentSlide, previousSlideRef.current);
				previousSlideRef.current = currentSlide;
				setSlideEpoch((epoch) => epoch + 1);
				finalizeRevealAutoAnimateRestState(deckEl);
				scheduleFinalizeAutoAnimateRest();
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
			<PresentationSlideAspectContext.Provider value={slideAspect}>
			<PresentationInteractionProvider>
				<PresentationSlideInteractionBridge deckRef={deckRef} />
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
			</PresentationSlideAspectContext.Provider>
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

	function stockRevealAutoAnimateMatcherHost(): AutoAnimateMatcherHost {
		return {
			findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
				const reserved = new Set<HTMLElement>();
				for (const fromElement of fromScope.querySelectorAll<HTMLElement>(selector)) {
					const fromKey = serializer(fromElement);
					for (const toElement of toScope.querySelectorAll<HTMLElement>(selector)) {
						if (reserved.has(toElement) || serializer(toElement) !== fromKey) {
							continue;
						}
						pairs.push({ from: fromElement, to: toElement });
						reserved.add(toElement);
						break;
					}
				}
			},
		};
	}

	describe("resolveRevealArrangement", () => {
		it("appends target companions for morphFrom on the labels slide", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "col1" }, { id: "tile" }],
					embodiments: [
						{ kind: "text", id: "label", lines: ["A"], level: "heading" },
						{ kind: "figure", id: "tile-figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
					],
				},
			]);
			const previousSlide: Slide = {
				arrangement: {
					id: "focus",
					dispositions: [
						{
							participantId: "tile",
							embodimentId: "tile-figure",
							emphasis: "active",
							position: { x: 0.5, y: 0.5, width: 0.2, height: 0.2 },
						},
					],
				},
			};
			const resolved = resolveRevealArrangement(
				scope,
				{
					id: "labels",
					dispositions: [
						{
							participantId: "col1",
							embodimentId: "label",
							emphasis: "active",
							morphFrom: [
								{
									participantId: "tile",
									embodimentId: "tile-figure",
									position: { x: 0.2, y: 0.3, width: 0.2, height: 0.1 },
								},
							],
						},
					],
				},
				{ previousSlide },
			);
			expect(resolved).toHaveLength(2);
			const companion = resolved.find((entry) => entry.revealMorphCompanion === "target");
			expect(companion?.morphId).toBe("tile");
			expect(companion?.position).toEqual({ x: 0.2, y: 0.3, width: 0.2, height: 0.1 });
			expect(companion?.revealMorphFromFrame).toEqual({ x: 0.5, y: 0.5, width: 0.2, height: 0.2 });
		});

		it("sets revealMorphToFrame on focus tiles when the next slide morphFrom references them", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "tile" }, { id: "col1" }],
					embodiments: [
						{ kind: "figure", id: "tile-figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
						{ kind: "text", id: "label", lines: ["A"], level: "heading" },
					],
				},
			]);
			const focusPosition = { x: 0.5, y: 0.5, width: 0.2, height: 0.2 };
			const labelPosition = { x: 0.2, y: 0.3, width: 0.2, height: 0.1 };
			const resolved = resolveRevealArrangement(
				scope,
				{
					id: "focus",
					dispositions: [
						{
							participantId: "tile",
							embodimentId: "tile-figure",
							emphasis: "active",
							position: focusPosition,
						},
					],
				},
				{
					nextSlide: {
						arrangement: {
							id: "labels",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "label",
									emphasis: "active",
									morphFrom: [
										{
											participantId: "tile",
											embodimentId: "tile-figure",
											position: labelPosition,
										},
									],
								},
							],
						},
					},
				},
			);
			expect(resolved.find((entry) => entry.participant.id === "tile")?.revealMorphToFrame).toEqual(
				labelPosition,
			);
		});

		it("morphFrameCssVars maps normalized frames to presentation custom properties", () => {
			expect(
				morphFrameCssVars(
					{ x: 0.1, y: 0.2, width: 0.3, height: 0.4 },
					{ x: 0.5, y: 0.6, width: 0.2, height: 0.1 },
				),
			).toEqual({
				"--presentation-morph-frame-left": "10%",
				"--presentation-morph-frame-top": "20%",
				"--presentation-morph-frame-width": "30%",
				"--presentation-morph-frame-height": "40%",
				"--presentation-frame-left": "50%",
				"--presentation-frame-top": "60%",
				"--presentation-frame-width": "20%",
				"--presentation-frame-height": "10%",
			});
		});

		it("keeps target-ghost rest crop on the source tile frame and morph crop on the label slot", () => {
			const crop = { x: 0.8, y: 0.7, width: 0.15, height: 0.2 };
			const embodiment = { kind: "figure" as const, src: "/catalogue.png", crop };
			const sourceFrame = { x: 0.77, y: 0.11, width: 0.2, height: 0.78 };
			const labelFrame = { x: 0.653333, y: 0.44, width: 0.246667, height: 0.12 };
			const vars = figureCropBackgroundVarsTargetGhost(embodiment, crop, sourceFrame, labelFrame);
			const sourceOnly = figureCropBackgroundVars(embodiment, crop, sourceFrame);
			const labelOnly = figureCropBackgroundVars(embodiment, crop, labelFrame);
			expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe(
				sourceOnly["--presentation-figure-bg-size" as keyof typeof sourceOnly],
			);
			expect(vars["--presentation-figure-bg-size-morph" as keyof typeof vars]).toBe(
				labelOnly["--presentation-figure-bg-size" as keyof typeof labelOnly],
			);
		});

		it("appends source companions for morphTo on the whole slide", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "whole" }, { id: "tile-a" }],
					embodiments: [
						{ kind: "figure", id: "whole--figure", src: "/a.png" },
						{ kind: "figure", id: "tile-a--figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
					],
				},
			]);
			const nextSlide: Slide = {
				arrangement: {
					id: "tiles",
					dispositions: [
						{ participantId: "tile-a", embodimentId: "tile-a--figure", emphasis: "active" },
					],
				},
			};
			const resolved = resolveRevealArrangement(
				scope,
				{
					id: "whole",
					dispositions: [
						{
							participantId: "whole",
							embodimentId: "whole--figure",
							emphasis: "active",
							morphTo: [{ participantId: "tile-a", position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 } }],
						},
					],
				},
				{ nextSlide },
			);
			expect(resolved).toHaveLength(2);
			const companion = resolved.find((entry) => entry.revealMorphCompanion === "source");
			expect(companion?.morphId).toBe("tile-a");
		});
	});

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
			expect(container.querySelector('[data-id^="goal"]')).toBeTruthy();
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
				expect(slide.classList.contains("presentation-arrangement--intro")).toBe(true);
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
			expect(slide("affiliations-3")?.textContent).toContain("Chair X");
			expect(slide("affiliations-3")?.querySelector(".presentation-affiliation-morph-source")).toBeNull();
			expect(
				slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.classList.contains("presentation-affiliation-row"),
			).toBe(true);
			expect(slide("affiliations-3")?.querySelector('[data-id="institutions--x"]')?.textContent).toContain("Chair X");
			expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"] [data-id="institutions--x"]')).toBeTruthy();
			expect(slide("affiliations-1")?.querySelector('h4[data-id="authors--Alice Example"] sup')?.textContent).toBe("a");
			const marked2 = slide("affiliations-2")?.querySelector('h4[data-id="authors--Alice Example"] sup');
			expect(marked2?.textContent).toBe("a,1");
			expect(marked2?.querySelector("span:not(.opacity-20)")?.textContent).toBe("1");
			const marked3 = slide("affiliations-3")?.querySelector('h4[data-id="authors--Alice Example"] sup');
			expect(marked3?.textContent).toBe("a,1,x");
			expect(marked3?.querySelector("span:not(.opacity-20)")?.textContent).toBe("x");
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
				'h2[data-id^="description"], p[data-id^="description"], h2[data-id^="goal"], p[data-id^="goal"]',
				"presentation-morph-text--secondary",
			);
			expect(container.querySelector('h2[data-id^="title"].presentation-morph-text--secondary')).toBeNull();
			expect(container.querySelector('h2[data-id^="goal"].presentation-morph-text--title')).toBeNull();
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

		it("enables reveal auto-animate on intro morph slides with unmatched layering", () => {
			const deck = intro({ language: "de",
				title: { full: ["A"], short: "Short" },
				description: { full: ["D"], short: "D short" },
				goal: ["G1", "G2"],
				authors: {
					lines: [
						[{ name: "Alice", marks: ["a"] }, { name: "Bob", marks: ["a"] }],
						[{ name: "Carol", marks: ["a"] }],
					],
				},
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			for (const slide of container.querySelectorAll(
				'.slides > section > section[data-auto-animate-id="einleitung--m0"]',
			)) {
				expect(slide.classList.contains("presentation-arrangement--intro")).toBe(true);
				expect(slide.hasAttribute("data-auto-animate")).toBe(true);
				expect(slide.querySelector(".presentation-arrangement-surface")).toBeNull();
			}
			expect(globalsCssSource).toMatch(
				/\.presentation-arrangement--intro:not\(\.presentation-arrangement--positioned\)[\s\S]*:is\(h1,\s*h2,\s*h3,\s*h4,\s*p\)[\s\S]*margin:\s*0/s,
			);
			expect(globalsCssSource).not.toMatch(/data-auto-animate-id\^="intro--"/);
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
			expect(slide("title")?.querySelector('h2[data-id^="title"]')).toBeTruthy();
			expect(slide("title")?.querySelector('[data-id^="description"]')).toBeNull();
			expect(slide("description")?.querySelector('h2[data-id^="description"]')).toBeTruthy();
			expect(slide("goal")?.querySelector('h2[data-id^="goal"]')).toBeTruthy();
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

		it("renders glassy teaser veils on media embodiments with optional labels", () => {
			const deck: Presentation = {
				id: "teaser-test",
				name: "Teaser",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "teaser",
								thoughts: [
									{
										id: "teaser",
										participants: [{ id: "fig" }, { id: "clip" }, { id: "embed" }],
										embodiments: [
											{ kind: "figure", id: "fig--figure", src: "/a.png", teaser: {} },
											{
												kind: "video",
												id: "clip--video",
												src: "/demo.mp4",
												teaser: { label: "Coming soon" },
											},
											{
												kind: "iframe",
												id: "embed--iframe",
												src: "/demo.html",
												teaser: { label: "Interactive demo" },
											},
										],
										slides: [
											{
												arrangement: {
													id: "slide",
													dispositions: [
														{ participantId: "fig", embodimentId: "fig--figure", emphasis: "active" },
														{ participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
														{ participantId: "embed", embodimentId: "embed--iframe", emphasis: "active" },
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
			const figureVeil = container.querySelector('[data-id="fig"] .presentation-media-teaser__veil');
			const videoLabel = container.querySelector(
				'[data-id="clip"] .presentation-media-teaser__label',
			);
			const iframeVeil = container.querySelector('[data-id="embed"] .presentation-media-teaser__veil');
			expect(figureVeil).toBeTruthy();
			expect(videoLabel?.textContent).toBe("Coming soon");
			expect(iframeVeil?.querySelector(".presentation-media-teaser__label")?.textContent).toBe(
				"Interactive demo",
			);
			expect(container.querySelector('[data-id="embed"] iframe.presentation-media-iframe')).toBeTruthy();
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
			expect(tileFrame.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
			const tiles = [...container.querySelectorAll(".presentation-morph-slot--figure")] as HTMLElement[];
			for (const node of tiles) {
				expect(node.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
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
			const pairs = presentationAutoAnimateMatcher.call(
				{ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost,
				fromSlide!,
				toSlide!,
			);
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
			const slot = container.querySelector('[data-id="catalogue-col1"]') as HTMLElement | null;
			const cropSlot = slot?.querySelector(".presentation-morph-slot--figure") as HTMLElement | null;
			expect(cropSlot?.style.backgroundImage).toContain("/catalogue.png");
			expect(cropSlot?.querySelector("h2")).toBeNull();
			expect(slot?.style.left).toBe("35%");
			expect(cropSlot?.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
			expect(cropSlot?.style.getPropertyValue("--presentation-figure-bg-position")).toBe("0% 50%");
			expect(slot?.querySelector(".presentation-figure-scroll-viewport--axis-x")).not.toBeNull();
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
			expect(sheet.innerHTML).not.toContain("presentation-morph-target-fade-in");
		});

		it("preserves anisotropic scale() in the auto-animate sheet for intro flow morph", () => {
			const sheet = { innerHTML: "transform: scale(1, 2) !important;" };
			patchPresentationAutoAnimateStyleSheet(sheet, 0.8, { introFlowMorph: true });
			expect(sheet.innerHTML).toContain("scale(1, 2)");
			expect(sheet.innerHTML).not.toContain("scale(2)");
		});

		it("leaves intro flow auto-animate sheets identical to reveal.js output", () => {
			const fromSlide = document.createElement("section");
			fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const toSlide = document.createElement("section");
			toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const sheet = { innerHTML: "transform: translate(10px, 20px) scale(0.4, 1.2) !important;" };
			patchPresentationAutoAnimateRunStyleSheet(sheet, 0.8, fromSlide, toSlide);
			expect(sheet.innerHTML).toBe("transform: translate(10px, 20px) scale(0.4, 1.2) !important;");
		});

		it("uses pending slide pair when reveal autoanimate omits event slides", () => {
			const fromSlide = document.createElement("section");
			fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const toSlide = document.createElement("section");
			toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const resolved = resolvePresentationAutoAnimateRunSlides({}, { fromSlide, toSlide });
			const sheet = { innerHTML: "transform: translate(10px, 20px) scale(0.4, 1.2) !important;" };
			patchPresentationAutoAnimateRunStyleSheet(sheet, 0.8, resolved.fromSlide, resolved.toSlide);
			expect(resolved).toEqual({ fromSlide, toSlide });
			expect(sheet.innerHTML).toBe("transform: translate(10px, 20px) scale(0.4, 1.2) !important;");
		});

		it("preserves anisotropic scale() in the auto-animate sheet for many-to-one morph", () => {
			const sheet = { innerHTML: "transform: scale(0.4, 0.9) !important;" };
			patchPresentationAutoAnimateStyleSheet(sheet, 0.8, { manyToOneMorph: true });
			expect(sheet.innerHTML).toContain("scale(0.4, 0.9)");
			expect(sheet.innerHTML).not.toContain("scale(0.9)");
			expect(sheet.innerHTML).toContain("presentation-target-ghost-frame 0.8s ease forwards");
		});

		it("animates figure crop and target-ghost frames only during many-to-one auto-animate", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, resolve } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "globals.css"), "utf8");
			expect(css).toContain("presentation-figure-crop-morph-from-rest");
			expect(css).toContain("presentation-figure-crop-morph-grid-to-focus");
			expect(
				css.match(
					/presentation-morph-crop-from[\s\S]*?presentation-figure-crop-morph-from-rest/,
				),
			).toBeTruthy();
			expect(css).toContain("@keyframes presentation-target-ghost-frame");
			expect(css).toContain(
				"section.presentation-arrangement--many-to-one-morph[data-auto-animate=\"running\"]",
			);
			expect(css).toContain(".presentation-morph-crop-from");
			expect(css).toContain(".presentation-morph-crop-to");
		});

		it("rests morph-source ghosts with opacity only so reveal can measure FLIP targets", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, resolve } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const cssPath = resolve(dirname(fileURLToPath(import.meta.url)), "globals.css");
			const css = readFileSync(cssPath, "utf8");
			const restRule =
				css.match(
					/\.reveal \.presentation-target-ghost \{[\s\S]*?\}/,
				)?.[0] ?? "";
			expect(restRule).toContain("opacity: 0");
			expect(restRule).not.toContain("opacity: 0 !important");
			expect(restRule).not.toContain("visibility: hidden");
			expect(css).toMatch(
				/section\[data-auto-animate="running"\][\s\S]*?presentation-target-ghost-fade-out/,
			);
			expect(restRule).toContain("pointer-events: none !important");
			expect(restRule).toContain("z-index: 0");
			expect(css).toContain(
				"> .presentation-interactive-disposition:not(.presentation-target-ghost):not(.presentation-source-ghost)",
			);
			expect(css).toMatch(
				/section\[data-auto-animate="pending"\][\s\S]*?\.presentation-target-ghost/,
			);
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

		it("computes pdf cover scale from container and page viewport", () => {
			expect(pdfCoverScale(768, 280, 595, 842)).toBeCloseTo(768 / 595);
			expect(pdfCoverScale(200, 400, 595, 842)).toBeCloseTo(400 / 842);
			expect(pdfCoverScale(0, 400, 595, 842)).toBeNull();
			expect(pdfScrollCoverScale(768, 280, 595, 842)).toBeCloseTo(768 / 595);
			expect(pdfScrollCoverScale(200, 400, 595, 842)).toBeCloseTo(400 / 842);
		});

		it("figureCoverScrollElementStyle fits one axis for wide video frames", () => {
			const style = figureCoverScrollElementStyle(565, 350, 1200 / 1080);
			expect(style.width).toBe("100%");
			expect(style.height).toBe("auto");
			expect(style.aspectRatio).toBe(String(1200 / 1080));
		});

		it("navigates only within a declared pdf page subset", () => {
			const thesis: PdfEmbodiment = {
				kind: "pdf",
				id: "thesis--doc",
				src: "/thesis.pdf",
				page: 25,
				pages: [1, 12, 25, 35, 42, 43, 51],
			};
			expect(pdfEmbodimentInitialPage(thesis)).toBe(25);
			expect(pdfEmbodimentInitialPage({ ...thesis, page: 99 })).toBe(1);
			expect(pdfPageNavEnabled(thesis, null)).toBe(true);
			expect(pdfPageNavEnabled({ ...thesis, pages: [1] }, null)).toBe(false);
			expect(pdfCanGoToPreviousPage(25, thesis, null)).toBe(true);
			expect(pdfCanGoToNextPage(51, thesis, null)).toBe(false);
			expect(pdfAdjacentPage(25, "next", thesis, null)).toBe(35);
			expect(pdfAdjacentPage(51, "next", thesis, null)).toBe(51);
			expect(pdfAdjacentPage(51, "prev", thesis, null)).toBe(43);
			expect(pdfAdjacentPage(1, "prev", thesis, null)).toBe(1);
		});

		it("figureCoverOverflowAxis picks the overflowing axis from frame and source aspect", () => {
			expect(figureCoverOverflowAxis(200, 100, 1)).toBe("y");
			expect(figureCoverOverflowAxis(100, 200, 1)).toBe("x");
			expect(figureCoverOverflowAxis(100, 100, 1)).toBeNull();
		});

		it("figureScrollOffsetForBackgroundPosition maps origin percents to scroll offsets", () => {
			expect(figureScrollOffsetForBackgroundPosition("y", 50, 1000, 400)).toBe(300);
			expect(figureScrollOffsetForBackgroundPosition("y", 0, 1000, 400)).toBe(0);
			expect(figureScrollOffsetForBackgroundPosition("x", 0, 800, 200)).toBe(0);
		});

		it("figureBackgroundSizeScrollAxis maps crop background-size to one scroll axis", () => {
			expect(figureBackgroundSizeScrollAxis("400% auto")).toBe("x");
			expect(figureBackgroundSizeScrollAxis("auto 1600%")).toBe("y");
			expect(figureBackgroundSizeScrollAxis("cover")).toBeNull();
			expect(figureCropScrollBackgroundSize("cover", "x")).toBe("100% auto");
			expect(figureCropScrollBackgroundSize("cover", "y")).toBe("auto 100%");
		});

		it("figureWheelZoomStep clamps ctrl+wheel zoom between cover baseline and max", () => {
			expect(figureWheelZoomStep(-100, 1)).toBeCloseTo(1.1);
			expect(figureWheelZoomStep(100, 1)).toBe(1);
			expect(figureWheelZoomStep(-100, FIGURE_WHEEL_ZOOM_MAX)).toBe(FIGURE_WHEEL_ZOOM_MAX);
			expect(figureWheelZoomStep(100, 2)).toBeCloseTo(2 / 1.1);
		});

		it("figureBackgroundSizeZoomed scales crop background-size strings", () => {
			expect(figureBackgroundSizeZoomed("400% auto", 2)).toBe("800% auto");
			expect(figureBackgroundSizeZoomed("auto 1600%", 1.5)).toBe("auto 2400%");
			expect(figureBackgroundSizeZoomed("cover", 2)).toBe("cover");
		});

		it("figureCoverScrollElementStyle grows with ctrl+wheel zoom", () => {
			const base = figureCoverScrollElementStyle(565, 350, 1200 / 1080);
			const zoomed = figureCoverScrollElementStyle(565, 350, 1200 / 1080, 2);
			expect(base.width).toBe("100%");
			expect(zoomed.width).toBe("200%");
		});

		it("figureCoverScrollContentSize enables both axes when zoomed past cover", () => {
			const square = figureCoverScrollContentSize(100, 100, 1, 2);
			expect(square.axis).toBe("both");
			const tall = figureCoverScrollContentSize(200, 100, 1, 2);
			expect(tall.axis).toBe("both");
		});

		it("mediaTeaserActive is true only when teaser is set", () => {
			expect(mediaTeaserActive(undefined)).toBe(false);
			expect(mediaTeaserActive({})).toBe(true);
			expect(mediaTeaserActive({ label: "Preview" })).toBe(true);
		});

		it("figureEmbodimentScrollEnabled defaults on and respects mosaic and scroll:false", () => {
			expect(figureEmbodimentScrollEnabled({ kind: "figure", id: "a", src: "/a.png" })).toBe(true);
			expect(
				figureEmbodimentScrollEnabled({
					kind: "figure",
					id: "a",
					src: "/a.png",
					scroll: false,
				}),
			).toBe(false);
			expect(
				figureEmbodimentScrollEnabled({
					kind: "figure",
					id: "a",
					src: "/a.png",
					mosaic: { rows: 2, columns: 2 },
				}),
			).toBe(false);
		});

		it("renders clipped crop figures when scroll is disabled", () => {
			const container = document.createElement("div");
			const deck: Presentation = {
				id: "scroll-off",
				chapters: [
					{
						id: "c",
						sequences: [
							{
								id: "s",
								thoughts: [
									{
										id: "t",
										participants: [{ id: "figure" }],
										embodiments: [
											{
												kind: "figure",
												id: "figure--img",
												src: "/catalogue.png",
												crop: { x: 0, y: 0, width: 0.5, height: 1 },
												scroll: false,
											},
										],
										slides: [
											{
												arrangement: {
													id: "a",
													dispositions: [
														{
															participantId: "figure",
															embodimentId: "figure--img",
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
				'[data-id="figure"].presentation-morph-slot--figure',
			) as HTMLElement | null;
			expect(slot).not.toBeNull();
			expect(container.querySelector(".presentation-figure-scroll-viewport")).toBeNull();
		});

		it("figureScrollOverlayThumbMetrics sizes overlay thumbs from scroll metrics", () => {
			expect(figureScrollOverlayThumbMetrics(280, 1086, 0).visible).toBe(true);
			expect(figureScrollOverlayThumbMetrics(280, 1086, 0).thumbSize).toBeGreaterThan(0);
			expect(figureScrollOverlayThumbMetrics(280, 280, 0).visible).toBe(false);
		});

		it("styles figure scroll viewports for one-axis overflow", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, resolve } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "globals.css"), "utf8");
			expect(css).toContain(".presentation-figure-scroll-viewport--axis-x");
			expect(css).toContain(".presentation-figure-scroll-viewport--axis-y");
			expect(css).toContain(".presentation-figure-scroll-scroller--axis-x");
			expect(css).toContain(".presentation-figure-scroll-scroller--axis-y");
			expect(css).toContain(".presentation-figure-scroll-scroller--axis-both");
			expect(css).toContain(".presentation-figure-scroll-viewport--overlay");
			expect(css).toContain(".presentation-figure-scroll-bar-thumb");
			expect(css).toContain(".presentation-figure-scroll-media");
			expect(css).toContain(".presentation-figure-scroll-content");
			expect(css).toContain("--scrollbar-thumb-active");
			expect(css).toContain(".presentation-figure-scroll-bar:hover .presentation-figure-scroll-bar-thumb");
			expect(css).toContain(".presentation-figure-scroll-bar--dragging .presentation-figure-scroll-bar-thumb");
			expect(css).toContain(".presentation-figure-scroll-bar-thumb:hover");
			expect(css).toContain(".presentation-figure-scroll-bar-thumb:active");
			expect(css).not.toContain("scrollbar-gutter: stable");
			expect(css).toMatch(
				/\.presentation-morph-slot--figure:not\(\.presentation-figure-scroll-content\)[\s\S]*width:\s*100%\s*!important/,
			);
		});

		it("uses cover for full image and uniform crop zoom for partial crops", () => {
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
			expect(square["--presentation-figure-bg-size" as keyof typeof square]).toBe("400% auto");
			expect(wide["--presentation-figure-bg-size" as keyof typeof wide]).toBe("1600% auto");
			expect(wide["--presentation-figure-bg-position" as keyof typeof wide]).toBe("0% 50%");
			expect(square["--presentation-figure-bg-position" as keyof typeof square]).toBe("0% 50%");
		});

		it("uses CSS cover for full catalogue in the catalogue frame", () => {
			const sourceAspect = 1222 / 896;
			const crop = { x: 0, y: 0, width: 1, height: 1 };
			const frame = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
			const vars = figureCropBackgroundVars(
				{ kind: "figure", src: "/bauteilbörse.png", crop, sourceAspect },
				crop,
				frame,
			);
			expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe("cover");
			expect(vars["--presentation-figure-bg-position" as keyof typeof vars]).toBe("50% 50%");
		});

		it("uses windowed mosaic for rest and grid and centered crop for morph on catalogue-to-focus", () => {
			const sourceAspect = 1222 / 896;
			const catalogueFrame = { x: 0.127, y: 0.2, width: 0.746, height: 0.75 };
			const mosaic = { rows: 3, columns: 5, frame: catalogueFrame };
			const crop = {
				x: catalogueFrame.x + catalogueFrame.width / 5,
				y: catalogueFrame.y + catalogueFrame.height / 3,
				width: catalogueFrame.width / 5,
				height: catalogueFrame.height / 3,
			};
			const gridFrame = { x: 0.127, y: 0.2, width: 0.1492, height: 0.1823 };
			const focusFrame = { x: 0.1, y: 0.2, width: 0.15, height: 0.18 };
			const labelFrame = { x: 0.6, y: 0.44, width: 0.24, height: 0.12 };
			const windowed = mosaicWindowedCoverVars(
				{ column: 1, row: 1 },
				mosaic,
				catalogueFrame,
				sourceAspect,
			);
			const morphPos = figureCropBackgroundPosition(crop);
			const morphSize = figureCropBackgroundSize(crop, labelFrame, sourceAspect);
			const vars = figureCropBackgroundVars(
				{
					kind: "figure",
					id: "tile-figure",
					src: "/bauteilbörse.png",
					crop,
					sourceAspect,
					mosaic,
				},
				crop,
				focusFrame,
				labelFrame,
				gridFrame,
			);
			expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe(windowed.size);
			expect(vars["--presentation-figure-bg-position" as keyof typeof vars]).toBe(
				`${windowed.posX}% ${windowed.posY}%`,
			);
			expect(vars["--presentation-figure-bg-grid-size" as keyof typeof vars]).toBe(windowed.size);
			expect(vars["--presentation-figure-bg-grid-position" as keyof typeof vars]).toBe(
				`${windowed.posX}% ${windowed.posY}%`,
			);
			expect(vars["--presentation-figure-bg-size-morph" as keyof typeof vars]).toBe(morphSize);
			expect(vars["--presentation-figure-bg-position-morph" as keyof typeof vars]).toBe(
				`${morphPos.posX}% ${morphPos.posY}%`,
			);
		});

		it("mosaic windowed cover uses columns×100% width for catalogue 3×5 on a wide slide", () => {
			const sourceAspect = 1222 / 896;
			const frame = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
			const slideAspect = 960 / 700;
			const vars = mosaicWindowedCoverVars(
				{ column: 0, row: 1 },
				{ rows: 3, columns: 5 },
				frame,
				sourceAspect,
				slideAspect,
			);
			expect(vars.size).toBe("500% auto");
			expect(vars.posX).toBe(0);
		});

		it("sets revealMorphFromMorphToFrame from the previous slide morphTo slots", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "tile" }],
					embodiments: [
						{
							kind: "figure",
							id: "tile-figure",
							src: "/a.png",
							crop: { x: 0, y: 0, width: 0.5, height: 1 },
						},
					],
				},
			]);
			const gridFrame = { x: 0.1, y: 0.1, width: 0.2, height: 0.3 };
			const focusFrame = { x: 0.4, y: 0.2, width: 0.15, height: 0.18 };
			const resolved = resolveRevealArrangement(
				scope,
				{
					id: "focus",
					dispositions: [
						{
							participantId: "tile",
							embodimentId: "tile-figure",
							emphasis: "active",
							position: focusFrame,
						},
					],
				},
				{
					previousSlide: {
						arrangement: {
							id: "catalogue",
							dispositions: [
								{
									participantId: "catalogue",
									embodimentId: "catalogue--figure",
									morphTo: [{ participantId: "tile", embodimentId: "tile-figure", position: gridFrame }],
								},
							],
						},
					},
				},
			);
			expect(resolved.find((entry) => entry.participant.id === "tile")?.revealMorphFromMorphToFrame).toEqual(
				gridFrame,
			);
		});

		it("assigns windowed mosaic cover per split tile", () => {
			const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
			const mosaic = { rows: 2, columns: 2, frame };
			const tiles = splitFigureGrid({ rows: 2, columns: 2, frame });
			const embodiment = {
				kind: "figure" as const,
				src: "/catalogue.png",
				mosaic,
			};
			const expected = tiles.map((tile, index) => {
				const column = index % 2;
				const row = Math.floor(index / 2);
				return mosaicWindowedCoverVars({ column, row }, mosaic, frame, 1);
			});
			const varsList = tiles.map((tile) =>
				figureCropBackgroundVars(embodiment, tile.crop, tile.position),
			);
			expect(varsList.map((vars) => vars["--presentation-figure-bg-size" as keyof typeof vars])).toEqual(
				expected.map((entry) => entry.size),
			);
			expect(varsList.map((vars) => vars["--presentation-figure-bg-position" as keyof typeof vars])).toEqual(
				expected.map((entry) => `${entry.posX}% ${entry.posY}%`),
			);
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
			slide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
			const morphSource = document.createElement("div");
			morphSource.className = "presentation-target-ghost";
			morphSource.dataset.autoAnimateTarget = "0";
			slide.appendChild(morphSource);
			deckEl.appendChild(slide);
			document.body.appendChild(deckEl);
			finalizeRevealAutoAnimateRestState(deckEl);
			expect(slide.getAttribute("data-auto-animate")).toBe("");
			expect(slide.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
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

		it("detects reveal slide auto-animate pending or running on section or deck", () => {
			const section = document.createElement("section");
			const deck = document.createElement("div");
			deck.className = "reveal";
			deck.appendChild(section);
			expect(isRevealSlideAutoAnimating(section)).toBe(false);
			expect(isSectionAutoAnimating(section)).toBe(false);
			section.setAttribute("data-auto-animate", "pending");
			expect(isSectionAutoAnimating(section)).toBe(true);
			expect(isRevealSlideAutoAnimating(section)).toBe(true);
			section.setAttribute("data-auto-animate", "");
			const other = document.createElement("section");
			other.setAttribute("data-auto-animate", "running");
			deck.appendChild(other);
			expect(isSectionAutoAnimating(section)).toBe(false);
			expect(isRevealSlideAutoAnimating(section)).toBe(true);
		});

		it("detects consecutive slides in the same auto-animate run", () => {
			const focus = document.createElement("section");
			focus.setAttribute("data-auto-animate-id", "medien--m0");
			const labels = document.createElement("section");
			labels.setAttribute("data-auto-animate-id", "medien--m0");
			const overview = document.createElement("section");
			overview.setAttribute("data-auto-animate-id", "medien--m1");
			expect(slidesShareAutoAnimateId(focus, labels)).toBe(true);
			expect(slidesShareAutoAnimateId(focus, overview)).toBe(false);
		});

		it("clears settled state when arriving on a slide and prepares it before morph to listed targets", () => {
			const deckEl = document.createElement("div");
			deckEl.className = "reveal";
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
			deckEl.appendChild(labels);
			prepareArrangementBeforeAutoAnimate(focus, labels);
			expect(focus.classList.contains("presentation-arrangement--settled")).toBe(true);
			expect(focus.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(true);
			expect(labels.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(true);
		});

		it("syncManyToOneGhostMorphFramesFromDom uses live source tile geometry on target ghosts", () => {
			const deckEl = document.createElement("div");
			deckEl.className = "reveal";
			const focus = document.createElement("section");
			focus.setAttribute("title", "catalogue-focus");
			focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
			focus.style.cssText = "position:relative;width:var(--layout-deck-width);height:33.75rem;";
			const labels = document.createElement("section");
			labels.setAttribute("title", "catalogue-labels");
			labels.style.cssText = "position:relative;width:var(--layout-deck-width);height:33.75rem;";
			const source = document.createElement("div");
			source.className =
				"presentation-interactive-disposition presentation-interactive-disposition--canvas-framed";
			source.setAttribute("data-id", "Stütze");
			Object.assign(source.style, {
				position: "absolute",
				left: "20%",
				top: "10%",
				width: "15%",
				height: "70%",
			});
			const ghost = document.createElement("div");
			ghost.className =
				"presentation-interactive-disposition presentation-target-ghost presentation-interactive-disposition--canvas-framed";
			ghost.setAttribute("data-id", "Stütze");
			Object.assign(ghost.style, {
				position: "absolute",
				left: "60%",
				top: "44%",
				width: "24%",
				height: "12%",
			});
			const slot = document.createElement("div");
			slot.className = "presentation-morph-slot presentation-morph-slot--figure";
			slot.dataset.presentationMorphCrop = JSON.stringify({ x: 0, y: 0, width: 0.5, height: 1 });
			ghost.append(slot);
			focus.append(source);
			labels.append(ghost);
			deckEl.append(focus, labels);
			document.body.append(deckEl);
			syncManyToOneGhostMorphFramesFromDom(focus, labels);
			expect(ghost.style.getPropertyValue("--presentation-morph-frame-left")).toBe("20%");
			expect(ghost.style.getPropertyValue("--presentation-morph-frame-top")).toBe("10%");
			expect(ghost.style.getPropertyValue("--presentation-morph-frame-width")).toBe("15%");
			expect(ghost.style.getPropertyValue("--presentation-morph-frame-height")).toBe("70%");
			expect(ghost.style.getPropertyValue("--presentation-frame-left")).toBe("60%");
			deckEl.remove();
		});

		it("does not mark many-to-one morph for catalogue to focus auto-animate", () => {
			const deckEl = document.createElement("div");
			deckEl.className = "reveal";
			const catalogue = document.createElement("section");
			catalogue.setAttribute("title", "catalogue");
			const focus = document.createElement("section");
			focus.setAttribute("title", "catalogue-focus");
			focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
			deckEl.append(catalogue, focus);
			expect(isManyToOneMorphTransition(catalogue, focus)).toBe(false);
			prepareArrangementBeforeAutoAnimate(catalogue, focus);
			expect(focus.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
			expect(catalogue.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
		});

		it("renders expanded source ghosts for one-to-many morphTo", () => {
			const deck: Presentation = {
				id: "source-ghost",
				name: "Source ghost",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "split",
										participants: [{ id: "whole" }, { id: "tile-a" }],
										embodiments: [
											{ kind: "figure", id: "whole--figure", src: "/catalogue.png" },
											{
												kind: "figure",
												id: "tile-a--figure",
												src: "/catalogue.png",
												crop: { x: 0, y: 0, width: 0.5, height: 1 },
											},
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
															position: { x: 0.1, y: 0.1, width: 0.8, height: 0.8 },
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
															position: { x: 0.05, y: 0.2, width: 0.4, height: 0.6 },
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
			const wholeSlide = container.querySelector('section[title="whole"]') as HTMLElement;
			const sourceGhost = wholeSlide.querySelector(
				'.presentation-interactive-disposition.presentation-source-ghost[data-id="tile-a"]',
			);
			expect(sourceGhost).toBeTruthy();
			expect(wholeSlide.querySelectorAll(".presentation-morph-one").length).toBe(1);
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

	describe("reveal text morph", () => {
		it("pairs intro leaf text nodes, not disposition wrappers", () => {
			const fromSlide = document.createElement("section");
			fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const toSlide = document.createElement("section");
			toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const fromWrap = document.createElement("div");
			fromWrap.className = "presentation-interactive-disposition";
			const fromHeading = document.createElement("h2");
			fromHeading.setAttribute("data-id", "description");
			fromHeading.textContent = "Long description line";
			fromWrap.append(fromHeading);
			fromSlide.append(fromWrap);
			const toWrap = document.createElement("div");
			toWrap.className = "presentation-interactive-disposition";
			const toHeading = document.createElement("h2");
			toHeading.setAttribute("data-id", "description");
			toHeading.textContent = "Short";
			toWrap.append(toHeading);
			toSlide.append(toWrap);
			expect(isRevealAutoAnimatePairSource(fromWrap)).toBe(false);
			expect(isRevealAutoAnimatePairSource(fromHeading)).toBe(true);
			const pairs = presentationAutoAnimateMatcher.call(
				stockRevealAutoAnimateMatcherHost(),
				fromSlide,
				toSlide,
			);
			expect(pairs).toEqual([
				{
					from: fromHeading,
					to: toHeading,
				},
			]);
		});

		it("pairs short and full heading-block lines via a shared --0 suffix", () => {
			const fromSlide = document.createElement("section");
			fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const toSlide = document.createElement("section");
			toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const fromHeading = document.createElement("h2");
			fromHeading.setAttribute("data-id", "description--0");
			fromHeading.textContent = "Short";
			fromSlide.append(fromHeading);
			const toLine0 = document.createElement("h2");
			toLine0.setAttribute("data-id", "description--0");
			toLine0.textContent = "Long line one";
			const toLine1 = document.createElement("h2");
			toLine1.setAttribute("data-id", "description--1");
			toLine1.textContent = "Long line two";
			toSlide.append(toLine0, toLine1);
			const pairs = presentationAutoAnimateMatcher.call(
				stockRevealAutoAnimateMatcherHost(),
				fromSlide,
				toSlide,
			);
			expect(pairs).toEqual([
				{
					from: fromHeading,
					to: toLine0,
				},
			]);
		});

		it("uses stock reveal auto-animate options on intro text pairs", () => {
			const fromSlide = document.createElement("section");
			fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const toSlide = document.createElement("section");
			toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
			const fromHeading = document.createElement("h2");
			fromHeading.setAttribute("data-id", "description--0");
			fromSlide.append(fromHeading);
			const toHeading = document.createElement("h2");
			toHeading.setAttribute("data-id", "description--0");
			toSlide.append(toHeading);
			const pairs = presentationAutoAnimateMatcher.call(
				stockRevealAutoAnimateMatcherHost(),
				fromSlide,
				toSlide,
			);
			expect(pairs[0]?.options).toBeUndefined();
		});

		it("uses slide-local measure options on non-intro text pairs", () => {
			const fromSlide = document.createElement("section");
			fromSlide.className = "presentation-arrangement--interactive";
			const toSlide = document.createElement("section");
			toSlide.className = "presentation-arrangement--interactive";
			const heading = document.createElement("h2");
			heading.setAttribute("data-id", "label");
			expect(revealTextAutoAnimatePairOptions(heading, fromSlide, toSlide)).toEqual({
				scale: false,
				measure: revealInkMeasureForAutoAnimate,
			});
		});

		it("measures morph text relative to the slide box, not the viewport", () => {
			const section = document.createElement("section");
			section.className = "presentation-arrangement--interactive";
			const heading = document.createElement("h2");
			heading.textContent = "Plattform";
			section.append(heading);
			document.body.append(section);
			section.getBoundingClientRect = () => new DOMRect(500, 0, 960, 700);
			heading.getBoundingClientRect = () => new DOMRect(780, 300, 400, 48);
			const viewport = heading.getBoundingClientRect();
			const measured = revealInkMeasureForAutoAnimate(heading);
			document.body.removeChild(section);
			expect(measured.x).not.toBeCloseTo(viewport.left);
			expect(measured.x).toBeGreaterThanOrEqual(0);
			expect(measured.x + measured.width).toBeLessThanOrEqual(960);
		});
	});

	describe("presentation interaction geometry", () => {
		it("detects disposition ephemeral layout from anchor and transform", () => {
			const declared = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
			expect(dispositionHasEphemeralLayout(undefined, declared, undefined, false)).toBe(false);
			expect(dispositionHasEphemeralLayout(declared, declared, undefined, false)).toBe(false);
			expect(
				dispositionHasEphemeralLayout({ x: 0.3, y: 0.3, width: 0.4, height: 0.2 }, declared, undefined, false),
			).toBe(true);
		});

		it("detects slide ephemeral layout and reset hotspot proximity", () => {
			const declared = new Map<string, DispositionPosition | undefined>([
				["a", { x: 0.2, y: 0.3, width: 0.4, height: 0.2 }],
			]);
			expect(slideHasEphemeralLayout(new Map(), new Set(), declared)).toBe(false);
			expect(
				slideHasEphemeralLayout(
					new Map([["a", { x: 0.2, y: 0.3, width: 0.4, height: 0.2 }]]),
					new Set(),
					declared,
				),
			).toBe(false);
			expect(
				slideHasEphemeralLayout(
					new Map([["a", { x: 0.3, y: 0.3, width: 0.4, height: 0.2 }]]),
					new Set(),
					declared,
				),
			).toBe(true);
			expect(slideHasEphemeralLayout(new Map(), new Set(["a"]), declared)).toBe(true);
			const section = document.createElement("section");
			document.body.appendChild(section);
			section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
			expect(pointerNearSlideResetHotspot(section, 920, 20)).toBe(true);
			expect(pointerNearSlideResetHotspot(section, 100, 20)).toBe(false);
			document.body.removeChild(section);
		});

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
			const crossingMarquee = normalizeMarquee({ x: 0.7, y: 0.7 }, { x: 0.1, y: 0.1 });
			const windowMarquee = normalizeMarquee({ x: 0.1, y: 0.1 }, { x: 0.5, y: 0.5 });
			expect(marqueeSelectionRule({ x: 0.7, y: 0.7 }, { x: 0.1, y: 0.1 })).toBe("crossing");
			expect(marqueeSelectionRule({ x: 0.1, y: 0.1 }, { x: 0.5, y: 0.5 })).toBe("window");
			expect(marqueeSelects(crossingMarquee, inside, "crossing")).toBe(true);
			expect(marqueeSelects(windowMarquee, inside, "window")).toBe(true);
			expect(marqueeSelects(windowMarquee, partial, "window")).toBe(false);
			expect(marqueeSelects(crossingMarquee, partial, "crossing")).toBe(true);
		});

		it("detects reveal overview from an element inside the deck", () => {
			const reveal = document.createElement("div");
			reveal.className = "reveal";
			const section = document.createElement("section");
			reveal.appendChild(section);
			expect(revealDeckInOverview(section)).toBe(false);
			reveal.classList.add("overview");
			expect(revealDeckInOverview(section)).toBe(true);
		});

		it("suppresses the reveal overview slide click after pointer gestures", () => {
			const reveal = document.createElement("div");
			reveal.className = "reveal";
			const slide = document.createElement("section");
			const inner = document.createElement("div");
			reveal.appendChild(slide);
			slide.appendChild(inner);
			let overviewClickCalls = 0;
			slide.addEventListener(
				"click",
				() => {
					overviewClickCalls += 1;
				},
				true,
			);
			suppressRevealOverviewSlideNavigation({ target: inner } as Event);
			const click = new MouseEvent("click", { bubbles: true, cancelable: true });
			inner.dispatchEvent(click);
			expect(overviewClickCalls).toBe(0);
			expect(click.defaultPrevented).toBe(true);
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
			expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.25, height: 0.1 }, measured)).toBe(
				false,
			);
			expect(isFlowPixelOffsetTransform({ x: 12, y: -4, width: 0.25, height: 0.1 }, measured)).toBe(true);
			expect(isFlowPixelOffsetTransform(offset, undefined)).toBe(true);
			expect(isFlowPixelOffsetTransform({ ...offset, width: 0.3 }, measured)).toBe(false);
			const section = document.createElement("section");
			section.style.width = "var(--layout-deck-width)";
			section.style.height = "var(--layout-deck-height)";
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
					enlarged: false,
				}),
			).toEqual(transformFrameStyle(rect));
			expect(
				interactiveDispositionChromeStyle({
					selected: true,
					effectiveRect: rect,
					canvasFramed: true,
					enlarged: false,
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

		it("maps flow pointer delta through drag target visual scale", () => {
			const section = document.createElement("section");
			section.className = "presentation-arrangement--interactive";
			document.body.appendChild(section);
			section.getBoundingClientRect = () => new DOMRect(0, 0, 480, 350);
			Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
			const delta = flowPointerDeltaToLocal(section, 100, 200, 150, 260);
			document.body.removeChild(section);
			expect(delta.dx).toBeCloseTo(100);
			expect(delta.dy).toBeCloseTo(120);
		});

		it("intro morph title placements have no declared slide frame", () => {
			const deck = intro({
				language: "de",
				title: { full: ["Title"], short: "T" },
				description: { full: ["D"], short: "d" },
				goal: ["G"],
				authors: { lines: [[{ name: "A" }]] },
				affiliations: {
					steps: [
						[{ mark: "a", name: "Faculty" }],
						[{ mark: "a", name: "Faculty" }, { mark: "1", name: "Uni" }],
						[
							{ mark: "a", name: "Faculty" },
							{ mark: "1", name: "Uni", shortName: "U", suffix: { mark: "x", name: "Chair" } },
						],
					],
				},
			});
			const thought = deck.chapters[0]!.sequences[0]!.thoughts[0]!;
			const renderSlide = expandThoughtSlides(thought)[0]!;
			const scope = buildResolutionScope([thought]);
			const resolved = resolveRevealArrangement(scope, renderSlide.arrangement, {});
			const layout = buildInteractiveSlideLayout(renderSlide.id, resolved, true);
			expect(layout.placements.every((entry) => entry.sectionRect === undefined)).toBe(true);
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
			expect(delta.dx).toBeCloseTo(40);
			expect(delta.dy).toBeCloseTo(40);
		});

		it("does not treat sub-unit flow drag offsets as normalized slide frames", () => {
			const measured = { x: 0.35, y: 0.4, width: 0.3, height: 0.08 };
			expect(isFlowPixelOffsetTransform({ x: 12, y: 4, width: 0.3, height: 0.08 }, measured)).toBe(true);
			expect(isNormalizedSlideFrame({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 })).toBe(true);
			expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 }, measured)).toBe(
				false,
			);
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

		it("scales group members and toggles enlarge", () => {
			const a = { x: 0.1, y: 0.2, width: 0.2, height: 0.2 };
			const b = { x: 0.5, y: 0.2, width: 0.2, height: 0.2 };
			const group = groupBoundingRect([a, b]);
			expect(group?.width).toBeCloseTo(0.6);
			const grown = { x: 0, y: 0.1, width: 0.8, height: 0.3 };
			const scaledA = scaleRectWithinGroup(a, group!, grown);
			expect(scaledA.x).toBeCloseTo(0);
			const full = toggleEnlargeRect(a, undefined);
			expect(full.rect).toEqual(SLIDE_INTERACTIVE_ENLARGE_FRAME);
			expect(full.stash).toEqual(a);
			const restored = toggleEnlargeRect(full.rect, full.stash);
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
			} = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
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
			const pairs = presentationAutoAnimateMatcher.call(
				{ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost,
				focusSlide,
				labelSlide,
			);
			expect(pairs.every((pair) => !elementIsSourceGhostAnchor(pair.from))).toBe(true);
			expect(pairs.every((pair) => !elementIsSourceGhostAnchor(pair.to))).toBe(true);
			expect(pairs.every((pair) => elementIsTargetGhostAnchor(pair.to))).toBe(true);
			expect(pairs.every((pair) => elementIsInteractiveFigureDisposition(pair.from))).toBe(true);
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
				await import("@semio-tech/framework-presentation-core");
			type SlideFile = import("@semio-tech/framework-presentation-core").SlideFile;
			const { presentationMeta } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
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
				'.presentation-morph-one .presentation-morph-slot--figure[role="img"]',
			) as HTMLElement | null;
			expect(catalogueFigure?.style.backgroundImage).toContain("bauteilb");
			expect(catalogueSlide.querySelectorAll(".presentation-morph-one").length).toBe(1);
			const sourceGhosts = catalogueSlide.querySelectorAll(
				".presentation-interactive-disposition.presentation-source-ghost",
			);
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

		it("renders target ghosts but no source ghosts on focus and labels slides", async () => {
			const { loadPresentationFromSlideGlob } = await import("@semio-tech/framework-presentation-core");
			type SlideFile = import("@semio-tech/framework-presentation-core").SlideFile;
			const { presentationMeta } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
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
			const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
			const labelSlide = mountRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
			expect(focusSlide.querySelectorAll(".presentation-source-ghost").length).toBe(0);
			expect(labelSlide.querySelectorAll(".presentation-source-ghost").length).toBe(0);
			expect(labelSlide.querySelectorAll(".presentation-target-ghost").length).toBeGreaterThanOrEqual(8);
			for (const ghost of labelSlide.querySelectorAll<HTMLElement>(
				".presentation-interactive-disposition.presentation-target-ghost",
			)) {
				expect(ghost.style.pointerEvents).toBe("none");
				expect(ghost.getAttribute("aria-hidden")).toBe("true");
			}
			mountRoot.remove();
		});

		it("puts reveal data-id on catalogue tile wrappers for catalogue-to-focus morph", async () => {
			const { collectPresentationSlides, loadPresentationFromSlideGlob } =
				await import("@semio-tech/framework-presentation-core");
			type SlideFile = import("@semio-tech/framework-presentation-core").SlideFile;
			const { presentationMeta, CATALOGUE_FOCUS_TILES } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
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
				await import("@semio-tech/framework-presentation-core");
			type SlideFile = import("@semio-tech/framework-presentation-core").SlideFile;
			const { presentationMeta, CATALOGUE_FOCUS_TILES } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
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
			expect(
				catalogueSlide.querySelectorAll(".presentation-interactive-disposition.presentation-source-ghost")
					.length,
			).toBe(10);
			catalogueSlide.setAttribute("data-auto-animate", "pending");
			const pairs = presentationAutoAnimateMatcher.call(
				{ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost,
				catalogueSlide,
				focusSlide,
			);
			const componentTileIds = CATALOGUE_FOCUS_TILES.map((tile) => tile.participantId);
			expect(componentTileIds.every((id) => pairs.some((pair) => pair.from.getAttribute("data-id") === id))).toBe(
				true,
			);
			mountRoot.remove();
		});

		it("places catalogue-labels target ghosts at inline label frames", async () => {
			const { collectPresentationSlides, loadPresentationFromSlideGlob } =
				await import("@semio-tech/framework-presentation-core");
			type SlideFile = import("@semio-tech/framework-presentation-core").SlideFile;
			const { presentationMeta, inlineColumnLabelPosition } =
				await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
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
				await import("@semio-tech/framework-presentation-core");
			type SlideFile = import("@semio-tech/framework-presentation-core").SlideFile;
			const { presentationMeta } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
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

		const twoSlideDeck: Presentation = {
			id: "interactive-two-slide",
			name: "Interactive Two Slide",
			chapters: [
				{
					id: "main",
					sequences: [
						{
							id: "main",
							thoughts: [
								{
									id: "pair",
									participants: [{ id: "box" }],
									embodiments: [{ kind: "text", id: "box--main", lines: ["Hello"], level: "body" }],
									slides: [
										{
											arrangement: {
												id: "alpha",
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
										{
											arrangement: {
												id: "beta",
												dispositions: [
													{
														participantId: "box",
														embodimentId: "box--main",
														emphasis: "active",
														position: { x: 0.55, y: 0.55, width: 0.3, height: 0.15 },
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
			expect(disposition.querySelector(".presentation-interaction-enlarge")).toBeTruthy();
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
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			const reveal = container.querySelector(".reveal") as HTMLElement;
			const background = document.createElement("div");
			background.className = "slide-background present";
			reveal.appendChild(background);
			act(() => {
				pointerClick(background, 8, 8);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
		});

		it("keeps selection when empty slide click started in reveal overview", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const reveal = container.querySelector(".reveal") as HTMLElement;
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			reveal.classList.add("overview");
			act(() => {
				pointerClick(layer);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			reveal.classList.remove("overview");
			act(() => {
				pointerClick(layer);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
		});

		it("allows intro flow disposition drag with pixel offsets", () => {
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
			const titleLine = section.querySelector('h2[data-id^="title"]') as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(disposition, 330, 120, 300, 80);
			mockClientRect(titleLine, 330, 120, 300, 80);
			act(() => {
				pointerClick(disposition);
				pointerDrag(disposition, 480, 320, 560, 360);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--offset")).toBe(true);
			expect(disposition.style.transform).toContain("translate3d(");
		});

		it("pairs intro description morph on leaf text for reveal auto-animate", () => {
			unmountPresentation();
			const deck = intro({
				language: "de",
				title: { full: ["Entwerfen mit Bestand"], short: "Entwerfen mit Bestand" },
				description: {
					full: [
						"Eine offene Plattform für einen KI-unterstützten, performance-optimierten und integrativen Entwurfsprozess mit wiederverwendeten Baukomponenten",
					],
					short: "Plattform zum Entwerfen mit wiederverwendete Bauteilen",
				},
				goal: ["Mehr Zeit zum manuellen Entwerfen", "dank Automatisierung!"],
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
			const descriptionSlide = container.querySelector(
				'section[title="description"]',
			) as HTMLElement;
			const goalSlide = container.querySelector('section[title="goal"]') as HTMLElement;
			expect(descriptionSlide.hasAttribute("data-auto-animate")).toBe(true);
			expect(descriptionSlide.getAttribute("data-auto-animate-id")).toBe(
				goalSlide.getAttribute("data-auto-animate-id"),
			);
			const pairs = presentationAutoAnimateMatcher.call(
				stockRevealAutoAnimateMatcherHost(),
				descriptionSlide,
				goalSlide,
			);
			expect(pairs.some((pair) => pair.from.getAttribute("data-id") === "description")).toBe(true);
			expect(
				pairs.every((pair) => pair.from.matches("h1, h2, h3, h4, h5, h6, p")),
			).toBe(true);
			const descriptionPair = pairs.find((pair) => pair.from.getAttribute("data-id") === "description");
			expect(descriptionPair?.to.getAttribute("data-id")).toBe("description");
			expect(descriptionPair?.to.textContent).toContain("Plattform zum Entwerfen");
			expect(descriptionPair?.options).toBeUndefined();
			expect(goalSlide.querySelector('h2[data-id="description"]')).toBeTruthy();
			const authorsSlide = container.querySelector('section[title="authors"]') as HTMLElement;
			const goalToAuthors = presentationAutoAnimateMatcher.call(
				stockRevealAutoAnimateMatcherHost(),
				goalSlide,
				authorsSlide,
			);
			expect(
				goalToAuthors.some(
					(pair) =>
						pair.from.getAttribute("data-id") === "description" &&
						pair.to.getAttribute("data-id") === "description",
				),
			).toBe(true);
			const authorsPair = goalToAuthors.find((pair) => pair.from.getAttribute("data-id")?.startsWith("authors--"));
			expect(authorsPair).toBeTruthy();
			expect(authorsPair?.from.matches("h4")).toBe(true);
			expect(authorsPair?.options).toBeUndefined();
			expect(
				authorsSlide.querySelector('h2[data-id="description"].opacity-20'),
			).toBeTruthy();
		});

		it("pairs authors across affiliation steps with non-zero morph measure delta", () => {
			unmountPresentation();
			const deck = intro({
				language: "de",
				title: { full: ["A"], short: "A" },
				description: { full: ["D"], short: "D" },
				goal: ["G"],
				authors: {
					lines: [
						[{ name: "Alice Example" }, { name: "Bob Beta" }],
						[{ name: "Carol Creator" }],
					],
				},
				affiliations: {
					steps: [
						[{ mark: "a", name: "Faculty" }],
						[
							{ mark: "a", name: "Faculty" },
							{ mark: "1", name: "Uni" },
						],
						[
							{ mark: "a", name: "Faculty" },
							{ mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair" } },
						],
					],
				},
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const authorsSlide = container.querySelector('section[title="authors"]') as HTMLElement;
			const aff1Slide = container.querySelector('section[title="affiliations-1"]') as HTMLElement;
			const pairs = presentationAutoAnimateMatcher.call(
				stockRevealAutoAnimateMatcherHost(),
				authorsSlide,
				aff1Slide,
			);
			const authorsPair = pairs.find((pair) => pair.from.getAttribute("data-id")?.startsWith("authors--"));
			expect(authorsPair).toBeTruthy();
			expect(authorsPair?.options).toBeUndefined();
			expect(authorsPair?.from.textContent).toContain("Alice Example");
			expect(authorsPair?.to.textContent).toContain("A.");
		});

		it("resizes flow disposition from se handle when nested reveal section has zero height", () => {
			const deck: Presentation = {
				id: "flow-resize",
				name: "Flow Resize",
				chapters: [
					{
						id: "main",
						sequences: [
							{
								id: "main",
								thoughts: [
									{
										id: "flow",
										participants: [{ id: "label" }],
										embodiments: [
											{ kind: "text", id: "label--body", lines: ["Flow label"], level: "body" },
										],
										slides: [
											{
												arrangement: {
													id: "flow",
													dispositions: [
														{
															participantId: "label",
															embodimentId: "label--body",
															emphasis: "active",
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
			const section = container.querySelector(
				".slides > section > section.presentation-arrangement--interactive:not(.presentation-arrangement--intro)",
			) as HTMLElement;
			const stack = section.parentElement as HTMLElement;
			section.classList.add("present");
			stack.classList.add("present");
			Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(stack, "offsetHeight", { value: 700, configurable: true });
			Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
			Object.defineProperty(section, "offsetHeight", { value: 0, configurable: true });
			const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
			const label = disposition.querySelector("p") as HTMLElement;
			mockClientRect(stack, 0, 0, 960, 700);
			mockClientRect(section, 0, 0, 960, 0);
			mockClientRect(disposition, 330, 300, 300, 80);
			mockClientRect(label, 330, 300, 300, 80);
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

		it("toggles enlarge on an intro flow disposition", () => {
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
			Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
			mockClientRect(stack, 0, 0, 960, 700);
			mockClientRect(section, 0, 0, 960, 700);
			const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
			const heading = disposition.querySelector("h2") as HTMLElement;
			mockClientRect(disposition, 330, 300, 300, 80);
			mockClientRect(heading, 330, 300, 300, 80);
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			const enlargeButton = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			expect(enlargeButton).toBeTruthy();
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			expect(parseFloat(disposition.style.height)).toBeCloseTo(SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100);
			const content = disposition.querySelector(
				".presentation-interactive-disposition__content",
			) as HTMLElement;
			expect(content.style.transform).toMatch(/^scale\(/);
			expect(section.querySelector(".presentation-interaction-slide-reset-host")).toBeTruthy();
			const resetButton = disposition.querySelector(
				".presentation-interaction-reset",
			) as HTMLButtonElement;
			expect(resetButton).toBeTruthy();
			act(() => {
				resetButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
			expect(content.style.transform).toBe("");
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
			expect(globalsCssSource).toMatch(
				/\.presentation-arrangement-surface\s*>\s*\.presentation-interactive-disposition--enlarged/s,
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

		it("toggles enlarge on a canvas-framed disposition", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			act(() => {
				pointerClick(disposition);
			});
			const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
			expect(enlargeButton.getAttribute("aria-pressed")).toBe("false");
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
			expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
			const enlargeOn = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			expect(enlargeOn.getAttribute("aria-pressed")).toBe("true");
			act(() => {
				enlargeOn.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
			const enlargeOff = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			expect(enlargeOff.getAttribute("aria-pressed")).toBe("false");
		});

		it("restores pre-enlarge frame after exit enlarge following drag", () => {
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
			const declaredLeft = parseFloat(disposition.style.left);
			act(() => {
				pointerDrag(disposition, 300, 280, 400, 320);
			});
			const draggedLeft = parseFloat(disposition.style.left);
			expect(draggedLeft).not.toBeCloseTo(declaredLeft, 1);
			const enlargeButton = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
			expect(parseFloat(disposition.style.left)).toBeCloseTo(draggedLeft, 1);
			expect(parseFloat(disposition.style.left)).not.toBeCloseTo(declaredLeft, 1);
		});

		it("exits enlarge through the corner control while slide reset host is active", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			act(() => {
				pointerClick(disposition);
			});
			const enlargeButton = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			act(() => {
				enlargeButton.click();
			});
			expect(section.querySelector(".presentation-interaction-slide-reset-host")).toBeTruthy();
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
			expect(enlargeButton.getAttribute("aria-pressed")).toBe("false");
		});

		it("enlarges when pointerdown lands on the svg icon inside the enlarge button", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			const enlargeButton = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			const icon = enlargeButton.querySelector("svg");
			expect(icon).toBeTruthy();
			act(() => {
				pointerClick(icon!);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
		});

		it("keeps enlarge when empty slide click clears selection", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			act(() => {
				pointerClick(disposition);
			});
			const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			act(() => {
				pointerClick(canvas, 8, 8);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			const enlargeWhileDeselected = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			expect(enlargeWhileDeselected).toBeTruthy();
			act(() => {
				enlargeWhileDeselected.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
		});

		it("clears drag and selection after navigating away and back", async () => {
			const alphaRef = collectPresentationSlides(twoSlideDeck)[0];
			const betaRef = collectPresentationSlides(twoSlideDeck)[1];
			expect(alphaRef?.slide).toBe("alpha");
			expect(betaRef?.slide).toBe("beta");
			let revealApi: Reveal.Api | undefined;
			act(() => {
				mountPresentation(container, twoSlideDeck, {
					hash: false,
					slideNumber: false,
					surfaceChrome: false,
					onRevealReady: (api) => {
						revealApi = api;
					},
				});
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
			const alphaSlide = container.querySelector('section[title="alpha"]') as HTMLElement;
			const disposition = alphaSlide.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(disposition, 192, 210, 384, 140);
			const originLeft = disposition.style.left;
			act(() => {
				pointerClick(disposition);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
			act(() => {
				pointerDrag(disposition, 300, 280, 380, 320);
			});
			const modifiedLeft = disposition.style.left;
			expect(modifiedLeft).not.toBe(originLeft);
			await revealApi!.slide(betaRef!.h, betaRef!.v);
			await new Promise((resolve) => setTimeout(resolve, 50));
			await revealApi!.slide(alphaRef!.h, alphaRef!.v);
			await new Promise((resolve) => setTimeout(resolve, 50));
			const alphaSlideAgain = container.querySelector('section[title="alpha"]') as HTMLElement;
			const dispositionAgain = alphaSlideAgain.querySelector("[data-disposition-id]") as HTMLElement;
			expect(dispositionAgain.style.left).toBe(originLeft);
			expect(dispositionAgain.classList.contains("presentation-interactive-disposition--selected")).toBe(
				false,
			);
		});

		it("resets the whole slide from the proximity control in the top-right corner", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			expect(globalsCssSource).toMatch(
				/\.presentation-interaction-slide-reset-host\s*\{[^}]*pointer-events:\s*none/s,
			);
			expect(globalsCssSource).toMatch(
				/\.presentation-interactive-disposition\s*>\s*\.presentation-interaction-actions\s*\{[^}]*z-index:\s*90/s,
			);
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(disposition, 192, 210, 384, 140);
			const originLeft = disposition.style.left;
			expect(section.querySelector(".presentation-interaction-slide-reset")).toBeNull();
			act(() => {
				pointerDrag(disposition, 300, 280, 380, 320);
			});
			expect(disposition.style.left).not.toBe(originLeft);
			const slideResetHost = section.querySelector(
				".presentation-interaction-slide-reset-host",
			) as HTMLElement;
			const slideReset = slideResetHost.querySelector(
				".presentation-interaction-slide-reset",
			) as HTMLButtonElement;
			expect(slideResetHost).toBeTruthy();
			expect(slideResetHost.classList.contains("presentation-interaction-slide-reset-host--near")).toBe(
				false,
			);
			act(() => {
				window.dispatchEvent(
					new PointerEvent("pointermove", { bubbles: true, clientX: 920, clientY: 20, pointerId: 2 }),
				);
			});
			expect(slideResetHost.classList.contains("presentation-interaction-slide-reset-host--near")).toBe(
				true,
			);
			act(() => {
				slideReset.click();
			});
			expect(disposition.style.left).toBe(originLeft);
			expect(section.querySelector(".presentation-interaction-slide-reset")).toBeNull();
		});

		it("resets a dragged canvas-framed disposition to its declared position", () => {
			act(() => {
				mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
			const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
			const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(canvas, 0, 0, 960, 700);
			mockClientRect(disposition, 192, 210, 384, 140);
			const originLeft = disposition.style.left;
			const originTop = disposition.style.top;
			act(() => {
				pointerDrag(disposition, 300, 280, 380, 320);
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
			expect(disposition.style.left).not.toBe(originLeft);
			expect(disposition.querySelector(".presentation-interaction-enlarge")).toBeTruthy();
			const reset = disposition.querySelector(".presentation-interaction-reset") as HTMLButtonElement;
			expect(reset).toBeTruthy();
			const actions = disposition.querySelector(".presentation-interaction-actions")!;
			const buttons = [...actions.querySelectorAll("button")];
			expect(buttons[0]?.classList.contains("presentation-interaction-reset")).toBe(true);
			expect(buttons[1]?.classList.contains("presentation-interaction-enlarge")).toBe(true);
			act(() => {
				reset.click();
			});
			expect(disposition.style.left).toBe(originLeft);
			expect(disposition.style.top).toBe(originTop);
			expect(disposition.querySelector(".presentation-interaction-reset")).toBeNull();
		});

		it("scales pdf pages to cover the disposition frame", async () => {
			const deck: Presentation = {
				id: "pdf-cover",
				name: "Pdf Cover",
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
			const frame = disposition.querySelector(".presentation-disposition-frame") as HTMLElement;
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(frame, 96, 385, 768, 280);
			await act(async () => {
				const expected = pdfScrollCoverScale(768, 280, 595, 842);
				expect(expected).not.toBeNull();
				for (let attempt = 0; attempt < 20; attempt += 1) {
					await new Promise((resolve) => setTimeout(resolve, 0));
					const page = disposition.querySelector(".react-pdf__Page") as HTMLElement | null;
					const scale = Number(page?.dataset.scale ?? 0);
					if (expected !== null && Math.abs(scale - expected) < 0.01) {
						expect(scale).toBeCloseTo(expected);
						return;
					}
				}
				const page = disposition.querySelector(".react-pdf__Page") as HTMLElement;
				expect(Number(page.dataset.scale)).toBeCloseTo(expected ?? 0);
			});
		});

		it("drops nested pdf frame and uses enlarged slide sizing when toggling enlarge", () => {
			const deck: Presentation = {
				id: "pdf-enlarge",
				name: "Pdf Enlarge",
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
			const enlargeButton = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			const content = disposition.querySelector(
				".presentation-interactive-disposition__content",
			) as HTMLElement;
			mockClientRect(content, 48, 52, 864, 595);
			act(() => {
				enlargeButton.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
			expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
			expect(disposition.querySelector(".presentation-disposition-frame")).toBeNull();
			const pageCanvas = disposition.querySelector(
				".presentation-media-pdf canvas",
			) as HTMLCanvasElement | null;
			if (pageCanvas) {
				expect(pageCanvas.height).toBeGreaterThan(400);
			}
			expect(globalsCssSource).toMatch(
				/\.presentation-interactive-disposition--kind-pdf\.presentation-interactive-disposition--enlarged[\s\S]*\.presentation-media-pdf-document[\s\S]*height\s*:\s*100%/s,
			);
		});

		it("shows center-bottom pdf page nav when enlarged and switches pages", async () => {
			const deck: Presentation = {
				id: "pdf-page-nav",
				name: "Pdf Page Nav",
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
			await act(async () => {
				await new Promise((resolve) => setTimeout(resolve, 0));
			});
			expect(disposition.querySelector(".presentation-pdf-page-nav")).toBeTruthy();
			const content = disposition.querySelector(
				".presentation-interactive-disposition__content",
			) as HTMLElement;
			mockClientRect(content, 48, 52, 864, 595);
			const enlargeButton = disposition.querySelector(
				".presentation-interaction-enlarge",
			) as HTMLButtonElement;
			act(() => {
				enlargeButton.click();
			});
			await act(async () => {
				await new Promise((resolve) => setTimeout(resolve, 0));
			});
			const nav = disposition.querySelector(".presentation-pdf-page-nav");
			expect(nav).toBeTruthy();
			const page = disposition.querySelector(".react-pdf__Page") as HTMLElement;
			expect(page.dataset.page).toBe("1");
			const nextButton = disposition.querySelector(
				".presentation-pdf-page-nav__button--next",
			) as HTMLButtonElement;
			const prevButton = disposition.querySelector(
				".presentation-pdf-page-nav__button--prev",
			) as HTMLButtonElement;
			expect(prevButton.disabled).toBe(true);
			expect(nextButton.disabled).toBe(false);
			act(() => {
				nextButton.click();
			});
			await act(async () => {
				await new Promise((resolve) => setTimeout(resolve, 0));
			});
			expect(disposition.querySelector(".react-pdf__Page")?.getAttribute("data-page")).toBe("2");
			expect(
				(disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement).disabled,
			).toBe(false);
		});

		it("navigates only within pdf pages declared on the embodiment", async () => {
			const deck: Presentation = {
				id: "pdf-page-subset",
				name: "Pdf Page Subset",
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
											{
												kind: "pdf",
												id: "thesis--doc",
												src: "/thesis.pdf",
												page: 1,
												pages: [1, 12, 25],
											},
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
			mockClientRect(section, 0, 0, 960, 700);
			mockClientRect(disposition, 96, 385, 768, 280);
			act(() => {
				pointerClick(disposition);
			});
			await act(async () => {
				await new Promise((resolve) => setTimeout(resolve, 0));
			});
			const nextButton = () =>
				disposition.querySelector(".presentation-pdf-page-nav__button--next") as HTMLButtonElement;
			const prevButton = () =>
				disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement;
			const pageNumber = () =>
				disposition.querySelector(".react-pdf__Page")?.getAttribute("data-page");
			expect(pageNumber()).toBe("1");
			act(() => {
				const next = nextButton();
				pointerClick(next);
				next.click();
			});
			expect(disposition.classList.contains("presentation-interactive-disposition--gesturing")).toBe(
				false,
			);
			await act(async () => {
				await new Promise((resolve) => setTimeout(resolve, 0));
			});
			expect(pageNumber()).toBe("12");
			act(() => {
				const next = nextButton();
				pointerClick(next);
				next.click();
			});
			await act(async () => {
				await new Promise((resolve) => setTimeout(resolve, 0));
			});
			expect(pageNumber()).toBe("25");
			expect(nextButton().disabled).toBe(true);
			act(() => {
				prevButton().click();
			});
			await act(async () => {
				await new Promise((resolve) => setTimeout(resolve, 0));
			});
			expect(pageNumber()).toBe("12");
		});

		it("toggles enlarge on a cropped figure tile disposition", () => {
			const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
			const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
			const deck: Presentation = {
				id: "split-enlarge",
				name: "Split Enlarge",
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
			const enlargeButton = tile.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
			act(() => {
				enlargeButton.click();
			});
			expect(tile.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
			expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
			expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(false);
			expect(tile.style.position).toBe("absolute");
			expect(tile.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
			expect(tile.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
			expect(canvas.contains(tile)).toBe(true);
			const cropSlot = tile.querySelector(".presentation-morph-slot--figure") as HTMLElement | null;
			expect(cropSlot).toBeTruthy();
			expect(tile.querySelector(".presentation-media-figure")).toBeNull();
			expect(cropSlot?.style.backgroundImage).toContain("catalogue.png");
			const bgSize = cropSlot?.style.getPropertyValue("--presentation-figure-bg-size");
			expect(bgSize).toBeTruthy();
			expect(bgSize).not.toBe("100% 100%");
			expect(globalsCssSource).toMatch(
				/\.presentation-interactive-disposition--enlarged:not\(\.presentation-interactive-disposition--offset\)[\s\S]*\.presentation-figure-crop-fill[\s\S]*width\s*:\s*100%\s*!important/s,
			);
			act(() => {
				enlargeButton.click();
			});
			expect(tile.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
			expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
		});
	});
}
//#endregion 🧪Tests

//#region 🔖json

//#region 🔖Renderer
/** @emoji 🧬 Renders parsed JSON as an interactive syntax tree. */
export interface JsonTreeRenderer {
	render(data: unknown): ReactNode;
}

function jsonPreview(value: unknown): string {
	if (value === null) {
		return "null";
	}
	switch (typeof value) {
		case "string":
			return value.length > 48 ? `${value.slice(0, 45)}…` : value;
		case "number":
		case "boolean":
		case "undefined":
			return String(value);
		case "object":
			return Array.isArray(value) ? `Array(${value.length})` : `Object(${Object.keys(value as object).length})`;
		default:
			return String(value);
	}
}

function jsonEntries(value: unknown): readonly (readonly [string, unknown])[] {
	if (typeof value !== "object" || value === null) {
		return [];
	}
	if (Array.isArray(value)) {
		return value.map((entry, index) => [String(index), entry] as const);
	}
	return Object.entries(value as Record<string, unknown>);
}

function JsonScalar({ value }: { readonly value: unknown }): ReactNode {
	if (value === null) {
		return <span className="presentation-json-null">null</span>;
	}
	switch (typeof value) {
		case "string":
			return <span className="presentation-json-string">"{value}"</span>;
		case "number":
			return <span className="presentation-json-number">{value}</span>;
		case "boolean":
			return <span className="presentation-json-boolean">{String(value)}</span>;
		default:
			return <span className="presentation-json-unknown">{String(value)}</span>;
	}
}

function JsonBranch({
	label,
	value,
	depth,
	defaultExpanded,
}: {
	readonly label: string;
	readonly value: unknown;
	readonly depth: number;
	readonly defaultExpanded: boolean;
}): ReactNode {
	const [expanded, setExpanded] = useState(defaultExpanded);
	const isArray = Array.isArray(value);
	const entries = jsonEntries(value);
	const canExpand = entries.length > 0;
	if (!canExpand) {
		return (
			<div className="presentation-json-line" style={{ paddingInlineStart: `${depth}ch` }}>
				<span className="presentation-json-key">{label}</span>
				<span className="presentation-json-colon">: </span>
				<JsonScalar value={value} />
			</div>
		);
	}
	return (
		<div className="presentation-json-branch">
			<button
				type="button"
				className="presentation-json-line presentation-json-toggle"
				style={{ paddingInlineStart: `${depth}ch` }}
				aria-expanded={expanded}
				onClick={() => setExpanded((open) => !open)}
			>
				<span className="presentation-json-caret" aria-hidden="true">
					{expanded ? "▾" : "▸"}
				</span>
				<span className="presentation-json-key">{label}</span>
				<span className="presentation-json-colon">: </span>
				<span className="presentation-json-meta">{isArray ? `[${entries.length}]` : `{${entries.length}}`}</span>
				{!expanded ? (
					<>
						<span className="presentation-json-colon"> </span>
						<span className="presentation-json-preview">{jsonPreview(value)}</span>
					</>
				) : null}
			</button>
			{expanded
				? entries.map(([key, entry]) => (
						<JsonBranch
							key={key}
							label={isArray ? `[${key}]` : key}
							value={entry}
							depth={depth + 1}
							defaultExpanded={depth < 1}
						/>
					))
				: null}
		</div>
	);
}

function DefaultJsonTree({ data }: { readonly data: unknown }): ReactNode {
	if (typeof data !== "object" || data === null) {
		return (
			<div className="presentation-json-tree">
				<JsonScalar value={data} />
			</div>
		);
	}
	const isArray = Array.isArray(data);
	const entries = jsonEntries(data);
	return (
		<div className="presentation-json-tree">
			{entries.map(([key, entry]) => (
				<JsonBranch
					key={key}
					label={isArray ? `[${key}]` : key}
					value={entry}
					depth={0}
					defaultExpanded
				/>
			))}
		</div>
	);
}

const defaultJsonTreeRenderer: JsonTreeRenderer = {
	render(data) {
		return <DefaultJsonTree data={data} />;
	},
};

let jsonTreeRenderer: JsonTreeRenderer = defaultJsonTreeRenderer;

/** @emoji 🔌 Replaces the JSON tree renderer (tests or alternate renderers). */
export function setJsonTreeRenderer(renderer: JsonTreeRenderer): void {
	jsonTreeRenderer = renderer;
}

/** @emoji 🧬 Renders JSON through the active {@link JsonTreeRenderer}. */
export function renderJsonTree(data: unknown): ReactNode {
	return jsonTreeRenderer.render(data);
}
//#endregion 🔖Renderer

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("renderJsonTree", () => {
		it("renders nested object keys in preview metadata", () => {
			expect(jsonPreview({ item: { item_id: "x" }, tags: ["a", "b"] })).toBe("Object(2)");
			expect(jsonPreview(["alpha", "beta"])).toBe("Array(2)");
			expect(jsonPreview(null)).toBe("null");
		});

		it("accepts nested null and undefined property values", () => {
			expect(() =>
				renderJsonTree({
					price_amount: null,
					currency: undefined,
					nested: { value: null },
				}),
			).not.toThrow();
		});
	});
}
//#endregion 🧪Tests
//#endregion 🔖json

//#region 🔖PlayHost
import type { ReactElement } from "react";
import type { AppRendererContribution, UiPanelHostSurfaceNode } from "@semio-tech/framework-platform-core";
import { useApp, CommandBus, useControllerStore, controllerBackedExampleContribution } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort, cn, Button, floatingFieldSurfaceClass, floatingMenuSurfaceClass, shellChromeTitleClassName } from "@semio-tech/ui-react";
import * as React from "react";
import { FIGURE_TILE_PDF_PAGE_ASPECT, NORMALIZED_RECT_MIN_FRACTION, figureTileMediaKindFromFile, moveNormalizedRect, resizeNormalizedRect, type FigureTileMediaKind, type FigureTileSource, type NormalizedRectHandle } from "@semio-tech/framework-presentation-core";
import { PRESENTATION_PLAY_CONTROLLER_ID, PRESENTATION_PLAY_ICON_DETAILS, PRESENTATION_PLAY_ICON_HIERARCHY, PRESENTATION_PLAY_IDLE_SNAPSHOT, PRESENTATION_PLAY_STORE_ID, PRESENTATION_PLAY_SURFACE_ID, PresentationPlayController, type PresentationPlaySnapshot, presentationPlayWindowBodies, presentationPlaySidePanelBodies } from "@semio-tech/framework-presentation-core";

const PRESENTATION_TILE_HANDLES: readonly NormalizedRectHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const PRESENTATION_TILE_VIEWPORT_MIN_ZOOM = 0.2;
const PRESENTATION_TILE_VIEWPORT_MAX_ZOOM = 12;
const PRESENTATION_FIGURE_FILE_ACCEPT =
	"image/*,video/*,application/pdf,.pdf,.svg,.png,.jpg,.jpeg,.webp,.gif,.bmp,.avif,.mp4,.webm,.ogg,.ogv,.mov,.m4v,.mkv";

function clampFigureTileZoom(zoom: number): number {
	return Math.min(PRESENTATION_TILE_VIEWPORT_MAX_ZOOM, Math.max(PRESENTATION_TILE_VIEWPORT_MIN_ZOOM, zoom));
}

interface FigureTileViewportState {
	readonly zoom: number;
	readonly panX: number;
	readonly panY: number;
}

interface FigureTileContentLayout {
	readonly width: number;
	readonly height: number;
	readonly offsetX: number;
	readonly offsetY: number;
}

function figureTileContentLayout(viewportWidth: number, viewportHeight: number, aspect: number): FigureTileContentLayout {
	if (viewportWidth <= 0 || viewportHeight <= 0) {
		return { width: 1, height: 1, offsetX: 0, offsetY: 0 };
	}
	const viewportAspect = viewportWidth / viewportHeight;
	if (viewportAspect >= aspect) {
		const height = viewportHeight;
		const width = height * aspect;
		return { width, height, offsetX: (viewportWidth - width) / 2, offsetY: 0 };
	}
	const width = viewportWidth;
	const height = width / aspect;
	return { width, height, offsetX: 0, offsetY: (viewportHeight - height) / 2 };
}

function figureTileZoomAtClient(
	viewport: FigureTileViewportState,
	clientX: number,
	clientY: number,
	viewportRect: DOMRect,
	layout: FigureTileContentLayout,
	deltaScale: number,
): FigureTileViewportState {
	const nextZoom = clampFigureTileZoom(viewport.zoom * deltaScale);
	if (nextZoom === viewport.zoom) {
		return viewport;
	}
	const anchorX = clientX - viewportRect.left;
	const anchorY = clientY - viewportRect.top;
	const contentX = (anchorX - layout.offsetX - viewport.panX) / viewport.zoom;
	const contentY = (anchorY - layout.offsetY - viewport.panY) / viewport.zoom;
	return {
		zoom: nextZoom,
		panX: anchorX - layout.offsetX - contentX * nextZoom,
		panY: anchorY - layout.offsetY - contentY * nextZoom,
	};
}

function revokeFigureObjectUrl(url: string | null): void {
	if (url?.startsWith("blob:")) {
		URL.revokeObjectURL(url);
	}
}

function probeFigureTileMediaAspect(
	src: string,
	kind: FigureTileMediaKind,
): Promise<number> {
	if (kind === "video") {
		return new Promise((resolve, reject) => {
			const video = document.createElement("video");
			video.preload = "metadata";
			video.onloadedmetadata = () => {
				const aspect = video.videoWidth > 0 && video.videoHeight > 0 ? video.videoWidth / video.videoHeight : 16 / 9;
				resolve(aspect);
			};
			video.onerror = () => reject(new Error("video metadata"));
			video.src = src;
		});
	}
	if (kind === "pdf") {
		return Promise.resolve(FIGURE_TILE_PDF_PAGE_ASPECT);
	}
	return new Promise((resolve, reject) => {
		const img = new Image();
		img.onload = () => {
			const aspect = img.naturalWidth > 0 && img.naturalHeight > 0 ? img.naturalWidth / img.naturalHeight : 1;
			resolve(aspect);
		};
		img.onerror = () => reject(new Error("image metadata"));
		img.src = src;
	});
}

function FigureTileMediaPreview(props: { readonly source: FigureTileSource }): ReactElement {
	const { source } = props;
	const kind = source.kind ?? "figure";
	if (kind === "video") {
		return (
			<video
				className="pointer-events-none absolute inset-0 h-full w-full object-contain"
				src={source.src}
				muted
				playsInline
				preload="metadata"
				controls={false}
			/>
		);
	}
	if (kind === "pdf") {
		const page = source.pdfPage ?? 1;
		const pdfSrc = `${source.src}#page=${page}&view=FitH`;
		return <iframe className="pointer-events-none absolute inset-0 h-full w-full border-0 bg-background" src={pdfSrc} title="PDF preview" />;
	}
	return <img alt="" className="pointer-events-none absolute inset-0 h-full w-full object-contain" draggable={false} src={source.src} />;
}

function FigureSourcePicker(props: {
	readonly onPickFile: (file: File) => void;
}): ReactElement {
	const { onPickFile } = props;
	const fileInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
	const [dragActive, setDragActive] = reactHostPort.useState(false);

	const onInputChange = reactHostPort.useCallback(
		(event: React.ChangeEvent<HTMLInputElement>) => {
			const file = event.target.files?.[0];
			if (file) {
				onPickFile(file);
			}
			event.target.value = "";
		},
		[onPickFile],
	);

	const onDragOver = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
		event.preventDefault();
		setDragActive(true);
	}, []);

	const onDragLeave = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
		event.preventDefault();
		setDragActive(false);
	}, []);

	const onDrop = reactHostPort.useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			event.preventDefault();
			setDragActive(false);
			const file = event.dataTransfer.files?.[0];
			if (file) {
				onPickFile(file);
			}
		},
		[onPickFile],
	);

	return (
		<div
			className={cn(
				"flex min-h-0 flex-1 flex-col items-center justify-center gap-3 border-dashed p-6 text-center",
				floatingFieldSurfaceClass,
				dragActive && "border-primary",
			)}
			onDragLeave={onDragLeave}
			onDragOver={onDragOver}
			onDrop={onDrop}
		>
			<Icon icon="image-up" size="large" className="text-muted-foreground" />
			<div className="flex flex-col gap-1">
				<p className={shellChromeTitleClassName}>Pick figure media</p>
				<p className="text-muted-foreground text-xs">Image, SVG, video, or PDF — drag and drop or choose a file</p>
			</div>
			<Button id="presentation.play.pick-figure" type="button" variant="secondary" onClick={() => fileInputRef.current?.click()}>
				Choose file…
			</Button>
			<input accept={PRESENTATION_FIGURE_FILE_ACCEPT} className="hidden" onChange={onInputChange} ref={fileInputRef} type="file" />
		</div>
	);
}

function usePresentationPlayController(): PresentationPlayController | undefined {
	const { runtime } = useApp();
	return runtime.getActiveApp()?.controller as PresentationPlayController | undefined;
}

function usePresentationPlaySnapshot(): PresentationPlaySnapshot {
	const ctrl = usePresentationPlayController();
	return useControllerStore(ctrl, PRESENTATION_PLAY_STORE_ID) ?? PRESENTATION_PLAY_IDLE_SNAPSHOT;
}

function clampUnit(value: number): number {
	return Math.min(1, Math.max(0, value));
}

function normalizedPointFromClient(
	clientX: number,
	clientY: number,
	viewportRect: DOMRect,
	viewport: FigureTileViewportState,
	layout: FigureTileContentLayout,
): { readonly x: number; readonly y: number } {
	const localX = (clientX - viewportRect.left - layout.offsetX - viewport.panX) / viewport.zoom;
	const localY = (clientY - viewportRect.top - layout.offsetY - viewport.panY) / viewport.zoom;
	return {
		x: clampUnit(localX / layout.width),
		y: clampUnit(localY / layout.height),
	};
}

function normalizedRectFromDrag(
	start: { readonly x: number; readonly y: number },
	end: { readonly x: number; readonly y: number },
): DispositionPosition {
	const x = Math.min(start.x, end.x);
	const y = Math.min(start.y, end.y);
	const width = Math.max(NORMALIZED_RECT_MIN_FRACTION, Math.abs(end.x - start.x));
	const height = Math.max(NORMALIZED_RECT_MIN_FRACTION, Math.abs(end.y - start.y));
	return {
		x: clampUnit(x),
		y: clampUnit(y),
		width: Math.min(width, 1 - x),
		height: Math.min(height, 1 - y),
	};
}

function FigureTilesSurfaceHost({ node }: { readonly node: UiPanelHostSurfaceNode }): ReactElement {
	const { runtime } = useApp();
	const controller = usePresentationPlayController();
	const snapshot = usePresentationPlaySnapshot();
	const viewportRef = reactHostPort.useRef<HTMLDivElement | null>(null);
	const contentRef = reactHostPort.useRef<HTMLDivElement | null>(null);
	const figureObjectUrlRef = reactHostPort.useRef<string | null>(null);
	const spacePressedRef = reactHostPort.useRef(false);
	const [viewportSize, setViewportSize] = reactHostPort.useState({ width: 0, height: 0 });
	const [viewport, setViewport] = reactHostPort.useState<FigureTileViewportState>({ zoom: 1, panX: 0, panY: 0 });
	const [spacePressed, setSpacePressed] = reactHostPort.useState(false);
	const [isPanning, setIsPanning] = reactHostPort.useState(false);
	const [marquee, setMarquee] = reactHostPort.useState<{ readonly start: { readonly x: number; readonly y: number }; readonly end: { readonly x: number; readonly y: number } } | null>(null);
	const dragRef = reactHostPort.useRef<
		| {
				readonly kind: "move" | NormalizedRectHandle | "marquee" | "pan";
				readonly tileId?: string;
				readonly startClient: { readonly x: number; readonly y: number };
				readonly startCrop?: DispositionPosition;
				readonly marqueeStart?: { readonly x: number; readonly y: number };
				readonly startPan?: { readonly x: number; readonly y: number };
		  }
		| null
	>(null);

	reactHostPort.useEffect(() => {
		if (!snapshot.clipboardPrompt || snapshot.clipboardEpoch <= 0) {
			return;
		}
		void navigator.clipboard?.writeText(snapshot.clipboardPrompt).catch(() => undefined);
	}, [snapshot.clipboardEpoch, snapshot.clipboardPrompt]);

	const dispatch = reactHostPort.useCallback(
		(command: string, args?: unknown) => {
			if (!controller) {
				return;
			}
			runtime.commandBus.dispatch(controller.id, command, args);
		},
		[controller, runtime.commandBus],
	);

	reactHostPort.useEffect(() => () => revokeFigureObjectUrl(figureObjectUrlRef.current), []);

	reactHostPort.useEffect(() => {
		setViewport({ zoom: 1, panX: 0, panY: 0 });
	}, [snapshot.source.src]);

	reactHostPort.useEffect(() => {
		const onKeyDown = (event: KeyboardEvent) => {
			if (event.code !== "Space" || event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
				return;
			}
			event.preventDefault();
			spacePressedRef.current = true;
			setSpacePressed(true);
		};
		const onKeyUp = (event: KeyboardEvent) => {
			if (event.code !== "Space") {
				return;
			}
			spacePressedRef.current = false;
			setSpacePressed(false);
		};
		window.addEventListener("keydown", onKeyDown);
		window.addEventListener("keyup", onKeyUp);
		return () => {
			window.removeEventListener("keydown", onKeyDown);
			window.removeEventListener("keyup", onKeyUp);
		};
	}, []);

	const applyFigureFile = reactHostPort.useCallback(
		(file: File) => {
			const kind = figureTileMediaKindFromFile(file.type, file.name);
			if (!kind) {
				return;
			}
			revokeFigureObjectUrl(figureObjectUrlRef.current);
			const url = URL.createObjectURL(file);
			figureObjectUrlRef.current = url;
			void probeFigureTileMediaAspect(url, kind)
				.then((sourceAspect) => {
					dispatch("setSource", {
						src: url,
						kind,
						sourceAspect,
						...(kind === "pdf" ? { pdfPage: 1 } : {}),
					});
				})
				.catch(() => {
					revokeFigureObjectUrl(url);
					if (figureObjectUrlRef.current === url) {
						figureObjectUrlRef.current = null;
					}
				});
		},
		[dispatch],
	);

	const hasFigure = snapshot.source.src.trim().length > 0;
	const aspect = snapshot.source.sourceAspect ?? 1;
	const contentLayout = reactHostPort.useMemo(
		() => figureTileContentLayout(viewportSize.width, viewportSize.height, aspect),
		[aspect, viewportSize.height, viewportSize.width],
	);

	reactHostPort.useEffect(() => {
		const element = viewportRef.current;
		if (!element || !hasFigure) {
			return;
		}
		const observer = new ResizeObserver(([entry]) => {
			const { width, height } = entry.contentRect;
			setViewportSize({ width, height });
		});
		observer.observe(element);
		return () => observer.disconnect();
	}, [hasFigure]);

	reactHostPort.useEffect(() => {
		const element = viewportRef.current;
		if (!element || !hasFigure) {
			return;
		}
		const onWheel = (event: WheelEvent) => {
			event.preventDefault();
			const rect = element.getBoundingClientRect();
			const layout = figureTileContentLayout(viewportSize.width, viewportSize.height, aspect);
			const deltaScale = event.deltaY < 0 ? 1.1 : 1 / 1.1;
			setViewport((current) => figureTileZoomAtClient(current, event.clientX, event.clientY, rect, layout, deltaScale));
		};
		element.addEventListener("wheel", onWheel, { passive: false });
		return () => element.removeEventListener("wheel", onWheel);
	}, [aspect, hasFigure, viewportSize.height, viewportSize.width]);

	const viewportPoint = reactHostPort.useCallback(
		(clientX: number, clientY: number) => {
			const rect = viewportRef.current?.getBoundingClientRect();
			if (!rect) {
				return { x: 0, y: 0 };
			}
			return normalizedPointFromClient(clientX, clientY, rect, viewport, contentLayout);
		},
		[contentLayout, viewport],
	);

	const onContentPointerDown = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!viewportRef.current) {
				return;
			}
			const target = event.target as HTMLElement;
			if (target.dataset.tileHandle || target.dataset.tileId) {
				return;
			}
			if (event.button === 1 || (event.button === 0 && (spacePressedRef.current || event.altKey))) {
				dragRef.current = {
					kind: "pan",
					startClient: { x: event.clientX, y: event.clientY },
					startPan: { x: viewport.panX, y: viewport.panY },
				};
				setIsPanning(true);
				event.currentTarget.setPointerCapture(event.pointerId);
				return;
			}
			if (event.button !== 0) {
				return;
			}
			const point = viewportPoint(event.clientX, event.clientY);
			dragRef.current = {
				kind: "marquee",
				startClient: { x: event.clientX, y: event.clientY },
				marqueeStart: point,
			};
			setMarquee({ start: point, end: point });
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[viewport.panX, viewport.panY, viewportPoint],
	);

	const onTilePointerDown = reactHostPort.useCallback(
		(tileId: string, crop: DispositionPosition) => (event: React.PointerEvent) => {
			event.stopPropagation();
			if (spacePressedRef.current || event.altKey) {
				return;
			}
			dispatch("setSelectedIds", { ids: [tileId] });
			dragRef.current = {
				kind: "move",
				tileId,
				startClient: { x: event.clientX, y: event.clientY },
				startCrop: crop,
			};
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[dispatch],
	);

	const onHandlePointerDown = reactHostPort.useCallback(
		(tileId: string, crop: DispositionPosition, handle: NormalizedRectHandle) => (event: React.PointerEvent) => {
			event.stopPropagation();
			dispatch("setSelectedIds", { ids: [tileId] });
			dragRef.current = {
				kind: handle,
				tileId,
				startClient: { x: event.clientX, y: event.clientY },
				startCrop: crop,
			};
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[dispatch],
	);

	const onPointerMove = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			const drag = dragRef.current;
			if (!drag) {
				return;
			}
			if (drag.kind === "pan" && drag.startPan) {
				setViewport((current) => ({
					...current,
					panX: drag.startPan!.x + (event.clientX - drag.startClient.x),
					panY: drag.startPan!.y + (event.clientY - drag.startClient.y),
				}));
				return;
			}
			const scaleX = contentLayout.width * viewport.zoom;
			const scaleY = contentLayout.height * viewport.zoom;
			const dx = scaleX > 0 ? (event.clientX - drag.startClient.x) / scaleX : 0;
			const dy = scaleY > 0 ? (event.clientY - drag.startClient.y) / scaleY : 0;
			if (drag.kind === "marquee" && drag.marqueeStart) {
				const end = viewportPoint(event.clientX, event.clientY);
				setMarquee({ start: drag.marqueeStart, end });
				return;
			}
			if (!drag.tileId || !drag.startCrop) {
				return;
			}
			const nextCrop =
				drag.kind === "move"
					? moveNormalizedRect(drag.startCrop, dx, dy)
					: resizeNormalizedRect(drag.startCrop, drag.kind, dx, dy);
			dispatch("setTileCrop", { id: drag.tileId, crop: nextCrop });
		},
		[contentLayout.height, contentLayout.width, dispatch, viewport.zoom, viewportPoint],
	);

	const onPointerUp = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			const drag = dragRef.current;
			if (!drag) {
				return;
			}
			if (drag.kind === "marquee" && drag.marqueeStart) {
				const end = viewportPoint(event.clientX, event.clientY);
				const crop = normalizedRectFromDrag(drag.marqueeStart, end);
				dispatch("addTile", { crop });
				setMarquee(null);
			}
			if (drag.kind === "pan") {
				setIsPanning(false);
			}
			dragRef.current = null;
			try {
				event.currentTarget.releasePointerCapture(event.pointerId);
			} catch {
				// pointer already released
			}
		},
		[dispatch, viewportPoint],
	);

	const onViewportDoubleClick = reactHostPort.useCallback((event: React.MouseEvent<HTMLDivElement>) => {
		const target = event.target as HTMLElement;
		if (target.dataset.tileHandle || target.dataset.tileId) {
			return;
		}
		setViewport({ zoom: 1, panX: 0, panY: 0 });
	}, []);

	if (node.controllerId !== PRESENTATION_PLAY_CONTROLLER_ID || node.surfaceId !== PRESENTATION_PLAY_SURFACE_ID) {
		return <div className="p-2 text-xs text-muted-foreground">Invalid presentation tile surface binding</div>;
	}

	if (!hasFigure) {
		return (
			<div className="flex h-full min-h-0 w-full p-2">
				<FigureSourcePicker onPickFile={applyFigureFile} />
			</div>
		);
	}

	const viewportCursor = isPanning ? "grabbing" : spacePressed ? "grab" : undefined;

	return (
		<div className="flex h-full min-h-0 w-full flex-col">
			<div ref={viewportRef} className="relative min-h-0 flex-1 overflow-hidden bg-muted/30" style={{ cursor: viewportCursor }}>
				<div
					ref={contentRef}
					className="absolute touch-none select-none"
					style={{
						left: contentLayout.offsetX,
						top: contentLayout.offsetY,
						width: contentLayout.width,
						height: contentLayout.height,
						transform: `translate(${viewport.panX}px, ${viewport.panY}px) scale(${viewport.zoom})`,
						transformOrigin: "0 0",
					}}
					onPointerDown={onContentPointerDown}
					onPointerMove={onPointerMove}
					onPointerUp={onPointerUp}
					onPointerCancel={onPointerUp}
					onDoubleClick={onViewportDoubleClick}
				>
					<FigureTileMediaPreview source={snapshot.source} />
					{snapshot.tiles.map((tile) => {
						const selected = snapshot.selectedIds.includes(tile.id);
						return (
							<div
								key={tile.id}
								data-tile-id={tile.id}
								className={cn(
									"absolute box-border cursor-move border-2",
									selected ? "border-primary bg-primary/20" : "border-accent bg-accent/10",
								)}
								style={{
									left: `${tile.crop.x * 100}%`,
									top: `${tile.crop.y * 100}%`,
									width: `${tile.crop.width * 100}%`,
									height: `${tile.crop.height * 100}%`,
								}}
								onPointerDown={onTilePointerDown(tile.id, tile.crop)}
							>
								<span className={cn("pointer-events-none absolute left-0 top-0 max-w-full truncate px-1 text-2xs", floatingMenuSurfaceClass)}>{tile.name}</span>
								{selected
									? PRESENTATION_TILE_HANDLES.map((handle) => (
											<button
												key={handle}
												type="button"
												data-tile-handle={handle}
												className="bg-primary absolute z-10 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full border border-background"
												style={{
													left: handle.includes("w") ? "0%" : handle.includes("e") ? "100%" : "50%",
													top: handle.includes("n") ? "0%" : handle.includes("s") ? "100%" : "50%",
													cursor: `${handle}-resize`,
												}}
												onPointerDown={onHandlePointerDown(tile.id, tile.crop, handle)}
											/>
										))
									: null}
							</div>
						);
					})}
					{marquee ? (
						<div
							className="border-primary/80 bg-primary/10 pointer-events-none absolute border border-dashed"
							style={{
								left: `${Math.min(marquee.start.x, marquee.end.x) * 100}%`,
								top: `${Math.min(marquee.start.y, marquee.end.y) * 100}%`,
								width: `${Math.abs(marquee.end.x - marquee.start.x) * 100}%`,
								height: `${Math.abs(marquee.end.y - marquee.start.y) * 100}%`,
							}}
						/>
					) : null}
				</div>
			</div>
		</div>
	);
}

/** @emoji 🛝 Presentation app renderer for playground and OS shells. */
export const presentationAppRenderer: AppRendererContribution = {
  windowBodies: presentationPlayWindowBodies,
  sidePanelBodies: presentationPlaySidePanelBodies,
  surfaceHosts: {
    [PRESENTATION_PLAY_SURFACE_ID]: FigureTilesSurfaceHost,
  },
  tabIcons: {
    [PRESENTATION_PLAY_ICON_HIERARCHY]: "list-tree",
    [PRESENTATION_PLAY_ICON_DETAILS]: "clipboard-list",
  },
  examples: controllerBackedExampleContribution(PRESENTATION_PLAY_CONTROLLER_ID, []),
};
//#endregion 🔖PlayHost
