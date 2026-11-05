import { spawn } from "node:child_process";
import { createInterface } from "node:readline";
const child = spawn("repo", ["mcp"], { cwd: process.cwd(), env: process.env, shell: process.platform === "win32", stdio: ["pipe", "pipe", "inherit"] });
const pending = new Map<number, (value: any) => void>();
let sequence = 0;
createInterface({ input: child.stdout! }).on("line", line => { try { const value = JSON.parse(line); if (value.id !== undefined) pending.get(value.id)?.(value); } catch {} });
const request = (method: string, params: unknown) => new Promise<any>(resolve => { const id = ++sequence; pending.set(id, resolve); child.stdin!.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n"); });
try {
 await request("initialize", { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "codex-native-inspect", version: "1" } });
 child.stdin!.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
 const list = await request("tools/list", {});
 console.log(JSON.stringify(list.result.tools.filter((tool: any) => /doctor|process|toolchain|native|cargo|capacity|lease|status/.test(tool.name)).map((tool: any) => ({ name: tool.name, description: tool.description, inputSchema: tool.inputSchema }))));
} finally { child.stdin!.end(); child.kill(); }
