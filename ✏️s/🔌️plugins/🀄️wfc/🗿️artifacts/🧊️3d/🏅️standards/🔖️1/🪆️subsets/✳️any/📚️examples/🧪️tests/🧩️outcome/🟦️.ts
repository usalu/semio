// 🎯️ Every bundled wfc3d example's committed outcome, read from the fixtures rather than transcribed
// — an example that gains or loses a vector cannot leave this half behind.

import { describe, expect, it } from "vitest";

import corridor from "../../🚪️two-room-corridor/🧫️fixtures/🧩️example/🔣️.json";
import facade from "../../🧱️wall-roof-facade-strip/🧫️fixtures/🧩️example/🔣️.json";
import tower from "../../🗼️tower-stack/🧫️fixtures/🧩️example/🔣️.json";

const outcomes = [corridor, facade, tower];

describe("wfc3d example outcomes", () => {
  it("is the roster the subset declares, in the picker's own order", () => {
    expect(outcomes.map((outcome) => outcome.example)).toEqual(["two-room-corridor", "wall-roof-facade-strip", "tower-stack"]);
  });

  it("gives every example its own seed, so two examples never share a solve", () => {
    const seeds = outcomes.map((outcome) => outcome.seed);
    expect(new Set(seeds).size).toBe(seeds.length);
  });

  it.each(outcomes)("$example assigns exactly its own slots from its own catalogue", (outcome) => {
    const assigned = Object.entries(outcome.assignments as Record<string, string>);
    expect(assigned).toHaveLength(outcome.slots);
    expect(outcome.satisfiable).toBe(assigned.length > 0);
    for (const [, tile] of assigned) {
      expect(outcome.tiles).toContain(tile);
    }
  });

  it("ships at least one example whose slots are not all the same size", () => {
    expect(outcomes.some((outcome) => outcome.example === "tower-stack" && outcome.slots > 4)).toBe(true);
  });
});
