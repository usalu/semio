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
	AffiliationEntry,
	AffiliationsEmbodiment,
	Arrangement,
	AuthorPerson,
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
	AffiliationEntry,
	AffiliationsEmbodiment,
	Arrangement,
	AuthorPerson,
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
function emphasisClass(emphasis: ParticipantEmphasis): string | undefined {
	return emphasis === "muted" ? "opacity-20" : undefined;
}

function lineClass(embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string | undefined {
	return [embodiment.fit ? "r-fit-text" : undefined, emphasisClass(emphasis)].filter(Boolean).join(" ") || undefined;
}

function centeredLineClass(embodiment: TextEmbodiment, emphasis: ParticipantEmphasis): string {
	return [lineClass(embodiment, emphasis), "text-center"].filter(Boolean).join(" ");
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
	const centeredHeadingClass = centeredLineClass(embodiment, emphasis);

	switch (root) {
		case "title":
			return (
				<h1 data-id={anchorId} className={[mutedClass, "text-center"].filter(Boolean).join(" ") || "text-center"}>
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
				<div data-id={anchorId} className="w-full text-center">
					<h2 className={centeredHeadingClass}>{embodiment.lines[0]}</h2>
				</div>
			);
		case "heading-block":
			return (
				<div data-id={anchorId} className="w-full text-center">
					{embodiment.lines.map((line) => (
						<h2 key={line} className={centeredHeadingClass}>
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

function AuthorsMorphView({
	morphId: anchorId,
	embodiment,
}: {
	readonly morphId: string;
	readonly embodiment: AuthorsEmbodiment;
}): ReactNode {
	const namesMuted = embodiment.id === "marked";
	const rows = authorRows(embodiment);
	return (
		<div data-id={anchorId} className="w-full max-w-full text-center">
			{rows.map((line, lineIndex) => (
				<div
					key={`${anchorId}-line-${lineIndex}`}
					className="flex w-full flex-row flex-wrap items-center justify-center gap-x-10 gap-y-2"
				>
					{line.map((person) => (
						<h4 key={person.name} className="m-0 shrink-0 text-center">
							{namesMuted ? <span className="opacity-20">{person.name}</span> : person.name}
							{person.marks && person.marks.length > 0 ? <sup>{person.marks.join(",")}</sup> : null}
						</h4>
					))}
				</div>
			))}
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
		<div data-id={anchorId} className="w-full text-center">
			<h5 className="text-center">
				{embodiment.entries.map((entry) => (
					<span key={`${entry.mark}-${entry.suffix?.mark ?? ""}`}>
						<sup>{entry.mark}</sup>
						{entry.name}
						{entry.suffix ? (
							<>
								{" "}
								<sup>{entry.suffix.mark}</sup>
								{entry.suffix.name}
							</>
						) : null}
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
		relaxHiddenPreflight();
		const deck = new Reveal(deckEl, revealOptions);
		deckRef.current = deck;
		const onSlideChanged = (): void => {
			relaxHiddenPreflight();
			syncRevealBackgroundKind(deckEl);
		};
		void deck.initialize().then(() => {
			relaxHiddenPreflight();
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

		const testAffiliationSteps = {
			steps: [
				[{ mark: "1", name: "Uni" }],
				[
					{ mark: "1", name: "Uni" },
					{ mark: "a", name: "Faculty" },
				],
				[
					{ mark: "1", name: "Uni", suffix: { mark: "x", name: "Chair X" } },
					{ mark: "a", name: "Faculty" },
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
			expect(container.querySelector('[data-id="title"]')).toBeTruthy();
			expect(container.querySelector('[data-id="description"]')).toBeTruthy();
			expect(container.querySelector('[data-id="goal"]')).toBeTruthy();
			expect(container.querySelector('[data-id="authors"]')).toBeTruthy();
			expect(container.querySelector('[data-id="institutions"]')).toBeTruthy();
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
						[{ name: "Alice", marks: ["1", "a"] }, { name: "Bob" }],
						[{ name: "Carol" }],
					],
				},
				affiliations: testAffiliationSteps,
			});
			act(() => {
				mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
			});
			const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
			expect(slide("title")?.querySelector('div[data-id="title"] h2')).toBeTruthy();
			expect(slide("title")?.querySelectorAll('div[data-id="title"] h2').length).toBe(3);
			expect(slide("description")?.querySelector('div[data-id="title"] h2')).toBeTruthy();
			expect(slide("description")?.querySelector('h2[data-id="title"]')).toBeNull();
			expect(slide("description")?.querySelector('div[data-id="description"] h2')).toBeTruthy();
			expect(slide("description")?.querySelectorAll('div[data-id="description"] h2').length).toBe(2);
			expect(slide("goal")?.querySelector('div[data-id="description"] h2')?.textContent).toBe("D short");
			expect(slide("goal")?.querySelector('div[data-id="goal"] h2')).toBeTruthy();
			const authorLines = slide("authors")?.querySelectorAll('div[data-id="authors"] > div');
			expect(authorLines?.length).toBe(2);
			expect(slide("authors")?.querySelectorAll('div[data-id="authors"] h4').length).toBe(3);
			expect(slide("affiliations-1")?.querySelectorAll('div[data-id="institutions"] sup').length).toBe(1);
			expect(slide("affiliations-2")?.querySelectorAll('div[data-id="institutions"] sup').length).toBe(2);
			expect(slide("affiliations-3")?.querySelector('div[data-id="institutions"] h5')).toBeTruthy();
			expect(slide("affiliations-3")?.textContent).toContain("Chair X");
			const marked = slide("affiliations-3")?.querySelector('div[data-id="authors"] sup');
			expect(marked?.textContent).toBe("1,a");
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
			const morphSections = container.querySelectorAll(".slides > section > section[data-auto-animate]");
			expect(morphSections.length).toBe(7);
			const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
			expect(slide("title")?.querySelector('[data-id="title"]')).toBeTruthy();
			expect(slide("title")?.querySelector('[data-id="description"]')).toBeNull();
			expect(slide("description")?.querySelector('[data-id="description"]')).toBeTruthy();
			expect(slide("goal")?.querySelector('[data-id="goal"]')).toBeTruthy();
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
	});
}
//#endregion 🧪Tests
