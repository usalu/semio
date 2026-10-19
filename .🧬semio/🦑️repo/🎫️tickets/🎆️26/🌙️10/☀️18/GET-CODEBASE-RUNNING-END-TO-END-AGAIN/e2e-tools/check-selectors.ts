/** 🔎 Static selector audit for the three end-to-end suites: every `data-*` hook, slot value, element id, `.semio-*` class and
 * pane/route token a spec uses must occur in non-test source. Usage: `bun check-selectors.ts <play|demonstrator|course>`. */
import { execFileSync } from "node:child_process";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const root = "/Users/ueli/Documents/semio";
const product = process.argv[2];
const specs: Record<string, string[]> = {
  play: ["🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts", "🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🧫️fixtures/🔀️roles/🔣️.json", "🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🧫️fixtures/🧊️generation3d/🔣️.json"],
  demonstrator: ["♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts"],
  course: [],
};
if (product === "course") {
  const base = "🎓️teaching/🏛️architecture/📚️course";
  const dirs = ["🚀️site-boot", "🪪️first-visit", "🥞️layered-home", "🎯️quiz-runs", "⛰️challenge-levels", "🏆️live-leaderboard", "🗣️both-languages", "📱️phone", "📐️adaptive-layout", "👥️shared-presence", "🔌️connection-shortage", "📴️proctor-away", "🛟️proctor-faults", "🐕️pet-walk", "🐕️pet-walk/🧭️cast-and-choices", "🐕️pet-walk/🪜️mischief-ladder-and-phone", "🐕️pet-walk/🧰️pet-kit"];
  specs.course = [`${base}/🎭️e2e/🚶️learner/🟦️.ts`, ...dirs.map((d) => `${base}/🧪️tests/${d}/🟦️.ts`)].filter((p) => existsSync(join(root, p)));
}
const camel = (value: string) => value.replace(/-([a-z0-9])/g, (_, c: string) => c.toUpperCase());
const tokens = new Map<string, { kind: string; alternatives: string[]; where: Set<string> }>();
const add = (kind: string, token: string, alternatives: string[], where: string) => {
  const key = `${kind}\u0000${token}`;
  const entry = tokens.get(key) ?? { kind, alternatives: [token, ...alternatives], where: new Set() };
  entry.where.add(where);
  tokens.set(key, entry);
};
for (const spec of specs[product] ?? []) {
  const text = readFileSync(join(root, spec), "utf8");
  const where = spec.split("/").slice(-2).join("/");
  for (const m of text.matchAll(/\bdata-[a-z][a-z0-9-]*/g)) add("attr", m[0], [camel(m[0].slice(5))], where);
  for (const m of text.matchAll(/data-slot(?:\^|\$|\*)?=\\?"([a-z][a-z0-9-]*)\\?"/g)) add("slot", m[1]!, [], where);
  for (const m of text.matchAll(/\[id=\\?"([^"\\]+)\\?"\]/g)) add("id", m[1]!, [], where);
  for (const m of text.matchAll(/\.semio-[a-z0-9-]+/g)) add("class", m[0].slice(1), [], where);
  for (const m of text.matchAll(/getByTestId\(\s*"([^"]+)"/g)) add("testid", m[1]!, [], where);
  for (const m of text.matchAll(/(?:"id":|dismissControlId":|controlId":)\s*"([^"]+)"/g)) add("id", m[1]!, [], where);
  for (const m of text.matchAll(/"(?:graph|controlKey|outputKey|examplePicker)":\s*"([^"]+)"/g)) add("fixture", m[1]!, [], where);
}
const files = execFileSync("git", ["ls-files", "-co", "--exclude-standard", "-z", "--", "*.ts", "*.tsx", "*.rs", "*.json", "*.css", "*.html"], { cwd: root, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter((p) => p && !/(^|\/)(🧪️tests|🎭️e2e|\.🧬semio|node_modules|dist)\//.test(p) && !/\.(spec|test)\./.test(p));
const remaining = new Map(tokens);
const hits = new Map<string, string>();
for (const file of files) {
  if (remaining.size === 0) break;
  let text: string;
  try { text = readFileSync(join(root, file), "utf8"); } catch { continue; }
  for (const [key, entry] of remaining) {
    const found = entry.alternatives.find((alternative) => text.includes(alternative));
    if (found !== undefined) { hits.set(key, file); remaining.delete(key); }
  }
}
console.log(JSON.stringify({ product, specs: specs[product], files: files.length, tokens: tokens.size, found: hits.size, missing: [...remaining].map(([key, entry]) => ({ kind: entry.kind, token: key.split("\u0000")[1], in: [...entry.where] })) }, null, 1));
