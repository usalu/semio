import { readFileSync, writeFileSync } from "node:fs";
const root = process.argv[2]!;
const edit = (file: string, pairs: readonly (readonly [string, string])[]): void => {
  let text = readFileSync(`${root}/${file}`, "utf8");
  for (const [from, to] of pairs) {
    if (!text.includes(from)) throw new Error(`missing in ${file}: ${from.slice(0, 60)}`);
    text = text.replace(from, to);
  }
  writeFileSync(`${root}/${file}`, text);
};
edit("🧬️schema/🟦️.ts", [
  ["How a shape of any view is a solid of its rig-plane profile, authored in the bone's local frame like an elevation shape and placed with the full 3D frame of its bone (spread, twist and sagittal rotation) and projected to the view (elevation, front or plan), so one 3D definition reads right in all three views", "How a shape of any viewpoint is a solid of its rig-plane profile, authored in the bone's local frame like a side elevation shape and placed with the full 3D frame of its bone (spread, twist and sagittal rotation) and projected to the viewpoint (side elevation, front elevation or plan), so one 3D definition reads right in all three views"],
  ["about the vertical axis through the origin of `axisBone` at elevation x `axisX` (lateral `axisZ`, default 0) in the rest pose;", "about an axis through the rest origin of `axisBone`, offset by `axisX` ahead and `axisZ` laterally (default 0) in the bone-local frame and carried with that bone; `axisBone` is the shape's own bone or one of its ancestors, so the solid moves rigidly with its bone;"],
]);
edit("README.md", [
  ["`solid` is for front and plan shapes only and excludes `followTip`;", "`solid` is allowed in every viewpoint and excludes `followTip`;"],
  ["World x right, y up, ground at y = 0;", "World frame: x ahead, y up, z left of the asset (a left-handed triple), ground at y = 0; the side elevation is seen from the asset's right with ahead on the paper's right, the front elevation from ahead (the asset's left on the paper's right), the plan (top view) from above (the asset's left up the page);"],
]);
