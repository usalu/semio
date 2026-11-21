/** 💾️ Prepares the actual natural Save transfer's segmented, archive-checked delivery. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";

const ticket = dirname(import.meta.path);
const paths = {"file":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/📤️file/🟦️.ts","scope":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx","tests":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/🟦️.ts"};
const output = join(ticket, "🗑️generated", "natural-media-save");
const stage = join(output, "overlay.json");
const digest = (value: string) => createHash("sha256").update(value).digest("hex");
const replace = (text: string, before: string, after: string): string => {
  assert.equal(text.split(before).length, 2, before);
  return text.replace(before, after);
};
const options = new Set(process.argv.slice(2));
if (options.has("--apply")) {
  const changes = JSON.parse(readFileSync(stage,"utf8")) as readonly {path:string;before:string;after:string}[];
  const selected = options.has("--tests") ? changes.slice(0,6) : options.has("--runtime") ? changes.slice(6) : changes;
  for (const row of selected) assert.equal(digest(readFileSync(row.path,"utf8")),row.before,row.path);
  for (const row of selected) {
    assert.equal(digest(readFileSync(row.path,"utf8")),row.before,row.path);
    writeFileSync(row.path,row.after);
    assert.equal(readFileSync(row.path,"utf8"),row.after,row.path);
  }
  console.log("[DEBUG] Applied " + selected.length + " exact natural Save paths");
} else {
  const revisions: {path:string;before:string;after:string}[] = [];
  const update = (path: string, edit: (source: string) => string) => {
    assert.ok(resolve(path).length <= 256,path);
    const source = readFileSync(path,"utf8"), next = edit(source);
    assert.notEqual(source,next,path);
    revisions.push({path,before:digest(source),after:next});
  };
  const cases = dirname(paths.tests);
  const archiveCases = [
    {id: "archive-refused-before-delivery", delivered: false, error: "natural-file.saved-revision-changed", sink: ["open", "write", "write", "abort"]},
    {id: "archive-cancelled-before-delivery", delivered: false, error: null, sink: ["open", "write", "write", "abort"]},
  ];
  update(join(cases,"🧬️schema","📜️declarations.json"), source => {
    const schema = JSON.parse(source);
    schema.properties.deliveryCases.items.properties.id.enum.push(...archiveCases.map(row => row.id));
    return JSON.stringify(schema,null,2) + "\n";
  });
  update(join(cases,"📜️declarations.json"), source => {
    const fixture = JSON.parse(source);
    fixture.deliveryCases.push(...archiveCases);
    return JSON.stringify(fixture,null,2) + "\n";
  });
  update(paths.tests,source => replace(source,
    "progress: () => {}, yieldTurn: async () => {}, sinkFactory:",
    'progress: () => {}, yieldTurn: async () => {}, ...(row.id.startsWith("archive-") ? { beforeDelivery: async () => {\n        expect(pages).toBe(fixture.chunks.length + 1);\n        expect(sink).toEqual(["open", "write", "write"]);\n        if (row.id === "archive-cancelled-before-delivery") abort.abort();\n        else throw new Error("natural-file.saved-revision-changed");\n      } } : {}), sinkFactory:'));
  update(dirname("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/💾️natural-scope/🟦️.tsx") + "/🔣️.json", source => {
    const fixture = JSON.parse(source);
    fixture.segmentedSave = [
      {id: "current", chunks: [[37, 80], [68, 70]], delivered: true},
      {id: "edited", chunks: [[37, 80], [68, 70]], delivered: false},
    ];
    return JSON.stringify(fixture,null,2) + "\n";
  });
  update("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/💾️natural-scope/🟦️.tsx", source => {
    let result = replace(source, 'import { encodePackValue }', 'import { EXPORT_MEDIA_FILE_ACTION_ID } from "@semio-tech/framework";\nimport { encodePackValue }');
    result = replace(result,'downloads: [] as string[] }','downloads: [] as string[], segmented: [] as number[][], aborted: false }');
    result = replace(result,'downloadMediaExportBytes: (file: string) => { selection.downloads.push(file); },','downloadMediaExportBytes: (file: string) => { selection.downloads.push(file); }, shellSegmentedDownloadSinkFactory: async (file: string) => ({ write: async (bytes: Uint8Array) => { selection.segmented.push([...bytes]); }, close: async () => { selection.downloads.push(file); }, abort: async () => { selection.aborted = true; } }),');
    result = replace(result,'naturalFileFormatV1: () => ({ formatKind: "txt", extension: ".txt", mediaType: "text/plain", binary: false })','naturalFileFormatV1: (app: { actions: unknown[] }) => app.actions.length ? ({ formatKind: "s.stdio.pdf@1.7", extension: ".pdf", mediaType: "application/pdf", binary: true }) : ({ formatKind: "txt", extension: ".txt", mediaType: "text/plain", binary: false })');
    result = replace(result,'function setup() {','function setup(segmented = false) {');
    result = replace(result,'  const app = { id: document.surface.appId, io: { artifactSchema: document.schema }, windowKinds: [], modes: [], breadcrumb: [] };','  const media = { outputPort: "artifact:native", filename: "document.pdf", mediaType: "application/pdf", formatKind: "s.stdio.pdf@1.7" };\n  const actions = segmented ? [{ id: EXPORT_MEDIA_FILE_ACTION_ID, args: Object.entries(media).map(([id, value]) => ({ id, default: value })) }] : [];\n  const ports = segmented ? [{ id: media.outputPort, direction: "out", kindId: media.formatKind, mediaType: { class: "data", form: "document" } }] : [];\n  const app = { id: document.surface.appId, io: { artifactSchema: document.schema, ports }, actions, windowKinds: [], modes: [], breadcrumb: [] };');
    result = replace(result,'selection.downloads = [];','selection.downloads = []; selection.segmented = []; selection.aborted = false;');
    result = replace(result,'function setupSave(bound = true) {\n  const view = setup();','function setupSave(bound = true, segmented = false) {\n  const view = setup(segmented);');
    return result.trimEnd() + "\n\n" + "for (const outcome of fixture.segmentedSave) it(\"Save delivers staged native output \" + outcome.id, async () => {\n  const view = setupSave(true, true);\n  const portId = \"artifact:native\", mediaType = \"application/pdf\", formatKind = \"s.stdio.pdf@1.7\";\n  const identity = { app_instance_id: view.target.instanceId, parent_document_id: document.documentId, base_revision: 3n };\n  const handle = { ...identity, operation_id: 8n, generation: 2n };\n  const submitted = vi.fn(async () => handle);\n  const segments = outcome.chunks.map(bytes => Uint8Array.from(bytes));\n  Object.assign(view.handle, {\n    readAppDocumentIdentity: async () => identity,\n    submitMediaExport: submitted,\n    pollMediaExport: async () => ({ handle, state: \"complete\", applied_progress: 2n, checkpoint_available: false, mime_type: mediaType, total_bytes: BigInt(segments.reduce((total, bytes) => total + bytes.length, 0)), detail: \"\", descriptor: [...encodePackValue({ portId, kindId: formatKind, mediaType: { class: \"data\", form: \"document\" }, wire: { kind: \"binary\", format_kind: formatKind } })] }),\n    takeMediaExportChunk: async () => {\n      const data = segments.shift();\n      if (data !== undefined) return { handle, data, terminal: false };\n      if (outcome.id === \"edited\") view.handle.readAppDocumentArchive.mockResolvedValue({ ...archive, parent_pack: [9] });\n      return { handle, data: new Uint8Array(), terminal: true };\n    },\n    cancelMediaExport: vi.fn(),\n  });\n  const saving = view.state.saveNaturalFile();\n  await vi.waitFor(() => expect(view.observer()).toBeDefined());\n  expect(submitted).not.toHaveBeenCalled();\n  view.observer().archive(encodeDocumentArchiveBytes(archive));\n  await saving;\n  expect(submitted).toHaveBeenCalledExactlyOnceWith(view.target.instanceId, portId, document.documentId, 3n);\n  expect(view.handle.exportAppMedia).not.toHaveBeenCalled();\n  expect(selection.segmented).toEqual(outcome.chunks);\n  expect(selection.downloads).toHaveLength(outcome.delivered ? 1 : 0);\n  expect(selection.aborted).toBe(!outcome.delivered);\n  expect(view.calls.includes(\"shell.documentTransfer.file-saved\")).toBe(outcome.delivered);\n});\n";
  });
  update("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts", source => replace(source, '  engineSuite("📤️checked-media-export"),', '  engineSuite("📤️checked-media-export"),\n  engineSuite("💾️natural-scope", "tsx"),'));
  update(paths.file,source => {
    let result = replace(source,
      'import { EXPORT_MEDIA_FILE_ACTION_ID, EXPORT_SNAPSHOT_FILE_ACTION_ID, type AppDefinition, type MediaType }',
      'import { EXPORT_MEDIA_FILE_ACTION_ID, EXPORT_SNAPSHOT_FILE_ACTION_ID, NATURAL_FILE_PORT_ID, type AppDefinition, type MediaType }');
    result = replace(result, "  readonly yieldTurn?: () => Promise<void>;", "  readonly yieldTurn?: () => Promise<void>;\n  readonly beforeDelivery?: () => Promise<void>;");
    result = replace(result,"  const actions = [...app.actions, ...app.windowKinds.flatMap(kind => kind.actions)]","  const actions = [...(app.actions ?? []), ...app.windowKinds.flatMap(kind => kind.actions ?? [])]");
    result = replace(result, "/** 🧾️ Checks the completed binary representation",
"/** 🎛️ Resolves a producer for the exact native binary format selected by Save. */\nexport function declaredNaturalMediaFileExportV1(app: AppDefinition, format: Readonly<{formatKind: string; mediaType: string; binary: boolean}>, filename: string): MediaFileExportV1 | null {\n  if (!format.binary) return null;\n  const declaration = declaredMediaFileExportV1(app, EXPORT_MEDIA_FILE_ACTION_ID);\n  return declaration?.outputPort === NATURAL_FILE_PORT_ID && declaration.formatKind === format.formatKind && declaration.mediaType === format.mediaType\n    ? {...declaration, filename}\n    : null;\n}\n\n/** 🧾️ Checks the completed binary representation");
    return replace(result, "    const beforeDelivery = await port.readAppDocumentIdentity(instanceId);", "    if (cancelled()) { await sink.abort(\"media-export.cancelled\"); return false; }\n    await operation.beforeDelivery?.();\n    if (cancelled()) { await sink.abort(\"media-export.cancelled\"); return false; }\n    const beforeDelivery = await port.readAppDocumentIdentity(instanceId);");
  });
  update(paths.scope, source => {
    let result = replace(source,"import { parseSurfaceAppId, type AppDefinition","import { NATURAL_FILE_PORT_ID, parseSurfaceAppId, type AppDefinition");
    result = replace(result,"import { declaredMediaFileExportV1, downloadDeclaredMediaFileV1, type MediaFileExportActionIdV1 }","import { declaredMediaFileExportV1, declaredNaturalMediaFileExportV1, downloadDeclaredMediaFileV1, type MediaFileExportActionIdV1 }");
    result = replace(result,
      '    const file = format === null ? "" : naturalFileNameV1(owned.target.app.id, format.extension, new Date());',
      '    const file = format === null ? "" : naturalFileNameV1(owned.target.app.id, format.extension, new Date());\n    const declaration = format === null ? null : declaredNaturalMediaFileExportV1(owned.target.app, format, file);');
    result = replace(result,
      '      if (format === null || exportMedia === undefined) throw new Error("natural-file.exporter-unavailable");',
      '      if (format === null || (declaration === null && exportMedia === undefined)) throw new Error("natural-file.exporter-unavailable");');
    return replace(result, "      const artifact = await exportMedia(owned.target.instanceId, \"artifact:native\");\n      if (artifact === null || task.signal.aborted) throw task.signal.reason ?? new Error(\"natural-file.export-empty\");\n      if (!heldArchive()) throw new Error(\"natural-file.export-owner-retired\");\n      const exportedArchive = await readOwnedDocumentArchiveV1(owned.handle, entry, runtimeKey, core.openDocumentSessionsRef.current);\n      if (task.signal.aborted || !heldArchive() || exportedArchive === null || !sameLocalDocumentArchiveV1(encodeDocumentArchiveBytes(exportedArchive), expected)) throw task.signal.reason ?? new Error(\"natural-file.saved-revision-changed\");\n      if (!naturalMediaDescriptorMatchesV1(decodePackValue(artifact.descriptor), format)) throw new Error(\"natural-file.export-descriptor-mismatch\");\n      downloadMediaExportBytes(file, format.mediaType, artifact.data);", "      const verifyDelivery = async (): Promise<void> => {\n        if (!heldArchive()) throw new Error(\"natural-file.export-owner-retired\");\n        const exportedArchive = await readOwnedDocumentArchiveV1(owned.handle, entry, runtimeKey, core.openDocumentSessionsRef.current);\n        if (task.signal.aborted || !heldArchive() || exportedArchive === null || !sameLocalDocumentArchiveV1(encodeDocumentArchiveBytes(exportedArchive), expected)) throw task.signal.reason ?? new Error(\"natural-file.saved-revision-changed\");\n      };\n      if (declaration !== null) {\n        const delivered = await downloadDeclaredMediaFileV1({ instanceId: owned.target.instanceId, declaration, port: owned.handle, signal: task.signal, current: heldArchive, progress: task.progress, sinkFactory: shellSegmentedDownloadSinkFactory, beforeDelivery: verifyDelivery });\n        if (!delivered) throw task.signal.reason ?? new Error(\"natural-file.export-owner-retired\");\n      } else {\n        const artifact = await exportMedia!(owned.target.instanceId, NATURAL_FILE_PORT_ID);\n        if (artifact === null || task.signal.aborted) throw task.signal.reason ?? new Error(\"natural-file.export-empty\");\n        await verifyDelivery();\n        if (!naturalMediaDescriptorMatchesV1(decodePackValue(artifact.descriptor), format)) throw new Error(\"natural-file.export-descriptor-mismatch\");\n        downloadMediaExportBytes(file, format.mediaType, artifact.data);\n      }");
  });
  mkdirSync(output,{recursive:true});
  assert.ok(existsSync(output));
  writeFileSync(stage,JSON.stringify(revisions,null,2)+"\n");
  for (const [index,row] of revisions.entries()) writeFileSync(join(output,index+"-"+row.path.split("/").at(-1)),row.after);
  console.log("[DEBUG] Staged "+revisions.length+" natural Save paths");
}
