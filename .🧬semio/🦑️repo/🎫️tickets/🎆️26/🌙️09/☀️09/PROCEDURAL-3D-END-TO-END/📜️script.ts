/** 🎫️ Ticket-owned repo MCP client for the procedural widget completion. */
const [method, params] = process.argv.slice(2);
const child = Bun.spawn(["bun", "./📜️script.ts", "dev", "mcp", "stdio", "client"], { stdin: "pipe", stdout: "pipe", stderr: "inherit" });
const send = (id: number | undefined, method: string, params: unknown) => child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
const timeout = setTimeout(() => { child.kill(); process.exitCode = 1; }, 30000);
send(1, "initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "codex", version: "1" } });
let pending = "";
for await (const bytes of child.stdout) {
  pending += new TextDecoder().decode(bytes);
  let end: number;
  while ((end = pending.indexOf("\n")) >= 0) {
    const line = pending.slice(0, end);
    pending = pending.slice(end + 1);
    if (!line.trim()) continue;
    const response = JSON.parse(line);
    if (response.id === 1) {
      send(undefined, "notifications/initialized", {});
      setTimeout(() => send(2, method, JSON.parse(params)), 300);
    } else if (response.id === 2) {
      console.log(JSON.stringify(response, null, 2));
      clearTimeout(timeout);
      child.kill();
    }
  }
}
