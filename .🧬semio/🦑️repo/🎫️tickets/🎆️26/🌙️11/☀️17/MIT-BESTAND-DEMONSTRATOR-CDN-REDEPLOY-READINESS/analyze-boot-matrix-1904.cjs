// 📊 Parses the static Playwright JSON report into per-test digests for the boot-matrix audit.
const fs = require("fs");
const path = require("path");
const dir = path.dirname(process.argv[1]);
const json = JSON.parse(fs.readFileSync(path.join(dir, "🗑️generated", "static-playwright.json"), "utf8"));
const strip = (s) => s.replace(/\x1b\[[0-9;]*m/g, "");
const rows = [];
function walk(suite, trail) {
  for (const child of suite.suites || []) walk(child, trail.concat(child.title));
  for (const spec of suite.specs || []) {
    for (const t of spec.tests) for (const r of t.results) {
      rows.push({ spec: spec.title, project: t.projectName, status: r.status, duration: r.duration, errors: (r.errors || []).map((e) => strip(e.message || "")) });
    }
  }
}
json.suites.forEach((s) => walk(s, []));
const classify = (msg) => {
  const lines = msg.split("\n");
  const head = lines[0];
  const consoleItems = [];
  if (head.startsWith("Error: unexpected browser console errors")) {
    for (const l of lines) {
      const m = l.match(/^\+\s{3}"(.*)",?$/);
      if (m) consoleItems.push(m[1]);
    }
  }
  return { head, consoleItems };
};
const order = ["generator", "koordinator", "aggregator", "energie", "aussuchen", "bearbeiten", "verfolgen", "statik"];
for (const r of rows.sort((a, b) => order.indexOf(a.spec.split(" ")[0]) - order.indexOf(b.spec.split(" ")[0]) || a.project.localeCompare(b.project))) {
  console.log(`=== ${r.spec} [${r.project}] status=${r.status} duration=${r.duration}ms`);
  for (const e of r.errors) {
    const c = classify(e);
    console.log(`  - ${c.head.slice(0, 200)}`);
    if (c.consoleItems.length) {
      const counts = new Map();
      for (const it of c.consoleItems) {
        const key = it.startsWith("[DEBUG] leaf prior_slots") ? "[DEBUG] leaf prior_slots=... (noise)"
          : it.startsWith("[DEBUG] marker") ? "[DEBUG] marker ... (noise)"
          : it.replace(/epoch=\d+/g, "epoch=N").slice(0, 400);
        counts.set(key, (counts.get(key) || 0) + 1);
      }
      for (const [k, n] of counts) console.log(`      console x${n}: ${k}`);
    }
    const body = e.split("\n").filter((l) => /^\s*>|Expected|Received|at \/|waiting for|TimeoutError|locator|Call log/.test(l)).slice(0, 12);
    for (const l of body) console.log(`      | ${l.slice(0, 300)}`);
  }
}
