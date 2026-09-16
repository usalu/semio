import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));

describe("example 🏠️house", () => {
  const text = readFileSync(join(here, "../../../../🖼️assets/🏠️house/🗣️.dsl.semio"), "utf8");

  it("ships the solids-only house asset", () => {
    expect(text.startsWith("semio fem.fem3d.dsl v1")).toBe(true);
    expect(text.includes("frame id=")).toBe(false);
    expect(text.includes("bar id=")).toBe(false);
    for (const solid of ["raft", "ground_slab", "wall_west", "wall_east", "wall_south", "wall_north", "attic_slab", "roof"]) {
      expect(text).toMatch(new RegExp(`^\\s+${solid} `, "m"));
    }
  });

  it("draws the walls in elevation and the roof as a swept section", () => {
    expect(text).toMatch(/^\s+wall_west .* masonry x$/m);
    expect(text).toMatch(/^\s+wall_south .* masonry y$/m);
    expect(text).toMatch(/^\s+roof .* timber y$/m);
    expect(text).toMatch(/^\s+raft .* concrete z$/m);
  });

  it("rests on a support grid under the raft", () => {
    expect(text.match(/^\s+s\d+_\d+ f\d+_\d+ \[ Tx Ty Tz \]$/gm)?.length).toBe(63);
  });
});
