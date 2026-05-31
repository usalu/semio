// #region 🧲Header
/** @emoji 📽 33. Projektetage — declarative paper intro via `@framework/presentation`. */
// #endregion 🧲Header

// #region 🔌Adapters
import { countArrangements, intro } from "@framework/presentation/core";
import { Expertise, mountPresentation } from "@framework/presentation/renderer/react";
import "./globals.css";
// #endregion 🔌Adapters

//#region 🔖Deck
const deck = intro({
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
				{ name: "Ueli Saluz", marks: ["1", "a", "x"] },
				{ name: "Phillipp Geyer", marks: ["1", "a", "x"] },
			],
			[
				{ name: "Kinan Sarakbi", marks: ["2", "a", "y"] },
				{ name: "Christoph Gengnagel", marks: ["2", "a", "y"] },
			],
		],
	},
	affiliations: {
		steps: [
			[
				{ mark: "1", name: "Leibniz Universität Hannover" },
				{ mark: "2", name: "Universität der Künste" },
			],
			[
				{ mark: "1", name: "Leibniz Universität Hannover" },
				{ mark: "2", name: "Universität der Künste" },
				{ mark: "a", name: "Fakultät für Architektur" },
			],
			[
				{
					mark: "1",
					name: "Leibniz Universität Hannover",
					suffix: { mark: "x", name: "Nachhaltige Gebäudesysteme" },
				},
				{
					mark: "2",
					name: "Universität der Künste",
					suffix: { mark: "y", name: "Konstruktives Entwerfen" },
				},
				{ mark: "a", name: "Fakultät für Architektur" },
			],
		],
	},
});

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
		it("declares seven intro slides", () => {
			expect(countArrangements(deck)).toBe(7);
		});
	});
}
//#endregion 🧪Tests
