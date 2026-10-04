/** 🏷️ Lists the quantity labels of multi-dimension matchings, every axis label and every short label of the four energy quizzes. */
import physics from "../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json";
import heating from "../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/❓️quiz/🔣️.json";
import cooling from "../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/❓️quiz/🔣️.json";
import demand from "../../../../../../../🎓️teaching/🏛️architecture/⚡️energy/📊️demand/❓️quiz/🔣️.json";

for (const quiz of [physics, heating, cooling, demand] as any[]) {
  for (const task of quiz.tasks) {
    if (task.kind === "matching") for (const d of task.dimensions) console.log("[DEBUG] dim", task.id, task.dimensions.length, JSON.stringify(d.quantity.label), JSON.stringify(d.quantity.short));
    if (task.kind === "sorting") console.log("[DEBUG] sortq", task.id, JSON.stringify(task.quantity.label), JSON.stringify(task.quantity.short));
    for (const a of task.axes ?? []) console.log("[DEBUG] axis", task.id, JSON.stringify(a.label), JSON.stringify(a.short));
    for (const c of task.categories ?? []) if (c.short) console.log("[DEBUG] cat short", JSON.stringify(c.short));
    for (const i of task.items ?? []) if (i.short) console.log("[DEBUG] item short", JSON.stringify(i.short));
  }
}
