import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/O2";
const jobs: Record<string, { scales: string[]; whens: string[] }> = JSON.parse(process.argv[2]!);
for (const [id, job] of Object.entries(jobs)) {
  for (const scale of job.scales) {
    for (const when of job.whens) {
      const [kind, name, time] = when.split(":");
      const args = ["--file", `${S}/🗂️catalog/🪑️objects/${id}/🔣️.json`, "--scale", scale, "--views", "--no-png", "--out", `${OUT}/${id}`];
      if (kind === "clip") args.push("--clip", name!, "--time", time ?? "0");
      else args.push("--pose", name!);
      try {
        await runPreview(args);
      } catch (error) {
        console.log("[DEBUG] render failed", id, scale, when, (error as Error).message);
      }
    }
  }
  console.log("[DEBUG] rendered", id);
}
