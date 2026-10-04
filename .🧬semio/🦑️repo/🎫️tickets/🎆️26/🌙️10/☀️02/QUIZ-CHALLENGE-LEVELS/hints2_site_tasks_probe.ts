/** 🔎️ Prints the shape of every task of the four energy quizzes the challenge-levels spec drives: kind, draw, quantity
 * and dimensions (short, prefixed, additive, scale), and per item value(s), short and familiar. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "⚡️energy");
for (const quiz of ["🧲️physics", "🔥️heating", "📊️demand"]) {
  const source = JSON.parse(readFileSync(join(root, quiz, "❓️quiz", "🔣️.json"), "utf8"));
  for (const task of source.tasks) {
    console.log(`\n${quiz} ${task.id} ${task.kind} draw=${task.draw ?? "-"} items=${task.items.length}`);
    if (task.quantity) console.log("  quantity", JSON.stringify(task.quantity));
    for (const dimension of task.dimensions ?? []) console.log("  dimension", dimension.id, JSON.stringify(dimension.quantity));
    for (const axis of task.axes ?? []) console.log("  axis", axis.id, JSON.stringify(axis.short ?? null));
    for (const category of task.categories ?? []) console.log("  category", category.id, JSON.stringify(category.short ?? null), JSON.stringify(category.profile ?? null));
    for (const item of task.items) console.log("  item", item.id, item.value ?? JSON.stringify(item.values ?? item.category), item.familiar ? "familiar" : "", JSON.stringify(item.short ?? null));
  }
}
