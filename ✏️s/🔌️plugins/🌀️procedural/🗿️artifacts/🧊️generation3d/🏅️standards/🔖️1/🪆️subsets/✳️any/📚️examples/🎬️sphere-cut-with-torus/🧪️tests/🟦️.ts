import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "../../../../../../../../../../../../🧪️artifact.ts";
const here = dirname(fileURLToPath(import.meta.url));
describe("sphere-cut-with-torus", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(here, "../🖼️assets/🗣️sphere-cut-with-torus.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
});
