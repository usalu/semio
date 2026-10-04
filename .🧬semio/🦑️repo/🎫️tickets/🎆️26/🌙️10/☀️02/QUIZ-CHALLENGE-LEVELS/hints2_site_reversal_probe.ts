/** 🔁️ Over every draw of 10 of the 14 physics powers: how many items miss their key when the sorting is fully reversed
 * (a key off its value by more than the reach), so the spec knows whether a reversed sorting always exceeds the cap of 3. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const file = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "⚡️energy", "🧲️physics", "❓️quiz", "🔣️.json");
const task = JSON.parse(readFileSync(file, "utf8")).tasks.find((candidate: { id: string }) => candidate.id === "powers");
const values: number[] = task.items.map((item: { value: number }) => item.value);
const draw: number = task.draw;
const counts = new Map<number, number>();
function* choose(start: number, left: number, picked: number[]): Generator<number[]> {
  if (left === 0) return yield picked;
  for (let index = start; index <= values.length - left; index++) yield* choose(index + 1, left - 1, [...picked, values[index]!]);
}
for (const picked of choose(0, draw, [])) {
  const sorted = [...picked].sort((left, right) => left - right);
  const reach = Math.min(Math.sqrt(sorted[sorted.length - 1]! / sorted[0]!), 1000) * (1 + 1e-9);
  const misses = sorted.filter((value, place) => {
    const key = sorted[sorted.length - 1 - place]!;
    return Math.max(key / value, value / key) > reach;
  }).length;
  counts.set(misses, (counts.get(misses) ?? 0) + 1);
}
console.log([...counts.entries()].sort((left, right) => left[0] - right[0]));
