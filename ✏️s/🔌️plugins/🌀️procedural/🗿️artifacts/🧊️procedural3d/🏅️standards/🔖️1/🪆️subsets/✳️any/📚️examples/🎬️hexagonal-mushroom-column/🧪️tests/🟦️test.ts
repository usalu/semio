import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
const here = dirname(fileURLToPath(import.meta.url));
describe("hexagonal-mushroom-column", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(here, "../🖼️assets/🗣️hexagonal-mushroom-column.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
});
