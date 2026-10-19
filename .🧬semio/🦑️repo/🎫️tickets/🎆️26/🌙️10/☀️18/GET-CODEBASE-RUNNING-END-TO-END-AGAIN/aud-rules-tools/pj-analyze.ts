const S = process.argv[2];
const data: Record<string, { type: string; body: string }> = JSON.parse(await Bun.file(`${S}/pj-contents.json`).text());
const SCRIPT = "📜️script.ts";
const execs: Record<string, number> = {};
const cmdForms: Record<string, number> = {};
const deviations: string[] = [];
const noExec: string[] = [];
const parseErr: string[] = [];
let targets = 0;
for (const [p, v] of Object.entries(data)) {
  if (v.type === "missing") continue;
  let j: any;
  try { j = JSON.parse(v.body); } catch { parseErr.push(p); continue; }
  for (const [name, t] of Object.entries<any>(j.targets ?? {})) {
    targets++;
    const ex = t.executor ?? (t.command ? "(command-shorthand)" : "(none)");
    execs[ex] = (execs[ex] ?? 0) + 1;
    const cmds: string[] = [];
    if (t.options?.command) cmds.push(String(t.options.command));
    for (const c of t.options?.commands ?? []) cmds.push(String(c.command ?? c));
    if (t.command) cmds.push(String(t.command));
    if (ex === "(none)") { noExec.push(`${p}::${name}`); continue; }
    if (cmds.length === 0) {
      deviations.push(`${p}::${name} executor=${ex} NO-COMMAND`);
      continue;
    }
    for (const c of cmds) {
      const form = c.includes(SCRIPT) ? c.replace(/\S*📜️script\.ts/, "<PATH>/📜️script.ts").replace(/^(\S+)\s+<PATH>/, "$1 <PATH>").split(" ").slice(0, 2).join(" ") : "NON-SCRIPT";
      cmdForms[`${ex} :: ${form}`] = (cmdForms[`${ex} :: ${form}`] ?? 0) + 1;
      if (!c.includes(SCRIPT) || ex !== "repo:script") deviations.push(`${p}::${name} executor=${ex} cmd=${c.slice(0, 160)}`);
    }
  }
}
console.log("targets", targets, "parseErr", parseErr.length, parseErr.slice(0, 5));
console.log("executors:", JSON.stringify(execs, null, 1));
console.log("no-executor targets (aggregators/dependsOn-only):", noExec.length);
console.log("cmd forms:"); for (const [k, n] of Object.entries(cmdForms).sort((a, b) => b[1] - a[1])) console.log(n, k);
console.log("deviations:", deviations.length);
await Bun.write(`${S}/pj-deviations.txt`, deviations.join("\n") + "\n");
await Bun.write(`${S}/pj-noexec.txt`, noExec.join("\n") + "\n");
