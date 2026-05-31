// #region 🧲Header
/** @emoji 📽 React + reveal.js renderer for `@framework/presentation/core` declarative decks. */
// #endregion 🧲Header

// #region 🔌Adapters
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import "./globals.css";
import {
	applyElementsSurfaceChrome,
	Expertise,
	type ElementsSurfaceChromeInput,
} from "@ui/react";
import { act, useEffect, useRef, type FC, type ReactNode } from "react";
import { createRoot, type Root } from "react-dom/client";
import type {
	AffiliationsEmbodiment,
	Arrangement,
	AuthorsEmbodiment,
	BulletEmbodiment,
	Embodiment,
	FigureEmbodiment,
	ParticipantEmphasis,
	Presentation,
	ResolvedPlacement,
	TextEmbodiment,
	Thought,
	Transition,
} from "@framework/presentation/core";
import {
	intro,
	resolveArrangement,
	resolveTextMorphRoot,
	type TextMorphRoot,
} from "@framework/presentation/core";
// #endregion 🔌Adapters

export type {
	AffiliationsEmbodiment,
	Arrangement,
	AuthorsEmbodiment,
	BulletEmbodiment,
	Embodiment,
	FigureEmbodiment,
	Participant,
	ParticipantEmphasis,
	ParticipantPlacement,
	Presentation,
	ResolvedPlacement,
	Sequence,
	TextEmbodiment,
	Thought,
	Transition,
} from "@framework/presentation/core";

export {
	countArrangements,
	intro,
	morphId,
	resolveArrangement,
	resolveEmbodiment,
	resolveTextMorphRoot,
} from "@framework/presentation/core";
export type { TextMorphRoot } from "@framework/presentation/core";
export { Expertise } from "@ui/react";

//#region 🔖MountOptions
/** @emoji ⚙️ Reveal.js and @ui/react surface chrome options for {@link mountPresentation}. */
export interface PresentationMountOptions {
	readonly surfaceChrome?: ElementsSurfaceChromeInput | false;
	readonly transition?: "fade" | "slide" | "convex" | "concave" | "zoom" | "none";
	readonly hash?: boolean;
	readonly slideNumber?: boolean;
	readonly width?: number;
	readonly height?: number;
}
//#endregion 🔖MountOptions

const DEFAULT_SURFACE_CHROME: ElementsSurfaceChromeInput = {
	theme: "system",
	device: "desktop",
	expertise: Expertise.NORMAL,
};

//#region 🔖RevealChrome
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

//#region 🔖MorphMatcher
interface MorphPair {
	from: HTMLElement;
	to: HTMLElement;
	options?: Record<string, unknown>;
}

interface MorphMatcherHost {
	getAutoAnimatePairs(fromSlide: HTMLElement, toSlide: HTMLElement): MorphPair[];
}

interface MorphRect {
	x: number;
	y: number;
	width: number;
	height: number;
}

/**
 * @emoji 📐 Measures an element with its slide forced visible.
 *
 * reveal.js hides out-of-view slides with the `hidden` attribute, but its auto-animate measurement
 * only un-hides slides flagged via inline `style.display`. A `hidden` `from` slide therefore collapses
 * to the deck origin and morph elements fly by the slide's centering offset. Removing `hidden` (and any
 * `display:none`) before {@link Element.getBoundingClientRect} restores a centered, comparable rect.
 */
function measureMorphRect(element: HTMLElement): MorphRect {
	const restore: Array<() => void> = [];
	for (let node: HTMLElement | null = element.closest("section"); node; node = node.parentElement?.closest("section") ?? null) {
		const slide = node;
		if (slide.hasAttribute("hidden")) {
			slide.removeAttribute("hidden");
			restore.push(() => slide.setAttribute("hidden", ""));
		}
		if (slide.style.display === "none" || slide.style.display === "") {
			const previous = slide.style.display;
			slide.style.display = "block";
			restore.push(() => {
				slide.style.display = previous;
			});
		}
	}
	const rect = element.getBoundingClientRect();
	for (let index = restore.length - 1; index >= 0; index -= 1) {
		restore[index]?.();
	}
	return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
}

/**
 * @emoji 🔗 reveal.js `autoAnimateMatcher`: reveal's own pairing plus a visible-measure.
 *
 * Keeps reveal's declarative `data-id`/text matching while guaranteeing centered `from`/`to` rects
 * so participants morph between embodiments instead of flying in from the deck origin.
 */
function morphMatcher(this: MorphMatcherHost, fromSlide: HTMLElement, toSlide: HTMLElement): MorphPair[] {
	const pairs = this.getAutoAnimatePairs(fromSlide, toSlide);
	for (const pair of pairs) {
		pair.options = { ...(pair.options ?? {}), measure: measureMorphRect };
	}
	return pairs;
}
//#endregion 🔖MorphMatcher

//#region 🔖MorphView
function emphasisClass(emphasis: ParticipantEmphasis): string | undefined {
	return emphasis === "muted" ? "opacity-20" : undefined;
}

