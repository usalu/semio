import { load } from "./graph.mjs";
const { tops } = load(process.argv[2]);
const region = process.argv[3];
for (const t of tops) if (t.region === region) {
  const d = [...t.deps.keys()].map((x) => x.names[0] + (x.region === region ? "" : "^")).join(",");
  const u = tops.filter((o) => o.region === region && o.deps.has(t)).map((o) => o.names[0]).length;
  console.log(`${t.start}-${t.end} ${t.exported ? "+" : ""}${t.names.join("/")} usedIn:${u} deps:[${d}]`);
}
