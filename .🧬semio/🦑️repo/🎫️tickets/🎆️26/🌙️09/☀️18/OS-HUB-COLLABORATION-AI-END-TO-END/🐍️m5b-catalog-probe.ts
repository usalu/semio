/**
 * 🔭️ M5b — drives the REAL `semio-os-mcp` stdio server with `.mcp.json`'s `semio` argv (only the
 * executable swapped for `--bin`) and captures what an agent sees: three `capabilities_search`
 * queries with score spread, the exclusion of raw input events, two `capabilities_describe` input
 * schemas, the destructive/approval pair of a replace-document verb, and the `resources/list` roster.
 *
 * Usage: `bun <this file> --label before|after --bin <path-to-semio-os-mcp>`
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
const bin = resolve(repoRoot, flag("bin", ".tmp-ticket-0918/wp-m5b/target/debug/semio-os-mcp"));

type Response = { id: number | string | null; result?: unknown; error?: { code: number; message: string; data?: unknown } };
type Hit = Record<string, unknown>;

const QUERIES = ["draw rectangle", "add layer", "export pdf"] as const;
const INPUT_QUERIES = ["canvas pointer move", "escape the canvas", "pointer down"] as const;
const DESCRIBE = ["draw.s.draw.drawing@1/*#editor.addLayer", "layout.s.layout.layout@1/*#editor.addPage", "draw.s.draw.drawing@1/*#editor.canvasPointerMove", "draw.s.draw.drawing@1/*#editor.canvasEscape"] as const;
const DESTRUCTIVE = ["draw.s.draw.drawing@1/*#editor.setSnapshot", "draw.s.draw.drawing@1/*#editor.deleteLayer", "layout.s.layout.layout@1/*#editor.exportPdf", "note.s.note.note@1/*#editor.deleteSelection"] as const;

function mcpArgv(): string[] {
  const config = JSON.parse(readFileSync(resolve(repoRoot, ".mcp.json"), "utf8")) as { mcpServers: Record<string, { args: string[] }> };
  const entry = config.mcpServers.semio;
  if (!entry) throw new Error(".mcp.json has no `semio` server entry");
  const index = entry.args.indexOf("os");
  return ["stdio", ...entry.args.slice(index + 1)];
}

async function main(): Promise<void> {
  const argv = mcpArgv();
  const say = (text: string): void => console.log(text);
  say(`# m5b capability-catalog capture — ${label}`);
  say(`# spawn: ${bin} ${argv.join(" ")}`);
  say(`# utc:   ${new Date().toISOString()}`);
  const child = spawn(bin, argv, { cwd: repoRoot, stdio: ["pipe", "pipe", "pipe"] });
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
      if (line.startsWith("{")) {
        const message = JSON.parse(line) as Response;
        const settle = typeof message.id === "number" ? pending.get(message.id) : undefined;
        if (settle) {
          pending.delete(message.id as number);
          settle(message);
        }
      }
      index = buffer.indexOf("\n");
    }
  });
  child.stderr.setEncoding("utf8");
  child.stderr.on("data", (chunk: string) => stderr.push(chunk));
  let nextId = 1;
  const call = (method: string, params?: unknown): Promise<Response> => {
    const id = nextId++;
    const promise = new Promise<Response>((settle, reject) => {
      pending.set(id, settle);
      setTimeout(() => reject(new Error(`timeout waiting for ${method}`)), 180_000);
    });
    child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
    return promise;
  };
  const tool = async (name: string, argumentsValue: unknown): Promise<Record<string, unknown> | undefined> => {
    const response = await call("tools/call", { name, arguments: argumentsValue });
    const result = response.result as { structuredContent?: Record<string, unknown>; isError?: boolean; content?: Array<{ text?: string }> } | undefined;
    if (!result?.structuredContent) say(`  (${name} error) ${JSON.stringify(response.error ?? result?.content?.[0]?.text ?? result).slice(0, 400)}`);
    return result?.structuredContent;
  };

  const initialize = await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m5b-probe", version: "0.0.0" } });
  const serverInfo = (initialize.result as { serverInfo?: { name?: string; version?: string } } | undefined)?.serverInfo;
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
  say(`\n## initialize — ${serverInfo?.name ?? "?"}@${serverInfo?.version ?? "?"}`);
  const context = await tool("context_resolve", { principal: "agent:local", locale: "en" });
  say(`## context_resolve — catalogHash=${String(context?.catalogHash ?? "?")} capabilities=${String(context?.capabilityCount ?? "?")}`);

  const verdicts: string[] = [];
  for (const query of QUERIES) {
    const structured = await tool("capabilities_search", { query, limit: 10 });
    const results = (structured?.results as Hit[] | undefined) ?? [];
    say(`\n## capabilities_search ${JSON.stringify(query)} — ${results.length} shown, total=${String(structured?.total ?? "n/a")}`);
    say(`| # | score | capabilityId | kind | title | description |`);
    say(`|---|---|---|---|---|---|`);
    results.forEach((hit, position) => {
      const score = typeof hit.score === "number" ? hit.score.toFixed(4) : "?";
      say(`| ${position + 1} | ${score} | ${String(hit.capabilityId)} | ${String(hit.kind ?? "")} | ${String(hit.title ?? "")} | ${String(hit.description ?? "").slice(0, 90)} |`);
    });
    const top = results.slice(0, 5).map((hit) => (typeof hit.score === "number" ? hit.score.toFixed(4) : "?"));
    const tiedTop = results.length > 1 && results[0]?.score === results[1]?.score;
    const empty = results.filter((hit) => String(hit.description ?? "").length === 0).length;
    say(`> top-5 distinct scores: ${new Set(top).size}/${top.length} · top hit tied: ${tiedTop} · empty descriptions: ${empty}/${results.length}`);
    verdicts.push(`${query}: top=${String(results[0]?.capabilityId ?? "-")} tied=${tiedTop} distinctTop5=${new Set(top).size}/${top.length}`);
  }

  for (const query of INPUT_QUERIES) {
    const structured = await tool("capabilities_search", { query, limit: 100 });
    const results = (structured?.results as Hit[] | undefined) ?? [];
    const leaked = results.filter((hit) => /canvasPointer|canvasEscape|canvasDoubleClick|engagementInput|PointerDown|PointerMove|PointerUp/.test(String(hit.capabilityId)));
    say(`\n## input-event exclusion ${JSON.stringify(query)} — ${results.length} hits, ${leaked.length} raw input event(s): ${leaked.map((hit) => String(hit.capabilityId)).join(", ")}`);
    verdicts.push(`exclusion ${query}: leaked=${leaked.length}`);
  }

  for (const capabilityId of DESCRIBE) {
    const structured = await tool("capabilities_describe", { capabilityId });
    say(`\n## capabilities_describe ${JSON.stringify(capabilityId)}`);
    if (!structured) continue;
    say(`title=${JSON.stringify(structured.title)} kind=${JSON.stringify(structured.kind)} audience=${JSON.stringify(structured.audience ?? null)}`);
    say(`description=${JSON.stringify(structured.description)}`);
    say(`inputSchema=${JSON.stringify(structured.inputSchema)}`);
  }

  for (const capabilityId of DESTRUCTIVE) {
    const structured = await tool("capabilities_describe", { capabilityId });
    const effects = structured?.effects as { destructive?: boolean } | undefined;
    const policy = structured?.policy as { approval?: string } | undefined;
    say(`\n## destructive ${JSON.stringify(capabilityId)} — destructive=${String(effects?.destructive)} approval=${String(policy?.approval)}`);
    verdicts.push(`destructive ${capabilityId}: ${String(effects?.destructive)}/${String(policy?.approval)}`);
  }

  const listed = await call("resources/list", {});
  const uris = ((listed.result as { resources?: Array<{ uri: string }> } | undefined)?.resources ?? []).map((resource) => resource.uri);
  const artifactsCount = uris.filter((uri) => uri === "semio://workspace/artifacts").length;
  say(`\n## resources/list — ${uris.length} resources, semio://workspace/artifacts ×${artifactsCount}, duplicates=${uris.length - new Set(uris).size}`);
  verdicts.push(`resources/list workspace/artifacts x${artifactsCount}`);

  say(`\n## verdicts`);
  for (const verdict of verdicts) say(`- ${verdict}`);
  child.stdin.end();
  child.kill();
  const skips = stderr.join("").split("\n").filter((line) => line.includes("skipping plugin")).length;
  say(`\n## registry diagnostics — ${skips} skip line(s)`);
}

await main();
