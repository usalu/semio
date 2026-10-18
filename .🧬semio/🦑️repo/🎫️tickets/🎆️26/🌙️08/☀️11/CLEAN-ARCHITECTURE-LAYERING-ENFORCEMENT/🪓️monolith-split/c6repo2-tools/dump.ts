import { parseFile } from "./lib.ts";
const p = parseFile(process.argv[2]);
for (const u of p.units) {
  const nm = [...u.names].join(",") || p.text.slice(u.start, u.start + 60).replace(/\s+/g, " ");
  console.log(`${u.id}\tL${u.startLine}\t${u.lines}\t${u.exported ? "E" : "-"}\t${u.kind.replace("Declaration", "")}\td${u.deps.size}\t${u.region.split(" > ").map((r) => r.slice(0, 22)).join(">")}\t${nm.slice(0, 110)}`);
}
