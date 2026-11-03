import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const sub = (from: string, to: string): void => {
  if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 60)}`);
  t = t.replace(from, to);
};
sub("- `revolved`: axis through the rest origin of `axisBone`, offset by `axisX` ahead and `axisZ` laterally in the bone-local frame, carried with that bone. `axisBone`", "- `revolved`: the axis passes through the point translated from the rest origin of `axisBone` by `axisX` along world x and `axisZ` along world z at rest (not along the bone's `u` and `w`), in the bone-local direction `axis` (`u`, `v` or `w`), and is then carried with the bone frame. `axisBone`");
sub("## 10. Inconsistencies found", `## 10. Sheet arrangement (three views on one sheet)

The sheet (\`🔨️modules/📤️export/📑️sheet\`) composes the plan, the front elevation and the side elevation of ONE evaluated state at one paper scale as a plain \`DrawingScene\`.
- The side elevation is the main view. The plan shares its paper x (world x) with it; the front shares its paper y (the ground line) with it.
- Third-angle (ISO 5456-2 method 3, ANSI): plan above the side elevation, front elevation to its right. First-angle (method 1, DIN, SN): plan below, front to its left. The projection is always named on the sheet (\`--projection first|third\`), never defaulted.
- Because the front is seen from ahead, the asset's left is on its paper right, and in the plan the asset's left is up the page: the front's paper x and the plan's paper up both run along +z.
- Each view carries its own overall dimension measured on the drawn figure without ground, the caption of its viewpoint, the scale bar and the projection symbol.

## 11. Inconsistencies found`);
writeFileSync(file, t);
