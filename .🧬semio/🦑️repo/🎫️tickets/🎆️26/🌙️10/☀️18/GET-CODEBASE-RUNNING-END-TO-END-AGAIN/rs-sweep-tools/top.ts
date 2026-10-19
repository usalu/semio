const f = process.argv[2], kind = process.argv[3], n = +(process.argv[4] ?? 25), re = process.argv[5];
const l = require("fs").readFileSync(f, "utf8").trim().split("\n").map((x: string) => JSON.parse(x)).filter((x: any) => x.kind === kind);
const key = (x: any) => { const m = re ? new RegExp(re).exec(x.msg) : null; return m ? m[1] : x.msg.slice(0, 120); };
const c = new Map<string, { n: number; ex: any }>();
for (const x of l) { const k = key(x); const e = c.get(k); if (e) e.n++; else c.set(k, { n: 1, ex: x }); }
console.log(kind, l.length, "distinct", c.size);
for (const [k, v] of [...c].sort((a, b) => b[1].n - a[1].n).slice(0, n)) console.log(String(v.n).padStart(6), k.slice(0, 150), "|", v.ex.crate, v.ex.file.slice(-70) + ":" + v.ex.line);
