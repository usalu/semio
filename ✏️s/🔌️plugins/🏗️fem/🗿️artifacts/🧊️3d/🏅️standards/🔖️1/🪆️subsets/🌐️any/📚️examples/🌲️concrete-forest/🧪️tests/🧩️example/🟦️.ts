import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));

describe("example 🌲️concrete-forest", () => {
  const text = readFileSync(join(here, "../../../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio"), "utf8");

  it("ships the stacked two-piece frame asset", () => {
    expect(text.startsWith("semio fem.fem3d.dsl v1")).toBe(true);
    expect(text.match(/^\s+frame id=/gm)?.length).toBe(20);
    expect(text.match(/^\s+[lu](c[12][bt]|v[0-6]) /gm)?.length).toBe(20);
    expect(text.includes("solid ")).toBe(false);
  });

  it("stacks the upper piece on the lower c-t vortices", () => {
    expect(text).toContain("lc1t 2.70001 2.338269 3");
    expect(text).toContain("uc1t 2.70001 2.338269 6");
    expect(text).toContain("frame id=u_col1 start=lc1t end=uc1t");
  });
});
