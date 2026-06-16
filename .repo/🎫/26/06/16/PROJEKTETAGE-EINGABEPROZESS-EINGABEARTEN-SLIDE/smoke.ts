#!/usr/bin/env bun
/** @emoji 🧪 Smoke-check Eingabeprozess Eingabearten slide deck wiring. */
import {
	PRESENTATION_DEFAULT_SLIDE_ASPECT,
	collectPresentationSlides,
	loadPresentationFromSlideGlob,
} from "@framework/presentation/core";
import { presentationMeta } from "../../../mit-bestand/präsentation/33.projektetage/spec.ts";

const projektetageRoot = new URL("../../../mit-bestand/präsentation/33.projektetage/", import.meta.url).pathname;
const slideModules = import.meta.glob<{ default: unknown }>(
	"../../../mit-bestand/präsentation/33.projektetage/slide/**/*.ts",
	{ eager: true, root: projektetageRoot },
);
const deck = loadPresentationFromSlideGlob(presentationMeta, slideModules);
const systematik = deck.chapters
	.find((chapter) => chapter.name === "Bauteilportal")
	?.sequences.find((sequence) => sequence.name === "Systematik");
const eingabeprozess = systematik?.thoughts.find((thought) => thought.name === "Eingabeprozess");
const eingabearten = eingabeprozess?.slides.find((slide) => slide.arrangement.name === "Eingabearten");

if (!eingabearten) {
	throw new Error("Eingabeprozess / Eingabearten slide missing");
}
expect(systematik?.thoughts.map((thought) => thought.name)).toEqual([
	"Eingabeprozess",
	"Konnektivität",
	"Typologien",
]);
expect(eingabearten.arrangement.dispositions).toHaveLength(1);
expect(eingabearten.arrangement.dispositions[0]?.participantId).toBe("eingabeprozess-eingabearten");
const embodiment = eingabearten.embodiments?.find((entry) => entry.id === "eingabeprozess-eingabearten--figure");
expect(embodiment).toMatchObject({
	kind: "figure",
	src: "/eingabeprozess-eingabearten.png",
	alt: "Eingabearten im Eingabeprozess",
	sourceAspect: 3586 / 1346,
});
const position = eingabearten.arrangement.dispositions[0]?.position;
expect(position).toBeDefined();
expect((position!.width / position!.height) * PRESENTATION_DEFAULT_SLIDE_ASPECT).toBeCloseTo(3586 / 1346, 10);

const bookmark = collectPresentationSlides(deck).find(
	(slide) => slide.thought === "Eingabeprozess" && slide.slide === "Eingabearten",
);
console.log("[DEBUG] Eingabeprozess Eingabearten bookmark:", bookmark);
console.log("ok");

function expect(value: unknown) {
	return {
		toEqual(expected: unknown) {
			const left = JSON.stringify(value);
			const right = JSON.stringify(expected);
			if (left !== right) {
				throw new Error(`Expected ${right} but got ${left}`);
			}
		},
		toHaveLength(length: number) {
			if (!Array.isArray(value) || value.length !== length) {
				throw new Error(`Expected length ${length} but got ${Array.isArray(value) ? value.length : typeof value}`);
			}
		},
		toBe(expected: unknown) {
			if (value !== expected) {
				throw new Error(`Expected ${String(expected)} but got ${String(value)}`);
			}
		},
		toMatchObject(expected: Record<string, unknown>) {
			for (const [key, expectedValue] of Object.entries(expected)) {
				const actual = (value as Record<string, unknown> | null | undefined)?.[key];
				if (JSON.stringify(actual) !== JSON.stringify(expectedValue)) {
					throw new Error(`Expected ${key}=${JSON.stringify(expectedValue)} but got ${JSON.stringify(actual)}`);
				}
			}
		},
		toBeDefined() {
			if (value === undefined) {
				throw new Error("Expected value to be defined");
			}
		},
		toBeCloseTo(expected: number, precision: number) {
			const actual = value as number;
			const diff = Math.abs(actual - expected);
			const tolerance = 10 ** -precision / 2;
			if (diff > tolerance) {
				throw new Error(`Expected ${actual} to be close to ${expected}`);
			}
		},
	};
}
