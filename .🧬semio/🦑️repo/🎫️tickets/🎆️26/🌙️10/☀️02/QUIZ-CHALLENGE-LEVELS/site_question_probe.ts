/** 🧪️ Checks offline, over many sheets and both languages, that the questions the client renders for the core's hints
 * pass the site spec's expectations of `⛰️challenge-levels` (its wordings, references and factor bounds, copied here),
 * before the slow e2e runs; prints one sample question per case and language. */
import { readFileSync } from "node:fs";
import { hintsOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/⛰️challenge/🟦️.ts";
import { sheetOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/🃏️sheet/🟦️.ts";
import { classificationHintText, compareText, quizText } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🟦️.tsx";

type Locale = "en" | "de";
const load = (path: string) => JSON.parse(readFileSync(new URL(`../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/${path}/❓️quiz/🔣️.json`, import.meta.url), "utf8"));
const physics = load("🧲️physics");
const powers = physics.tasks.find((task: { id: string }) => task.id === "powers");
const heating = load("🔥️heating");
const uValues = heating.tasks.find((task: { id: string }) => task.id === "u-values");
const demand = load("📊️demand");
const profiles = demand.tasks.find((task: { id: string }) => task.id === "standard-profiles");

const FACTOR = "([\\d.,]+)";
const WORDINGS: Record<string, Record<Locale, (small: string, large: string) => string>> = {
  sumUnder: { en: (s, l) => `Are you sure ${FACTOR} × ${s} together only add up to 1 × ${l}\\?`, de: (s, l) => `Bist du sicher, dass ${FACTOR} × ${s} zusammen nur 1 × ${l} ergeben\\?` },
  sumOver: { en: (s, l) => `Are you sure it takes ${FACTOR} × ${s} to add up to 1 × ${l}\\?`, de: (s, l) => `Bist du sicher, dass es ${FACTOR} × ${s} braucht, um 1 × ${l} zu ergeben\\?` },
  ratioUnder: { en: (s, l) => `Are you sure ${l} is only ${FACTOR} times as large as ${s}\\?`, de: (s, l) => `Bist du sicher, dass ${l} nur ${FACTOR}-mal so groß ist wie ${s}\\?` },
  ratioOver: { en: (s, l) => `Are you sure ${l} is as much as ${FACTOR} times as large as ${s}\\?`, de: (s, l) => `Bist du sicher, dass ${l} ganze ${FACTOR}-mal so groß ist wie ${s}\\?` },
};
const quoted = (label: string, locale: Locale): string => (locale === "en" ? `“${label}”` : `„${label}“`);
const escaped = (text: string): string => text.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
const failures: string[] = [];
const samples = new Map<string, string>();
const fail = (why: string): void => void failures.push(why);

function check(task: any, locale: Locale, question: string, wording: string, small: string, large: string, claim: number, label: string): void {
  if (!samples.has(`${label} ${locale}`)) samples.set(`${label} ${locale}`, question);
  const name = (id: string): string => escaped(quoted(task.items.find((item: any) => item.id === id).label[locale], locale));
  const found = new RegExp(`^${WORDINGS[wording]![locale](name(small), name(large))}$`, "u").exec(question);
  if (!found) return fail(`${label} ${locale}: ${question} is not ${wording} of ${small}/${large}`);
  const digits = found[1]!;
  const factor = Number(locale === "en" ? digits.replaceAll(",", "") : digits.replaceAll(".", "").replace(",", "."));
  if (!(factor / claim > 1 / 1.1 && factor / claim < 1.1)) fail(`${label} ${locale}: ${digits} vs ${claim}`);
  if (wording.endsWith("Under") ? factor > claim * (1 + 1e-9) : factor < claim * (1 - 1e-9)) fail(`${label} ${locale}: ${digits} crosses ${claim}`);
}

for (let seed = 1; seed <= 300; seed++) {
  for (const locale of ["en", "de"] as const) {
    const text = quizText(locale);
    const sheet = sheetOf(physics, seed, "easy").tasks.find((task) => task.id === "powers")!;
    const value = (id: string): number => powers.items.find((item: any) => item.id === id).value;
    const order = sheet.items.map((item) => item.id).sort((left, right) => value(left) - value(right));
    const label = (id: string): string => powers.items.find((item: any) => item.id === id).label[locale];
    const n = order.length;
    const [smallest, second, below, next, largest] = [order[0]!, order[1]!, order[n - 3]!, order[n - 2]!, order[n - 1]!];
    const exchanged = hintsOf(powers, sheet, { kind: "sorting", order: [largest, ...order.slice(1, -1), smallest] });
    const said = (id: string, hints: any[]): string => compareText(hints.find((hint) => hint.item === id), powers.quantity, label, text, locale);
    if (exchanged.length !== 2) fail(`powers exchanged ${seed}: ${exchanged.length} hints`);
    check(powers, locale, said(largest, exchanged), "sumOver", largest, second, value(second) / value(smallest), "powers-largest");
    check(powers, locale, said(smallest, exchanged), "sumOver", next, smallest, value(largest) / value(next), "powers-smallest");
    const top = hintsOf(powers, sheet, { kind: "sorting", order: [...order.slice(0, -2), largest, next] });
    const reach = Math.min(Math.sqrt(value(largest) / value(smallest)), 1000);
    const far = value(largest) / value(next) > reach * (1 + 1e-9);
    if (top.length !== (far ? 2 : 0)) fail(`powers top ${seed}: ${top.length} hints, far ${far}`);
    if (far) {
      check(powers, locale, said(largest, top), "sumUnder", below, largest, value(next) / value(below), "powers-top-largest");
      check(powers, locale, said(next, top), "sumOver", below, next, value(largest) / value(below), "powers-top-next");
    }

    const heatingSheet = sheetOf(heating, seed, "easy").tasks.find((task) => task.id === "u-values")!;
    if (heatingSheet.kind === "matching") {
      const dimension = uValues.dimensions[0];
      const v = (id: string): number => uValues.items.find((item: any) => item.id === id).values[dimension.id];
      const items = heatingSheet.items.map((item) => item.id);
      const ranked = [...items].sort((left, right) => v(left) - v(right));
      const cards = heatingSheet.dimensions[0]!.cards!;
      const assigned: Record<string, number> = Object.fromEntries(items.map((id) => [id, cards.indexOf(v(id))]));
      const [small, large] = [ranked[0]!, ranked[ranked.length - 1]!];
      assigned[small] = cards.indexOf(v(large));
      assigned[large] = cards.indexOf(v(small));
      const hints = hintsOf(uValues, heatingSheet, { kind: "matching", assignments: { [dimension.id]: assigned } });
      const labelOf = (id: string): string => uValues.items.find((item: any) => item.id === id).label[locale];
      const of = (id: string): string => compareText(hints.find((hint: any) => hint.item === id) as any, dimension.quantity, labelOf, text, locale);
      if (hints.length !== 2) fail(`u-values ${seed}: ${hints.length} hints`);
      check(uValues, locale, of(large), "ratioOver", large, ranked[1]!, v(ranked[1]!) / v(small), "u-values-largest");
      check(uValues, locale, of(small), "ratioOver", ranked[ranked.length - 2]!, small, v(large) / v(ranked[ranked.length - 2]!), "u-values-smallest");
    }

    const demandSheet = sheetOf(demand, seed, "easy").tasks.find((task) => task.id === "standard-profiles")!;
    if (demandSheet.kind === "classification") {
      const items = demandSheet.items.map((item) => item.id);
      const moved = items[0]!;
      const home = profiles.items.find((item: any) => item.id === moved).category;
      const own = profiles.categories.find((category: any) => category.id === home).profile;
      const reachOf = (axis: string): number => {
        const values = profiles.categories.map((category: any) => category.profile[axis]);
        return (Math.max(...values) - Math.min(...values)) / 2;
      };
      let found = { category: "", axis: "", ratio: -Infinity };
      for (const category of profiles.categories) {
        if (category.id === home) continue;
        for (const axis of profiles.axes) {
          const ratio = Math.abs(category.profile[axis.id] - own[axis.id]) / reachOf(axis.id);
          if (ratio > found.ratio) found = { category: category.id, axis: axis.id, ratio };
        }
      }
      const assignments = Object.fromEntries(items.map((id) => [id, profiles.items.find((item: any) => item.id === id).category]));
      assignments[moved] = found.category;
      const hints = hintsOf(profiles, demandSheet, { kind: "classification", assignments });
      const chosen = profiles.categories.find((category: any) => category.id === found.category);
      const axis = profiles.axes.find((candidate: any) => candidate.id === found.axis);
      const value = `${new Intl.NumberFormat(locale, { maximumFractionDigits: 6 }).format(chosen.profile[found.axis])} ${axis.unit}`;
      const item = quoted(profiles.items.find((candidate: any) => candidate.id === moved).label[locale], locale);
      const expected = locale === "en" ? `Are you sure ${item} fits ${chosen.label.en}, with ${axis.label.en} at about ${value}?` : `Bist du sicher, dass ${item} zu ${chosen.label.de} passt, mit ${axis.label.de} bei rund ${value}?`;
      if (hints.length !== 1 || hints[0]!.item !== moved) fail(`profiles ${seed}: ${JSON.stringify(hints)}`);
      const question = classificationHintText(demandSheet as any, hints[0]!, text, locale);
      if (!samples.has(`profiles ${locale}`)) samples.set(`profiles ${locale}`, question);
      if (question.replace(/ /gu, " ") !== expected) fail(`profiles ${seed} ${locale}: ${question} ≠ ${expected}`);
      if (!(found.ratio > 1 + 1e-9)) fail(`profiles ${seed}: ratio ${found.ratio}`);
    }
  }
}
for (const [key, question] of samples) console.log(`[DEBUG] ${key}: ${question}`);
console.log(`[DEBUG] failures ${failures.length}`);
for (const failure of failures.slice(0, 20)) console.log(`[DEBUG] ${failure}`);
