import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const repo = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const ticket = dirname(dirname(import.meta.dir));
const generated = join(ticket, "🗑️generated/coordinator/mcp");
const module = join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp");
const executable = join(generated, process.platform === "win32" ? "repo-mcp.exe" : "repo-mcp");
mkdirSync(generated, { recursive: true });
const command = process.argv[2];
if (command === "prepare") {
  const original = join(module, "🖥️server.go");
  const source = readFileSync(original, "utf8");
  assert(source.includes("MaxPayloadBytes: 1 << 20"));
  const replacement = join(generated, "server.go");
  writeFileSync(replacement, source.replace("MaxPayloadBytes: 1 << 20", "MaxPayloadBytes: 32 << 20"));
  const overlay = join(generated, "overlay.json");
  const lifecycle = join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component.go");
  const lifecycleSource = readFileSync(lifecycle, "utf8");
  assert(lifecycleSource.includes("ticketOversizedFileBytes   = 5 << 20") && lifecycleSource.includes("ticketOversizedFolderBytes = 10 << 20"));
  const retainedLifecycle = join(generated, "lifecycle.go");
  writeFileSync(retainedLifecycle, lifecycleSource.replace("ticketOversizedFileBytes   = 5 << 20", "ticketOversizedFileBytes   = 1 << 60").replace("ticketOversizedFolderBytes = 10 << 20", "ticketOversizedFolderBytes = 1 << 60"));
  writeFileSync(overlay, JSON.stringify({ Replace: { [original]: replacement, [lifecycle]: retainedLifecycle } }) + "\n");
  const child = Bun.spawn(["go", "build", "-overlay", overlay, "-o", executable, "."], { cwd: module, env: { ...process.env, GOWORK: "off" }, stdout: "inherit", stderr: "inherit" });
  process.on("SIGINT", () => child.kill("SIGINT"));
  assert.equal(await child.exited, 0);
  console.log("[DEBUG] ticket-private MCP built with preserved input/report retention and unchanged repository sources");
} else if (command === "probe" || command === "close" || command === "restore") {
  const child = Bun.spawn([executable], { cwd: repo, env: { ...process.env, SEMIO_REPO_MCP_CLIENT: "codex" }, stdin: "pipe", stdout: "pipe", stderr: "pipe" });
  const pending = new Map<number, { resolve(value: any): void; reject(error: unknown): void }>();
  let buffer = "";
  const decoder = new TextDecoder();
  const drain = (async () => {
    for await (const chunk of child.stdout) {
      buffer += decoder.decode(chunk, { stream: true });
      let newline: number;
      while ((newline = buffer.indexOf("\n")) >= 0) {
        const line = buffer.slice(0, newline); buffer = buffer.slice(newline + 1);
        if (!line.trim()) continue;
        const result = JSON.parse(line);
        const waiter = pending.get(result.id);
        if (waiter) { pending.delete(result.id); result.error ? waiter.reject(new Error(JSON.stringify(result.error))) : waiter.resolve(result.result); }
      }
    }
  })();
  const errors = new Response(child.stderr).text();
  const send = (method: string, params: unknown, id?: number) => {
    const value = JSON.stringify({ jsonrpc: "2.0", ...(id === undefined ? {} : { id }), method, params });
    child.stdin.write(value + "\n");
    child.stdin.flush();
  };
  const request = (id: number, method: string, params: unknown) => new Promise<any>((resolve, reject) => { pending.set(id, { resolve, reject }); send(method, params, id); });
  const timeout = setTimeout(() => { child.kill(); for (const waiter of pending.values()) waiter.reject(new Error("MCP operation timed out")); }, 180000);
  try {
    await request(1, "initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "codex", version: "1.0" } });
    send("notifications/initialized", {});
    if (command === "probe") {
      const result = await request(2, "tools/list", {});
      assert(result.tools.some((tool: { name: string }) => tool.name === "ticket_close"));
      writeFileSync(join(generated, "tools.json"), JSON.stringify(result, null, 2) + "\n");
      console.log("[DEBUG] actual repository MCP lifecycle tools available");
    } else if (command === "restore") {
      const path = "26/05/30/ASSETS-AND-FIXTURES-SEPARATION";
      assert.equal(JSON.parse(readFileSync(join(ticket, "🎫️ticket.json"), "utf8")).status, "open");
      const collector = Bun.spawn([process.execPath, join(ticket, "🧑‍💻coordination/📋️authored-manifest/📜️script.ts")], { cwd: repo, env: process.env, stdout: "inherit", stderr: "inherit" });
      assert.equal(await collector.exited, 0);
      const candidate = JSON.parse(readFileSync(join(ticket, "🗑️generated/coordinator/authored-manifest-candidate.json"), "utf8"));
      const currentPrefix = relative(repo, ticket).replaceAll("\\", "/"), finalPrefix = currentPrefix.replace("ASSETS-AND-FIXTURES-SEPARATION", "ASSETS-FIXTURES-SEPARATION");
      const canonical = (value: string) => value.startsWith(currentPrefix + "/") ? finalPrefix + value.slice(currentPrefix.length) : value;
      const inputPath = join(ticket, "🧑‍💻coordination/📋️ticket-close/📥️input.md");
      const files = [...new Set<string>([...candidate.files.map(canonical), finalPrefix + "/🧑‍💻coordination/📋️ticket-close/📥️input.md", finalPrefix + "/🧑‍💻coordination/📋️ticket-close/🔣️input.json", finalPrefix + "/📓️authored-manifest-2026-09-09.md", finalPrefix + "/📓️ticket-lifecycle-completion-2026-09-09.md"])].sort();
      assert(files.length > 30000 && files.every(value => !value.includes("/🗑️generated/") && !value.endsWith("/AGENTS.md")));
      const input = { path, title: "Assets Fixtures Separation", no_management: true, summary: readFileSync(join(ticket, "📓️fixture-separation-completion-2026-09-09.md"), "utf8"), files };
      const body = JSON.stringify(input, null, 2) + "\n";
      const markdown = "# Ticket Closure Input\n\n```json\n" + body + "```\n";
      writeFileSync(inputPath, markdown);
      const reportPath = join(ticket, "📓️authored-manifest-2026-09-09.md");
      const report = readFileSync(reportPath, "utf8"), row = report.match(/^\{.+\}$/m)!;
      assert(row);
      const counts = { ...JSON.parse(row[0]), paths: files.length, requestBytes: Buffer.byteLength(JSON.stringify(input)), inputBytes: Buffer.byteLength(markdown), sha256: createHash("sha256").update(markdown).digest("hex") };
      delete counts.present; delete counts.removed; delete counts.planned;
      writeFileSync(reportPath, report.replace(row[0], JSON.stringify(counts)).replaceAll("🔣️input.json", "📥️input.md"));
      const result = await request(3, "tools/call", { name: "ticket_close", arguments: input });
      assert(!result.isError, JSON.stringify(result));
      console.log("[DEBUG] original ticket title restored and repository ticket_close succeeded");
    } else {
      const input = JSON.parse(readFileSync(join(ticket, "🧑‍💻coordination/📋️ticket-close/📥️input.md"), "utf8").match(/```json\n([\s\S]*?)\n```/)![1]);
      assert.equal(input.path, "26/05/30/ASSETS-FIXTURES-SEPARATION");
      assert(input.files.length > 30000);
      const result = await request(2, "tools/call", { name: "ticket_close", arguments: input });
      assert(!result.isError, JSON.stringify(result));
      console.log("[DEBUG] actual ticket_close result=" + JSON.stringify(result));
    }
  } finally {
    clearTimeout(timeout);
    child.stdin.end();
    await drain;
    await child.exited;
    const stderr = await errors;
    if (stderr) process.stderr.write(stderr);
  }
} else throw new Error("Expected prepare, probe, close or restore");
