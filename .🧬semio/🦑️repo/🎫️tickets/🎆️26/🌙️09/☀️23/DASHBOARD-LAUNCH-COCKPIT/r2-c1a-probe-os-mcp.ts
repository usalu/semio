const root = new URL("../../../../../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1");
const budgetMs = Number(process.argv[2] ?? 150_000);
const clientFile = process.argv[3] ?? ".mcp.json";
const serverName = process.argv[4] ?? "semio";
const entry = (JSON.parse(await Bun.file(`${root}${clientFile}`).text()).mcpServers[serverName]) as { command: string; args: string[] };
console.log(`[probe] ${entry.command} ${entry.args.join(" ")}`);
const child = Bun.spawn([entry.command, ...entry.args], { cwd: root, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env, CARGO_TARGET_DIR: process.env.CARGO_TARGET_DIR ?? `${root}.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-c1a` } });
const started = Date.now();
const initialize = { jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "r2-c1a-probe", version: "0" } } };
child.stdin.write(`${JSON.stringify(initialize)}\n`);
let stderr = "";
(async () => { for await (const chunk of child.stderr) stderr += new TextDecoder().decode(chunk); })();
const reader = child.stdout.getReader();
let buffer = "";
const outcome = await Promise.race([
  (async () => {
    for (;;) {
      const { value, done } = await reader.read();
      if (done) return { kind: "exit" as const };
      buffer += new TextDecoder().decode(value);
      const line = buffer.split("\n").find((candidate) => candidate.includes('"id":1'));
      if (line) return { kind: "response" as const, line };
    }
  })(),
  new Promise<{ kind: "timeout" }>((resolve) => setTimeout(() => resolve({ kind: "timeout" }), budgetMs)),
]);
const elapsed = Date.now() - started;
child.kill();
const exitCode = await Promise.race([child.exited, new Promise<null>((resolve) => setTimeout(() => resolve(null), 3000))]);
console.log(`[probe] outcome=${outcome.kind} after ${elapsed} ms exit=${exitCode}`);
if (outcome.kind === "response") console.log(`[probe] response: ${outcome.line.slice(0, 400)}`);
console.log(`[probe] stderr tail:\n${stderr.split("\n").slice(-12).join("\n")}`);
process.exit(outcome.kind === "response" ? 0 : 1);
