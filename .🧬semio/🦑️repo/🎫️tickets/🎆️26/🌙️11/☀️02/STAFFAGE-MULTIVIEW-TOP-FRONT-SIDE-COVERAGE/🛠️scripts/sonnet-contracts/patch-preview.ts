import { readFileSync, writeFileSync } from "node:fs";
const edits: readonly (readonly [string, readonly (readonly [string, string])[]])[] = [
  [process.argv[2]!, [
    ["`--viewpoint` draws the side (`elevation`), `front` or top (`plan`) view; `--views --projection first|third` writes one sheet per view in", "`--viewpoint` draws the side elevation (`elevation`), the front elevation (`front`) or the plan (top view, `plan`); `--views --projection first|third` writes one sheet per representation in"],
    ["with the three views of the same pose or clip instant at the same scale.", "with the three views (side elevation, front elevation, plan) of the same pose or clip instant at the same scale."],
    ["in any of the seven views to SVG and PNG, a contact sheet of all poses × views,", "in any of the seven representations to SVG and PNG, a contact sheet of all poses × representations,"],
  ]],
  [process.argv[3]!, [
    ["Per-section", "Per-section"],
  ]],
];
for (const [file, pairs] of edits.slice(0, 1)) {
  let t = readFileSync(file, "utf8");
  for (const [from, to] of pairs) {
    if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 60)}`);
    t = t.replace(from, to);
  }
  writeFileSync(file, t);
}
