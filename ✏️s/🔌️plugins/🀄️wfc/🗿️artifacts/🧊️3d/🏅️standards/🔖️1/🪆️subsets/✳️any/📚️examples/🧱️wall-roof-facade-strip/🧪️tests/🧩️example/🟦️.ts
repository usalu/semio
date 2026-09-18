// 🧪️ Example `wall-roof-facade-strip` — the committed outcome fixture is the one statement of this example's shape.
// The Rust sibling asserts it against the live builder AND the live solve; this half asserts the
// fixture is internally coherent, so a hand-edit that no Rust test happens to cover still fails.

import { describe, expect, it } from "vitest";

import outcome from "../../🧫️fixtures/🧩️example/🔣️.json";

describe("wall-roof-facade-strip", () => {
  it("is this example's own committed outcome", () => {
    expect(outcome.schema).toBe("s.wfc.wfc3d.example-outcome/v1");
    expect(outcome.example).toBe("wall-roof-facade-strip");
    expect(outcome.seed).toBeGreaterThan(0);
  });

  it("is a non-empty problem over a real tile catalogue", () => {
    expect(outcome.slots).toBeGreaterThan(0);
    expect(outcome.edges).toBeGreaterThan(0);
    expect(outcome.tiles.length).toBeGreaterThan(1);
    expect(new Set(outcome.tiles).size).toBe(outcome.tiles.length);
  });

  it("assigns exactly its own slots, and only tiles it declares", () => {
    const assigned = Object.entries(outcome.assignments as Record<string, string>);
    expect(assigned).toHaveLength(outcome.slots);
    expect(outcome.satisfiable).toBe(assigned.length > 0);
    for (const [slot, tile] of assigned) {
      expect(outcome.tiles, `slot ${slot} is assigned an undeclared tile ${tile}`).toContain(tile);
    }
  });

  it("lists its tiles in canonical ascending order", () => {
    expect(outcome.tiles).toEqual([...outcome.tiles].sort());
  });
});
