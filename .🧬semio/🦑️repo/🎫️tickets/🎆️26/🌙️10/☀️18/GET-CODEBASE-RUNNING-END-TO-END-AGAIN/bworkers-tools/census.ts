// 🔬 Census of execution commands by first token over the static graph.
const graph = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/bworkers-graph/graph.json").text());
const count = new Map<string, number>(), examples = new Map<string, string>();
let total = 0;
for (const project of Object.values<any>(graph.projects)) for (const [name, target] of Object.entries<any>(project.targets)) {
  const e = target.execution ?? {};
  const ops: string[] = e.kind === "noop" ? ["<noop>"] : e.program !== undefined ? [e.program] : e.commands ? e.commands.map((c: any) => typeof c === "string" ? c : c.command) : e.command ? [e.command] : ["<none>"];
  for (const op of ops) {
    total++;
    const toks = op.split(/\s+/);
    let key = toks[0]!;
    if (key === "bun") key = "bun " + (/^(test|build|x|run|install|exec)$/.test(toks[1] ?? "") ? toks[1] : /\.(ts|mjs|js|tsx)$/.test(toks[1] ?? "") ? "<script>" : toks[1]);
    count.set(key, (count.get(key) ?? 0) + 1);
    if (!examples.has(key)) examples.set(key, `${project.name}:${name} :: ${op.slice(0, 140)}`);
  }
}
console.log("operations", total);
for (const [k, v] of [...count].sort((a, b) => b[1] - a[1]).slice(0, 40)) console.log(String(v).padStart(5), k, "|", examples.get(k));
