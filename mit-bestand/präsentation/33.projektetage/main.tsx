// #region 🧲Header
/** @emoji 📽 Reveal.js deck for 33. Projektetage with @ui/react chrome and live playground iframes. */
// #endregion 🧲Header

// #region 🔌Adapters
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import "./globals.css";
import {
	playgroundEmbedUrl,
	type PlaygroundSiteKind,
} from "../../../ui/styling/playground-embed-url.ts";
import { Button, Card, CardGrid } from "@ui/react";
import { StrictMode, useEffect, useRef, type FC, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
// #endregion 🔌Adapters

//#region 🔖Embeds
const EMBED_KINDS: readonly PlaygroundSiteKind[] = ["cad", "2d", "3d", "5d"];

const EMBED_LABELS: Record<PlaygroundSiteKind, string> = {
	semio: "Sketchpad",
	cad: "CAD spatial",
	"2d": "Puzzle 2D",
	"3d": "Puzzle 3D",
	"5d": "Puzzle 5D",
};

/** @emoji 🖼 Full-size playground iframe for one embed kind (dev localhost vs production host). */
const PlaygroundEmbed: FC<{ readonly kind: PlaygroundSiteKind; readonly title?: string }> = ({
	kind,
	title,
}) => {
	const src = playgroundEmbedUrl(kind, import.meta.env.DEV);
	return (
		<iframe
			title={title ?? EMBED_LABELS[kind]}
			src={src}
			className="h-[min(62vh,520px)] w-full border border-border bg-background"
			loading="lazy"
			allow="fullscreen"
		/>
	);
};
//#endregion 🔖Embeds

//#region 🔖Slides
const TitleSlide: FC = () => (
	<section>
		<h1 className="r-fit-text text-primary">33. Projektetage</h1>
		<p className="mt-6 text-2xl opacity-80">Mit Bestand · Zukunft Bau</p>
		<p className="mt-4 text-lg opacity-60">semio playgrounds embedded in reveal.js</p>
	</section>
);

const ComponentsSlide: FC = () => (
	<section>
		<h2>@ui/react</h2>
		<CardGrid className="mt-8 max-w-4xl">
			<Card title="Kit-of-parts">
				<p className="text-sm opacity-80">Port-based design modeling in the browser.</p>
				<div className="mt-4 flex gap-2">
					<Button variant="default">Primary</Button>
					<Button variant="outline">Outline</Button>
				</div>
			</Card>
			<Card title="Playgrounds">
				<p className="text-sm opacity-80">CAD, 2D, 3D, and 5D demos load live in the next slides.</p>
			</Card>
		</CardGrid>
	</section>
);

const EmbedSlide: FC<{ readonly kind: PlaygroundSiteKind }> = ({ kind }) => (
	<section>
		<h2>{EMBED_LABELS[kind]}</h2>
		<p className="mb-4 text-sm opacity-70">
			{import.meta.env.DEV ? "Dev" : "Production"}: {playgroundEmbedUrl(kind, import.meta.env.DEV)}
		</p>
		<PlaygroundEmbed kind={kind} />
	</section>
);

const EndSlide: FC = () => (
	<section>
		<h2>Danke</h2>
		<p className="mt-6 text-xl opacity-80">33.projektetage.zukunft-bau.mit-bestand.de</p>
	</section>
);
//#endregion 🔖Slides

//#region 🔖Deck
function Deck(): ReactNode {
	const deckDivRef = useRef<HTMLDivElement>(null);
	const deckRef = useRef<Reveal.Api | null>(null);

	useEffect(() => {
		if (!deckDivRef.current || deckRef.current) {
			return;
		}
		const deck = new Reveal(deckDivRef.current, {
			transition: "fade",
			hash: true,
			slideNumber: true,
			width: 1280,
			height: 720,
		});
		deckRef.current = deck;
		void deck.initialize();
		return () => {
			deckRef.current?.destroy();
			deckRef.current = null;
		};
	}, []);

	return (
		<div className="reveal h-full w-full" ref={deckDivRef}>
			<div className="slides">
				<TitleSlide />
				<ComponentsSlide />
				{EMBED_KINDS.map((kind) => (
					<EmbedSlide key={kind} kind={kind} />
				))}
				<EndSlide />
			</div>
		</div>
	);
}

function mount(): void {
	const el = document.getElementById("root");
	if (!el) {
		return;
	}
	createRoot(el).render(
		<StrictMode>
			<Deck />
		</StrictMode>,
	);
}

if (typeof document !== "undefined" && !import.meta.vitest) {
	mount();
}
//#endregion 🔖Deck

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("playgroundEmbedUrl", () => {
		it("uses localhost ports in dev", () => {
			expect(playgroundEmbedUrl("cad", true)).toBe("http://localhost:6020");
			expect(playgroundEmbedUrl("2d", true)).toBe("http://localhost:6012");
			expect(playgroundEmbedUrl("3d", true)).toBe("http://localhost:6013");
			expect(playgroundEmbedUrl("5d", true)).toBe("http://localhost:6014");
		});

		it("uses public hosts in production", () => {
			expect(playgroundEmbedUrl("cad", false)).toBe("https://play.cad.semio-tech.com");
			expect(playgroundEmbedUrl("2d", false)).toBe("https://play.2d.semio-tech.com");
			expect(playgroundEmbedUrl("3d", false)).toBe("https://play.3d.semio-tech.com");
			expect(playgroundEmbedUrl("5d", false)).toBe("https://play.5d.semio-tech.com");
		});
	});
}
//#endregion 🧪Tests
