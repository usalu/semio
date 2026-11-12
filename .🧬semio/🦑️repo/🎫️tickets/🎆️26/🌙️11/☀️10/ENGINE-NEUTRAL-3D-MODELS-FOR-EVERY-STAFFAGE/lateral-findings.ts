import { readFileSync } from "node:fs";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
for (const slug of ["pregnant-person", "sleeping-person", "tour-guide"]) {
  const e = catalogEntries().find((x) => x.slug === slug)!;
  const asset = loadAsset(e.path);
  const issues = validateAsset(asset, {});
  console.log(slug, issues.length);
  for (const i of issues) console.log("  ", i.code, JSON.stringify(i.data));
}
