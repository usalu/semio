import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const [asset, out, ...jobs] = process.argv.slice(2);
for (const job of jobs) {
  const [scale, kind, rest] = job.split(":");
  const args = ["--asset", asset!, "--scale", scale!, "--views", "--view", "light-line", "--dpi", "200", "--out", out!];
  if (kind === "pose") args.push("--pose", rest!);
  else if (kind === "clip") {
    const [clip, time] = rest!.split("@");
    args.push("--clip", clip!, "--time", time!);
  }
  try {
    for (const path of await runPreview(args)) if (path.endsWith(".png")) console.log(path);
  } catch (error) {
    console.log(`[DEBUG] failed ${job}: ${(error as Error).message}`);
  }
}
