/** 🧾️ Checks the real task planner's renderer declaration and each generated contract owner. */
import { expect, test } from "bun:test";
import { planBuildTasksV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/🟦️.ts";

const roots = [
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript",
  "🧰️framework/📦️packages/🦀️rust",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust",
  "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust"
];

test("renderer typecheck plans the four generated contracts before TypeScript", async () => {
  const projects = Object.fromEntries(await Promise.all(roots.map(async root => {
    const project = await Bun.file(root + "/📋️project.json").json();
    return [project.name, { ...project, root }];
  })));
  const selected = "@semio-tech/framework-renderer-react:typecheck";
  const prerequisites = [
    "@semio-tech/framework-actor-rs:typegen",
    "@semio-tech/framework-os-shell-rs:typegen",
    "@semio-tech/framework-rs:generate",
    "@semio-tech/ui-contract-rs:generate",
  ];
  const plan = planBuildTasksV1({ projects, dependencies: [] } as Parameters<typeof planBuildTasksV1>[0], { tasks: [selected] });
  expect(plan.selected).toEqual([selected]);
  expect(plan.tasks[selected]?.dependencies).toEqual(prerequisites);
  for (const task of prerequisites) expect(plan.tasks[task]?.definition.executor).toBe("repo:script");
});
