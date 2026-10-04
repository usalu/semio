/** 🗣️ Reads every effective item, category, axis and quantity name (`short ?? label`) of the four energy quizzes inside the
 * hint templates ("N × „…“ zusammen", "„…“ ist größer als „…“", "in puncto …") so awkward shorts stand out, and checks the
 * content rules (≤ 40, no leading digit, no parenthesis, distinct per task). */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

type Text = { en: string; de: string };
type Part = { id: string; label: Text; short?: Text; familiar?: boolean };
const root = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy";
const quizzes = ["🧲️physics", "🔥️heating", "❄️cooling", "📊️demand"];
const name = (part: Part) => part.short ?? part.label;
const problems: string[] = [];
const COUNTABLE = { en: /^(?!(?:The|A|An|One) )(?!\p{L}+ing (?:(?:an?|the)\b|\d))[^,']*$/u, de: /^(?!(?:Der|Die|Das|Ein|Eine) )(?!\p{Lu}\p{Ll}+en (?:eines|einer|des|der) )[^,']*$/u } as const;
const rejected = { en: ["Boiling a litre of water", "Heating 1 litre of water", "The Sun", "World's annual primary energy use", "Apartment, nominal ventilation", "One full smartphone charge"], de: ["Aufkochen eines Liters Wasser", "Eine volle Smartphone-Ladung", "Wohnung, Nennlüftung"] };
const accepted = { en: ["Burning tea light", "Building at night", "Heating load of an unrenovated house", "Charging power of a home wall box", "Germany’s primary energy use per year"], de: ["Aufgekochter Liter Wasser", "Brennendes Teelicht", "Deutschlands Primärenergie pro Jahr"] };
for (const language of ["en", "de"] as const) {
  for (const text of rejected[language]) if (COUNTABLE[language].test(text)) problems.push(`rule misses ${language} ${text}`);
  for (const text of accepted[language]) if (!COUNTABLE[language].test(text)) problems.push(`rule rejects ${language} ${text}`);
}
for (const folder of quizzes) {
  const quiz = JSON.parse(readFileSync(resolve(root, folder, "❓️quiz/🔣️.json"), "utf8"));
  for (const task of quiz.tasks) {
    const quantity: Part | undefined = task.quantity;
    console.log(`\n## ${quiz.id}/${task.id} (${task.kind}${quantity ? `, ${name(quantity).en} / ${name(quantity).de}` : ""})`);
    for (const dimension of task.dimensions ?? []) console.log(`  in ${name(dimension.quantity).en} | in puncto ${name(dimension.quantity).de}`);
    for (const axis of task.axes ?? []) console.log(`  axis: in ${name(axis).en} | in puncto ${name(axis).de}`);
    for (const category of task.categories ?? []) console.log(`  category: fits ${name(category).en} | zu ${name(category).de} passt`);
    const items: Part[] = task.items;
    for (const [index, item] of items.entries()) {
      const other = items[(index + 1) % items.length];
      const n = name(item), o = name(other);
      console.log(`  ${item.short ? "S" : "-"}${item.familiar ? "F" : " "} 1,000 × “${n.en}” together only add up to 1 × “${o.en}”`);
      console.log(`      1.000 × „${n.de}“ zusammen nur 1 × „${o.de}“ ergeben`);
      console.log(`      “${n.en}” is larger than “${o.en}” | „${n.de}“ größer ist als „${o.de}“`);
      for (const language of ["en", "de"] as const) {
        const text = n[language];
        if ([...text].length > 40) problems.push(`${quiz.id}/${task.id}/${item.id}: ${language} > 40`);
        if (!/^[^\d()][^()]*$/u.test(text) && item.short) problems.push(`${quiz.id}/${task.id}/${item.id}: ${language} digit or parenthesis`);
        if (!COUNTABLE[language].test(text)) problems.push(`${quiz.id}/${task.id}/${item.id}: ${language} not countable: ${text}`);
      }
    }
    for (const language of ["en", "de"] as const) {
      const names = items.map((item) => name(item)[language]);
      for (const [index, value] of names.entries()) if (names.indexOf(value) !== index) problems.push(`${quiz.id}/${task.id}: duplicate ${language} ${value}`);
    }
  }
}
console.log(problems.length === 0 ? "\nproblems: none" : `\nproblems:\n${problems.join("\n")}`);
