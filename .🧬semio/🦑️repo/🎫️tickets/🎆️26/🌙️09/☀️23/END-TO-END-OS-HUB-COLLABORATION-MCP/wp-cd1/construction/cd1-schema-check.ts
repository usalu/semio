/** 🧪️ CD1: validates every typology asset of a tree against the `spatial.typology` schema (AJV 2020) + the census rule. Usage: bun cd1-schema-check.ts <tree-root> */
import Ajv2020 from "ajv/dist/2020.js";
import { readFileSync } from "node:fs";
import { Glob } from "bun";
const root = process.argv[2]!;
const base = `${root}/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any`;
const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(`${base}/✏️editor/⚙️engine/🧬️typology/🧬️schema/🔣️.json`, "utf8")));
let n = 0;
const bad: string[] = [];
for (const file of new Glob("📚️examples/🖼️assets/🏗️modelDefinitions/**/🗂️typologies/**/🔣️typology.json").scanSync(base)) {
  const asset = JSON.parse(readFileSync(`${base}/${file}`, "utf8"));
  n++;
  if (!validate(asset)) bad.push(`${asset.id}: ${JSON.stringify(validate.errors)}`);
  const twoPoint = (asset.actions as string[]).some((a) => a.endsWith("From2PointsAndHeight"));
  if (twoPoint !== Boolean(asset.construction)) bad.push(`${asset.id}: construct action ${twoPoint} vs construction ${Boolean(asset.construction)}`);
  if (asset.construction && !asset.primitiveKinds.includes(asset.construction.from2PointsAndHeight.primitive)) bad.push(`${asset.id}: construction primitive outside primitiveKinds`);
}
console.log(JSON.stringify({ assets: n, bad }, null, 1));
