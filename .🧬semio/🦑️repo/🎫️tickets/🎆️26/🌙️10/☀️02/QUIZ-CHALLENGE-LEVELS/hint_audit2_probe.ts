/** 🔬️ Round 2 of the hint audit: for 30 seeds per live energy quiz, every task at `easy` is answered with an adjacent swap, a
 * swap of two far-apart items, the extremes swapped, one item at the opposite end, a three-place shift and a fully random
 * answer (same generator and seeds as `hint_audit_probe.ts`); the core's `hintsOf` hints are rendered by the React target's own
 * `compareText` and `classificationHintText` (names by `hintName`, the quantity named for several dimensions) in English and
 * German into `🗑️generated/hint-audit-2/<mode>/hints.jsonl` and `density.json`. Modes (env `MODE`): `live` (default), `nofamiliar`
 * (the `familiar` flags stripped, the counterfactual reference choice), `fulllabels` (items named by their full labels). */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { hintsOf, sheetOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";
import { compareText, hintName } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🧩️task/🟦️.tsx";
import { classificationHintText } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🗂️classification/🟦️.tsx";
import { localized, quizText } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts";

const MODE = process.env.MODE ?? "live";
const SEED_FROM = Number(process.env.SEED_FROM ?? 1);
const ENERGY = new URL("../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/", import.meta.url);
const OUT = new URL(`./🗑️generated/hint-audit-2/${MODE}${SEED_FROM === 1 ? "" : `-${SEED_FROM}`}/`, import.meta.url);
mkdirSync(OUT, { recursive: true });
const quizzes = ["🧲️physics", "🔥️heating", "❄️cooling", "📊️demand"].map((folder) => JSON.parse(readFileSync(new URL(`${folder}/❓️quiz/🔣️.json`, ENERGY), "utf8")));
const FAMILIAR = new Set<string>(quizzes.flatMap((quiz) => quiz.tasks.flatMap((task: any) => (task.items ?? []).filter((item: any) => item.familiar === true).map((item: any) => `${quiz.id}/${task.id}/${item.id}`))));
if (MODE === "nofamiliar") for (const quiz of quizzes) for (const task of quiz.tasks) for (const item of task.items ?? []) delete item.familiar;
const texts = { en: quizText("en"), de: quizText("de") };

function rng(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}
const pick = <T,>(random: () => number, list: readonly T[]): T => list[Math.floor(random() * list.length)]!;
function shuffled<T>(random: () => number, list: readonly T[]): T[] {
  const copy = [...list];
  for (let i = copy.length - 1; i > 0; i--) {
    const j = Math.floor(random() * (i + 1));
    [copy[i], copy[j]] = [copy[j]!, copy[i]!];
  }
  return copy;
}

const FLAWS = ["adjacent", "swapFar", "extremes", "opposite", "shift3", "random"] as const;
type Flaw = (typeof FLAWS)[number];

function flawedOrder<T>(random: () => number, truth: readonly T[], flaw: Flaw): T[] {
  const n = truth.length;
  const order = [...truth];
  const swap = (i: number, j: number): void => void ([order[i], order[j]] = [order[j]!, order[i]!]);
  switch (flaw) {
    case "adjacent": {
      const i = Math.floor(random() * (n - 1));
      swap(i, i + 1);
      return order;
    }
    case "swapFar": {
      const pairs: [number, number][] = [];
      for (let i = 0; i < n; i++) for (let j = i + 1; j < n; j++) if (j - i >= Math.ceil(n / 2)) pairs.push([i, j]);
      const [i, j] = pick(random, pairs);
      swap(i, j);
      return order;
    }
    case "extremes":
      swap(0, n - 1);
      return order;
    case "opposite": {
      const k = Math.floor(random() * n);
      const [moved] = order.splice(k, 1);
      if (k < n / 2) order.push(moved!);
      else order.unshift(moved!);
      return order;
    }
    case "shift3": {
      const k = Math.floor(random() * n);
      const [moved] = order.splice(k, 1);
      order.splice(Math.min(n - 1, Math.max(0, k + (random() < 0.5 ? -3 : 3))), 0, moved!);
      return order;
    }
    case "random":
      return shuffled(random, order);
  }
}

const records: any[] = [];
const density: { quiz: string; task: string; seed: number; flaw: Flaw; kind: string; items: number; hints: number; wrong: number }[] = [];

for (const quiz of quizzes) {
  for (let seed = SEED_FROM; seed < SEED_FROM + 30; seed++) {
    const sheet = sheetOf(quiz, seed, "easy");
    for (const sheetTask of sheet.tasks) {
      const task = quiz.tasks.find((candidate: { id: string }) => candidate.id === sheetTask.id);
      const items = task.items as { id: string; value?: number; values?: Record<string, number>; category?: string; familiar?: boolean; label: any }[];
      const familiarOf = (id: string): boolean => FAMILIAR.has(`${quiz.id}/${task.id}/${id}`);
      for (const flaw of FLAWS) {
        const random = rng(seed * 7919 + FLAWS.indexOf(flaw) * 131 + sheetTask.id.length);
        const base = { quiz: quiz.id, task: task.id, seed, flaw };
        const sheetItem = (id: string) => sheetTask.items.find((item: { id: string }) => item.id === id)!;
        const nameOf = (locale: "en" | "de") => (id: string): string => (MODE === "fulllabels" ? localized(sheetItem(id).label, locale) : hintName(sheetItem(id), locale));
        if (sheetTask.kind === "sorting") {
          const truth = sheetTask.items.map((i: { id: string }) => i.id).sort((a: string, b: string) => items.find((i) => i.id === a)!.value! - items.find((i) => i.id === b)!.value!);
          const order = flawedOrder(random, truth, flaw);
          const hints = hintsOf(task, sheetTask, { kind: "sorting", order });
          const keys = sheetTask.keys!;
          const keyOf = (id: string): number => keys[order.indexOf(id)]!;
          const value = (id: string): number => items.find((i) => i.id === id)!.value!;
          let wrong = 0;
          for (let r = 0; r < truth.length; r++) if (order[r] !== truth[r]) wrong++;
          density.push({ ...base, kind: "sorting", items: truth.length, hints: hints.length, wrong });
          for (const hint of hints) {
            if (hint.kind !== "compare") continue;
            records.push({ ...base, kind: "compare", item: hint.item, other: hint.other, otherFamiliar: familiarOf(hint.other), itemFamiliar: familiarOf(hint.item), itemEn: nameOf("en")(hint.item), otherEn: nameOf("en")(hint.other), itemDe: nameOf("de")(hint.item), otherDe: nameOf("de")(hint.other), claim: keyOf(hint.item) / keyOf(hint.other), truth: value(hint.item) / value(hint.other), factor: hint.factor, difference: hint.difference, verdict: hint.verdict, additive: sheetTask.quantity.additive, en: compareText(hint, sheetTask.quantity, nameOf("en"), texts.en, "en"), de: compareText(hint, sheetTask.quantity, nameOf("de"), texts.de, "de") });
          }
        } else if (sheetTask.kind === "matching") {
          const dims = sheetTask.dimensions;
          const assignments: Record<string, Record<string, number>> = {};
          let wrong = 0;
          for (const dimension of dims) {
            const cards = dimension.cards!;
            const valued = sheetTask.items.map((i: { id: string }) => i.id).filter((id: string) => Number.isFinite(items.find((i) => i.id === id)!.values?.[dimension.id]));
            const truth = [...valued].sort((a, b) => items.find((i) => i.id === a)!.values![dimension.id]! - items.find((i) => i.id === b)!.values![dimension.id]!);
            const cardRanks = cards.map((card: number, index: number) => ({ card, index })).filter((entry: { card: number }) => Number.isFinite(entry.card)).sort((a: { card: number }, b: { card: number }) => a.card - b.card);
            const order = flawedOrder(random, truth, flaw);
            const slot: Record<string, number> = {};
            order.forEach((id, rank) => (slot[id] = cardRanks[rank]!.index));
            assignments[dimension.id] = slot;
            for (let r = 0; r < truth.length; r++) if (order[r] !== truth[r]) wrong++;
          }
          const hints = hintsOf(task, sheetTask, { kind: "matching", assignments });
          density.push({ ...base, kind: "matching", items: sheetTask.items.length * dims.length, hints: hints.length, wrong });
          for (const hint of hints) {
            if (hint.kind !== "compare") continue;
            const dimension = dims.find((d: { id: string }) => d.id === hint.dimension)!;
            const cardOf = (id: string): number => dimension.cards![assignments[dimension.id]![id]!]!;
            const value = (id: string): number => items.find((i) => i.id === id)!.values![dimension.id]!;
            const named = dims.length > 1 ? (MODE === "fulllabels" ? (locale: "en" | "de") => localized(dimension.quantity.label, locale) : (locale: "en" | "de") => hintName(dimension.quantity, locale)) : undefined;
            records.push({ ...base, kind: "compare", item: hint.item, other: hint.other, otherFamiliar: familiarOf(hint.other), itemFamiliar: familiarOf(hint.item), dimension: hint.dimension, dims: dims.length, itemEn: nameOf("en")(hint.item), otherEn: nameOf("en")(hint.other), itemDe: nameOf("de")(hint.item), otherDe: nameOf("de")(hint.other), claim: cardOf(hint.item) / cardOf(hint.other), truth: value(hint.item) / value(hint.other), factor: hint.factor, difference: hint.difference, verdict: hint.verdict, additive: dimension.quantity.additive, en: compareText(hint, dimension.quantity, nameOf("en"), texts.en, "en", named?.("en")), de: compareText(hint, dimension.quantity, nameOf("de"), texts.de, "de", named?.("de")) });
          }
        } else if (sheetTask.kind === "classification") {
          const ids = sheetTask.items.map((i: { id: string }) => i.id);
          const own = (id: string): string => items.find((i) => i.id === id)!.category!;
          const cats = sheetTask.categories.map((c: { id: string }) => c.id);
          const answer: Record<string, string> = Object.fromEntries(ids.map((id: string) => [id, own(id)]));
          const other = (id: string): string[] => cats.filter((c: string) => c !== answer[id]);
          if (flaw === "adjacent" || flaw === "swapFar" || flaw === "extremes" || flaw === "shift3") {
            const pairs: [string, string][] = [];
            for (const a of ids) for (const b of ids) if (a < b && own(a) !== own(b)) pairs.push([a, b]);
            const rounds = flaw === "adjacent" ? 1 : flaw === "swapFar" ? 1 : flaw === "extremes" ? 2 : 3;
            for (let k = 0; k < rounds; k++) {
              const [a, b] = pick(random, pairs);
              [answer[a], answer[b]] = [answer[b]!, answer[a]!];
            }
          } else if (flaw === "opposite") {
            const id = pick(random, ids);
            answer[id] = pick(random, other(id));
          } else for (const id of ids) answer[id] = pick(random, cats);
          const hints = hintsOf(task, sheetTask, { kind: "classification", assignments: answer });
          density.push({ ...base, kind: "classification", items: ids.length, hints: hints.length, wrong: ids.filter((id: string) => answer[id] !== own(id)).length });
          for (const hint of hints) {
            records.push({ ...base, kind: hint.kind, item: hint.item, hint, itemFamiliar: familiarOf(hint.item), itemEn: nameOf("en")(hint.item), itemDe: nameOf("de")(hint.item), ...(hint.kind === "group" ? { other: hint.other, otherFamiliar: familiarOf(hint.other), together: hint.together, otherEn: nameOf("en")(hint.other), otherDe: nameOf("de")(hint.other) } : {}), ...(hint.kind === "profile" ? { other: hint.other, above: hint.above, axis: hint.axis, category: hint.category } : {}), en: classificationHintText(sheetTask, hint, texts.en, "en"), de: classificationHintText(sheetTask, hint, texts.de, "de") });
          }
        }
      }
    }
  }
}

writeFileSync(new URL("hints.jsonl", OUT), records.map((r) => JSON.stringify(r)).join("\n"));
writeFileSync(new URL("density.json", OUT), JSON.stringify(density));
const kinds = new Map<string, number>();
for (const r of records) {
  const key = `${r.kind}${r.verdict === undefined ? "" : `:${r.verdict}`}`;
  kinds.set(key, (kinds.get(key) ?? 0) + 1);
}
console.log("[DEBUG]", MODE, "records", records.length, "distinct EN", new Set(records.map((r) => r.en)).size, "distinct DE", new Set(records.map((r) => r.de)).size);
console.log("[DEBUG]", JSON.stringify([...kinds]));
