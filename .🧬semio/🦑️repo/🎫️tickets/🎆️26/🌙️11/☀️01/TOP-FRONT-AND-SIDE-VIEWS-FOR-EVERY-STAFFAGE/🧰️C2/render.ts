import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { readFileSync } from "node:fs";

const jobs: string[][] = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const job of jobs) {
  try {
    for (const p of await runPreview(job)) if (p.endsWith(".png")) console.log("[DEBUG] " + p);
  } catch (e) {
    console.log("[DEBUG] FAIL " + job.join(" ") + " :: " + (e as Error).message.slice(0, 300));
  }
}
