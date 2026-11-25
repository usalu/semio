/** ✂️ Section label: a section is named by its letters, the same in every language, so the drawing lies at the same place on the paper in English and in German.
 *
 * THE REFERENCE is svgelements (`svgShapeBoxes` of 🧪️tests/🧰️support/🐍️.ts): where the paths of the written SVG (body, marker and dimension lines; the lettering itself is text) land in each language.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/✂️section-label/🔣️.json — the vectors
 * @see ../../🔨️modules/🗂️catalog/🟦️.ts — the rule `section-label`
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AssetDocument, Language, Text } from "../../🧬️schema/🟦️.ts";
import { SECTION_LABEL, validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool, withScratch } from "../🧰️support/🧰️.ts";
import { svgShapeBoxes } from "../🧰️support/🐍️.ts";

type Vectors = { asset: string; scale: number; section: "front"; host: "elevation"; tolerancePaperMm: number; apartPaperMm: number; labels: (Text & { reported: boolean })[]; letters: Text; phrase: Text };
const vectors = fixture<Vectors>("✂️section-label");
const source = (JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🗂️catalog/🪑️objects", vectors.asset, "🔣️.json"), "utf8")) as AssetDocument).asset;
const named = (label: Text): Asset => {
  const asset = structuredClone(source) as Asset;
  for (const representation of Object.values(asset.representations)) if (representation.status === "authored" && representation.sections?.[vectors.section] !== undefined) (representation.sections[vectors.section] as { label: Text }).label = { en: label.en, de: label.de };
  return asset;
};

describe("✂️ the name of a section", () => {
  it("matches one or two capital letters", () => {
    for (const label of vectors.labels) expect(SECTION_LABEL.test(label.en) && label.en === label.de, `${label.en} / ${label.de}`).toBe(!label.reported);
  });

  vectors.labels.forEach((label) =>
    it(`${label.reported ? "reports" : "accepts"} "${label.en}" / "${label.de}"`, { timeout: 600_000 }, () => {
      const issues = validateAsset(named(label)).filter((issue) => issue.code === "section-label");
      expect(issues.length > 0, JSON.stringify(issues.slice(0, 1))).toBe(label.reported);
      for (const issue of issues) expect(issue.message.en.length > 0 && issue.message.de.length > 0).toBe(true);
    }),
  );

  it("keeps every drawn path at the same paper position in both languages exactly when the section is named by letters, as svgelements reads it", { timeout: 600_000 }, async () => {
    requireTool("uv");
    await withScratch((directory) => {
      const spread = (label: Text): number => {
        const boxes = (["en", "de"] as const).map((language: Language) => {
          const selector = { kind: "scale", scale: { denominator: vectors.scale } } as const;
          const asset = named(label);
          const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
          const file = join(directory, `${language}-${label.en.length}.svg`);
          writeFileSync(file, writeSvg(depict(state, { ...defaultDepictionOptions("light-line", language, vectors.scale, vectors.host), representation: selector })));
          return svgShapeBoxes(file).filter((box) => box.id !== "staffage-background");
        });
        expect(boxes[0]!.length).toBeGreaterThan(0);
        expect(boxes[1]!.map((box) => box.id)).toEqual(boxes[0]!.map((box) => box.id));
        return Math.max(...boxes[0]!.map((box, i) => Math.max(Math.abs(box.minX - boxes[1]![i]!.minX), Math.abs(box.minY - boxes[1]![i]!.minY), Math.abs(box.maxX - boxes[1]![i]!.maxX), Math.abs(box.maxY - boxes[1]![i]!.maxY))));
      };
      expect(spread(vectors.letters)).toBeLessThanOrEqual(vectors.tolerancePaperMm);
      expect(spread(vectors.phrase)).toBeGreaterThan(vectors.apartPaperMm);
    });
  });
});
