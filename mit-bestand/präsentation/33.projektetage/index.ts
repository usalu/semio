// #region 🧲Header
/** @emoji 📽 33. Projektetage — declarative eg-ice-25 intro via `@framework/presentation`. */
// #endregion 🧲Header

// #region 🔌Adapters
import { countArrangements, intro } from "@framework/presentation/core";
import { mountPresentation } from "@framework/presentation/renderer/react";
import "./globals.css";
// #endregion 🔌Adapters

//#region 🔖Deck
const deck = intro({
	id: "projektetage",
	name: "semio · EG-ICE intro",
	brand: "semio",
	title: {
		full: [
			"Large-Language-Model-based",
			"Building-Information-Model Alignment",
			"for Automatic-Compliance-Checking",
		],
		short: "LLM-based BIM Alignment for ACC",
	},
	description: [
		"Towards Closing the Gap between",
		"Model Authoring and Model Checking",
		"for Kit-of-Parts Architecture",
	],
	authors: [
		{ name: "Ueli Saluz", marks: ["1", "a"] },
		{ name: "Ildar Baimuratov", marks: ["1", "b"] },
		{ name: "Philipp Geyer", marks: ["1", "a"] },
	],
	affiliations: [
		{ mark: "1", name: "Leibniz University Hannover" },
		{ mark: "a", name: "Faculty of Architecture" },
		{ mark: "b", name: "Faculty of Computer Science" },
	],
});

function mount(): void {
	const el = document.getElementById("root");
	if (!el) {
		return;
	}
	mountPresentation(el, deck, { hash: true, slideNumber: true, transition: "fade" });
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
