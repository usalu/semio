// #region 🧲Header
/** @emoji 📽 33. Projektetage — declarative paper intro via `@framework/presentation`. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
	countArrangements,
	intro,
	splitFigureGrid,
	type Presentation,
	type SplitTile,
	type Thought,
} from "@framework/presentation/core";
import { Expertise, mountPresentation } from "@framework/presentation/renderer/react";
import "./globals.css";
// #endregion 🔌Adapters

//#region 🔖Deck
const ASSET_CATALOGUE = "./bauteilbo\u0308rse.png";
const ASSET_VIDEO = "./bauen-mit-bestand.mp4";
const ASSET_THESIS_PDF = "./bachelor-thesis-ueli-saluz.pdf";

const CATALOGUE_FRAME = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
const CATALOGUE_SPLIT_FRAME = { x: 0.18, y: 0.12, width: 0.64, height: 0.52 };
const CATALOGUE_TILES_ASSEMBLED = splitFigureGrid({
	rows: 3,
	columns: 5,
	frame: CATALOGUE_FRAME,
	gap: 0,
});
const CATALOGUE_TILES_SPREAD = splitFigureGrid({
	rows: 3,
	columns: 5,
	frame: CATALOGUE_SPLIT_FRAME,
	gap: 0.02,
});
const CATALOGUE_FOCUS_TILES: SplitTile[] = CATALOGUE_TILES_SPREAD.filter((tile) =>
	tile.key.startsWith("tile-r1-"),
).map((tile, column) => ({
	...tile,
	position: {
		x: 0.04 + column * 0.192,
		y: 0.22,
		width: 0.17,
		height: 0.56,
	},
	emphasis: "active" as const,
}));

const introDeck = intro({
	id: "projektetage",
	name: "33. Projektetage",
	title: {
		full: ["Entwerfen mit Bestand"],
		short: "Entwerfen mit Bestand",
	},
	description: {
		full: [
			"Eine offene Plattform für einen KI-unterstützten, performance-optimierten und integrativen Entwurfsprozess mit wiederverwendeten Baukomponenten",
		],
		short: "Plattform zum Entwerfen mit wiederverwendete Bauteilen",
	},
	goal: ["Mehr Zeit zum manuellen Entwerfen", "dank Automatisierung!"],
	authors: {
		lines: [
			[
				{ name: "Ueli Saluz", marks: ["a", "1", "x"] },
				{ name: "Phillipp Geyer", marks: ["a", "1", "x"] },
			],
			[
				{ name: "Kinan Sarakbi", marks: ["a", "2", "y"] },
				{ name: "Christoph Gengnagel", marks: ["a", "2", "y"] },
			],
		],
	},
	affiliations: {
		steps: [
			[{ mark: "a", name: "Fakultät für Architektur" }],
			[
				{ mark: "a", name: "Fakultät für Architektur" },
				{ mark: "1", name: "Leibniz Universität Hannover" },
				{ mark: "2", name: "Universität der Künste" },
			],
			[
				{ mark: "a", name: "Fakultät für Architektur" },
				{
					mark: "1",
					name: "Leibniz Universität Hannover",
					shortName: "LUH",
					suffix: { mark: "x", name: "Nachhaltige Gebäudesysteme" },
				},
				{
					mark: "2",
					name: "Universität der Künste",
					shortName: "UdK",
					suffix: { mark: "y", name: "Konstruktives Entwerfen" },
				},
			],
		],
	},
});

const mediaThought: Thought = {
	id: "media",
	transition: { kind: "morph" },
	participants: [
		{
			id: "catalogue",
			embodiments: [
				{
					kind: "figure",
					src: ASSET_CATALOGUE,
					alt: "Komponentenkatalog",
				},
			],
		},
		{
			id: "demo-video",
			embodiments: [
				{
					kind: "video",
					src: ASSET_VIDEO,
					muted: true,
					controls: true,
				},
			],
		},
		{
			id: "thesis",
			embodiments: [
				{
					kind: "pdf",
					src: ASSET_THESIS_PDF,
					page: 1,
					alt: "Bachelorarbeit Ueli Saluz",
				},
			],
		},
	],
	arrangements: [
		{
			id: "catalogue",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					position: CATALOGUE_FRAME,
				},
			],
		},
		{
			id: "catalogue-tiles",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					split: { tiles: CATALOGUE_TILES_ASSEMBLED },
				},
			],
		},
		{
			id: "catalogue-split",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					split: { tiles: CATALOGUE_TILES_SPREAD },
				},
			],
		},
		{
			id: "catalogue-focus",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "active",
					split: { tiles: CATALOGUE_FOCUS_TILES },
				},
			],
		},
		{
			id: "media-suite",
			dispositions: [
				{
					participantId: "catalogue",
					emphasis: "muted",
					position: { x: 0.02, y: 0.05, width: 0.3, height: 0.35 },
				},
				{
					participantId: "demo-video",
					emphasis: "active",
					position: { x: 0.35, y: 0.1, width: 0.6, height: 0.5 },
				},
				{
					participantId: "thesis",
					emphasis: "active",
					position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
				},
			],
		},
	],
};

const deck: Presentation = {
	...introDeck,
	sequences: [
		{
			id: "main",
			thoughts: [...introDeck.sequences[0]!.thoughts, mediaThought],
		},
	],
};

function mount(): void {
	const el = document.getElementById("root");
	if (!el) {
		return;
	}
	mountPresentation(el, deck, {
		transition: "fade",
		hash: false,
		slideNumber: false,
		surfaceChrome: { theme: "dark", device: "desktop", expertise: Expertise.NORMAL },
	});
}

if (typeof document !== "undefined" && !import.meta.vitest) {
	mount();
}
//#endregion 🔖Deck

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("projektetage deck", () => {
		it("declares intro plus media arrangement slides", () => {
			expect(countArrangements(deck)).toBe(12);
		});

		it("splits the catalogue figure into fifteen morph tiles", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const tilesArrangement = media?.arrangements.find((a) => a.id === "catalogue-tiles");
			expect(tilesArrangement?.dispositions[0]?.split?.tiles).toHaveLength(15);
		});

		it("includes figure, video, and pdf participants in the media thought", () => {
			const media = deck.sequences[0]?.thoughts.find((t) => t.id === "media");
			const kinds = media?.participants.flatMap((p) => p.embodiments.map((e) => e.kind)) ?? [];
			expect(kinds).toEqual(["figure", "video", "pdf"]);
		});
	});
}
//#endregion 🧪Tests
