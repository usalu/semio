import io, sys
p = sys.argv[1]
s = io.open(p, encoding="utf-8", newline="").read()
reps = [
("""import { learnerTag } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";""",
"""import { learnerTag, runSeed, scoreRun, sheetOf, type Answer, type Quiz, type SheetTask } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts";
import { readFileSync } from "node:fs";"""),
("""    { run: "3".repeat(32), quiz: "heating", status: "open", startedAt: 1_790_602_000_000 },""",
"""    { run: "3".repeat(32), quiz: "demand", status: "open", startedAt: 1_790_602_000_000 },"""),
("""const others = ["Mira",""",
"""const energy = join(here, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "⚡️energy");
const quizOf = (folder: string): Quiz => JSON.parse(readFileSync(join(energy, folder, "❓️quiz", "🔣️.json"), "utf8")) as Quiz;

function sloppy(task: SheetTask): Answer {
  switch (task.kind) {
    case "classification":
      return { kind: "classification", assignments: Object.fromEntries(task.items.map((item, index) => [item.id, task.categories[index % task.categories.length]!.id])) };
    case "sorting":
      return { kind: "sorting", order: task.items.map((item) => item.id) };
    case "matching":
      return { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item, index) => [item.id, index]))])) };
  }
}

const physics = quizOf("🧲️physics");
const physicsRun = "1".repeat(32);
const physicsSheet = sheetOf(physics, runSeed(physicsRun));
const physicsAnswers = Object.fromEntries(physicsSheet.tasks.map((task) => [task.id, sloppy(task)]));
const demand = quizOf("📊️demand");
const demandRun = "3".repeat(32);
const runViews: Readonly<Record<string, unknown>> = {
  [physicsRun]: { run: physicsRun, learner, quiz: "physics", status: "submitted", sheet: physicsSheet, answers: physicsAnswers, result: scoreRun(physics, physicsSheet, physicsAnswers), startedAt: 1_790_600_000_000, submittedAt: 1_790_600_600_000 },
  [demandRun]: { run: demandRun, learner, quiz: "demand", status: "open", sheet: sheetOf(demand, runSeed(demandRun)), answers: {}, startedAt: 1_790_602_000_000 },
};

const others = ["Mira","""),
("""    if (query.type === "leaderboard") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(leaderboard) });""",
"""    if (query.type === "leaderboard") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(leaderboard) });
    if (query.type === "run") return route.fulfill({ status: 200, contentType: "application/json", body: snapshot(runViews[(query as unknown as { readonly run: string }).run]) });"""),
("""    await page.getByRole("button", { name: "All badges" }).click();
    await page.getByRole("heading", { level: 1, name: "Badges" }).waitFor();
    await shoot(page, "badges", width, scheme, homeErrors);""",
"""    await page.getByRole("button", { name: "All badges" }).click();
    await page.getByRole("heading", { level: 1, name: "Badges" }).waitFor();
    await shoot(page, "badges", width, scheme, homeErrors);
    await page.setViewportSize({ width, height });
    await page.goto(site);
    await page.locator('[data-card="board"] tbody tr').first().waitFor();
    await page.getByRole("region", { name: "Energy Demand" }).getByRole("button", { name: "Resume quiz" }).click();
    await page.locator('[data-card="task"]').waitFor();
    await shoot(page, "run", width, scheme, homeErrors);
    await page.setViewportSize({ width, height });
    await page.goto(site);
    await page.locator('[data-card="board"] tbody tr').first().waitFor();
    await page.getByRole("region", { name: "Physical Understanding" }).getByRole("button", { name: "View last result" }).click();
    await page.locator('[data-card="task-result"]').first().waitFor();
    await shoot(page, "results", width, scheme, homeErrors);"""),
]
for a, b in reps:
    if s.count(a) != 1:
        sys.exit(f"anchor {s.count(a)}: {a[:60]!r}")
    s = s.replace(a, b)
io.open(p, "w", encoding="utf-8", newline="").write(s)
print("patched")
