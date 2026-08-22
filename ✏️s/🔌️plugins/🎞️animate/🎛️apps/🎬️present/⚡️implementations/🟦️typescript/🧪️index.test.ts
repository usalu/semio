// #region 🔌️Adapters
import { describe, expect, it } from "vitest";
import {
	buildResolutionScope,
	collectPresentationSlides,
	loadPresentationFromSlideGlob,
	resolveArrangement,
} from "./📦️index";
// #endregion 🔌️Adapters

//#region 🧪️Recovery
describe("animate present core recovery", () => {
	it("assembles, collects, and resolves a slide glob hierarchy", () => {
		const presentation = loadPresentationFromSlideGlob(
			{ id: "recovery", name: "Recovery", language: "en" },
			{
				"./slide/Chapter/Sequence/Thought/Second.ts": {
					default: {
						order: 2,
						arrangement: {
							id: "second",
							dispositions: [{ participantId: "title", embodimentId: "title--main", emphasis: "muted" }],
						},
					},
				},
				"./slide/Chapter/Sequence/Thought/First.ts": {
					default: {
						order: 1,
						participants: [{ id: "title" }],
						embodiments: [{ kind: "text", id: "title--main", lines: ["Owned core"], level: "title" }],
						arrangement: {
							id: "first",
							dispositions: [{ participantId: "title", embodimentId: "title--main", emphasis: "active" }],
						},
					},
				},
			},
		);
		const chapter = presentation.chapters[0]!;
		const sequence = chapter.sequences[0]!;
		const thought = sequence.thoughts[0]!;
		const slides = collectPresentationSlides(presentation);
		const resolved = resolveArrangement(buildResolutionScope([presentation, chapter, sequence, thought]), thought.slides[0]!.arrangement);

		expect([chapter.name, sequence.name, thought.name]).toEqual(["Chapter", "Sequence", "Thought"]);
		expect(slides).toMatchObject([
			{ h: 0, v: 0, chapter: "Chapter", sequence: "Sequence", thought: "Thought", slide: "First" },
			{ h: 0, v: 1, chapter: "Chapter", sequence: "Sequence", thought: "Thought", slide: "Second" },
		]);
		expect(resolved).toMatchObject([
			{
				participant: { id: "title" },
				embodiment: { kind: "text", id: "title--main", lines: ["Owned core"] },
				emphasis: "active",
			},
		]);
	});
});
//#endregion 🧪️Recovery
