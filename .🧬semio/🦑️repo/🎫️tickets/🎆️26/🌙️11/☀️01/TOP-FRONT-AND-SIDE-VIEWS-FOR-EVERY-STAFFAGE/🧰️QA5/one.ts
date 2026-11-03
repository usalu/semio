import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
const out = process.env.QA5_OUT!;
for (const scale of ["50", "100"]) for (const pose of ["supine", "side", "curled"]) for (const vp of ["elevation", "front", "plan"]) {
  try { const r = await runPreview(["--asset", "sleeping-person", "--scale", scale, "--viewpoint", vp, "--no-png", "--pose", pose, "--out", `${out}/sleeping-person`]); console.log("[DEBUG]", scale, pose, vp, JSON.stringify(r)?.slice(-90)); } catch (e) { console.log("[DEBUG] ERR", scale, pose, (e as Error).message); }
}
