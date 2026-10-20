import fs from "node:fs";
import path from "node:path";
import { altOf, entries, stepsOf, sha, setGroup, GROUPS } from "./lib.mjs";
const here = import.meta.dir;
const modules = { tokio: "./g1-tokio.mjs", json: "./g2-json.mjs", wasm: "./g3-wasm.mjs", react: "./g4-react.mjs" };
const only = process.argv.slice(2);
const out = path.join(here, "staged");
fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(path.join(out, "after"), { recursive: true });
const report = {};
for (const name of GROUPS) {
  if (only.length && !only.includes(name)) continue;
  if (!fs.existsSync(path.join(here, modules[name].slice(2)))) continue;
  setGroup(name);
  report[name] = (await import(modules[name])).build() ?? null;
}
const rows = [];
let index = 0;
for (const [file, state] of entries().sort(([a], [b]) => (a < b ? -1 : 1))) {
  index++;
  const ext = path.extname(file) || ".txt";
  const steps = [];
  let previous = state.before;
  for (const step of stepsOf(file)) {
    if (step.after === previous) continue;
    const name = `after/${String(index).padStart(4, "0")}-${step.group}${ext}`;
    fs.writeFileSync(path.join(out, name), step.after, "utf8");
    steps.push({ group: step.group, beforeSha256: previous === null ? null : sha(previous), afterSha256: sha(step.after), after: name, ...(steps.length === 0 && altOf(file).length ? { alsoPendingFrom: altOf(file) } : {}) });
    previous = step.after;
  }
  rows.push({ path: file, steps });
}
fs.writeFileSync(path.join(out, "manifest.json"), JSON.stringify({ version: 2, groups: GROUPS, root: "repository root", files: rows }, null, 1) + "\n");
const longest = Math.max(...rows.map((row) => [...row.path].length));
console.log(JSON.stringify({ files: rows.length, longestRelativePath: longest, byGroup: Object.fromEntries(GROUPS.map((g) => [g, rows.filter((r) => r.steps.some((s) => s.group === g)).length])), report }));
