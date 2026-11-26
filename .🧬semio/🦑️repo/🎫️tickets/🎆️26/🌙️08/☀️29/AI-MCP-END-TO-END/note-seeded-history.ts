import assert from "node:assert/strict";
import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import type { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { CallToolResultSchema } from "@modelcontextprotocol/sdk/types.js";
import { createHeadlessCallerSurfaceV1, exportedDocumentMatchesSnapshotV1, minimalInputForSchema } from "@semio-tech/framework-os-mcp";
import { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 as pins } from "../../../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts";
import { verifyNoteJourney } from "./note-mcp-replay-verify.ts";

const artifactId = "note-seeded-sdk";

export async function seedNote(generated: string, folder: string) {
  const pair = (await readFile(join(generated,"note-wasi-seeded-pair.tsv"),"utf8")).trim().split("\t");
  assert.equal(pair.length,2);
  const paths = ["PACK","SPR"].map(key => join(folder,key));
  for (const [i,path] of paths.entries()) {
    const bytes = Buffer.from(pair[i],"base64");
    assert(bytes.length && bytes.toString("base64") === pair[i]);
    await writeFile(path,bytes);
  }
  const binary = join(generated,process.platform === "win32" ? "wfc-seeded-folder.exe" : "wfc-seeded-folder");
  assert.equal(Bun.spawnSync([binary,"seed",folder,artifactId,...paths,pins.artifactKind],{stderr:"inherit",stdout:"inherit"}).exitCode,0);
}

export async function verifySeededNote(client: Client, model: string, generated: string, readOnly=false, exportArchive=true) {
  const call = async (name: string,args: Record<string,unknown>) => {
    const reply = await client.callTool({name,arguments:args},CallToolResultSchema,{timeout:300_000});
    console.log("[DEBUG]",name,JSON.stringify(reply.structuredContent).slice(0,1500));
    assert(!reply.isError,JSON.stringify(reply.structuredContent));
    assert(reply.structuredContent);
    return reply.structuredContent;
  };
  const snapshot = async () => {
    const reply = await call("artifact_snapshot",{artifactId});
    const envelope = reply.untrusted as {schema:string;content:{packBase64:string;sprBase64:string}};
    assert.equal(envelope.schema,"semio.mcp.untrusted-content/v1");
    return envelope.content;
  };
  const opened = await call("artifact_open",{artifactId});
  assert.equal(opened.artifactKind,pins.artifactKind);
  const baseline = await snapshot();
  const expected = (await readFile(join(generated,"note-wasi-seeded-pair.tsv"),"utf8")).trim().split("\t");
  assert.deepEqual([baseline.packBase64,baseline.sprBase64],expected);
  if (readOnly) {
    await client.ping();
    console.log("[DEBUG] Verified Note folder identity, Pack, SPR and ping");
    if (!exportArchive) return;
    const exported=await call("artifact_export",{artifactId,format:"artifact:out"});
    assert.equal(exported.artifactId,artifactId);
    const media=exported.untrusted as {content:{contentBase64:string}};
    assert(exportedDocumentMatchesSnapshotV1(media.content.contentBase64,expected.join(".")));
    await client.ping();
    console.log("[DEBUG] Verified Note folder identity, Pack, SPR, archive and ping");
    return;
  }
  const context = await call("context_resolve",{locale:"en"});
  const described = await call("capabilities_describe",{capabilityId:pins.capabilityId});
  const input = minimalInputForSchema(described.inputSchema ?? (described.capability as any)?.inputSchema);
  const descriptor = JSON.parse(await readFile(join(model,"🔣️.json"),"utf8"));
  const app = descriptor.manifest.apps.find((row:{id:string})=>row.id===pins.caller.appId);
  const surface=createHeadlessCallerSurfaceV1(app,{sessionId:String(context.sessionId),principal:String(context.principal),artifactId,pluginId:pins.pluginId,windowId:"note-seeded-editor"},pins.caller);
  const prepare = () => call("action_prepare",{capabilityId:pins.capabilityId,input,target:surface.target,context:surface.context});
  const prepared = await prepare();
  const invoked = await call("action_invoke",{preparedActionHandle:prepared.preparedHandle});
  assert.equal(invoked.status,"SUCCEEDED");
  assert.equal(typeof invoked.undoToken,"string");
  const invoke = await snapshot();
  await call("history_undo",{undoToken:invoked.undoToken});
  const undo = await snapshot();
  await call("history_redo",{undoToken:invoked.undoToken});
  const redo = await snapshot();
  const member = await prepare();
  const began = await call("transaction_begin",{preparedHandles:[member.preparedHandle]});
  await call("transaction_rollback",{transactionHandle:began.transactionHandle});
  const rollback = await snapshot();
  const journey={artifactId,input,observations:{baseline,invoke,undo,redo,rollback}};
  await writeFile(join(generated,"note-seeded-sdk-history.json"),JSON.stringify(journey));
  verifyNoteJourney(join(generated,"note-wasi-run","_run.js"),journey);
}
