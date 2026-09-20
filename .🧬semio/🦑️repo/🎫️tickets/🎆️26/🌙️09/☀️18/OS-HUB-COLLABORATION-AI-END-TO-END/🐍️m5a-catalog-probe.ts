/**
 * 🔎️ M5a — drives the REAL `semio-os-mcp` stdio server exactly the way `.mcp.json`'s `semio` entry
 * does (same argv, same `--folder .`/`--scopes`) and captures what an AI agent actually sees when it
 * looks for a verb: three `capabilities_search` queries plus one `capabilities_describe`, printed
 * with the fields that decide tool choice (title, description, audience, score, input schema).
 *
 * Usage: `bun <this file> [--label before|after] [--bin <path>]`
 */
import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const args = process.argv.slice(2);
const flag = (name: string, fallback: string): string => {
  const index = args.indexOf(`--${name}`);
  return index >= 0 && args[index + 1] ? String(args[index + 1]) : fallback;
};
const label = flag("label", "before");
const binOverride = flag("bin", "");

type Response = { id: number | string | null; result?: unknown; error?: { code: number; message: string; data?: unknown } };

const QUERIES = (flag("queries", "") ? flag("queries", "").split("|") : ["draw rectangle", "add a layer to the drawing", "export the document as pdf"]) as readonly string[];
/** 🔎️ Optional `--kind mutation` narrowing, so the probe can ask the exact question the live agent
 * loop's (c)/(e) precondition asks. */
const KIND = flag("kind", "");

function mcpConfigCommand(): { command: string; argv: string[] } {
  const config = JSON.parse(readFileSync(resolve(repoRoot, ".mcp.json"), "utf8")) as { mcpServers: Record<string, { command: string; args: string[] }> };
  const entry = config.mcpServers.semio;
  if (!entry) throw new Error(".mcp.json has no `semio` server entry");
  return { command: entry.command, argv: entry.args };
}

async function main(): Promise<void> {
  const direct = binOverride.length > 0;
  const spawned = direct ? { command: resolve(repoRoot, binOverride), argv: ["stdio", "--folder", ".", "--scopes", "workspace.read,artifact.write,inference.execute,ui.observe,ui.control"] } : mcpConfigCommand();
  const lines: string[] = [];
  const say = (text: string): void => {
    lines.push(text);
    console.log(text);
  };
  say(`# m5a capability-catalog capture — ${label}`);
  say(`# spawn: ${spawned.command} ${spawned.argv.join(" ")}`);
  say(`# cwd:   ${repoRoot}`);
  say(`# utc:   ${new Date().toISOString()}`);

  const child = spawn(spawned.command, spawned.argv, { cwd: repoRoot, stdio: ["pipe", "pipe", "pipe"], env: { ...process.env, CLAUDE_CODE_SESSION_ID: process.env.CLAUDE_CODE_SESSION_ID ?? "m5a-probe", CLAUDE_CODE_MESSAGING_TOKEN: process.env.CLAUDE_CODE_MESSAGING_TOKEN ?? "m5a-probe" } });
  const pending = new Map<number, (value: Response) => void>();
  const stderr: string[] = [];
  let buffer = "";
  child.stdout.setEncoding("utf8");
  child.stdout.on("data", (chunk: string) => {
    buffer += chunk;
    let index = buffer.indexOf("\n");
    while (index >= 0) {
      const line = buffer.slice(0, index).trim();
      buffer = buffer.slice(index + 1);
      if (line.length > 0) {
        try {
          const message = JSON.parse(line) as Response;
          const settle = typeof message.id === "number" ? pending.get(message.id) : undefined;
          if (settle) {
            pending.delete(message.id as number);
            settle(message);
          }
        } catch {
          /* 🙈️ non-JSON chatter on stdout is a server bug, captured via stderr instead */
        }
      }
      index = buffer.indexOf("\n");
    }
  });
  child.stderr.setEncoding("utf8");
  child.stderr.on("data", (chunk: string) => stderr.push(chunk));

  let nextId = 1;
  const call = async (method: string, params?: unknown): Promise<Response> => {
    const id = nextId++;
    const promise = new Promise<Response>((settle, reject) => {
      pending.set(id, settle);
      setTimeout(() => reject(new Error(`timeout waiting for ${method}`)), 120_000);
    });
    child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
    return promise;
  };

  const initialize = await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m5a-probe", version: "0.0.0" } });
  const serverInfo = (initialize.result as { serverInfo?: { name?: string; version?: string } } | undefined)?.serverInfo;
  say(`\n## initialize — ${serverInfo?.name ?? "?"}@${serverInfo?.version ?? "?"}`);
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);

  const context = await call("tools/call", { name: "context_resolve", arguments: { principal: "agent:local", locale: "en" } });
  const summary = (context.result as { structuredContent?: { catalogHash?: string; capabilityCount?: number } } | undefined)?.structuredContent;
  say(`## context_resolve — catalogHash=${summary?.catalogHash ?? "?"} capabilities=${summary?.capabilityCount ?? "?"}`);

  for (const query of QUERIES) {
    const response = await call("tools/call", { name: "capabilities_search", arguments: KIND ? { query, kind: [KIND] } : { query } });
    const structured = (response.result as { structuredContent?: { results?: Array<Record<string, unknown>>; total?: number; nextCursor?: string | null } } | undefined)?.structuredContent;
    const results = structured?.results ?? [];
    say(`\n## capabilities_search ${JSON.stringify(query)} — ${results.length} result(s), total=${structured?.total ?? "n/a"} nextCursor=${String(structured?.nextCursor ?? "n/a")}`);
    say(`| # | score | capabilityId | title | description |`);
    say(`|---|---|---|---|---|`);
    results.forEach((hit, position) => {
      const score = typeof hit.score === "number" ? hit.score.toFixed(4) : "?";
      say(`| ${position + 1} | ${score} | ${String(hit.capabilityId)} | ${String(hit.title ?? "")} | ${String(hit.description ?? "")} |`);
    });
    const distinct = new Set(results.map((hit) => (typeof hit.score === "number" ? hit.score.toFixed(4) : "?")));
    const empty = results.filter((hit) => String(hit.description ?? "").length === 0).length;
    say(`> distinct scores: ${distinct.size}/${results.length} · empty descriptions: ${empty}/${results.length}`);
  }

  for (const capabilityId of ["draw.s.draw.drawing@1/*#editor.addLayer", "draw.s.draw.drawing@1/*#editor.canvasPointerMove"]) {
    const response = await call("tools/call", { name: "capabilities_describe", arguments: { capabilityId } });
    const structured = (response.result as { structuredContent?: Record<string, unknown> } | undefined)?.structuredContent;
    say(`\n## capabilities_describe ${JSON.stringify(capabilityId)}`);
    if (!structured) {
      say(`(no structuredContent) ${JSON.stringify(response.error ?? response.result)}`);
      continue;
    }
    say(`title=${JSON.stringify(structured.title)} kind=${JSON.stringify(structured.kind)} audience=${JSON.stringify(structured.audience ?? null)}`);
    say(`description=${JSON.stringify(structured.description)}`);
    say(`useWhen=${JSON.stringify(structured.useWhen ?? [])}`);
    say(`inputSchema=${JSON.stringify(structured.inputSchema)}`);
  }

  child.stdin.end();
  child.kill();
  const diagnostics = stderr.join("");
  const skips = diagnostics.split("\n").filter((line) => line.includes("skipping plugin")).length;
  say(`\n## registry diagnostics — ${skips} skip line(s)`);
  process.stdout.write("");
  await new Promise((settle) => setTimeout(settle, 50));
}

await main();
