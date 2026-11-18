import fs from "node:fs";
const G = process.cwd() + "/" + fs.readdirSync(".").find((n) => n.endsWith("generated")) + "/";
const rd = (pre) => { const o = {}; for (const f of fs.readdirSync(G).filter((f) => f.startsWith(pre) && f.endsWith(".jsonl"))) for (const l of fs.readFileSync(G + f, "utf8").trim().split("\n")) { if (!l) continue; const j = JSON.parse(l); o[j.asset] = j; } return o; };
const head = {};
const cur = { ...rd("sil-current-"), ...rd("sil-veg-") };
const cat = {}; for (const l of fs.readFileSync(process.argv[2], "utf8").trim().split("\n")) { const [c, a] = l.split(" "); cat[a] = c; }
const key = (x) => `${x.scale}${x.kind}${x.view ?? x.axis ?? ""}`;
const rows = [];
for (const a of Object.keys(cur)) {
  const c = cur[a].fails ?? [], h = head[a]?.fails;
  const kinds = { bbox: 0, iou: 0, hausdorff: 0 }; for (const f of c) kinds[f.kind]++;
  const scales = [...new Set(c.map((f) => f.scale))].sort((x, y) => y - x).map((s) => "1:" + s).join(" ");
  const worst = c.filter((f) => f.kind === "hausdorff").sort((x, y) => y.h / y.limit - x.h / x.limit)[0];
  rows.push({ a, cat: cat[a] ?? "?", n: c.length, ...kinds, scales, worst: worst ? `${worst.h} m vs ${worst.limit} (1:${worst.scale} ${worst.view})` : "", head: h ? h.length : "n/a" });
}
rows.sort((x, y) => x.cat.localeCompare(y.cat) || x.n - y.n);
let out = "| Category | Asset | Failing checks | IoU | Hausdorff | bbox | Scales | Worst Hausdorff |\n|---|---|---:|---:|---:|---:|---|---|\n";
for (const r of rows) out += `| ${r.cat} | ${r.a} | ${r.n} | ${r.iou} | ${r.hausdorff} | ${r.bbox} | ${r.scales} | ${r.worst} |\n`;
fs.writeFileSync(G + "table.md", out);
const tot = { assets: rows.length, failing: rows.filter((r) => r.n).length, checks: rows.reduce((s, r) => s + r.n, 0), iou: rows.reduce((s, r) => s + r.iou, 0), hd: rows.reduce((s, r) => s + r.hausdorff, 0), bbox: rows.reduce((s, r) => s + r.bbox, 0) };
console.log(JSON.stringify(tot));
const by = {}; for (const r of rows) { by[r.cat] ??= { a: 0, f: 0, n: 0 }; by[r.cat].a++; by[r.cat].f += r.n ? 1 : 0; by[r.cat].n += r.n; } console.log(JSON.stringify(by));
const sc = {}; for (const a of Object.keys(cur)) for (const f of cur[a].fails ?? []) { const k = `1:${f.scale} ${f.kind}`; sc[k] = (sc[k] || 0) + 1; } console.log(JSON.stringify(sc));
console.log("easy (<=3 checks):", rows.filter((r) => r.n > 0 && r.n <= 3).map((r) => r.a).join(", "));
