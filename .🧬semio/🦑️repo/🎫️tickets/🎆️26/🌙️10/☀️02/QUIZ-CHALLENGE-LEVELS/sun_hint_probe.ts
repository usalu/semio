/** 🌞️ Probes the easy hint of the physics `powers` sorting when its largest drawn item stands on the smallest key: the core's `hintsOf` over sheets that deal the Sun and over sheets that do not, and whether a sheet survives a JSON round trip unchanged. */
import { readFileSync } from "node:fs";
import { hintsOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/⛰️challenge/🟦️.ts";
import { sheetOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/🃏️sheet/🟦️.ts";

const physics = JSON.parse(readFileSync(new URL("../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json", import.meta.url), "utf8"));
const task = physics.tasks.find((candidate: { id: string }) => candidate.id === "powers");
const counts = { sun: 0, sunHinted: 0, other: 0, otherHinted: 0, roundTripDiffers: 0 };
for (let seed = 1; seed <= 400; seed++) {
  const sheet = sheetOf(physics, seed, "easy");
  const sheetTask = sheet.tasks.find((candidate) => candidate.id === "powers")!;
  const ids = sheetTask.items.map((item) => item.id);
  const value = (id: string): number => task.items.find((item: { id: string }) => item.id === id).value;
  const order = [...ids].sort((left, right) => value(left) - value(right));
  const largest = order[order.length - 1]!;
  const hints = hintsOf(task, sheetTask, { kind: "sorting", order: [largest, ...order.slice(0, -1)] });
  const hinted = hints.some((hint) => hint.kind === "magnitude" && hint.item === largest && hint.direction === "low");
  if (largest === "sun") {
    counts.sun++;
    if (hinted) counts.sunHinted++;
  } else {
    counts.other++;
    if (hinted) counts.otherHinted++;
  }
  if (JSON.stringify(JSON.parse(JSON.stringify(sheet))) !== JSON.stringify(sheet)) counts.roundTripDiffers++;
  if (seed === 1 || (largest === "sun" && counts.sun === 1)) console.log("[DEBUG]", seed, largest, JSON.stringify(sheetTask.keys), JSON.stringify(hints));
}
console.log("[DEBUG]", JSON.stringify(counts));
