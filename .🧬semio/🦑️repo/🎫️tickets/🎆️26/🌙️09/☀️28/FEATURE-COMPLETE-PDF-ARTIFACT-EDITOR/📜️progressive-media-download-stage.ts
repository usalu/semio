/** 🌊 Stages direct bounded receipt and Blob-backed shell delivery. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
const paths = {"decl":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/📤️file/🟦️.ts","life":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/🚚️lifecycle/🟦️.ts","test":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/🟦️.ts","sink":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📤️SegmentedDownload/🟦️.ts","shell":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts","component":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts"};
const root = join(dirname(import.meta.path),"🗑️generated","progressive-media-download");
const overlay = join(root,"overlay.json");
const hash = (value: string) => createHash("sha256").update(value).digest("hex");
const replace = (s:string,a:string,b:string):string => {
  assert.equal(s.split(a).length,2,a);
  return s.replace(a,b);
};
const options = new Set(process.argv.slice(2));
if (options.has("--apply")) {
  const rows=JSON.parse(readFileSync(overlay,"utf8")) as readonly {path:string;before:string;after:string}[];
  for(const row of rows) assert.equal(hash(readFileSync(row.path,"utf8")),row.before,row.path);
  for(const row of rows) {
    assert.equal(hash(readFileSync(row.path,"utf8")),row.before,row.path);
    writeFileSync(row.path,row.after);
    assert.equal(readFileSync(row.path,"utf8"),row.after,row.path);
  }
  console.log("[DEBUG] Applied " + rows.length + " progressive download paths");
} else {
  const rows: {path:string;before:string;after:string}[] = [];
  const update=(path:string,edit:(content:string)=>string) => {
    assert.ok(resolve(path).length <= 256,path);
    const source=readFileSync(path,"utf8"), after=edit(source);
    assert.notEqual(source,after,path);
    rows.push({path,before:hash(source),after});
  };
  const cases=dirname(paths.test);
  update(join(cases,"🧬️schema","📜️declarations.json"),source=> {
    const schema=JSON.parse(source);
    schema.properties.deliveryCases.items.properties.transport = {type:"array",items:{enum:["open","take","write","close"]}};
    return JSON.stringify(schema,null,2)+"\n";
  });
  update(join(cases,"📜️declarations.json"),source=> {
    const fixture=JSON.parse(source);
    fixture.deliveryCases.find((row:{id:string})=>row.id==="complete").transport=["open","take","write","take","write","take","close"];
    return JSON.stringify(fixture,null,2)+"\n";
  });
  update(paths.test,source=> {
    let next=replace(source,'    const abort = new AbortController(), sink: string[] = [], names:','    const abort = new AbortController(), sink: string[] = [], transport: string[] = [], names:');
    next=replace(next,'takeMediaExportChunk: async () => ({ handle, data: Uint8Array.from(fixture.chunks[pages++] ?? []), terminal: pages > fixture.chunks.length }),','takeMediaExportChunk: async () => { transport.push("take"); return { handle, data: Uint8Array.from(fixture.chunks[pages++] ?? []), terminal: pages > fixture.chunks.length }; },');
    for(const event of ["open","write","close"]) next=replace(next,'sink.push("'+event+'");','sink.push("'+event+'"); transport.push("'+event+'");');
    return replace(next,"    expect(sink).toEqual(row.sink);","    expect(sink).toEqual(row.sink);\n    if (row.transport !== undefined) expect(transport).toEqual(row.transport);");
  });
  update(paths.component,source=> {
    let next=replace(source,'const delivered: { filename: string; mimeType: string; text: string }[] = [];','const delivered: { filename: string; mimeType: string; blob: Blob }[] = [];');
    next=replace(next,'(filename, mimeType, assembled) => delivered.push({ filename, mimeType, text: new TextDecoder().decode(assembled) })','(filename, blob) => delivered.push({ filename, mimeType: blob.type, blob })');
    return replace(next,'expect(delivered).toEqual([{ filename: "nakagin-capsule-tower.json", mimeType: "application/json", text: payload }]);','expect(delivered).toEqual([{ filename: "nakagin-capsule-tower.json", mimeType: "application/json", blob: expect.any(Blob) }]);\n    const { blob } = delivered[0]!;\n    expect(blob.type).toBe("application/json");\n    const text = await new Promise<string>((resolve, reject) => {\n      const reader = new FileReader();\n      reader.onload = () => resolve(String(reader.result));\n      reader.onerror = () => reject(reader.error);\n      reader.readAsText(blob);\n    });\n    expect(text).toBe(payload);');
  });
  update(paths.life,source=> {
    let next=replace(source,'async function collectExport(operation: MediaExportOperation, generation?: bigint): Promise<MediaExportBytes | null> {','export type MediaExportReceiverV1 = Readonly<{\n  begin: (descriptor: readonly number[]) => Promise<void> | void;\n  page: (chunk: Uint8Array, completed: number, total: number) => Promise<void> | void;\n}>;\n\nasync function receiveExport(operation: MediaExportOperation, receiver: MediaExportReceiverV1, generation?: bigint): Promise<MediaExportStatus | null> {');
    next=replace(next,'  const chunks: Uint8Array[] = [];\n  let bytes = 0n;','  const total = Number(status.total_bytes);\n  let deliveryRefusal: { reason: unknown } | undefined;\n  let bytes = 0n;');
    next=replace(next,'  let drained = false;\n  try {','  let drained = false;\n  try {\n    if (!operation.cancelled() && refusal === null) {\n      try { await receiver.begin(status.descriptor); }\n      catch (reason) { deliveryRefusal = { reason }; }\n    }');
    next=replace(next,'      if (!operation.cancelled() && refusal === null && chunk.data.byteLength > 0) chunks.push(chunk.data);','      if (chunk.terminal) {\n        drained = true;\n        if (bytes !== status.total_bytes) refusal ??= new Error("media-export.length");\n      }\n      if (bytes > status.total_bytes) refusal ??= new Error("media-export.length");\n      if (!operation.cancelled() && refusal === null && deliveryRefusal === undefined && chunk.data.byteLength > 0) {\n        try { await receiver.page(chunk.data, Number(bytes), total); }\n        catch (reason) { deliveryRefusal = { reason }; }\n      }');
    next=replace(next,'        drained = true;\n        if (bytes !== status.total_bytes) refusal ??= new Error("media-export.length");\n        if (operation.cancelled()) return null;\n        if (refusal) throw refusal;\n        return { chunks, mediaType: status.mime_type, descriptor: status.descriptor };','        if (operation.cancelled()) return null;\n        if (refusal) throw refusal;\n        if (deliveryRefusal !== undefined) throw deliveryRefusal.reason;\n        return status;');
    return replace(next,'/** 📤️ Collects a declared output', 'async function collectExport(operation: MediaExportOperation, generation?: bigint): Promise<MediaExportBytes | null> {\n  const chunks: Uint8Array[] = [];\n  const status = await receiveExport(operation, { begin: () => {}, page: chunk => { chunks.push(chunk); } }, generation);\n  return status === null ? null : { chunks, mediaType: status.mime_type, descriptor: status.descriptor };\n}\n\n/** 🚚️ Delivers one authenticated page per awaited turn and drains to terminal after a refusal. */\nexport async function receiveMediaExportBytes(operation: MediaExportOperation, receiver: MediaExportReceiverV1): Promise<boolean> {\n  return await receiveExport(operation, receiver) !== null;\n}\n\n/** 📤️ Collects a declared output');
  });
  update(paths.decl,source=> {
    let next=replace(source,"{ collectMediaExportBytes, type MediaTransportPort }","{ receiveMediaExportBytes, type MediaTransportPort }");
    next=replace(next,"{ SegmentedDownloadSinkFactory }","{ SegmentedDownloadSink, SegmentedDownloadSinkFactory }");
    return replace(next,next.trimEnd().slice(next.indexOf("  const exported = await collectMediaExportBytes({")),"  const delivery: { sink?: SegmentedDownloadSink } = {};\n  const abort = async (reason: unknown): Promise<void> => {\n    const sink = delivery.sink;\n    delete delivery.sink;\n    await sink?.abort(reason);\n  };\n  try {\n    const exported = await receiveMediaExportBytes({\n      instanceId, parentDocumentId: identity.parent_document_id, revision: identity.base_revision,\n      outputPort: declaration.outputPort, mediaType: declaration.mediaType, port, cancelled,\n      progress: status => operation.progress(Number(status.applied_progress > BigInt(Number.MAX_SAFE_INTEGER) ? BigInt(Number.MAX_SAFE_INTEGER) : status.applied_progress), null),\n      ...(operation.yieldTurn === undefined ? {} : { yieldTurn: operation.yieldTurn }),\n    }, {\n      begin: async descriptor => {\n        if (!mediaFileDescriptorMatchesV1(decodePackValue(Uint8Array.from(descriptor)), declaration)) throw new Error(\"media-export.descriptor-mismatch\");\n        const latest = await port.readAppDocumentIdentity(instanceId);\n        if (cancelled()) return;\n        if (!sourceCurrent(latest)) throw new Error(\"media-export.source-changed\");\n        delivery.sink = await operation.sinkFactory(declaration.filename, declaration.mediaType);\n      },\n      page: async (chunk, completed, total) => {\n        if (delivery.sink === undefined) throw new Error(\"media-export.sink-unavailable\");\n        await delivery.sink.write(chunk);\n        operation.progress(completed, total);\n      },\n    });\n    if (!exported || cancelled()) { await abort(\"media-export.cancelled\"); return false; }\n    if (delivery.sink === undefined) throw new Error(\"media-export.sink-unavailable\");\n    await operation.beforeDelivery?.();\n    if (cancelled()) { await abort(\"media-export.cancelled\"); return false; }\n    const beforeDelivery = await port.readAppDocumentIdentity(instanceId);\n    if (cancelled()) { await abort(\"media-export.cancelled\"); return false; }\n    if (!sourceCurrent(beforeDelivery)) throw new Error(\"media-export.source-changed\");\n    await delivery.sink.close();\n    return true;\n  } catch (error) {\n    await abort(error);\n    throw error;\n  }\n}");
  });
  update(paths.sink,source=> {
    const start=source.indexOf("/** 🧺 Assembles the drained"),end=source.indexOf("/** 🧵 Drains exactly",start);
    assert.ok(start>=0&&end>start);
    return source.slice(0,start)+"/** 🧺 Keeps immutable Blob pages until the anchor receives their ordered representation. */\nexport function createBufferedDownloadSink(filename: string, mimeType: string, deliver: (filename: string, blob: Blob) => void): SegmentedDownloadSink {\n  let blocks: Blob[] | undefined = [];\n  return {\n    write: async chunk => {\n      if (blocks === undefined) throw new Error(\"segmented-download-sink-retired\");\n      const bytes = chunk.buffer instanceof ArrayBuffer ? new Uint8Array(chunk.buffer, chunk.byteOffset, chunk.byteLength) : chunk.slice();\n      blocks.push(new Blob([bytes]));\n    },\n    close: async () => {\n      if (blocks === undefined) throw new Error(\"segmented-download-sink-retired\");\n      const contents = new Blob(blocks, { type: mimeType });\n      blocks = undefined;\n      deliver(filename, contents);\n    },\n    abort: async () => {\n      blocks = undefined;\n    },\n  };\n}\n\n/** 🌊 Supplies an ordered Blob download without a browser-specific file picker. */\nexport function segmentedDownloadSinkFactory(deliver: (filename: string, blob: Blob) => void): SegmentedDownloadSinkFactory {\n  return async (filename, mimeType) => createBufferedDownloadSink(filename, mimeType, deliver);\n}\n\n"+source.slice(end);
  });
  update(paths.shell,source=> {
    let next=replace(source,'/** 📥️ Host delivery of already-assembled bytes — the blob-and-anchor half of {@link downloadMediaExport}, reached by the segmented lane once its chunks are drained. */','/** 📥️ Delivers one native byte array as an anchor download. */');
    next=replace(next,'  if (typeof document === "undefined") return;\n  const blob = new Blob([bytes.slice()], { type: mimeType });\n  const url = URL.createObjectURL(blob);','  if (typeof document === "undefined") return;\n  downloadMediaExportBlob(filename, new Blob([bytes.slice()], { type: mimeType }));\n}\n\n/** 🧾️ Keeps the anchored Blob alive through a complete browser download handoff. */\nfunction downloadMediaExportBlob(filename: string, blob: Blob): void {\n  if (typeof document === "undefined") return;\n  const url = URL.createObjectURL(blob);');
    next=replace(next,'/** 🌊 The sink factory every shell drain uses: assembled chunks delivered as one file through {@link downloadMediaExportBytes}. */','/** 🌊 The default shell sink delivers the verified Blob of its ordered pages. */');
    return replace(next,'segmentedDownloadSinkFactory(downloadMediaExportBytes)','segmentedDownloadSinkFactory(downloadMediaExportBlob)');
  });
  mkdirSync(root,{recursive:true});
  writeFileSync(overlay,JSON.stringify(rows,null,2)+"\n");
  console.log("[DEBUG] Staged " + rows.length + " progressive download paths");
}
