/** 🧪️ Ticket tool of work package H3: writes a copy of a species document with extra clips that lay a state's overlay clip under another clip (the union of their tracks; a bone channel both animate is refused), so the preview tool can show what the stage composes — e.g. a trick played while a state is on.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/h3_overlay.ts" <species.json> <out.json> <overlay>+<clip> [<overlay>+<clip> ...]
 * The new clips are called `<overlay>-under-<clip>`, last as long as `<clip>` and loop when it does.
 */
import { readFileSync, writeFileSync } from "node:fs";
import type { Clip, Species } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";

const [source, target, ...pairs] = process.argv.slice(2);
const species = JSON.parse(readFileSync(source!, "utf8")) as Species;
const clipOf = (id: string): Clip => {
  const found = species.clips.find((clip) => clip.id === id);
  if (found === undefined) throw new Error(`no clip ${id}`);
  return found;
};
const added: Clip[] = [];
for (const pair of pairs) {
  const [overlay, top] = pair.split("+").map(clipOf) as [Clip, Clip];
  const taken = new Set(top.tracks.map((track) => `${track.bone}.${track.channel}`));
  const clash = overlay.tracks.filter((track) => taken.has(`${track.bone}.${track.channel}`));
  if (clash.length > 0) throw new Error(`${pair}: both animate ${clash.map((track) => `${track.bone}.${track.channel}`).join(", ")}`);
  added.push({ id: `${overlay.id}-under-${top.id}`, seconds: top.seconds, loop: top.loop, tracks: [...overlay.tracks, ...top.tracks] });
}
writeFileSync(target!, JSON.stringify({ ...species, clips: [...species.clips, ...added] }, null, 2));
console.log(`wrote ${target} with ${added.map((clip) => clip.id).join(", ")}`);
