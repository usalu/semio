/** 🧮️ Ticket tool of work package R: reads a Playwright JSON report and prints, per test result, its duration and the
 * steps that took longest — the evaluations in the page are the waits for a stillness, a walk or an encounter.
 *
 * Usage: node ".../wp_r_steps.mjs" <report.json> [slowest=6]
 */
import { readFileSync } from "node:fs";

const report = JSON.parse(readFileSync(process.argv[2], "utf8"));
const slowest = Number(process.argv[3] ?? 6);
const lines = [];
const flat = (steps, into = []) => {
  for (const step of steps ?? []) {
    into.push(step);
    flat(step.steps, into);
  }
  return into;
};
const walk = (suite) => {
  for (const spec of suite.specs ?? [])
    for (const test of spec.tests ?? [])
      for (const result of test.results ?? []) {
        const steps = flat(result.steps)
          .filter((step) => (step.steps ?? []).length === 0)
          .sort((a, b) => b.duration - a.duration)
          .slice(0, slowest)
          .map((step) => `${step.title} ${step.duration} ms`);
        lines.push(`${result.status} ${result.duration} ms — ${spec.title.slice(0, 60)}\n    ${steps.join("\n    ")}`);
      }
  for (const child of suite.suites ?? []) walk(child);
};
for (const suite of report.suites ?? []) walk(suite);
process.stdout.write(`${lines.join("\n")}\n`);
