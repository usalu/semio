const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad";
const data: Record<string, { type: string; body: string }> = JSON.parse(await Bun.file(`${S}/pj-contents.json`).text());
const dev: string[] = []; const cls: Record<string, number> = {};
let total = 0, withCmd = 0;
for (const [p, v] of Object.entries(data)) {
  if (v.type === "missing") continue;
  const j = JSON.parse(v.body);
  for (const [name, t] of Object.entries<any>(j.targets ?? {})) {
    total++;
    const cmds: string[] = [];
    if (t.options?.command) cmds.push(String(t.options.command));
    for (const c of t.options?.commands ?? []) cmds.push(String(c.command ?? c));
    const ex = t.executor ?? (cmds.length ? "IMPLICIT:run-commands" : "NONE");
    if (cmds.length) withCmd++;
    cls[ex] = (cls[ex] ?? 0) + 1;
    if (ex === "NONE") continue;
    if (ex !== "repo:script") dev.push(`${p} :: ${name} :: executor=${ex} :: ${cmds.join(" && ").slice(0, 170)}`);
    else for (const c of cmds) if (!c.includes("📜️script.ts")) dev.push(`${p} :: ${name} :: repo:script NON-SCRIPT :: ${c.slice(0, 170)}`);
  }
}
console.log({ total, withCmd, cls });
console.log("deviations", dev.length);
const byExec: Record<string, number> = {};
for (const d of dev) { const k = d.split(" :: ")[2]; byExec[k] = (byExec[k] ?? 0) + 1; }
console.log(byExec);
await Bun.write(`${S}/pj-deviations.txt`, dev.join("\n") + "\n");
