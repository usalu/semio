/** 🔎️ Probes the easy hints the site spec provokes, over many sheets, with the core's `hintsOf`: physics `powers` with the
 * largest item on the smallest key and the rest shifted up, and with the extremes exchanged; heating `u-values` with the
 * cards of the extremes exchanged; demand `standard-profiles` with the first item moved into each other category. Prints
 * how often each reference and wording comes out, so the spec asserts only what the design fixes. */
import { readFileSync } from "node:fs";
import { hintsOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/⛰️challenge/🟦️.ts";
import { sheetOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/🃏️sheet/🟦️.ts";

const load = (path: string) => JSON.parse(readFileSync(new URL(`../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/${path}/❓️quiz/🔣️.json`, import.meta.url), "utf8"));
const tally = new Map<string, number>();
const count = (key: string): void => void tally.set(key, (tally.get(key) ?? 0) + 1);

const physics = load("🧲️physics");
const powers = physics.tasks.find((task: { id: string }) => task.id === "powers");
const heating = load("🔥️heating");
const uValues = heating.tasks.find((task: { id: string }) => task.id === "u-values");
const demand = load("📊️demand");
const profiles = demand.tasks.find((task: { id: string }) => task.id === "standard-profiles");

for (let seed = 1; seed <= 600; seed++) {
  const powersSheet = sheetOf(physics, seed, "easy").tasks.find((task) => task.id === "powers")!;
  const value = (id: string): number => powers.items.find((item: { id: string }) => item.id === id).value;
  const order = powersSheet.items.map((item) => item.id).sort((left, right) => value(left) - value(right));
  const largest = order[order.length - 1]!;
  const n = order.length;
  for (const hint of hintsOf(powers, powersSheet, { kind: "sorting", order: [largest, ...order.slice(0, -1)] })) {
    if (hint.kind !== "compare") continue;
    count(`powers shift ${hint.item === largest ? "largest" : "other"} other=order[${order.indexOf(hint.other)}] under=${hint.under}`);
  }
  for (const hint of hintsOf(powers, powersSheet, { kind: "sorting", order: [largest, ...order.slice(1, -1), order[0]!] })) {
    if (hint.kind !== "compare") continue;
    count(`powers swap n=${n} item=order[${order.indexOf(hint.item)}] other=order[${order.indexOf(hint.other)}] under=${hint.under} factor<1=${hint.factor! < 1}`);
  }
  const top = [...order.slice(0, -2), largest, order[n - 2]!];
  for (const hint of hintsOf(powers, powersSheet, { kind: "sorting", order: top })) {
    if (hint.kind !== "compare") continue;
    count(`powers top-two item=order[${order.indexOf(hint.item)}] other=order[${order.indexOf(hint.other)}] under=${hint.under} sun=${largest === "sun"}`);
  }
  if (largest === "sun") count("powers sun drawn");

  const heatingSheet = sheetOf(heating, seed, "easy").tasks.find((task) => task.id === "u-values")!;
  if (heatingSheet.kind === "matching") {
    const dimension = uValues.dimensions[0].id;
    const v = (id: string): number => uValues.items.find((item: { id: string }) => item.id === id).values[dimension];
    const ranked = heatingSheet.items.map((item) => item.id).sort((left, right) => v(left) - v(right));
    const cards = heatingSheet.dimensions[0]!.cards!;
    const cardOf = (val: number): number => cards.indexOf(val);
    const assigned: Record<string, number> = Object.fromEntries(ranked.map((id) => [id, cardOf(v(id))]));
    const [small, large] = [ranked[0]!, ranked[ranked.length - 1]!];
    assigned[small] = cardOf(v(large));
    assigned[large] = cardOf(v(small));
    const equalSmall = ranked.filter((id) => v(id) === v(small)).length;
    const equalLarge = ranked.filter((id) => v(id) === v(large)).length;
    for (const hint of hintsOf(uValues, heatingSheet, { kind: "matching", assignments: { [dimension]: assigned } })) {
      if (hint.kind !== "compare") continue;
      const other = ranked.indexOf(hint.other);
      count(`u-values ${hint.item === small ? "smallest" : hint.item === large ? "largest" : "other"} other=ranked[${other === ranked.length - 2 ? "n-2" : other}] under=${hint.under} factor<1=${hint.factor! < 1} equalSmall=${equalSmall} equalLarge=${equalLarge} equalOther=${v(hint.other) === v(hint.item === small ? large : small)}`);
    }
  }

  const demandSheet = sheetOf(demand, seed, "easy").tasks.find((task) => task.id === "standard-profiles")!;
  if (demandSheet.kind === "classification") {
    const items = demandSheet.items.map((item) => item.id);
    const moved = items[0]!;
    const own = profiles.items.find((item: { id: string }) => item.id === moved).category;
    for (const category of demandSheet.categories) {
      if (category.id === own) continue;
      const assignments = Object.fromEntries(items.map((id) => [id, profiles.items.find((item: { id: string }) => item.id === id).category]));
      assignments[moved] = category.id;
      const hints = hintsOf(profiles, demandSheet, { kind: "classification", assignments });
      count(`profiles move ${hints.map((hint) => `${hint.kind}${hint.kind === "profile" ? `:${hint.axis}` : ""}`).join(",") || "none"} categories=${demandSheet.categories.length}`);
    }
  }
}
for (const [key, value] of [...tally].sort()) console.log(`[DEBUG] ${value}\t${key}`);
