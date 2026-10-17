/** 🧪 Probe: a literal new URL(path, import.meta.url) versus fixtureUrl under vitest jsdom. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { fixtureUrl } from "../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/📖️source-text/🟦️.ts";
import { rasterOwnershipFixtureUrl } from "../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts";

describe("jsdom url probe", () => {
  it("literal new URL form", () => {
    const url = new URL("../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🖼️scene-raster-ownership/🔣️.json", import.meta.url);
    console.log("[DEBUG] literal href", url.href.slice(0, 60), url.protocol, typeof document);
    expect(url.protocol).toBe("file:");
    expect(readFileSync(url, "utf8").length).toBeGreaterThan(10);
  });
  it("fixtureUrl form and the ui entry export", () => {
    const url = fixtureUrl(import.meta.url, "../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🖼️scene-raster-ownership/🔣️.json");
    expect(readFileSync(url, "utf8").length).toBeGreaterThan(10);
    expect(readFileSync(rasterOwnershipFixtureUrl, "utf8")).toBe(readFileSync(url, "utf8"));
    console.log("[DEBUG] entry href", rasterOwnershipFixtureUrl.protocol, rasterOwnershipFixtureUrl.constructor.name);
  });
});
