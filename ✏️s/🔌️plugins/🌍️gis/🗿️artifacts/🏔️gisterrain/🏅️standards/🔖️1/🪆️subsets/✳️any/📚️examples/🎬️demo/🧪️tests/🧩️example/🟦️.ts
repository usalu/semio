import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "../../../../../../../../../../../../../🧪️tests/🗿️artifact-runner/🟦️.ts";
const here = dirname(fileURLToPath(import.meta.url));
describe("demo", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(here, "../../../../🖼️assets/🎬️demo/🗣️.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
});
