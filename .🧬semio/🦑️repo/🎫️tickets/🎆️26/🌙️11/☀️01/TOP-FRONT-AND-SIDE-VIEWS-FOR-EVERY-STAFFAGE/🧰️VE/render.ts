import { readFileSync } from "node:fs";
import { runPreview, type Rasterizer } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const none: Rasterizer = { rasterize() {} };
const jobs = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { argv: string[] }[];
for (const job of jobs) {
  try {
    const written = await runPreview([...job.argv, "--no-png"], none);
    console.log("[DEBUG] ok", written.join(" "));
  } catch (error) {
    console.log("[DEBUG] fail", job.argv.join(" "), String(error).split("\n")[0]);
  }
}