function lineClass(embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string | undefined {
	return [embodiment.fit ? "r-fit-text" : undefined, emphasisClass(emphasis)].filter(Boolean).join(" ") || undefined;
}

/** @emoji 🎯 Renders {@link TextEmbodiment} with reveal.js `data-id` + eg-ice-25 DOM roots. */
function TextMorphView({
	morphId: anchorId,
	embodiment,
	emphasis,
}: {
	readonly morphId: string;
	readonly embodiment: TextEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	const mutedClass = emphasisClass(emphasis);
	const root = resolveTextMorphRoot(embodiment);
	const fitLineClass = lineClass(embodiment, emphasis);

	switch (root) {
		case "title":
			return (
				<h1 data-id={anchorId} className={mutedClass}>
					{embodiment.lines[0]}
				</h1>
			);
		case "body":
			return (
				<div data-id={anchorId}>
					{embodiment.lines.map((line) => (
						<p key={line} className={mutedClass}>
							{line}
						</p>
					))}
				</div>
			);
		case "heading-line":
		case "subheading-line":
			return (
				<h2 data-id={anchorId} className={fitLineClass}>
					{embodiment.lines[0]}
				</h2>
			);
		case "heading-block":
			return (
				<div data-id={anchorId}>
					{embodiment.lines.map((line) => (
						<h2 key={line} className={fitLineClass}>
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

function AuthorsMorphView({
	morphId: anchorId,
	embodiment,
}: {
	readonly morphId: string;
	readonly embodiment: AuthorsEmbodiment;
}): ReactNode {
	const namesMuted = embodiment.id === "marked";
	return (
		<div data-id={anchorId}>
			<div className="flex flex-row">
				{embodiment.people.map((person) => (
					<h4 key={person.name}>
						{namesMuted ? <span className="opacity-20">{person.name}</span> : person.name}
						{person.marks && person.marks.length > 0 ? <sup>{person.marks.join(",")}</sup> : null}
					</h4>
				))}
			</div>
		</div>
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
		<div data-id={anchorId}>
			<h5>
				{embodiment.entries.map((entry) => (
					<span key={entry.mark}>
						<sup>{entry.mark}</sup>
						{entry.name}
						<br />
					</span>
				))}
			</h5>
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
			<ul>
				{embodiment.items.map((item) => (
					<li key={item}>{item}</li>
				))}
			</ul>
		</div>
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
		<div data-id={anchorId} className={emphasisClass(emphasis)}>
			<img src={embodiment.src} alt={embodiment.alt ?? ""} />
		</div>
	);
}

function MorphPlacementView({ placement }: { readonly placement: ResolvedPlacement }): ReactNode {
	const { embodiment, emphasis, morphId: anchorId } = placement;
	switch (embodiment.kind) {
		case "text":
			return <TextMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
		case "authors":
			return <AuthorsMorphView morphId={anchorId} embodiment={embodiment} />;
		case "affiliations":
			return <AffiliationsMorphView morphId={anchorId} embodiment={embodiment} />;
		case "bullet":
			return <BulletMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
		case "figure":
			return <FigureMorphView morphId={anchorId} embodiment={embodiment} emphasis={emphasis} />;
		default: {
			const _exhaustive: never = embodiment;
			return _exhaustive;
		}
	}
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
	return (
		<section {...(morph ? { "data-auto-animate": "" } : {})} title={arrangement.id}>
			{resolved.map((placement) => (
				<MorphPlacementView
					key={`${arrangement.id}-${placement.morphId}-${placement.embodimentId ?? "default"}`}
					placement={placement}
				/>
			))}
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

	useEffect(() => {
		const deckEl = deckDivRef.current;
		if (!deckEl || deckRef.current) {
			return;
		}
		const revealOptions: Reveal.Options = {
			transition: options?.transition ?? "fade",
			autoAnimate: true,
			autoAnimateMatcher: morphMatcher,
		};
		if (options?.hash === true) {
			revealOptions.hash = true;
		}
		if (options?.slideNumber === true) {
			revealOptions.slideNumber = true;
		}
		if (options?.width ?? presentation.width) {
			revealOptions.width = options?.width ?? presentation.width;
		}
		if (options?.height ?? presentation.height) {
			revealOptions.height = options?.height ?? presentation.height;
		}
		const deck = new Reveal(deckEl, revealOptions);
		deckRef.current = deck;
		const onSlideChanged = (): void => {
			syncRevealBackgroundKind(deckEl);
		};
		void deck.initialize().then(() => {
			syncRevealBackgroundKind(deckEl);
			deck.on("slidechanged", onSlideChanged);
		});
		return () => {
			deck.off("slidechanged", onSlideChanged);
			try {
				deck.destroy();
			} catch {
				// reveal destroy may throw if already torn down
			}
			deckRef.current = null;
		};
	}, []);

	return (
		<div className="reveal" ref={deckDivRef} style={{ width: "100vw", height: "100vh" }}>
			<div className="slides">
				{presentation.sequences.map((sequence) => (
					<section key={sequence.id}>
						{sequence.thoughts.flatMap((thought) =>
							thought.arrangements.map((arrangement) => (
								<ArrangementSection
									key={`${sequence.id}-${thought.id}-${arrangement.id}`}
									thought={thought}
									arrangement={arrangement}
									transition={thought.transition}
								/>
							)),
						)}
					</section>
				))}
			</div>
		</div>
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

		beforeEach(() => {
			container = document.createElement("div");
			document.body.appendChild(container);
		});

		afterEach(() => {
			unmountPresentation();
			container.remove();
		});

		it("renders five vertical sections for the intro template", () => {
			const deck = intro({
				brand: "semio",
				title: { full: ["A", "B", "C"], short: "Short" },
				description: ["D1", "D2"],
				authors: [{ name: "Alice" }],
				affiliations: [{ mark: "1", name: "Uni" }],
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false });
			});
			const sections = container.querySelectorAll(".slides > section > section");
			expect(sections[0]?.hasAttribute("data-auto-animate")).toBe(true);
			expect(sections.length).toBe(5);
			const revealEl = container.querySelector(".reveal");
			expect(revealEl?.getAttribute("style")).toContain("100vw");
			expect(container.querySelector('[data-id="name"]')).toBeTruthy();
			expect(container.querySelector('[data-id="title"]')).toBeTruthy();
			expect(container.querySelector('[data-id="subtitle"]')).toBeTruthy();
			expect(container.querySelector('[data-id="authors"]')).toBeTruthy();
			expect(container.querySelector('[data-id="institutions"]')).toBeTruthy();
		});

		it("morphs by reveal pairs and forces a visible measure so hidden from-slides never collapse", () => {
			const from = document.createElement("section");
			from.innerHTML = '<h1 data-id="name">semio</h1>';
			const to = document.createElement("section");
			to.setAttribute("hidden", "");
			to.innerHTML = '<h1 data-id="name">semio</h1>';
			document.body.append(from, to);
			const defaultPairs: MorphPair[] = [{ from: from.firstElementChild as HTMLElement, to: to.firstElementChild as HTMLElement }];
			const host: MorphMatcherHost = { getAutoAnimatePairs: () => defaultPairs };
			const pairs = morphMatcher.call(host, from, to);
			expect(pairs).toHaveLength(1);
			const measure = pairs[0]?.options?.measure as ((el: HTMLElement) => MorphRect) | undefined;
			expect(typeof measure).toBe("function");
			measure?.(to.firstElementChild as HTMLElement);
			expect(to.hasAttribute("hidden")).toBe(true);
			from.remove();
			to.remove();
		});

		it("applies muted opacity on layered title slide", () => {
			const deck = intro({
				brand: "semio",
				title: { full: ["A"], short: "Short" },
				description: ["D"],
				authors: [{ name: "Alice" }],
				affiliations: [{ mark: "1", name: "Uni" }],
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const titleSlide = container.querySelector('.slides > section > section[title="subtitle"]');
			expect(titleSlide?.querySelector(".opacity-20")).toBeTruthy();
		});

		it("matches eg-ice-25 intro morph DOM per arrangement", () => {
			const deck = intro({
				brand: "semio",
				title: { full: ["A", "B", "C"], short: "Short" },
				description: ["D1", "D2"],
				authors: [{ name: "Alice", marks: ["1", "a"] }],
				affiliations: [{ mark: "1", name: "Uni" }],
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
			expect(slide("brand")?.querySelector('h1[data-id="name"]')?.textContent).toBe("semio");
			expect(slide("title")?.querySelector('div[data-id="title"] h2')).toBeTruthy();
			expect(slide("title")?.querySelectorAll('div[data-id="title"] h2').length).toBe(3);
			expect(slide("subtitle")?.querySelector('h2[data-id="title"]')).toBeTruthy();
			expect(slide("subtitle")?.querySelector('div[data-id="subtitle"] h2')).toBeTruthy();
			expect(slide("authors")?.querySelector('div[data-id="authors"] h4')).toBeTruthy();
			expect(slide("institutions")?.querySelector('div[data-id="institutions"] h5')).toBeTruthy();
			const marked = slide("institutions")?.querySelector('div[data-id="authors"] sup');
			expect(marked?.textContent).toBe("1,a");
		});

		it("enables reveal auto-animate and tags every morph arrangement with data-auto-animate", () => {
			const deck = intro({
				brand: "semio",
				title: { full: ["A", "B", "C"], short: "Short" },
				description: ["D1", "D2"],
				authors: [{ name: "Alice" }],
				affiliations: [{ mark: "1", name: "Uni" }],
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const morphSections = container.querySelectorAll(".slides > section > section[data-auto-animate]");
			expect(morphSections.length).toBe(5);
			for (const section of morphSections) {
				expect(section.querySelector('[data-id="name"]')?.tagName).toBe("H1");
			}
		});
	});
}
//#endregion 🧪Tests
