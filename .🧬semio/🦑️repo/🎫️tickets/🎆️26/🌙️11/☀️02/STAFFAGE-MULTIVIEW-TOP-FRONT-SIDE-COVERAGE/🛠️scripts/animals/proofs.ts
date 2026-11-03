import { catalog, authoredScales } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
const [, , out, png, ...ids] = process.argv;
let n = 0, fail = 0;
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const clip = a.clips.find((c) => c.id === a.presentation.defaultClip) ?? a.clips[0];
  const jobs: string[][] = [];
  for (const d of [50, 200].filter((x) => authoredScales(a).includes(x))) {
    const base = ["--asset", a.id, "--scale", String(d), "--views", "--projection", "third", "--out", out, ...(png === "png" ? [] : ["--no-png"])];
    jobs.push([...base, "--pose", a.presentation.defaultPose]);
    if (clip) for (const f of [0, 0.5]) jobs.push([...base, "--clip", clip.id, "--time", String(+(clip.durationS * f).toFixed(3))]);
  }
  for (const j of jobs) { try { await runPreview(j); n++; } catch (e) { fail++; console.log("FAIL", a.id, j.slice(2, 8).join(" "), String((e as Error).message).slice(0, 160)); } }
}
console.log("renders", n, "failures", fail);
