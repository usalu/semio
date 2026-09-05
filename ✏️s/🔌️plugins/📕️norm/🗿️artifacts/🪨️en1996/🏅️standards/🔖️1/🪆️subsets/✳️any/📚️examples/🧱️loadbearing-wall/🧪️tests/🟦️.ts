import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
const here = dirname(fileURLToPath(import.meta.url));
describe("loadbearing-wall", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(here, "../🖼️assets/🧱️loadbearing-wall/🗣️.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
});
