import { Workspace, plain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🧰️support/🟦️.ts";
const ws = await Workspace.open("debug");
try {
  for (const i of [0, 1, 2]) { const r = await ws.semio(["run", "tool:workspace/idle", "--env", `SESSION=${i}`, "--detach"]); console.log("idle", i, r.code, JSON.stringify(r.stdout.slice(0, 200)), JSON.stringify(r.stderr.slice(0, 300))); }
  console.log(JSON.stringify((await ws.tasks()).map((t) => [t.place, t.status, t.code, t.commandId])));
  const log = await ws.ok(["logs", "1"]);
  console.log("log1", JSON.stringify(plain(log).slice(0, 300)));
  const bytes = 3 * 1024 * 1024;
  const run = await ws.semio(["run", "tool:workspace/burst", "--param", `bytes=${bytes}`], { timeoutMs: 120000 });
  const text = plain(run.stdout);
  console.log("burst", run.code, run.stdout.length, JSON.stringify(text.slice(-200)));
  const lines = text.split("\n").filter(Boolean);
  console.log("lines", lines.length, "tail", JSON.stringify(lines.slice(-3)));
} finally { await ws.close(); }
