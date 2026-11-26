/** 🐿️ The closing repair of `animals/red-squirrel` (idempotent): `bun squirrel.ts <in.json|red-squirrel> <out.json>` puts the guard locks of the brush back into the body and thermal lists of 1:100, 1:200 and 1:500 (28, 18 and 12 locks, pin `squirrelBrush.guardLocks` of `🧊️model-skeleton`), each after the tail link of its bone and each with its `tail-fur` surface, as `./brush-locks.json` holds them: the locks of the state of 2026-11-22 13:42, every one a `texture` loft whose stations are stations of its 1:50 lock. The pass of 2026-11-22 17:15 had taken them out as "below the feature"; a `texture` solid is not judged by the feature. The body decision of each scale says what the list holds.
 * @see ./brush-locks.json */
import { readFileSync } from "node:fs";
import { readDoc, writeDoc } from "../MR/lib.ts";

const [source, out] = process.argv.slice(2);
const read = readDoc(source!);
const asset = read.doc.asset;
type Entry = { id: string; bone?: string; of?: string };
const locks = JSON.parse(readFileSync(new URL("./brush-locks.json", import.meta.url), "utf8")) as Record<string, { body: Entry[]; thermal: Entry[] }>;
const BRUSH = [
  [/the brush is six links without single hair locks, which are 7 mm wide and far below the feature/, "the brush is six links with a few hair locks that carry the fringe of its outline"],
  [/der Schwanz besteht aus sechs Gliedern ohne einzelne Haarsträhnen, die mit 7 mm Breite weit unter dem kleinsten Merkmal liegen/, "der Schwanz besteht aus sechs Gliedern mit wenigen Haarsträhnen, die den Saum seines Umrisses tragen"],
] as const;
for (const [scale, add] of Object.entries(locks)) {
  const volume = asset.volumes[scale];
  const have = volume.body.filter((s: Entry) => s.id.startsWith("brush-lock-")).length;
  if (have === 0) {
    for (const lock of add.body) {
      const group = volume.body.filter((s: Entry) => s.bone === lock.bone && (s.id === lock.bone || s.id.startsWith("brush-lock-")));
      volume.body.splice(volume.body.indexOf(group.at(-1)) + 1, 0, structuredClone(lock));
    }
    for (const surface of add.thermal) {
      const bone = add.body.find((s) => s.id === surface.of)!.bone;
      const group = volume.thermal.filter((s: Entry) => s.of === bone || add.body.some((l) => l.id === s.of && l.bone === bone));
      volume.thermal.splice(volume.thermal.indexOf(group.at(-1)) + 1, 0, structuredClone(surface));
    }
  } else if (have !== add.body.length) throw new Error(`1:${scale} holds ${have} locks, neither none nor ${add.body.length}`);
  const text = volume.decisions[0];
  text.en = text.en.replace(/: \d+ solids\./, `: ${volume.body.length} solids.`).replace(BRUSH[0][0], BRUSH[0][1]);
  text.de = text.de.replace(/: \d+ Körper\./, `: ${volume.body.length} Körper.`).replace(BRUSH[1][0], BRUSH[1][1]);
  console.log(`1:${scale}: ${have === 0 ? "added" : "kept"} ${add.body.length} locks; body ${volume.body.length}, thermal ${volume.thermal.length}, structure ${volume.structure.length}`);
}
console.log(writeDoc(read, out) ? `written ${out}` : "unchanged");
