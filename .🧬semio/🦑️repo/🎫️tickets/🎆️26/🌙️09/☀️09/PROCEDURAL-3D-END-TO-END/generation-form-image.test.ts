import { expect, test } from "bun:test";
import { PNG } from "pngjs";

const fixture = await Bun.file("✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🧫️fixtures/form-image.json").json();
for (const [locale, row] of Object.entries(fixture) as [string, any][]) {
  test(locale, () => {
    const [image, absent] = row.steps[0].blocks;
    expect(new TextEncoder().encode(image.src).length).toBe(row.sourceBytes);
    expect(row.sourceBytes > 512).toBe(locale === "en");
    const source = new URL(image.src.trim());
    const imageBytes = Buffer.from(source.pathname.replace(/^image\/png;base64,/, ""), "base64");
    const png = PNG.sync.read(imageBytes);
    expect([png.width, png.height]).toEqual([1, 1]);
    expect(image.description?.trim() || image.label).toBe(row.alt);
    expect(absent.src?.trim() || row.fallback).toBe(row.fallback);
  });
}
