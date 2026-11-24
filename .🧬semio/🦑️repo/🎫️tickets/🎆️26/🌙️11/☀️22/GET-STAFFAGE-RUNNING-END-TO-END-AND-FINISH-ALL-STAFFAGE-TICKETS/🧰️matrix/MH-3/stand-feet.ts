/** 🦶️ Splits the lumped contact `stand-foot` of `person-pushing-bicycle` into the two feet of the centre stand (D45): `bun stand-feet.ts <id|file.json> [--write] [--out <file>]`. The stand has two legs 0.09 m either side of the frame plane; the one anchor without `z` lay on the centre line between them, 79 mm off both feet in the front and in the plan. `stand-foot-l` and `stand-foot-r` take its place at the lateral position of their foot with the width of one foot; the pose and clip contacts name both, the clip event that targeted the anchor targets the left foot. A second run changes nothing. It runs before `fit.ts` in the chain of the asset. */
import { readDoc, writeDoc } from "../MR/lib.ts";

const args = process.argv.slice(2);
const out = args.includes("--out") ? args[args.indexOf("--out") + 1] : undefined;
const read = readDoc(args[0]!);
const asset = read.doc.asset;
const index = asset.anchors.findIndex((anchor: any) => anchor.id === "stand-foot");
if (index >= 0) {
  const lumped = asset.anchors[index];
  const foot = (side: "l" | "r"): any => ({ ...lumped, id: `stand-foot-${side}`, title: side === "l" ? { en: "Left stand foot", de: "Linker Ständerfuß" } : { en: "Right stand foot", de: "Rechter Ständerfuß" }, footprintWidthM: 0.022, z: side === "l" ? 0.09 : -0.09 });
  asset.anchors.splice(index, 1, foot("l"), foot("r"));
  for (const pose of asset.poses) if (Array.isArray(pose.contacts)) pose.contacts = pose.contacts.flatMap((id: string) => (id === "stand-foot" ? ["stand-foot-l", "stand-foot-r"] : [id]));
  for (const clip of asset.clips) {
    if (Array.isArray(clip.contacts)) clip.contacts = clip.contacts.flatMap((contact: any) => (contact.anchor === "stand-foot" ? [{ ...contact, anchor: "stand-foot-l" }, { ...contact, anchor: "stand-foot-r" }] : [contact]));
    for (const event of clip.events ?? []) if (event.target === "stand-foot") event.target = "stand-foot-l";
  }
}
console.log(index >= 0 ? "stand-foot split into stand-foot-l and stand-foot-r" : "unchanged: no lumped stand-foot");
if (args.includes("--write") || out !== undefined) console.log(writeDoc(read, out) ? `written: ${out ?? read.file}` : "file unchanged");
