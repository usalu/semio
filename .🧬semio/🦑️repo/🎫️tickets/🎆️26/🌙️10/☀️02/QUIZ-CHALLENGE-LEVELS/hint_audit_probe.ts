/** 🔬️ Audits the easy hints over flawed answers: for 30 seeds per live energy quiz, every task at `easy` is answered with
 * an adjacent swap, a swap of two far-apart items, the extremes swapped, one item at the opposite end, a three-place
 * shift and a fully random answer; the core's `hintsOf` hints are rendered by the React target's own `compareText` and
 * `classificationHintText` in English and German, with the claim, the truth and the density, into
 * `🗑️generated/hint-audit/hints.jsonl` and `summary.json`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { hintsOf, sheetOf } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";
import { compareText } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🧩️task/🟦️.tsx";
import { classificationHintText } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🗂️classification/🟦️.tsx";
import { localized, quizText } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts";

const ENERGY = new URL("../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/", import.meta.url);
const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
mkdirSync(OUT, { recursive: true });
const quizzes = ["🧲️physics", "🔥️heating", "❄️cooling", "📊️demand"].map((folder) => JSON.parse(readFileSync(new URL(`${folder}/❓️quiz/🔣️.json`, ENERGY), "utf8")));
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

interface Record {
  readonly quiz: string;
  readonly task: string;
  readonly seed: number;
  readonly flaw: Flaw;
  readonly kind: string;
  readonly item: string;
  readonly other?: string;
  readonly itemEn?: string;
  readonly otherEn?: string;
  readonly itemDe?: string;
  readonly otherDe?: string;
  readonly dimension?: string;
  readonly claim?: number;
  readonly truth?: number;
  readonly factor?: number;
  readonly under?: boolean;
  readonly together?: boolean;
  readonly en: string;
  readonly de: string;
}
const records: Record[] = [];
const density: { quiz: string; task: string; seed: number; flaw: Flaw; items: number; hints: number; wrong: number }[] = [];

for (const quiz of quizzes) {
  for (let seed = 1; seed <= 30; seed++) {
    const sheet = sheetOf(quiz, seed, "easy");
    for (const sheetTask of sheet.tasks) {
      const task = quiz.tasks.find((candidate: { id: string }) => candidate.id === sheetTask.id);
      const items = task.items as { id: string; value?: number; values?: Record<string, number>; category?: string; label: any }[];
      for (const flaw of FLAWS) {
        const random = rng(seed * 7919 + FLAWS.indexOf(flaw) * 131 + sheetTask.id.length);
        const base = { quiz: quiz.id, task: task.id, seed, flaw };
        const labelOf = (locale: "en" | "de") => (id: string): string => localized(sheetTask.items.find((item: { id: string }) => item.id === id)!.label, locale);
        if (sheetTask.kind === "sorting") {
          const truth = sheetTask.items.map((i: { id: string }) => i.id).sort((a: string, b: string) => items.find((i) => i.id === a)!.value! - items.find((i) => i.id === b)!.value!);
          const order = flawedOrder(random, truth, flaw);
          const hints = hintsOf(task, sheetTask, { kind: "sorting", order });
          const keys = sheetTask.keys!;
          const keyOf = (id: string): number => keys[order.indexOf(id)]!;
          const value = (id: string): number => items.find((i) => i.id === id)!.value!;
          let wrong = 0;
          for (let r = 0; r < truth.length; r++) if (order[r] !== truth[r]) wrong++;
          density.push({ ...base, items: truth.length, hints: hints.length, wrong });
          for (const hint of hints) {
            if (hint.kind !== "compare") continue;
            records.push({ ...base, kind: "compare", item: hint.item, other: hint.other, itemEn: labelOf("en")(hint.item), otherEn: labelOf("en")(hint.other), itemDe: labelOf("de")(hint.item), otherDe: labelOf("de")(hint.other), claim: keyOf(hint.item) / keyOf(hint.other), truth: value(hint.item) / value(hint.other), factor: hint.factor, under: hint.under, en: compareText(hint, sheetTask.quantity, labelOf("en"), texts.en, "en"), de: compareText(hint, sheetTask.quantity, labelOf("de"), texts.de, "de") });
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
          density.push({ ...base, items: sheetTask.items.length * dims.length, hints: hints.length, wrong });
          for (const hint of hints) {
            if (hint.kind !== "compare") continue;
            const dimension = dims.find((d: { id: string }) => d.id === hint.dimension)!;
            const cardOf = (id: string): number => dimension.cards![assignments[dimension.id]![id]!]!;
            const value = (id: string): number => items.find((i) => i.id === id)!.values![dimension.id]!;
            records.push({ ...base, kind: "compare", item: hint.item, other: hint.other, dimension: dimension.id, itemEn: labelOf("en")(hint.item), otherEn: labelOf("en")(hint.other), itemDe: labelOf("de")(hint.item), otherDe: labelOf("de")(hint.other), claim: cardOf(hint.item) / cardOf(hint.other), truth: value(hint.item) / value(hint.other), factor: hint.factor, under: hint.under, en: compareText(hint, dimension.quantity, labelOf("en"), texts.en, "en"), de: compareText(hint, dimension.quantity, labelOf("de"), texts.de, "de") });
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
          density.push({ ...base, items: ids.length, hints: hints.length, wrong: ids.filter((id: string) => answer[id] !== own(id)).length });
          for (const hint of hints) {
            records.push({ ...base, kind: hint.kind, item: hint.item, ...(hint.kind === "group" ? { other: hint.other, otherEn: labelOf("en")(hint.other), otherDe: labelOf("de")(hint.other), together: hint.together } : {}), itemEn: labelOf("en")(hint.item), itemDe: labelOf("de")(hint.item), en: classificationHintText(sheetTask, hint, texts.en, "en"), de: classificationHintText(sheetTask, hint, texts.de, "de") });
          }
        }
      }
    }
  }
}

writeFileSync(new URL("hints.jsonl", OUT), records.map((r) => JSON.stringify(r)).join("\n"));
writeFileSync(new URL("density.json", OUT), JSON.stringify(density));
const distinct = new Set(records.map((r) => r.en));
console.log("[DEBUG] records", records.length, "distinct EN", distinct.size, "distinct DE", new Set(records.map((r) => r.de)).size);
const kinds = new Map<string, number>();
for (const r of records) kinds.set(`${r.kind}${r.under === undefined ? "" : r.under ? ":under" : ":over"}`, (kinds.get(`${r.kind}${r.under === undefined ? "" : r.under ? ":under" : ":over"}`) ?? 0) + 1);
console.log("[DEBUG]", JSON.stringify([...kinds]));
