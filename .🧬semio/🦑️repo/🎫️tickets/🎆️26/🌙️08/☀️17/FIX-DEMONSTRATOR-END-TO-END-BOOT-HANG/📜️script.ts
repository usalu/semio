/** 🔎️ Ticket-local MCP diagnostics through the repository's configured stdio entry point. */

//#region 🔌️Client
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
//#endregion 🔌️Client

//#region 🧪️ComponentProbe
async function probePlugin(): Promise<void> {
  const pluginId = process.argv[3] ?? "demonstrator";
  const appId = process.argv[4] ?? "s.puzzle.puzzle3d@1/*#editor";
  const modulePath = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules", pluginId, `semio_s_plugin_${pluginId}.js`);
  const { createActorApi } = await import(pathToFileURL(modulePath).href);
  const { decodePackValue, decodeScenePackValue } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/💻️os/🟦️component.ts")).href);
  const appIds = appId === "all" ? ["s.puzzle.puzzle3d@1/*#editor", "s.cad.cad@1/*#editor", "s.sourcing.curate@1/*#editor", "s.process.process3d@1/*#editor", "s.gis.gismap@1/*#editor"] : [appId];
  for (const [index, id] of appIds.entries()) {
    const api = await createActorApi(`${pluginId}#diagnostic-${index}`);
    console.log(`[DEBUG] Component loaded; opening ${id}`);
    try {
      const result = await api.poll([{ kind: "instance-open", payload: { instance: 1, appId: id, actor: "local", config: [], assets: [], capabilities: [], quotas: [] } }], undefined, { fuel: 50_000_000, wallMs: 100, maxEffects: 64, maxPatchBytes: 1 << 20 });
      console.log(JSON.stringify(result, (_, value) => typeof value === "bigint" ? value.toString() : value));
      const surfaces = process.argv.slice(5).filter((argument) => argument !== "--close");
      let events = surfaces.map((surface) => ({ kind: "surface-visible", payload: { surface: { instance: 1, surface } } }));
      const published = new Set<string>();
      for (let turn = 0; surfaces.length && turn < 4096; turn++) {
        const outcome = await api.poll(events.length ? events : [{ kind: "wake" }], undefined, { fuel: 50_000_000, wallMs: 100, maxEffects: 64, maxPatchBytes: 1 << 20 });
        console.log(JSON.stringify({ turn, ...outcome }, (key, value) => typeof value === "bigint" ? value.toString() : value instanceof Uint8Array ? decodePackValue(value) : key === "doc" && Array.isArray(value?.bytes) ? { snapshot: decodeScenePackValue(Uint8Array.from(value.bytes)) } : value));
        if (outcome.status.tag === "faulted") throw new Error(`Surface turn faulted for ${id}`);
        events = outcome.uiPatches.map((patch) => {
          published.add(patch.surface.surface);
          return { kind: "patch-ack", payload: { surface: patch.surface, revision: patch.revision } };
        });
        if (outcome.status.tag === "idle" && !events.length) break;
      }
      if (surfaces.some((surface) => !published.has(surface))) throw new Error(`Missing surfaces: ${surfaces.filter((surface) => !published.has(surface)).join(", ")}`);
      if (surfaces.length) console.log(`[DEBUG] Published ${published.size} requested surfaces for ${id}`);
      if (process.argv.includes("--close")) {
        let closed = false;
        for (let turn = 0; turn < 4096; turn++) {
          const outcome = await api.poll(turn ? [{ kind: "wake" }] : [{ kind: "instance-close", payload: { instance: 1 } }], undefined, { fuel: 50_000_000, wallMs: 100, maxEffects: 64, maxPatchBytes: 1 << 20 });
          if (outcome.status.tag === "faulted") throw new Error(`Close fault: ${JSON.stringify(decodePackValue(outcome.status.val))}`);
          if (outcome.status.tag === "idle") { closed = true; console.log(`[DEBUG] Closed ${id} after ${turn + 1} turns`); break; }
          if (turn % 128 === 0) console.log(`[DEBUG] Closing ${id}: turn ${turn}`);
        }
        if (!closed) throw new Error(`Close did not become idle for ${id}`);
      }
    } catch (error) {
      console.error(error);
      process.exitCode = 1;
    }
  }
}
//#endregion 🧪️ComponentProbe

//#region 🔎️Inspect
async function inspectRepo(): Promise<void> {
const client = new Client({ name: "demonstrator-ticket", version: "1.0.0" }, { capabilities: {} });
const transport = new StdioClientTransport({
  command: "bun",
  args: ["./📜️script.ts", "dev", "mcp", "stdio", "codex"],
  cwd: root,
  env: { ...process.env, SEMIO_BUILD_BUDGET_MS: "60000" },
  stderr: "inherit",
});
try {
  await client.connect(transport);
  console.log(JSON.stringify({ kind: "goals", result: await client.readResource({ uri: "repo://goals" }) }));
  const listed = await client.listTools();
  console.log(JSON.stringify({ kind: "ticket-tools", tools: listed.tools.filter((tool) => /ticket_(open|reopen|close|read|list)/.test(tool.name)) }));
} catch (error) {
  console.error(String(error));
  process.exitCode = 1;
} finally {
  await client.close();
}
}

try {
  if (process.argv[2] === "probe-plugin") await probePlugin();
  else await inspectRepo();
} catch (error) {
  console.error(error);
  process.exitCode = 1;
}
//#endregion 🔎️Inspect
