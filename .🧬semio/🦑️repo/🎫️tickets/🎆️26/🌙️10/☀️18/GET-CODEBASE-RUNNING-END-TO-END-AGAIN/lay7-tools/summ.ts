const d = JSON.parse(await Bun.file(process.argv[2]!).text());
const REPO = "🧰️framework/🛍️products/🦑️repo/";
const m: Record<string, number> = {};
for (const [t, n] of Object.entries(d.targets as Record<string, number>)) m[t] = n;
console.log(Object.entries(m).sort((a, b) => b[1] - a[1]).map(([t, n]) => `${n}\t${t.slice(REPO.length)}`).join("\n"));
const fm: Record<string, number> = {};
for (const r of d.rows) { const k = r.from.split("/").slice(0, 8).join("/").replace("🧰️framework/🛍️products/💻️os/", ""); fm[k] = (fm[k] ?? 0) + 1; }
console.log("----"); console.log(JSON.stringify(fm, null, 1));
