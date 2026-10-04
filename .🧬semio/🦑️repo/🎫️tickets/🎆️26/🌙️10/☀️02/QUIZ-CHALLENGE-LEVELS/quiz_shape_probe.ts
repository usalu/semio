/** 🔬️ Prints the shape of the heating and demand quizzes (tasks, kinds, draws, dimensions, axes, categories with
 * descriptions) as the e2e breadth specs need them. Usage: `bun quiz_shape_probe.ts` */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/⚡️energy");
for (const name of ["🔥️heating", "📊️demand", "❄️cooling", "🧲️physics"]) {
  const quiz = JSON.parse(readFileSync(resolve(root, name, "❓️quiz/🔣️.json"), "utf8"));
  console.log(`== ${quiz.id}`);
  for (const task of quiz.tasks) {
    console.log(`  ${task.kind} ${task.id} items=${task.items.length} draw=${task.draw ?? "-"} quantity=${JSON.stringify(task.quantity?.unit ?? null)} dims=${JSON.stringify((task.dimensions ?? []).map((d: { id: string; quantity: { unit: string; prefixed: boolean } }) => `${d.id}:${d.quantity.unit}:${d.quantity.prefixed}`))}`);
    if (task.axes) console.log(`    axes=${JSON.stringify(task.axes.map((a: { id: string; unit?: string; min?: number; max?: number }) => `${a.id}:${a.unit}:${a.min}..${a.max}`))}`);
    if (task.categories) console.log(`    categories=${JSON.stringify(task.categories.map((c: { id: string; description?: { en: string } }) => `${c.id}:${c.description ? "desc" : "-"}`))}`);
  }
}
