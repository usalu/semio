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
	goal: ["Mehr Zeit zum manuellen Entwerfen dank Automatisierung!"],
	authors: [
		{ name: "Ueli Saluz", marks: ["1", "a", "x"] },
		{ name: "Kinan Sarakbi", marks: ["2", "a", "y"] },
		{ name: "Phillipp Geyer", marks: ["1", "a", "x"] },
	],
	affiliations: [
		{ mark: "1", name: "Leibniz Universität Hannover" },
		{ mark: "2", name: "Universität der Künste" },
		{ mark: "a", name: "Fakultät für Architektur" },
		{ mark: "x", name: "Nachhaltige Gebäudesysteme" },
		{ mark: "y", name: "Konstruktives Entwerfen" },
	],
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
		it("declares five intro slides", () => {
			expect(countArrangements(deck)).toBe(5);
		});
	});
}
//#endregion 🧪Tests
