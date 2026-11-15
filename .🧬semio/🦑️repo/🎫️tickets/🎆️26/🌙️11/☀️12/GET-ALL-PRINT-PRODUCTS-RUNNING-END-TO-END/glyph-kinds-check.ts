import { readFileSync } from "node:fs";
const c = JSON.parse(readFileSync("/Users/Shared/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🖌️kind-paint/🔣️.json", "utf8")), cat = JSON.parse(readFileSync("/Users/Shared/semio/🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json", "utf8"));
for (const g of c.glyphKinds) console.log(g.slug, cat.kinds.find((k: any) => k.slug === g.slug)?.family === g.family, readFileSync("/Users/Shared/semio/🧰️framework/🛍️products/📓️print/🖋️latex/" + g.source, "utf8").includes(g.definition));
