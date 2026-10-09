import { readdirSync, readFileSync, existsSync } from "node:fs";
const root = ".🧬semio/🦑️repo/⚡️cache/tests/results";
for (const d of readdirSync(root).filter((n) => n.includes("-692e06-"))) {
  const file = `${root}/${d}/📤️results.jsonl`;
  if (!existsSync(file)) continue;
  const rows = readFileSync(file, "utf8").split("\n").filter(Boolean).map((l) => JSON.parse(l));
  const bad = rows.filter((r) => r.status !== "passed");
  console.log(d.replace(/^.*-692e06-/, ""), rows.length, bad.length ? bad.map((r) => `${r.scenario}:${r.status} ${JSON.stringify(r.diagnostics).slice(0, 400)}`).join(" | ") : "all passed");
}
