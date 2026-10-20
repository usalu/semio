const f = process.argv[2], n = +(process.argv[3] ?? 60), kinds = process.argv[4]?.split(",");
const l = require("fs").readFileSync(f, "utf8").trim().split("\n").map((x: string) => JSON.parse(x)).filter((x: any) => !kinds || kinds.includes(x.kind));
const norm = (x: any) => {
  let m = x.msg.replace(/ \[[^\]]*\]$/, "").replace(/\(defined as:.*\)$/, "").replace(/^(use|path|macro) `[^`]*`: /, "");
  return `${x.kind} ${x.pkg ?? x.crate.split("#")[0]} :: ${m.slice(0, 110)}`;
};
const c = new Map<string, { n: number; files: Set<string>; ex: any }>();
for (const x of l) { const k = norm(x); let e = c.get(k); if (!e) c.set(k, (e = { n: 0, files: new Set(), ex: x })); e.n++; e.files.add(x.file); }
console.log("findings", l.length, "clusters", c.size);
for (const [k, v] of [...c].sort((a, b) => b[1].n - a[1].n).slice(0, n)) console.log(String(v.n).padStart(5), String(v.files.size).padStart(3), k);
