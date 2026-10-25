import { readFileSync, writeFileSync } from "node:fs";
const [x, pk, out] = process.argv.slice(2) as [string, string, string];
const packages: any[] = JSON.parse(readFileSync(pk, "utf8"));
const rows: any[] = JSON.parse(readFileSync(x, "utf8"));
const edges = rows.flatMap((v) => packages.filter((p) => p.sourceOwner === v.owner).map((p) => ({ rule: "no-cross-package-relative-" + p.name, from: v.from, to: v.to })));
writeFileSync(out, JSON.stringify({ edges }, null, 1));
console.log(edges.length);
