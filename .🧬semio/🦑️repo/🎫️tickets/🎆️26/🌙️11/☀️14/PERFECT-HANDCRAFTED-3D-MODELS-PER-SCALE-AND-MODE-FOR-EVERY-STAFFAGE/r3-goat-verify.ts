/** 🩺️ Round 3 goat: compares the contact anchors of every pose between the catalog rig and the working rig and runs the catalog validator on the working asset. Usage: bun r3-goat-verify.ts [--issues] */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { entryOf, type Json } from "./volumes-lib.ts";
import { SPEC, working } from "./r3-goat-work.ts";

const before: Json = entryOf(SPEC).asset;
const after: Json = working();
const view = { kind: "scale", scale: { denominator: 50 } };
let worst = 0;
for (const pose of after.poses as Json[]) {
  const a: Json = evaluate(before as never, { kind: "pose", pose: pose.id } as never, view as never);
  const b: Json = evaluate(after as never, { kind: "pose", pose: pose.id } as never, view as never);
  const rows: string[] = [];
  for (const id of ["contact-hn", "contact-hf", "contact-fn", "contact-ff"]) {
    const p = a.anchors[id]?.at;
    const q = b.anchors[id]?.at;
    if (p === undefined || q === undefined) continue;
    const d = Math.hypot(p.x - q.x, p.y - q.y);
    if (pose.contacts.includes(id)) worst = Math.max(worst, d);
    rows.push(`${id}${pose.contacts.includes(id) ? "*" : ""} (${q.x.toFixed(3)},${q.y.toFixed(3)}) d=${d.toFixed(4)}`);
  }
  console.log(pose.id, rows.join("  "));
}
console.log("worst planted contact shift", worst.toFixed(5));
if (process.argv.includes("--issues")) {
  const issues = validateAsset(after as never) as Json[];
  const by = new Map<string, Json[]>();
  for (const i of issues) by.set(i.code, [...(by.get(i.code) ?? []), i]);
  console.log(`validateAsset: ${issues.length} issue(s)`);
  for (const [code, list] of by) console.log("  ", code, list.length, "|", list.slice(0, 3).map((i) => `${i.path}: ${i.message.en.slice(0, 220)}`).join(" || "));
}
