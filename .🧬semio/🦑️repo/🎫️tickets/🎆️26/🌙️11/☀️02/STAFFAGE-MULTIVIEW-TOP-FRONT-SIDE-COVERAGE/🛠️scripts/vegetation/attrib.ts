import { execSync } from "node:child_process";
import { L, evaluateMod } from "./lib.ts";
for (const d of ["date-palm", "hosta-clump", "agave"]) {
  const head = JSON.parse(execSync(`git -C C:/git/semio show "HEAD:🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/${d}/🔣️.json"`, { maxBuffer: 1e9, encoding: "utf8" })).asset;
  const cur = JSON.parse(require("node:fs").readFileSync(`${L}/🗂️catalog/🌳️vegetation/${d}/🔣️.json`, "utf8")).asset;
  for (const [name, a] of [["HEAD", head], ["current", cur]] as const) {
    const res: string[] = [];
    for (const den of [50, 100, 200, 500]) { if (a.representations[String(den)].status !== "authored") continue; try { evaluateMod.evaluate(structuredClone(a), { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: den } }); res.push(`${den}:ok`); } catch (e: any) { res.push(`${den}:FAIL(${e.message.slice(0, 60)})`); } }
    console.log(d, name, res.join(" "));
  }
}
