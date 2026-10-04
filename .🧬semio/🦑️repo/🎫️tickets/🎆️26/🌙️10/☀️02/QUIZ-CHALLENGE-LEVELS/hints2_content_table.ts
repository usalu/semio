/** 🏷️ Lists every short label and familiar flag of the four energy quizzes as a Markdown table and flags rule breaks
 * (short > 40, label > 40 without short, digit or parenthesis in a short, duplicate effective names in a task). */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

type Text = { en: string; de: string };
type Part = { id: string; label: Text; short?: Text; familiar?: boolean };
const root = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy";
const quizzes = ["🧲️physics", "🔥️heating", "❄️cooling", "📊️demand"];
const length = (text: string) => [...text].length;
const problems: string[] = [];
const rows: string[] = ["| Quiz / task | Part | Label EN (len) | Label DE (len) | Short EN (len) | Short DE (len) | Familiar |", "|---|---|---|---|---|---|---|"];
const cell = (text: string) => `${text.replace(/\u00a0/g, " ")} (${length(text)})`;
for (const folder of quizzes) {
  const quiz = JSON.parse(readFileSync(resolve(root, folder, "❓️quiz/🔣️.json"), "utf8"));
  for (const task of quiz.tasks) {
    const parts: [string, Part][] = [
      ...task.items.map((item: Part) => ["item", item] as [string, Part]),
      ...(task.categories ?? []).map((category: Part) => ["category", category] as [string, Part]),
      ...(task.axes ?? []).map((axis: Part) => ["axis", axis] as [string, Part]),
      ...(task.dimensions ?? []).map((dimension: { id: string; quantity: Part }) => ["quantity", { ...dimension.quantity, id: dimension.id }] as [string, Part]),
      ...(task.quantity ? [["quantity", { ...task.quantity, id: "quantity" }] as [string, Part]] : []),
    ];
    for (const [kind, part] of parts) {
      const where = `${quiz.id}/${task.id}/${part.id}`;
      for (const language of ["en", "de"] as const) {
        if (part.short === undefined && length(part.label[language]) > 40) problems.push(`${where}: label ${language} > 40 without short`);
        if (part.short !== undefined && length(part.short[language]) > 40) problems.push(`${where}: short ${language} > 40`);
        if (part.short !== undefined && !/^[^\d()][^()]*$/u.test(part.short[language])) problems.push(`${where}: short ${language} has a leading digit or parenthesis`);
      }
      if (part.short !== undefined || part.familiar === true)
        rows.push(`| ${quiz.id} / ${task.id} | ${kind} \`${part.id}\` | ${cell(part.label.en)} | ${cell(part.label.de)} | ${part.short ? cell(part.short.en) : "–"} | ${part.short ? cell(part.short.de) : "–"} | ${part.familiar ? "yes" : ""} |`);
    }
    for (const language of ["en", "de"] as const) {
      const names = task.items.map((item: Part) => (item.short ?? item.label)[language]);
      for (const [index, name] of names.entries()) if (names.indexOf(name) !== index) problems.push(`${quiz.id}/${task.id}: duplicate ${language} name ${name}`);
    }
    if (task.kind !== "classification") {
      const familiar = task.items.filter((item: Part) => item.familiar === true).length;
      console.log(`${quiz.id}/${task.id}: ${familiar} familiar of ${task.items.length}`);
    }
  }
}
console.log(rows.join("\n"));
console.log(problems.length === 0 ? "problems: none" : `problems:\n${problems.join("\n")}`);
