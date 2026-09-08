// #region 🧲️Header
/** @emoji 📽️ React + reveal.js renderer for `@semio-tech/animate-presentation-core` declarative decks. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
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
  VideoEmbodiment,
} from "@semio-tech/animate-presentation-core";
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
  unionDispositionPositions,
} from "@semio-tech/animate-presentation-core";
import {
  applyElementsSurfaceChrome,
  DEFAULT_UI_DRIVER,
  Icon,
  Scrollable,
  SelectionMarquee,
  type ElementsSurfaceChromeInput,
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
  createRoot,
  type Root,
} from "@semio-tech/ui-react";
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import "./🎨️.css";
import {
  PdfCanvasResourceOwner,
  pdfCanvasBitmapSize,
  pdfCanvasStatusAnnouncement,
  type PdfCanvasDocument,
  type PdfCanvasLoadingTask,
  type PdfCanvasPage,
  type PdfCanvasPort,
  type PdfCanvasRenderTask,
  type PdfCanvasStatus,
} from "./🔨️modules/🔌️pdf-canvas-port/🟦️.ts";
import { compileOwnedMarkdownToHtml } from "./🔨️modules/📝️markdown-html-compiler/🟦️.ts";
// #endregion 🔌️Adapters

export {
  PdfCanvasResourceOwner,
  pdfCanvasBitmapSize,
  pdfCanvasStatusAnnouncement,
  type PdfCanvasDocument,
  type PdfCanvasLoadingTask,
  type PdfCanvasPage,
  type PdfCanvasPort,
  type PdfCanvasRenderTask,
  type PdfCanvasStatus,
  type PdfCanvasViewport,
} from "./🔨️modules/🔌️pdf-canvas-port/🟦️.ts";

// #region 🔖️Markdown
/** @emoji 📝️ Compiles markdown source into an HTML fragment. */
export interface MarkdownHtmlCompiler {
  compile(markdown: string): Promise<string>;
}

const defaultMarkdownHtmlCompiler: MarkdownHtmlCompiler = {
  compile: compileOwnedMarkdownToHtml,
};

const markdownHtmlCompiler = ephemeralBox<MarkdownHtmlCompiler>("s.plugins.animate.apps.presentation.renderer.react.component.tsx.markdownHtmlCompiler", defaultMarkdownHtmlCompiler);

/** @emoji 🔌️ Replaces the markdown HTML compiler (tests or alternate renderers). */
export function setMarkdownHtmlCompiler(compiler: MarkdownHtmlCompiler): void {
  markdownHtmlCompiler.current = compiler;
}

/** @emoji 📝️ Compiles markdown through the active {@link MarkdownHtmlCompiler}. */
export function compileMarkdownToHtml(markdown: string): Promise<string> {
  return markdownHtmlCompiler.current.compile(markdown);
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx");
  await registerTests1(import.meta.vitest, { compileMarkdownToHtml }, { directory: import.meta.dir, url: import.meta.url });
}
// #endregion 🔖️Markdown

export type {
  AffiliationEntry,
  AffiliationsEmbodiment,
  Arrangement,
  AuthorPerson,
  AuthorsEmbodiment,
  BulletEmbodiment,
  Chapter,
  Disposition,
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
  PdfEmbodiment,
  Presentation,
  RenderSlide,
  ResolvedDisposition,
  Sequence,
  Slide,
  TextEmbodiment,
  Thought,
  Transition,
  VideoEmbodiment,
} from "@semio-tech/animate-presentation-core";

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
} from "@semio-tech/animate-presentation-core";
export type { MorphFromSlot, PresentationLanguageKind, PresentationSlideBookmark, PresentationSlideBookmarkParamKeys, PresentationSlideRef, RenderSlide, Slide, TextMorphRoot } from "@semio-tech/animate-presentation-core";

//#region 🔖️MountOptions
/** @emoji ⚙️ Reveal.js and @semio-tech/ui-react surface chrome options for {@link mountPresentation}. */
export interface PresentationMountOptions {
  readonly surfaceChrome?: ElementsSurfaceChromeInput | false;
  readonly transition?: "fade" | "slide" | "convex" | "concave" | "zoom" | "none";
  /** @emoji 🔗️ Sync slide position to the URL hash; defaults to true. */
  readonly hash?: boolean;
  readonly slideNumber?: boolean;
  readonly width?: number;
  readonly height?: number;
  /** @emoji 🎞️ Called once reveal.js finished initializing (tests and tooling). */
  readonly onRevealReady?: (api: Reveal.Api) => void;
}

/** @emoji 🔗️ Writes reveal.js hash with localized bookmark params after the path; bookmark params are ignored for navigation. */
export function syncPresentationSlideUrl(presentation: Presentation, indices: { readonly h: number; readonly v: number }): void {
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

/** @emoji 🔗️ Reads reveal.js slide indices from the URL hash; trailing bookmark query params are ignored. */
export function readPresentationSlideIndicesFromUrl(hash: string = typeof window !== "undefined" ? window.location.hash : ""): { readonly h: number; readonly v: number } | null {
  return parsePresentationSlideHash(hash);
}
//#endregion 🔖️MountOptions

const DEFAULT_SURFACE_CHROME: ElementsSurfaceChromeInput = {
  appearance: "system",
  device: "desktop",
  driver: DEFAULT_UI_DRIVER,
};

//#region 🔖️RevealChrome
/** @emoji 📐️ Writes reveal slide dimensions as CSS variables for positioned arrangement canvases. */
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
  const durationSeconds = typeof deck?.getConfig().autoAnimateDuration === "number" ? deck.getConfig().autoAnimateDuration : 1;
  deckEl.style.setProperty("--presentation-auto-animate-duration", `${durationSeconds}s`);
}

/** @emoji 🌓️ Align reveal `has-dark-background` with `html.dark` from system chrome. */
export function syncRevealBackgroundKind(deckEl: HTMLElement | null): void {
  if (!deckEl || typeof document === "undefined") {
    return;
  }
  const dark = document.documentElement.classList.contains("dark");
  deckEl.classList.toggle("has-dark-background", dark);
  deckEl.classList.toggle("has-light-background", !dark);
}
//#endregion 🔖️RevealChrome

//#region 🔖️RevealMorph
/** @emoji 👻️ reveal.js-only morph companion role (not part of presentation core). */
export type RevealMorphCompanionKind = "source" | "target";

