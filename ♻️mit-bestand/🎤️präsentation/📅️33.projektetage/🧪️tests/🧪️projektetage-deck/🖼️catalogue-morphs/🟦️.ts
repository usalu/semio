/** 🖼️ The catalogue morph and auto animate tests. */
import { bauteilboersenThought } from "../🧰️deck-test-kit/🟦️.ts";
import type { ProjektetageDeckTestKit } from "../🧰️deck-test-kit/🟦️.ts";

/** 🎭 Registers the catalogue assembly, morph target, focus, auto animate and label morph tests. */
export function registerCatalogueMorphTests(kit: ProjektetageDeckTestKit): void {
  const { CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3, arrangementRestDispositions, buildResolutionScope, columnLabelMorphFrom, deck, expandThoughtSlides, expect, inlineColumnLabelPosition, it, resolveArrangement } = kit;
  it("assembles the catalogue with morphTo targets for the focus slide", () => {
    const bauteilboersen = bauteilboersenThought(deck);
    const catalogue = bauteilboersen?.slides.find((slide) => slide.arrangement.id === "catalogue");
    expect(catalogue?.arrangement.dispositions).toHaveLength(1);
    expect(catalogue?.arrangement.dispositions[0]?.morphTo).toHaveLength(10);
    expect(catalogue?.arrangement.settleBeforeMorphTo).toBeUndefined();
  });
  it("names all ten catalogue morph targets semantically", () => {
    const bauteilboersen = bauteilboersenThought(deck);
    const catalogue = bauteilboersen?.slides.find((slide) => slide.arrangement.id === "catalogue");
    expect(catalogue?.arrangement.dispositions[0]?.morphTo?.map((slot) => slot.participantId)).toEqual([
      "Rippenplatte 1",
      "Rippenplatte 2",
      "Rippenplatte 3",
      "Rippenplatte 4",
      "Rippenplatte 5",
      "Rippenplatte 6",
      "Unterzug 1",
      "Unterzug 2",
      "Unterzug 3",
      "Stütze",
    ]);
  });
  it("focuses ten catalogue tile participants for column morph", () => {
    const bauteilboersen = bauteilboersenThought(deck);
    const focus = bauteilboersen?.slides.find((slide) => slide.arrangement.id === "catalogue-focus");
    const dispositions = focus?.arrangement.dispositions ?? [];
    expect(dispositions).toHaveLength(10);
    expect(dispositions.map((disposition) => disposition.participantId)).toEqual(["Rippenplatte 1", "Rippenplatte 2", "Rippenplatte 3", "Rippenplatte 4", "Rippenplatte 5", "Rippenplatte 6", "Unterzug 1", "Unterzug 2", "Unterzug 3", "Stütze"]);
  });
  it("assigns one auto-animate run across catalogue, focus, and labels", () => {
    const bauteilboersen = bauteilboersenThought(deck);
    expect(bauteilboersen).toBeDefined();
    const expanded = expandThoughtSlides(bauteilboersen!);
    const morphIds = expanded.filter((slide) => ["catalogue", "catalogue-focus", "catalogue-labels"].includes(slide.id)).map((slide) => slide.autoAnimateId);
    expect(morphIds).toHaveLength(3);
    expect(new Set(morphIds).size).toBe(1);
    expect(morphIds[0]).toBeTruthy();
  });
  it("expands ten morph sources and three resting labels on Bauteilbeschriftungen", () => {
    const bauteilboersen = bauteilboersenThought(deck);
    expect(bauteilboersen).toBeDefined();
    const expanded = expandThoughtSlides(bauteilboersen!);
    const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
    expect(labelSlide).toBeDefined();
    expect(arrangementRestDispositions(labelSlide!.arrangement)).toHaveLength(3);
    const morphFromSlots = labelSlide!.arrangement.dispositions.flatMap((disposition) => disposition.morphFrom ?? []);
    expect(morphFromSlots).toHaveLength(10);
  });
  it("morphs tile figures into label positions before column text appears", () => {
    const labelPosition = inlineColumnLabelPosition(0);
    const slots = columnLabelMorphFrom("col1", labelPosition);
    expect(slots?.every((slot) => slot.position === labelPosition)).toBe(true);
    expect(slots?.every((slot) => slot.embodimentId.endsWith("-figure"))).toBe(true);
  });
  it("morphs each column participant into inline label dispositions on one row", () => {
    const bauteilboersen = bauteilboersenThought(deck);
    expect(bauteilboersen).toBeDefined();
    const expanded = expandThoughtSlides(bauteilboersen!);
    const focusSlide = expanded.find((slide) => slide.id === "catalogue-focus");
    const labelSlide = expanded.find((slide) => slide.id === "catalogue-labels");
    const labelDispositions = labelSlide ? arrangementRestDispositions(labelSlide.arrangement) : [];
    expect(labelDispositions).toHaveLength(3);
    expect(labelDispositions.map((disposition) => disposition.participantId)).toEqual([CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3]);
    expect(labelDispositions.every((disposition) => disposition.embodimentId.endsWith("--label"))).toBe(true);
    const yPositions = labelDispositions.map((disposition) => disposition.position?.y);
    expect(new Set(yPositions).size).toBe(1);
    const scope = buildResolutionScope([bauteilboersen!]);
    const resolved = resolveArrangement(scope, labelSlide!.arrangement);
    expect(resolved.filter((entry) => entry.embodiment.kind === "text")).toHaveLength(3);
  });
}
