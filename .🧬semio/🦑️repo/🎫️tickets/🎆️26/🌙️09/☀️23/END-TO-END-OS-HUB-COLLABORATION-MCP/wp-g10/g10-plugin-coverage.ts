#!/usr/bin/env bun
/** 🧮️ G10 S3: every installed plugin package over the semio MCP, measured per package on a fresh `--folder` space with the
 * staged binary. Per package: capabilities (paginated `capabilities_search` by owner), mutations, empty descriptions, verbs
 * whose declared args are none, declared inferences (`inference_list`), and for every installed artifact kind of the package
 * `artifact_create` + one non-destructive, argument-free-or-defaulted mutation through `action_prepare`/`action_invoke`.
 * usage: bun g10-plugin-coverage.ts <outDir> [pluginId…] */
import { appendFileSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const mod = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts");
const outDir = process.argv[2]!;
const only = new Set(process.argv.slice(3));
mkdirSync(outDir, { recursive: true });
const rowsPath = join(outDir, "coverage-rows.jsonl");
writeFileSync(rowsPath, "");
const log = (line: string) => {
  console.log(`${new Date().toISOString()} ${line}`);
};
const CALL_MS = 240_000;
let proc = mod.spawnRawMcp(mod.requireMcpBinary("/Users/ueli/Documents/semio"), ["stdio", "--folder", mkdtempSync(join(tmpdir(), "g10-coverage-")), "--scopes", "workspace.read,artifact.write,inference.execute", "--no-bridge", "--auto-approve", "all"]);
const open = async () => {
  proc.writeRaw(JSON.stringify({ jsonrpc: "2.0", id: 9001, method: "server/discover", params: {} }));
  await proc.nextLine(120_000);
};
await open();
const call = async (name: string, args: Record<string, unknown>, budgetMs = CALL_MS): Promise<{ ok: boolean; value: any; ms: number }> => {
  const started = Date.now();
  try {
    const response: any = await proc.request("tools/call", { name, arguments: args }, budgetMs);
    const result = response.result;
    return { ok: !response.error && result?.isError !== true, value: response.error ?? result?.structuredContent ?? result, ms: Date.now() - started };
  } catch (error) {
    log(`call ${name} threw ${String(error).slice(0, 200)}; respawning the gateway`);
    await proc.close().catch(() => undefined);
    proc = mod.spawnRawMcp(mod.requireMcpBinary("/Users/ueli/Documents/semio"), ["stdio", "--folder", mkdtempSync(join(tmpdir(), "g10-coverage-")), "--scopes", "workspace.read,artifact.write,inference.execute", "--no-bridge", "--auto-approve", "all"]);
    await open();
    return { ok: false, value: { code: "CLIENT_TIMEOUT", message: String(error).slice(0, 300) }, ms: Date.now() - started };
  }
};

const probe = await call("artifact_create", { artifactId: "coverage-probe-kinds", kind: "no.such.kind" });
const installedKinds: string[] = probe.value?.details?.installedKinds ?? [];
const inferences = await call("inference_list", {});
const declaredInferences: any[] = inferences.value?.declared ?? inferences.value?.services ?? inferences.value?.inferences ?? [];
log(`installed kinds=${installedKinds.length} declared inferences=${declaredInferences.length} (keys ${Object.keys(inferences.value ?? {}).join(",")})`);

const pluginIds = [...new Set(installedKinds.filter((kind) => kind.startsWith("s.")).map((kind) => kind.split(".")[1]!))].filter((id) => only.size === 0 || only.has(id)).sort();
const table: string[] = [];
for (const pluginId of pluginIds) {
  const capabilities: any[] = [];
  let cursor: string | undefined;
  do {
    const page = await call("capabilities_search", { query: pluginId, owner: pluginId, limit: 100, ...(cursor ? { cursor } : {}) });
    capabilities.push(...(page.value?.results ?? []));
    cursor = page.value?.nextCursor ?? undefined;
  } while (cursor);
  const described: any[] = [];
  for (const capability of capabilities) {
    const answer = await call("capabilities_describe", { capabilityId: capability.capabilityId });
    described.push(answer.value?.capability ?? answer.value);
  }
  const mutations = described.filter((capability) => capability?.kind === "mutation");
  const emptyDescriptions = described.filter((capability) => !String(capability?.description ?? "").trim()).length;
  const noArgs = mutations.filter((capability) => (capability?.presentation?.args ?? []).length === 0).length;
  const destructive = mutations.filter((capability) => capability?.effects?.destructive === true).length;
  const pluginInferences = declaredInferences.filter((row) => JSON.stringify(row).includes(`"${pluginId}"`) || String(row?.artifactKind ?? row?.kind ?? "").startsWith(`s.${pluginId}.`)).length;
  const kinds = installedKinds.filter((kind) => kind.startsWith(`s.${pluginId}.`) || kind === `s.${pluginId}`);
  const kindRows: string[] = [];
  let created = 0;
  let invoked = 0;
  for (const kind of kinds) {
    const artifactId = `coverage-${kind.replaceAll(".", "-")}`;
    const create = await call("artifact_create", { artifactId, kind }, 900_000);
    const row: Record<string, unknown> = { pluginId, kind, create: create.ok ? "ok" : String(create.value?.code ?? "error"), createMs: create.ms, createDetail: create.ok ? `${create.value?.sizeBytes ?? "?"} B` : String(create.value?.message ?? "").slice(0, 220) };
    if (create.ok) {
      created += 1;
      const candidates = mutations
        .filter((capability) => capability?.artifactKind === kind)
        .sort((left, right) => Number(left?.effects?.destructive === true) - Number(right?.effects?.destructive === true) || Number((right?.presentation?.args ?? []).length > 0) - Number((left?.presentation?.args ?? []).length > 0));
      const attempts: string[] = [];
      for (const capability of candidates.slice(0, 8)) {
        const schema = capability.inputSchema ?? capability.input_schema ?? {};
        const input: Record<string, unknown> = {};
        let feasible = true;
        const byType: Record<string, unknown> = { string: "G10", number: 1, integer: 1, boolean: false, array: [], object: {} };
        for (const name of schema.required ?? []) {
          const property = schema.properties?.[name] ?? {};
          const type = Array.isArray(property.type) ? property.type[0] : property.type;
          if (property.default !== undefined) input[name] = property.default;
          else if (Array.isArray(property.enum) && property.enum.length > 0) input[name] = property.enum[0];
          else if (type in byType) input[name] = byType[type];
          else feasible = false;
        }
        if (!feasible) {
          attempts.push(`${capability.id}: needs required args`);
          continue;
        }
        const prepared = await call("action_prepare", { capabilityId: capability.id, input });
        if (!prepared.ok) {
          attempts.push(`${capability.id}: prepare ${prepared.value?.code} ${String(prepared.value?.message ?? "").slice(0, 120)}`);
          continue;
        }
        const done = await call("action_invoke", { preparedActionHandle: prepared.value.preparedHandle });
        if (done.ok && done.value?.status === "SUCCEEDED") {
          invoked += 1;
          row.invoke = "ok";
          row.invokeVerb = capability.id;
          row.invokeMs = done.ms;
          break;
        }
        attempts.push(`${capability.id}: invoke ${done.value?.status ?? done.value?.code} ${String(done.value?.message ?? "").slice(0, 120)}`);
      }
      if (row.invoke !== "ok") {
        row.invoke = candidates.length === 0 ? "no-candidate" : "failed";
        row.invokeAttempts = attempts;
      }
    }
    appendFileSync(rowsPath, `${JSON.stringify(row)}\n`);
    kindRows.push(`${kind}:${row.create}/${row.invoke ?? "-"}`);
    log(`${pluginId} ${kind} create=${row.create} (${create.ms} ms) invoke=${row.invoke ?? "-"} ${row.invokeVerb ?? ""}`);
  }
  const summary = { pluginId, capabilities: described.length, mutations: mutations.length, destructive, emptyDescriptions, mutationsWithoutArgs: noArgs, inferences: pluginInferences, kinds: kinds.length, created, invoked };
  appendFileSync(rowsPath, `${JSON.stringify({ summary })}\n`);
  table.push(`| ${pluginId} | ${described.length} | ${mutations.length} | ${destructive} | ${emptyDescriptions} | ${noArgs} | ${pluginInferences} | ${created}/${kinds.length} | ${invoked}/${created} |`);
  log(`SUMMARY ${JSON.stringify(summary)}`);
}
const header = "| package | capabilities | mutations | destructive | empty description | mutations declaring no args | inferences | artifact_create ok/kinds | action_invoke ok/created |\n|---|---|---|---|---|---|---|---|---|";
writeFileSync(join(outDir, "coverage-table.md"), `${header}\n${table.join("\n")}\n`);
console.log(`${header}\n${table.join("\n")}`);
await proc.close();
