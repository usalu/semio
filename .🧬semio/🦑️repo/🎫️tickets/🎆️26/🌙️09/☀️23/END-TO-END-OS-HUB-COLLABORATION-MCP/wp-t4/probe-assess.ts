import { assessTestImplementationPath, testTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const base = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/";
for (const p of ["🖌️update-part2d/🧪️tests/🧪️circle-to-rectangle/🦀️.rs", "📐change-part-kind-unit/🧪️tests/🧪️switches-unit-to-centimeter/🦀️.rs", "📍move-grip2d/🧪️tests/🧪️swings-north-grip-along-the-rim/🦀️.rs"]) console.log(p, JSON.stringify(assessTestImplementationPath(base + p, testTaxonomy(root))));
