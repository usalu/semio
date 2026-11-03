import { readFileSync, writeFileSync } from "node:fs";
const root = process.argv[2]!;
const edit = (file: string, pairs: readonly (readonly [string, string])[]): void => {
  let t = readFileSync(`${root}/${file}`, "utf8");
  for (const [from, to] of pairs) {
    if (!t.includes(from)) throw new Error(`missing in ${file}: ${from.slice(0, 70)}`);
    t = t.split(from).join(to);
  }
  writeFileSync(`${root}/${file}`, t);
};
edit("🔨️modules/🎨️depiction/✂️section/🟦️.ts", [
  ["const CAPTIONS: Readonly<Record<Viewpoint, Text>>", "/** 🏷️ The one name of each viewpoint in captions, sheets, viewer and validator messages (terminology R1): Side elevation / Seitenansicht, Front elevation / Vorderansicht, Plan (top view) / Draufsicht. */\nexport const VIEWPOINT_CAPTIONS: Readonly<Record<Viewpoint, Text>>"],
  ["const SECTION_NAMES: Readonly<Record<\"front\" | \"plan\", Text>>", "/** 🏷️ The one name of a cut: Section / Schnitt for the front elevation, Plan section / Grundriss-Schnitt for the plan. */\nexport const SECTION_CAPTIONS: Readonly<Record<\"front\" | \"plan\", Text>>"],
  ["CAPTIONS[viewpoint][language]", "VIEWPOINT_CAPTIONS[viewpoint][language]"],
  ["SECTION_NAMES[viewpoint", "SECTION_CAPTIONS[viewpoint"],
]);
edit("🔨️modules/🗂️catalog/🟦️.ts", [
  ["const VIEWPOINT_NAMES: Readonly<Record<Viewpoint, Text>> = { elevation: { en: \"side elevation\", de: \"Seitenansicht\" }, front: { en: \"front elevation\", de: \"Vorderansicht\" }, plan: { en: \"plan (top view)\", de: \"Draufsicht\" } };", "const VIEWPOINT_NAMES: Readonly<Record<Viewpoint, Text>> = Object.fromEntries(VIEWPOINTS.map((viewpoint) => [viewpoint, { en: VIEWPOINT_CAPTIONS[viewpoint].en.toLowerCase(), de: VIEWPOINT_CAPTIONS[viewpoint].de }])) as Record<Viewpoint, Text>;"],
  ['import type { EvaluatedState } from "../🔗️contracts/🟦️.ts";', 'import type { EvaluatedState } from "../🔗️contracts/🟦️.ts";\nimport { VIEWPOINT_CAPTIONS } from "../🎨️depiction/✂️section/🟦️.ts";'],
]);
edit("📦️packages/🟦️typescript/🟦️.ts", [["export { sectionMarker, sectionOf, viewpointCaption }", "export { SECTION_CAPTIONS, VIEWPOINT_CAPTIONS, sectionMarker, sectionOf, viewpointCaption }"]]);
