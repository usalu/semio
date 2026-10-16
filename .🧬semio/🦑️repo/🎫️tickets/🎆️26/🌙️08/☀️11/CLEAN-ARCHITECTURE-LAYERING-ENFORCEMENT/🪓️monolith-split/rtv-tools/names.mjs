import { load } from "./graph.mjs";
const { tops } = load(process.argv[2]);
const by = new Map();
for (const t of tops) { if (!by.has(t.region)) by.set(t.region, []); by.get(t.region).push(t); }
for (const [r, ts] of by) { console.log(`\n## ${r} (${ts.reduce((a, t) => a + t.end - t.start + 1, 0)} lines)`); console.log(ts.map((t) => (t.exported ? "+" : "") + (t.names.join("/") || t.kind) + `:${t.end - t.start + 1}`).join("  ")); }
