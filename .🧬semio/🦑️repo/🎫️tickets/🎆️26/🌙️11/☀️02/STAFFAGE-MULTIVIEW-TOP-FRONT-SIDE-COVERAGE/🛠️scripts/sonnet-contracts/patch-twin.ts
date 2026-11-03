import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const lines = readFileSync(file, "utf8").split("\n");
const swap = (prefix: string, to: string): void => {
  const i = lines.findIndex((l) => l.startsWith(prefix));
  if (i < 0) throw new Error(prefix);
  lines[i] = to;
};
swap(" * - Model space is a right-handed elevation plane", " * - World frame (asset-local and scene frame are one): x ahead, y up, z lateral with the left of the asset at +z, a left-handed triple; origin on the ground at the asset's reference point. The side elevation shows the x-y plane seen from the asset's right with ahead on the right of the paper.");
swap("/** 👁️ The projection an authored geometry set is drawn in.", "/** 👁️ The projection an authored geometry set is drawn in. `elevation` is the side elevation, the rig plane (paper x = world x ahead, paper up = y), seen from the asset's right (-z). `front` is the front elevation, seen from ahead (+x): paper x = world z (the asset's left is paper right), paper up = y. `plan` is the plan (top view), seen from above (+y): paper x = world x, paper up = world z (the asset's left is up the page). Front and plan shapes are authored in the rest pose and follow their bone through its full 3D frame. */");
swap("/** ✂️ A view drawn as a section:", "/** ✂️ A viewpoint drawn as a section: the cut plane lies at `plane` metres along the view axis (front: the forward position x of a plane across the forward axis, seen from ahead; plan: the height y of a horizontal plane, seen from above) and is named by `label` (such as A-A). The side elevation shows the cut-line marker at that plane; sheets and the viewer caption the cut \"Section A-A\" / \"Schnitt A-A\" (front) and \"Plan section B-B\" / \"Grundriss-Schnitt B-B\" (plan). */");
let text = lines.join("\n");
text = text.replace("whose orthographic projections are the elevation, the front and the plan.", "whose orthographic projections are the side elevation, the front elevation and the plan (top view).");
writeFileSync(file, text);
