import { readFileSync } from "node:fs";
import { dirname, resolve, relative } from "node:path";
const a = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const root = "/Users/ueli/Documents/semio";
const cycles: string[][] = a.rows.filter((r: any) => r.rule === "no-circular").map((r: any) => { const c = [r.from, ...r.cycle]; if (c[c.length - 1] === c[0]) c.pop(); return c; });
const edges = new Map<string, number>();
for (const c of cycles) for (let i = 0; i < c.length; i++) { const e = c[i] + "\t" + c[(i + 1) % c.length]; edges.set(e, (edges.get(e) ?? 0) + 1); }
const filter = process.argv[3] ?? "";
const strip = (p: string) => p.replace(/️/gu, "");
for (const [e, n] of [...edges.entries()].sort((x, y) => y[1] - x[1])) {
  const [from, to] = e.split("\t") as [string, string];
  if (filter && !from.includes(filter)) continue;
  const base = dirname(resolve(root, from));
  const lines = readFileSync(resolve(root, from), "utf8").split("\n");
  const hits: string[] = [];
  lines.forEach((line, i) => {
    for (const m of line.matchAll(/["'](\.[^"']*)["']/gu)) {
      const rel = relative(root, resolve(base, m[1]!));
      if (rel === to || rel === to.replace(/\/[^/]*$/u, "") || to.replace(/\.[a-z]+$/u, "") === rel) { hits.push(`${i + 1}: ${line.trim().slice(0, 240)}`); break; }
    }
  });
  console.log(`${n}x ${strip(from.replace("🧰️framework/", "FW/"))}\n    => ${strip(to.replace("🧰️framework/", "FW/"))}\n${hits.length ? hits.map((h) => "       " + strip(h)).join("\n") : "       (alias/dynamic import)"}`);
}
