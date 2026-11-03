import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/O2";
const [, , id, scalesArg, ...whens] = process.argv;
const scales = scalesArg!.split(",");
for (const scale of scales) {
  for (const when of whens) {
    const [kind, name, time] = when.split(":");
    const args = ["--file", `${S}/🗂️catalog/🪑️objects/${id}/🔣️.json`, "--scale", scale, "--views", "--no-png", "--out", `${OUT}/${id}`];
    if (kind === "clip") args.push("--clip", name!, "--time", time ?? "0");
    else args.push("--pose", name!);
    for (const p of await runPreview(args)) console.log(p);
  }
}
