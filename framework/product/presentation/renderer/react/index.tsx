// #region 🧲Header
/** @emoji 📽 React + reveal.js renderer for `@framework/presentation/core` declarative decks. */
// #endregion 🧲Header

// #region 🔌Adapters
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import "reveal.js/dist/theme/black.css";
import {
	Expertise,
	useElementsSurfaceChrome,
	type ElementsSurfaceChromeInput,
} from "@ui/react";
import { act, StrictMode, useEffect, useRef, type FC, type ReactNode } from "react";
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
import { intro, resolveArrangement } from "@framework/presentation/core";
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
	resolveArrangement,
	resolveEmbodiment,
} from "@framework/presentation/core";

//#region 🔖MountOptions
/** @emoji 🎨 Visual preset: fullscreen reveal deck chrome or @ui/react system shell. */
export type PresentationAppearance = "deck" | "elements";

/** @emoji ⚙️ Reveal.js and surface chrome options for {@link mountPresentation}. */
export interface PresentationMountOptions {
	readonly appearance?: PresentationAppearance;
	readonly surfaceChrome?: ElementsSurfaceChromeInput;
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

//#region 🔖EmbodimentView
function emphasisClass(emphasis: ParticipantEmphasis): string | undefined {
	return emphasis === "muted" ? "opacity-20" : undefined;
}

function TextEmbodimentView({
	participantId,
	embodiment,
	emphasis,
}: {
	readonly participantId: string;
	readonly embodiment: TextEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	const mutedClass = emphasisClass(emphasis);
	const fit = embodiment.fit ? "r-fit-text" : undefined;
	const lineClass = [fit, mutedClass].filter(Boolean).join(" ") || undefined;

	if (embodiment.level === "title") {
		return (
			<h1 data-id={participantId} className={mutedClass}>
				{embodiment.lines[0]}
			</h1>
		);
	}

	if (embodiment.level === "body") {
		return (
			<div data-id={participantId}>
				{embodiment.lines.map((line) => (
					<p key={line} className={mutedClass}>
						{line}
					</p>
				))}
			</div>
		);
	}

	if (embodiment.lines.length === 1) {
		return (
			<h2 data-id={participantId} className={lineClass}>
				{embodiment.lines[0]}
			</h2>
		);
	}

	return (
		<div data-id={participantId}>
			{embodiment.lines.map((line) => (
				<h2 key={line} className={lineClass}>
					{line}
				</h2>
			))}
		</div>
	);
}

function AuthorsEmbodimentView({
	participantId,
	embodiment,
}: {
	readonly participantId: string;
	readonly embodiment: AuthorsEmbodiment;
}): ReactNode {
	const namesMuted = embodiment.id === "marked";
	return (
		<div data-id={participantId}>
			<div className="flex flex-row">
				{embodiment.people.map((person) => (
					<h4 key={person.name}>
						{namesMuted ? <span className="opacity-20">{person.name}</span> : person.name}
						{person.marks?.map((mark) => (
							<sup key={mark}>{mark}</sup>
						))}
					</h4>
				))}
			</div>
		</div>
	);
}

function AffiliationsEmbodimentView({
	participantId,
	embodiment,
}: {
	readonly participantId: string;
	readonly embodiment: AffiliationsEmbodiment;
}): ReactNode {
	return (
		<div data-id={participantId}>
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

function BulletEmbodimentView({
	participantId,
	embodiment,
	emphasis,
}: {
	readonly participantId: string;
	readonly embodiment: BulletEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	return (
		<div data-id={participantId} className={emphasisClass(emphasis)}>
			<ul>
				{embodiment.items.map((item) => (
					<li key={item}>{item}</li>
				))}
			</ul>
		</div>
	);
}

function FigureEmbodimentView({
	participantId,
	embodiment,
	emphasis,
}: {
	readonly participantId: string;
	readonly embodiment: FigureEmbodiment;
	readonly emphasis: ParticipantEmphasis;
}): ReactNode {
	return (
		<div data-id={participantId} className={emphasisClass(emphasis)}>
			<img src={embodiment.src} alt={embodiment.alt ?? ""} />
		</div>
	);
}

function EmbodimentView({ placement }: { readonly placement: ResolvedPlacement }): ReactNode {
	const { participant, embodiment, emphasis } = placement;
	switch (embodiment.kind) {
		case "text":
			return <TextEmbodimentView participantId={participant.id} embodiment={embodiment} emphasis={emphasis} />;
		case "authors":
			return <AuthorsEmbodimentView participantId={participant.id} embodiment={embodiment} />;
		case "affiliations":
			return <AffiliationsEmbodimentView participantId={participant.id} embodiment={embodiment} />;
		case "bullet":
			return <BulletEmbodimentView participantId={participant.id} embodiment={embodiment} emphasis={emphasis} />;
		case "figure":
			return <FigureEmbodimentView participantId={participant.id} embodiment={embodiment} emphasis={emphasis} />;
		default: {
			const _exhaustive: never = embodiment;
			return _exhaustive;
		}
	}
}
//#endregion 🔖EmbodimentView

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
				<EmbodimentView key={`${arrangement.id}-${placement.participant.id}`} placement={placement} />
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

	const appearance = options?.appearance ?? "deck";

	useEffect(() => {
		if (!deckDivRef.current || deckRef.current) {
			return;
		}
		const revealOptions: Reveal.Options = {
			transition: options?.transition ?? "fade",
			autoAnimate: true,
		};
		if (appearance === "deck") {
			revealOptions.hash = options?.hash ?? false;
			revealOptions.slideNumber = options?.slideNumber ?? false;
		} else {
			revealOptions.hash = options?.hash ?? true;
			revealOptions.slideNumber = options?.slideNumber ?? true;
			revealOptions.backgroundColor = "var(--base)";
			if (options?.width ?? presentation.width) {
				revealOptions.width = options?.width ?? presentation.width;
			}
			if (options?.height ?? presentation.height) {
				revealOptions.height = options?.height ?? presentation.height;
			}
		}
		const deck = new Reveal(deckDivRef.current, revealOptions);
		deckRef.current = deck;
		void deck.initialize().then(() => {
			if (appearance !== "deck") {
				syncRevealBackgroundKind(deckDivRef.current);
				deck.on("slidechanged", () => syncRevealBackgroundKind(deckDivRef.current));
			}
		});
		return () => {
			try {
				deckRef.current?.destroy();
			} catch {
				// reveal destroy may throw if already torn down
			}
			deckRef.current = null;
		};
	}, [
		appearance,
		presentation.height,
		presentation.width,
		options?.hash,
		options?.height,
		options?.slideNumber,
		options?.transition,
		options?.width,
	]);

	const revealStyle =
		appearance === "deck" ? ({ width: "100vw", height: "100vh" } as const) : undefined;

	return (
		<div className={appearance === "deck" ? "reveal" : "reveal h-full w-full"} ref={deckDivRef} style={revealStyle}>
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

//#region 🔖Shell
const PresentationShellDeck: FC<{ readonly children: ReactNode }> = ({ children }) => (
	<div className="h-full w-full">{children}</div>
);

const PresentationShellElements: FC<{
	readonly children: ReactNode;
	readonly options?: PresentationMountOptions;
}> = ({ children, options }) => {
	useElementsSurfaceChrome(options?.surfaceChrome ?? DEFAULT_SURFACE_CHROME);

	useEffect(() => {
		if (typeof window === "undefined") {
			return;
		}
		const mq = window.matchMedia("(prefers-color-scheme: dark)");
		const onThemeChange = (): void => {
			syncRevealBackgroundKind(document.querySelector(".reveal"));
		};
		onThemeChange();
		mq.addEventListener("change", onThemeChange);
		return () => mq.removeEventListener("change", onThemeChange);
	}, []);

	return <div className="h-full w-full bg-base text-foreground">{children}</div>;
};

const PresentationShell: FC<{
	readonly children: ReactNode;
	readonly options?: PresentationMountOptions;
}> = ({ children, options }) => {
	if ((options?.appearance ?? "deck") === "deck") {
		return <PresentationShellDeck>{children}</PresentationShellDeck>;
	}
	return (
		<PresentationShellElements options={options}>{children}</PresentationShellElements>
	);
};
//#endregion 🔖Shell

//#region 🔖Mount
let mountedRoot: Root | null = null;

/** @emoji 🚀 Mounts a declarative presentation into a DOM root via React + reveal.js. */
export function mountPresentation(
	rootEl: HTMLElement,
	presentation: Presentation,
	options?: PresentationMountOptions,
): void {
	mountedRoot?.unmount();
	mountedRoot = createRoot(rootEl);
	mountedRoot.render(
		<StrictMode>
			<PresentationShell options={options}>
				<PresentationDeck presentation={presentation} options={options} />
			</PresentationShell>
		</StrictMode>,
	);
}

/** @emoji 🧹 Unmounts a presentation previously mounted with {@link mountPresentation}. */
export function unmountPresentation(): void {
	mountedRoot?.unmount();
	mountedRoot = null;
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
			expect(container.querySelector('[data-id="name"]')).toBeTruthy();
			expect(container.querySelector('[data-id="title"]')).toBeTruthy();
			expect(container.querySelector('[data-id="subtitle"]')).toBeTruthy();
			expect(container.querySelector('[data-id="authors"]')).toBeTruthy();
			expect(container.querySelector('[data-id="institutions"]')).toBeTruthy();
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
				mountPresentation(container, deck, { hash: false, slideNumber: false });
			});
			const titleSlide = container.querySelector('.slides > section > section[title="subtitle"]');
			expect(titleSlide?.querySelector(".opacity-20")).toBeTruthy();
		});
	});
}
//#endregion 🧪Tests
