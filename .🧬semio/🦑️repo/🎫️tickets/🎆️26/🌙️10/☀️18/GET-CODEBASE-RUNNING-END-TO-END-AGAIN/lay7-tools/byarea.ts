const d = JSON.parse(await Bun.file(process.argv[2]!).text());
const OS = "🧰️framework/🛍️products/💻️os/";
const m: Record<string, Record<string, number>> = {};
for (const r of d.rows) { const k = r.from.slice(OS.length).split("/").slice(0, 3).join("/"); (m[k] ??= {}); m[k]![r.to.split("/").slice(0,8).join("/").replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","")] = (m[k]![r.to] ?? 0) + 1; }
const tot = Object.entries(m).map(([k, v]) => [k, Object.values(v).length] as const).sort((a, b) => b[1] - a[1]);
for (const [k] of tot) console.log(k, JSON.stringify(m[k]));
