import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import postcss from "postcss";
import { prepareStorybookAssets } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🧰️preparation/🟦️.ts";
import { renderStylingArtifacts } from "../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts";
import { repoRootFromHere } from "../../📖️stories/🧭️coordination/🟦️.ts";

const root = repoRootFromHere();

describe("Storybook generated browser prerequisites", () => {
  it("respects cancellation before any publication", () => {
    expect(() => prepareStorybookAssets(() => { throw new Error("cancelled"); })).toThrow("cancelled");
  });

  it("prepares actual catalogs repeatably and validates emitted CSS independently", () => {
    let checks = 0;
    prepareStorybookAssets(() => { checks++; });
    const tokens = resolve(root, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🔤️tokens/🟦️.ts");
    const icons = resolve(root, "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🟦️.ts");
    const before = [readFileSync(tokens, "utf8"), readFileSync(icons, "utf8")];
    prepareStorybookAssets(() => { checks++; });
    expect([readFileSync(tokens, "utf8"), readFileSync(icons, "utf8")]).toEqual(before);
    expect(checks).toBeGreaterThan(8);
    expect(before[0]).toContain("STYLING_TOKENS");
    expect(before[1]).toContain("ICONS");
    for (const artifact of renderStylingArtifacts().filter(artifact => artifact.path.endsWith(".css"))) {
      expect(readFileSync(artifact.path, "utf8")).toBe(artifact.content);
      expect(postcss.parse(artifact.content).nodes.length).toBeGreaterThan(0);
    }
  });
});
