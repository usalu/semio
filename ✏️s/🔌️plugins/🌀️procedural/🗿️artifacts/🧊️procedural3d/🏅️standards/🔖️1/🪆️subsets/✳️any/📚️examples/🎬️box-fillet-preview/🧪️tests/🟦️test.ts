import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
const here = dirname(fileURLToPath(import.meta.url));
describe("box-fillet-preview", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(here, "../🖼️assets/🗣️box-fillet-preview.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
});