/** @emoji ✅️ Resolved disposition plus optional reveal.js morph companion metadata. */
export interface RevealResolvedDisposition extends ResolvedDisposition {
  readonly revealMorphCompanion?: RevealMorphCompanionKind;
  /** @emoji 📐️ Source slide frame for target ghosts: paired with {@link ResolvedDisposition.position} for frame and crop morph during auto-animate. */
  readonly revealMorphFromFrame?: DispositionPosition;
  /** @emoji 📐️ Target slide frame for source tiles: paired with {@link ResolvedDisposition.position} when the next slide morphFrom references this participant. */
  readonly revealMorphToFrame?: DispositionPosition;
  /** @emoji 📐️ Previous-slide morphTo slot frame (catalogue grid) for crop morph into {@link ResolvedDisposition.position}. */
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

function revealMorphCompanionFromMorphFrom(scope: ReturnType<typeof buildResolutionScope>, sourceSlide: Slide, arrangement: Arrangement, options?: { readonly morphLineTargets?: boolean }): RevealResolvedDisposition[] {
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
        throw new Error(`morphFrom slot for "${slot.participantId}" needs embodimentId (or a source disposition with embodimentId).`);
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

function revealMorphCompanionFromMorphTo(scope: ReturnType<typeof buildResolutionScope>, targetSlide: Slide, arrangement: Arrangement): RevealResolvedDisposition[] {
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
        throw new Error(`morphTo slot for "${slot.participantId}" needs embodimentId (or a target disposition with embodimentId).`);
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

/** @emoji 🔀️ Resolves an arrangement and appends reveal.js morph companions for one-to-many / many-to-one. */
export function resolveRevealArrangement(scope: ReturnType<typeof buildResolutionScope>, arrangement: Arrangement, context: { readonly previousSlide?: Slide; readonly nextSlide?: Slide }): RevealResolvedDisposition[] {
  const resolved = resolveArrangement(scope, arrangement) as RevealResolvedDisposition[];
  const companions: RevealResolvedDisposition[] = [];
  if (context.previousSlide !== undefined) {
    companions.push(...revealMorphCompanionFromMorphFrom(scope, context.previousSlide, arrangement));
  }
  const hasMorphTo = arrangement.dispositions.some((disposition) => (disposition.morphTo?.length ?? 0) > 0);
  if (context.nextSlide !== undefined && hasMorphTo) {
    companions.push(...revealMorphCompanionFromMorphTo(scope, context.nextSlide, arrangement));
  }
  const morphToFrames = context.nextSlide !== undefined ? revealMorphToFrameByParticipant(context.nextSlide) : undefined;
  const morphFromMorphToFrames = context.previousSlide !== undefined ? revealMorphFromMorphToFrameByParticipant(context.previousSlide) : undefined;
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
//#endregion 🔖️RevealMorph

//#region 🔖️ArrangementSettled
/** @emoji 🔗️ True when two slide sections share the same reveal.js auto-animate run id. */
export function slidesShareAutoAnimateId(fromSlide: HTMLElement | null | undefined, toSlide: HTMLElement | null | undefined): boolean {
  const fromId = fromSlide?.getAttribute("data-auto-animate-id");
  const toId = toSlide?.getAttribute("data-auto-animate-id");
  return fromId !== null && fromId !== undefined && fromId === toId;
}

/** @emoji ⏳️ True while reveal.js auto-animate is pending or running on one section. */
export function isSectionAutoAnimating(sectionEl: HTMLElement): boolean {
  const state = sectionEl.getAttribute("data-auto-animate");
  return state === "pending" || state === "running";
}

/** @emoji ⏳️ True while reveal.js auto-animate is measuring or running on this slide (or any slide in the deck). */
export function isRevealSlideAutoAnimating(sectionEl: HTMLElement): boolean {
  if (isSectionAutoAnimating(sectionEl)) {
    return true;
  }
  const deck = sectionEl.closest(".reveal");
  if (!(deck instanceof HTMLElement)) {
    return false;
  }
  return deck.querySelector('section[data-auto-animate="pending"], section[data-auto-animate="running"]') !== null;
}

/** @emoji ⏳️ Clears settled state on arrival so split tiles stay visible at rest; drops settled on slides no longer adjacent. */
export function syncArrangementSettledState(deckEl: HTMLElement, currentSlide: HTMLElement | null, previousSlide: HTMLElement | null): void {
  const settleSections = deckEl.querySelectorAll<HTMLElement>("section[data-settle-before-morph-to]");
  for (const section of settleSections) {
    if (section !== currentSlide && section !== previousSlide) {
      section.classList.remove("presentation-arrangement--settled");
    }
  }
  if (currentSlide?.hasAttribute("data-settle-before-morph-to")) {
    currentSlide.classList.remove("presentation-arrangement--settled");
  }
  if (currentSlide?.classList.contains("presentation-arrangement--settled") && currentSlide.getAttribute("data-auto-animate") !== "pending" && currentSlide.getAttribute("data-auto-animate") !== "running") {
    currentSlide.classList.remove("presentation-arrangement--settled");
  }
}

/** @emoji 🔗️ Resolves the reveal.js slide section at stack indices `h` / `v`. */
export function resolveRevealSlideAt(deckEl: HTMLElement, indices: { readonly h: number; readonly v: number }): HTMLElement | null {
  const horizontal = deckEl.querySelectorAll<HTMLElement>(".slides > section")[indices.h];
  if (!horizontal) {
    return null;
  }
  const vertical = horizontal.querySelectorAll<HTMLElement>("section");
  return vertical[indices.v] ?? horizontal;
}

/** @emoji 📖️ Reads reveal.js `slidechanged` slide elements extended onto the event object. */
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

/** @emoji 🧹️ Strips reveal.js FLIP `transform`/`transition` only; never `left`/`top`/`width`/`height` (React owns those on morph frames). */
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

/** @emoji ✅️ Clears reveal `pending`/`running` on slides so morph-source/into rest CSS applies after FLIP completes. */
export function finalizeRevealAutoAnimateRestState(deckEl: HTMLElement): void {
  for (const slide of deckEl.querySelectorAll<HTMLElement>('section[data-auto-animate="running"], section[data-auto-animate="pending"]')) {
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
  if (presentSlide?.classList.contains("presentation-arrangement--settled") && presentSlide.getAttribute("data-auto-animate") !== "pending" && presentSlide.getAttribute("data-auto-animate") !== "running") {
    presentSlide.classList.remove("presentation-arrangement--settled");
  }
}

/** @emoji 🔀️ Slide class while auto-animating a `data-settle-before-morph-to` many-to-one run (focus tiles → label ghosts). */
export const PRESENTATION_MANY_TO_ONE_MORPH_CLASS = "presentation-arrangement--many-to-one-morph";

/** @emoji 🔀️ True when `fromSlide` settles into `toSlide` via `data-settle-before-morph-to` (arrangement id on `title`). */
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

/** @emoji 🧹️ Clears {@link PRESENTATION_MANY_TO_ONE_MORPH_CLASS} from all slides in the deck. */
export function clearManyToOneMorphArrangementClass(deckEl: HTMLElement): void {
  for (const slide of deckEl.querySelectorAll<HTMLElement>(`section.${PRESENTATION_MANY_TO_ONE_MORPH_CLASS}`)) {
    slide.classList.remove(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
  }
}

/** @emoji 🔀️ Marks the active many-to-one morph run on `fromSlide` and `toSlide` only. */
export function syncManyToOneMorphArrangementClass(deckEl: HTMLElement, fromSlide: HTMLElement | null, toSlide: HTMLElement | null): void {
  clearManyToOneMorphArrangementClass(deckEl);
  if (fromSlide && toSlide && isManyToOneMorphTransition(fromSlide, toSlide)) {
    fromSlide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
    toSlide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
  }
}

/** @emoji ⏳️ Prepares settle + many-to-one frame/crop morph before reveal auto-animate measures FLIP. */
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
  return element.classList.contains("presentation-target-ghost") || element.closest(".presentation-target-ghost") !== null;
}

function elementIsSourceGhostAnchor(element: HTMLElement): boolean {
  return element.classList.contains("presentation-source-ghost") || element.closest(".presentation-source-ghost") !== null;
}

function elementIsMorphOneAnchor(element: HTMLElement): boolean {
  return element.classList.contains("presentation-morph-one") || element.closest(".presentation-morph-one") !== null;
}

function elementIsLabelMorphSource(element: HTMLElement): boolean {
  return element.classList.contains("presentation-affiliation-morph-source");
}

/** @emoji 🎯️ True when this node may be a reveal.js auto-animate pair endpoint (intro wrapper or canvas-framed tile). */
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

/** @emoji 📐️ Slide-local ink box for reveal.js `measure` (avoids viewport `getBoundingClientRect` fly-in when `center: true`). */
export function revealInkMeasureForAutoAnimate(element: HTMLElement): {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
} {
  const slide = element.closest("section.presentation-arrangement--interactive, section[data-auto-animate]");
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
  return slide.classList.contains("presentation-arrangement--intro") && !slide.classList.contains("presentation-arrangement--positioned");
}

/** @emoji 🎯️ reveal.js auto-animate options for canvas morph text (translate only). */
export function revealTextAutoAnimatePairOptions(_from: HTMLElement, _fromSlide: HTMLElement, _toSlide: HTMLElement): Record<string, unknown> {
  return {
    scale: false,
    measure: revealInkMeasureForAutoAnimate,
  };
}

function elementIsFigureMorphSlot(element: HTMLElement): boolean {
  return element.classList.contains("presentation-morph-slot--figure") || element.closest(".presentation-morph-slot--figure") !== null || element.querySelector(".presentation-morph-slot--figure") !== null;
}

function elementIsInteractiveFigureDisposition(element: HTMLElement): boolean {
  return element.classList.contains("presentation-interactive-disposition") && elementIsFigureMorphSlot(element) && !elementIsSourceGhostAnchor(element) && !elementIsTargetGhostAnchor(element);
}

/** @emoji 🎯️ Picks the `to` morph anchor for one `data-id` (focus tile → label target ghost, catalogue source → focus tile). */
export function resolveMorphAutoAnimateTo(fromElement: HTMLElement, toSlide: HTMLElement): HTMLElement | null {
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
    const targetGhost = candidates.find((candidate) => candidate.classList.contains("presentation-interactive-disposition") && elementIsTargetGhostAnchor(candidate));
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
  return candidates.find((candidate) => !elementIsSourceGhostAnchor(candidate) || (elementIsTargetGhostAnchor(candidate) && candidate.classList.contains("presentation-interactive-disposition"))) ?? null;
}

/** @emoji 🔗️ reveal.js auto-animate matcher: intro uses stock `data-id` pairing; catalogue uses ghost-aware pairing. */
export function presentationAutoAnimateMatcher(this: AutoAnimateMatcherHost, fromSlide: HTMLElement, toSlide: HTMLElement): { from: HTMLElement; to: HTMLElement; options?: Record<string, unknown> }[] {
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
      const options = isTextLeaf ? revealTextAutoAnimatePairOptions(fromElement, fromSlide, toSlide) : undefined;
      pairs.push({ from: fromElement, to: toElement, ...(options ? { options } : {}) });
    }
  }
  const reserved: HTMLElement[] = [];
  return pairs.filter((pair) => {
    if (reserved.includes(pair.to)) {
      return false;
    }
    if (elementIsFigureMorphSlot(pair.from) && pair.to.closest(".presentation-morph-target") !== null && !elementIsTargetGhostAnchor(pair.to)) {
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

/** @emoji 📐️ Rewrites reveal FLIP `scale(sx, sy)` to `scale(max(sx,sy))` so figure tiles zoom instead of squashing during auto-animate. */
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

/** @emoji 👻️ Figure ghost visibility during reveal auto-animate (FLIP only; no intro text keyframes). */
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

/** @emoji 🩹️ Patches reveal auto-animate sheet: uniform scale for catalogue tiles only; intro text keeps native FLIP. */
export function patchPresentationAutoAnimateStyleSheet(sheet: { innerHTML: string }, durationSeconds: number, options?: { readonly manyToOneMorph?: boolean; readonly introFlowMorph?: boolean }): void {
  let css = sheet.innerHTML;
  if (options?.manyToOneMorph !== true && options?.introFlowMorph !== true) {
    css = patchAutoAnimateUniformScale(css);
  }
  sheet.innerHTML = css + presentationMorphGhostAutoAnimateCss(durationSeconds);
}

/** @emoji 🩹️ Applies reveal auto-animate sheet fixes for the current slide pair. */
export function patchPresentationAutoAnimateRunStyleSheet(sheet: { innerHTML: string }, durationSeconds: number, fromSlide: HTMLElement | undefined, toSlide: HTMLElement | undefined): void {
  if (fromSlide !== undefined && toSlide !== undefined && isIntroFlowSlide(fromSlide) && isIntroFlowSlide(toSlide)) {
    return;
  }
  patchPresentationAutoAnimateStyleSheet(sheet, durationSeconds, {
    manyToOneMorph: fromSlide !== undefined && toSlide !== undefined && isManyToOneMorphTransition(fromSlide, toSlide),
  });
}

export interface PresentationAutoAnimateRunSlides {
  readonly fromSlide?: HTMLElement;
  readonly toSlide?: HTMLElement;
}

/** @emoji 🔎️ Resolves an auto-animate slide pair when reveal.js omits it from the `autoanimate` event. */
export function resolvePresentationAutoAnimateRunSlides(explicit: PresentationAutoAnimateRunSlides, pending: PresentationAutoAnimateRunSlides | undefined): PresentationAutoAnimateRunSlides {
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
//#endregion 🔖️ArrangementSettled

//#region 🔖️HiddenPreflight
/**
 * @emoji 🩹️ Lets reveal.js own slide visibility by relaxing Tailwind preflight's `[hidden]` reset.
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
      if (typeof styleRule.selectorText === "string" && styleRule.selectorText.includes("[hidden]") && styleRule.style?.getPropertyValue("display") === "none") {
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
//#endregion 🔖️HiddenPreflight

//#region 🔖️MorphView
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

//#region 🔖️SlideEpoch
const PresentationSlideEpochContext = createContext(0);

/** @emoji 🔗️ When true, the interactive canvas wrapper owns `data-id` for reveal.js auto-animate. */
const MorphAnchorOnWrapperContext = createContext(false);

/** @emoji ⛶️ When true, {@link PdfMorphView} measures against the enlarged slide content box, not the declared catalogue frame. */
const PresentationDispositionEnlargeContext = createContext(false);

/** @emoji 📐️ Live slide frame for {@link FigureMorphView} cover math (drag/resize updates this). */
const PresentationFigureCropFrameContext = createContext<DispositionPosition | undefined>(undefined);

/** @emoji 📐️ Slide width÷height for mosaic windowed-cover (`undefined` → use {@link FigureEmbodiment.sourceAspect}). */
const PresentationSlideAspectContext = createContext<number | undefined>(undefined);

/** @emoji 🆔️ Interactive disposition id for ephemeral pdf page navigation in {@link PdfMorphView}. */
const PresentationInteractiveDispositionIdContext = createContext<string | undefined>(undefined);

export function parsePresentationSlideCssSize(revealEl: HTMLElement | null): { readonly width: number; readonly height: number } {
  const width = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-width") ?? "960");
  const height = Number.parseFloat(revealEl?.style.getPropertyValue("--presentation-slide-height") ?? "700");
  return {
    width: Number.isFinite(width) && width > 0 ? width : 960,
    height: Number.isFinite(height) && height > 0 ? height : 700,
  };
}

//#region 🔌️PdfCanvasPort
interface PdfJsModule {
  readonly GlobalWorkerOptions: { workerSrc: string };
  getDocument(source: string): PdfCanvasLoadingTask;
}

const pdfWorkerReady = ephemeralBox<Promise<void> | null>("s.plugins.animate.apps.presentation.renderer.react.component.tsx.pdfWorkerReady", null);

async function loadPdfJsModule(): Promise<PdfJsModule> {
  const module = (await import("pdfjs-dist")) as unknown as PdfJsModule;
  if (!pdfWorkerReady.current) {
    pdfWorkerReady.current = Promise.resolve().then(() => {
      module.GlobalWorkerOptions.workerSrc = new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url).toString();
    });
  }
  await pdfWorkerReady.current;
  return module;
}

const defaultPdfCanvasPort: PdfCanvasPort = {
  load(source) {
    let disposed = false;
    let externalTask: PdfCanvasLoadingTask | null = null;
    const promise = loadPdfJsModule().then(async (module) => {
      if (disposed) {
        throw new Error("PDF load cancelled");
      }
      externalTask = module.getDocument(source);
      if (disposed) {
        await externalTask.destroy();
        throw new Error("PDF load cancelled");
      }
      return externalTask.promise;
    });
    return {
      promise,
      destroy() {
        disposed = true;
        return externalTask?.destroy();
      },
    };
  },
};

const pdfCanvasPort = ephemeralBox<PdfCanvasPort>("s.plugins.animate.apps.presentation.renderer.react.component.tsx.pdfCanvasPort", defaultPdfCanvasPort);

/** @emoji 🧪️ Replaces the PDF canvas implementation through the owned port. */
export function setPdfCanvasPort(port: PdfCanvasPort): void {
  pdfCanvasPort.current = port;
}

/** @emoji 🧹️ Restores the production PDF.js canvas implementation. */
export function resetPdfCanvasPort(): void {
  pdfCanvasPort.current = defaultPdfCanvasPort;
}
//#endregion 🔌️PdfCanvasPort

/** @emoji 📐️ Uniform PDF canvas scale so the page covers the disposition frame without distortion. */
export function pdfCoverScale(containerWidth: number, containerHeight: number, pageWidth: number, pageHeight: number): number | null {
  if (containerWidth <= 0 || containerHeight <= 0 || pageWidth <= 0 || pageHeight <= 0) {
    return null;
  }
  return Math.max(containerWidth / pageWidth, containerHeight / pageHeight);
}

/** @emoji 📐️ Uniform PDF canvas scale for one-axis cover scroll (fit width or height, overflow the other). */
export function pdfScrollCoverScale(containerWidth: number, containerHeight: number, pageWidth: number, pageHeight: number): number | null {
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

/** @emoji 📑️ Ordered PDF page numbers for navigation; empty means all document pages. */
export function pdfEmbodimentPageList(embodiment: PdfEmbodiment): readonly number[] {
  return embodiment.pages ?? [];
}

/** @emoji 📄️ Starting page for a pdf embodiment (subset or single page). */
export function pdfEmbodimentInitialPage(embodiment: PdfEmbodiment): number {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 0) {
    const declared = embodiment.page ?? pages[0];
    return pages.includes(declared) ? declared : pages[0];
  }
  return embodiment.page ?? 1;
}

/** @emoji 🧭️ Whether pdf prev/next controls apply for this embodiment. */
export function pdfPageNavEnabled(embodiment: PdfEmbodiment, numPages: number | null): boolean {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 1) {
    return true;
  }
  return pages.length === 0 && numPages !== null && numPages > 1;
}

/** @emoji ◀️ True when pdf page nav can move to an earlier page. */
export function pdfCanGoToPreviousPage(currentPage: number, embodiment: PdfEmbodiment, numPages: number | null): boolean {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 0) {
    return pages.indexOf(currentPage) > 0;
  }
  return currentPage > 1;
}

/** @emoji ▶️ True when pdf page nav can move to a later page. */
export function pdfCanGoToNextPage(currentPage: number, embodiment: PdfEmbodiment, numPages: number | null): boolean {
  const pages = pdfEmbodimentPageList(embodiment);
  if (pages.length > 0) {
    const index = pages.indexOf(currentPage);
    return index >= 0 && index < pages.length - 1;
  }
  return numPages !== null && currentPage < numPages;
}

/** @emoji 📄️ Target page for prev/next within a subset or the full document. */
export function pdfAdjacentPage(currentPage: number, direction: "prev" | "next", embodiment: PdfEmbodiment, numPages: number | null): number {
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

/** @emoji 📐️ Measures the PDF canvas viewport from the disposition frame or enlarged slide content box. */
function usePdfContainerSize(anchorRef: RefObject<HTMLDivElement | null>, position: DispositionPosition | undefined, slideEpoch: number, enlarged: boolean): { readonly width?: number; readonly height?: number } {
  const [size, setSize] = useState<{ readonly width?: number; readonly height?: number }>({});
  useEffect(() => {
    const el = anchorRef.current;
    if (!el) {
      return;
    }
    const measureTarget = (): HTMLElement | null => {
      if (enlarged) {
        return el.closest(".presentation-interactive-disposition--enlarged")?.querySelector(".presentation-interactive-disposition__content") ?? null;
      }
      const frame = el.closest(".presentation-disposition-frame");
      const scrollViewport = frame?.querySelector<HTMLElement>(".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport");
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
      const width = frame ? Math.floor(slide.width * frame.width) : Math.floor(slide.width * 0.8);
      const height = frame ? Math.floor(slide.height * frame.height) : Math.floor(slide.height * 0.4);
      if (width > 0 && height > 0) {
        setSize({ width, height });
      }
    };
    measure();
    const observed = measureTarget();
    const observer = observed && typeof ResizeObserver !== "undefined" ? new ResizeObserver(measure) : null;
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
  }, [anchorRef, enlarged, position?.height, position?.width, position?.x, position?.y, slideEpoch]);
  return size;
}
//#endregion 🔖️SlideEpoch

function lineClass(morphId: string, embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string | undefined {
  return [morphTextClass(morphId), embodiment.fit ? "r-fit-text" : undefined, emphasisClass(emphasis)].filter(Boolean).join(" ") || undefined;
}

function centeredLineClass(morphId: string, embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string {
  return [lineClass(morphId, embodiment, emphasis), "text-center"].filter(Boolean).join(" ");
}

/** @emoji 🎯️ Renders {@link TextEmbodiment}; `data-id` on leaf text unless the intro wrapper owns the morph anchor. */
/** @emoji 🎯️ Stable reveal.js `data-id` for one text line; single-line blocks use the morph id, multi-line use `--index`. */
function textMorphAnchorId(anchorId: string, lineIndex: number, lineCount: number, _root: ReturnType<typeof resolveTextMorphRoot>): string {
  return lineCount === 1 ? anchorId : `${anchorId}--${lineIndex}`;
}

function morphLeafDataId(anchorOnWrapper: boolean, id: string): { readonly "data-id"?: string } {
  return anchorOnWrapper ? {} : { "data-id": id };
}

function TextMorphView({ morphId: anchorId, embodiment, emphasis, anchorOnWrapper = false }: { readonly morphId: string; readonly embodiment: TextEmbodiment; readonly emphasis: ParticipantEmphasis; readonly anchorOnWrapper?: boolean }): ReactNode {
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
            <h2 key={`${anchorId}--${lineIndex}`} {...morphLeafDataId(anchorOnWrapper, textMorphAnchorId(anchorId, lineIndex, lineCount, root))} className={centeredHeadingClass}>
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
        <div key={`${anchorId}-line-${lineIndex}`} className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center gap-x-[0.35em]">
          {line.map((person) => {
            const displayName = authorDisplayName(person, embodiment);
            return (
              <h4 key={person.name} {...morphLeafDataId(anchorOnWrapper, `${anchorId}--${person.name}`)} className={morphTextClass(anchorId, "m-0 shrink-0 text-center")}>
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

function affiliationLineLabel(entry: AffiliationEntry, part: "line" | "suffix"): string {
  if (part === "suffix" && entry.suffix) {
    return entry.suffix.name;
  }
  return affiliationLineName(entry);
}

function affiliationLineMuted(partEmphasis: ParticipantEmphasis | undefined): boolean {
  return partEmphasis === "muted";
}

function affiliationLineContent(entry: AffiliationEntry, part: "line" | "suffix", label: string): ReactNode {
  const muted = part === "suffix" ? affiliationLineMuted(entry.suffixEmphasis) : affiliationLineMuted(entry.lineEmphasis);
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

function AffiliationsMorphView({ morphId: anchorId, embodiment, anchorOnWrapper = false }: { readonly morphId: string; readonly embodiment: AffiliationsEmbodiment; readonly anchorOnWrapper?: boolean }): ReactNode {
  const rowClass = morphTextClass(anchorId, "presentation-affiliation-row m-0 inline-flex max-w-full shrink-0 flex-row flex-nowrap items-center justify-center gap-x-[0.35em] text-center");
  return (
    <div className="presentation-intro-rows presentation-intro-affiliations flex w-full max-w-full flex-col items-center text-center">
      {embodiment.entries.map((entry) => {
        const displayLabel = affiliationLineLabel(entry, "line");
        return (
          <div key={entry.mark} className="presentation-intro-line flex w-full flex-row flex-wrap items-center justify-center gap-x-[0.35em]">
            <h4 {...morphLeafDataId(anchorOnWrapper, `${anchorId}--${entry.mark}`)} className={rowClass}>
              {affiliationLineContent(entry, "line", displayLabel)}
              {entry.suffix ? (
                <span {...morphLeafDataId(anchorOnWrapper, `${anchorId}--${entry.suffix.mark}`)} className="inline-flex shrink-0 items-center justify-center text-center">
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

function BulletMorphView({ morphId: anchorId, embodiment, emphasis }: { readonly morphId: string; readonly embodiment: BulletEmbodiment; readonly emphasis: ParticipantEmphasis }): ReactNode {
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

/** @emoji 🔗️ Resolves deck-relative figure paths against the Vite base URL. */
export function resolvePresentationAssetUrl(src: string): string {
  if (/^(?:[a-z]+:)?\/\//i.test(src) || src.startsWith("data:") || src.startsWith("blob:")) {
    return src;
  }
  const trimmed = src.replace(/^\.\//, "");
  const base = import.meta.env.BASE_URL ?? "/";
  return `${base.endsWith("/") ? base : `${base}/`}${trimmed}`.replace(/\/{2,}/g, "/");
}

/** @emoji 📐️ Physical aspect of a normalized crop (width÷height in source pixels). */
export function figureCropPhysicalAspect(crop: DispositionPosition, sourceAspect = 1): number {
  return (crop.width / crop.height) * sourceAspect;
}

const FIGURE_MOSAIC_ALIGN_EPSILON = 1e-4;

/** @emoji 🧩️ Grid column/row for a split crop, or null when the crop is not a mosaic cell. */
export function figureMosaicCellIndex(crop: DispositionPosition, mosaic: { readonly rows: number; readonly columns: number; readonly frame?: DispositionPosition }): { readonly column: number; readonly row: number } | null {
  const { rows, columns } = mosaic;
  if (rows < 1 || columns < 1) {
    return null;
  }
  const mosaicFrame = mosaic.frame ?? { x: 0, y: 0, width: 1, height: 1 };
  const colWidth = mosaicFrame.width / columns;
  const rowHeight = mosaicFrame.height / rows;
  if (Math.abs(crop.width - colWidth) > FIGURE_MOSAIC_ALIGN_EPSILON || Math.abs(crop.height - rowHeight) > FIGURE_MOSAIC_ALIGN_EPSILON) {
    return null;
  }
  const column = Math.round((crop.x - mosaicFrame.x) / colWidth);
  const row = Math.round((crop.y - mosaicFrame.y) / rowHeight);
  if (column < 0 || column >= columns || row < 0 || row >= rows) {
    return null;
  }
  if (Math.abs(crop.x - (mosaicFrame.x + column * colWidth)) > FIGURE_MOSAIC_ALIGN_EPSILON || Math.abs(crop.y - (mosaicFrame.y + row * rowHeight)) > FIGURE_MOSAIC_ALIGN_EPSILON) {
    return null;
  }
  return { column, row };
}

/** @emoji 🧩️ Edge-aligned background-position for one cell in a rows×columns sprite grid. */
export function figureMosaicBackgroundPosition(column: number, row: number, columns: number, rows: number): { readonly posX: number; readonly posY: number } {
  return {
    posX: columns <= 1 ? 50 : (column / (columns - 1)) * 100,
    posY: rows <= 1 ? 50 : (row / (rows - 1)) * 100,
  };
}

/** @emoji 📐️ Background-position along one axis when cover overflows (k≥1; k=1 → edge-aligned i/(n−1)). */
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

/** @emoji 🪟️ One mosaic cell as a window onto a single cover render of `frame` (no per-crop sprite zoom). */
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

/** @emoji 🖼️ True when the crop is the full source bitmap. */
function figureCropIsFullImage(crop: DispositionPosition): boolean {
  return crop.width >= 1 - FIGURE_MOSAIC_ALIGN_EPSILON && crop.height >= 1 - FIGURE_MOSAIC_ALIGN_EPSILON;
}

/** @emoji 🖼️ Positions a normalized source crop when background width is `(100/crop.width)%` (uniform, no distortion). */
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

/** @emoji 🖼️ Uniform background-size: `cover` for full image, else `N% auto` / `auto N%` zoomed to the crop (never dual-axis `%`). */
export function figureCropBackgroundSize(crop: DispositionPosition, frame: DispositionPosition, sourceAspect = 1): string {
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

/** @emoji 🖼️ Centered crop cover in a slide frame (non-mosaic / focus morph). */
function figureCropCoverVars(crop: DispositionPosition, frame: DispositionPosition, sourceAspect = 1): { readonly size: string; readonly posX: number; readonly posY: number } {
  const { posX, posY } = figureCropBackgroundPosition(crop);
  return {
    size: figureCropBackgroundSize(crop, frame, sourceAspect),
    posX,
    posY,
  };
}

/** @emoji 🪟️ Mosaic cell background vars (windowed cover of `mosaic.frame`). */
function figureMosaicCellCoverVars(crop: DispositionPosition, mosaic: FigureMosaicGrid, sourceAspect: number, slideAspect?: number): { readonly size: string; readonly posX: number; readonly posY: number } | null {
  const cell = figureMosaicCellIndex(crop, mosaic);
  if (!cell) {
    return null;
  }
  const mosaicFrame = mosaic.frame ?? { x: 0, y: 0, width: 1, height: 1 };
  return mosaicWindowedCoverVars(cell, mosaic, mosaicFrame, sourceAspect, slideAspect);
}

//#region 🔖️FigureScroll
/** @emoji 📐️ Default slide pixel size from {@link PresentationSlideAspectContext} (960×700 when unknown). */
function presentationSlidePixelSize(slideAspect?: number): { readonly width: number; readonly height: number } {
  const height = 700;
  return {
    width: slideAspect !== undefined ? slideAspect * height : 960,
    height,
  };
}
/** @emoji 📐️ Estimated frame pixels from normalized disposition (before DOM measure on hidden slides). */
export function estimateDispositionFramePixels(frame: DispositionPosition, slideWidth = 960, slideHeight = 700): { readonly width: number; readonly height: number } {
  return {
    width: frame.width * slideWidth,
    height: frame.height * slideHeight,
  };
}

/** @emoji 📜️ True when a figure should use one-axis cover scroll (mosaic tiles always clip). */
export function figureEmbodimentScrollEnabled(embodiment: FigureEmbodiment): boolean {
  if (embodiment.scroll === false || embodiment.mosaic !== undefined) {
    return false;
  }
  return true;
}

/** @emoji 📜️ True when a video should use one-axis cover scroll. */
export function videoEmbodimentScrollEnabled(embodiment: VideoEmbodiment): boolean {
  return embodiment.scroll !== false;
}

/** @emoji 📜️ True when a pdf page should use one-axis cover scroll. */
export function pdfEmbodimentScrollEnabled(embodiment: PdfEmbodiment): boolean {
  return embodiment.scroll !== false;
}

/** @emoji ✨️ True when a media embodiment should show the glassy teaser veil. */
export function mediaTeaserActive(teaser: MediaTeaser | undefined): teaser is MediaTeaser {
  return teaser !== undefined;
}

function MediaTeaserWrap({ teaser, children }: { readonly teaser?: MediaTeaser; readonly children: ReactNode }): ReactNode {
  if (!mediaTeaserActive(teaser)) {
    return children;
  }
  return (
    <div className="presentation-media-teaser">
      <div className="presentation-media-teaser__content">{children}</div>
      <div className="presentation-media-teaser__veil" aria-hidden={teaser.label === undefined} {...(teaser.label !== undefined ? { role: "img", "aria-label": teaser.label } : {})}>
        {teaser.label !== undefined ? <span className="presentation-media-teaser__label">{teaser.label}</span> : null}
      </div>
    </div>
  );
}

/** @emoji ↔ Which axis overflows under uniform cover (frame vs source aspect). */
export function figureCoverOverflowAxis(frameWidth: number, frameHeight: number, sourceAspect: number): "x" | "y" | null {
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

/** @emoji ↔ Scroll axis implied by a crop background-size (`N% auto` → x, `auto N%` → y). */
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

/** @emoji 📐️ Inner scroll content size for a crop background-size string. */
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

/** @emoji ↔ Scroll offset matching CSS background-position along one axis. */
export function figureScrollOffsetForBackgroundPosition(axis: "x" | "y", positionPercent: number, scrollSize: number, clientSize: number): number {
  const max = Math.max(0, scrollSize - clientSize);
  return (positionPercent / 100) * max;
}

/** @emoji 🖼️ Crop background-size on a scroll inner (span one scroll axis). */
export function figureCropScrollBackgroundSize(bgSize: string, scrollAxis?: "x" | "y" | null, zoom = 1): string {
  const axis = figureBackgroundSizeScrollAxis(bgSize) ?? scrollAxis ?? null;
  if (axis === "x") {
    return zoom > 1 ? `${zoom * 100}% auto` : "100% auto";
  }
  if (axis === "y") {
    return zoom > 1 ? `auto ${zoom * 100}%` : "auto 100%";
  }
  return figureBackgroundSizeZoomed(bgSize, zoom);
}

/** @emoji 🔍️ Minimum ctrl+wheel zoom (cover baseline). */
export const FIGURE_WHEEL_ZOOM_MIN = 1;

/** @emoji 🔍️ Maximum ctrl+wheel zoom multiplier. */
export const FIGURE_WHEEL_ZOOM_MAX = 8;

const FIGURE_WHEEL_ZOOM_FACTOR = 1.1;

/** @emoji 🔍️ Next zoom level from a wheel delta and current multiplier. */
export function figureWheelZoomStep(deltaY: number, currentZoom: number): number {
  const factor = deltaY < 0 ? FIGURE_WHEEL_ZOOM_FACTOR : 1 / FIGURE_WHEEL_ZOOM_FACTOR;
  return Math.min(FIGURE_WHEEL_ZOOM_MAX, Math.max(FIGURE_WHEEL_ZOOM_MIN, currentZoom * factor));
}

/** @emoji 🔍️ Keep the pointer anchor fixed while zooming scrollable figure content. */
export function figureWheelZoomAdjustScroll(scroller: HTMLElement, clientX: number, clientY: number, prevZoom: number, nextZoom: number): void {
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

/** @emoji 🔍️ Scale a crop `background-size` string by a zoom multiplier. */
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

/** @emoji 🔍️ Apply ctrl+wheel zoom to cover scroll content sizing. */
export function figureCoverScrollContentSize(portWidth: number, portHeight: number, sourceAspect: number, zoom = 1): { readonly axis: "x" | "y" | "both" | null; readonly style: CSSProperties } {
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

function figureCoverScrollContentSizeAtZoom(portWidth: number, portHeight: number, sourceAspect: number, _zoom: number): { readonly axis: "x" | "y" | null; readonly style: CSSProperties } {
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

/** @emoji 🔍️ Apply ctrl+wheel zoom to img/video cover element sizing. */
export function figureCoverScrollElementStyle(portWidth: number, portHeight: number, sourceAspect: number, zoom = 1): CSSProperties {
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

/** @emoji 🔍️ Ctrl+wheel zoom multiplier for the enclosing {@link FigureScrollViewport}. */
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

/** @emoji 📏️ Overlay scrollbar thumb size and offset from native scroll metrics. */
export function figureScrollOverlayThumbMetrics(clientSize: number, scrollSize: number, scrollPos: number, minThumb = 24): { readonly thumbSize: number; readonly thumbOffset: number; readonly visible: boolean } {
  if (clientSize <= 0 || scrollSize <= clientSize + 1) {
    return { thumbSize: 0, thumbOffset: 0, visible: false };
  }
  const thumbSize = Math.max(minThumb, (clientSize / scrollSize) * clientSize);
  const maxThumbOffset = Math.max(0, clientSize - thumbSize);
  const maxScroll = scrollSize - clientSize;
  const thumbOffset = maxThumbOffset <= 0 || maxScroll <= 0 ? 0 : (scrollPos / maxScroll) * maxThumbOffset;
  return { thumbSize, thumbOffset, visible: true };
}

function observeFigureScrollScrollerLayout(scroller: HTMLElement, onChange: () => void): () => void {
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

function observeFigureScrollScroller(scroller: HTMLElement, onChange: () => void): () => void {
  const stopLayout = observeFigureScrollScrollerLayout(scroller, onChange);
  scroller.addEventListener("scroll", onChange, { passive: true });
  return () => {
    scroller.removeEventListener("scroll", onChange);
    stopLayout();
  };
}

function useFigureScrollOverlayBar(scrollerRef: RefObject<HTMLElement | null>, barRef: RefObject<HTMLDivElement | null>, thumbRef: RefObject<HTMLDivElement | null>, axis: "x" | "y" | null, enabled: boolean): void {
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

function useFigureScrollViewportSync(viewportRef: RefObject<HTMLElement | null>, axis: "x" | "y" | null, scrollOrigin: { readonly x: number; readonly y: number }, enabled: boolean): void {
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
        viewport.scrollLeft = figureScrollOffsetForBackgroundPosition("x", positionPercent, scrollSize, clientSize);
      } else {
        viewport.scrollTop = figureScrollOffsetForBackgroundPosition("y", positionPercent, scrollSize, clientSize);
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
    figureWheelZoomAdjustScroll(scroller, pending.clientX, pending.clientY, pending.prevZoom, pending.nextZoom);
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
      const delta = scrollAxis === "x" ? (Math.abs(event.deltaX) > Math.abs(event.deltaY) ? event.deltaX : event.deltaY) : event.deltaY;
      if (delta === 0) {
        return;
      }
      const max = scrollAxis === "x" ? viewport.scrollWidth - viewport.clientWidth : viewport.scrollHeight - viewport.clientHeight;
      if (max <= 0) {
        return;
      }
      const next = scrollAxis === "x" ? viewport.scrollLeft + delta : viewport.scrollTop + delta;
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
      const metrics = figureScrollOverlayThumbMetrics(clientSize, scrollSize, overlayAxis === "y" ? scroller.scrollTop : scroller.scrollLeft);
      const maxThumbOffset = Math.max(0, clientSize - metrics.thumbSize);
      if (maxThumbOffset <= 0 || maxScroll <= 0) {
        return;
      }
      const pointer = overlayAxis === "y" ? event.clientY : event.clientX;
      const delta = pointer - drag.startPointer;
      const nextScroll = Math.max(0, Math.min(maxScroll, drag.startScroll + (delta / maxThumbOffset) * maxScroll));
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
      <div className={[figureScrollViewportClass(scrollAxis), "presentation-figure-scroll-viewport--overlay", className].filter(Boolean).join(" ")} style={style}>
        <div ref={scrollerRef} className={figureScrollScrollerClass(scrollAxis)} onWheel={onWheel}>
          {children}
        </div>
        {overlayAxis !== null ? (
          <div ref={barRef} className={`presentation-figure-scroll-bar presentation-figure-scroll-bar--axis-${overlayAxis}`} style={{ display: "none" }} aria-hidden="true">
            <div ref={thumbRef} className="presentation-figure-scroll-bar-thumb" onPointerDown={onThumbPointerDown} onPointerMove={onThumbPointerMove} onPointerUp={onThumbPointerUp} onPointerCancel={onThumbPointerUp} />
          </div>
        ) : null}
      </div>
    </FigureZoomContext.Provider>
  );
}
//#endregion 🔖️FigureScroll

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
  const mergedClassName = [className, scrollEnabled || zoom > 1 ? "presentation-figure-scroll-media" : undefined].filter(Boolean).join(" ");
  return <img className={mergedClassName} style={elementStyle} src={src} alt={alt} onLoad={onLoad} />;
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
  const mergedClassName = [className, scrollEnabled || zoom > 1 ? "presentation-figure-scroll-media" : undefined].filter(Boolean).join(" ");
  return <video className={mergedClassName} style={elementStyle} src={src} poster={poster} autoPlay={false} loop={loop ?? false} muted={muted ?? true} controls={controls ?? true} playsInline preload="metadata" onLoadedMetadata={onLoadedMetadata} />;
}

function FigureZoomedPdfPage({
  source,
  currentPage,
  coverScale,
  ready,
  ariaLabel,
  onDocumentLoadSuccess,
  onPageLoadSuccess,
}: {
  readonly source: string;
  readonly currentPage: number;
  readonly coverScale: number | null;
  readonly ready: boolean;
  readonly ariaLabel: string;
  readonly onDocumentLoadSuccess: (document: { readonly numPages: number }) => void;
  readonly onPageLoadSuccess: (page: PdfCanvasPage) => void;
}): ReactNode {
  const zoom = useFigureZoom();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const resourceOwnerRef = useRef<PdfCanvasResourceOwner | null>(null);
  resourceOwnerRef.current ??= new PdfCanvasResourceOwner();
  const [documentState, setDocumentState] = useState<{
    readonly source: string;
    readonly document: PdfCanvasDocument;
  } | null>(null);
  const [status, setStatus] = useState<PdfCanvasStatus>("loading");
  useEffect(() => {
    let active = true;
    const owner = resourceOwnerRef.current!;
    setDocumentState(null);
    setStatus("loading");
    const loadingTask = pdfCanvasPort.current.load(resolvePresentationAssetUrl(source));
    owner.beginLoad(loadingTask);
    void loadingTask.promise.then(
      (document) => {
        if (!active || !owner.acceptDocument(loadingTask, document)) {
          void document.destroy();
          return;
        }
        setDocumentState({ source, document });
        onDocumentLoadSuccess({ numPages: document.numPages });
      },
      () => {
        if (active) {
          setStatus("error");
        }
      },
    );
    return () => {
      active = false;
      owner.disposeDocument();
    };
  }, [onDocumentLoadSuccess, source]);
  useEffect(() => {
    const document = documentState?.source === source ? documentState.document : null;
    if (!document || !ready) {
      return;
    }
    let active = true;
    const owner = resourceOwnerRef.current!;
    owner.beginPage();
    setStatus("loading");
    void document
      .getPage(currentPage)
      .then(async (page) => {
        if (!active) {
          page.cleanup();
          return;
        }
        owner.acceptPage(page);
        onPageLoadSuccess(page);
        const canvas = canvasRef.current;
        const context = canvas?.getContext("2d") ?? null;
        if (!canvas || !context) {
          throw new Error("PDF canvas 2d context unavailable");
        }
        const scale = (coverScale ?? 1) * zoom;
        const pixelRatio = Math.max(1, globalThis.devicePixelRatio || 1);
        const viewport = page.getViewport({ scale });
        const renderViewport = page.getViewport({ scale: scale * pixelRatio });
        const bitmap = pdfCanvasBitmapSize(viewport, pixelRatio);
        canvas.width = bitmap.width;
        canvas.height = bitmap.height;
        canvas.style.width = `${viewport.width}px`;
        canvas.style.height = `${viewport.height}px`;
        canvas.dataset.page = String(currentPage);
        canvas.dataset.scale = String(scale);
        const renderTask = page.render({ canvas, canvasContext: context, viewport: renderViewport });
        owner.acceptRender(renderTask);
        await renderTask.promise;
        if (active) {
          setStatus("ready");
        }
      })
      .catch(() => {
        if (active) {
          setStatus("error");
        }
      });
    return () => {
      active = false;
      owner.disposePage();
    };
  }, [coverScale, currentPage, documentState, onPageLoadSuccess, ready, source, zoom]);
  const announcement = pdfCanvasStatusAnnouncement(status);
  return (
    <div className="presentation-media-pdf" data-pdf-status={status}>
      {announcement ? (
        <span className={`presentation-media-pdf-${status}`} role={announcement.role} aria-live={announcement.role === "status" ? "polite" : undefined}>
          {announcement.text}
        </span>
      ) : null}
      <canvas ref={canvasRef} role="img" aria-label={ariaLabel} style={{ visibility: status === "ready" ? "visible" : "hidden" }} />
    </div>
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
    ...(backgroundSize === "cover" ? figureCoverScrollContentSize(portWidth, portHeight, sourceAspect, zoom).style : figureCropScrollContentSize(figureBackgroundSizeZoomed(backgroundSize, zoom))),
    backgroundImage: backgroundVars.backgroundImage,
    ["--presentation-figure-bg-size" as string]: figureCropScrollBackgroundSize(backgroundSize, figureBackgroundSizeScrollAxis(backgroundSize) ?? figureCoverOverflowAxis(portWidth, portHeight, sourceAspect), zoom),
    ["--presentation-figure-bg-position" as string]: backgroundPosition,
    ["--presentation-figure-bg-size-morph" as string]: backgroundVars["--presentation-figure-bg-size-morph" as keyof typeof backgroundVars],
    ["--presentation-figure-bg-position-morph" as string]: backgroundVars["--presentation-figure-bg-position-morph" as keyof typeof backgroundVars],
    ["--presentation-figure-bg-grid-size" as string]: backgroundVars["--presentation-figure-bg-grid-size" as keyof typeof backgroundVars],
    ["--presentation-figure-bg-grid-position" as string]: backgroundVars["--presentation-figure-bg-grid-position" as keyof typeof backgroundVars],
  };
  return <div className={className} style={scrollContentStyle} {...(morphCropData !== undefined ? { "data-presentation-morph-crop": morphCropData } : {})} role="img" aria-label={ariaLabel} />;
}

/** @emoji 📐️ Reads `left`/`top`/`width`/`height` percent inline styles as a normalized disposition frame. */
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

/** @emoji 🔀️ Copies live focus-tile frames from slide 8 onto label-slide target ghosts before many-to-one FLIP. */
export function syncManyToOneGhostMorphFramesFromDom(fromSlide: HTMLElement, toSlide: HTMLElement): void {
  if (!isManyToOneMorphTransition(fromSlide, toSlide)) {
    return;
  }
  for (const ghost of toSlide.querySelectorAll<HTMLElement>(".presentation-target-ghost.presentation-interactive-disposition--canvas-framed")) {
    const morphId = ghost.getAttribute("data-id");
    if (!morphId) {
      continue;
    }
    const escapedId = morphId.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
    const sourceEl = fromSlide.querySelector<HTMLElement>(`.presentation-interactive-disposition--canvas-framed[data-id="${escapedId}"]`);
    if (!sourceEl) {
      continue;
    }
    const sourceFrame = measureElementRectInSection(sourceEl, fromSlide) ?? readPercentDispositionFrame(sourceEl);
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
        applyFigureCropCssVars(slot, figureCropBackgroundVarsTargetGhost({ kind: "figure", src: "", crop }, crop, sourceFrame, labelFrame));
      } catch {
        // skip invalid crop payload
      }
    }
    void ghost.offsetHeight;
  }
}

/** @emoji 📐️ Custom properties for morphing a canvas frame from `from` into `to` during reveal auto-animate. */
export function morphFrameCssVars(from: DispositionPosition, to: DispositionPosition): CSSProperties {
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

/** @emoji 📐️ Writes {@link morphFrameCssVars} onto an element via `style.setProperty` (required for CSS variables). */
export function applyMorphFrameCssVars(element: HTMLElement, from: DispositionPosition, to: DispositionPosition): void {
  for (const [key, value] of Object.entries(morphFrameCssVars(from, to))) {
    if (typeof value === "string") {
      element.style.setProperty(key, value);
    }
  }
}

/** @emoji 🖼️ Applies figure crop CSS variables from a vars object via `setProperty`. */
export function applyFigureCropCssVars(element: HTMLElement, vars: CSSProperties): void {
  for (const [key, value] of Object.entries(vars)) {
    if (typeof value === "string" && key.startsWith("--presentation-figure-bg-")) {
      element.style.setProperty(key, value);
    }
  }
}

/** @emoji 🖼️ CSS vars for crop tiles: mosaic windowed cover at rest/grid; centered crop cover for focus morph. */
export function figureCropBackgroundVars(embodiment: FigureEmbodiment, crop: DispositionPosition, frame?: DispositionPosition, morphToFrame?: DispositionPosition, fromMorphToFrame?: DispositionPosition, slideAspect?: number): CSSProperties {
  const mosaic = embodiment.mosaic;
  const sourceAspect = embodiment.sourceAspect ?? 1;
  const mosaicRest = mosaic ? figureMosaicCellCoverVars(crop, mosaic, sourceAspect, slideAspect) : null;
  const restBasis = frame ?? morphToFrame;
  const rest = mosaicRest ? mosaicRest : restBasis ? figureCropCoverVars(crop, restBasis, sourceAspect) : figureCropCoverVars(crop, { x: 0, y: 0, width: 1, height: 1 }, sourceAspect);
  const morphBasis = morphToFrame ?? frame;
  const morph = morphBasis ? figureCropCoverVars(crop, morphBasis, sourceAspect) : rest;
  const gridFrom = fromMorphToFrame
    ? mosaic
      ? (figureMosaicCellCoverVars(crop, mosaic, sourceAspect, slideAspect) ?? figureCropCoverVars(crop, fromMorphToFrame, sourceAspect))
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

/** @emoji 👻️ Target ghost crop vars: `--presentation-figure-bg-size` = source tile frame, `-morph` = label slot (many-to-one 8→9). */
export function figureCropBackgroundVarsTargetGhost(embodiment: FigureEmbodiment, crop: DispositionPosition, sourceFrame: DispositionPosition, labelFrame: DispositionPosition): CSSProperties {
  return figureCropBackgroundVars(embodiment, crop, sourceFrame, labelFrame);
}

function FigureImageMorphView({ morphId: anchorId, embodiment, emphasis, position }: { readonly morphId: string; readonly embodiment: FigureEmbodiment; readonly emphasis: ParticipantEmphasis; readonly position?: DispositionPosition }): ReactNode {
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
      const viewport = anchor.querySelector<HTMLElement>(".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport");
      const target = viewport ?? anchor;
      setPortSize({ width: target.clientWidth, height: target.clientHeight });
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(anchor);
    return () => observer.disconnect();
  }, [slideEpoch]);
  const estimatedPortSize = useMemo(() => {
    const slide = presentationSlidePixelSize(slideAspect);
    return position ? estimateDispositionFramePixels(position, slide.width, slide.height) : estimateDispositionFramePixels({ x: 0, y: 0, width: 1, height: 1 });
  }, [position, slideAspect]);
  const effectivePortSize = portSize.width > 0 ? portSize : estimatedPortSize;
  const coverScroll = figureCoverScrollContentSize(effectivePortSize.width, effectivePortSize.height, sourceAspect);
  const axis = coverScroll.axis;
  const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
  return (
    <div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
      <MediaTeaserWrap teaser={embodiment.teaser}>
        <FigureScrollViewport enabled={scrollEnabled} axis={axis} scrollOrigin={scrollOrigin} slideEpoch={slideEpoch} style={{ width: "100%", height: "100%" }}>
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
              if (embodiment.sourceAspect === undefined && image.naturalWidth > 0 && image.naturalHeight > 0) {
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
  const morphCropFrom = revealMorphCompanion === "target" && morphFrame !== undefined && position !== undefined;
  const morphCropTo = position !== undefined && (fromMorphToFrame !== undefined || morphToFrame !== undefined);
  const backgroundVars =
    morphCropFrom && morphFrame ? figureCropBackgroundVarsTargetGhost(embodiment, embodiment.crop!, morphFrame, position) : figureCropBackgroundVars(embodiment, embodiment.crop!, cropFrame, morphToFrame, fromMorphToFrame, slideAspect);
  const backgroundSize = String(backgroundVars["--presentation-figure-bg-size" as keyof typeof backgroundVars] ?? "cover");
  const backgroundPosition = String(backgroundVars["--presentation-figure-bg-position" as keyof typeof backgroundVars] ?? "50% 50%");
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
  const scrollAxis = figureBackgroundSizeScrollAxis(backgroundSize) ?? figureCoverOverflowAxis(effectivePortSize.width, effectivePortSize.height, sourceAspect);
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
          {...(morphCropFrom && embodiment.crop ? { "data-presentation-morph-crop": JSON.stringify(embodiment.crop) } : {})}
          role="img"
          aria-label={embodiment.alt ?? ""}
        />
      </MediaTeaserWrap>
    );
  }
  return (
    <MediaTeaserWrap teaser={embodiment.teaser}>
      <div ref={frameRef} {...(anchorOnWrapper ? {} : { "data-id": anchorId })} className={["presentation-disposition-frame", "presentation-disposition-frame--figure-scroll"].join(" ")} style={frameStyle}>
        <FigureScrollViewport enabled axis={scrollAxis} scrollOrigin={scrollOrigin} slideEpoch={slideEpoch} style={{ width: "100%", height: "100%" }}>
          <FigureZoomedCropScrollContent
            className={[slotClassName, "presentation-figure-scroll-content"].join(" ")}
            portWidth={effectivePortSize.width}
            portHeight={effectivePortSize.height}
            sourceAspect={sourceAspect}
            backgroundSize={backgroundSize}
            backgroundPosition={scrollAxis === "x" ? `0% ${scrollOrigin.y}%` : scrollAxis === "y" ? `${scrollOrigin.x}% 0%` : backgroundPosition}
            backgroundVars={backgroundVars}
            ariaLabel={embodiment.alt ?? ""}
            morphCropData={morphCropFrom && embodiment.crop ? JSON.stringify(embodiment.crop) : undefined}
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

/** @emoji 🏷️ Positioned text morph slot: `data-id` on the frame so reveal.js can morph figure crops into labels. */
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
      <div className={["presentation-disposition-frame", "presentation-morph-slot", "presentation-morph-slot--label", "presentation-morph-target", emphasisClass(emphasis)].filter(Boolean).join(" ")} style={frameStyle}>
        <h2 className={headingClass}>{embodiment.lines[0]}</h2>
      </div>
    );
  }
  if (resolveTextMorphRoot(embodiment) === "heading-block") {
    return (
      <div className={["presentation-disposition-frame", "presentation-morph-slot", "presentation-morph-slot--label", emphasisClass(emphasis)].filter(Boolean).join(" ")} style={frameStyle}>
        <TextMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />
      </div>
    );
  }
  const headingClass = centeredLineClass(anchorId, embodiment, emphasis);
  return (
    <div {...(anchorOnWrapper ? {} : { "data-id": anchorId })} className={["presentation-disposition-frame", "presentation-morph-slot", "presentation-morph-slot--label", emphasisClass(emphasis)].filter(Boolean).join(" ")} style={frameStyle}>
      <h2 className={headingClass}>{embodiment.lines[0]}</h2>
    </div>
  );
}

function VideoMorphView({ morphId: anchorId, embodiment, emphasis, position }: { readonly morphId: string; readonly embodiment: VideoEmbodiment; readonly emphasis: ParticipantEmphasis; readonly position?: DispositionPosition }): ReactNode {
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
      const viewport = anchor.querySelector<HTMLElement>(".presentation-figure-scroll-scroller, .presentation-figure-scroll-viewport");
      const target = viewport ?? anchor;
      setPortSize({ width: target.clientWidth, height: target.clientHeight });
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(anchor);
    return () => observer.disconnect();
  }, [slideEpoch]);
  const estimatedPortSize = useMemo(() => {
    const slide = presentationSlidePixelSize(slideAspect);
    return position ? estimateDispositionFramePixels(position, slide.width, slide.height) : estimateDispositionFramePixels({ x: 0, y: 0, width: 1, height: 1 });
  }, [position, slideAspect]);
  const effectivePortSize = portSize.width > 0 ? portSize : estimatedPortSize;
  const coverScroll = figureCoverScrollContentSize(effectivePortSize.width, effectivePortSize.height, sourceAspect);
  const axis = coverScroll.axis;
  const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
  return (
    <div ref={anchorRef} data-id={anchorId} className={morphAnchorClass(emphasis)}>
      <MediaTeaserWrap teaser={embodiment.teaser}>
        <FigureScrollViewport enabled={scrollEnabled} axis={axis} scrollOrigin={scrollOrigin} slideEpoch={slideEpoch} style={{ width: "100%", height: "100%" }}>
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

function IframeMorphView({ morphId: anchorId, embodiment, emphasis }: { readonly morphId: string; readonly embodiment: IframeEmbodiment; readonly emphasis: ParticipantEmphasis }): ReactNode {
  return (
    <div data-id={anchorId} className={morphAnchorClass(emphasis)}>
      <MediaTeaserWrap teaser={embodiment.teaser}>
        <iframe className="presentation-media-iframe" src={resolvePresentationAssetUrl(embodiment.src)} title={embodiment.title ?? ""} loading="eager" />
      </MediaTeaserWrap>
    </div>
  );
}

function MarkdownMorphView({ morphId: anchorId, embodiment, emphasis }: { readonly morphId: string; readonly embodiment: MarkdownEmbodiment; readonly emphasis: ParticipantEmphasis }): ReactNode {
  const [html, setHtml] = useState("");
  useEffect(() => {
    let cancelled = false;
    void (async () => {
      const markdown = embodiment.markdown ?? (await fetch(resolvePresentationAssetUrl(embodiment.src)).then((response) => response.text()));
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
    <div data-id={anchorId} className={[morphAnchorClass(emphasis), "presentation-markdown-morph", "h-full w-full min-h-0 min-w-0"].filter(Boolean).join(" ")} aria-label={embodiment.title}>
      <Scrollable orientation="both" className="h-full w-full p-small">
        <div className="prose prose-sm max-w-none dark:prose-invert presentation-markdown-prose presentation-markdown-prose--top-left" dangerouslySetInnerHTML={{ __html: html }} />
      </Scrollable>
    </div>
  );
}

function JsonMorphView({ morphId: anchorId, embodiment, emphasis }: { readonly morphId: string; readonly embodiment: JsonEmbodiment; readonly emphasis: ParticipantEmphasis }): ReactNode {
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
    <div data-id={anchorId} className={[morphAnchorClass(emphasis), "presentation-json-morph", "h-full w-full min-h-0 min-w-0"].filter(Boolean).join(" ")} aria-label={embodiment.title}>
      <Scrollable orientation="both" className="h-full w-full p-small">
        {renderJsonTree(data)}
      </Scrollable>
    </div>
  );
}

function PdfMorphView({ morphId: anchorId, embodiment, emphasis, position }: { readonly morphId: string; readonly embodiment: PdfEmbodiment; readonly emphasis: ParticipantEmphasis; readonly position?: DispositionPosition }): ReactNode {
  const anchorRef = useRef<HTMLDivElement>(null);
  const slideEpoch = useContext(PresentationSlideEpochContext);
  const enlarged = useContext(PresentationDispositionEnlargeContext);
  const dispositionId = useContext(PresentationInteractiveDispositionIdContext);
  const interaction = useContext(PresentationInteractionContext);
  const declaredPage = pdfEmbodimentInitialPage(embodiment);
  const currentPage = dispositionId !== undefined && interaction !== null ? interaction.getPdfPage(dispositionId, declaredPage) : declaredPage;
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
  const onPageLoadSuccess = useCallback((page: PdfCanvasPage) => {
    const viewport = page.getViewport({ scale: 1 });
    setPageViewport({ width: viewport.width, height: viewport.height });
  }, []);
  const scrollEnabled = pdfEmbodimentScrollEnabled(embodiment);
  const slideAspect = useContext(PresentationSlideAspectContext);
  const estimatedContainer = useMemo(() => {
    const slide = presentationSlidePixelSize(slideAspect);
    return position ? estimateDispositionFramePixels(position, slide.width, slide.height) : { width: 960, height: 700 };
  }, [position, slideAspect]);
  const effectiveContainer = containerSize.width !== undefined && containerSize.height !== undefined && containerSize.width > 0 && containerSize.height > 0 ? { width: containerSize.width, height: containerSize.height } : estimatedContainer;
  const pageAspect = pageViewport !== null && pageViewport.height > 0 ? pageViewport.width / pageViewport.height : 1;
  const scrollAxis = scrollEnabled && pageViewport !== null ? figureCoverOverflowAxis(effectiveContainer.width, effectiveContainer.height, pageAspect) : null;
  const scrollOrigin = resolveMediaScrollOrigin(embodiment.scrollOrigin);
  const coverScale = useMemo(() => {
    if (pageViewport === null) {
      return null;
    }
    if (effectiveContainer.width <= 0 || effectiveContainer.height <= 0) {
      return null;
    }
    if (scrollEnabled) {
      const port = position !== undefined ? estimatedContainer : containerSize.width !== undefined && containerSize.height !== undefined && containerSize.width > 0 && containerSize.height > 0 ? effectiveContainer : estimatedContainer;
      return pdfScrollCoverScale(port.width, port.height, pageViewport.width, pageViewport.height);
    }
    if (containerSize.width === undefined || containerSize.height === undefined) {
      return null;
    }
    return pdfCoverScale(containerSize.width, containerSize.height, pageViewport.width, pageViewport.height);
  }, [containerSize.height, containerSize.width, effectiveContainer.height, effectiveContainer.width, estimatedContainer.height, estimatedContainer.width, pageViewport, position, scrollEnabled]);
  const ready = containerSize.width !== undefined && containerSize.height !== undefined && containerSize.width > 0 && containerSize.height > 0;
  const selected = dispositionId !== undefined && interaction !== null && interaction.isSelected(dispositionId);
  const showPageNav = (enlarged || selected) && dispositionId !== undefined && interaction !== null && pdfPageNavEnabled(embodiment, numPages);
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
        <FigureScrollViewport enabled={scrollEnabled} axis={scrollAxis} scrollOrigin={scrollOrigin} slideEpoch={slideEpoch} style={{ width: "100%", height: "100%" }}>
          <div className={["presentation-media-pdf-document", scrollEnabled ? "presentation-media-pdf-document--scroll" : undefined].filter(Boolean).join(" ")}>
            <FigureZoomedPdfPage source={embodiment.src} currentPage={currentPage} coverScale={coverScale} ready={ready} ariaLabel={`PDF page ${currentPage}`} onDocumentLoadSuccess={onDocumentLoadSuccess} onPageLoadSuccess={onPageLoadSuccess} />
          </div>
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

function dispositionFrameStyle(position: DispositionPosition | undefined, style: DispositionStyle | undefined): React.CSSProperties | undefined {
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

function DispositionFrame({ disposition, children, overlay }: { readonly disposition: RevealResolvedDisposition; readonly children: ReactNode; readonly overlay?: boolean }): ReactNode {
  const frameStyle = dispositionFrameStyle(disposition.position, disposition.style);
  if (!frameStyle) {
    return children;
  }
  return (
    <div className={["presentation-disposition-frame", overlay ? "presentation-disposition-frame--overlay" : undefined].filter(Boolean).join(" ")} style={frameStyle}>
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
      content = <TextMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} anchorOnWrapper={anchorOnWrapper} />;
      break;
    case "authors":
      content = <AuthorsMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} anchorOnWrapper={anchorOnWrapper} />;
      break;
    case "affiliations":
      content = <AffiliationsMorphView morphId={anchorId} embodiment={embodiment} anchorOnWrapper={anchorOnWrapper} />;
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
      content = <FigureMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={disposition.position} anchorOnWrapper={anchorOnWrapper} revealMorphCompanion={disposition.revealMorphCompanion} />;
      break;
    case "video":
      content = <VideoMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={disposition.position} />;
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
      content = <PdfMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} position={disposition.position} />;
      break;
    default: {
      const _exhaustive: never = embodiment;
      content = _exhaustive;
    }
  }
  const overlay = disposition.embodiment.kind === "figure" && disposition.position !== undefined && !disposition.embodiment.crop;
  return (
    <DispositionFrame disposition={disposition} overlay={overlay}>
      {content}
    </DispositionFrame>
  );
}
//#endregion 🔖️MorphView

//#region 🔖️Interaction
const DISPOSITION_MIN_FRACTION = 0.02;
const POINTER_DRAG_THRESHOLD_PX = 3;

/** @emoji 🗺️ Whether reveal.js is showing the slide grid (Escape overview). */
export function revealDeckInOverview(element: Element | null): boolean {
  return element?.closest(".reveal")?.classList.contains("overview") ?? false;
}

/** @emoji 🗺️ Swallow the post-gesture click reveal.js uses to leave overview (capture on `.reveal`). */
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

/** @emoji 🖱️ Disposition ids to move/resize for this gesture (before async `selectIds` commits). */
function resolveDispositionDragGroupIds(id: string, wasSelected: boolean, additive: boolean, selectedIds: ReadonlySet<string>): readonly string[] {
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

/** @emoji 📐️ Ephemeral slide-space rectangle for interactive dispositions (normalized 0..1). */
export type DispositionTransform = DispositionPosition;

/** @emoji ↔ Marquee selection mode: crossing (L→R partial overlap) vs window (R→L full containment). */
export type MarqueeSelectionRule = "crossing" | "window";

/** @emoji ⊡ Eight resize handles on a disposition frame. */
export type DispositionResizeHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

const DISPOSITION_RESIZE_HANDLES: readonly DispositionResizeHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];

/** @emoji 🔑️ Stable id for one resolved disposition on a slide. */
export function dispositionInteractionId(renderSlideId: string, disposition: RevealResolvedDisposition, index: number): string {
  return `${renderSlideId}--${disposition.morphId}--${disposition.embodimentId ?? index}`;
}

/** @emoji 🔑️ Stable id for one split tile on an interactive slide. */
export function tileDispositionInteractionId(renderSlideId: string, disposition: RevealResolvedDisposition, dispositionIndex: number, tileKey: string): string {
  return `${dispositionInteractionId(renderSlideId, disposition, dispositionIndex)}--tile--${tileKey}`;
}

/** @emoji 🔑️ Stable id for a visual row band grouping split tiles on one disposition. */
export function rowBandInteractionId(renderSlideId: string, disposition: RevealResolvedDisposition, dispositionIndex: number, rowIndex: number): string {
  return `${dispositionInteractionId(renderSlideId, disposition, dispositionIndex)}--row--${rowIndex}`;
}

/** @emoji 🖱️ One interactive placement (whole disposition or a single split tile). */
export interface InteractiveDispositionPlacement {
  readonly id: string;
  readonly disposition: RevealResolvedDisposition;
  /** @emoji 📐️ Wrapper frame (row-local for tiles inside a visual row). */
  readonly declaredRect: DispositionPosition | undefined;
  /** @emoji 📐️ Slide-space frame for marquee, drag, and resize. */
  readonly sectionRect: DispositionPosition | undefined;
  /** @emoji 🔗️ reveal.js `data-id` on the canvas wrapper when the tile frame must not own it. */
  readonly revealMorphId?: string;
  readonly rowBandId?: string;
}

/** @emoji 📏️ Row-level hit target spanning all tiles in one visual row of a split disposition. */
export interface InteractiveRowBandPlacement {
  readonly id: string;
  readonly frame: DispositionPosition;
  readonly tileIds: readonly string[];
}

/** @emoji 🖱️ Interactive placements and row bands for one slide arrangement. */
export interface InteractiveSlideLayout {
  readonly placements: readonly InteractiveDispositionPlacement[];
  readonly rowBands: readonly InteractiveRowBandPlacement[];
}

/** @emoji 🧩️ Builds one interactive placement per resolved disposition. */
export function buildInteractiveSlideLayout(renderSlideId: string, resolved: readonly RevealResolvedDisposition[], morph = false): InteractiveSlideLayout {
  const placements: InteractiveDispositionPlacement[] = [];
  resolved.forEach((disposition, dispositionIndex) => {
    const declaredRect = declaredDispositionRect(disposition);
    placements.push({
      id: dispositionInteractionId(renderSlideId, disposition, dispositionIndex),
      disposition,
      declaredRect,
      sectionRect: declaredRect,
      revealMorphId: morph && declaredRect !== undefined ? (disposition.revealMorphCompanion !== undefined ? disposition.morphId : disposition.morphFrom?.length ? undefined : disposition.morphId) : undefined,
    });
  });
  return { placements, rowBands: [] };
}

/** @emoji 📐️ True when two normalized rectangles overlap with positive area. */
export function rectsIntersect(a: DispositionPosition, b: DispositionPosition): boolean {
  return a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y;
}

/** @emoji 📐️ True when outer fully contains inner. */
export function rectContains(outer: DispositionPosition, inner: DispositionPosition): boolean {
  return inner.x >= outer.x && inner.y >= outer.y && inner.x + inner.width <= outer.x + outer.width && inner.y + inner.height <= outer.y + outer.height;
}

/** @emoji ⊞ Normalized marquee rectangle from two pointer fractions. */
export function normalizeMarquee(start: { readonly x: number; readonly y: number }, end: { readonly x: number; readonly y: number }): DispositionPosition {
  const x = Math.min(start.x, end.x);
  const y = Math.min(start.y, end.y);
  return {
    x,
    y,
    width: Math.abs(end.x - start.x),
    height: Math.abs(end.y - start.y),
  };
}

/** @emoji ↔ Crossing when dragged right-to-left (end.x < start.x), else window. */
export function marqueeSelectionRule(start: { readonly x: number; readonly y: number }, end: { readonly x: number; readonly y: number }): MarqueeSelectionRule {
  return end.x < start.x ? "crossing" : "window";
}

/** @emoji 🎯️ Whether a marquee selects a target rect under crossing or window rules. */
export function marqueeSelects(marquee: DispositionPosition, target: DispositionPosition, rule: MarqueeSelectionRule): boolean {
  if (marquee.width <= 0 || marquee.height <= 0) {
    return false;
  }
  return rule === "crossing" ? rectsIntersect(marquee, target) : rectContains(marquee, target);
}

function clampFraction(value: number): number {
  return Math.max(0, Math.min(1, value));
}

/** @emoji ↔ Moves a normalized rect by fractional deltas (unbounded; follows pointer across the slide). */
export function translateDispositionRect(rect: DispositionPosition, dx: number, dy: number): DispositionPosition {
  return {
    x: rect.x + dx,
    y: rect.y + dy,
    width: rect.width,
    height: rect.height,
  };
}

/** @emoji ⊡ Resizes a normalized rect from one handle by fractional deltas. */
export function resizeDispositionRect(rect: DispositionPosition, handle: DispositionResizeHandle, dx: number, dy: number, minSize: number = DISPOSITION_MIN_FRACTION): DispositionPosition {
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
export function scaleRectWithinGroup(rect: DispositionPosition, oldGroup: DispositionPosition, newGroup: DispositionPosition): DispositionPosition {
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

/** @emoji ⛶️ Centered near-slide frame for interactive enlarge (uniform across figure, video, pdf, tiles). */
export const SLIDE_INTERACTIVE_ENLARGE_FRAME: DispositionPosition = {
  x: 0.05,
  y: 0.075,
  width: 0.9,
  height: 0.85,
};

/** @emoji ⛶️ Toggles uniform enlarged slide frame vs stashed pre-enlarge rect. */
export function toggleEnlargeRect(current: DispositionPosition, stash: DispositionPosition | undefined): { readonly rect: DispositionPosition; readonly stash: DispositionPosition | undefined } {
  if (stash !== undefined) {
    return { rect: stash, stash: undefined };
  }
  return { rect: SLIDE_INTERACTIVE_ENLARGE_FRAME, stash: current };
}

/** @emoji 📐️ reveal.js nested slide section with usable layout height for pointer math. */
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

/** @emoji 📐️ Client/layout bounds for slide-space fractions when inner reveal sections report zero height. */
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

/** @emoji 🖼️ Arrangement canvas when dispositions use declared slide-space frames; otherwise the slide coordinate root. */
export function dispositionPlacementContainer(sectionEl: HTMLElement, canvasPlacement: boolean): HTMLElement {
  if (!canvasPlacement) {
    return slideCoordinateRoot(sectionEl);
  }
  const canvas = sectionEl.querySelector(".presentation-arrangement-canvas");
  return canvas instanceof HTMLElement ? canvas : sectionEl;
}

/** @emoji 🖼️ Ink-bearing node used to measure a positioned disposition on the arrangement canvas. */
export function dispositionFrameElement(root: HTMLElement): HTMLElement {
  return (root.querySelector(".presentation-disposition-frame, .presentation-morph-slot--figure, .presentation-morph-anchor") as HTMLElement | null) ?? root;
}

/** @emoji ⊡ Selection chrome style: flow slides use measured ink frame; canvas-framed slides use wrapper inset via CSS. */
export function interactiveDispositionChromeStyle(options: { readonly selected: boolean; readonly effectiveRect: DispositionPosition | undefined; readonly canvasFramed: boolean; readonly enlarged: boolean }): CSSProperties | undefined {
  const { selected, effectiveRect, canvasFramed, enlarged } = options;
  if (!selected || !effectiveRect || enlarged || canvasFramed) {
    return undefined;
  }
  return transformFrameStyle(effectiveRect);
}

/** @emoji 📍️ Maps client coordinates to normalized fractions inside a section element. */
export function clientToSectionFraction(sectionEl: HTMLElement, clientX: number, clientY: number, options?: { readonly clamp?: boolean }): { readonly x: number; readonly y: number } {
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

/** @emoji 📍️ Maps an element's client rect to normalized fractions inside a section. */
export function measureElementRectInSection(element: HTMLElement, sectionEl: HTMLElement): DispositionPosition | null {
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
  probe.style.cssText = "position:fixed;left:0;top:0;visibility:hidden;pointer-events:none;width:max-content;max-width:none;";
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

/** @emoji 📍️ Ink bounds for text nodes (block headings otherwise span the full slide width). */
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
  const useTight = node.matches("[data-id], h1, h2, h3, h4, p, .presentation-morph-text, li, .presentation-intro-line") && box.width >= sectionWidth * 0.85;
  return useTight ? (tightElementBoundsRect(node) ?? box) : box;
}

function clientRectToSectionFraction(rect: DOMRect, sectionBounds: DOMRect): DispositionPosition {
  return {
    x: (rect.left - sectionBounds.left) / sectionBounds.width,
    y: (rect.top - sectionBounds.top) / sectionBounds.height,
    width: rect.width / sectionBounds.width,
    height: rect.height / sectionBounds.height,
  };
}

/** @emoji 📍️ Union of morph content bounds in section space (avoids full-width flow wrappers). */
export function measureDispositionBoundsInSection(root: HTMLElement, sectionEl: HTMLElement): DispositionPosition | null {
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

/** @emoji 📍️ Ink bounds as fractions inside a container (selection chrome and fill use this space). */
export function measureDispositionBoundsInContainer(root: HTMLElement, containerEl: HTMLElement): DispositionPosition | null {
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

/** @emoji 📏️ True when measured fractions are large enough to drag or resize reliably. */
export function isUsableMeasuredRect(rect: DispositionPosition): boolean {
  return rect.width >= DISPOSITION_MIN_FRACTION && rect.height >= DISPOSITION_MIN_FRACTION;
}

/** @emoji 📐️ Declared placement for one resolved disposition (includes dormant morph anchors at opacity 0). */
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

/** @emoji 📐️ Cumulative visual scale for one element (client rect vs layout box, includes ancestor transforms). */
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

/** @emoji 📐️ reveal.js scales slides visually; map screen-pointer deltas to local translate pixels. */
export function sectionVisualScale(sectionEl: HTMLElement): number {
  return elementVisualScale(slideCoordinateRoot(sectionEl));
}

/** @emoji ↔ Node that receives flow `translate3d` during drag (offset wrapper, else content, else disposition root). */
export function flowDragTransformElement(sectionEl: HTMLElement, root: HTMLElement | null, content: HTMLElement | null): HTMLElement {
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

/** @emoji ↔ Pointer travel in screen px → local px for CSS translate on the flow drag target. */
export function flowPointerDeltaToLocal(transformEl: HTMLElement, startClientX: number, startClientY: number, currentClientX: number, currentClientY: number): { readonly dx: number; readonly dy: number } {
  const scale = elementVisualScale(transformEl);
  const safe = Number.isFinite(scale) && scale > 0 ? Math.min(4, Math.max(0.05, scale)) : 1;
  return {
    dx: (currentClientX - startClientX) / safe,
    dy: (currentClientY - startClientY) / safe,
  };
}

/** @emoji ↔ Flow-layout drag: local-pixel translate (x/y are px, not normalized). */
export function flowDispositionOffsetStyle(transform: DispositionPosition): CSSProperties {
  return {
    transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
  };
}

/** @emoji 📐️ True when two normalized disposition frames differ. */
export function dispositionPositionChanged(a: DispositionPosition, b: DispositionPosition): boolean {
  return Math.abs(a.x - b.x) > 1e-6 || Math.abs(a.y - b.y) > 1e-6 || Math.abs(a.width - b.width) > 1e-6 || Math.abs(a.height - b.height) > 1e-6;
}

const SLIDE_INTERACTION_RESET_PROXIMITY_PX = 72;

/** @emoji 📐️ True when any disposition on the slide has ephemeral layout (drag, resize, enlarge, or pdf page). */
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

/** @emoji 📐️ True when a disposition transform differs from its declared anchor. */
export function dispositionHasEphemeralLayout(transform: DispositionTransform | undefined, anchorRect: DispositionPosition | undefined, measuredNatural: DispositionPosition | undefined, flowLayout: boolean): boolean {
  if (!transform) {
    return false;
  }
  if (!flowLayout) {
    return anchorRect !== undefined && dispositionPositionChanged(anchorRect, transform);
  }
  if (measuredNatural) {
    return dispositionPositionChanged(flowDispositionManipulationRect(measuredNatural, undefined), transform);
  }
  return true;
}

/** @emoji 🎯️ Pointer is within the top-right hotspot where the slide reset control lives. */
export function pointerNearSlideResetHotspot(section: HTMLElement, clientX: number, clientY: number, proximityPx = SLIDE_INTERACTION_RESET_PROXIMITY_PX): boolean {
  const bounds = slideLayoutBounds(section);
  const reachX = Math.max(proximityPx, bounds.width * 0.12);
  const reachY = Math.max(proximityPx, bounds.height * 0.12);
  return clientX >= bounds.right - reachX && clientY >= bounds.top && clientY <= bounds.top + reachY;
}

/** @emoji 📐️ Flow drag: preserve the wrapper footprint while ink is absolutely positioned inside. */
export function flowDispositionReserveStyle(reservePx: { readonly width: number; readonly height: number }): CSSProperties {
  return {
    minWidth: reservePx.width,
    minHeight: reservePx.height,
  };
}

/** @emoji 📐️ Flow transforms store pointer deltas; positioned transforms store slide-space frames. */
export function flowDispositionManipulationRect(measured: DispositionPosition, existing: DispositionPosition | undefined): DispositionPosition {
  if (existing) {
    return existing;
  }
  return { x: 0, y: 0, width: measured.width, height: measured.height };
}

/** @emoji 📐️ True when x/y/width/height are a normalized slide-space frame (not pixel drag storage). */
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

/** @emoji ↔ True when a flow transform only stores pixel translate with unchanged measured size. */
export function isFlowPixelOffsetTransform(transform: DispositionPosition, measured: DispositionPosition | undefined): boolean {
  const sizeMatches = measured === undefined ? transform.width > 0 && transform.width <= 1 && transform.height > 0 && transform.height <= 1 : transform.width === measured.width && transform.height === measured.height;
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

/** @emoji 🔍️ Uniform scale for pinned resize so ink zooms inside the frame without reflow overflow. */
export function interactiveDispositionContentScale(transform: DispositionPosition, baseline: DispositionPosition | undefined): number | null {
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

/** @emoji 🔍️ CSS transform that scales disposition content from its center during interactive resize. */
export function interactiveDispositionContentScaleStyle(scale: number): CSSProperties {
  return {
    transform: `scale(${scale})`,
    transformOrigin: "center center",
    willChange: "transform",
  };
}

/** @emoji 📍️ Converts a flow pixel-offset transform into a slide-space frame using measured ink bounds. */
export function flowPixelOffsetToSectionRect(measured: DispositionPosition, transform: DispositionPosition, sectionEl: HTMLElement, transformEl?: HTMLElement): DispositionPosition {
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

/** @emoji 🖱️ Per-slide selection, transforms, and enlarge flags; cleared only via reset controls. */
export function usePresentationInteraction(): PresentationInteractionState {
  const [selectedIds, setSelectedIds] = useState<ReadonlySet<string>>(() => new Set());
  const [transforms, setTransforms] = useState<ReadonlyMap<string, DispositionTransform>>(() => new Map());
  const [enlargedIds, setEnlargedIds] = useState<ReadonlySet<string>>(() => new Set());
  const [enlargeStashById, setEnlargeStashById] = useState<ReadonlyMap<string, DispositionPosition>>(() => new Map());
  const [pdfPageById, setPdfPageById] = useState<ReadonlyMap<string, number>>(() => new Map());

  const isSelected = useCallback((id: string) => selectedIds.has(id), [selectedIds]);

  const isEnlarged = useCallback((id: string) => enlargedIds.has(id), [enlargedIds]);

  const hasPdfPageOverride = useCallback((id: string) => pdfPageById.has(id), [pdfPageById]);

  const getPdfPage = useCallback((id: string, defaultPage: number) => pdfPageById.get(id) ?? defaultPage, [pdfPageById]);

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
      if (existing && !dispositionPositionChanged(existing, rect)) {
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

function SlideDispositionRegistryProvider({ children }: { readonly children: ReactNode }): ReactNode {
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
    if (existing && Math.abs(existing.x - rect.x) < 1e-5 && Math.abs(existing.y - rect.y) < 1e-5 && Math.abs(existing.width - rect.width) < 1e-5 && Math.abs(existing.height - rect.height) < 1e-5) {
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

  return <SlideDispositionRegistryContext.Provider value={registry}>{children}</SlideDispositionRegistryContext.Provider>;
}

function resolveEffectiveDispositionRect(id: string, declared: DispositionPosition | undefined, interaction: PresentationInteractionState, registry: SlideDispositionRegistry): DispositionPosition | undefined {
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
      className={["presentation-interactive-row-band", rowSelected ? "presentation-interactive-row-band--selected" : undefined].filter(Boolean).join(" ")}
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
}> = ({ id, disposition, declaredRect, sectionDeclaredRect, allDeclaredRects, sectionRef, revealMorphId, rowBandId }) => {
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
  const flowPixelOffset = transformed && flowLayout && transform !== undefined && isFlowPixelOffsetTransform(transform, measuredNatural);
  const flowSectionFrame = transformed && flowLayout && !flowPixelOffset;
  const enlarged = interaction.isEnlarged(id);
  const morphGhost = isRevealMorphCompanionOnly(disposition);
  const canvasFramed = declaredRect !== undefined && !enlarged;
  const canvasPlacement = interactionRect !== undefined;
  const effectiveRect = resolveEffectiveDispositionRect(id, interactionRect, interaction, registry);
  const canvasAnchorRect = interactionRect;
  const canvasLiveTransform = canvasFramed && transform && canvasAnchorRect ? transform : undefined;
  const canvasDragActive = Boolean(canvasLiveTransform && canvasAnchorRect && !enlarged && dispositionPositionChanged(canvasAnchorRect, canvasLiveTransform));
  const pinned = !enlarged && ((transformed && !flowLayout && !canvasFramed) || flowSectionFrame || (canvasFramed && canvasDragActive && !flowPixelOffset));
  const [gesturing, setGesturing] = useState(false);
  const [flowReservePx, setFlowReservePx] = useState<{ readonly width: number; readonly height: number } | null>(null);
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
    const measured = canvasPlacement ? measureElementRectInSection(dispositionFrameElement(root), container) : measureDispositionBoundsInSection(root, section);
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
    setEnlargedFlowContentScale(interactiveDispositionContentScale(SLIDE_INTERACTIVE_ENLARGE_FRAME, baseline));
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
          return flowPixelOffsetToSectionRect(measuredNatural, existing, section, flowDragTransformElement(section, rootRef.current, contentRef.current));
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
            rect = memberDeclared !== undefined ? natural : flowDispositionManipulationRect(natural, undefined);
          } else {
            rect = memberDeclared;
          }
        } else if (flowLayout && natural) {
          rect = isFlowPixelOffsetTransform(rect, natural) ? rect : flowDispositionManipulationRect(natural, undefined);
        }
        if (rect && (flowLayout || memberDeclared !== undefined || isUsableMeasuredRect(rect))) {
          startRects.set(memberId, rect);
        }
      }
      if (startRects.size === 0) {
        startRects.set(id, initialRect);
      }
      const startGroup = mode !== "move" && groupIds.length > 1 ? groupBoundingRect([...startRects.values()]) : null;
      let dragging = !requireDragThreshold;
      setGesturing(true);
      const flowTransformEl = flowDragTransformElement(section, rootRef.current, contentRef.current);
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
          if (Math.hypot(moveEvent.clientX - startClient.x, moveEvent.clientY - startClient.y) < POINTER_DRAG_THRESHOLD_PX) {
            return;
          }
          dragging = true;
        }
        moveEvent.preventDefault();
        const updates = new Map<string, DispositionTransform>();
        if (mode === "move") {
          for (const [memberId, rect] of startRects) {
            if (flowLayout) {
              const { dx, dy } = flowPointerDeltaToLocal(flowTransformEl, startClient.x, startClient.y, moveEvent.clientX, moveEvent.clientY);
              updates.set(memberId, {
                x: rect.x + dx,
                y: rect.y + dy,
                width: rect.width,
                height: rect.height,
              });
            } else {
              const container = dispositionPlacementContainer(section, allDeclaredRects.get(memberId) !== undefined);
              const bounds = container === fractionContainer && gesturePlacementBounds.width > 0 ? gesturePlacementBounds : container.getBoundingClientRect();
              const dx = bounds.width > 0 ? (moveEvent.clientX - startClient.x) / bounds.width : 0;
              const dy = bounds.height > 0 ? (moveEvent.clientY - startClient.y) / bounds.height : 0;
              updates.set(memberId, translateDispositionRect(rect, dx, dy));
            }
          }
        } else {
          const bounds = gesturePlacementBounds.width > 0 ? gesturePlacementBounds : fractionContainer.getBoundingClientRect();
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
    const dragMemberIds = resolveDispositionDragGroupIds(id, selected, additive, interaction.selectedIds);
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
    const dragMemberIds = resolveDispositionDragGroupIds(id, selected, false, interaction.selectedIds);
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

  const morphCropFrom = disposition.revealMorphCompanion === "target" && disposition.revealMorphFromFrame !== undefined && disposition.position !== undefined;
  const morphCropTo = disposition.position !== undefined && (disposition.revealMorphFromMorphToFrame !== undefined || disposition.revealMorphToFrame !== undefined);
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
  const flowDragOffsetStyle = flowPixelOffset && transform ? flowDispositionOffsetStyle(transform) : undefined;
  if (flowDragOffsetStyle) {
    Object.assign(wrapperFrame, flowDragOffsetStyle);
  }
  if (canvasFramed && canvasAnchorRect) {
    // 🔀️ The wrapper owns the reveal `data-id` morph anchor; placing it on the live ephemeral
    // rect (drag/resize) makes reveal.js auto-animate capture the modified frame as the morph
    // `from`, so morphs start from the current disposition including ephemeral modifications.
    // 👻️ Target ghosts keep label layout; live source frames are synced from slide 8 DOM before morph.
    const morphAnchorRect = disposition.revealMorphCompanion === "target" ? canvasAnchorRect : disposition.revealMorphCompanion === "source" ? canvasAnchorRect : (canvasLiveTransform ?? canvasAnchorRect);
    Object.assign(wrapperFrame, transformFrameStyle(morphAnchorRect));
    if (morphCropFrom && disposition.revealMorphFromFrame !== undefined && disposition.position !== undefined) {
      for (const [key, value] of Object.entries(morphFrameCssVars(disposition.revealMorphFromFrame, disposition.position))) {
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
  const wrapperStyle: CSSProperties | undefined = enlarged ? transformFrameStyle(SLIDE_INTERACTIVE_ENLARGE_FRAME) : Object.keys(wrapperFrame).length > 0 ? wrapperFrame : undefined;
  const resizeContentBaseline = measuredNatural ?? interactionRect;
  const resizeContentScale = !enlarged && !canvasFramed && !flowPixelOffset && transform && resizeContentBaseline ? interactiveDispositionContentScale(transform, resizeContentBaseline) : null;
  const contentScale = enlargedFlowContentScale ?? resizeContentScale;
  const contentStyle: CSSProperties | undefined = contentScale !== null ? interactiveDispositionContentScaleStyle(contentScale) : undefined;
  const hasContentStyle = contentStyle !== undefined;
  const chromeLayoutRect = flowSectionFrame && transform ? transform : (canvasLiveTransform ?? effectiveRect);
  const sectionAnimating = sectionRef.current !== null && isSectionAutoAnimating(sectionRef.current);
  const flowInkChromeRect = useFlowInkFrame ? inkInWrapper : undefined;
  const chromeStyle: CSSProperties | undefined = interactiveDispositionChromeStyle({
    selected: (selected || gesturing) && !sectionAnimating,
    effectiveRect: flowInkChromeRect ?? chromeLayoutRect,
    canvasFramed,
    enlarged,
  });
  const showControls = !morphGhost && (selected || gesturing || enlarged) && Boolean(enlarged || selected || gesturing || (useFlowInkFrame ? inkInWrapper : (chromeLayoutRect ?? effectiveRect)));
  const showHandles = showControls && !enlarged;
  const canResetPosition = enlarged || dispositionHasEphemeralLayout(transform, interactionRect, measuredNatural, flowLayout);

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
      <div ref={contentRef} className="presentation-interactive-disposition__content" style={hasContentStyle ? contentStyle : undefined}>
        <PresentationInteractiveDispositionIdContext.Provider value={id}>
          <PresentationDispositionEnlargeContext.Provider value={enlarged}>
            <MorphAnchorOnWrapperContext.Provider value={Boolean(revealMorphId && declaredRect !== undefined)}>
              <PresentationFigureCropFrameContext.Provider value={canvasLiveTransform ?? displayDisposition.position}>
                <MorphDispositionView disposition={displayDisposition} />
              </PresentationFigureCropFrameContext.Provider>
            </MorphAnchorOnWrapperContext.Provider>
          </PresentationDispositionEnlargeContext.Provider>
        </PresentationInteractiveDispositionIdContext.Provider>
        {showHandles ? (
          <div className="presentation-interactive-disposition__chrome" style={chromeStyle} aria-hidden>
            {DISPOSITION_RESIZE_HANDLES.map((handle) => (
              <div key={handle} className={`presentation-interaction-handle presentation-interaction-handle--${handle}`} onPointerDown={onHandlePointerDown(handle)} />
            ))}
          </div>
        ) : null}
      </div>
      {showControls ? (
        <div className="presentation-interaction-actions">
          {canResetPosition ? (
            <button type="button" className="presentation-interaction-reset" title="Reset position" onClick={onResetClick}>
              <Icon icon="rotate-ccw" size="small" />
            </button>
          ) : null}
          <button type="button" className="presentation-interaction-enlarge" title={enlarged ? "Exit enlarge" : "Enlarge"} aria-pressed={enlarged} onClick={onEnlargeClick}>
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
  if (target.closest(".presentation-target-ghost") || target.closest(".presentation-source-ghost")) {
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
  return target instanceof Element && Boolean(target.closest(".slide-background.present, .slide-background-content"));
}

/** @emoji 🖱️ Marquee and deselect on slide background; mounted on the arrangement section capture phase. */
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
      return resolveEffectiveDispositionRect(targetId, declaredRects.get(targetId), interaction, registry);
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
        if (!moved && Math.hypot(moveEvent.clientX - event.clientX, moveEvent.clientY - event.clientY) < POINTER_DRAG_THRESHOLD_PX) {
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
      host.classList.toggle("presentation-interaction-slide-reset-host--near", pointerNearSlideResetHotspot(section, clientX, clientY));
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

  const marqueeRule = marquee === null ? null : marqueeSelectionRule(marquee.start, marquee.end);

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
//#endregion 🔖️Interaction

//#region 🔖️ArrangementSection
function resolveSceneClipSrc(sceneHash: string): string {
  return `scenes/${sceneHash}/scene.mp4`;
}

const AnimateSceneEmbed: FC<{ readonly sceneHash: string }> = ({ sceneHash }) => (
  <div className="presentation-animate-scene" data-scene-hash={sceneHash}>
    <video className="presentation-media-video presentation-animate-scene__video" src={resolveSceneClipSrc(sceneHash)} controls playsInline muted loop preload="metadata" />
    <canvas className="presentation-animate-scene__canvas" aria-hidden />
  </div>
);

const ArrangementSection: FC<{
  readonly presentation: Presentation;
  readonly chapter: Chapter;
  readonly sequence: Sequence;
  readonly thought: Thought;
  readonly renderSlide: RenderSlide;
}> = ({ presentation, chapter, sequence, thought, renderSlide }) => {
  const sectionRef = useRef<HTMLElement>(null);
  const scope = useMemo(() => resolutionScopeForArrangement(presentation, chapter, sequence, thought, renderSlide.arrangement), [presentation, chapter, sequence, thought, renderSlide.arrangement]);
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
  const resolved = useMemo(() => resolveRevealArrangement(scope, renderSlide.arrangement, slideContext), [scope, renderSlide.arrangement, slideContext]);
  const morph = renderSlide.autoAnimateId !== undefined;
  const positioned = resolved.some((disposition) => disposition.position !== undefined);
  const layoutResolved = positioned && !morph ? centerRevealResolvedArrangement(resolved) : resolved;
  const interactiveLayout = useMemo(() => buildInteractiveSlideLayout(renderSlide.id, layoutResolved, morph), [layoutResolved, morph, renderSlide.id]);
  const declaredRects = useMemo(() => {
    const map = new Map<string, DispositionPosition | undefined>();
    for (const entry of interactiveLayout.placements) {
      map.set(entry.id, entry.sectionRect ?? entry.declaredRect);
    }
    return map;
  }, [interactiveLayout.placements]);
  const interactiveDispositionIds = useMemo(() => interactiveLayout.placements.filter((entry) => !isRevealMorphCompanionOnly(entry.disposition)).map((entry) => entry.id), [interactiveLayout.placements]);
  const sceneHash = renderSlide.metadata?.sceneHash ?? renderSlide.arrangement.metadata?.sceneHash;
  const placements = (
    <>
      {sceneHash ? <AnimateSceneEmbed sceneHash={sceneHash} /> : null}
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
}> = ({ sectionRef, morph, autoAnimateId, settleBeforeMorphTo, slideId, positioned, dispositionIds, declaredRects, placements }) => {
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
      {...(settleBeforeMorphTo?.length ? { "data-settle-before-morph-to": settleBeforeMorphTo.join(",") } : {})}
      title={slideId}
      className={["presentation-arrangement--interactive", positioned ? "presentation-arrangement--positioned" : undefined, isIntroArrangementId(slideId) ? "presentation-arrangement--intro" : undefined].filter(Boolean).join(" ")}
    >
      {positioned ? (
        <div className="presentation-arrangement-surface">
          <InteractionLayer marquee={backgroundInteraction.marquee} />
          <div className="presentation-arrangement-canvas">{placements}</div>
          <SlideInteractionReset sectionRef={sectionRef} dispositionIds={dispositionIds} declaredRects={declaredRects} />
        </div>
      ) : (
        <>
          <InteractionLayer marquee={backgroundInteraction.marquee} />
          {placements}
          <SlideInteractionReset sectionRef={sectionRef} dispositionIds={dispositionIds} declaredRects={declaredRects} />
        </>
      )}
    </section>
  );
};
//#endregion 🔖️ArrangementSection

//#region 🔖️PresentationInteractionProvider
/** @emoji 🖱️ Clears selection and ephemeral layout when reveal.js changes the active slide. */
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
        if (event.button === 0 && deckEl && !deckEl.classList.contains("overview") && isRevealSlideBackgroundPointerTarget(event.target)) {
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
  return <PresentationInteractionContext.Provider value={interaction}>{children}</PresentationInteractionContext.Provider>;
};
//#endregion 🔖️PresentationInteractionProvider

//#region 🔖️PresentationDeck
/** @emoji 🎞️ Maps a {@link Presentation} to reveal.js DOM. */
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
      const durationSeconds = typeof deck.getConfig().autoAnimateDuration === "number" ? deck.getConfig().autoAnimateDuration : 1;
      if (autoAnimateFinalizeTimer !== undefined) {
        clearTimeout(autoAnimateFinalizeTimer);
      }
      autoAnimateFinalizeTimer = setTimeout(
        () => {
          finalizeRevealAutoAnimateRestState(deckEl);
          autoAnimateFinalizeTimer = undefined;
        },
        durationSeconds * 1000 + 80,
      );
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
      const runSlides = resolvePresentationAutoAnimateRunSlides({ fromSlide, toSlide }, pendingAutoAnimateRunSlides);
      if (runSlides.fromSlide && runSlides.toSlide) {
        prepareArrangementBeforeAutoAnimate(runSlides.fromSlide, runSlides.toSlide);
      }
      const sheet = animateEvent.sheet;
      if (sheet && typeof sheet.innerHTML === "string") {
        const durationSeconds = typeof deck.getConfig().autoAnimateDuration === "number" ? deck.getConfig().autoAnimateDuration : 1;
        patchPresentationAutoAnimateRunStyleSheet(sheet, durationSeconds, runSlides.fromSlide, runSlides.toSlide);
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
                        <ArrangementSection key={`${chapter.id}-${sequence.id}-${thought.id}-${renderSlide.id}`} presentation={presentation} chapter={chapter} sequence={sequence} thought={thought} renderSlide={renderSlide} />
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
//#endregion 🔖️PresentationDeck

//#region 🔖️Mount
const mountedRoot = ephemeralBox<Root | null>("s.plugins.animate.apps.presentation.renderer.react.component.tsx.mountedRoot", null);
const surfaceChromeCleanup = ephemeralBox<(() => void) | null>("s.plugins.animate.apps.presentation.renderer.react.component.tsx.surfaceChromeCleanup", null);

/** @emoji 🚀️ Mounts a declarative presentation into a DOM root via React + reveal.js (eg-ice-25 reveal wiring). */
export function mountPresentation(rootEl: HTMLElement, presentation: Presentation, options?: PresentationMountOptions): void {
  surfaceChromeCleanup.current?.();
  surfaceChromeCleanup.current = null;
  const chrome = options?.surfaceChrome;
  if (chrome !== false) {
    surfaceChromeCleanup.current = applyElementsSurfaceChrome(chrome ?? DEFAULT_SURFACE_CHROME);
  }
  mountedRoot.current?.unmount();
  mountedRoot.current = createRoot(rootEl);
  mountedRoot.current.render(<PresentationDeck presentation={presentation} options={options} />);
}

/** @emoji 🧹️ Unmounts a presentation previously mounted with {@link mountPresentation}. */
export function unmountPresentation(): void {
  mountedRoot.current?.unmount();
  mountedRoot.current = null;
  surfaceChromeCleanup.current?.();
  surfaceChromeCleanup.current = null;
}
//#endregion 🔖️Mount

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests2 } = await import("./🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx");
  await registerTests2(import.meta.vitest, { FIGURE_WHEEL_ZOOM_MAX, PRESENTATION_MANY_TO_ONE_MORPH_CLASS, Reveal, SLIDE_INTERACTIVE_ENLARGE_FRAME, act, buildInteractiveSlideLayout, buildResolutionScope, clearRevealAutoAnimateInlineLayout, clientToSectionFraction, collectPresentationSlides, dispositionHasEphemeralLayout, dispositionPlacementContainer, elementIsInteractiveFigureDisposition, elementIsSourceGhostAnchor, elementIsTargetGhostAnchor, expandThoughtSlides, figureBackgroundSizeScrollAxis, figureBackgroundSizeZoomed, figureCoverOverflowAxis, figureCoverScrollContentSize, figureCoverScrollElementStyle, figureCropBackgroundPosition, figureCropBackgroundSize, figureCropBackgroundVars, figureCropBackgroundVarsTargetGhost, figureCropScrollBackgroundSize, figureEmbodimentScrollEnabled, figureScrollOffsetForBackgroundPosition, figureScrollOverlayThumbMetrics, figureWheelZoomStep, finalizeRevealAutoAnimateRestState, flowDispositionManipulationRect, flowDispositionOffsetStyle, flowPixelOffsetToSectionRect, flowPointerDeltaToLocal, groupBoundingRect, interactiveDispositionChromeStyle, interactiveDispositionContentScale, interactiveDispositionContentScaleStyle, intro, isFlowPixelOffsetTransform, isManyToOneMorphTransition, isNormalizedSlideFrame, isRevealAutoAnimatePairSource, isRevealSlideAutoAnimating, isSectionAutoAnimating, isUsableMeasuredRect, marqueeSelectionRule, marqueeSelects, measureDispositionBoundsInContainer, measureDispositionBoundsInSection, mediaTeaserActive, morphFrameCssVars, morphId, mosaicWindowedCoverVars, mountPresentation, normalizeMarquee, patchAutoAnimateUniformScale, patchPresentationAutoAnimateRunStyleSheet, patchPresentationAutoAnimateStyleSheet, pdfAdjacentPage, pdfCanGoToNextPage, pdfCanGoToPreviousPage, pdfCoverScale, pdfEmbodimentInitialPage, pdfPageNavEnabled, pdfScrollCoverScale, pointerNearSlideResetHotspot, prepareArrangementBeforeAutoAnimate, presentationAutoAnimateMatcher, readPresentationSlideIndicesFromUrl, rectContains, rectsIntersect, relaxHiddenPreflight, remapSplitDispositions, resetPdfCanvasPort, resizeDispositionRect, resolveArrangement, resolvePresentationAssetUrl, resolvePresentationAutoAnimateRunSlides, resolveRevealArrangement, revealDeckInOverview, revealInkMeasureForAutoAnimate, revealTextAutoAnimatePairOptions, scaleRectWithinGroup, setPdfCanvasPort, slideCoordinateRoot, slideHasEphemeralLayout, slidesShareAutoAnimateId, split, splitFigureGrid, suppressRevealOverviewSlideNavigation, syncArrangementSettledState, syncManyToOneGhostMorphFramesFromDom, syncPresentationSlideUrl, tightElementBoundsRect, toggleEnlargeRect, transformFrameStyle, translateDispositionRect, unmountPresentation }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests

//#region 🔖️json

//#region 🔖️Renderer
/** @emoji 🧬️ Renders parsed JSON as an interactive syntax tree. */
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

function JsonBranch({ label, value, depth, defaultExpanded }: { readonly label: string; readonly value: unknown; readonly depth: number; readonly defaultExpanded: boolean }): ReactNode {
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
      <button type="button" className="presentation-json-line presentation-json-toggle" style={{ paddingInlineStart: `${depth}ch` }} aria-expanded={expanded} onClick={() => setExpanded((open) => !open)}>
        <span className="presentation-json-caret" aria-hidden="true">
          {expanded ? "▾️" : "▸️"}
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
      {expanded ? entries.map(([key, entry]) => <JsonBranch key={key} label={isArray ? `[${key}]` : key} value={entry} depth={depth + 1} defaultExpanded={depth < 1} />) : null}
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
        <JsonBranch key={key} label={isArray ? `[${key}]` : key} value={entry} depth={0} defaultExpanded />
      ))}
    </div>
  );
}

const defaultJsonTreeRenderer: JsonTreeRenderer = {
  render(data) {
    return <DefaultJsonTree data={data} />;
  },
};

const jsonTreeRenderer = ephemeralBox<JsonTreeRenderer>("s.plugins.animate.apps.presentation.renderer.react.component.tsx.jsonTreeRenderer", defaultJsonTreeRenderer);

/** @emoji 🔌️ Replaces the JSON tree renderer (tests or alternate renderers). */
export function setJsonTreeRenderer(renderer: JsonTreeRenderer): void {
  jsonTreeRenderer.current = renderer;
}

/** @emoji 🧬️ Renders JSON through the active {@link JsonTreeRenderer}. */
export function renderJsonTree(data: unknown): ReactNode {
  return jsonTreeRenderer.current.render(data);
}
//#endregion 🔖️Renderer

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests3 } = await import("./🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx");
  await registerTests3(import.meta.vitest, { jsonPreview, renderJsonTree }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
